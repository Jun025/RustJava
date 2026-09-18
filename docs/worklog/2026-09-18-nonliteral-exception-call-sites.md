# 2026-09-18 — How big is the literal-only blind spot? Zero, and here is the predicate that says so

Round: `rustjava-count-nonliteral-exception-call-sites`
Adopted proposal: `2026-09-18-named-exception-classes-are-loadable#p0`
— *"The new check only sees class names written out in full; nobody knows yet how many are built at
run time instead, so we cannot say how big the blind spot is."*

**This is a measurement round. Nothing in `scripts/` or any `.rs` changed.** The output is a number
and the predicate that produced it.

## Answer

| bucket (bare `exception(` = the `Jvm::exception` axis) | count |
|---|---|
| `definition` — `pub async fn exception(&self, r#type: &str, …)` in `jvm/src/jvm.rs:944` | 1 |
| `literal_java` — first argument is a `"java/…"`/`"javax/…"` string literal | **846** |
| `literal_other` — first argument is a string literal with any other prefix | **0** |
| **`nonliteral`** — **first argument is built at run time (variable, `const`, `format!`, …)** | **0** |
| total `exception(` occurrences in tracked `*.rs` | 847 |

**M = 0. The blind spot is empty today.** Every one of the 846 call sites spells its class name as a
`java/`- or `javax/`-prefixed literal, which is exactly the set
`scripts/check-named-exception-classes-are-loadable.py` already reads — so the check's floor and its
ceiling currently coincide.

Where the 846 live: **781** in product code, **65** under test trees, **0** on a commented-out line.
(The checker counts all three the same way; the split is here only so the number is not mistaken for
a product-only figure.)

`literal_other = 0` is worth stating separately: the checker *also* skips a literal that is not
`java/`-prefixed (e.g. `"org/rustjava/…"`), and there are none of those either. So the checker is not
missing literals for prefix reasons, only for run-time-assembly reasons — of which there are none.

## The predicate

Kept deliberately close to the checker's own, so the two numbers are comparable rather than merely
similar: same file set (workspace `*.rs`, `target/` and `.git` pruned), same whole-file matching so
rustfmt's line break after `exception(` is crossed. Two things differ, and both are necessary:

```python
SITE    = re.compile(r'(?P<prefix>[A-Za-z0-9_]*)exception\(\s*')   # the checker's anchor
LITERAL = re.compile(r'"((?:[^"\\]|\\.)*)"')                       # a plain "…" first argument
DEFN    = re.compile(r'\bfn\s+$')                                  # `fn exception(` is not a call

# bucket = literal_java if the literal starts java/ or javax/
#          literal_other if it is a literal with another prefix
#          nonliteral otherwise            <-- anything unrecognised lands HERE, not in a safe bucket
```

1. The `java/` requirement is **dropped** — we look at whatever the first argument *is*. The checker
   asks "is this name loadable"; this asks "is there a name here at all".
2. `exception(` as a bare substring also matches **eight other functions** —
   `assert_exception(`, `suppress_io_exception(`, `assert_null_pointer_exception(` and five more,
   41 sites in total, whose first parameter is `jvm`, not a class name. Counting those as
   "names built at run time" would have produced **M = 33**, which is a wrong answer to the
   question asked: they are a different function. They are split into their own bucket.

Full script: `~/orchestrator/reports/evidence/rustjava-count-nonliteral-exception-call-sites/enumerate.py`.

## Why the zero is a measured zero

A zero from a predicate that cannot see anything is worthless, so the predicate was tested in both
directions before the number was believed.

**Control** — the predicate must reproduce a number the checker already vouches for. Counting only
literal calls that fit on *one* line gives **812**, which is precisely the checker's own pre-fix
figure (846 total − 34 that rustfmt had broken across a newline, recorded in its docstring). Same
file set, same anchor.

**Mutation probe** — five shapes injected into a product file (`jvm/src/jvm.rs`), measured, reverted:

| injected first argument | bucket it landed in |
|---|---|
| `name` (a `&str` variable) | `nonliteral` ✔ |
| `&format!("java/lang/{}", name)` | `nonliteral` ✔ |
| `SOME_CONST` | `nonliteral` ✔ |
| `r#"java/lang/RawString"#` (raw string) | `nonliteral` ✔ |
| `"org/rustjava/NotJavaPrefixed"` | `literal_other` ✔ |

`nonliteral 0 → 4`, `literal_other 0 → 1`; after revert, back to `0 / 0` with a clean tree. The raw
string landing in `nonliteral` rather than being read as a literal is the intended bias: an
unrecognised spelling is reported as blind spot, never silently as safe.

## What the predicate still cannot see

- **Macro expansion is counted once, at the body.** Four `macro_rules!` in
  `rustjava-runtime/src/classes/java/util/arrays.rs` contain **3** `exception(` sites between them and
  are invoked **22** times, so an expansion-basis count is **865**, not 846. Every one of those names
  is a literal inside the macro body, so this changes the *site* count and not the answer: it is not
  a blind spot, it is a units mismatch, and both this round and the checker use source units.
- **Token-pasted call sites** (`concat_idents!`/`paste!` building the identifier `exception`) would be
  invisible to any text predicate. Measured: this tree has `macro_rules!` in **2** files total, and
  neither constructs a function name. Also 0 for `Jvm::exception` passed as a value or called UFCS,
  and 0 for `exception (` written with a space.
- **A literal that is simply wrong** — a typo matching some other real class — is the checker's own
  documented limit, unchanged here.
- **Other panic paths** (`new_class(`, `find_class(`) are outside the adopted proposal's point and
  were not counted; those return `Result` to their caller rather than unwrapping.

## Judgement on the adopted proposal

The premise was **true and worth asking**: nobody had measured this, and the checker's docstring
asserts the limitation without sizing it. The answer happens to be 0 — which does **not** make the
checker's floor fictional, it makes it *currently tight*. Nothing prevents the next round from
writing `jvm.exception(&name, …)`; the predicate above is what would notice.

Whether to promote that predicate into a check (fail when `nonliteral > 0`) is **deliberately left
open** — the adopted proposal asked for a count, not a gate, and a gate on a baseline of 0 is a
separate decision with its own cost. It is filed below as a proposal instead of being built here.
