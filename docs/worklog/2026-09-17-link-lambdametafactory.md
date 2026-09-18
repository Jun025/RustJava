# 2026-09-17 — Linking the second call site: `LambdaMetafactory.metafactory`

`taskId: rustjava-adopt-link-stringconcatfactory-p2` ·
adopts `2026-09-16-link-stringconcatfactory#p2`

## The proposal's cost estimate was wrong, and that is the whole shape of this round

> "unlike string concatenation it genuinely needs invoke machinery … the `java.lang.invoke` package
> this round avoided becomes unavoidable." — the proposal, effort **L**

What a `metafactory` call site produces is an **object**: an instance of the functional interface
whose single method runs the implementation. A `MethodHandle` is how a real JVM delivers that; it
is not what the call site means. And this runtime already has both halves of the delivery:

* `MethodBody::Rust(Box<dyn JvmCallback>)` — a method whose body is Rust rather than bytecode,
* `Jvm::register_class` — a class definition made at runtime and registered by name.

So the class the factory would spin is built directly (`jvm-bytecode/src/lambda.rs`): interface
from the call site's return type, one field per captured value (javac's own `arg$n`), one method —
the interface's — implemented by a Rust body that invokes the implementation. No `java.lang.invoke`
was added. Nothing in `jvm/` changed at all.

Two things fell out of the existing design rather than being built:

* **The GC already traces it.** `find_all_fields` walks `ClassDefinition::fields`, so captured
  objects are reachable because they are fields — which is also why they are fields.
* **Registration is idempotent.** `register_class_internal` ends in `.or_insert(class)`, so two
  threads reaching the same call site both build a class and the first one wins. No lock.

## What is not linked

The implementation's signature has to line up with the interface method's as a pass-through:
identical primitives, or a reference either way. Where a real `LambdaMetafactory` would insert an
adapter — box an `int` into an `Object`, unbox, widen — this refuses the call site, and because the
check runs at lowering time from descriptors alone, the class either loads or does not. There is no
path that fails halfway through a call.

`test-data/indy/LambdaBoxing.class` is that boundary, committed and asserted: OpenJDK 26.0.1 runs
it and prints `3`; here it is refused. The follow-up proposal in the `.json` is to close it.

## Where the observability was hard

Three checks did not die when they were broken, and each needed something different.

**The void drop.** A `void` interface method whose implementation returns a value has to discard
it. Delete that and every test still passed: the stray value lands on the operand stack *below*
everything the following instructions push, and well-formed bytecode never pops it again. Nothing
inside the interpreter can see it. It is visible only from outside — `Thread.run()` invokes
`Runnable.run()V` from Rust and converts the result with `From<JavaValue> for ()`, which panics on
anything but `Void`. The fixture now hands a lambda to a `Thread`, and the mutation dies with
`Expected void, got Int(7)`.

**REF_invokeSpecial.** javac stopped emitting kind 7 for lambda bodies at Java 11 (nestmates), so
no `--release 21` fixture can reach that branch — measured on the same source: `--release 8` gives
kind 7, `--release 21` gives kind 5. Class files that old are what this runtime is for, so the
branch stays and `LambdaCapturingThis` is compiled at 8. `tests/test_fixture_pins.rs` now pins per
fixture instead of pinning one release for all — an exemption would have made the fixture's whole
point unchecked.

**The static arguments.** The identity check has four axes and each has a near miss; the argument
*count* and *kinds* are two further checks and had none, so a bootstrap naming `metafactory`
correctly while carrying four arguments, or a String where the instantiated type belongs, would
have been read as if it were the real thing. Two more hand-assembled fixtures. All six are valid
class files that OpenJDK loads and refuses at linkage — which is what makes them near misses rather
than corrupt files.

## Mutation matrix

Product-side, each reverted after measuring. Unmutated: **576 passed / 0 failed**.

| | mutation | result |
|---|---|---|
| M1 | `lambda::lower` not called | **red** — nothing links |
| M2–M5 | identity: owning class / method name / descriptor / reference kind, one at a time | **red** ×4, each by its own near miss |
| M6 | the pass-through signature check ignored | **red** — `LambdaBoxing` links |
| M7 | `REF_invokeSpecial` dropped from the dispatch | **red** — `LambdaCapturingThis` |
| M8 | receiver left in the argument list as well as used | **red** |
| M9 | the `void` drop removed | **red** — `Expected void, got Int(7)` (survived before the fixture reached `Thread.run()`) |
| M10 | captured values not stored | **red** |
| M11 | `REF_newInvokeSpecial` yields null | **red** |
| M12 | the `--release 8` pin entry removed | **red** — the pin test |
| M13 | the instantiated-argument kind check removed | **red** |
| M14 | four static arguments accepted | **red** |

14 of 14. `cargo test --all`: **573 → 576** passed / 0 failed / 1 ignored (baseline measured on
`origin/main` in a separate worktree). DoD's seven commands rc=0; regenerating the indy fixtures
leaves every pre-existing one byte-identical.
