# STATE

## 진행중
- (없음)

## 완료
- [rustjava-runtime-time-todo-impl] RuntimeImpl 시간 API `todo!()` 3건 제거(now/sleep/yield) +
  test_utils `r#yield` 구현 + tokio `time` 피처 추가 + 회귀 잠금 픽스처(`test_data/TimeApi`).
  ★게이트③ 완료: PR #2 스쿼시 머지 → main `13ab950`(2026-07-23), 브랜치 정리 완료.
- [rustjava-classfile-parse-error-propagation] 클래스파일 파싱 실패를 패닉 대신
  `java.lang.ClassFormatError` 로 전파(절단/매직 불일치/미지원 상수풀 태그 구분).
  ★게이트③ 완료: PR #3 스쿼시 머지 → main `549b9eb`(2026-07-23), 브랜치 정리 완료.
- [rustjava-tracing-attributes-pin-removal] `#[tracing::instrument]` 1건을 수동 span 으로 대체,
  `tracing-attributes` 상한 핀 제거(tracing 0.1.41→0.1.44 언프리즈), wasm32 clippy CI 커버리지
  교정. ★게이트③ 완료: PR #4 approve 핀 `0a19f38` 확인 → main 충돌 해소(docs-only) 후
  스쿼시 머지(2026-07-23), 브랜치 정리 완료.
- [rustjava-unsupported-charset-exception] 미지원 charset `unimplemented!()` 패닉 3지점을
  `java.io.UnsupportedEncodingException`(신설) throw 로 전환, String↔InputStreamReader 지원
  charset 을 공용 `charset::Charset` 으로 일치(ISO-8859-1/US-ASCII 가 Reader 에서도 동작).
  부수: `System.setProperty` 반환 시그니처 JDK 규격화(Object→String, jvm 부트스트랩 포함),
  `Throwable.getMessage()` 신설, 픽스처 `test_data/UnsupportedCharset`.
  ★게이트③ 완료: PR #5 approve 핀 `dd9fcdf` 확인(이후 이동분은 문서화된 main 동기화 머지
  3건뿐, diff-of-diffs 로 코드 동일성 검증) → 스쿼시 머지(2026-07-23), 브랜치 정리 완료.
- [2026-07-25-rustjava-branch-hygiene] 원격 잔존 브랜치 2건 판정. ①
  `dependabot/cargo/tracing-attributes-0.1.31` → PR #4 로 `tracing-attributes` 의존 자체가
  소멸(Cargo.toml/lock 전무)해 패치 대상 라인 부재 → origin 에서 삭제 완료. ②
  `wie-ktf-hardening` → 12커밋 중 8건이 upstream/main 에 스쿼시 반영/대체(#174~#182),
  4건 유효 잔존 → **혼재 판정으로 보존**, 커밋별 판정표를 총괄에 제출(코드 변경 0).

## 다음

### ①(최우선) upstream 동기화 — 2026-08-15 실측
`git rev-list --left-right --count origin/main...upstream/main` → **`9  32`**
(origin 이 9 앞섬 · **32 뒤처짐**. 2026-07-25 의 「20 뒤처짐」은 낡았다.)
- 공통조상 `62cf0c6` · upstream tip `95ebc5c` · origin tip `2e61e93`.
- ★**충돌 예상 17파일**(`git merge-tree --write-tree --name-only origin/main upstream/main` 실측):
  `Cargo.lock` · `classfile/src/{class,constant_pool,error,lib}.rs` ·
  `java_runtime/src/classes/java/io.rs` · `java_runtime/src/classes/java/io/input_stream_reader.rs` ·
  `java_runtime/src/classes/java/io/unsupported_encoding_exception.rs`(add/add) ·
  `java_runtime/src/classes/java/lang.rs` · `.../java/lang/string.rs` · `.../java/lang/thread.rs` ·
  `java_runtime/src/loader.rs` · 테스트 2건 · `jvm_rust/src/class_definition.rs` ·
  `src/runtime.rs` · `test_utils/src/lib.rs`.
- ★**충돌의 성격 = «우리 PR #3·#5 와 upstream 의 독립 구현이 정면 충돌»**. upstream 에도
  `class_format_error.rs` · `unsupported_encoding_exception.rs` 가 **독자적으로 존재**한다
  (`423d1bd Hide classfile errors behind class definition errors`, `fe5d116` 등).
  ⇒ 단순 «우리 것 채택»이 아니라 **의미 대조 후 선택**이 필요하다.
- ★**주의 — 이 축에서 우리가 upstream 보다 앞선 지점이 있다.** upstream
  `input_stream_reader.rs` 는 charset 을 **UTF-8/EUC-KR 만 하드코딩**하고 그 외는
  `UnsupportedEncodingException` 을 던진다. 우리 PR #5 는 공용 `charset::Charset` 으로
  일반화해 **ISO-8859-1/US-ASCII 도 동작**한다. 머지 시 upstream 판본을 그대로 취하면 **퇴행**이다.

### ②`wie-ktf-hardening` 잔존분 — 2026-08-15 재판정으로 **4건 → 2건**
★**선행 확인 종결**: upstream `agent/runtime-api-gaps`(`6309d47`)는 **미머지가 아니다** —
**PR #190 로 2026-07-25 04:59Z 스쿼시 머지**(머지커밋 `c4665b0`, +33,109/−1,040)됐고
그래서 브랜치가 upstream 에서 **삭제**됐다. 즉 「삼키는지」는 이제 **upstream/main 에 직접 묻는다**.
항목별 판정(전부 `upstream/main` 트리 실측):

| 항목 | upstream/main 현재 | 판정 |
|---|---|---|
| `Timer.schedule(TimerTask;J)V` 1회성 | `timer.rs` `schedule_once` 존재 | **삼킴 → 무효** |
| `StringBuffer.insert(ILjava/lang/String;)` | `insert_string` 존재 | **삼킴 → 무효** |
| `StringBuffer.append([CII)` null→NPE | null 가드 **+ 범위 가드**까지 존재 | **삼킴 → 무효** |
| `ByteArrayInputStream.<init>([B)` null 가드 | `is_null()` 가드 존재(`be77dc6`) | **삼킴 → 무효** |
| `Class.forName` null/not-found 가드 | 둘 다 존재 | **삼킴 → 무효** |
| `Integer.byteValue()/shortValue()` | `java/lang/Number` 가 **구현 제공**, Integer 가 상속(`#176`) | **삼킴 → 무효** |
| `System.arraycopy` null 가드 | **부재** | ★**유효 잔존** |
| `String.<init>([B)` / `([C)` null 가드 | **부재** | ★**유효 잔존** |

★유효 잔존 2건의 **파괴력 근거**(추정 아님): `ClassInstanceRef::deref` 가
`self.instance.as_ref().unwrap()` 이라 **null 참조를 넘기면 Rust 패닉 = 호스트 프로세스 abort**다
(`jvm/src/class_instance.rs`). `System::arraycopy` 는 `jvm.load_array(&src, …)`,
`String::init_with_byte_array`/`init_with_char_array` 는 `jvm.array_length(&value)` 로 곧장 deref 한다.
※**형제 호출부도 같이 봐라**: `String::init_with_partial_byte_array` 도 가드가 없다
(반면 `init_with_partial_char_array` 는 upstream 이 이미 가드를 넣었다 — 짝이 안 맞는다).
※**전역 수리는 불가** — `Deref` 는 `Result` 를 못 돌려준다. upstream 방식(진입부 `is_null()` 가드)이 정답이다.

### ③다음 회차 발권 후보(우선순위 순)
1. **`rustjava-upstream-sync-32-commits`**(P1·L·high) — ①의 머지. 완료 정의 = 충돌 17파일 해소 +
   `cargo fmt --check`/`clippy`/`test` green + **PR #5 의 charset 일반화 퇴행 0**(회귀 픽스처
   `test_data/UnsupportedCharset` 통과로 잠근다).
