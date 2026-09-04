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

### ★★[2026-09-05 재측 · 사전 등록된 시점에 도달했다] 「부재」를 기계로 잡지 «않는다»

채택 제안 `2026-09-04-worklog-mandate-and-local-gate#p0` 에 대한 **결정**이다.
★**위 「10번째 착지 회차에 재측하라」가 «충족»됐다** — 2026-09-04 이후 착지 회차 **11**.
⇒ ★**골대를 옮기지 않고, 그때 등록한 두 조건을 그대로 재서 판정한다.**

> ★**결정: 「워크로그 «부재»」를 잡는 기계 강제를 «지금은 넣지 않는다».**
> ★**`check-worklog-json.py` 의 «존재하는 `.json` 만 검사한다» 설계와 «소급 금지»는 그대로 둔다.**

**⒜ 미작성 회차 — 사전 등록 임계 `≥ 2 (>20%)`**

| 창(`b3a4cf4..origin/main` · first-parent) | 착지 회차 | 미작성 | 비율 |
|---|---|---|---|
| 규약 착지 이후 **전건** | **15** | ★**1** | **6.7%** |
| ★**규약을 «알 수 있었던» 회차만** | **14** | ★**0** | ★**0%** |

★**그 1건은 `11ef5010`(PR #13 · S2)이고 «규약보다 먼저 갈라졌다» — 옮겨 적지 않고 쟀다**:
PR #13 `createdAt` **2026-08-23T23:37:41Z** ↔ 규약 착지 `b3a4cf4` **2026-08-26T02:57:28Z**(≈2.6일 먼저).
⇒ ★**임계 «2» 미달**(측정 1 · 정상 참작분을 빼면 0).

**⒝ 열린 카드 — 사전 등록 임계 `< 5`**: `.json` **15**개 · proposals **26** − disposed **13** = ★**13 open**
(기준선 **5** → **13**). ⇒ ★**임계 미달**(늘었다 = 의무가 카드를 «만들고» 있다).

⇒ ★★**두 조건 «다» 발화하지 않는다.** 사전 등록한 규칙대로 **문안을 그대로 두고 기계 강제를 넣지 않는다.**

**★약점을 숨기지 않는다 — 이것이 재개 조건의 근거다**
- ⒤★**「100%」는 «단일 집행 주체»의 습관일 수 있다.** 리니지는 **13개**로 갈렸지만(2026-08-27~09-05),
  ★**git author 는 하나라 «누가 집행했나»를 가릴 수 없다.** ⇒ 일반화의 근거로 쓰지 마라.
- ⒥★**정본 술어는 느슨하다** — 「`docs/worklog/` 를 만졌다」는 **옛 워크로그에 한 줄 추가**도 통과시킨다.
  ★이번 창에서는 엄격 술어(**새 `.json` 추가**)와 **같은 14** 라 물리지 않았지만, 그 약점은 **남아 있다**.
- ⒦★**실패는 여전히 «조용하다»** — 부재는 어떤 검사도 울리지 않는다. 오늘 그 대가를 치르지 않았을 뿐이다.

**★★재개 조건 — 세는 명령과 «오늘의 값»**
> ★**위 ⒜ 를 다시 재서 «규약을 알 수 있었던 회차 중 미작성」이 «1건이라도» 나오면 이 결정을 다시 연다.**
> ★**「오래됐다」·「불안하다」는 재개 사유가 «아니다».**

```sh
# 미작성 회차(엄격 술어 — 새 .json 쌍을 «추가»했는가) · b3a4cf4 = 규약 착지
for c in $(git rev-list --first-parent b3a4cf4..origin/main); do
  git diff --name-status "$c^1" "$c"     | /usr/bin/grep -q '^A[[:space:]]*docs/worklog/.*\.json$' || echo "$c no-worklog"
done | /usr/bin/grep -c .
```
★**오늘의 값 = `1`**(그 1건 = 규약 이전 분기 `11ef5010`) ⇒ ★**«2 이상»이 되면 재개**다.

**★재개하면 «무엇을» 만들지 미리 적어 둔다**(그때 설계부터 시작하지 않게):
⒜★**baseline sha 를 쓰지 마라** — 소급 금지를 깨고, 기준선 자체가 낡는다.
⒝★**PR diff 로 «조건부» 판정하라**: 「`REPORT.md` 의 후속 추천 구역을 만졌으면 `docs/worklog/` 에 **새 쌍**도 있어야 한다」.
  ⇒ 의무가 조건부(「후속 추천을 적은 회차」)이므로 **검사도 조건부**여야 한다. baseline 불요 · 소급 0.
⒞★**정상 참작 경로 = 「그 PR 이 규약 착지보다 먼저 생성됐는가」**(`gh pr view --json createdAt`) — ★**제목·라벨로 봐주지 마라**
  (문구는 바뀌고 라벨은 붙이면 그만이다 · 이 저장소가 반복해 규탄한 형태).
⒟★**`check-worklog-json.py` 의 기존 6축을 약화하지 마라** — 「부재」는 **별 축**이다.
## Testing Boundaries
- Keep `rustjava-runtime/tests/classes` limited to Java standard library class and API behavior.
- Test JVM and interpreter semantics, including class initialization, bytecode execution, and monitor behavior, with compiled Java fixtures under `test-data/src` and expected output under `test-data`, executed by `tests/test_class.rs`.
- Do not place JVM core behavior tests in the `rustjava-runtime` standard library test tree.

## Compatibility Sources
- Implement Java compatibility from public specifications, Javadocs, and observable behavior tests. Do not consult or reproduce OpenJDK or other Java runtime implementation source code; keep the implementation independent to avoid licensing and provenance concerns.
