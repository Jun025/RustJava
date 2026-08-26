# 2026-08-26 · 회차 워크로그 `.json` + `proposals` 규약 이식 (RustJava)

`taskId`: `rustjava-worklog-json-proposals-convention` ·
채택 근거: `2026-08-23-worklog-json-with-proposals-convention#p2`(qts 회차 제안 · 운영자 cockpit 채택)

## 무엇을 · 왜
cockpit 「후속 작업 추천」 커버리지 6 repo 중 채워진 것이 2개(otterpebble·dodu)뿐이었고,
RustJava 는 그중에서도 `docs/worklog` **디렉터리 자체가 없어** 구조적으로 0건이었다.
qts 회차(2026-08-23)가 세운 규약을 **스키마 발명 0** 으로 복제해 이 repo 에 심었다.

## 한 일
- `AGENTS.md` 에 「Round Worklog `docs/worklog/`」 절 — `.md`(사람 축) + `.json`(기계 축) 한 쌍,
  소비처가 실제로 읽는 키 표, 그 밖의 키는 자유, **과거 소급 없음**.
- `scripts/check-worklog-json.py` — qts 의 guardrail 6축을 그대로 옮긴 잠금.
- `.github/workflows/rust.yml` 에 `worklog_json` job 1개(ubuntu 단일 러너).
- 이 파일과 짝 `.json` — 규약의 자기증명 겸 `docs/worklog/` 개시.

## 사용자 영향
착지 후 cockpit 「후속 작업 추천」에 RustJava 카드가 처음으로 뜬다(이 회차 제안 2건).
그 전까지 수치는 바뀌지 않는다 — 소비처가 `origin/main` 을 `git archive` 로 읽기 때문이다.

## 한계 (정직한 대가)
- **규약을 심었다 ≠ 제안이 계속 뜬다.** 다음 회차들이 워크로그를 쓰지 않으면 수치는
  이 회차 2건에서 멈춘다. 「의무화할 것인가」는 아래 제안 #0 이 진다.
- 회차마다 파일 2개를 더 쓰는 부담이 는다.
- 잠금이 `cargo test` 밖(CI job)에 있어 로컬 DoD 3명령으로는 돌지 않는다 — 제안 #1.

## 후속 제안
기계 축은 짝 `.json` 의 `proposals[]` 에 있다. 요약: ⑴RustJava 회차에 워크로그 의무화 여부 결정
⑵잠금을 `cargo test` 안으로 들일지(=`serde_json` dev-dep) 아니면 DoD 문안에 1줄 추가할지 결정.