2. **`rustjava-null-guard-arraycopy-and-string-init`**(P2·S·low) — ②의 유효 잔존 2건.
   ★**①의 뒤**여야 한다(같은 파일이 충돌 목록에 있다). 재현 =
   `System.arraycopy(null,0,dst,0,0)` · `new String((byte[])null)` · `new String((char[])null)`
   각각 현재 **패닉**, 기대 `NullPointerException`. 형제 `String.<init>([BII)` 도 함께.
3. **`rustjava-invokedynamic-and-cp-tags-15-18`**(P2·M·med) — 아래 미해결 항목.

### ④미해결 — ★「이미 처리됐다」는 통설은 실측으로 **거짓**이다
- ★`jvm_rust/src/interpreter.rs` `Opcode::Invokedynamic(_) => todo!()` 는
  **origin/main·upstream/main 양쪽에 그대로 살아 있다**(2026-08-15 실측).
  `rustjava-classfile-parse-error-propagation` 은 **클래스파일 파싱** 경로를 고쳤을 뿐
  **인터프리터 opcode 는 건드리지 않았다** — 이 축은 미해결이다.
- ★javac 21 익명 내부 클래스 "Malformed class file": 미해결. 원인 후보가 좁혀졌다 —
  `classfile/src/constant_pool.rs` 의 태그 분기가 **origin·upstream 모두 1~12 까지뿐**이고
  **15(MethodHandle)·16(MethodType)·17(Dynamic)·18(InvokeDynamic) 분기가 없다.**
  javac 9+ 는 문자열 `+` 연결조차 invokedynamic 으로 낸다.
  ⇒ STATE 구판의 「태그 15~18 아님」은 **근거 없이 배제한 것**으로 보인다. 위 invokedynamic 축과 **한 티켓으로 묶어라**.
- ★InputStreamReader 디코더: **origin/main 에서 미해결**(read 마다
  `Charset::resolve(...).new_stream_decoder()` 로 새로 만든다). `rustjava-unsupported-charset-exception`
  이 고친 것은 **미지원 charset 패닉**이지 **경계 유실**이 아니다 — 별개 결함이다.
  ※단 upstream 은 완화책을 갖고 있다(`endOfInput` 필드 + UTF-8 lead-byte 역주사 · EUC-KR `>=0x81`
  홀드백). ⇒ **①의 머지로 함께 들어온다.** 별건 발권 전에 ① 이후 상태를 다시 재라.

### ⑤운영 메모
- 발권 대기 태스크 없음 — 열린 PR 0(2026-08-15 확인), origin 작업 브랜치는 보존 판정된
  `wie-ktf-hardening` 1건. 그 브랜치는 위 ②로 **잔존 가치가 2건까지 줄었다** —
  2 건을 새 브랜치로 옮겨 심고 나면 **보존 근거가 소멸**한다.
- ★PR 발권 시 `--repo Jun025/RustJava` 명시(2026-07-22 upstream 오발행 사고 재발 방지).
- ★upstream 발신(PR·이슈·코멘트·push)은 **티켓이 명시 허가할 때만**. 기본은 조회뿐.
