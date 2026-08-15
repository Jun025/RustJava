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
★★**형제 호출부 «전수»**(2026-08-16 재실측 · `java/lang/String` 의 `<init>` 오버로드 **10건 전건** 대조).
가드가 **없어서 null 을 넘기면 호스트 abort** 인 것이 **7건**이다:

| `<init>` 서술자 | 구현 | null 이 닿는 자리 | 가드 |
|---|---|---|---|
| `([B)V` | `init_with_byte_array` | `jvm.array_length(&value)` | **없음** |
| `([C)V` | `init_with_char_array` | `jvm.array_length(&value)` | **없음** |
| `([BII)V` | `init_with_partial_byte_array` | `jvm.load_array(&value, …)` | **없음** |
| `([BLjava/lang/String;)V` | `init_with_byte_array_charset` | `jvm.array_length(&value)` | **없음** |
| `([BIILjava/lang/String;)V` | `init_with_partial_byte_array_charset` | `jvm.load_array(&value, …)` | **`charset_name` 만** 검사 · `value` **없음** |
| `(Ljava/lang/String;)V` | `init_with_string` | `value_range` → `jvm.get_field(this, …)` | **없음** |
| `(Ljava/lang/StringBuffer;)V` | `init_with_string_buffer` | `jvm.invoke_virtual(&value, "toString", …)` | **없음** |
| `([CII)V` | `init_with_partial_char_array` | — | ★있음 |
| `(II[C)V` | `init_with_shared_char_array` | — | ★있음 |
| `()V` | `init_empty` | 인자 없음 | 해당 없음 |

★**`get_field`/`invoke_virtual` 도 `&Box<dyn ClassInstance>` 를 받는다** — 배열 인자만의 문제가 아니다.
⇒ ★**가드가 «짝이 안 맞는» 것이 핵심이다**: `([CII)`·`(II[C)` 만 막혀 있고 `[B` 계열은 전부 뚫려 있다.
※**전역 수리는 불가** — `Deref` 는 `Result` 를 못 돌려준다. upstream 방식(진입부 `is_null()` 가드)이 정답이다.

### ③다음 회차 발권 후보(우선순위 순)
0. ★★**`rustjava-pr8-claude-md-prune-disposition`**(P1·S·low) — ★**PR #8 처분 «판정»**.
   ★**이 항목이 목록 맨 위인 이유**: 아래 1~3 은 전부 «앞으로 할 일»인데 #8 은 **이미 벌어져 멈춰 있는 일**이다.
   재료(2026-08-16 자력 실측, 판정은 하지 않았다):
   - `Jun025/RustJava#8` `[rustjava-claude-md-prune]` · head `feat/rustjava-claude-md-prune`(`7e73a1c`) ·
     `createdAt`/`updatedAt` **둘 다 `2026-08-05T07:56:55Z`** · `OPEN`/`MERGEABLE` · draft 아님.
   - ⇒ **생성 후 11일간 손대지 않은 채 열려 있다.** 커밋도 push 도 코멘트도 그날 이후 0.
   - ★**원장 비대칭**: `reports/rustjava-claude-md-prune.done.md` 는 실재하는데
     **`reports/rustjava-claude-md-prune.review.md` 가 없다** ⇒ ★**게이트②가 아예 돌지 않고 좌초했다.**
   - 내용은 `CLAUDE.md` **문서 전용**(autonomous-sop 절 삭제 + Goal/Constraints/DoD 신설).
     ★현 `main` 의 `CLAUDE.md` 에는 그 변경이 **없다** — 즉 #8 이 닫히지 않는 한 레인 헌장이 두 판본으로 갈린다.
   - ★**충돌 전망**: `CLAUDE.md` 는 위 「충돌 17파일」에 **없다** ⇒ ①머지와 독립적으로 처분 가능하다.
   ★**처분(게이트② 재발권 / 닫기 / 재작성)은 이 항목이 «판정할 몫»이다 — 여기서 정하지 않았다.**
1. **`rustjava-upstream-sync-32-commits`**(P1·L·high) — ①의 머지. 완료 정의 = 충돌 17파일 해소 +
   `cargo fmt --check`/`clippy`/`test` green + **PR #5 의 charset 일반화 퇴행 0**(회귀 픽스처
   `test_data/UnsupportedCharset` 통과로 잠근다).
