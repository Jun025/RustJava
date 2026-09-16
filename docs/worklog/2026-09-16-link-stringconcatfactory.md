# 2026-09-16 — Linking one call site: `StringConcatFactory.makeConcatWithConstants`

`taskId: rustjava-link-stringconcatfactory-makeconcatwithconstants`

## Before / after, by execution

javac 9+ lowers string `+` to an `invokedynamic` bound to `StringConcatFactory`. Before:

```
java.lang.UnsupportedOperationException: Unsupported class file feature: invokedynamic
	at java/lang/ClassLoader.defineClass(...)
```

Note the frame: the refusal happened at **class definition**, so the class never reached execution
at all. After, `test-data/StringConcat.class` runs and prints `a0`, compared byte-for-byte against
`test-data/StringConcat.txt` by `tests/test_class.rs`.

## The hard part was *where*, not *how*

`BootstrapMethods` is a **class** attribute. `Interpreter::run` is handed a method's `Code`
attribute and nothing else — there is no path from the interpreter to the bootstrap table.

The only place holding both is `ClassDefinitionImpl::from_classfile`, so that is where the call
site is resolved, and the result is written into the code as `Opcode::InvokedynamicStringConcat`.
`Interpreter::run` keeps its signature; no caller changed.

Lowering runs **before** the verifier, and that ordering is the design: whatever is still an
`Opcode::Invokedynamic` when `verify` runs is a bootstrap we do not link, so the existing refusal
rule keeps working untouched. The verifier was not modified at all.

No `java.lang.invoke` was built. There is still no `MethodHandle` and no `CallSite`. The factory's
contract is a string template, and a template can be honoured without the machinery that normally
delivers it — that is exactly the line between "one call site" and "a linkage mechanism".

## The measurement that changed the work

Recognition matches four axes: reference kind, owning class, method name, descriptor. Deleting
that identity check **left every existing test green**: `Lambda` and `ConstantKinds` carry
MethodType/MethodHandle static arguments, so they are rejected by the recipe check *before* the
identity check is ever consulted. The identity check was load-bearing in argument only — nothing
tested it.

So `test-data/src/indy/make_indy_fixtures.py` now emits `NotStringConcatFactory`: kind 6, the
same method name, the same descriptor and a String first static argument — differing from the real
factory in the **owning class alone**. Only the identity check can refuse it.

## Evidence

| mutation | result |
|---|---|
| sever the link (`lower` returns immediately) | `test_class` (output comparison) and `test_only_the_string_concat_bootstrap_is_linked` both red |
| link indiscriminately (drop the four-axis check) | red — via `NotStringConcatFactory`. **Green before that fixture existed.** |

`cargo test --all`: 568 passed / 0 failed / 1 ignored. The count is unchanged — one test was
replaced and one fixture pair added — so the count is not the evidence; the mutations are.
Fixture regeneration is idempotent: existing `test-data/indy/` files unchanged.

## What this costs

The risk changes kind: from "does not run" to **"may run wrongly"**, which is worse. That is why
acceptance is an output comparison rather than "no longer refused". And a linking path invites the
next factory request — the four-axis match is the line held against that.

## Follow-ups

See `proposals` in the sibling `.json`.
