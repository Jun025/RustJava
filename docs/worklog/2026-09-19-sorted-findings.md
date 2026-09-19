# 2026-09-19 — The check disagreed with itself between runs. One `sorted()` fixed it.

Round: `2026-09-19-partial-clone-blob-vs-absence-p0`
Adopted proposal: `2026-09-19-partial-clone-blob-vs-absence#p0` —
*"Sort the checker's findings so two runs can be compared."*

## The premise, re-measured on today's `origin/main`

Eight runs of the **unchanged** checker over the same range (`e53b2142^..e53b2142`), under
`PYTHONHASHSEED=random`, hashing the output:

```
6  16efef4228f50b1147cb57f8e1aced09
2  9b1105a77918f30b31c42c890815d10c
```

Two orderings of the same six findings. The proposal is right, and it is right for the reason it
gives: this cost a real round — a before/after diff of this checker looked like a regression until
the *unchanged* version was shown to disagree with itself.

## Where it came from, which is not quite where the proposal looked

The proposal says *"Findings are accumulated in a set and printed in iteration order."* Close, but
the print site was already fine: names are emitted through `sorted(theirs_symbols - …)`, so findings
**within a path** were always ordered. What was unordered is the **path loop** —
`for path in filter(None, changed)`, where `changed` is a `set` built from two `git diff --name-only`
results. So the fix is on the outer loop, not at the print:

```python
for path in sorted(filter(None, changed)):
```

**Changed: 1 file, +7/−1 lines** (one of them code, six a comment recording the measurement).

## Axis — bidirectional, on the product script

| form | 8 runs, `PYTHONHASHSEED=random` |
|---|---|
| that line reverted | **2 orderings** (6 + 2) |
| with it | **1 ordering** (8 / 8) |

Content is untouched in both directions: 6 findings, `rc=1`, and the same summary line
`8 merge(s) in e53b2142^..e53b2142: 6 definition(s) dropped without a trailer`.

**On the acceptance wording**: it asks for "revert it and get red". This change has no verdict to
flip — it changes output *order*, not outcome, so `rc` is 1 before and after by design. The axis is
therefore the ordering count above, measured in both directions on the real script.

## What this costs

- **The report no longer reflects traversal order.** Nothing reads it, which is exactly why this was
  safe — and also why it was never noticed.
- **It makes the report comparable, not the check deterministic.** `examined` counts, which merges
  appear, and everything else that depends on repository state still vary with the repository. A
  future round that diffs two runs across *different* trees will still see real differences; this
  only removes the false ones.
- **It fixes this checker only.** The sibling checks were not audited for the same shape in this
  round; `check-named-exception-classes-are-loadable.py` emits its missing list through `sorted(…)`
  and walks files through a sorted `rust_files()`, so it appeared safe, but that is an observation
  in passing rather than a measurement.
- Runtime: **no significant change** — before 2.09 / 2.51 / 2.41 s, after 2.52 / 1.90 / 1.65 s
  (overlapping). Sorting a set of a few dozen paths is not where this check spends its time.

## Effort

The proposal estimated **S** and that was right: the change is one line. The round's actual work was
the measurement — 8 runs × 2 directions — which is what turns "should be sorted" into "was unordered,
here by how much".
