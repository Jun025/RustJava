# RustJava Agent Guidelines

## Build/Test Commands
- **Build**: `cargo build` (workspace), `cargo build -p <crate>` (single crate)
- **Test all**: `cargo test`
- **Single test**: `cargo test <test_name>` or `cargo test -p <crate> <test_name>`
- **Format**: `cargo fmt`, **Lint**: `cargo clippy`

## Code Style
- **Edition**: Rust 2024, `#![no_std]` for core crates (use `alloc` crate)
- **Line width**: 150 chars (rustfmt.toml)
- **Indent**: 4 spaces, LF line endings
- **Imports**: Group `alloc`/`core` first, then external crates, then `crate::` local imports
- **Naming**: snake_case for functions/files, PascalCase for types/traits
- **Error handling**: Use `Result<T>` (alias for `result::Result<T, JavaError>`), never panic in library code
- **Async**: Use `#[async_trait::async_trait]` for async trait methods, `#[tokio::test]` for async tests

## Git Workflow
- **Never commit directly to `main`**: always work on a short-lived branch.
- **Clean up merged branches (MANDATORY)**: once a branch's work is complete and merged into `main`, delete it — remote and local — and sync local `main`. Prefer `gh pr merge --delete-branch` (deletes the remote), then locally `git branch -D <branch>` and `git fetch --prune`. Use `-D` (force) because squash-merged branches aren't recognized as merged by `-d`. Leave no stale merged branches behind — only `main` and in-progress work remain. Never re-merge or re-PR an already-merged branch.

## Project Structure
- `jvm/` - Core JVM implementation (`#![no_std]`)
- `jvm-bytecode/` - JVM class and bytecode implementation
- `rustjava-runtime/` - Java standard library implementations
- `classfile/` - Class file parser
- `jvm-class-proto/` - Java class prototypes
- `jvm-types/` - Shared JVM metadata types
- `test-utils/` - Shared test utilities

## Round Worklog `docs/worklog/` — human `.md` + machine `.json`, always a pair
When a round leaves follow-up proposals or their disposition, drop **two files with the same
basename** in `docs/worklog/`: `YYYY-MM-DD-<slug>.md` (the human axis) and
`YYYY-MM-DD-<slug>.json` (the machine axis). Without the `.json`, the proposal is
**structurally unreachable** by the cockpit "후속 작업 추천" panel — its scanner reads `.json` only.

**Do not invent a schema** — these key names are shared with otterpebble/dodu/qts. The consumer
(`/api/proposals`, `scanRepoSimple`) reads exactly these:

| key | type | what the consumer does with it |
|---|---|---|
| `date` | `"YYYY-MM-DD"` | sort axis (falls back to the filename's first 10 chars — set it anyway) |
| `proposals[]` | array of objects | one element = one card. `ref` is derived as `<basename>#p<0-based index>` |
| `proposals[].title` `plainSummary` `userBenefit` `why` `tradeoff` `effort` `target` | string | card body — fill **all 7**; an empty string renders as an empty field |
| `adoptedProposals[]` · `declinedProposals[]` | string (`ref`) array | removes that `ref` from the open recommendations (disposition record) |

Any other key (`schema`, `taskId`, `summary`, `changes`, `verification`, `issues`, …) is free —
the consumer does not read them, so they are for humans and the next round.

**No retroactive conversion.** The convention applies to new rounds only; the lock asks only
"if a `.json` exists, is it well-formed and does it have its `.md` sibling" — it never demands a
`.json` for an existing `.md`. Lock: `scripts/check-worklog-json.py`, run by the `worklog_json`
CI job **and by the local DoD** (`CLAUDE.md` §Definition of Done — 4th command; `cargo test` does
not cover docs, so it stays a plain script).

**Mandatory since 2026-09-04** (decision, not a habit): a round that writes 후속 추천 into
`REPORT.md` also writes the `docs/worklog/` pair. Rationale — the failure is **silent and
unlockable**: the lock only validates a `.json` that *exists*, so a round that skips it produces
zero cards with no error anywhere. Measured before deciding: of the **4 rounds landed since the
convention itself landed** (`b3a4cf4`, 2026-08-26 → `origin/main`), **3 wrote the pair (75%)**;
the one miss (PR #13) branched off *before* the convention, so among rounds that could have known
it is **3/3**. The mandate costs one line and converts an observed habit (n=3, all one lineage)
into a checked one.

**Revert numbers — recount at the 10th round landed after 2026-09-04.** Do not re-argue this
from taste; re-measure:

```sh
# ⒜ 미작성 회차: 착지 회차 중 worklog 쌍이 없는 것
for c in $(git rev-list --first-parent <2026-09-04-이후-첫-착지>..origin/main); do
  git diff --name-only "$c^1" "$c" | /usr/bin/grep -q '^docs/worklog/' || echo "$c no-worklog"; done
# ⒝ 열린 카드 수: proposals 총합 − (adopted + declined)
python3 -c "import json,glob;p=a=0
for f in glob.glob('docs/worklog/*.json'):
    d=json.load(open(f));p+=len(d.get('proposals',[]));a+=len(d.get('adoptedProposals',[]))+len(d.get('declinedProposals',[]))
print('open cards:',p-a)"
```

- ⒜ **≥ 2 (>20%)** ⇒ DoD 에 적어도 안 지켜진다는 뜻이다. 문안을 **빼거나** 기계 강제로 올려라 —
  문서에만 둔 채로 유지하지 마라(그 상태가 가장 나쁘다: 규칙은 있고 효력은 없다).
- ⒝ **< 5** ⇒ 의무화해도 카드가 늘지 않았다는 뜻이므로 **의무 자체를 재검토**하라.
  Baseline measured 2026-09-04: 9 proposals − 4 disposed = **5 open**. So ⒝ means "fewer than today".
## Testing Boundaries
- Keep `rustjava-runtime/tests/classes` limited to Java standard library class and API behavior.
- Test JVM and interpreter semantics, including class initialization, bytecode execution, and monitor behavior, with compiled Java fixtures under `test-data/src` and expected output under `test-data`, executed by `tests/test_class.rs`.
- Do not place JVM core behavior tests in the `rustjava-runtime` standard library test tree.

## Compatibility Sources
- Implement Java compatibility from public specifications, Javadocs, and observable behavior tests. Do not consult or reproduce OpenJDK or other Java runtime implementation source code; keep the implementation independent to avoid licensing and provenance concerns.
