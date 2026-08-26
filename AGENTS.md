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
- `jvm_rust/` - Rust-based JVM interpreter
- `java_runtime/` - Java standard library implementations
- `classfile/` - Class file parser
- `java_class_proto/` - Java class prototypes
- `test_utils/` - Shared test utilities

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
CI job (`cargo test` does not cover docs).
