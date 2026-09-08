# 2026-09-08 — 「모르는 도구」는 red 다 · 단 «등록 한 줄»로 지나간다

채택 = `2026-09-08-parity-axis-a-tool-name#p0`. 티켓 = `rustjava-parity-unknown-tool-step-red-decision`.

## 결정

> ★**`CHECK_TOOLS` 에도 `SETUP_TOOLS` 에도 «없는» 도구를 부르는 `run:` step 은 FAIL 이다.**
> ★**등록은 한 줄** — 진짜 검사면 `CHECK_TOOLS`(+ DoD 블록), 셋업이면 `SETUP_TOOLS` 에 «왜»와 함께.

세 선택지 중 **⒞ 조건부**. ⒜(무조건 red)·⒝(찍기 유지)를 버린 이유는 §4 ⑺ 에 둘 다 적었다.

## ★수가 설계를 고쳤다 — 발권 소견의 전제가 반증됐다

발권 소견은 「목록 밖이 **0** 이면 ⒞ 가 값한다(비용 0)」였다. ★**재보니 «1»이다**:
`rust.yml` 의 `run:` step **7** · 축 A **6** · 목록 밖 **1 = `git`**(`git config --global core.autocrlf false` · windows 줄끝 셋업).

⇒ ★**「목록 밖 = 즉시 red」로 갔으면 착지 첫날부터 «옳은 워크플로»가 red 다.**
★「목록 밖」은 한 가지가 아니라 **둘**이었다 — DoD 가 **몰라야 할 셋업**(`git`) ↔ DoD 가 **배웠어야 할 미등록 검사**(`npm`).
그 둘을 가르는 것이 `SETUP_TOOLS` 이고, ⇒ ★**⒞ 는 «⒜의 완화»가 아니라 «⒜가 성립하는 유일한 형태»다.**

## 개악 대조 12종 · 위양성 0

RED 6종: **N1 지정 개악(`npm test`)** · N2 `make` · N3 `if:` 아래 숨김 · N6 DoD 쪽 거울 · N7 `sh -c` 겹 · **M2 반대 개악(cargo step 삭제)**.
그리고 선행 회차의 지정 개악 **M1b** 도 여전히 red = ★**기존 축 무손상**.
GREEN: N1' 원복 · ★**N4·N5 등록 경로 «둘 다»** · 무해 편집 3종.
전문 표 = `docs/upstream-sync-approach.md` §4 ⑺.

## ★선행 술어를 다시 만들지 않았다

`git diff --numstat f133cd30 -- scripts/check-dod-ci-parity.py` → ★**`33  0`**(추가만 · 삭제 0).
`parse_ci`·`tool_of`·`CHECK_TOOLS`·축 A/B 비교는 **한 줄도 안 건드렸다**.

## 천장 — ★알고 둔다

- `sh -c 'npm test'` 는 도구가 `sh` 로 읽혀 **red 가 난다**(N7 실측). 다만 `sh` 를 등록해 버리면 그 겹 아래는 다시 안 보인다 —
  ★**등록 줄 자체가 리뷰 대상이다.**
- 블록 스칼라의 도구는 **첫 줄**로만 읽는다(선행 회차 그대로).
- **새 워크플로 0 · 새 잡 0** — 판정은 기존 `dod_parity` 잡의 rc 에 얹었다.

## ★이 회차가 «고친» 내 결함 하나

직전 회차(PR #33)가 §4 를 편집하며 `## 5. 단계 분할` **제목을 삼켰다**(Edit 의 `old_string` 에 넣고 `new_string` 에서 안 되살렸다).
그 사이 판본은 §5 가 ⑹ 문단 꼬리에 붙어 있었다. ★**이 회차가 복구했다** — 숨기지 않고 적는다.
