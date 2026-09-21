# 2026-09-19 — Should the non-literal blind spot be a gate? No. Both of the proposal's premises expired.

Round: `2026-09-18-nonliteral-exception-call-sites-p0`
Adopted proposal: `2026-09-18-nonliteral-exception-call-sites#p0` —
*"Decide whether nonliteral exception() call sites should be a check, now that the baseline is 0."*

## Decision

**No gate.** `check-named-exception-classes-are-loadable.py` now **counts and prints** the blind spot
and **never fails on it**. The reasoning is recorded in the checker's own docstring, where the next
round will read it before re-proposing the gate.

## The proposal is one day old and both of its premises are already false

It was written by the round that measured the blind spot at 0. Re-measured against `origin/main`
@ `e9910a7a`:

| premise, quoted from the proposal | status now |
|---|---|
| *"now that the baseline is **0**"* | **false — it is 1.** `jvm/tests/test_exception_construction.rs:19` |
| *"instead of **crashing the whole runtime**"* | **false — it no longer crashes.** |

**The one non-literal site is correct, and it has to be non-literal.** That test asks
`Jvm::exception` for a class no loader can provide. Written as a literal, *this very check* reports
it and goes red — measured when it was first written. So it passes the name through a variable. A
gate at zero would have been **red on `main` the day it was proposed**, against a site that is right.

The proposal predicted this in its own `why` field: *"a gate whose baseline is 0 has its own cost —
it turns a legitimate future refactor (passing a name through a variable) into a red that must be
argued down."* That cost stopped being hypothetical roughly four hours after the sentence was
written.

**And the harm it guards is smaller than the proposal's `userBenefit` says.** Since
`rustjava-jvm-exception-throws-instead-of-unwrap` landed (`e9910a7a`), an unloadable name does not
abort the process — it returns the `NoClassDefFoundError` the loader raised. So a run-time-assembled
unloadable name now means *the caller catches the wrong class*, which is a real bug and a quiet one,
but it is not the crash the gate was argued for.

## What was built instead

The proposal's actual worry is in its `tradeoff`: *"not adding it means the floor stays unmeasured
between rounds."* That is fixed without the gate — the number is printed on every run, pass or fail:

```
Blind spot: 1 call site(s) build the class name at run time, so this check
does not see them. Not an error -- an unloadable name there raises NoClassDefFoundError
rather than the intended exception, which is a wrong catch, not a crash:
  ? jvm/tests/test_exception_construction.rs:19
✓ 43 named exception class(es) across 846 call site(s); all 268 loadable
```

**Changed: 1 file, +90/−6 lines (`git diff --numstat`), 0 new commands, 0 new CI jobs.** Exit codes are untouched (0/1/2
mean exactly what they meant).

## Axis — bidirectional, on product code

| probe | result |
|---|---|
| inject a run-time-assembled `self.exception(name, …)` into **`jvm/src/jvm.rs`** | `Blind spot: **2**` and it names `jvm/src/jvm.rs:1390` |
| revert | `Blind spot: **1**`, tree clean |
| remove the helper-name split from the predicate | `Blind spot: **42**` — 33 helper calls + 8 helper definitions + the 1 real site |

The third probe is the one that shows the number *means* something: `exception(` is a substring of
eight helper functions in the test trees whose first parameter is `jvm`, not a class name. Without
that split the report would be off by a factor of 42.

**Note on the axis clause**: the acceptance asks for "revert it and get red". This change is
deliberately incapable of red — that is the decision. So the axis is the *number* moving and naming
the new site, plus `rc` staying 0 in both directions, which is what "reported, not gated" has to
mean.

## What this costs

- **A number that nobody is forced to act on.** A gate makes you deal with it; a printed line can be
  scrolled past. That is the trade this round chose, and it is the strongest argument for the gate.
- **The count is only as good as its predicate** — the 42 above is what a one-line mistake looks
  like. It is not tested by anything except the probes in this round.
- **The gate's *input set* is now a thing this file can get wrong, and that is a cost the first
  version of this round paid.** "Exit codes are untouched" was true of what 0/1/2 *mean*, and it hid
  the axis that actually matters: merging the two scans put the report axis's prefix filter on the
  gate axis too, so a call written `raise_exception("java/lang/X", …)` was dropped from **both** —
  not gated, not reported. Measured by gate 2 and reproduced here: an injected
  `raise_exception("java/lang/TotallyUnloadableProbe", …)` gave **rc 0** against that form where the
  previous version gave **rc 1**. Fixed by keeping the gate axis unfiltered. The lesson is not the
  bug, it is that "the exit codes are unchanged" says nothing about **what is fed into them**.
- **Product versus test is not distinguished.** Today all non-literal sites are tests; the report
  does not say so, and a product site would read identically.
- Runtime: **no significant change** — before 1.33 / 3.20 / 1.55 / 1.58 s, after 1.73 / 1.88 / 1.44 /
  1.54 s (overlapping). Getting there took two rewrites; see below.

## Three rewrites before this was free

The first two were regressions I introduced; the third is what finally paid for the feature. All
measured, none spotted by eye:

1. **A second full walk of every `*.rs`** — the report scanned the tree again. 3.3–4.3 s → 10.0–13.3 s.
   Merged into one pass shared by both axes.
2. **A per-character newline index** built in Python for every file, to make line numbers cheap.
   It *was* the remaining regression (still ~2×). Replaced with `text.count("\n", 0, …)` — there are
   ~850 matches in the whole tree, so counting per match beats indexing per character.
3. Then the anchor itself: `[A-Za-z0-9_]*exception\(` forced the engine to try every word-character
   position. Anchoring on the literal `exception\(` and testing the preceding character instead is
   the same question and restored parity.

## What would reopen this

- A run-time-assembled name appears in **product** code (every site today is a test), or
- the count grows without a round noticing — which is exactly what the printed line is for.
