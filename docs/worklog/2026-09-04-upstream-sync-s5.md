# [2026-09-04] upstream 동기 S5 — 컷 `c4665b0` (Java 1.2 API 확장 · 충돌 3)

**티켓**: `rustjava-upstream-sync-s5-with-remeasured-conflicts`
**컷**: `c4665b0620bae7b8859b22589b76c205f924ce3c`(#190 · 2026-07-25 · 6커밋 · 171파일 +33,138/−1,058)

---

## 착수 실측 — ★§5 상시 규칙대로 base 를 병기한다

| 축 | 값 |
|---|---|
| base(착수 시 `main`) | **`1983d9f`** |
| `merge-base origin/main upstream/main` | **`3296139c`** |
| behind | **30** |
| 착수 시 충돌 재측정 | **3건** — `Cargo.lock` · `string.rs` · `test_timer.rs` |

⇒ ★**§5 「[2026-09-03 재측정]」 표와 일치**했다. 그 사이 PR #19·#20 이 착지했으나 **둘 다 문서 회차**라 수가 안 바뀌었다.
★그래도 재측정을 건너뛰지 않았다 — 「upstream/main 이 계속 전진하기 때문」이 그 의무의 현행 근거다.

---

## 충돌 3건 처분

### 1. `Cargo.lock` — **재생성**

§2 처분표 #1 대로 손 머지 0. upstream 판본을 취한 뒤 `cargo build` 로 다시 만들었다(+22/−15).

### 2. `java_runtime/src/classes/java/lang/string.rs` — ★★**설계 판단**

**⒜ 양쪽이 «각각 무엇을 했나»**

| | 규모 | 한 일 |
|---|---|---|
| upstream `3296139`→`c4665b0` | **+285/−31** | Java 1.2 API 확장 — `copyValueOf(char[])`·`copyValueOf(char[],int,int)` **신설**, `compareTo(Object)` 브리지 추가, 전 메서드에 `MethodAccessFlags::PUBLIC` 부여(`ba5797b` 스윕의 선행분), `valueOf` 계열 정비 |
| 우리 `3296139`→`origin/main` | **+8/−28** | charset 해석을 **로컬 하드코딩 표 → 공용 `charset::Charset`** 로 라우팅(4 호출부)하고 그 표(`decode_str`/`encode_str`) **20줄을 삭제** |

**⒝ 우리 8줄이 «무엇을 지키려던 것인가»** — ★**charset 해석의 «단일 출처»**다. 셋으로 갈라 적는다:
1. **ISO-8859-1·US-ASCII 가 `String` 과 `InputStreamReader` 양쪽에서 «같은 표»로 동작**할 것(PR #5 의 목적).
   표가 두 벌이면 한쪽만 고쳐져 조용히 갈린다.
2. **미지원 charset 의 `UnsupportedEncodingException` throw 지점을 `Charset::resolve` 한 곳으로 모을 것**
   (명시 charset 경로 = `new String(byte[],String)`·`getBytes(String)`).
3. **기본 charset 경로**(`new String(byte[])`·`getBytes()`)는 `Charset::from_name` 이 실패하면 **UTF-8 폴백** — JDK 규격이 그렇다.

**⒞ upstream 판본에서 그것은 ⓘ충족되나 / ⓙ다시 얹어야 하나 / ⓚ불필요해졌나** → ★**ⓙ 다시 얹어야 한다.**
근거는 둘이고 **둘 다 실측**이다:
- ★`git ls-tree c4665b0 -r --name-only | grep -i charset` → **0건** ⇒ **upstream 에 `charset` 모듈이 «없다».**
- ★upstream `c4665b0` 의 `string.rs` 에 `decode_str`(:1078)·`encode_str`(:1087)이 **그대로 있다** ⇒ 삼키지 않았다.

★★**그런데 «얹기»는 이미 되어 있었다 — 진짜 판단은 «버릴 것인가»였다.**
충돌면은 파일 **끝 블록 한 곳**(hunk 1개)뿐이고, 그 안에 upstream 의 **`copy_value_of` 2종(신규 API)** 과
**되살아난 `decode_str`/`encode_str`** 가 **함께** 있었다. 반면 그 표를 쓰던 **4개 호출부는 충돌 없이
자동병합돼 우리 `Charset` 라우팅을 유지**했다(`:240`·`:414` = `Charset::from_name` · `:744`·`:784` = `Charset::resolve`).

⇒ ★**처분: upstream 의 `copy_value_of`·`copy_value_of_range` 는 «취하고», `decode_str`/`encode_str` 는 «버렸다».**
★**통째로 취했으면 표 2함수가 dead code 로 남아** §5 의 S3 완료조건(「`charset.rs` 배선으로 dead code 0」)을 깼고
`clippy -D warnings` 가 red 를 냈을 것이다.

★★**이것이 티켓이 경고한 「union 자동병합」의 실례다 — 형태를 기억하라**:
**충돌면은 «표»에 났는데, 의미가 갈린 곳은 «충돌하지 않은 호출부»였다.** 충돌면만 보고 한쪽을 고르면 그 둘이 어긋난다.

### 3. `java_runtime/tests/classes/java/util/test_timer.rs` — ★**`upstream 채택`. 예측이 반증됐다**

티켓·§5 는 이 건을 「**되얹기**(S4 의 500→2000ms 여백을 upstream 판본 위에 다시 얹는 기계 작업)」로 예측했다.
★**실측은 다르다** — upstream 이 **파일을 통째로 바꿨다**(+626/−46):

| | 우리(HEAD) | upstream(`c4665b0`) |
|---|---|---|
| 테스트 | `test_timer` · `test_timer_periodic` **2건** | `timer_*` **12건** |
| 시간 축 | `Thread.sleep(2000)` 뒤 `run_count > 2` — **벽시계** | ★**manual clock**(`TestRuntime::new_with_queued_spawns_and_manual_clock`) + **queued spawn**(`next_spawn`) + **monitor notification**(`object_wait_prepare`/`object_wait`) |
| 커버리지 | one-shot + fixed-rate 2케이스 | one-shot · fixed-rate 따라잡기 · fixed-delay · cancel · min-heap · 재사용 거부 · 오버플로 2종 · Date 스케줄 · 예외 종료 … |

★**`Thread.sleep` 기반 단정 0건**이다(`TEST_BARRIER_TIMEOUT` 은 **행(hang) 방지용**이지 타이밍 단정이 아니다).
우리 2건은 upstream 12건의 **부분집합**이다.
⇒ ★**되얹을 자리가 «사라졌다».** 2000ms 를 다시 넣는 것은 upstream 이 방금 없앤 벽시계 의존을 **되살리는 것**이라
이 회차의 목적(수렴)과 정반대다.

★★**S4 가 「남는 별 축」으로 지목한 «우리 테스트의 시간 의존»은 이로써 «소멸»했다 — 그 축으로 발권하지 마라.**

★**단 티켓이 「왜 2000 인지의 근거를 지우지 마라」고 못박았다. 코드 줄은 사라졌으므로 원장에 보존한다**
(S4 회차 문안 인용 · `docs/upstream-sync-approach.md` §5 착지 기록에도 같은 인용을 넣었다):

> 500ms 창은 **컷 이전부터** 기대 ~10회 대비 `run_count` **3~4**만 냈다(조건 맞춘 교대 실행에서 컷 전후 mean **3.5** 동일).
> upstream 이 같은 여백을 **두 번**(`895d67d` 2025-08-20 · `ad8b477` 2025-10-04) 이미 넓혔다 —
> ★**지목된 커밋보다 11개월 앞서 이미 만성 flaky 였다.** `> 2` 단정은 **불변**이고 창만 넓혔으며,
> 대가는 감도였다(red 문턱 1회전 **~167ms → ~667ms** · 약 4배 둔화).

---

## ★★충돌 «목록에 없던» 파손 1건 — §4 가 경고한 형태가 실제로 났다

**증상**: `cargo test --all` 에서 **`NoSuchMethodError` 3건**
(`bio_06_bio_07_buffered_writer_contract` · `ps_02_uses_default_encoding…` · `pw_04_println_uses_line_separator…`).
★**충돌 마커는 한 줄도 안 떴다.**

**근인**: upstream 신규 io 테스트가 `java/lang/System.setProperty` 를 **`)Ljava/lang/Object;`** 로 부르는데,
우리는 **PR #5 에서 JDK 규격대로 `)Ljava/lang/String;`** 으로 고쳐 뒀다(`System.setProperty` 는 이전 값을 `String` 으로 돌려준다).

**처분**: ★**서술자만 `String` 으로 맞췄다 — 5곳**
(`test_print_writer.rs` 1 · `test_buffered_streams.rs` 1 · `test_print_stream.rs` 3).
값은 전부 `let _:` 로 버려지므로 **바인딩 타입은 무접촉**이다.

★★**새 처분이 아니다 — 앞 회차가 이미 같은 결정을 했다.** `test_boolean.rs`·`test_integer.rs`·`test_long.rs` 가
**각 2곳씩** 같은 형태로 이미 `String` 으로 되어 있었고(그래서 이번엔 자동병합으로 통과했다),
`git diff c4665b0` 로 보면 그 세 파일이 정확히 **`+2/−2`(Object→String)** 다.
⇒ ★**내가 발명한 예외가 아니라 이 리니지의 확립된 처분을 «같은 자리에» 적용한 것**이다.

★**`java/util/Properties.setProperty` 의 `Object` 반환은 JDK 규격상 옳다 ⇒ 무접촉**
(`test_properties.rs` 의 `Object` 서술자 5곳 그대로).

---

## 계보 — ★이 repo 가 «네 번» 잃었던 것

| 축 | 착지 전 | 착지 후 |
|---|---|---|
| `merge-base origin/main upstream/main` | `3296139c` | ★**`c4665b0`** |
| behind | **30** | ★**24** |
| 머지커밋 부모 | — | ★**2개**(`1983d9f` + `c4665b0`) |

★**`-s ours` 는 쓰지 않았다** — `merge-base` 가 이미 `3296139c` 로 서 있어(PR #18·#19·#20 이 `--merge` 로 착지) 복원할 것이 없었다.
⇒ 계약 5(무해성 근거)는 **해당 없음**.
★★**게이트③이 `--squash` 면 이 전진이 통째로 사라진다** — `<id>-merge` 티켓에 **`merge_strategy: merge` 필수**.

---

## green

| 명령 | rc |
|---|---|
| `cargo fmt --all -- --check` | **0** |
| `cargo clippy --all -- -D warnings` | **0** |
| `cargo clippy --workspace --exclude test_utils --target wasm32-unknown-unknown -- -D warnings` | **0** |
| `cargo test --all` | **0** — ★**427 passed / 0 failed / 1 ignored** |
| `python3 scripts/check-worklog-json.py`(신설 4번째 DoD 명령) | **0** |

★**새 red 0**: 착수 baseline **261 passed / 0 failed / 1 ignored** → 착지 **427 / 0 / 1**(**+166**, 실패 증가 0).
★S1 169 → S2 191 → S3 216 → S4 261 → **S5 427**.

**「해소분 0」 증명**(S4~S7 공통 완료조건): `c4665b0` 대비 **삭제 파일 0건** ·
다른 파일 **47건 전수가 우리 fork 고유 자산**(원장 8 · worklog 12 · CI 3 · charset·string·system·thread·lib ·
픽스처 5 · 우리 테스트 8 · `interpreter.rs` clippy allow · `jvm.rs` · `test_class_format.rs` · `Cargo.*`).
★`git grep 'tracing::instrument\|tracing-attributes'` **실사용 0**(thread.rs 의 «왜 수동 span 인지» 주석 1건뿐) ·
`tests/test_class_format.rs` **4/4**.

---

## 경계 준수

머지 **0**(PR 제출까지) · 새 PR **1**(base `main` · ★스택 아님) · force-push **0** · 리베이스 **0** ·
`main` 직접 push **0** · S6·S7 **무접촉**(이 회차는 S5 하나) · upstream 코드 «개선» **0**(수렴만) ·
upstream(`dlunch/RustJava`) 발신 **0** · 맨 `grep` **0**(전건 `/usr/bin/grep`) ·
§5 재측정 표 **재작성 0**(인용 + 착지 기록 절 신설).