2. **`rustjava-null-guard-string-init-and-arraycopy`**(P2·S·low) — ②의 유효 잔존 2건 + 형제 전수.
   ★**①의 뒤**여야 한다 — ★**근거 정정(2026-08-16)**: 선행 이유는 **`string.rs` 가 충돌 목록에 있기 때문**이다.
   ★**`system.rs` 는 충돌 목록에 «없다»**(`merge-tree` 출력에서 `Auto-merging` 만 있고 `CONFLICT` 줄이 없다) —
   구판이 두 파일 다 충돌이라고 적은 것은 오류이니 **충돌 해소 대상으로 잡지 마라.**
   - **범위**: `System.arraycopy` + ②의 표에서 **가드 없음 7건 전부**
     (`([B)` · `([C)` · `([BII)` · `([BLjava/lang/String;)` · `([BIILjava/lang/String;)` ·
     `(Ljava/lang/String;)` · `(Ljava/lang/StringBuffer;)`).
   - **재현**: `System.arraycopy(null,0,dst,0,0)` · `new String((byte[])null)` · `new String((char[])null)` ·
     `new String((byte[])null,0,0)` · `new String((byte[])null,"UTF-8")` · `new String((byte[])null,0,0,"UTF-8")` ·
     `new String((String)null)` · `new String((StringBuffer)null)` → 전부 현재 **패닉**, 기대 `NullPointerException`.
   - **완료 정의**: 8케이스 픽스처 잠금 + 3종 green. ★가드 스타일은 upstream 기존 방식(진입부 `is_null()`)에 맞춘다.
3. **`rustjava-invokedynamic-cp-tags-15-18-support`**(P2·M·med) — 아래 ④ 참조.
   ★**「한 티켓으로 묶는 이유」가 2026-08-16 로 바뀌었다.** 구판 논거(「파서만 고치면 인터프리터가
   `todo!()` 로 죽는다」)는 ★**①머지 뒤 성립하지 않는다** — upstream 이 `jvm_rust/src/verifier.rs` 로
   **패닉 축을 이미 제거**했기 때문이다(④ 참조). 그리고 ③은 ① 뒤에 도는 티켓이다.
   ⇒ ★**새 논거**: ①머지 뒤 남는 것은 **「깔끔한 거부」에서 「실제 지원」으로 올리는 일**이고,
   그것은 **파서(태그 15~18 수용) 없이는 시작조차 못 하고, 부트스트랩 실행(`verifier`·인터프리터) 없이는
   끝나지 않는다** — ★**두 끝이 «같은 기능 하나»의 앞뒤라서 한 티켓이다**(구판처럼 「어느 쪽만 고치면
   반대쪽에서 죽는다」가 아니다).
   - ★**착수 전 필수**: ①머지 뒤 `jvm_rust/src/{verifier,error}.rs` 와 `classfile/src/constant_pool.rs` 의
     **실제 착지 상태를 다시 재라**. `jvm_rust/src/class_definition.rs` 는 **충돌 17파일에 이미 들어 있다** —
     즉 ①이 반드시 손대는 자리다.
   - ★**범위 축소 권고 불변**: 크면 「파서 태그 수용 + 명확한 미지원 예외」까지로 자른다.

### ④미해결 — ★「이미 처리됐다」는 통설은 실측으로 **거짓**이다
- ★`jvm_rust/src/interpreter.rs` `Opcode::Invokedynamic(_) => todo!()` 는
  **origin/main·upstream/main 양쪽에 그대로 살아 있다** — ★**문자열로는 참이지만 «도달 가능성»이 다르다**
  (2026-08-16 정정. 구판은 여기서 멈춰 «양쪽 동일»로 읽었는데 **틀렸다**).
  ★★**upstream 에는 `jvm_rust/src/verifier.rs` 가 새로 있고**(origin 에는 **없다**),
  거기서 `Opcode::Invokedynamic(_)` 를 만나면 `ClassDefinitionError::UnsupportedFeature("invokedynamic")`
  로 **거부**한다. 이 `verify` 는 `jvm_rust/src/class_definition.rs` 가 **클래스 정의 시점에** 부르므로,
  upstream 에서는 ★**인터프리터의 `todo!()` 에 닿기 전에 깔끔한 오류로 끊긴다.**
  방증: 같은 테스트가 origin 은 `test_invokedynamic_consumes_reserved_bytes`,
  upstream 은 ★`test_invokedynamic_is_rejected` 로 **이름부터 바뀌어 있다**(`classfile/src/opcode.rs`).
  ⇒ ★★**①머지가 끝나면 «패닉 축»은 소멸한다.** 남는 것은 **«거부» → «실제 지원»** 이다.
  ※`rustjava-classfile-parse-error-propagation` 이 이 축을 처리했다는 통설은 **여전히 거짓**이다 —
  그 티켓은 **클래스파일 파싱** 경로만 고쳤고 인터프리터·verifier 와 무관하다.
