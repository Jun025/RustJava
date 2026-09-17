# 2026-09-17 — 버전 표 25행 백필 (rustjava-adopt-link-stringconcatfactory-p2-fix2)

PR **#61** 이 게이트③에서 막혔는데, 막은 것은 **CI 도 충돌도 아니었다**.

- 검수 핀 `85cf0fba` 에서 `ci-presence` → **rc=0 CI_GREEN**
- `git merge origin/main` → **코드 충돌 0**(원장 2파일만 충돌)
- 그런데 **합친 결과**가 `test_fixture_pins` 의
  `committed_fixtures_keep_their_recorded_class_file_version` 을 **red** 로 만든다 — 미등재 **25건**.

## 원인은 형상이 아니라 «시점»이다

PR **#57** 이 `test-data/class-file-versions.txt` 와 함께
「**미등재 픽스처는 핀을 실패시킨다**」를 **의도적으로** 세우고 06:37 에 착지했다
(그 자신의 주석이 「이미 아는 것만 검사하는 핀은 새로 추가된 것을 덮지 못한다」고 적었다 — 옳다).

#61 의 25개 픽스처는 그보다 **먼저** 만들어졌다.
⇒ **#57 이 착지한 그 순간부터 #61 은 표에 25행을 빚졌고, 아무도 그 사실을 말해 주지 않았다.**

## 한 일

```
python3 test-data/src/record-class-file-versions.py    → recorded 181 fixtures
git diff --numstat test-data/class-file-versions.txt   → 25  0
```

★**`25  0` 의 「0」이 이 회차의 안전선이다.** 생성기는 «지금 디스크에 있는 것»을 기록하므로,
픽스처가 그 사이 오염됐다면 **오염된 값을 정답으로 굳힌다**. 삭제행이 하나라도 있었으면
그것은 「기존 픽스처가 재생성됐다」는 뜻이고 **다른 사건**이라 멈췄을 자리다.

추가된 25행은 이 PR 이 만든 25개 `.class` 파일과 **집합이 정확히 같다**(남의 픽스처 혼입 0).

## 표가 실제로 규율을 집행하는가 — 양방향

| | 결과 |
|---|---|
| 표에서 `65.0 indy/LambdaKinds.class` 한 행 제거 | ★**red** — 그 파일명을 정확히 지목 |
| 되돌림 | ★**green** 3 passed / 0 failed |

한 방향만 쟀으면 「상수 pass 로 바꿔도 통과」를 못 가른다.

## base 당김 — 원장 2파일 합집합

`REPORT.md`·`STATE.md` 의 최상단 삽입 충돌 2건. 한쪽 통째 채택 **0**, 시간순 합집합.
줄 단위 양방향으로 증명했다 — 양측 고유줄 중 결과에 없는 것 **0** · 결과에만 있는 줄 **0**.

★같은 파일(`classfile/src/validation.rs`)을 다투던 **#62** 의 기여는 자동 병합 뒤에도 **전건 잔존**했다
(적재 가능 상수 집합 · 이 PR 의 서술자 팔 — **둘 다** 살아 있고, `06fa865c` 대비 diff 는 이 PR 의 추가분뿐이다).

## 검증

`test_fixture_pins` **3/0** · `cargo test --all` **581 / 0 / 1**(브랜치 단독 578 → #62 착지분 +3) ·
DoD **7명령 전건 rc=0**(`+beta` clippy · wasm32 clippy 포함).

## 남는 것

이건 **부채 상환이지 수리가 아니다** — 다음에 같은 형태의 규율이 착지하면 그때 열려 있는 PR 이
똑같이 조용히 빚을 진다. 그 축은 제안 `#p0` 으로 남겼다.
