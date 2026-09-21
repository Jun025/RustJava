# 2026-09-19 — `java/lang/String` is on the error path too, and it recurses

Task: `rustjava-error-path-needs-java-lang-string-measure-first`
Adopted proposal: `2026-09-19-fallback-class-absence-fails-at-construction#p0`
Cited tree: `origin/main` `64cc4f6440af5790685a633a3c13d4d32ea62ae5`

## The question, and why it was a question

The round that closed the `NoClassDefFoundError` cycle deliberately claimed nothing about String. It
said so in its own worklog: no harness run was done for it, and whether String **recurses**, **fails
cleanly**, or is **already resident by construction time** was unknown. The proposal was explicit that
the measurement had to come first — if String were resident well before any error path could run, the
check would be dead weight on every start-up and one more line asserting something that cannot happen.

So this round measured before it decided anything.

## Premises, re-checked

- `Jvm::exception` calls `JavaLangString::from_rust_string` at `jvm/src/jvm.rs:990`, **before**
  `new_class` at `:995`. String is on the error path exactly as `NoClassDefFoundError` is. ✓
- `test_utils::test_jvm_hiding(hidden, give_up_after)` matches the hidden name exactly, so it takes
  `java/lang/String` without touching the harness. ✓

## The measurement

`give_up_after` is the harness relenting: after that many refusals the real class is handed over, so a
run that would otherwise recurse forever ends and can be observed instead of crashing the runner.

| `give_up_after` | result |
|---|---|
| 5, 20, 60 | survives — construction ends in a plain `NoClassDefFoundError` |
| **116** | survives (reproduced twice) |
| **117** | ★ `fatal runtime error: stack overflow, aborting` — SIGABRT, rc 134 (reproduced twice) |
| 120, 160, 200 | aborts |
| **100000** | aborts — so the cap is the harness relenting, **not a floor** |

Found by bisection between a surviving 60 and an aborting 120. The boundary is sharp and deterministic.

**Answer: ⒜ it recurses.** Not ⒝, not ⒞.

### Why it is not ⒞ (already resident), structurally

- `bootstrap_classes` in `Jvm::new` is **6 names** — `Object`, `Runnable`, `Thread`, `[B`,
  `Serializable`, `Class` — and `java/lang/String` is **not among them**.
- `JavaLangClass::from_rust_class` stores the class name as a **byte array** (`nameBytes`, `[B`), not
  a String — so setting up the bootstrap classes' `Class` objects does not pull String in either.
- The first thing in construction that needs a String is the **properties loop**, and by then the
  loader is the only source.

## The decision: put the check in

Outcome ⒜ is the branch where the proposal's own tradeoff does not apply — String is not resident, and
without the check a host with an incomplete class set gets a SIGABRT rather than a named failure.

Same idiom as the existing `NoClassDefFoundError` check: ask the loader **directly**, then resolve.
The direct question is the part that matters — a bare `resolve_class` would hand the failure to
`Jvm::exception`, which is the cycle itself.

**Placement is not cosmetic.** It goes *before* the properties loop, not beside the existing check.
The existing check sits *after* that loop, so a String-shaped failure reaches the recursion before
anything downstream could report it.

## Cost

The proposal named "every start-up" as the cost, so it is measured rather than asserted.

**Deterministic count** (harness counter at `give_up_after=0` — never hides, only counts):

| form | times the loader is asked for `java/lang/String` |
|---|---|
| with check | **2** |
| without | **1** |

One extra question. The `resolve_class` beside it does not add a second resolution — it *moves* the
one the properties loop already did, which then finds String registered.

**Wall-clock timing was attempted and discarded.** A sibling lane was running cargo on another repo;
the same form's p50 swung between 69 ms and 1690 ms, and across 8 interleaved rounds the with-check
form was *faster* than the without-check form in 3 of them. That measures load, not cost. No wall-clock
number is claimed — only that the difference is below this host's noise floor.

## Bidirectional axis (product call site, `jvm/src/jvm.rs` — not a fixture copy)

| form | result |
|---|---|
| original | green — 2 passed |
| check deleted | ★ `fatal runtime error: stack overflow, aborting` — SIGABRT signal 6 |
| restored | green — 2 passed |

Deleting the check does not merely fail the test: it **takes the test binary down**, which is the
point. An abort is what a host embedding this runtime would get.

## What this round does not claim

- Only these two classes are covered. Whatever else the error path reaches — through the constructors
  and static initialisers of `String` and `NoClassDefFoundError` — is **unmeasured**. Out of scope by
  the ticket's own non-goal; carried as the follow-up proposal.
- The one extra loader question per start-up is real and unavoidable while keeping the property that
  makes the check work. It is the same price the `NoClassDefFoundError` check already pays.
