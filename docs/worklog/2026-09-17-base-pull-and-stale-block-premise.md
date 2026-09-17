# 2026-09-17 — base 당김 · ★전제 반증 (rustjava-adopt-link-stringconcatfactory-p1-fix2)

이 회차는 **「`test-data/src/indy/make_indy_fixtures.py` 4구역 코드 충돌을 합집합으로 풀어라」**로 발권됐다.
★**그 충돌은 없다.**

## 반증 — 시각으로 재구성한다

| 시각 | 무슨 일 |
|---|---|
| 12:12 | `-p1-merge` 회차가 head **`38efee33`** 에서 `blocked`(코드 충돌 4구역) |
| ★**14:10** | `-p1-fix` 회차가 **`0f06b93f`** 푸시 — 커밋 제목 그대로 「merge origin/main — **코드 충돌 1건을 «합집합»으로 해소한다**」 |
| ★**15:43** | 게이트②가 **그 head `0f06b93f` 를 approve** |

⇒ ★**발권 근거(12:12 회신)가 그 사이 낡았다.** `38efee33` 은 `0f06b93f` 의 **조상**이다.

지금 다시 재면:
```
git merge origin/main        # base 뒤처짐 9
UU REPORT.md    ← 원장
UU STATE.md     ← 원장
(make_indy_fixtures.py 는 충돌하지 않는다)
```
★**재발도 불가능하다** — 뒤진 9커밋 중 그 파일을 만진 것이 **0건**이다(「지금은 없다」가 아니라 「이 base 에서는 날 수 없다」).

## 그래도 합집합이 «진짜»인지 다시 쟀다

합집합의 전형적 실패는 **한쪽 의도가 조용히 빠지는 것**이다. 승인된 head 위에서 세 축으로 확인했다.

**⒜ 한 파일에 양쪽 진입점이 다 있다**
`recipe_arity_call_site`(ours · #60) `:108` · `MAKECONCAT_DESCRIPTOR` `:156` + `make_concat_call_site`(theirs · #59) `:159`,
픽스처 표에 **두 가족이 함께** 있다(`MakeConcat*` 3 · `RecipeWants*` 3).

**⒝ 생성기를 실제로 돌렸다** — 10개 픽스처 **전건 바이트 불변**. ★**한 생성기가 양쪽 산출물을 낸다**는 것이 합집합의 실질이다.

**⒞ 양방향 개악**

| 개악 | ours 테스트 | theirs 테스트 |
|---|---|---|
| **M1** ours 산출물 3개만 치움 | ★**red** | green ×2 |
| **M2** theirs 산출물 3개만 치움 | green | ★**red ×2** |

⇒ 「선택」이 아니라 **합집합**이다. (복원 후 바이트 불변 확인.)

## 실제로 남아 있던 것 — 충돌이 아니라 «부채»

base 를 당기면 **#57** 의 `test-data/class-file-versions.txt` 가 들어오고,
이 PR 의 픽스처 3개가 미등재라 `test_fixture_pins` 가 **red** 가 된다.
★게이트② 검수가 이미 「`-merge` 회차가 base 당김 + **표 3행**을 함께 진다」로 지목한 그것이다.

```
python3 test-data/src/record-class-file-versions.py
git diff --numstat test-data/class-file-versions.txt   → 3  0
```
★**삭제 0** = 기존 픽스처 재생성 0. 추가 3행의 경로는 이 PR 이 추가한 3개 `.class` 와 **집합 동일**.

## 원장 충돌

`REPORT.md`·`STATE.md` 최상단 삽입 2건 — **합집합·시간순**(main 측 17:09 → ours 14:10).
줄 단위 양방향 보존 확인. ★한 줄이 「main 측 결손」으로 잡히는데, 그것은 `STATE.md` 의 「진행중」 문단이고
**base == theirs** 라 3-way 가 **ours 를 택한 정상 결과**다 — 결손이 아니다.

## 검증
`test_class_format` **16/0** · `test_fixture_pins` **3/0** · `cargo test --all` **578 / 0 / 1** · DoD **7명령 전건 rc=0**.

## 대가
★이 회차가 head 를 게이트② 핀 `0f06b93f` 에서 **움직인다** ⇒ PR #60 은 **재검 대상**이다.
피할 수 없다 — 버전 표는 원장 파일이 아니라서 머지 템플릿 2-c⒜ 가 `-merge` 회차에게 그것을 허용하지 않는다.