- ★javac 21 익명 내부 클래스 "Malformed class file": 미해결. 원인 후보가 좁혀졌다 —
  `classfile/src/constant_pool.rs` 의 태그 분기가 **origin·upstream 모두 1~12 까지뿐**이고
  **15(MethodHandle)·16(MethodType)·17(Dynamic)·18(InvokeDynamic) 분기가 없다.**
  javac 9+ 는 문자열 `+` 연결조차 invokedynamic 으로 낸다.
  ⇒ STATE 구판의 「태그 15~18 아님」은 **근거 없이 배제한 것**으로 보인다.
  ★**이 절반은 ①머지 뒤에도 그대로 남는다** — verifier 는 파싱이 끝난 뒤에 도는데, 태그 15~18 은
  **파싱 단계에서 먼저 막힌다.** 위 invokedynamic 축과 **한 티켓**(브리프 ③)으로 묶는 근거가 이것이다.
- ★InputStreamReader 디코더: **origin/main 에서 미해결**(read 마다
  `Charset::resolve(...).new_stream_decoder()` 로 새로 만든다). `rustjava-unsupported-charset-exception`
  이 고친 것은 **미지원 charset 패닉**이지 **경계 유실**이 아니다 — 별개 결함이다.
  ※단 upstream 은 완화책을 갖고 있다(`endOfInput` 필드 + UTF-8 lead-byte 역주사 · EUC-KR `>=0x81`
  홀드백). ⇒ **①의 머지로 함께 들어온다.** 별건 발권 전에 ① 이후 상태를 다시 재라.

### ⑤운영 메모 — ★2026-08-16 자력 실측으로 교체(구판 「열린 PR 0」은 **거짓이었다**)

★**열린 PR = 2건**(`gh pr list -R Jun025/RustJava --state open`):

| PR | 브랜치 | 생성 | 마지막 갱신 | 상태 | 원장 |
|---|---|---|---|---|---|
| **#8** `[rustjava-claude-md-prune]` | `feat/rustjava-claude-md-prune` | `2026-08-05T07:56:55Z` | **같은 시각** | `OPEN`/`MERGEABLE` | `.done.md` 有 · ★**`.review.md` 無** |
| **#9** `[rustjava-lane-restart-upstream-sync-precondition]` | `feat/rustjava-lane-restart-upstream-sync-precondition` | `2026-08-15T14:49:33Z` | — | `OPEN`/`MERGEABLE` | 게이트② 진행 중 |

★**#8 은 11일째 좌초 중이고 게이트②가 아예 돌지 않았다** ⇒ 처분은 **브리프 ③-0** 으로 넘겼다.

★**원격 브랜치 = 4건**(`git ls-remote --heads origin`): `main` + 아래 3건.

| 브랜치 | 성격 | 처분 |
|---|---|---|
| `wie-ktf-hardening` | 보존 판정(2026-07-25) | 위 ②로 **잔존 가치가 2건까지 줄었다** — 브리프 ③-2 가 그 2건을 새 브랜치로 옮겨 심으면 ★**보존 근거가 소멸**한다 |
| `feat/rustjava-claude-md-prune` | ★**PR #8 의 head — 좌초 중** | 브리프 ③-0 이 판정 |
| `feat/rustjava-lane-restart-upstream-sync-precondition` | PR #9 의 head(진행 중) | 게이트③에서 정리 |

⇒ ★**「발권 대기 태스크 없음」이 아니다** — 처분 대기 1건(#8) + 브리프 3건이 서 있다.
- ★PR 발권 시 `--repo Jun025/RustJava` 명시(2026-07-22 upstream 오발행 사고 재발 방지).
- ★upstream 발신(PR·이슈·코멘트·push)은 **티켓이 명시 허가할 때만**. 기본은 조회뿐.
