# 2026-09-19 — `Jvm::exception` had the report in its hand and unwrapped it into an abort

Round: `rustjava-jvm-exception-throws-instead-of-unwrap`
Adopted proposal: `2026-09-18-named-exception-classes-are-loadable#p1`
— *"When the runtime cannot build the exception it wants to raise, it crashes the process; it should
report the failure the Java way instead."*

## What was wrong

`Jvm::exception` (`jvm/src/jvm.rs`) is the function every error path ends in — 846 call sites across
the workspace write `return Err(jvm.exception("java/lang/…", …).await)`. It ran:

```rust
let message_str = JavaLangString::from_rust_string(self, message).await.unwrap();
let instance = self.new_class(r#type, "(Ljava/lang/String;)V", (message_str,)).await.unwrap();
```

Both of those calls return `jvm::Result<T>`, which is `Result<T, JavaError>` — **their failure is
already a Java exception**. The unwraps threw that away and aborted the process instead. The
measured panic says so in one line:

```
panicked at jvm/src/jvm.rs:948:94:
called `Result::unwrap()` on an `Err` value: JavaException(ClassInstance(java/lang/NoClassDefFoundError))
```

`load_class` had correctly raised `NoClassDefFoundError` for the missing class — the report the
caller should have received — and it was carried into the panic message instead of being returned.

## The change

Return it. No new enum variant, no signature change:

```rust
let message_str = match JavaLangString::from_rust_string(self, message).await {
    Ok(x) => x,
    Err(e) => return e,
};

match self.new_class(r#type, "(Ljava/lang/String;)V", (message_str,)).await {
    Ok(instance) => JavaError::JavaException(instance),
    Err(e) => e,
}
```

This is also what a JVM does when raising one exception runs into another: the second one
propagates. The caller now gets the *real* failure — `NoClassDefFoundError` for the missing class,
or whatever the constructor threw — instead of a dead process.

## Is it reachable? Yes, and it is measured, not theoretical

| axis | measured |
|---|---|
| Within this repo's own call sites | **0** — all 43 named classes are loadable (locked by `check-named-exception-classes-are-loadable.py`, landed PR #72) **and** all 43 declare `<init>(Ljava/lang/String;)V` directly (measured this round: 0 missing, 0 relying on an ancestor) |
| Through the public API | **reachable** — `pub async fn exception` is public and `wie` embeds this crate. Any caller naming a class its loader cannot provide aborts the host process |

So the trigger is not reachable from code inside this tree today, and that is exactly why it had to
be reproduced through the API rather than asserted. The test does that.

## Axis (bidirectional, on the product function — not a fixture copy)

`jvm/tests/test_exception_construction.rs` asks for a class no loader provides and asserts the
caller receives `NoClassDefFoundError`.

| form of `jvm/src/jvm.rs` | result |
|---|---|
| before (the two `.unwrap()`s) | **FAILED** — `panicked at jvm/src/jvm.rs:948:94: called Result::unwrap() on an Err value: JavaException(java/lang/NoClassDefFoundError)` |
| after | **ok** — 1 passed |

Reverting the change turns that test red, which is what makes it an axis rather than a demo.

## Caller fan-out — why nothing else had to change

The brief asks specifically for paths that would pass *silently* if the return type moved. The
return type does **not** move, which is the point of this shape:

- `.exception(` call sites: **846 before, 846 after**, none edited.
- `JavaError::` sites: **527**, none edited. Of these, 460 are `let …else`/`if let`, 41 construct,
  16 are match arms, 7 are `matches!`. Adding a variant to `JavaError` — the other way to do this —
  would have left every one of those 460 silently taking its `else` branch, and that is the trap
  this shape avoids entirely. `jvm/src/jvm.rs:1073` (`let JavaError::JavaException(exception) = &err;`)
  compiles *only* because the enum has one variant, so a variant would also have broken it.

**The one real behavioural change**, stated plainly: in the failure case the caller now receives a
*different exception class* than it asked for. **12 product sites** dispatch on the class of a caught
exception (`Err(JavaError::JavaException(e)) if jvm.is_instance(&*e, "java/io/IOException")` in
`print_stream`/`print_writer`/`filter_output_stream`/`formatter`, and `NumberFormatException` in
`integer`/`long`). If the exception they expected could not be built, their guard will not match and
the error propagates instead of being caught. That is strictly better than the process dying, but it
is a real difference and not a no-op.

## Two things this round changed *outside* its own diff, and neither was optional

**1. It falsified three passages in `check-named-exception-classes-are-loadable.py`, so they were
rewritten.** The checker asserted the thing this round removed: *"it unwraps an `Err` and aborts the
process"*, *"DELIBERATELY NOT DONE HERE: turning the `.unwrap()` into a thrown exception"*, and the
failure message *"Jvm::exception unwraps new_class(), so each of these panics instead of throwing."*
Leaving those would have been a false claim stated with authority in the exact place the next round
would read it.

The lock keeps its job but its *reason* changes, and that is the honest way to put it: an unloadable
name no longer kills the runtime, it now raises **the wrong exception** — the caller asked for
`IOException`, gets `NoClassDefFoundError`, and the `catch` meant to handle it does not match. That
is quieter than a crash, so the lock matters at least as much as before. Its predicate was **not**
touched, and its axis was re-verified in both directions: removing one registration →
`rc=1  ✗ java/lang/BootstrapMethodError`, restored → `rc=0`.

**2. It creates this tree's first non-literal `exception(` call site — the sibling round's measured
zero becomes one.** `2026-09-18-nonliteral-exception-call-sites` measured `nonliteral = 0` and
deliberately did *not* promote that into a gate, filing the question as a proposal instead. Within a
day a legitimate reason to have one appeared: **this test must name a class that cannot be loaded**,
which is precisely what a literal cannot do here without turning the loadable-check red (measured: it
did, reporting `jvm/tests/test_exception_construction.rs:13`). Had the zero been gated, this test
would have been blocked by it. Re-measured after this round: **`nonliteral = 1`**, and that one is
this test.

*Also found while re-measuring*: that predicate counts the token inside **comments** — a comment here
mentioning it inline read as a second non-literal site (2, not 1) until the comment was reworded. Not
a defect in this round's work, but it belongs to whoever picks up that proposal.

## What this does *not* fix

- **The degenerate case is unchanged, and it is not a panic — it is unbounded recursion.** If
  `java/lang/NoClassDefFoundError` itself were unloadable, `load_class` → `exception` → `new_class`
  → `load_class` recurses. That cycle exists in both forms: the old `.unwrap()` never bounded it,
  because the inner `new_class` never returns for the unwrap to inspect. **This is read off the call
  graph, not measured** — constructing a JVM whose loader lacks that class needs a custom loader
  harness, which was out of this round's scope. Filed as a proposal below.
- The `from_rust_string` unwrap is fixed the same way, but **no reproduction was found for it**:
  it fails only if `java/lang/String` itself cannot be built. Not measured, not claimed.
- Nothing about *which* classes are loadable changed; that axis is the checker's and is untouched.
