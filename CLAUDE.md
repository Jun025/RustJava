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
- `cargo fmt --check` · `cargo clippy` · `cargo test` 전부 green(명령 상세 = `AGENTS.md`).
- 착수·완료마다 STATE.md 의 "진행중/완료/다음" 을 갱신하고, 완료 시 REPORT.md 상단에
  `[YYYY-MM-DD]` 요약 3줄(무엇을·왜·사용자 영향) + 후속 추천을 append 한다.
- ★작업 티켓의 **완주 지점 = PR 오픈**이다. 머지는 게이트② 검수자가 approve 와 같은 턴에 집행한다(`<id>-merge` 는 예외 경로)
  (`main` 직push 0 — `AGENTS.md` §Git Workflow).
