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
- Recompiled at 21 they gain `BootstrapMethods` and lose `StringBuilder` outright.
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

**Not explained.** `NativeMethod` also changes opcode counts at 21 (`invokespecial` 3→2,
`invokevirtual` 1→2) with no nest attributes and no `StringBuilder` on either side. The cause was
not isolated. It is recorded here as a real difference rather than folded into the two axes above.

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

So the honest split is: **the target matters for 5 of the 20 examined, and for the other 16 it
buys nothing** — which is also why uniformity is not worth its cost.

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

## Reopen this decision if

- **The runtime drops support for the old shapes.** If RustJava stops accepting major ≤ 52, or
  stops needing the `StringBuilder` and `access$` paths, the 52 cohort stops being coverage.
- **That coverage turns up somewhere else.** The claim above is a measurement with a date: zero
  `StringBuilder` and zero `access$` in the 64 generator fixtures. If a later round adds fixtures
  that cover those paths deliberately, the root cohort becomes redundant and this should be re-run.
- **The freeze goes away.** This decision assumes `class-file-versions.txt` plus
  `test_fixture_pins.rs` keep the versions from moving on their own. Without that guarantee the
  question is no longer "should they be uniform" but "why did they change".
- **`NativeMethod`'s difference is explained** and turns out to be a third version axis — that
  would widen the "version is the subject" set and strengthen, not weaken, this decision.

Re-measure rather than re-argue. The commands are in the round's worklog:
`docs/worklog/2026-09-18-root-fixture-target-decision.md`.
