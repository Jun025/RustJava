## [2026-10-04] 열린 제안 카드 20장 재판정 + thread.rs 수동 span 을 래퍼로 분리 (rustjava-open-proposal-cards-triage-and-take-one)

- 무엇을: 열린 카드 20장을 판정해 17장을 근거와 함께 내렸고(이미 해소 11 · 다른 repo·총괄 몫 6), `2026-08-27-upstream-sync-s4#p1` 1장을 채택해 `thread.rs` 의 수동 span 을 `ThreadStartProxy::call` 래퍼로 옮기고 본문(`run`)을 upstream 글자 그대로 되돌렸다. 남은 열린 카드는 2장이다.
- 왜: 수동 span 이 본문 전체를 `async { … }.instrument(span)` 로 한 단 들여써서 upstream 이 본문 한 줄만 고쳐도 충돌했다(S1·S3·S4·S7·S8 다섯 번). 지금 대기 중인 upstream 10커밋도 본문 안 `put_field(…"alive"…)` 줄을 고친다.
- 사용자 영향: 동작 변화 없음(같은 이름·필드의 span 이 같은 future 를 감싼다). 다음 upstream 동기 회차의 `thread.rs` 충돌이 1 → 0 이 된다.

## 실측 — 이번 변경이 충돌을 없앴는가

`git merge-file` 로 같은 3-way 를 두 번 쳤다(base = `merge-base HEAD upstream/main` · theirs = `upstream/main`, behind 10):

| ours | 충돌 hunk |
|---|---|
| `origin/main` 의 `thread.rs` | **1** |
| 이 브랜치의 `thread.rs` | **0** |

이 브랜치 쪽 병합 결과와 `upstream/main` 의 차이는 `use tracing::Instrument;` 1줄과 span 래퍼 11줄(우리 고유분)뿐이다.
`#[tracing::instrument]` 를 되살리지 않은 이유는 그대로다 — PR #4(tracing-attributes 가 no_std 를 깬다).

## 판정표

