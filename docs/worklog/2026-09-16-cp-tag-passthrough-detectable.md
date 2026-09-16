# 2026-09-16 — Making the constant-pool tag switch's pass-through branch observable

`taskId: rustjava-cp-tag-switch-passthrough-mutation-detectable`

## The premise, demonstrated before touching anything

`ConstantPoolItem::parse_tagged` ends in `_ => Err(...)` — the branch that rejects tags that cannot
appear in a class file. Mutating it to accept:

```rust
_ => Ok((data, Self::Integer(0))),
```

`test_unsupported_constant_pool_tag_raises_class_format_error` — the test whose stated job is
"we still reject unknown constant pool tags" — **still passed**. Across the whole suite exactly one
test caught the mutation, and it was a `classfile` unit test, not the end-to-end one.

## Why it passed — narrowed by experiment, not by guessing

The old test overwrote the tag byte of `Hello.class`'s **first** constant pool entry. That entry is
a `Methodref` the code invokes, so overwriting it breaks the file along several independent paths
at once. `ClassFileError` flattens every parse failure into `"Invalid class file"` (a limitation
the test file already documented), so the assertion cannot distinguish *rejected because the tag is
unknown* from *rejected because the class fell apart*.

The obvious competing hypothesis — that a pass-through consuming zero bytes desynchronises the
reader and breaks the pool that way — was **tested and rejected**: a mutation consuming exactly
four bytes, matching the `Methodref` payload it replaced, still left the test green.

## The fix is not a tighter assertion

There is nothing to tighten; the message is flat. The input has to reach the branch as the **only**
defect. `test-data/src/cp/make_cp_fixtures.py` emits `UnreferencedTag{13,14,19}.class`: a class
valid in every respect except one pool entry that is

* **unreferenced** — nothing points at it, so no downstream consumer can reject it instead;
* **payload-free** and **last** — which is what an unassigned tag looks like, and which means a
  pass-through consuming nothing leaves the reader correctly positioned at `access_flags`.

All three properties are load-bearing. Without the last two, a mutated parser produces a
*differently broken* class and the test goes green again for a new wrong reason.

## Evidence

| | pass-through mutated | result |
|---|---|---|
| before this round | reject → accept | test **ok** (the defect) |
| after this round | reject → accept | test **red**, failing with `must be rejected: ""` — the empty output meaning the class ran to completion |
| after this round | a *different* branch (tag 16) mutated | **6 tests red** — the suite still guards the whole switch |

`cargo test --all`: 568 passed / 0 failed / 1 ignored — unchanged, because one test was replaced.
The count is not the evidence; the before/after mutation pair is.

**Product code is untouched in the final diff.** The mutations were temporary demonstrations and
were reverted; `git status classfile/ jvm-bytecode/` reports zero changed files, and the restored
`constant_pool.rs` hashes back to `393e0594d7eb647e`.

## Fallout check

No other test used the old in-test fixtures (`BadTag*`: zero references). The `hello_class()` and
`fixture()` helpers are still used 4 and 5 times respectively, so nothing was orphaned.

The new assertion should not add false positives: the fixture is built to carry exactly one defect,
so there is little surface for an unrelated change to disturb it.

## Follow-ups

See `proposals` in the sibling `.json`.
