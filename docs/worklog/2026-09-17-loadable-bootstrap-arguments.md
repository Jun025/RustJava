# 2026-09-17 — Bootstrap arguments have to be constants you can actually load

`taskId: rustjava-adopt-bound-bootstrap-static-arguments-p0` ·
adopts `2026-09-16-bound-bootstrap-static-arguments#p0`

The previous round checked that every `bootstrap_arguments` entry **points at something**. JVMS
4.7.23 asks for more: each one has to be a **loadable constant** — Integer, Float, Long, Double,
Class, String, MethodHandle, MethodType, Dynamic. A file whose argument named a Utf8 parsed fine.

## The proposal's own worry, measured first

> "getting it wrong turns every lambda class corrupt"

That worry is exact, and it is why `attribute.rs` keeps bootstrap arguments unresolved: a lambda's
are MethodHandle and MethodType entries this crate has no payload for. Resolving them would turn
every lambda from *unsupported* into *corrupt*.

A tag test is not that. It asks **which variant** an entry is; it never looks inside one. To show
that rather than assert it, every committed class file was parsed before and after:

| | parses | fails |
|---|---|---|
| before | 144 | 12 |
| after | **144** | **12** |

Identical — the 12 are the deliberately-corrupt fixtures that already failed. **Zero** files changed
side, lambdas included.

## What the real JVM says

Repointing `StringConcat.class`'s single bootstrap argument at pool entry #4 (a Utf8):

```
$ java -cp . StringConcat        # OpenJDK 26.0.1
java.lang.ClassFormatError: argument_index 4 has bad constant type in class file StringConcat
```

So it is a **format** error, and the message names the same concept this predicate now applies.

## Why it is worth having at all

Without the rule the file is not accepted forever — it fails **later and in the wrong words**.
`ConstantPoolReference::from_constant_pool` returns `None` for a Utf8, the linker declines to link
the call site, and the verifier reports `UnsupportedOperationException`: *we do not support this
file*. About a file that is simply broken. Keeping "unsupported" and "malformed" apart is a line
this repository has drawn in several rounds, and this is the same line.

## Mutations

| | mutation | result |
|---|---|---|
| **M1** | back to presence-only | **red** |
| **M2** | widen the loadable set by one (admit Utf8) | **red** |

M2 is the one that matters: it shows the assertion is about the **boundary**, not merely that some
check exists.

## Scope

The test is a byte patch on an existing fixture — no new committed binary, and every length field
untouched, so the argument's *kind* is the only thing wrong with the mutated file. `cargo test
--all`: **575 passed / 0 failed / 1 ignored**.

What this predicate still cannot ask is anything needing a payload: whether a `Dynamic` argument's
own descriptor is a field descriptor, or whether a `MethodHandle` argument resolves to a real
member. That is by design, not an omission.
