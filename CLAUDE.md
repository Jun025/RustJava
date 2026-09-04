@AGENTS.md

## Goal
`no_std` Rust 로 쓰인 **JVM 구현체**다. 에이전트의 일은 JVM·클래스파일 파서·Java 표준 라이브러리의
**정확성**을 올리는 것 — 라이브러리 코드의 패닉을 `Result<T>`/Java 예외로 바꾸고 회귀를 픽스처로 잠근다.
- ★**연방 내 위치**: RustJava 는 **wie(에뮬레이터)의 비벤더 upstream 의존성**이고 플랫폼(otterpebble)은
  여기를 직접 손대지 않는다 — 여기서 고칠 것은 **JVM 런타임 내부뿐**이다.
  (정본 = otterpebble `.claude/rules/repo-boundaries.md` #4)
- ★**포크다**(`origin`=`Jun025/RustJava` · `upstream`=`dlunch/RustJava`). upstream 발신(PR·이슈·코멘트·
  push)은 **티켓이 명시 허가할 때만** 하고 기본은 조회뿐이며, `gh` 호출엔 `-R Jun025/RustJava` 를
  붙인다(2026-07-22 upstream 오발행 사고).

## Constraints
- 클라우드 표면(claude --remote, Cowork, 앱 채팅/Projects) 사용 금지. 로컬전용.
- 위험 변경(대량 삭제·스키마 변경·배포) 전 git 체크포인트 커밋을 먼저 남긴다. force-push/rebase 금지.
- ★**티켓 없는 착수 금지**(2026-07-26 dispatch-guardrail-scope-fix-002): 착수 지시에 대응하는
  **티켓 파일이 `~/orchestrator/tasks/` 에 없으면 편집·커밋·push 하지 않는다.**
  읽기전용 조사까지만 하고 **총괄에게 확인을 구한다.**
  (dispatcher 를 거치지 않고 생성된 세션 — `Dispatch(Cowork)` 등 — 의 **원장 밖 변경 방지**.)
  - ★**한계(자기신고형 — 과장 금지)**: 세션이 스스로 `tasks/` 에 티켓을 만들면 조건을 충족시킬 수
    있고, **티켓 파일에 provenance 가 없어**(발권 주체 필드도, `bin/queue-lint` 의 출신 검사도 없다)
    **검증기로 막을 수 없다.** ⇒ 이 조항은 **1차 방어**이며 **최종 방어는 diff 검토**다.

## Definition of Done
- ★**`.github/workflows/rust.yml` 이 «치는 그대로» 전부 green** — 축약하지 말고 이 여섯 줄을 그대로 쳐라:
  ```
  cargo fmt --all -- --check
  cargo clippy --all -- -D warnings
  cargo +beta clippy --all -- -D warnings
  cargo clippy --workspace --exclude test-utils --target wasm32-unknown-unknown -- -D warnings
  cargo test --all
  python3 scripts/check-worklog-json.py
  ```
  ★★**`+beta` 줄을 빼지 마라 — CI 는 이 검사들을 «6셀»(toolchain 2 × OS 3)로 친다.**
  `rust.yml` 의 `strategy.matrix.rust = [stable, beta]` 가 그 차원이고, 이 repo 엔 `rust-toolchain.toml` 이
  **없어** 맨 `cargo` 는 **stable 1개**로만 돈다. ⇒ ★**toolchain 축을 안 적으면 「rust.yml 이 치는 그대로」가 «거짓»이 된다.**
  ★**실측 근거**(2026-09-04): `#[allow(clippy::double_must_use)]` **9곳을 전건 지우면**
  ★**stable clippy 는 0인데 beta clippy 가 error 7** 이다 ⇒ ★**그 자산의 그물은 «beta 축»에만 있다.**
  ★★**축약본도 쓰지 마라 — 그것이 2026-09-04 에 실제로 구멍을 만들었다.** 종전 문안은
  `cargo fmt --check`·`cargo clippy`·`cargo test` 로 줄여 적어 ★**wasm32 줄이 통째로 빠져 있었고**,
  그 줄이 ★**`--exclude <크레이트 이름>` 이 사는 «유일한» 자리**다. ⇒ upstream 이 `test_utils` 를
  `test-utils` 로 개명(S8)했을 때 **로컬에서는 어떤 명령으로도 드러나지 않고 CI 에서만** 빨개졌다.
  ⇒ ★**CI 가 검사를 늘리거나 «매트릭스 차원»을 바꾸면 이 블록도 «같이» 고쳐라**(아래 재개 조건이 둘 다 센다).
  ※OS 축(3종)은 로컬에서 재현할 수 없다 — ★**그 차원만은 CI 가 유일한 그물이고, 그것은 «알고 두는» 값이다.**
- 착수·완료마다 STATE.md 의 "진행중/완료/다음" 을 갱신하고, 완료 시 REPORT.md 상단에
  `[YYYY-MM-DD]` 요약 3줄(무엇을·왜·사용자 영향) + 후속 추천을 append 한다.
- ★**후속 추천을 적었으면 `docs/worklog/YYYY-MM-DD-<slug>.{md,json}` 한 쌍도 남긴다** —
  `.json` 이 없으면 그 추천은 cockpit 「후속 작업 추천」 패널에 **구조적으로 도달하지 못한다**
  (규약·되돌릴 수 = `AGENTS.md` §Round Worklog).
- ★작업 티켓의 **완주 지점 = PR 오픈**이다. 머지는 게이트② 검수자가 approve 와 같은 턴에 집행한다(`<id>-merge` 는 예외 경로)
  (`main` 직push 0 — `AGENTS.md` §Git Workflow).
