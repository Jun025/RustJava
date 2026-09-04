# [2026-09-04] upstream 동기 S7 — 컷 `ba5797b` + §5 서식에 «델타/누적» 축

**티켓**: `rustjava-upstream-sync-s7-and-fix-the-conflict-count-format`
**컷**: `ba5797b8eb4cf376fdd63129903d319d1d7acf98`(#201 · **1커밋** · 319파일 +20,118/−5,729)
⇒ ★★**계획 「7회차」(S1~S7) 완주.**

---

## 1. 착수 재측정 — ★**신 서식으로 적은 첫 수**

> 컷 `ba5797b` 충돌 = ★**누적 1 · 델타 +1**(base `3e02f8c` · merge-base `95ebc5c`) — `java/lang/thread.rs`

★**「예측대로였다」가 아니라 「재서 1 이었다」**(§5 예측도 1 이었으나 그것을 전제로 쓰지 않았다).

★★**`string.rs` 가 집합에서 «빠졌다»** — S6 이 해소했고 이 **1커밋** 구간에서 upstream 이 안 건드렸다.
⇒ ★**누적은 «줄어들 수도» 있다.** 이것이 「누적을 델타로 대신할 수 없다」의 실측 근거이고,
§5 서식 절의 **본보기**로 그대로 넣었다.

## 2. ★§5 정본 서식 수정 (이 티켓의 «절반»)

| | 문면 |
|---|---|
| **전** | `**형식**: \`<수>(base <sha> · merge-base <sha>)\`` |
| **후** | `**형식**: ★**\`<수> <델타\|누적>(base <sha> · merge-base <sha>)\`**` |

+ 보강 문단: **누적/델타의 정의**(무엇을 답하는 수인가) · ★**「델타 0 은 «풀 것이 없다»가 아니다」** ·
**S6 실사고 인용**(티켓 Goal 이 실제로 그렇게 읽었다) · ★**본보기 = 위 S7 수** ·
★**「과거 기록 소급 수정 0」과 「옛 값은 대체로 누적」 한 줄**(티켓 2⒞).

★**왜 이 회차인가**: S7 이 충돌 수를 «새로» 적는 회차라 ★**서식을 고치고 그 서식으로 «본보기»를 남기는 것이
같은 순간에 된다.** S7 만 했으면 다음 회차가 같은 혼동을 반복한다.

## 3. `thread.rs` — ★**«직교»다. 「어느 쪽이 이기나」가 아니다**(계약 5 ⒜⒝⒞)

| | 규모(merge-base `95ebc5c` 기준) | 한 일 |
|---|---|---|
| ⒜ upstream `→ba5797b` | **+23/−10** | `invoke_virtual` 에 ★**«선언 클래스» 인자 추가**(#201 가상 디스패치 해석) + `<init>(Z)V` 에 `PRIVATE` |
| ⒝ 우리 `→origin/main` | **+50/−42** | PR #4 의 **수동 span** — `#[tracing::instrument]` 가 no_std 빌드를 깨서(tokio-rs/tracing#3388) `info_span!` + `.instrument(span)` 로 대체. ★**의미 변경은 3줄**이고 나머지 전부가 `async {}` 로 감싸며 생긴 **들여쓰기**다 |

⒞ ★★**직교다** — upstream 은 «호출 **인자**»를 바꾸고 우리는 «그 호출들을 **감싸는** span»을 바꾼다.
텍스트가 같은 자리에 있어 충돌했을 뿐 **의미가 겹치지 않는다** ⇒ ★**우열 판정 대상이 아니다.**
**처분**: 우리 구조(span·들여쓰기)를 **뼈대**로 두고 upstream 의 새 인자 **3곳**을 그 안에 얹었다 —
`"java/lang/Thread"`(run) · `&exception.class_definition().name()`(printStackTrace) · `"java/io/StringWriter"`(toString).
★**S1·S3·S4 에서 3회 확립된 전략과 «같다»** — 이번이 **4회째**다.

**검증**: 수동 span 2요소(`info_span!` `:269` · `.instrument(span)` `:329`) **생존** ·
`invoke_virtual` **9곳 전건 새 시그니처**(구식 잔존 0) ·
`#[tracing::instrument]` **어트리뷰트 실사용 0**(★`grep` 이 세는 1건은 «왜 수동 span 인지» 설명하는 **주석**이다).

## 4. ★★「충돌 0으로 들어온」 파손 — **4회째**이고 ★**축은 «처음»이다**

**증상**: `cargo test --all` 이 **`E0061` × 5** 로 컴파일 실패.
`test_input_stream_reader.rs:134·157`(`read`) · `test_string.rs:1035`(`getBytes`) · `:1042`·`:1117`(`getMessage`).

**근인**: upstream 이 `invoke_virtual` **공용 API 시그니처**를 4인자 → 5인자로 바꿨는데,
★**«우리 고유 테스트»**(PR #5 자산)가 **구식 4인자**로 남았다.
★★**우리 줄이라 upstream 이 안 건드렸고 ⇒ 충돌이 «날 수가 없다».**

★★**이 축은 앞 세 번과 «반대 방향»이다 — 그 구별이 이번 회차의 산출이다**:

| 회차 | 방향 | 무엇이 잡았나 |
|---|---|---|
| S3 · S5 · S6 | upstream 이 **«새 파일»**을 들여와 **우리 규격을 안 지킨다** | 정독 / 자산 서술자 grep / 테스트 |
| ★**S7** | upstream 이 **«공용 API 시그니처»**를 바꿔 **«우리 파일»이 낡는다** | ★**컴파일** |

⇒ ★**「신규 파일을 훑는다」로는 이번 것을 못 잡는다.** 다행히 **타입 검사가 잡는 축**이라 조용히 새지 않았다.

**처분**: ★**upstream 자신의 관용구를 그대로 채택**했다(발명 0) — `ba5797b` 의 upstream 테스트를 실측해 맞췄다:
`read`·`getMessage` → `&x.class_definition().name()` · `getBytes` → `"java/lang/String"`.
★**단언은 한 줄도 안 건드렸다**(계약 7 의 «약화 0» 축).

## 5. 「해소분 0」 · `Cargo.lock` (계약 6)

**⒜⒝ 정독** — 위험은 **양쪽이 둘 다 만진 파일**에만 있다: `comm -12`(우리 **52** ∩ upstream **319**) ⇒ ★**15건**만 정독.
전건 우리 자산 생존: `charset` 라우팅(`string.rs` **4** · `input_stream_reader.rs` **3**) ·
`setProperty` 서술자 `String`(`system.rs`·`jvm.rs`) · `double_must_use` allow **7곳**(`jvm.rs` 6 · `interpreter.rs` 1).
`ba5797b` 대비 ★**삭제 파일 0건** · 다른 파일 **52건 전수가 우리 fork 고유 자산**.

**⒞ `Cargo.lock`** — ★**내려간 0 · 올라간 0 · 추가 0 · 제거 0**(`async-trait` **0.1.92 유지**).
이 컷은 의존 그래프를 건드리지 않는다 ⇒ S5 를 문 하강 압력이 **애초에 없었다**.

## 6. 스위트 (계약 7)

| 명령 | stable | ★beta |
|---|---|---|
| `cargo fmt --all -- --check` | **0** | — |
| `cargo clippy --all -- -D warnings` | **0** | **0** |
| `cargo clippy --workspace --exclude test_utils --target wasm32-unknown-unknown -- -D warnings` | **0** | **0** |
| `cargo test --all` | **0** — **554 / 0 / 1** | **0** — **554 / 0 / 1** |
| `python3 scripts/check-worklog-json.py` | **0** | — |

★★**시험 수 증감이 «0» 이고, 그것이 «맞다»**(계약 7 이 출처를 요구한 자리):
upstream 자신의 테스트 함수도 `95ebc5c` **547** → `ba5797b` **547** 이다 —
`ba5797b` 는 78개 테스트 파일을 **+14,730/−3,536** 로 만지지만 ★**시그니처 스윕**이지 테스트 «추가»가 아니다.
⇒ ★**「554 → 554」는 «아무 일도 안 일어났다»가 아니라 «스윕이 통과했다»이다.**

★**약화 축 0**: `#[ignore]` **1 → 1** · 우리 테스트 함수 **558 → 558** · 단언 삭제 **0** · `#[should_panic]` 조작 0.
★**S4·S5·S6 축 생존**: 픽스처 4파일 · `tests/test_class_format.rs` **4/4** · `tracing-attributes` **0**.

## 7. 계보 (계약 4 · 넷 다)

| 축 | 값 |
|---|---|
| 머지커밋 부모 | ★**2개** — `3e02f8c` + `ba5797b` |
| `merge-base origin/main upstream/main` | ★**`95ebc5c` → `ba5797b`** |
| behind | ★**13 → 12** |
| `-s ours` | **쓰지 않았다**(merge-base 가 이미 `95ebc5c` 로 서 있었다 — S5 이후 3회 연속) |

★★**게이트③이 `--squash` 면 이 전진이 통째로 사라진다** — `<id>-merge` 에 **`merge_strategy: merge` 필수**.

## 8. 경계 준수

머지 **0**(PR 제출까지) · 새 PR **1**(base `main` · 스택 아님) · force-push **0** · 리베이스 **0** ·
`main` 직접 push **0** · ★**S8 집행 0**(총괄 보류분 · 남은 behind 12 = 개명 스윕) ·
upstream 코드 «개선» **0**(수렴만 — 5곳은 upstream 관용구 채택) · ★**§5 과거 기록 소급 수정 0** ·
시크릿 출력 **0** · upstream 발신 **0** · 맨 `grep` **0**(전건 `/usr/bin/grep`).