| 카드 | 판정 | 근거 |
|---|---|---|
| `2026-08-27-upstream-sync-s4#p0` (timer 시간 의존) | ⒜ 해소 | S5 가 upstream `c4665b06` 의 `test_timer.rs` 를 채택했다(`2026-09-04-upstream-sync-s5.md` §3). 테스트는 이제 `TestRuntime::advance_time`(`test-utils/src/lib.rs:139`, 가상 시계)으로 돌고 `test_timer_periodic` 은 없다 |
| `2026-08-27-upstream-sync-s4#p1` (thread.rs span 이동) | ⒞ **채택** | 위 실측 |
| `2026-08-27-upstream-sync-squash-convergence#p0` | ⒜ 해소 | `~/orchestrator/contracts/upstream-sync-repos.conf` 에 rustjava 등재 · `bin/queue-lint` 검사 22 가 `merge_strategy:` 선언을 강제한다 |
| `2026-09-03-upstream-sync-s5-s7-remeasure#p1` | ⒜ 해소 | S5 가 `rustjava-upstream-sync-s5-with-remeasured-conflicts` 로 발권돼 string.rs 판단까지 끝냈다(`2026-09-04-upstream-sync-s5.md`) |
| `2026-09-04-parity-sibling-repo-survey#p0` (wie +beta) | ⒝ wie | 넘길 곳 = wie 레인. 읽기만 해 본 결과 wie `AGENTS.md:70` 에 `cargo +beta clippy` 가 이미 있다 — 총괄이 닫힘 확인만 하면 된다 |
| `2026-09-04-parity-sibling-repo-survey#p1` (qts ruff format) | ⒝ qts | 넘길 곳 = qts 레인. qts `Makefile:31` 에 `ruff format --check .` 가 이미 있다 — 같은 처리 |
| `2026-09-04-upstream-sync-s5#p1` (충돌 목록 밖 파손) | ⒜ 해소 | `scripts/check-merge-dropped-symbols.py`(2026-09-18 · DoD·CI)가 «충돌 표시 없이 사라진 정의»를 잡는다. 서술자 어긋남은 그때처럼 `cargo test --all` 이 잡는다. 위험 서술은 `docs/upstream-sync-approach.md` §4 에 있다 |
| `2026-09-04-upstream-sync-s6#p0` (S7 발권) | ⒜ 해소 | S7 착지(`2026-09-04-upstream-sync-s7.md`) |
| `2026-09-04-upstream-sync-s7#p0` (S8 발권) | ⒜ 해소 | S8 착지, behind 12 → 0(`2026-09-04-upstream-sync-s8.md`) |
| `2026-09-04-upstream-sync-s8#p1` (정기 축) | ⒜ 해소 | `2026-09-04-upstream-sync-cadence-decision.md`(PR #29 · `behind ≥ 20`) + `.github/workflows/upstream-behind.yml`(`2026-09-05-upstream-behind-scheduled-workflow.md`) |
| `2026-09-07-parity-per-repo-parser-axis-design#p1` (qts 도달 가능성 락) | ⒝ qts | 넘길 곳 = qts 레인(`tests/guardrail/` · DoD) |
| `2026-09-07-parity-per-repo-parser-axis-design#p2` (wie 천장 ③) | ⒝ wie | 넘길 곳 = wie 레인(`wie_cli/tests/support/dod_ci_parity.rs` `ceilings()`) |
| `2026-09-12-net-guard-subclass-fixture#p0` (null 가드 축 종결 기록) | ⒜ 해소 | 기록 자체가 그 회차 워크로그 47행에 있다(「이 축에서 더 만들 티켓이 없다」) — 카드가 요구한 산출물이 이미 존재한다 |
| `2026-09-12-test-class-scratch-premise#p0` (병렬 러너 시 픽스처별 cwd) | ⒜ 해소 | 전제가 `tests/test_class.rs:11-19` 주석에 적혀 있어 병렬 러너를 들이는 회차가 그 자리에서 읽는다. 병렬 러너 도입 계획은 없다(카드 스스로 «조건부») |
| `2026-09-17-base-pull-and-stale-block-premise#p0` | ⒝ 총괄 | 넘길 곳 = orchestrator 발권 절차(차단 리포트의 head ↔ PR 현재 head 대조). 카드 target 자체가 «not this repo» |
| `2026-09-17-fixture-single-defect-audit#p0` (상시 vs 1회) | ⒜ 해소 | `947fe90f` 가 결정했다 — `AGENTS.md` Testing Boundaries 에 «수동 · CI 는 돌리지 않는다»로 등재 |
| `2026-09-17-fixture-version-table-backfill#p0` (새 규칙이 열린 PR 에 진 빚) | ⒝ 총괄 | 넘길 곳 = orchestrator 게이트③ 머지 템플릿(base pull 단계). 드러난 자리가 base pull 이고 그 단계는 이미 의무다 |
| `2026-09-17-ldc-asm-regeneration-declined#p0` (ldc 양성 픽스처를 실 JVM 으로) | ⒞ 열림 유지 | `test-data/src/verify-javac-fixtures.sh` 에 ldc 줄이 아직 없다 |
| `2026-09-18-root-fixture-target-decision#p0` (새 픽스처 target 규칙) | ⒞ 열림 유지 | `test-data/class-file-versions.txt` 머리가 «The rule for *new* fixtures is left open» 이라 적고 있다 |
| `2026-09-18-root-fixture-target-decision#p1` (재개 조건 관찰자) | ⒜ 해소 | 대상 문서는 `947fe90f` 로 지워지고 조건이 `class-file-versions.txt` 머리로 접혔다. 남은 조건 셋 중 «동결이 사라짐»·«런타임이 major ≤52 를 버림»은 `test_fixture_pins`/`test_class` 가 red 로 알린다. 나머지 «다른 곳에 커버리지가 생김»은 결함이 아니라 판단이라 관찰자를 둘 것이 아니다 |

## 계수 (처분 후)

```
$ python3 -c "...docs/next.md 의 명령 그대로..."
2026-09-17-ldc-asm-regeneration-declined#p0
2026-09-18-root-fixture-target-decision#p0
```
20 → **2**.
