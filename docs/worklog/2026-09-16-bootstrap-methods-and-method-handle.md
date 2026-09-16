# 2026-09-16 — `BootstrapMethods` becomes a structure; nothing links a call site yet

Ticket: `rustjava-invokedynamic-bootstrapmethods-and-methodhandle`.
Adopted proposal: `2026-09-16-cp-tags-15-18-parse#p0`.

The proposal asked for "`invokedynamic` 실행 — BootstrapMethods 파싱 + 콜사이트 링크" and, in its own
`tradeoff`, said the work must be split into ⒜ BootstrapMethods parsing + MethodHandle resolution
and ⒝ linking one `StringConcatFactory` call site. `STATE.md` ④-1 carried that split forward.
**This round is ⒜ only.** The proposal's `target` field lists
`jvm-bytecode/src/{verifier,interpreter}.rs` — that belongs to ⒝; neither file is touched here,
and §"The gate" says why touching them now would be a defect rather than progress.

## What landed

One file of production code, `classfile/src/attribute.rs`:

- `AttributeInfo::BootstrapMethods(Vec<u8>)` → `Vec<BootstrapMethod>`.
- `BootstrapMethod { method: MethodHandleRef, arguments: Vec<u16> }`.
- `MethodHandleRef { kind: MethodHandleKind, member: FieldMethodref }`.
- `MethodHandleKind` — the nine `reference_kind` values of JVMS 4.4.8, named.

Plus two stale comments corrected (see §"Stale claims") and one new fixture.

### Why `attribute.rs` and not `constant_pool.rs`

