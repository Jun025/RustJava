## [2026-09-26] PR #81 을 닫았다 — #83 과 같은 변경이었다 (rustjava-pr81-disposition-after-pr83-landed)

- **무엇을**: PR #81(`feat/rustjava-sorted-findings` · CONFLICTING)을 닫고 `docs/next.md` 「카드 밖」 줄을 비웠다.
- **왜**: #81 의 유일한 코드 hunk(`scripts/check-merge-dropped-symbols.py` 의 `for path in sorted(filter(None, changed))`)는 같은 ref 의 PR #83(`750d30d4`)이 이미 착지시켰다 — main 의 같은 파일 :208 · 차이는 주석 문구뿐. 채택 기록 `2026-09-19-partial-clone-blob-vs-absence#p0` 도 `docs/worklog/2026-09-19-merge-drops-deterministic-order.json` 에 있다. 나머지 변경(REPORT.md·STATE.md)은 2026-09-25 동결 대상이다.
- **사용자 영향**: 없음. 열린 PR 목록에서 중복 1건이 빠진다.
