## [2026-09-25] `REPORT.md`·`STATE.md` 동결 — 회차 기록은 회차별 파일로 (rustjava-report-state-md-per-round-files-port-from-wie)
- 무엇을: 두 원장 머리에 동결 1줄을 달고 해시로 고정했다(`scripts/check-ledgers-frozen.py` · CI `worklog_json` job + 로컬 DoD). 회차 기록은 `docs/worklog/YYYY-MM-DD-<slug>.md`, 진행중은 `gh pr list`, 다음은 `docs/next.md` 로 옮겼다.
- 왜: 모든 PR 이 두 파일 맨 앞에 덧붙여서 착지 1회가 열린 형제 PR 전건을 충돌시켰다. 2026-09-10~25 PR 61건에 합집합 머지가 42건이었다(`origin/main` 실측). wie `a5091df6` 선례를 포팅했다.
- 사용자 영향: 없음(문서·검사). 회차 PR 이 공유 줄을 만지지 않으니 원장 충돌 재회차가 사라진다. ★열린 PR #81 은 두 원장을 만진다 — 착지 전에 그 항목을 자기 worklog `.md` 로 옮겨야 한다.

wie 와 다른 점: 새 `docs/report/NNNN--` 연번 디렉터리 대신 기존 `docs/worklog/` 의 `date-slug` 이름을 쓴다(파일 이름이 겹치지 않으니 연번 충돌 검사기가 필요 없다). `STATE.md` 도 한 번에 전부 동결했다(wie 는 진행중·완료를 두 번에 걸쳐 포인터로 바꿨다). 과거 기록은 옮기지 않았다.
