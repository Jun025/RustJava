# 2026-09-19 — The check now proves its own reading of the loader, instead of trusting it

Round: `rustjava-assert-loadable-rederivation-did-not-come-up-short`
Adopted proposal: `2026-09-19-loadable-set-source-of-truth#p0`

The proposal has **no `how` field** — `title`, `plainSummary`, `userBenefit`, `why`, `tradeoff`,
`effort`, `target` only. What follows says where this implementation matches its `why` and where it
deliberately departs from its `tradeoff`.

## First: is anything short today?

No. Measured on `origin/main` @ `ad9eb1a6`, four independent counts of the same thing:

| count | value |
|---|---|
| `_proto()` occurrences in `loader.rs` | **268** |
| `_proto(),` lines | 268 |
| `crate::classes::` occurrences | 268 |
| registrations `REGISTERED` parses | 268 |
| distinct names resolved | **268** |

Also measured, because they are what would make the assertion fire *wrongly*: registrations sharing
a line **0**, `_proto()` without a trailing comma **0**, `_proto()` inside a comment **0**, duplicate
resolved names **0**.

So the assertion starts green and guards a regression rather than fixing a present defect. The
proposal says as much; this round confirms it with numbers rather than assuming it.

## What was added — two axes, both fail-closed

1. **Parsed vs. witnessed.** `REGISTERED` is the pattern under suspicion, so counting its own matches
   proves nothing. `PROTO_CALL` counts the same calls a second way, by the one token a registration
   cannot be written without. Mismatch ⇒ `cannot measure` (exit 2).
2. **Registrations vs. distinct names.** Two registrations resolving to one name means the resolution
   is wrong — it is exactly what bare-name keying did. Mismatch ⇒ exit 2, **naming the collapsed
   pairs** so a genuine duplicate registration can be told from a mis-attribution at a glance.

**Changed: 1 file, +38/−2.**

## Axis — bidirectional, on the product script, by re-creating the two real defects

| the script, mutated back to a defect it actually had | result |
|---|---|
| `((?:as\|list)_proto)` → `(as_proto)` (the first draft) | **rc 2** — `265 registrations parsed but 268 proto calls are in rustjava-runtime/src/loader.rs` |
| key by bare type name (the first draft) | **rc 2** — `268 registrations resolved to only 263 names`, then names them: `java/util/Formatter <- java::util::Formatter::as_proto(), java::util::logging::Formatter::as_proto()`, … |
| unmutated | **rc 0** — `✓ 43 named exception class(es) across 846 call site(s); all 268 loadable` |

The second message is the point of the round: gate 2 found that colliding pair by reading the source
alongside the script. The script now says it.

## The three questions the brief asked

**⒜ What is it compared against?** The source itself — a second count of the same file. Not a stored
baseline, not the previous run.

**⒝ First run, and legitimate decreases?** They do not arise, and that is *why* this shape was
chosen. A remembered number would have to answer both; a self-contained invariant answers neither
because it never remembers anything. Removing a class legitimately drops all counts together and
stays green.

**⒞ Die or speak?** **Die — exit 2, `cannot measure`.** The sibling round
(`2026-09-18-nonliteral-exception-call-sites#p0`) chose "count and print, never fail" for the
non-literal blind spot, and this round deliberately differs: that is a *known limitation* being
sized, this is the check *mis-reading its own input*. The file already has a category for the
latter — it dies on a registered entry whose name cannot be resolved — and this is the same failure
one step earlier. A check whose loadable set is short reports real classes as unloadable and missing
ones as present; printing that and exiting 0 would be the silent pass the file's docstring is about.

## What this costs

- **Two more numbers that have to stay true.** If `loader.rs` ever calls `as_proto()` outside the
  registration array, `PROTO_CALL` counts it and the check goes red on a correct tree. Measured
  today: every one of the 268 calls is a registration (`crate::classes::` count matches exactly).
- **A false red is possible for a genuine duplicate registration** — two entries deliberately naming
  one class. There are none today, and the message names them, but it would be a red on a tree that
  is arguably fine.
- **It is a floor, not a proof** — the proposal's own words. A registration mapped to a *wrong but
  distinct* name keeps both counts at 268 and passes. This catches undercounts, which is what both
  measured false greens were.
- Runtime: **no significant change** — before 6.47 / 8.01 / 4.57 s, after 6.35 / 5.68 / 7.53 s
  (overlapping; the machine is loaded and the spread is wider than the effect).

## Departure from the proposal's `tradeoff`

It predicted the check would be *"coupled to the textual shape of `loader.rs` (one registration per
line), which is true today and is not guaranteed."* That coupling was avoidable and was avoided:
counting `_proto()` **occurrences** rather than lines makes the witness independent of line layout
and of trailing-comma style. Measured equal to the line count today (268 = 268), so nothing is lost
by the more robust form.