`MethodHandleKind` arguably belongs next to `CONSTANT_MethodHandle`. It went here because the
only thing that reads it today is the attribute in the same file, and the sibling round
(`rustjava-ldc-tags-15-16-17-still-malformed`, PR #44) has `constant_pool.rs` open with edits to
the very enum and function a new type would sit beside. Moving a type later is cheap; a code
conflict between two open PRs in the same parser is not. Recorded as follow-up ⑶ so the choice is
revisited rather than inherited.

## The boundary of "MethodHandle 결정" — named, not hand-waved

The ticket asked for this explicitly and asked not to claim it all works. It does not all work.

**What works:** a `CONSTANT_MethodHandle` entry is decoded into the reference kind and the class,
name and descriptor *as written in the class file*.

**What does not, each named:**

1. **No class loading.** `java/lang/invoke/StringConcatFactory` is a string, not a loaded class.
2. **No member lookup.** Nothing checks that `makeConcatWithConstants` exists or has that descriptor.
3. **No access checking** (JVMS 5.4.3.5 → 5.4.4).
4. **No `java.lang.invoke.MethodHandle` object, because the package does not exist here.**
   Measured, not assumed: `rustjava-runtime/src/classes/java/` contains `io lang net text util`
   and no `invoke` directory, and `ledger-grep -rln 'java/lang/invoke' rustjava-runtime/src/`
   returns **0 files**. Creating that package is ⒝'s work.
5. **No kind↔target pairing check in this code.** JVMS 4.4.8 says kinds 1–4 take a Fieldref,
   5–8 a Methodref or InterfaceMethodref, 9 an InterfaceMethodref. `validation.rs` already
   enforces that and this parser deliberately does not repeat it — two copies of one rule is how
   they start disagreeing. The test asserts the rule really is enforced there, so the omission is
   *unnecessary* rather than merely *absent*.
6. **`bootstrap_method_attr_index` is still not bounds-checked** against the array it indexes.
   Now possible for the first time, deliberately not done — see §"What was left undone".
7. **Static arguments stay as raw indices.** The next section is entirely about this.

## The one real decision: arguments stay indices

Resolving `arguments: Vec<u16>` into `Vec<ConstantPoolReference>` is the obvious next line. It is
a regression, and the regression is invisible to the fixture that already existed.

`LambdaMetafactory.metafactory` takes three static arguments — `MethodType`, `MethodHandle`,
`MethodType` — none of which this crate can represent with a useful payload. Forcing resolution
makes the *parse* fail, which turns a lambda-bearing class from

```
java.lang.UnsupportedOperationException: Unsupported class file feature: invokedynamic
```

back into

```
java.lang.ClassFormatError: Invalid class file
```

i.e. it silently undoes the "corrupt vs. unsupported" distinction that the last two rounds were
spent building, for the single most common shape javac emits.

**Measured (M3), not reasoned.** And the sharp part: under M3 the pre-existing fixture
`StringConcat.class` **stays green**, because its one static argument is an ordinary `String`.
A round that only had that fixture would have made this change, watched the suite pass, and
shipped the regression. That is why `test-data/indy/Lambda.class` is committed here.

## Adversarial mutations

Each applied to the finished tree, then reverted; the suite is green again after each.

| # | mutation | what it models | result |
|---|---|---|---|
| M1 | `num_bootstrap_arguments` read as `u8` instead of `u16` | one field width wrong | both structure tests **FAILED** |
| M2 | `bootstrap_method_ref` index read `+1` | one field offset wrong | all three tests **FAILED** |
| M3 | every static argument required to resolve | the "obvious next line" | `Lambda` **FAILED** at both layers; **`StringConcat` stayed green** |
| M4 | verifier's `Opcode::Invokedynamic(_)` arm removed | is `todo!()` still unreachable? | `panicked at jvm-bytecode/src/interpreter.rs:631` — **host abort** |

M4 is the ticket's gate, and it reproduces the previous round's measurement to the line number.
The `todo!()` is reachable **only** with that arm gone, so it remains unreachable here, and
`verifier.rs` is untouched (`git diff --stat` on it is empty).

## What each assertion is bitten by

The ticket asked for this pairing explicitly, because "wrote it down, nothing locks it" is this
ledger's recurring failure.

| claim | what bites it |
|---|---|
| the attribute parses into structure | every field of the entry asserted, not the count — M1 and M2 both go red |
| the method handle is decoded, not just counted | kind + class + name + descriptor compared to javap's output |
| the recipe argument index is meaningful | the index is dereferenced through the pool and compared to `String("a\u{1}")` |
| arguments survive as indices | `Lambda` locked at two layers: `ClassInfo::parse` (indices) and `run_class` (the sentence a user reads) |
| kinds outside 1..=9 are rejected | 0, 10, 255 → `InvalidFormat` |
| pairing is `validation.rs`'s job, not ours | kinds 1, 4, 9 mispaired → `InvalidFormat`; **kind 5, correctly paired → parses**, so the assertion is not vacuous |
| `todo!()` unreachable | M4 |

## Stale claims — closed by count, not by list

Two comments asserted the attribute was an unparsed blob and became false with this change.
`~/orchestrator/bin/ledger-grep -rn 'BootstrapMethods'` over `*.rs` and `*.md` (excluding
`target/` and worklogs) returned 13 lines; of those, two were live false claims:

- `classfile/src/constant_pool.rs` — "`BootstrapMethods` is still kept as raw bytes". Replaced:
  the index is still not dereferenced, but now for a different and true reason.
- `classfile/src/validation.rs` — "still an unparsed byte blob, so there is nothing to bound it
  against". Replaced: there now *is* something to bound it against; what is missing is access to
  the class attributes from that function.

The rest are either this round's own code and tests, or history — the previous round's completed
`STATE.md` entry and `REPORT.md` follow-up, which describe what *that* round did and are still
true of it. Those were left alone.

## What was left undone, on purpose

`Dynamic`/`InvokeDynamic` entries carry a `bootstrap_method_attr_index` that JVMS requires to be
a valid index into this array, and a class pool containing them requires the attribute to exist
at all. OpenJDK enforces both — it rejected the previous round's hand-built fixture with
`Missing BootstrapMethods attribute`. Both checks became possible for the first time today and
neither was added:

- it is validation, not parsing, and this ticket is scoped to parsing;
- it is a behaviour change — files that parse today would start being rejected;
- `STATE.md` ④-2 already carries a named follow-up for the sibling gap (tag vs. major version),
  and these belong in one round rather than split across two.

## Numbers

- `cargo test --all`: **558 / 0 / 1 → 562 / 0 / 1** (+4, zero regressions).
- DoD 7 commands, rc=0 each.
- `jvm-bytecode/src/verifier.rs` and `jvm-bytecode/src/interpreter.rs`: untouched.
- No runtime (`rustjava-runtime/`) change; no call-site linking; no `java.lang.invoke`.
