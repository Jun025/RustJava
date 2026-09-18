# Should the root fixtures be recompiled onto one target?

**Decision: no. `test-data/` keeps its five class file versions.**
The spread is not neglect — for a JVM implementation it is coverage, and this document is the
measurement that establishes which parts of it are coverage and which parts are merely history.

Adopted proposal: `2026-09-17-test-data-version-freeze#p0`. Measured 2026-09-18 against
`origin/main` @ `8c7b473f` with javac 26.0.2.1.

## What is actually there

Read from the fixtures themselves (`od -An -tu1 -j6 -N2` on bytes 6-7), not from the freeze file:

| major | Java | root fixtures |
|---|---|---|
| 52 | 8 | 40 |
| 65 | 21 | 62 |
| 66 | 22 | 8 |
| 68 | 24 | 1 |
| 70 | 26 | 3 |
| | | **114** |

## Where the version *is* the thing under test

Two lowering axes change with the target, and in both cases the older shape is the one a JVM has
to keep supporting. Both were measured by recompiling the committed source at `--release 21` and
comparing, not by reading the spec.

**1. String concatenation.** `javac` lowers `+` to `StringBuilder` through Java 8 and to
`invokedynamic StringConcatFactory` from 9 on.

- Three major-52 fixtures carry `StringBuilder`: `FormatterIntegration`,
  `FormatterIntegration$FailingAppendable`, `NullSpecGuards`.
- Recompiled at 21 all three gain `BootstrapMethods`. Two of them — `FormatterIntegration` and
  `NullSpecGuards` — lose `StringBuilder` entirely. The third does not: `FailingAppendable`
  declares `private final StringBuilder output` at `test-data/src/FormatterIntegration.java:36`,
  so that use is the program asking for the class by name rather than the compiler reaching for it,
  and no target changes it. What moves is the concatenation, not every mention.
- `StringConcat.class` (major 65) is the **only** root fixture that has `invokedynamic` at all.

So the root tree holds exactly one fixture per lowering strategy. Retargeting the 52 cohort would
delete the pre-indy side of that pair and leave two fixtures asserting the same path.

**2. Nestmate access.** Before JEP 181 (Java 11), a nested class reaching a private member went
through a synthetic `access$NNN` bridge; from 11 on, `NestHost`/`NestMembers` permit it directly.

| fixture | committed (52) | recompiled (21) |
|---|---|---|
| `ThreadInterruption` | `access$` ×10, no Nest attrs | `access$` ×0, `NestMembers` |
| `MonitorSemantics` | `access$` ×4, no Nest attrs | `access$` ×0, `NestMembers` |

This repo added `NestHost`/`NestMembers` handling only recently, which makes both sides live test
surface rather than legacy.

**`NativeMethod` is the same second axis wearing a different face.** It changes opcode counts at
21 (`invokespecial` 3→2, `invokevirtual` 1→2) with no nest attributes on either side, which this
document first recorded as unexplained. The gate-2 review isolated it: the whole `javap -c -p` diff
is one line, `invokespecial` becoming `invokevirtual` on a call to `private native void missing()`
made from `static main` in the same class. After JEP 181 a member of a nest reaches its own private
methods directly, so there is no `access$` bridge to delete here — the class is its own nest host —
and only the opcode moves. So all three of the differing fixtures are accounted for by the two axes,
with nothing left over.

**This coverage exists nowhere else.** Across the 64 generator-built fixtures in `test-data/cp`,
`indy`, `ldc` and `attr`, the counts of `StringBuilder` and of `access$` bridges are both **zero**.
The root major-52 cohort is the only place either path is exercised.

## Where the version is irrelevant

Of the 40 major-52 fixtures, 20 have a rebuildable source and carry neither `StringBuilder` nor
`BootstrapMethods`. Recompiling those at `--release 21` and comparing the full opcode sequence:

- **16 produce an identical instruction sequence.** For these the target is genuinely arbitrary —
  `ArrayEdgeCases`, `BooleanTest`, `CheckCast` and the rest lower the same way at 8 and at 21.
- 3 differ, and they are the two nestmate cases plus `NativeMethod` above.
- 1 could not be rebuilt alone (it references a sibling; see `verify-javac-fixtures.sh`).

So the honest split is: **of the 20 examined the target matters for 3, one cannot be rebuilt
alone, and for the remaining 16 it buys nothing** — which is also why uniformity is not worth its
cost. The three are the two nestmate cases and `NativeMethod`.

A first draft of this document said "5 of the 20". That was wrong by its own arithmetic — 5 + 16 is
21 — and wrong in the direction that flatters the conclusion, because it reached the 5 by adding
the two string-concatenation fixtures, which the selection had *already excluded* for carrying
`StringBuilder`. The conclusion does not rest on this ratio (it rests on the zero-coverage
measurement below), but a policy document's headline number should be countable from its own
bullets, so it is stated here with the set it counts: the 20 are the major-52 fixtures that are
top-level and free of `StringBuilder`.

## Why not unify anyway

The proposal's stated benefit is that a fixture's shapes would be "predictable from one number".
The measurement inverts that argument. Today major 52 reliably *means* "pre-indy, pre-nestmate
shapes", and that is a signal a reader can use. Flattening everything to 65 does not make the
number more informative; it removes the distinction the number currently carries, and it does so
by deleting the only coverage of two paths the runtime still has to implement.

Against that, the cost is real: recompiling changes bytes, every root fixture's version is pinned
in `test-data/class-file-versions.txt` and asserted by `tests/test_fixture_pins.rs`, and 66 root
fixtures have their stdout compared against a committed `.txt` by `tests/test_class.rs`. A bulk
retarget is therefore a per-fixture judgement, and for 16 of 20 the judgement returns "no change".

## What this does not decide

Nothing here says which target a *new* fixture should be built for. This round looked only at the
fixtures that already exist and decided not to move them; the rule for new ones is a separate
question and was deliberately left open. Until it is answered, a new fixture's target is whatever
its author chose, and the freeze records that rather than judging it.

## Reopen this decision if

- **The runtime drops support for the old shapes.** If RustJava stops accepting major ≤ 52, or
  stops needing the `StringBuilder` and `access$` paths, the 52 cohort stops being coverage.
- **That coverage turns up somewhere else.** The claim above is a measurement with a date: zero
  `StringBuilder` and zero `access$` in the 64 generator fixtures. If a later round adds fixtures
  that cover those paths deliberately, the root cohort becomes redundant and this should be re-run.
- **The freeze goes away.** This decision assumes `class-file-versions.txt` plus
  `test_fixture_pins.rs` keep the versions from moving on their own. Without that guarantee the
  question is no longer "should they be uniform" but "why did they change".
- **A third version axis turns up.** `NativeMethod` was the open candidate and is now closed as
  the nestmate axis again, so the current set is two. A genuinely new one would widen the "version
  is the subject" set and strengthen, rather than weaken, this decision.

Re-measure rather than re-argue. The commands are in the round's worklog:
`docs/worklog/2026-09-18-root-fixture-target-decision.md`.
