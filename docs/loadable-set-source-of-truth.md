# Should the loadable set be read from the loader instead of re-derived?

**Decision: no. `check-named-exception-classes-are-loadable.py` keeps parsing the Rust source.**
The proposal's premise is correct — three of the four defects really were in the re-derivation — but
reading from the loader buys less than it looks and costs a production API plus a build dependency,
and the failure mode it aims at is detectable for nothing without giving either up.

Adopted proposal: `2026-09-18-named-exception-classes-are-loadable#p2`. Measured 2026-09-19 against
`origin/main` @ `ddc6c4ce`.

## First: is "three of four" true?

It is. Read from the fix commit `89c2e83c`, not from the prose about it — four distinct defects, and
which half of the checker each lived in:

| # | defect | fixed by | half |
|---|---|---|---|
| 1 | only `as_proto()` matched, so three `list_proto()` registrations read as absent | `REGISTERED` regex gains `(?:as\|list)_proto` | **loadable (re-derivation)** |
| 2 | first `name:` in an `impl` block attributed the wrong class when a type holds two proto constructors | `IMPL_BLOCK` → `IMPL_START` + `PROTO_FN`, per function | **loadable (re-derivation)** |
| 3 | bare type names as keys, so one of a colliding pair answered for its twin | key becomes `(module, type, function)` | **loadable (re-derivation)** |
| 4 | line-by-line scan missed 34 calls rustfmt had broken across a newline | `NAMED.finditer(line)` → `finditer(text)` | **named (call-site scan)** |

So 3/4, as claimed. What the claim does *not* say, and what decides this: **the fourth is in the half
that reading the loader cannot remove.**

## Why reading from the loader buys less than it looks

The check compares two sets. Reading the loader replaces one of them:

- `loadable` — which classes the runtime can resolve. This *could* come from the runtime.
- `named` — which class names the Rust code passes to `Jvm::exception`. This **cannot**. There is no
  way to learn it except by reading the source, short of executing all 846 call sites.

Defect 4 was in `named`, and that half keeps every hazard that made it: 846 call sites, rustfmt
splitting calls across lines, and — found **4 h 31 min later** by
`2026-09-18-nonliteral-exception-call-sites` (`89c2e83c` 19:49 → `128e0fe5` 00:21, which is "the next
day" only by the calendar) — the same anchor matching **eight other function names**: `exception(` is
a substring of `assert_exception(`, `suppress_io_exception(` and six more, **41 sites** whose first
argument is `jvm` rather than a class name. Counting them would have answered **33** instead of the
correct **0**. Reading the loader removes three defects' worth of parsing and leaves the parser.

## What it would cost

**A production API that exists only for the check.** `get_runtime_class_proto` builds its 268
registrations as a **local array inside the function** and consumes it with
`protos.into_iter().find(|proto| proto.name == name)`. There is no enumeration — measured: zero
public functions returning the proto list. So emitting the names means either exporting that array
from `rustjava-runtime`, or writing a test that restates the list, which re-creates the two-sources
problem the proposal is trying to remove.

**A build dependency on a check that has none.** Measured: the checker runs in **0.75–0.96 s** on
nothing but source text. Its own job, `named_exception_classes`, is five lines — checkout, `python3
script`. Four of the five jobs in `rust.yml` need **no toolchain** (`worklog_json`, `merge_drops`,
`dod_parity`, `named_exception_classes`); only `rust_ci` does. (Line counts differ among those four —
`merge_drops` is seven, carrying `fetch-depth: 0` — which is why the shared property named here is the
toolchain, not the length.) Reading from the loader moves this
check across that line, in CI and in the local DoD both.

The proposal names the drift risk itself: a generated list goes stale when the emitter is not re-run.
This repo already runs that pattern once — `test-data/class-file-versions.txt` with
`record-class-file-versions.py` — and `AGENTS.md` has to spell out that adding a fixture means
editing the freeze file *in the same commit*, because otherwise it silently rots.

## The deciding measurement: the failure mode is cheap to catch anyway

The proposal's fear is that a re-derivation bug produces a false green. That bug has a signature —
the parse comes out *short* — and one invariant sees it:

```
registration lines in loader.rs  ==  registrations the checker parsed  ==  distinct names it resolved
```

Run against both versions of the checker on today's tree:

| checker | parsed registrations | distinct names | vs 268 registration lines |
|---|---|---|---|
| first draft (`38cbab7e`) | **265** | **263** | **breaks — caught on the spot** |
| current | 268 | 268 | passes |

So defects 1 and 3 would have been caught in the first round, by a check that keeps the
independence, the 0.8 s, and the zero build steps. That is not built here — the adopted proposal
asked for a decision, not a third mechanism — and is filed as a follow-up.

*Not claimed*: that this invariant catches every mis-attribution. A registration mapped to a wrong
but still distinct name can keep the count at 268. And it catches an undercount only on the
**loadable** side: all three of its terms — registration lines, parsed registrations, resolved names —
are read from `loader.rs` and `classes/`, so it never sees the `named` count at all. Of the two
measured false greens it therefore catches **one** (defect 3, the colliding bare-name key) and misses
defect 4, whose undercount was on the `named` side (812 against 846). What it does catch is one false
green and one false red.

## What would reopen this

The proposal states the deciding count itself: *"how many more re-derivation defects show up — if
this round was the last one, the parsing version is cheaper."* Today that count is **0 since the fix
landed**, and that number is worth exactly what one day of evidence is worth — which is why it is not
the argument above.

**Re-measure at the third defect.** If a third defect is found in the loadable re-derivation (the
`REGISTERED` / `PROTO_FN` / keying path — not the `named` scan, which reading the loader does not
fix), this decision reopens and the emitter becomes the cheaper option. Count them the same way this
document did: from the fix commits, classified by which half they lived in.
