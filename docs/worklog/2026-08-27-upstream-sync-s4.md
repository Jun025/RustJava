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

복원 = `git merge -s ours --no-ff 822504b` · 트리 무변경 **0줄** 실측 · `merge-base` → `822504b`.
충돌 2건 = `java_runtime/src/classes/java/lang/thread.rs` · `jvm/src/jvm.rs`.

★**계획서 예측이 또 빗나갔다 — 이제 3회 연속이다**(S3 +9↔11 · S4 0↔2). **예측은 하한이다.**

## 해소

| 파일 | 처분 | 무엇을 살렸나 |
|---|---|---|
| `jvm/src/jvm.rs` | **합집합** | upstream 신규 `load_bootstrap_class()` + 우리 `#[allow(clippy::double_must_use)]`(PR #14). 두 변경이 «같은 자리»에 왔을 뿐 의미 충돌 0 |
| `java/lang/thread.rs` | **양쪽 병합** | upstream 의 `GlobalRef<Thread>` 전환(`this: GlobalRef<Thread>` · `(*self.this).clone()` · `new_global_ref`) + 우리 `tracing::info_span!` 수동 span(PR #4) |

★**`thread.rs` 는 S1·S3·S4 «세 회차 연속» 충돌이다** — upstream 이 `ThreadStartProxy::call` 을 반복 재작성한다.
전략은 불변: **upstream 본문을 뼈대로 취하고 `#[tracing::instrument]` 한 줄만 수동 span 으로 치환**한다.

## 들여온 upstream 회귀 — `test_timer_periodic`

3트리 대조(각 8~10회):

| 트리 | 결과 |
|---|---|
| `origin/main`(S3 착지본) | **10/10 pass** |
| ★**순정 upstream `3296139`** | `RUNCOUNT` 6·3·2·2·3·4·2·5 ⇒ **3/8 실패** |
| 우리 S4 머지결과 | `RUNCOUNT` 2~3 ⇒ 2~4/10 실패 |

⇒ ★**우리 해소 탓이 아니다.** 수동 span 을 «제거한» 프로브도 같은 비율이라 그 축이 아님이 실측됐다.
근인 후보 = `e557673`(JNI-style global references)이 GC 에 전역참조 스캔을 추가해 TimerThread 1회전이
**~110~150ms** 로 늘어난 것(500ms 창에서 기대 10회 → 실측 2~6회).

**처분 — 여백만 넓혔다**: `Thread.sleep(500)` → `2000`. ★**`assert!(run_count > 2)` 단정은 불변** ·
`#[ignore]` 추가 0 · 테스트 삭제 0. 2000ms 에서 `RUNCOUNT` 13~18 로 **10/10 pass**.
upstream 자신이 같은 자리를 두 번 넓혔다(`895d67d Fix timer periodic test flakiness` · `ad8b477 Add more margin`)
⇒ 선례에 맞는 처분이다. ★**성능 회귀 그 자체는 남아 있고 upstream 발신이 필요하므로 이 리니지 밖이다.**

## 검증
CI `rust.yml` 4종 **전건 rc=0** · `cargo test --all` **261 passed / 0 failed / 1 ignored**(S3 216 → +45).
「해소분 0」 = upstream `3296139` 대비 **삭제 파일 0** · 다른 파일 **37건 전수가 우리 fork 고유 자산**.
