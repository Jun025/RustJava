# 2026-09-19 — Read the loadable set from the loader, or keep re-deriving it?

Round: `rustjava-loadable-set-from-loader-vs-rederive-decision`
Adopted proposal: `2026-09-18-named-exception-classes-are-loadable#p2`

**This is a decision round. No code changed** — no `.rs`, no `scripts/`. The output is a decision, the
measurements behind it, and a re-measure trigger. Full reasoning:
`docs/loadable-set-source-of-truth.md`.

## Decision

**Keep re-deriving from source.** Do not read the loadable set from the loader.

## The premise was checked first, and it holds

The proposal rests on one number — *"three of this round's four defects came from that
re-derivation"* — so it was verified against the fix commit `89c2e83c` rather than the prose:

| # | defect | half |
|---|---|---|
| 1 | only `as_proto()` matched → three `list_proto()` registrations read as absent | loadable |
| 2 | first `name:` in an `impl` attributed the wrong class when a type has two proto constructors | loadable |
| 3 | bare type names as keys → one of a colliding pair answered for its twin | loadable |
| 4 | line-by-line scan lost 34 calls rustfmt had split across a newline | **named** |

**3/4 — true as stated.**

## Why the answer is still "keep re-deriving"

1. **It removes one of the two parsers, not the parsing.** `named` — which class names the Rust code
   passes to `Jvm::exception` — cannot come from the loader; there is no way to know it but to read
   the source. Defect 4 lived there, and so did the next one found **4 h 31 min later** (`89c2e83c`
   19:49 → `128e0fe5` 00:21 — "a day later" only by the calendar): the sibling round
   (`2026-09-18-nonliteral-exception-call-sites`) hit the same anchor matching **eight other function
   names** — `exception(` is a substring of `assert_exception(`, `suppress_io_exception(` and six
   more, **41 sites** whose first argument is `jvm`, not a class name. Counting them would have
   answered **33** where the correct answer is **0**.
2. **It needs a production API that only the check would use.** `get_runtime_class_proto` builds its
   268 registrations as a **local array inside the function** and consumes them with
   `.find(|proto| proto.name == name)`. Measured: **zero** public functions enumerate the protos. So
   the runtime would grow an export to serve a checker — or a test would restate the list, which is
   the two-sources problem again.
3. **It puts a build under a check that has none.** Measured **0.75 / 0.96 / 0.77 s**, source text
   only. Its CI job is checkout + `python3 script`; **4 of the 5 jobs** in `rust.yml` are that shape,
   and only `rust_ci` needs a toolchain.
4. **The failure mode is catchable for free.** See below.

## The measurement that decided it

A re-derivation bug produces a *short* parse, and one invariant sees that:
`registration lines == parsed registrations == distinct names resolved`.

| checker | parsed | names | vs 268 lines in `loader.rs` |
|---|---|---|---|
| first draft `38cbab7e` | **265** | **263** | **breaks — would have been caught in round 1** |
| current | 268 | 268 | passes |

Defects 1 and 3 were visible without giving up independence, the 0.8 s, or the zero build steps.
Filed as a follow-up rather than built here, because the adopted proposal asked for a decision.

*Not claimed*: that the invariant catches every mis-attribution — a registration mapped to a wrong
but distinct name keeps the count at 268. And it sees an undercount only on the **loadable** side: all
three of its terms come from `loader.rs` and `classes/`, never from the `named` count. So of the two
measured false greens it catches **one** (defect 3) and misses defect 4, whose undercount was on the
`named` side (812 against 846) — what it catches is one false green and one false red.

## Where this round's own rule slipped (gate 2, F1)

The rule this round applied to the "3 of 4" claim — *read it from the commit, not from the prose
about it* — was not applied to the sibling citation. The sibling's worklog says comments were a
measured **zero** ("0 on a commented-out line"); the trap it actually hit was the anchor matching
eight other function names. Worse, the same commit's `STATE.md` described that sibling correctly six
lines further down, so **one commit stated the same fact two ways**. Corrected above and in three
other places. The lesson is not "cite more carefully" but the rule that was already written here:
apply it to *every* claim, including the ones that merely set the scene.

## A premise in the brief that does not hold for this repo

The brief warned that reading from the loader might break **machine independence**, citing *"이 repo 의
`machine-independence-guard` 가 그 축이고 오늘 그 잡 하나로 전 PR 이 red 였다"*. Measured: **RustJava has no
such guard** — `git ls-files | grep -i machine-independence` is empty, and `rust.yml` has exactly five
jobs (`rust_ci`, `worklog_json`, `merge_drops`, `dod_parity`, `named_exception_classes`). That axis
belongs to a different repo. The real cost of ⒜ here is the toolchain/build dependency in (3), which
is what this decision weighed instead.

## Re-measure trigger

The proposal names the deciding count: *"how many more re-derivation defects show up."* Today it is
**0 since the fix landed** — one day of evidence, which is why it is not the argument.

**Reopen at the third defect** found in the loadable re-derivation path (`REGISTERED` / `PROTO_FN` /
keying) — not in the `named` scan, which reading the loader does not fix. Count from the fix commits,
classified by half, exactly as the table above does.
