## [2026-09-29] 헌장 DoD 에 build-slot 한 줄 (rustjava-charter-wrap-heavy-builds-with-build-slot)
- 무엇을: `CLAUDE.md` §Definition of Done 에 «무거운 빌드·테스트는 `~/orchestrator-live/bin/build-slot run -- <cmd>` 로 감싼다(대기 상한 30분 · rc 는 cmd 의 rc · 라이브에 없으면 맨 명령)» 한 줄을 더했다. 코드 변경 0.
- 왜: 여러 세션이 한 호스트에서 cargo 를 동시에 돌려 부하가 몰렸다. 공유 슬롯(기본 3)으로 동시 실행 수를 묶는 정본 문안(`templates/repo-CLAUDE.md`)을 이 repo 헌장에도 옮겼다.
- 사용자 영향: 없음(에이전트 작업 규율). 이 워크로그는 게이트③ 머지 회차가 동봉했다(구현 PR 은 티켓 범위 «CLAUDE.md 1파일» 로 워크로그를 싣지 않았다).
