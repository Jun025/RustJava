# 2026-09-17 — A recipe that contradicts its call site

`taskId: rustjava-adopt-link-stringconcatfactory-p1` ·
adopts `2026-09-16-link-stringconcatfactory#p1`

The proposal asked a question rather than prescribing a fix: when a linked recipe's arity
disagrees with the call site, is a runtime `BootstrapMethodError` the right diagnosis, or should
this move to `classfile`'s validation as a `ClassFormatError`? It offered its own answer —
"the current runtime check is cheap and correct; the question is only whether the diagnosis is in
the right place."

Both halves of that sentence turned out to be false.

## What OpenJDK does, measured rather than argued

There was no fixture for this shape, so three were assembled (`recipe_arity_call_site` in
`test-data/src/indy/make_indy_fixtures.py`) and run on OpenJDK 26.0.1:

```
$ java -cp . RecipeWantsFewerArguments
Exception in thread "main" java.lang.BootstrapMethodError: bootstrap method initialization exception
	at java.base/java.lang.invoke.BootstrapMethodInvoker.invoke(BootstrapMethodInvoker.java:187)
	at java.base/java.lang.invoke.CallSite.makeSite(CallSite.java:310)
	at RecipeWantsFewerArguments.main(Unknown Source)
Caused by: java.lang.invoke.StringConcatException: Mismatched number of concat arguments: recipe wants 1 arguments, but signature provides 2
	at java.base/java.lang.invoke.StringConcatFactory.argumentMismatch(StringConcatFactory.java:470)
```

All three fixtures behave the same way: `BootstrapMethodError`, caused by `StringConcatException`,
and the frames say `linkCallSite` — the call site body never ran.

So the proposal's suggested move is **rejected**: this is not a `ClassFormatError`. The file
parses, and the class file format has nothing to say about what a bootstrap's static arguments
mean; it is a linkage error, and moving it to parse time would have made this runtime disagree
with the JVM it is imitating — and would have cost exactly what the proposal's own tradeoff line
predicted (walking bootstrap arguments during validation, which `attribute.rs` deliberately
avoids). The `where` half of the proposal's question therefore had a cheaper answer than either
option it listed.

## What was actually broken

**⑴ The branch could not throw.** `java.lang.BootstrapMethodError` does not exist in this runtime.
`jvm.exception("java/lang/BootstrapMethodError", …)` resolves the class, fails, and `jvm.rs:948`
unwraps that failure — so the error path **panicked** with a `NoClassDefFoundError` instead of
throwing. The first fixture written for it hit exactly that:

```
thread '…' panicked at jvm/src/jvm.rs:948:94:
called `Result::unwrap()` on an `Err` value: JavaException(ClassInstance(java/lang/NoClassDefFoundError))
```

It stayed green for as long as it did because nothing had ever executed the branch: a recipe can
only contradict its call site in a hand-built file, and no such file existed. The class is now
there — a `LinkageError` child, as in the JDK — and mutation M4 below re-creates the panic, so it
is load-bearing rather than merely present.

**⑵ The check only looked for a shortfall.** It fired when the recipe ran out of arguments. The
opposite direction — a recipe *shorter* than the call site — is invisible to a guard shaped that
way: the loop simply never asks for the surplus arguments, so `RecipeWantsFewerArguments` printed
a quietly wrong `a` and exited 0. A wrong answer, where the direction the proposal named produced
a refusal. So the comparison became an equality, on both axes (arguments and constants), which is
also what `StringConcatFactory` itself does.

## Why it is checked before anything is converted

There is no `CallSite` to link here, so "linkage" collapses into the first execution of the
opcode; the closest this design can come to OpenJDK's ordering is to compare the counts before
converting anything. That is not cosmetic: conversion runs `String.valueOf`, which calls user
`toString()`, so converting first means running user code on a call site already known to be
malformed — and letting that code's exception replace this diagnosis.

## Mutation matrix

Product-side mutations, each reverted after measuring. Unmutated: 574 passed / 0 failed.

| | mutation | result |
|---|---|---|
| M1 | delete the agreement check | **red** — all three fixtures run and print |
| M2 | `!=` weakened to `>` (shortfall only) | **red** — `RecipeWantsFewerArguments` prints `a` |
| M3 | drop the constants half of the condition | **red** — `RecipeWantsAConstant` prints `a` |
| M4 | unregister `BootstrapMethodError` from `loader.rs` | **red** — the panic at `jvm.rs:948` returns |

`cargo test --all`: **573 → 574** passed / 0 failed / 1 ignored (baseline measured on
`origin/main` in a separate worktree). DoD's seven commands rc=0. Regenerating the indy fixtures
leaves the four pre-existing ones byte-identical.

## For the next round

The lock worth having is not about string concatenation: **72** `java/…Error|Exception` names
appear in this workspace's error paths, and before this round exactly one of them had no proto in
`rustjava-runtime` — each such name is a panic waiting for the first execution of its branch. The
baseline is now 0, which is the cheapest moment to nail it down. See `proposals` in the `.json`.
