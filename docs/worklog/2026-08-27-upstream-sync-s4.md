# S4 — upstream 컷 `3296139` 머지 (물량 회차 · GlobalRef / CLI classpath / CDC text)

티켓 `rustjava-upstream-sync-s4`. 정본 = `docs/upstream-sync-approach.md` §5(7회차 · 한 티켓 = 한 컷).

## 기준선 재측정 (착수 시점 2026-08-26T22:50Z)

| 축 | 값 |
|---|---|
| `origin/main` tip | `4bb796d`(S3 착지) |
| `upstream/main` tip | `ba5797b`(불변) |
| `merge-base origin/main upstream/main` | ★**`62cf0c6`** — 최초 공통조상 |
| `1f356ae`·`af4f6f8`·`822504b` 가 `origin/main` 의 조상? | ★**전건 NO** |
| `rev-list --left-right --count origin/main...upstream/main` | `18  33` |

⇒ ★**스쿼시 3회(#11·#13·#16)가 족보를 원점으로 되돌렸다.** base 는 `origin/main` 으로 잡되(선행 PR 0건이라
스택할 이유가 없다) **조상 복원이 선행 조건**이다.

## 충돌 — 예측 대 실측

| | 계획서 §5 예측 | 복원 «전» 실측 | 복원 «후» 실측 |
|---|---|---|---|
| S4 새 충돌 | **0** | **20** | ★**2** |

복원 = `git merge -s ours --no-ff 822504b` · `merge-base` → `822504b`.
★**무해성 근거는 «`git diff --stat origin/main HEAD` 빈 출력»이 «아니다»** — `-s ours` 는 정의상 우리 트리를
유지하므로 항상 참이다. 근거는 ★**`--diff-filter=D` 0 + 충돌 2파일 양방향 전문 대조**다.
충돌 2건 = `java_runtime/src/classes/java/lang/thread.rs` · `jvm/src/jvm.rs`.

★**계획서 예측이 또 빗나갔다 — 이제 3회 연속이다**(S3 +9↔11 · S4 0↔2). **예측은 하한이다.**

## 해소

| 파일 | 처분 | 무엇을 살렸나 |
|---|---|---|
| `jvm/src/jvm.rs` | **합집합** | upstream 신규 `load_bootstrap_class()` + 우리 `#[allow(clippy::double_must_use)]`(PR #14). 두 변경이 «같은 자리»에 왔을 뿐 의미 충돌 0 |
| `java/lang/thread.rs` | **양쪽 병합** | upstream 의 `GlobalRef<Thread>` 전환(`this: GlobalRef<Thread>` · `(*self.this).clone()` · `new_global_ref`) + 우리 `tracing::info_span!` 수동 span(PR #4) |

★**`thread.rs` 는 S1·S3·S4 «세 회차 연속» 충돌이다** — upstream 이 `ThreadStartProxy::call` 을 반복 재작성한다.
전략은 불변: **upstream 본문을 뼈대로 취하고 `#[tracing::instrument]` 한 줄만 수동 span 으로 치환**한다.

## `test_timer_periodic` 여백 — ★«회귀»가 아니라 «만성 경계 테스트»다

★**이 절의 초판은 「컷이 들여온 회귀」로 적었고 그것은 틀렸다.** 근인은 «수»가 아니라 ★**«측정 조건»**이었다 —
`origin/main` **10/10**(★**단독 실행**)과 순정 upstream **3/8 실패**(★**전 스위트 병렬**)를 나란히 놓았다.

**조건을 맞춘 교대 실행 재측정**(drift 제거):

| 조건 | `4bb796d`(컷 **전**) | `3296139`(컷 **후**) |
|---|---|---|
| **단독 실행** · 교대 10회 | `3 3 3 3 4 4 4 4 3 4` · mean **3.5** | `4 3 4 3 4 3 4 3 4 3` · mean **3.5** |
| **전 스위트 병렬** · 교대 8회 | `4 4 4 4 3 4 3 4` | `4 3 3 6 4 4 4 4` |

⇒ ★**두 조건 어디서도 차이가 없다.** 500ms 창에서 기대 10회 대비 **3~4회**는 ★**컷 이전부터 그랬다.**

★**사료가 그 자체로 반증이다**: upstream 이 같은 자리를 넓힌 `895d67d`(**2025-08-20** "Fix timer periodic test
flakiness")·`ad8b477`(**2025-10-04** "Add more margin to timer test")는 ★**둘 다 이미 `origin/main` 의 조상**이고,
근인으로 지목했던 `e557673`(GlobalRef)은 **2026-07-18** 이다 ⇒ ★**11개월 앞서 이미 만성 flaky 였다.**

**처분은 그대로다**: `Thread.sleep(500)` → **`2000`** · ★**`assert!(run_count > 2)` 불변** · `#[ignore]` 0 · 삭제 0.
성격만 정정한다 — ★**「가리는 여백」이 아니라 «만성 경계 테스트에 정상 여백을 준 것»**이다.

★**대가를 숨기지 않는다 — 감도가 내려갔다.** 제품 호출부(`timer_thread.rs::run()`) 돌연변이 실측:
「루프 sleep 16ms → 700ms(**5.6배** 저하)」는 여전히 **red**, 「→ 300ms(**2.4배**)」는 ★**이제 통과**한다.
red 문턱이 1회전 **~167ms → ~667ms**(약 **4배** 둔화)다. 그 상한을 테스트 주석에 박았다.
★**시간 의존 자체를 없앨지는 별 판단이다** — `proposals[0]`.

## 검증
CI `rust.yml` 4종 **전건 rc=0** · `cargo test --all` **261 passed / 0 failed / 1 ignored**(S3 216 → +45).
「해소분 0」 = upstream `3296139` 대비 **삭제 파일 0** · 다른 파일 **39건**(핀 `1a90e7e` 기준) **전수가 우리 fork 고유 자산**.
