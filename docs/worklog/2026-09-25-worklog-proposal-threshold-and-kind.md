## [2026-09-25] worklog 제안 문턱 + `kind` 필드 (rustjava-worklog-proposal-threshold-and-kind)
- 무엇을: `AGENTS.md` 에 제안 문턱(추천도 3문 참조 · 0개가 정상 · worklog 당 최대 2 · 체인 3대째 meta 금지)과 선택 필드 `proposals[i].kind`(`product`|`meta`)를 넣고, `check-worklog-json.py` 가 `kind` 값과 «새/변경 worklog 의 제안 3개 이상»을 red 로 잡게 했다. 과거 파일은 base 와 바이트 동일하면 통과(CI 는 `WORKLOG_BASE=HEAD^1` · fetch-depth 2).
- 왜: 09-14~24 제안 63건 · 채택 1건당 새 제안 0.98 · meta 62% · 최장 체인 11대 — 입구에 문턱이 없었다.
- 사용자 영향: 없음(문서·검사기만). 후속 제안 없음.
