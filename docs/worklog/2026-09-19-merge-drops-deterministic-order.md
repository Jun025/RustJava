# 2026-09-19 — `check-merge-dropped-symbols.py` now prints in a stable order

Adopts `2026-09-19-partial-clone-blob-vs-absence#p0` ("Sort the checker's findings so two runs can
be compared").

## What was wrong

The checker printed the same findings in a different order on different runs, so diffing two runs
showed differences that were not really there. Reproduced on `origin/main`'s own version, range
`e53b2142^..e53b2142` (8 merges, 6 dropped definitions, rc 1): **ten runs under
`PYTHONHASHSEED=random` produced exactly two distinct orderings**, 6 of one and 4 of the other. The
two differ only in which path's block comes first — `tests/test_class_format.rs` or
`test-data/src/indy/make_indy_fixtures.py`.

## Where it actually came from — the proposal's diagnosis was close but not exact

The proposal said findings are "accumulated in a set and printed in iteration order". They are not:
`findings` is a **list**, and the names within a single path were **already** `sorted()`. Auditing
all four sets in the file:

| set | reaches output order? |
|---|---|
| `found` in `symbols()` | no — consumed by `sorted()` at the diff site |
| `names` in `excused()` | no — membership tests only (`name not in accounted`) |
| `theirs_symbols - symbols(...)` | no — already wrapped in `sorted()` |
| **`changed` in `check()`** | **yes** — a set of paths, iterated directly |

So **one** of four candidate sources reaches the output, and it is the path iteration, not the
finding accumulation. The symptom the proposal described is real; the sentence naming its cause
is not, and a fix aimed literally at "the print site" would have worked by accident.

## The change

One line in `check()`:

```python
for path in sorted(filter(None, changed)):
```

Sorted **at the source rather than at the print site**, because `findings` is also *returned* by
`check()`. Sorting in `main()` would order what gets printed and leave the return value still
dependent on hash order, which is a smaller fix wearing the same clothes.

## Verification

Bidirectional, in the product call site (not a copy of it):

| version | distinct orderings / 10 runs | rc |
|---|---|---|
| `origin/main` (before) | **2** | 1 |
| with the fix | **1** | 1 |
| fix removed again (mutation) | **2** | 1 |
| restored | **1** | 1 |

Findings themselves are untouched: 15 output lines before and after, identical when both are
sorted as sets, same rc. The fix changes order and nothing else.

## What this costs

- **The output no longer reflects traversal order.** Nothing reads it, so the practical cost is
  zero, but it is a real property that was removed rather than nothing at all.
- **Nothing locks it.** This repo has **no test harness for `scripts/`** (0 `test*.py` files).
  Measured with the nondeterminism deliberately put back: all four python checkers exit 0
  (`check-worklog-json`, `check-dod-ci-parity`, `check-named-exception-classes-are-loadable`,
  `check-merge-dropped-symbols` itself) and `cargo fmt --check` exits 0. The two Rust axes not
  re-run under the mutation — `clippy` and `cargo test` — do not read this script's output at all,
  so the honest summary is that **the guard against this class of defect is zero**. This fix is a
  habit, not a rule, and a later edit can undo it silently. That is the one genuine gap this round
  leaves, and it is filed as the follow-up proposal rather than built, because a harness is a
  larger decision than a one-line ordering fix.
- **Scope deliberately not widened.** The other three `scripts/` checkers were not audited for the
  same class of defect; that is part of the same follow-up.
