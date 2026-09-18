# 2026-09-19 — Refuse partial clones too? No. The ambiguity was one call, not one environment.

Round: `rustjava-partial-clone-refusal-decision`
Adopted proposal: `2026-09-18-merge-drops-no-silent-git-failure#p0`
— *"The check now refuses to run in a shallow clone, but a clone fetched without file contents can
still make it look like nothing is there."*

## Decision

**Do not refuse partial clones.** Fix the one call that could not tell "the path is not in this tree"
from "the blob is not here": `symbols()` now asks `git ls-tree` before reading a failed `git show` as
absence. The decision and its numbers are recorded in `preflight()`'s docstring so the next round
does not re-ask.

## The proposal was half right, and the half matters

Everything below was measured on a real `--filter=blob:none` clone of this repository, not reasoned
about. Range `e53b2142^..e53b2142` throughout — the known-accident merge, 6 dropped definitions.

| clone | promisor | result | time |
|---|---|---|---|
| full | — | `6 definition(s) dropped` · **rc 1** | 2.5 s |
| blobless | **reachable** | `6 definition(s) dropped` · **rc 1** — *identical* | 15.7 s |
| blobless (fresh) | **unreachable** | `0 definition(s) dropped` · ★**rc 0 — green** | 8.3 s |

So:

- **"A partial clone makes it look like nothing is there" is false on its own.** With the promisor
  reachable git fetches the blobs transparently and the answer is byte-identical. Refusing partial
  clones would refuse a setup that works — and unlike a shallow clone, a partial one can go get what
  it is missing.
- **The silent green is real, but the trigger is narrower**: the promisor has to be *unreachable*. In
  that state the check printed `✓ … (3 file(s) examined)` and exited 0 where a full clone reports six
  dropped definitions. That is exactly the failure class this script exists for.

*Method note*: the first offline attempt reported the **correct** answer, because the earlier online
run had already cached those blobs. The measurement above is from a **fresh** clone with the remote
broken before any read. A contaminated clone answers the wrong question.

## Is the risk realised here? No — measured, and it does not change the decision

`git ls-files .github | grep filter:` → nothing; **no workflow uses a partial clone**, and
`merge_drops` explicitly checks out with `fetch-depth: 0`. So today the false green needs a developer
laptop with a blobless clone and no network. That is *why* refusing would be the wrong trade — it
would cost a working configuration to close a case nobody is in — but it is not a reason to leave the
ambiguity, because the cost of closing it turned out to be one git call.

## Why `ls-tree`, and not the two alternatives

- **Not refusal** (`remote.origin.partialclonefilter`): the detection works — measured `blob:none`,
  while `rev-parse --is-shallow-repository` returns false, which is exactly why the existing
  preflight misses this case — but it blocks the reachable-promisor case that gives the right answer.
- **Not matching git's error text**: the two failures *are* distinguishable by message —
  `fatal: path 'X' does not exist in 'REV'` versus
  `fatal: could not fetch <sha> from promisor remote` — but both exit 128, and the round that added
  `preflight` already weighed and deferred message-matching as fragile across git versions. Nothing
  here changes that.
- **`ls-tree` answers from the tree object**, which a partial clone holds even when it lacks blobs.
  Measured identical behaviour in both clones: path present → one entry, rc 0; path absent → empty
  output, rc 0. No message matching, no environment refused.

## Axis (bidirectional, on the product script)

| form | blobless + unreachable promisor | full clone |
|---|---|---|
| before | `0 dropped` · **rc 0** (silent green) | `6 dropped` · rc 1 |
| after | ★**rc 2** `cannot measure: …:jvm-bytecode/src/class_definition.rs is in that tree but its content could not be read…` | `6 dropped` · rc 1 |

And the case that must **not** break: blobless with a reachable promisor, after the fix → **rc 1, 6
dropped**. Over-blocking 0.

**Cost**: none measurable. Same range, three runs each — before 7.78 / 7.99 / 7.05 s, after 6.39 /
6.72 / 8.06 s (overlapping ranges). DoD's default range `origin/main..HEAD`: 0.46 s → 0.33 s. The
extra call only runs when `show` has already failed.

## Found while measuring, not fixed here

The checker's **output order is not stable between runs** — findings are emitted from a set. Measured
on `origin/main`'s own version with `PYTHONHASHSEED=random`: five runs gave two different orderings
(4 + 1). The rc and the set of findings are identical; only line order moves. This is pre-existing
and unrelated to this round — it is noted because it briefly looked like a regression in the diff of
before/after output, and the next person comparing two runs deserves to know. Filed as a proposal.
