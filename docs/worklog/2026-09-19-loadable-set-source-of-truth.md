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
   the source. Defect 4 lived there, and so did the next one found: the sibling round
   (`2026-09-18-nonliteral-exception-call-sites`) hit the same anchor matching its own token **inside
   a comment** a day later.
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
but distinct name keeps the count at 268. It catches the undercount class, which is what both
measured false greens were.

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
