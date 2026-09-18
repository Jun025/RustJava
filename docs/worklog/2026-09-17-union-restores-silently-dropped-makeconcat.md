# 2026-09-17 — 합집합이 «조용히 떨어진» makeconcat 가족을 되살렸다 (rustjava-adopt-link-stringconcatfactory-p2-fix3)

게이트③이 `code-conflict-out-of-scope` 로 세운 PR #61 의 충돌 4파일(원장 2 + 코드 2)을 합집합으로 해소했다.
제품 Rust **0줄** — 바뀐 것은 테스트와 픽스처 생성기다.

## ★브리프의 전제 하나가 반증됐다

브리프는 「ours 가 «의도적으로» 지운 54·16줄을 되살리지 마라」고 했고, 대전제 ⓒ 가 그 의도를 확인하라고 했다.
확인 결과 ★**두 파일의 성격이 정반대**다.

| 파일 | ours 삭제 | 정체 | 처분 |
|---|---|---|---|
| `test-data/src/indy/make_indy_fixtures.py` | 54줄 | ★**전건 makeconcat 가족** — ours 가 지운 적이 없다 | ★**되살렸다** |
| `tests/test_class_format.rs` | 16줄 | ★**진짜 ours 의도** | ★**되살리지 않았다** |

**⒜ `.py` 54줄은 선행 머지의 잔재다.** 이 브랜치의 머지 커밋 **둘**을 각각 부모별로 세면 이렇게 나온다:

```
e53b2142 [-p2-fix]   부모1 make_concat=0 fieldref=0 · 부모2 =1 =1  → 결과 =0 =0
514d5b08 [-p2-fix2]  부모1 make_concat=0 fieldref=0 · 부모2 =1 =1  → 결과 =0 =0
```

★**두 회차 모두 «부모2에 있던 것»을 결과에서 떨어뜨렸다.** 지운 대상은 `Pool.fieldref` ·
`MAKECONCAT_DESCRIPTOR` · `make_concat_call_site()` 전문 · `LINKED` 표 · 그 쓰기 루프이고,
ours 의 54줄 삭제는 **그 전부이자 그것뿐**이다(다른 의도 삭제 0).

**⒝ `.rs` 16줄은 되살리면 안 된다.** ours 는 `LambdaMetafactory` 를 **링크하도록** 만들었으므로
`test_only_the_string_concat_bootstrap_is_linked` 와 `test_lambda_class_reports_unsupported_feature_not_malformed`
두 단언이 **거짓이 됐고**, ours 가 `test_only_the_two_recognised_bootstraps_are_linked` 로 대체했다.

## ★되살린 쪽이 «죽은 코드»가 아님을 실행으로 보였다

```
rm test-data/indy/{MakeConcat,MakeConcatWrongDescriptor,MakeConcatWithArgument}.class
python3 test-data/src/indy/make_indy_fixtures.py
git status --porcelain -- test-data/indy   → (없음) = 3장 모두 «바이트 동일»하게 복구
```

★**대조**: 복원 «전» 생성기(`git show HEAD:…`)에는 `LINKED`·`make_concat_call_site` 가 **0건**이라
그 3장을 **낼 수 없었다**. ⇒ 방치하고 착지시켰다면 `main` 은 **커밋된 픽스처 3장을 설명하지 못하는 생성기**를
갖게 된다 — `test-data/src/audit-fixture-single-defect.py` 가 잡으려던 바로 그 조건이고,
★**그 감사기의 `CANON` 표가 MakeConcat 가족을 덮지 않아** 잡히지도 않았을 것이다.

## 회계 — 「삭제줄 부활 0」을 «수»로

| 축 | 값 | 뜻 |
|---|---|---|
| `.py` 해소본 ↔ `origin/main` | **156 / 0** | 삭제 0 — ours 의 54 는 **되살아났다**(그것이 옳다) |
| `.rs` 해소본 ↔ ours(HEAD) | **86 / 0** | theirs 기여만 들어왔고 **ours 무손실** |
| `.rs` 해소본 ↔ `origin/main` | **182 / 29** | 그 **29줄 = ours 가 지운 2함수 전문**(그 외 0) |
| 원장 2파일 | 양방향 **정확 일치** | `REPORT 60/0·103/0` · `STATE 27/0·83/1` |

## 양방향 개악 — 두 축이 독립임을 잠근다

| 개악 | 결과 |
|---|---|
| ours 픽스처(metafactory 7장)만 치움 | ★**ours 4건만 red** · theirs `…recipe…bootstrap_method_error` **ok** |
| theirs 픽스처(`RecipeWants*` 3장)만 치움 | ★**theirs 1건만 red** · ours 전건 **ok** |

## 검증

`cargo test --all` **583 passed / 0 failed / 1 ignored** · `test_fixture_pins` **3/0** ·
기존 픽스처 **바이트 불변** · 테스트 함수 **21개**(base 15 − ours 2 + ours 6 + theirs 2, 집합 정확 일치) ·
DoD 7명령 전건 rc=0.

## ★잃는 것 — 숨기지 않는다

- 합집합은 두 축의 테스트를 **둘 다** 안고 간다(스위트가 길어진다). 나중에 두 축이 **진짜로** 충돌하면
  그때 비용이 크고, 위 양방향 개악은 «오늘 독립»만 잠근다.
- `.rs` 는 **재구성**했다(유닛을 순서대로 다시 배치). 집합·순서를 수로 확인했지만 ★**순수 텍스트 병합보다
  손이 많이 간 해소**이고, 그만큼 리뷰가 diff 가 아니라 «집합 비교»에 의존한다.
- ★**이 회차는 복원이 옳다고 «판단»했다** — 브리프 문면(「되살리지 마라」)과 어긋나므로, 그 판단이 틀렸다면
  되돌릴 곳은 `.py` 의 복원분이다. 근거는 위 실행 증명(생성기↔커밋 픽스처) 하나에 걸려 있다.

## 후속

- ★**머지 결과에서 «부모2에만 있던 심볼»이 사라졌는지 세는 검사**(S) — 이번 손실은 충돌 표시가 전혀 없는
  **깨끗한 자동 병합**이라 사람도 `mergeable` 도 못 봤다. 상세는 같은 이름의 `.json` `proposals[0]`.
