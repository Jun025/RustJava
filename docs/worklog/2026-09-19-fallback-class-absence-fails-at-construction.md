# 2026-09-19 — The reporter cannot report its own absence. Ask the loader directly instead.

Round: `2026-09-19-exception-reports-instead-of-aborting-p0`
Adopted proposal: `2026-09-19-exception-reports-instead-of-aborting#p0` —
*"Bound the exception-construction recursion when the fallback class itself is unloadable."*

## The proposal was right, and this round is the first time it was measured

The round that filed it wrote, in its own worklog: *"This is read off the call graph, **not measured**
— constructing a JVM whose loader lacks that class needs a custom loader harness, which was out of
this round's scope."* That harness is `test_jvm_hiding` in `test-utils`, added here. With it:

| loader | result |
|---|---|
| hides `java/lang/NoClassDefFoundError`, cap 5 / 20 / 60 / 120 | completes — **6 / 21 / 61 / 121** round trips through the cycle |
| same, cap 160 or 200 | ★ `thread has overflowed its stack` · `fatal runtime error: stack overflow, aborting` · **SIGABRT** |

So the cycle is real, it has no floor, and it kills the process somewhere between **121 and 160**
levels. `load_class` reports a class it cannot provide by calling
`Jvm::exception("java/lang/NoClassDefFoundError", …)`; building that exception goes back through the
loader; if the loader cannot provide *that* class either, the two call each other forever.

## Two attempts that measurement rejected

Neither was wrong on paper. Both were wrong in the tree.

**1. Add it to `Jvm::new`'s `bootstrap_classes`.** Six tests failed instantly. The panic line mapped
to `threads.get_mut(&thread_id).unwrap()` — resolving a class runs its initialisation, which needs
the thread attached *below* that list. The list is for definitions that need nothing.

**2. Resolve it after the system class loader, so the registry answers later.** Normal startup passed
(6/6), and for every loader that *can* provide the class the cycle does become unreachable. But
against the hiding loader it **still overflowed the stack** — only now during construction. The
proposal's stated benefit is *"a clear failure instead of a stack overflow"*, and this was the same
stack overflow at a different time.

The reason is the shape of the problem, not the placement: **the reporting path cannot report the
absence of the reporter.** `resolve_class` hands failure to `Jvm::exception`, which is the cycle.

## What was built

Ask the loader **directly**, bypassing the exception machinery, before resolving:

```rust
assert!(
    jvm.inner.bootstrap_class_loader.load_class(&jvm, "java/lang/NoClassDefFoundError").await?.is_some(),
    "the class set has no java/lang/NoClassDefFoundError, which is the class this runtime reports \
     every other missing class with. …that recursion has no floor (measured: 121 round trips, then \
     the process aborts on a stack overflow). Add it to the class set."
);
jvm.resolve_class("java/lang/NoClassDefFoundError").await?;
```

One direct question turns the condition into an immediate, named failure; the `resolve_class` that
follows registers the class so the loader is never asked again on an error path.

**Changed: 3 files.** `jvm/src/jvm.rs` (the check + comment), `test-utils/src/lib.rs` (the harness),
`jvm/tests/test_exception_fallback_recursion.rs` (new, 1 test).

## Axis — bidirectional, on the product path

| form of `Jvm::new` | a class set without the reporter |
|---|---|
| before | `stack overflow, aborting` — **SIGABRT, the test binary dies** |
| after | panics at construction with the message above — `should_panic` test passes |

Reverting the check does not merely fail the test, it **takes the test binary down**, which is
exactly the failure a host used to get. Normal startup is unaffected: `test_string` 6/6 before and
after.

## What this costs

- **It is a panic, and `AGENTS.md` says library code should not panic.** Stated plainly because it is
  a real conflict. The alternatives were measured and rejected: returning `Err` is impossible here —
  `JavaError` carries a `ClassInstance`, and the missing class is precisely what would have to be
  instantiated — and adding a non-exception variant was rejected by the round that landed
  `Jvm::exception`, because it silently changes 460 `let…else` sites. `Jvm::new` already `.unwrap()`s
  for the same class of failure on its six bootstrap classes, so this is the file's existing answer
  to "the class set is unusable", now with a message instead of `called Option::unwrap() on a None value`.
- **One more class is resolved at every JVM startup**, for a condition that no complete class set
  will ever hit.
- **It fails earlier than strictly necessary.** A host whose class set lacks the reporter but which
  never takes an error path used to run fine; now it cannot construct a JVM at all. That is a
  deliberate trade — the alternative is that it runs until an error path aborts it.
- **The bound is on this cycle only.** Nothing here prevents a different pair of classes from
  recursing the same way; `java/lang/String` is the obvious candidate, since `exception()` builds the
  message string first. Not measured, not claimed, filed below.

## Effort

The proposal estimated **M**. That was right, and not for the reason it gave: the harness was the
easy part (~40 lines). The cost was that **the first two designs had to be measured to be rejected**.
