# 2026-09-16 — `bootstrap_method_attr_index` must name a bootstrap method that exists

`taskId: rustjava-bound-bootstrap-method-attr-index`

## What changed

A `Dynamic` or `InvokeDynamic` constant carries `bootstrap_method_attr_index`, an index into the
`bootstrap_methods` array of the class's `BootstrapMethods` attribute. Nothing checked it. A class
could index past the end of that table, or carry such a constant with no `BootstrapMethods`
attribute at all, and this runtime answered `UnsupportedOperationException` — *this runtime cannot
do that yet* — about a file no JVM can read.

`classfile/src/validation.rs` now has `bootstrap_method_indices_resolve`, called from
`validate_class`. Both failure shapes are **one predicate**, not two branches: an absent attribute
is a table of zero entries, which no index can name.

```rust
bootstrap_method_count.is_some_and(|count| (index as usize) < count)
```

## Why it is in `validate_class` and not `validate_constant_pool`

`validate_constant_pool` gets only the pool. This check needs the pool *and* the class attributes,
and `validate_class` is the only place holding both. That crossing is precisely why the check did
not exist: the comment standing at that spot said so, and left it "to the round that wants it".
This is that round; the comment is replaced by a pointer to the new function.

## Specification

- **JVMS 4.4.10** — `bootstrap_method_attr_index` must be a valid index into the `bootstrap_methods`
  array of the `BootstrapMethods` attribute of this class file.
- **JVMS 4.7.23** — a class whose constant pool holds a `Dynamic` or `InvokeDynamic` entry must have
  a `BootstrapMethods` attribute.
- **Reference runtime** — OpenJDK 26 answers `ClassFormatError: Missing BootstrapMethods attribute`
  for the absent case.

## What this costs — stated, not hidden

Class files that used to pass parsing and fail later as *unsupported* are now **rejected** at parse
time. Downstream that reads as a regression; it is the deliverable. No new error variant was added —
it folds into the existing `ClassFileError::InvalidFormat`, so no caller gains a case to handle.
The check walks the constant pool once per class load, which is not free, but it is one pass over a
map that `validate_constant_pool` already walks.

## Evidence

Four axes, and the mutation that separates them:

| axis | fixture / case | expected |
|---|---|---|
| ⒜ index past end | `LdcDynamicBSMIndexPastEnd` (index 1, one-entry table) | rejected |
| ⒝ valid index | `LdcDynamic`, `Ldc2WDynamic`, `StringConcat`, `Lambda`, `ConstantKinds` | passes |
| ⒞ attribute absent | `LdcDynamicNoBSM` | rejected |
| ⒟ attribute not needed | `Hello`, `Switch`, `OddEven`, `Superclass`, … | passes |

Mutations:

| mutation | red | covers |
|---|---|---|
| whole predicate → `true` | 1 test (`…naming_a_missing_bootstrap_method_is_malformed`) | ⒜ ⒞ |
| whole predicate → `false` | 24 tests | ⒝ and ⒟ **together** |
| inner predicate → `false` | 8 tests, all of them classes that carry a dynamic constant; `test_hello` etc. stay green | ⒝ **alone** |

The third mutation exists because the second conflates two axes — rejecting *everything* makes both
"valid index passes" and "class without the attribute passes" go red at once, so it cannot show
which one is locked. Rejecting only dynamic-carrying classes separates them.

`cargo test --all`: **568 passed / 0 failed / 1 ignored** — unchanged, because one test was
*replaced* rather than added. The count is not evidence here; the mutations are.

Fixture regeneration is idempotent: re-running `make_ldc_fixtures.py` left the ten existing
`.class` files byte-identical and added only the new one.

## Follow-ups

See `proposals` in the sibling `.json`.
