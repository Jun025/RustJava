# 2026-09-17 — Capture order, and a class file that killed the process

`taskId: rustjava-adopt-link-stringconcatfactory-p2-fix` ·
gate-2 request-changes follow-up for PR #61 (pin `87ef6a70`)

Two findings, both the reviewer's, both correct.

## F1 — the capture *order* was unobservable

The reviewer reversed one line in `LambdaBody::call` (`&self.captures` → `.iter().rev()`) and the
whole suite stayed green at **576 / 0**. Not a refusal — a **wrong answer**, silently.

The cause is in the fixtures, not the assertion style: every one of them captures **at most one
value**. `LambdaKinds` captures `base` in one lambda and `bound` in another; `Lambda` and
`LambdaCapturingThis` capture one apiece. With one capture there is no order to get wrong, so the
line that orders them is not covered by anything. The round's own mutation M10 ("captures not
stored") tested *presence*, which is a different axis and reads deceptively like the same one.

Two lambdas now capture two values each, and they were chosen to fail differently:

| lambda | captures | prints | why this one |
|---|---|---|---|
| `pair` | `(String, int)` | `a:7` | a swap shows up in the **text** |
| `weighted` | `(int, int)` | `120` | a swap shows up **only in the value** — no type check could catch it |

RM4 now dies on both: `a:7 → 7:a` and `120 → 2001`.

This is not an exotic shape. `(a, b) -> a + b` is what javac emits for the most ordinary lambda
there is, and until this round it would have silently returned the arguments backwards.

## F2 — a valid class file aborted the host process

```
thread 'main' panicked at jvm/src/type.rs:74:13:  Invalid type
```

A call site whose descriptor is `I` — a *field* descriptor — reaches `lower()`, which read it with
the panicking `JavaType::parse` / `as_method`. A panic is not a guest exception: the process dies.
`verifier.rs` already writes that exact sentence about `ldc` ("reaching it would abort the host,
not the guest"), so the repo's own doctrine names this a defect.

### Where to fix it, and why not where it was suggested

The review proposed two lines in `lambda.rs` (`try_parse` + `let … else`). That removes the abort,
but it answers **`UnsupportedOperationException`** — and that answer is wrong:

```
$ java -cp . MetafactoryFieldDescriptorCallSite      # OpenJDK 26.0.1
java.lang.ClassFormatError: Method "run" in class MetafactoryFieldDescriptorCallSite
                            has illegal signature "I"
```

The file is not *unsupported*. It is **malformed**, and this repository has spent several rounds on
keeping those two words apart. So the fix belongs where JVMS puts the rule.

JVMS 4.4.6 says a `NameAndType` descriptor is "a valid field descriptor or method descriptor" —
which it must be, because `Fieldref` and `Methodref` share that entry kind. *Which* one is decided
by the entry that refers to it, and 4.4.10 states it: `CONSTANT_InvokeDynamic` names a method,
`CONSTANT_Dynamic` names a field type. The existing `NameAndType` arm is therefore **right as it
stands** and was left alone; the missing check was on the referring arm, which the codebase already
does for `Methodref` via `validate_member_reference(..., MemberKind::Method)`.

**Cost measured before tightening**, as the ticket required: 175 committed class files, 44
invokedynamic/dynamic references, **exactly one** newly rejected — the fixture written for this
finding. Nothing else in the tree moves.

`lower()` still uses `try_parse`. That is a second layer, and I could not construct an input that
passes the tightened validation and still fails it — so **it is not independently observable**, and
I am not claiming a mutation kills it. It stays because the two layers fail differently: the outer
one produces a guest exception, and the missing inner one produces a dead process.

### A side effect worth naming

The same panic exists on `origin/main` through the string-concat path
(`Interpreter::extract_invoke_params`), which the review scoped out as a separate ticket. Putting
the rule at the usage site closes that entrance too — not by widening this round, but because a
rule in the right place covers everything that reads through it.

## Mutations

| | mutation | result |
|---|---|---|
| **RM4** | capture read order reversed (the reviewer's) | **red** — `7:a` / `2001` |
| **F2-M** | the `InvokeDynamic` descriptor rule reverted | **red** — the file is no longer `ClassFormatError` |

`cargo test --all`: **578 passed / 0 failed / 1 ignored**. Regenerating the indy fixtures leaves
every pre-existing one byte-identical — including after merging this branch's generator with the
sibling one that landed as #59, which is the check that the merge kept both.

## One process note

`git checkout --` to undo a mutation also reverted an **uncommitted** fix sitting in the same file,
and the next command reported the fix as missing. Commit before mutating; the mutation matrix in
this round was re-run on a committed tree.
