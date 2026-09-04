# upstream 동기화 접근안 (설계 · 2026-08-16)

티켓 `rustjava-upstream-sync-approach-plan`. **머지는 이 회차에 하지 않았다** —
`git merge` 실행 0 · 충돌 해소 0 · 코드 변경 0. 아래는 전부 `merge-tree`/`show`/`grep`
**읽기전용 조회**로 얻은 실측이다.

---

## 0. 재실측 — 선행(2026-08-15) 수치는 이미 낡았다

| 축 | 선행 회차(08-15) | **이번 재실측(08-16)** | 차이 |
|---|---|---|---|
| `rev-list --left-right --count origin/main...upstream/main` | `9  32` | **`10  33`** | 양쪽 +1 |
| origin tip | `2e61e93` | **`85f294a`** | PR #9 착지 |
| upstream tip | `95ebc5c` | **`ba5797b`** | `#201` 신규 |
| 공통조상 | `62cf0c6` | `62cf0c6` | 불변 |
| 충돌 파일 | **17** | **19** | **+2** |

★**증분 2건의 원인은 전부 upstream 신규 커밋 `ba5797b` 하나다.**
교차확인: 선행 시점 tip 쌍으로 다시 돌린 `git merge-tree --write-tree --name-only 2e61e93 95ebc5c`
는 지금도 **17**을 낸다 ⇒ 선행 수치는 그 시점에 옳았고, **upstream 이 움직여서** 바뀐 것이다.

★`ba5797b Fix virtual method dispatch resolution (#201)` 은 **319 files / +20,118 / −5,729** 의
저장소 전역 스윕이다(모든 `JavaMethodProto` 에 `MethodAccessFlags` 부여 + 가상 디스패치 해석 수정).
⇒ ★**이 축의 «충돌 목록»은 반감기가 짧다. 회차 착수 시점에 반드시 다시 재라.**

신규 충돌 2건: `java_runtime/src/classes/java/lang/class_format_error.rs`(add/add) ·
`java_runtime/src/classes/java/lang/throwable.rs`.

**baseline green 실측(현 `origin/main`)**: `cargo fmt --all -- --check` rc=0 ·
`cargo test --all` **149 passed / 0 failed / 1 ignored**.

---

## 1. 처분 어휘 (정의 — 아래 표는 이 넷만 쓴다)

| 어휘 | 뜻 |
|---|---|
| `upstream 채택` | upstream 쪽을 그대로 취한다. 우리 쪽에 살릴 것이 없다(동등하거나 upstream 이 상위집합). |
| `양쪽 병합` | upstream 을 뼈대로 취한 뒤 **우리 쪽의 특정 동작을 다시 얹는다**. 무엇을 얹는지 표에 적는다. |
| `재생성` | 손으로 풀지 않는다. 도구가 다시 만든다(`Cargo.lock`). |
| `별 회차` | 이 머지에서 결정하지 않고 별도 티켓으로 넘긴다. |

★`우리 유지`(우리 쪽 전량 채택)는 **한 파일도 나오지 않았다** — 아래 §2 가 그 근거다.

---

## 2. 충돌 19파일 처분표

hunk 수는 merge-tree 산출 트리(`043062b7`)의 블롭에서 `<<<<<<<` 를 센 값이다.

| # | 파일 | hunk | 충돌 성격 | 우리 쪽 근거 | upstream 쪽 근거 | **제안 처분** | 근거 |
|---|---|---|---|---|---|---|---|
| 1 | `Cargo.lock` | 1 | 잠금파일 | PR #4(`tracing-attributes` 제거) | `#201` 등 다수 | **재생성** | 손 머지 금지. `Cargo.toml` 확정 후 `cargo build` 로 다시 만든다. |
| 2 | `classfile/src/class.rs` | 3 | 파서 구조 재편 | PR #3 | `822504b`(#180) | **upstream 채택** | upstream 이 `parse` 를 `parse_info` + `validate` 로 분리하고 `validation::validate_class` 를 신설했다. 우리 쪽 `ParseError` 배선은 §3-B 로 대체된다. |
| 3 | `classfile/src/constant_pool.rs` | 2 | 파서 구조 재편 | PR #3 | `822504b` | **upstream 채택** | upstream `parse_all` 이 `IResult` 로 돌아가고 `count==0`(거부)/`count==1`(빈 풀) 처리를 추가했다 — 우리에게 없던 케이스다. |
| 4 | `classfile/src/error.rs` | 1 (add/add) | ★**진짜 설계 충돌** | PR #3 `ParseError`(5변형 + `Display` + `from_nom`) | `822504b` `ClassFileError{InvalidFormat, UnsupportedVersion(u16)}` | **upstream 채택** | §3-B 참조. **단 진단 문구 손실은 실비용이고 §4-A 가 그 값을 치른다.** |
| 5 | `classfile/src/lib.rs` | 1 | 재수출 이름 | PR #3 | `822504b` | **upstream 채택** | #4 의 종속. |
| 6 | `java_runtime/src/classes/java/io.rs` | 1 | 모듈 목록 | PR #5 | `af4f6f8`·`ba5797b` | **upstream 채택** | ★**우리 27개 항목 전건이 upstream 목록에 실재함을 대조 확인**했다(엄격한 상위집합). |
| 7 | `.../java/io/input_stream_reader.rs` | 2 | ★**charset 퇴행 지점** | PR #5(`Charset::resolve` 일반화) | `af4f6f8`(UTF-8/EUC-KR 하드코딩) + `endOfInput`/`decode_length` 경계 수정 | ★**양쪽 병합** | upstream 의 **디코더 경계 수정**을 취하고, charset 해석만 우리 `charset::Charset::resolve` 로 되돌린다. §4-B. |
| 8 | `.../java/io/unsupported_encoding_exception.rs` | 4 (add/add) | ★**«정면 충돌»이 아니다** | PR #5 신설 | `af4f6f8` 독자 신설 | **upstream 채택** | ★**선행 회차 판정 정정**: 두 판본은 **거의 동일**하다. 차이는 ⑴`access_flags`/`MethodAccessFlags::PUBLIC` 부여(`ba5797b` 스윕) ⑵`let _: () = …?; Ok(())` → 직접 반환 **문체**뿐이다. 의미 대조가 필요한 자리가 아니다. |
| 9 | `.../java/lang.rs` | 2 | 모듈 목록 | PR #3/#5 | `fe5d116`~`ba5797b` | **upstream 채택** | #6 과 동형. 우리 항목 전건 포함 + 60여 클래스 추가. |
| 10 | `.../java/lang/class_format_error.rs` | 2 (add/add) | ★**«정면 충돌»이 아니다** | PR #3 신설 | 독자 신설 | **upstream 채택** | #8 과 **완전히 같은 형태**(접근 플래그 + 문체). |
| 11 | `.../java/lang/string.rs` | 6 | charset + API 확장 | PR #5 | `822504b`·`c4665b0`·`6203996` | ★**양쪽 병합** | §4-B. ★**charset 축은 사실상 무쟁점으로 판명**됐다 — upstream 이 **동일한 charset 집합·동일한 별칭 정규화**를 독립 구현했다. |
| 12 | `.../java/lang/thread.rs` | 2 | ★**PR #4 정면 되돌림** | PR #4(수동 span) | `f9a315e`·`822504b`(`#[tracing::instrument]` 유지) | ★**양쪽 병합** | upstream 을 뼈대로 취한 뒤 **PR #4 의 수동 span 을 재적용**. §4-C — ★**여기가 이 머지의 최대 함정이다.** |
| 13 | `.../java/lang/throwable.rs` | 1 | 위치 충돌 | PR #5(`getMessage()` 신설) | `ba5797b` | **upstream 채택** | upstream 에 `getMessage`/`getCause` 가 **`PUBLIC` 플래그까지 붙어 실재**한다 ⇒ PR #5 의 기여는 **삼킴**. 충돌은 목록 내 위치 차이일 뿐. |
| 14 | `java_runtime/src/loader.rs` | 2 | 등록 목록 | — | `af4f6f8` 등 | **upstream 채택** | `UTFDataFormatException`·`ClassCircularityError` 등록 추가뿐. |
| 15 | `java_runtime/tests/.../io/test_input_stream_reader.rs` | 2 | 테스트 재작성 | PR #5(3 테스트) | `af4f6f8`·`ba5797b`(5 테스트) | ★**양쪽 병합** | ★**`test_isr_iso_8859_1` 은 우리에게만 있다 — 반드시 이식**(§4-B 잠금). `test_isr_unsupported_charset_throws` 는 upstream `test_input_stream_reader_rejects_unknown_encoding` 과 동치이므로 **버린다**. |
| 16 | `java_runtime/tests/.../lang/test_string.rs` | 2 | 테스트 확장 | PR #5(22 테스트) | `822504b` 이후(54 테스트) | ★**양쪽 병합** | 이름 집합 대조 결과 **우리 전용은 정확히 2건**: `test_get_bytes_unsupported_charset_throws` · `test_new_string_unsupported_charset_throws`. **그 2건만 이식**하고 나머지는 upstream. |
| 17 | `jvm_rust/src/class_definition.rs` | 1 | 오류형 + verifier | PR #3 | `822504b`·`423d1bd` | **upstream 채택** | `verifier::verify(&class)` 호출이 추가된다. STATE ④의 invokedynamic 축이 여기서 닫힌다. |
| 18 | `src/runtime.rs` | 2 | 예외 매핑 | PR #3(1종 매핑) | `822504b`(4종 매핑) | **upstream 채택** | §3-B. |
| 19 | `test_utils/src/lib.rs` | 1 | 예외 매핑 | PR #3 | `822504b` | **upstream 채택** | #18 과 동일 코드. |

### 집계

`upstream 채택` **13** · `양쪽 병합` **5** · `재생성` **1** · `우리 유지` **0** · `별 회차` **0**.

---

## 3. 판정 근거 — 선행 회차 전제 2건을 실측으로 정정한다

### A. 「우리 PR #3·#5 와 upstream 의 독립 구현이 **정면 충돌**」 → **과대평가였다**

선행 회차가 근거로 든 두 add/add 파일(`unsupported_encoding_exception.rs`,
`class_format_error.rs`)의 충돌 hunk를 전량 확인한 결과, **의미 차이가 0**이다.
남는 차이는 `ba5797b` 접근플래그 스윕과 `Ok(())` 문체뿐이다.
⇒ ★**«어느 쪽 구현을 남기는가»가 파일마다 갈리는 상황은 실제로는 «두 파일»이 아니라
«`classfile/src/error.rs` 한 파일»이다.** 나머지는 판단 없이 upstream 을 취하면 된다.

★**정정 사실을 남기는 이유**: 이 오판을 그대로 물려받으면 머지 회차가 «17파일 전부 의미 대조»로
크기를 잡고, 실제로는 필요 없는 곳에 검수 예산을 쓴다.

### B. `classfile/src/error.rs` — 유일한 진짜 설계 결정, 그리고 **upstream 이 이긴다**

| 축 | 우리(`ParseError`) | upstream(`ClassFileError` + `ClassDefinitionError`) |
|---|---|---|
| 변형 | `Truncated` · `BadMagic(u32)` · `UnsupportedConstantPoolTag{index,tag}` · `Malformed` · `TrailingData` | `InvalidFormat` · `UnsupportedVersion(u16)` |
| Java 예외 | `ClassFormatError` **1종** | `ClassFormatError` · `UnsupportedClassVersionError` · `VerifyError` · `UnsupportedOperationException` **4종** |
| 메시지 | 변형별 상세 문구 | `"Invalid class file"` 등 평문 |

⇒ ★**Java 관측면(=어떤 예외가 던져지는가)에서는 upstream 이 4배 세밀하고, Rust 내부 변형에서는
우리가 세밀하다.** JVM 구현체에서 값이 큰 쪽은 **관측면**이다 ⇒ `upstream 채택`.
PR #3 의 목적(「패닉 대신 `ClassFormatError`」)은 upstream 에서도 **그대로 성립**한다 — 삼킴이다.

**치르는 값 = 진단 문구 손실**, 그리고 그 값을 §4-A 가 실제로 청구한다.

---

## 4. 위험 항 — ★**충돌 목록에 «없는» 파일이 더 위험하다**

`merge-tree` 는 충돌만 보고한다. 아래 셋은 **충돌 0으로 조용히 머지된 뒤 깨진다.**
★**이 절이 이 설계 회차의 핵심 산출물이다.**

### A. `tests/test_class_format.rs` — ★충돌 0, 그러나 **4개 중 3개가 깨진다**

우리 전용 파일이라 upstream 과 충돌하지 않고 **그대로 머지된다.** 그런데 내용이
**메시지 문자열을 단정**한다:

- `test_truncated_class_raises_class_format_error` → `err.contains("Truncated")`
- `test_unsupported_constant_pool_tag_raises_class_format_error` → `err.contains("tag 18")`
- `test_bad_magic_raises_class_format_error` → `err.contains("magic")`
- `test_missing_class_still_raises_no_class_def_found_error` → 영향 없음

§3-B 대로 upstream 을 채택하면 메시지가 `"Invalid class file"` 로 평탄화되므로 **앞의 3건이 실패**한다.
★**충돌 마커는 한 줄도 안 뜬다.**

**처분(S3 회차의 계약)**: `ClassFormatError` **종류 단정은 유지**하고 **문구 단정만 완화**한다.
문구를 되살리려면 upstream `ClassFileError` 에 변형을 추가해야 하는데, 그것은 upstream 발신이
필요한 별 축이므로 **이 머지 리니지에서 하지 않는다**(`별 회차` 후보).
※재현 명령: `grep -n 'contains(' tests/test_class_format.rs`

### B. charset 퇴행 — ★**범위가 선행 판정보다 «좁다». `string.rs` 는 무쟁점이다**

★**실측 정정**: upstream 은 `String::decode_str`/`encode_str` 사설 헬퍼에서
**우리 PR #5 와 동일한 charset 집합**을 이미 구현했다 —
`UTF-8|UTF8` · `EUC-KR|EUCKR|KS-C-5601-1987|MS949|CP949` · `ISO-8859-1|LATIN1` · `US-ASCII|ASCII`,
별칭 정규화도 `to_ascii_uppercase().replace('_', "-")` 로 **동일**하고 인코딩 실패 대체문자도 `'?'` 로 동일하다.
⇒ **`string.rs` 에서의 charset 퇴행 위험은 0이다.**

★그리고 이 대조에서 **upstream 이 우리보다 JDK 규격에 맞는 지점**이 하나 나왔다:
upstream 은 **기본 charset 경로**(`System::get_charset` 사용 — `new String(byte[])`·`getBytes()`)에서
미지원 시 **UTF-8 로 폴백**하고, **명시 charset 경로**(`new String(byte[],String)`·`getBytes(String)`)에서만
`UnsupportedEncodingException` 을 던진다. 우리 쪽은 `Charset::resolve` 를 **네 경로 전부**에 걸어
기본 경로에서도 던진다 — JDK 는 던지지 않는다. ⇒ **이 지점도 upstream 이 옳다.**

**실제 퇴행 위험이 남는 곳은 `input_stream_reader.rs` 단 하나다.**
upstream 은 여기서만 `UTF-8`/`EUC-KR` 하드코딩을 유지한다 ⇒ 그대로 취하면
**ISO-8859-1·US-ASCII 가 Reader 에서 사라진다.**

★부수 위험: `java_runtime/src/charset.rs` 는 **우리 전용이라 충돌 없이 그대로 살아남는다.**
`string.rs`·`input_stream_reader.rs` 를 upstream 그대로 취하면 이 모듈은 **호출자 0 = dead code** 가 되어
`cargo clippy --all -- -D warnings` 에서 **CI red** 가 된다. ⇒ ★**「지우거나, 배선하거나」 둘 중 하나이고
답은 «배선»이다** — upstream 의 `decode_str`/`encode_str` 를 `charset::Charset` 으로 라우팅하면
중복 표까지 함께 사라진다.

**회귀 잠금(무엇으로 잡는가)** — 전부 **이미 존재하거나 이식만 하면 되는 것**이다:

| 잠금 | 위치 | 상태 | 무엇을 막나 |
|---|---|---|---|
| ★`test_data/UnsupportedCharset.class` + `.txt` | `test_data/` | ★**이미 실재·이미 돈다** | 종단 잠금. `tests/test_class.rs` 가 `test_data/*.class` 를 **디렉터리 스캔으로 자동 발견**하고 `.txt` 와 stdout 을 대조한다(이 드라이버는 양쪽 동일 = 충돌 없음). 기대 출력에 ★**`3` / `aéb`** 가 박혀 있어 **ISO-8859-1 이 InputStreamReader 를 통과하는지**를 그대로 잡는다. |
| `test_isr_iso_8859_1` | `java_runtime/tests/.../test_input_stream_reader.rs` | **이식 필요**(#15) | 단위 수준 ISO-8859-1 Reader 경로 |
| `test_get_bytes_unsupported_charset_throws`·`test_new_string_unsupported_charset_throws` | `.../test_string.rs` | **이식 필요**(#16) | 명시 charset 경로의 `UnsupportedEncodingException` |
| ★신규 1건 | `.../test_input_stream_reader.rs` | ★**추가 권고** | **US-ASCII** Reader 경로. 위 어느 잠금도 US-ASCII 를 **Reader** 로는 안 밟는다. |

★**`test_data/UnsupportedCharset` 이 이미 도는 잠금이라는 사실이 이 축의 안전판이다** —
머지 중 어느 단계에서 charset 을 떨어뜨려도 `cargo test --all` 이 즉시 red 가 된다.

### C. ★**tracing 함정 — `Cargo.toml` 은 조용히 머지되고 `thread.rs` 는 충돌한다**

| 자리 | 머지 결과 | 근거 |
|---|---|---|
| `Cargo.toml`(root) | ★**우리 쪽 유지** — `tracing = { … , default-features = false }`, `attributes` 피처 **없음** | 자동 머지(충돌 0) |
| `java_runtime/Cargo.toml` | ★**우리 쪽 유지** — `tracing-attributes = { version = "<0.1.29" }` 핀 **없음** | 자동 머지(충돌 0) |
| `java_runtime/src/.../thread.rs` | **충돌** — upstream 쪽에 `#[tracing::instrument(...)]` | `merge-tree` |

⇒ ★★**`thread.rs` 를 upstream 그대로 취하면 «`attributes` 피처가 꺼진 tracing 에 `#[tracing::instrument]`»
가 되어 컴파일이 깨진다.** 그리고 그것을 «피처를 되살려» 고치면 **PR #4 가 통째로 되돌아간다**
(`tracing-attributes` 상한 핀 부활 = tracing 계열 재동결).

**처분**: upstream `thread.rs` 를 뼈대로 취하되 — upstream 이 `attach_thread(instance)` ·
`invoke_virtual(&this, "java/lang/Thread", "run", …)` 로 **시그니처를 바꿨으므로 우리 쪽 본문은 못 쓴다** —
`#[tracing::instrument]` **한 줄만** PR #4 의 수동 span(`tracing::info_span!` + `.instrument(span)`)으로
치환한다. **`Cargo.toml` 두 파일은 손대지 않는다.**
※검증: 머지 후 `git grep -n 'tracing::instrument\|tracing-attributes'` 가 **0건**이어야 한다.

---

### ★★★[2026-09-04 결정] 「우리 자산이 낡는다」 — ★**상시 조항을 «넣지 않는다». 대신 구멍 하나를 막았다**

★**채택 제안 ★셋**(`2026-09-04-upstream-sync-s6#p1` · `2026-09-04-upstream-sync-s7#p1` · ★`2026-09-04-upstream-sync-s8#p0`)에 대한 **결정**이다.
★**[fix3 정정] 초판은 「둘」이었다** — 세 번째 판(`s8#p0`)이 2026-09-04 13:07 에 도착해 총괄이 「이미 발권됨」으로 해제했고, `adoptedProposals` 는 **세 ref** 다.

**결정문**:
> ★**동기 회차 계약에 「우리 자산 서술자 목록을 문서화하고 머지 후 재확인한다」 상시 조항을 «넣지 않는다».**
> ★**대신 «구멍 하나»를 막는다 — 로컬 DoD 가 CI 의 «매트릭스»를 재현하지 않았다.**
> ⇒ `CLAUDE.md` §Definition of Done 이 이제 **CI 명령 5줄 + toolchain 축 1줄 = «6줄»**을 축약 없이 싣는다.

★**[2026-09-04 정정 — 게이트② `request-changes`] 위 결정문의 초판은 「그물이 없던 자리 «하나» — CI 검사 5종 중
wasm32 clippy 한 줄」이었다.** 그것은 ★**계수 «1» 시절의 문면**이고, 계수가 **2**로 정정된 지금은 **낡았다**:
빠진 것은 «줄 하나»가 아니라 ★**«두 축»**(② target 축 = wasm32 `- run:` 줄 · ⑦ toolchain 축 = beta)이고,
둘은 ★**한 근인의 두 얼굴**이다. ⇒ 그래서 DoD 는 **5줄이 아니라 6줄**이다.

★**왜 조항이 아닌가 — «세고» 정했다**(먼저 목록을 만들지 않았다). 자산 8건을 **돌연변이로 깨뜨려** 무엇이 잡는지 쟀다:

| # | 우리 자산 | 돌연변이 | 무엇이 잡나 |
|---|---|---|---|
| ① | 픽스처 경로 문자열 | `test-data/` → `test_data/` | ★`cargo test` **RED** |
| ② | ★**CI 워크플로의 크레이트 이름**(`--exclude test-utils`) | `test-utils` → `test_utils` | ★★**로컬 4종 «어느 것도» 안 잡는다 — CI 만** |
| ③ | `System.setProperty` 서술자 | `String` → `Object` | ★`cargo test` **RED** |
| ④ | charset 라우팅(PR #5) | `Charset::resolve` → 폴백 우회 | ★`cargo test` **RED** |
| ⑤ | 수동 span(PR #4) | `.instrument(span)` 삭제 | ★`clippy` **RED**(미사용 import) |
| ⑥ | `ClassFormatError` 종류 단정(PR #3) | `ClassFormatError` → `Throwable` | ★`cargo test` **RED** |
| ⑦ | `double_must_use` allow(PR #14) | ★**9곳 «전건» 삭제** | ★★**CI 만**(stable clippy **0** ↔ ★**beta clippy RED** — rc=101 · 진단 **6**건) |
| ⑧ | 워크로그 잠금 스크립트 경로 | 스크립트 이동 | 로컬 DoD **와** CI 둘 다 잡는다(rc=2) |

⇒ ★★**「아무것도 없음」 칸은 «2개»(② · ⑦)다.**

★★**[2026-09-04 정정 — 게이트② `request-changes`] 초판은 ⑦을 「깨져도 무해」로 적었고 그것은 «거짓»이다.**
근인은 ★**돌연변이를 «비하중» 자리에 넣은 것**이다 — 초판은 `jvm-bytecode/src/interpreter.rs:1` **한 곳**만 지웠는데
그 크레이트 수준 attribute 는 **같은 파일의 함수별 `#[allow]` 에 가려진 «중복»**이고, `jvm/src/jvm.rs` 에 **7곳**이 그대로 남아 있었다.
★**세 갈래 재측정**(각 회 `cargo fmt --all` 로 삭제 artifact 정규화 후 · 2026-09-04):

| 돌연변이 | stable clippy | ★**beta clippy** |
|---|---|---|
| ⓪ 무돌연변이(기준선) | 0 | **0** |
| ① `interpreter.rs:1` 만 삭제(★초판이 한 것) | 0 | **0** ← 초판 결과 재현됨 |
| ② `jvm.rs:2` 만 삭제(함수별 6 존치) | 0 | **0** |
| ★**③ 9곳 «전건» 삭제** | 0 | ★**RED** — rc=**101** · `double_must_use` 진단 **6**건(`jvm/src/jvm.rs:140·404·438·779·791·1016`) · 요약 줄 「due to **6** previous errors」 |

⇒ ★**「지워도 stable·beta 둘 다 통과한다」는 «중복 attribute 하나»에만 참이고 «자산»에는 «거짓»이다.**
★**⑦의 그물은 «beta clippy» 뿐이고, 그것이 로컬 DoD 에 없었으므로 «CI 만» — ② 와 «문자 그대로 같은 칸»이다.**
★★**그래서 「1개면 그 하나를 고쳐라」 규칙은 이 표에 적용되지 않는다 — 계수가 2다.**

★★**그리고 ② 는 «조용히» 실패하지도 않았다 — 그 정정이 처방을 더 좁혔다.**
낡은 이름으로 치면 cargo 가 ★**`warning: excluded package(s) 'test_utils' not found in workspace`** 를 찍고 **빌드가 깨진다**(rc≠0).
⇒ 문제는 «침묵»이 아니라 ★**«늦음»**이다(push 후 CI 에서만 난다).
⇒ ★**근인은 「자산 목록이 없다」가 아니라 ★«로컬 DoD 가 CI 매트릭스를 재현하지 않는다»**이다.

★★**[2026-09-04 정정 — 계수 2 위에서 논거를 다시 세운다] 결론은 같지만 «이유»가 다르다.**
초판 논거(「1개니까 그 하나를 고쳐라」)는 ★**계수 정정으로 무효**가 됐다. 새 논거는 이것이다:
- ★★**② 와 ⑦ 은 «다른 자산»이 아니라 «한 근인의 두 얼굴»이다** — ②는 **빠진 `- run:` 줄**(wasm32 **target** 축) ·
  ⑦은 **빠진 toolchain 축**(beta). ⇒ ★**근인 «하나»(DoD ≠ CI 매트릭스)를 고치면 «둘 다» 그물을 얻는다.**
  ★**실측**: DoD 에 `cargo +beta clippy --all -- -D warnings` 를 넣자 ⑦의 9곳 전건 삭제가
  ★**로컬에서 RED**(rc=101 · 진단 **6**건)**로 잡힌다**(종전엔 CI 에서만 났다).
- ★**「조항이 «여럿»이면?」도 검토하고 기각한다** — 아래 사료 표대로 세 방향은 잡는 그물이 각각 다르므로
  방향마다 조항을 두면 **셋**이 되고, 그중 둘(테스트·컴파일 축)은 ★**이미 기계가 지키는 자리에 사람 확인을 얹는 것**이다.
- ⇒ 계약이 준 규칙(「1개면 …」)은 **적용되지 않지만**, 그 «취지»(★**「자산 목록」이 아니라 «구멍»을 고쳐라**)는
  계수 2 에서도 그대로 성립한다. ⇒ ★**구멍 «하나»(매트릭스 재현)를 고쳤다 — 축 두 개(target · toolchain)로.**

★**사료 — 「세 번」·「두 방향」은 확정하되, ★«한 조항으로 못 덮는다»가 결론이다**:

| 회차 | 낡은 것 | 방향 | 잡은 것 |
|---|---|---|---|
| S3 | `test_class_format.rs` 문구 단정 3건 | ⑴upstream **신규/판본 교체** | 테스트 |
| S5 | io 테스트 **5곳** `setProperty` 서술자 | ⑴ | 테스트 |
| S6 | regex 테스트 **3곳** 〃 | ⑴ | 정독(테스트 «전») |
| S7 | 우리 고유 테스트 **5곳** `invoke_virtual` | ⑵**공용 API 시그니처 변경** | 컴파일 |
| S8 | `rust.yml` 크레이트명 · `test_class_format.rs` 경로 4곳 | ⑶**개명** | ★CI 만 / 테스트 |

⇒ ★★**셋은 «같은 형태»가 아니다** — 잡는 그물이 각각 다르다(테스트·컴파일·CI).
★**그래서 조항 하나로 덮으려 하면 «이미 그물이 있는 여섯 자리»에까지 사람 확인을 얹게 된다** — 그것이 비용이다.

★★**[2026-09-04 정정 — C5] 「다섯」은 오기였다. 판정 시점 기준으로 «여섯»이 맞다.** 세는 법을 병기한다:
> 위 돌연변이 표에서 **★그물 칸이 「CI 만」이 «아닌» 행의 수**.
> **판정 시점(수리 전)**: 8행 − 「CI 만」 **2행**(② · ⑦) = ★**6** · **수리 후**: ★**8**(둘 다 로컬 그물을 얻었다).
```sh
# 표의 그물 칸을 센다(문서 실측 · 「CI 만」이 아닌 행)
sed -n '/^| # | 우리 자산/,/^$/p' docs/upstream-sync-approach.md \
| /usr/bin/grep -c '^| [①-⑧]' ; \
sed -n '/^| # | 우리 자산/,/^$/p' docs/upstream-sync-approach.md \
| /usr/bin/grep -c 'CI 만'
```

### ★★재개 조건 — «세는 법»과 «오늘의 값» (★**축이 «둘»이다**)

> ★**CI 가 치는 것 중 로컬 DoD 에 없는 것이 «1건이라도» 생기면 이 결정을 다시 연다.**
> ★★**«치는 것»은 «줄»만이 아니라 «매트릭스 차원»도 포함한다** — 그래서 축이 둘이다.

```sh
cd "$(git rev-parse --show-toplevel)"
DOD=$(sed -n '/## Definition of Done/,/^- 착수·완료마다/p' CLAUDE.md)

# ★축① — CI 의 `- run:` 줄 ∖ 로컬 DoD   (0 이어야 한다)
LC_ALL=C /usr/bin/grep -E '^[[:space:]]+- run: ' .github/workflows/rust.yml | sed 's/^[[:space:]]*- run: //' \
| while IFS= read -r c; do printf '%s' "$DOD" | LC_ALL=C /usr/bin/grep -qF -- "$c" || echo "MISSING-RUN: $c"; done \
| /usr/bin/grep -c .

# ★축② — CI 의 toolchain 매트릭스 ∖ 로컬 DoD   (0 이어야 한다)
LC_ALL=C /usr/bin/grep -E '^[[:space:]]+rust: \[' .github/workflows/rust.yml \
| sed 's/.*\[//; s/\].*//; s/, */\n/g' \
| while IFS= read -r tc; do
    [ "$tc" = stable ] && { printf '%s' "$DOD" | LC_ALL=C /usr/bin/grep -qE 'cargo (fmt|clippy|test)' || echo "MISSING-TC: $tc"; continue; }
    printf '%s' "$DOD" | LC_ALL=C /usr/bin/grep -qF -- "cargo +$tc " || echo "MISSING-TC: $tc"
  done | /usr/bin/grep -c .
```

★**오늘의 값**(2026-09-04 정정 후): **축① `0`** · **축② `0`**.
※착수 시엔 **축① 5 · 축② 1**(beta 가 DoD 에 없었다) — 축①은 원 회차가, 축②는 정정 회차가 0 으로 만들었다.

★★**축②를 왜 따로 두는가 — 축① 만으로는 «구조적으로» 못 본다.** 실측(정정 회차):

| 워크플로 돌연변이 | 축① | 축② | ★옳은 값인가 |
|---|---|---|---|
| ⓪ 무돌연변이 | **0** | **0** | ✔ |
| ① `rust: [stable, beta]` → `[stable]`(CI 가 beta 를 **버림**) | 0 | **0** | ✔ ★**0 이 «옳다»** — 술어가 `CI ∖ DoD` 라 **DoD 가 더 엄격한 것은 «구멍»이 아니다**(로컬이 CI 보다 한 축 더 돌 뿐) |
| ② → `[stable, beta, nightly]`(CI 가 **늘림**) | 0 | ★**1** | ✔ **축②만** 반응 |
| ③ `- run: cargo doc --no-deps` 1줄 추가 | ★**1** | 0 | ✔ **축①만** 반응 |

⇒ ★**①이 0 인 것을 «검사가 안 듣는다»로 읽지 마라** — 그 방향은 애초에 재개 사유가 아니다.
★**②·③ 이 각각 «자기 축»에서만 1 을 내는 것**이 두 축이 «겹치지 않고 덮는다»는 증거다.

★**1 이상이 되면**: ⑴그 줄/축을 DoD 에 넣거나 ⑵넣을 수 없는 이유를 적고 **이 결정을 재검토**하라.
★**「조항이 필요하다」로 바로 가지 마라 — 그때도 «먼저 세라»**(위 돌연변이 표를 다시 만들면 된다).

★★**[C7 고지 — 이 처방이 «못 잡는» 것]**
⒜**DoD 블록 자체의 개악은 자동으로 안 잡힌다** — 누가 그 블록에서 한 줄을 지우면 축①이 **1** 이 되지만,
★**그 축을 «돌리는 것»이 사람이다.** 기계 강제(DoD ↔ `rust.yml` 대조 CI job)는 이 회차 범위 밖이고,
★**고칠 수 없으면 «고지»가 처방이다.** ⇒ ★**이 절을 읽는 회차는 「축①·축②를 «실제로 돌렸는가»」를 회신에 적어라.**
⒝★**OS 축(macos·ubuntu·windows 3종)은 로컬에서 재현할 수 없다** — 그 차원은 ★**CI 가 «유일한» 그물**이고
대조 대상이 아니다. **결함이 아니라 «알고 두는» 값이다.**

★★★**[2026-09-04 규율 — 게이트② 2회 연속 반려가 만든 것] 「고쳤다」를 쓰기 «전»에 «세라».**
이 리니지는 ★**「해소했다고 신고한 것이 산출물에 그대로 남아 있는」 형태로 두 번 반려됐다.** 실측:
「다섯 → 여섯 통일」은 ★**4자리 중 1자리**만 됐고, 워크로그 `.json` 의 `verification` 은 ★**diff 에 한 줄도 없이**
철회된 라벨(「beta 도 ok — 깨져도 무해」)을 «검증 기록»이라는 더 권위 있는 필드에 계속 싣고 있었다.
⇒ ★**정정 회차의 마감 절차 3줄**(이 절을 읽는 회차는 그대로 하라):
1. ★**`git diff` 로 확인하지 마라 — «착지본 전문 검색»으로 확인하라.** diff 는 «안 바뀐 자리»를 보여주지 않는다.
   `LC_ALL=C /usr/bin/grep -rn '<옛 문자열>' --include='*.md' --include='*.json' .` → ★**0건이어야 한다.**
2. ★**분모는 「이 PR 이 만지는 문서 «전부» × 그 안의 필드 «전부»」다**(`git diff --name-only <base>..HEAD`).
   ★**`.json` 의 «필드»가 사각이다** — `summary`·`changes`·`issues` 를 고치고 `verification` 을 빠뜨리기 쉽다.
3. ★**정정한 수는 «파생 수»를 데리고 다닌다.** 계수를 1 → 2 로 고치면 「다섯」·「5줄」·「자리 하나」가 전부 낡는다 —
   ★**고친 수 하나마다 그 수에서 유도된 문장을 «검색해서» 세라.**
4. ★★**[2026-09-04 추가 · fix3] 「0건」을 «필터 걸린 검색»으로 받지 마라 — 이 회차가 «거짓 0»을 실제로 받았다.**
   fix3 이 잔존을 `grep … | grep -v '초판\|정정\|철회'` 로 세어 **전 패턴 0** 을 얻었는데, 그것은 ⑴zsh 가 따옴표 없는
   `$F`(파일 목록 변수)를 ★**단어분할하지 않아** grep 이 «파일 하나도 못 열고» stderr 로 죽었고 ⑵파이프의 `-c` 가
   그 빈 출력을 **0** 으로 센 결과였다. ★**진짜 잔존이 그 뒤에 «하나 더»(`changes[0]`) 있었다.**
   ⇒ ★**세는 법**: 파일을 **인자로 나열**하고(변수 전개에 기대지 마라 · `set -- $VAR` 도 zsh 에선 1개다),
   ★**먼저 «필터 없이» 총계를 내고 «그다음» 한 건씩 «정정 기록인가 잔존인가»를 눈으로 갈라라.**
   ★**필터는 세는 단계가 아니라 «읽는» 단계에 붙인다** — 세는 단계에 붙이면 그 필터가 잔존을 함께 지운다.

## 5. 단계 분할 — ★**커밋 수로 자르지 마라. 충돌은 앞쪽 7커밋에 몰려 있다**

각 컷 지점에서 `git merge-tree --write-tree --name-only origin/main <cut>` 을 돌린 실측:

| 컷 | 커밋 | 누적 충돌 |
|---|---|---|
| `3b620b8`(#173) | 1 | **0** |
| `fe5d116`(#174) | 2 | 1 |
| `f9a315e`(#175) | 3 | 2 |
| `1f356ae`(#179) | 5 | **2** |
| `af4f6f8`(#177) | 6 | **7** |
| `822504b`(#180) | 7 | ★**16** |
| `7dc1b90` | 10 | 17 |
| `c4665b0`(#190) | 21 | 17 |
| `95ebc5c` | 32 | 17 |
| `ba5797b`(#201) | 33 | **19** |

⇒ ★★**19개 충돌 중 16개가 앞쪽 7커밋에서 발생하고, 그 뒤 26커밋이 더하는 것은 3개뿐이다.**
「32커밋을 4등분」류의 분할은 **1회차에 16충돌을 전부 만나고** 나머지 회차는 빈손이 된다 — 무의미하다.
**축으로 자른다.**

### 제안 — 7회차

| 회차 | 목표(`git merge <cut>`) | 커밋 | diff 규모 | **새 충돌** | 축 | 성격 |
|---|---|---|---|---|---|---|
| **S1** | `1f356ae` | 5 | 66f +5,233/−149 | **2** (`lang.rs`·`thread.rs`) | ★**tracing / PR #4** | 설계 |
| **S2** | `af4f6f8` (#177) | 1 | 62f +3,245/−366 | **+5** (`io.rs`·`input_stream_reader.rs`·`unsupported_encoding_exception.rs`·`loader.rs`·`test_input_stream_reader.rs`) | ★**charset / PR #5** | 설계 |
| **S3** | `822504b` (#180) | 1 | 69f +2,127/−356 | **+9** (`classfile/*` 4 · `string.rs` · `test_string.rs` · `class_definition.rs` · `src/runtime.rs` · `test_utils`) | ★**오류 분류 / PR #3** | 설계 |
| **S4** | `3296139` (#184) | 8 | 50f +4,418/−187 | **0** | GlobalRef · CLI classpath · monitor | 물량 |
| **S5** | `c4665b0` (#190) | 6 | 171f +33,138/−1,058 | ~~**0**(`Cargo.lock`만)~~ → ★**+3** | Java 1.2 API 확장 | 물량 |
| **S6** | `95ebc5c` | 11 | 142f +17,593/−483 | ~~**0**~~ → ★**+0** | regex · Formatter · logging · LinkedHashMap | 물량 |
| **S7** | `ba5797b` (#201) | 1 | 319f +20,118/−5,729 | ~~**+3** (`class_format_error.rs`·`throwable.rs`·`Cargo.lock`)~~ → ★**+1** (`thread.rs`) | 접근 플래그 · 가상 디스패치 스윕 | 물량+ |

★**취소선 칸은 2026-08-16 계획 기준(base `03438b0`)의 사료다 — 지우지 마라.** 화살표 뒤 수는
**2026-09-03 재측정**(base = 현 `main` `8c1238b`)이고, 근거·전 기준 대조는 아래 **[2026-09-03 재측정]** 블록에 있다.

★**「새 충돌」은 «해당 컷에서 처음 충돌하는 파일 수»다.** 앞 회차가 착지하면 뒤 회차의 기준선이
바뀌므로 **각 회차 착수 시 재측정이 필수**다(§0 의 교훈 그대로).

> ★★**[2026-08-27 정정 — 이 표의 「새 충돌」은 «base 가 전진할 때»의 수다. 그 전제는 S1~S4 에서 깨져 있었다.]**
> 위 「앞 회차가 착지하면 뒤 회차의 기준선이 바뀐다」는 **거짓이었다.** 게이트③이 제품 repo 를
> `--squash` 로 착지시키므로 PR 브랜치의 upstream 머지 부모가 버려지고, `origin/main` 은 **내용만**
> 받은 채 **계보는 fork 시점(`62cf0c6a`)에 머문다.** ⇒ `git merge-base origin/main upstream/main` 이
> 전진하지 않으므로 **다음 회차는 앞 회차가 이미 닫은 충돌을 처음부터 다시 연다.**
>
> 실측(S1~S4 착지 «후»): 머지커밋 `6bfe97c4`·`11ef5010`·`4bb796de`·`3a597768` **전건 부모 1개**(=squash) ·
> `merge-base` **`62cf0c6a` 불변** · behind **33**.
>
> ★★**회차별 충돌은 «기준»을 달지 않으면 비교할 수 없다**(초판이 그 실수를 했다 — 아래 표의 네 칸이
> 서로 다른 기준에서 하나씩 뽑혀 **우상향 계열**로 읽혔다. 그 계열은 **어느 기준에서도 성립하지 않는다**):
>
> | 회차 | 그냥 재면(복원 전) | `-s ours` 복원 후 | 계보 상태 | ★재생분(앞 회차가 이미 닫은 자리) |
> |---|---|---|---|---|
> | S1 | **2** | — (**복원 불요** · fork 시점 base) | 해당 없음 | — |
> | S2 | **15** | **5** | 끊김 → 복원함 | ★**15 중 10** |
> | S3 | **11** | — (**복원 불요**) | ★**온전**(`merge-base` = `af4f6f8`) | ★**0** |
> | S4 | **20** | **2** | 끊김 → 복원함 | ★**20 중 18** |
>
> ★★**S3 는 «사례»가 아니라 «반례»다 — 계열에서 빼기만 하지 말고 이 문장을 남겨라.**
> S3 는 S2 브랜치 «위에» 쌓아 `merge-base` 가 `af4f6f8` 로 **정상**이었고 `-s ours` 복원을
> **하지 않았다**(`s3.done.md` §0-⑴). 그 **11** 은 **계보가 온전한 상태의 순수 신규 충돌**이고
> **재생분 0** 이다. ⇒ 「계보 미수렴의 비용」 계열에 넣으면 **근거가 아니라 반증**이 된다.
>
> ★**그래서 이 병의 근거는 «충돌 총수»가 아니라 «재생분»이다** — 기준이 하나이고 S3 도 자연히 설명된다:
> ★**S2 10 → S4 18**(둘 다 계보가 끊긴 회차 · 출처 `s2.done.md` 열거 · `s4.done.md` §2) · **S3 0**(계보 온전).
>
> ★**예측 대 실측 — «복원 전»(계획서가 쓴 `merge-tree origin/main <컷>` 과 같은 기준)으로 통일**:
> S1 예측 2 ↔ 실측 **2** · S2 +5 ↔ **15** · S3 +9 ↔ **11** · S4 **0 ↔ 20**.
> ⇒ S4 는 ★**복원 전 20 · 복원 후 2** 로 **어느 쪽으로 읽어도 「0」 예측을 빗나갔다**.
>
> ⇒ ★**S5~S7 의 「0」(`Cargo.lock`만)도 같은 전제 위의 수다. 그대로 믿지 마라 — «하한»으로 읽어라.**
> 이 표를 쓰기 전에 **반드시** `git merge-base origin/main upstream/main` 을 먼저 재고, 그것이
> 직전 컷으로 전진해 있지 않으면 **해당 회차 착수 시 `merge-tree` 로 전량 재측정**한다.
>
> **처분**: 계보 기록 커밋(`git merge -s ours <컷>` · 트리 변경 0)으로 `merge-base` 를 `3296139`
> 까지 끌어올렸다(behind 33 → 18). ★**단 이 커밋 자체가 `--squash` 로 착지하면 무의미하다** —
> 게이트③ 예외 판정은 총괄 소관(`REPORT.md` 후속 추천 참조).

### ★★[상시 규칙 · 2026-09-03 채택] 충돌 수를 적을 때는 «base» 를 반드시 병기한다

**형식**: ★**`<수> <델타|누적>(base <sha> · merge-base <sha>)`**.
★**병기 없는 충돌 수는 문서·회신·티켓 어디에도 쓰지 않는다.**

★★**[2026-09-04 보강 ② — 게이트② 지적 · S7 회차가 집행] «무엇을 센 수인가»(델타/누적)를 서식에 넣는다.**
초판은 base 만 못박아서 ★**같은 base 로 적어도 「3 → 3」과 「0」이 «둘 다» 규칙을 지켰다.**
- **누적** = 그 base 에서 그 컷을 머지하면 **실제로 열리는** 파일 수 ⇒ ★**「이번 회차가 풀 것」**
- **델타** = 직전 컷 누적 대비 **새로 «나타난»** 파일 수 ⇒ ★**「어느 컷이 그 파일을 처음 연다」**

★★**둘은 «다른 질문»이고, 델타 0 은 「풀 것이 없다」가 «아니다».**
★**실사고**(2026-09-04 S6): §5 가 「S6 새 충돌 **0**」으로 남긴 수는 **델타**였는데
★**티켓 Goal 이 그것을 「부딪힐 것이 없다」로 읽었다.** 실제로는 **누적 1**(`string.rs`)이었다 —
직전 컷에서 이미 열려 있던 파일이라 «새로 나타나지» 않았을 뿐, **풀 일은 그대로 있었다.**
⇒ ★**둘 중 하나만 적지 마라. 어느 쪽인지 «표시»하거나 «둘 다» 적어라.**

★**본보기(2026-09-04 S7 착수 재측정 — 이 서식으로 적은 첫 수)**:
> 컷 `ba5797b` 충돌 = ★**누적 1 · 델타 +1**(base `3e02f8c` · merge-base `95ebc5c`) — `thread.rs`.
> ※`string.rs` 는 이번 집합에서 **빠졌다**(S6 이 해소했고 이 1커밋 구간에서 upstream 이 안 건드렸다)
> ⇒ ★**누적은 «줄어들 수도» 있다** — 그래서 누적을 델타로 대신할 수 없다.

★**과거 기록은 소급 수정하지 않는다**(그 파일은 기록이다) — 아래 표·블록의 수는 **옛 서식**이고,
★그 값들은 대체로 **누적**이다(「새 충돌 +N」으로 적힌 칸만 델타다).

★★**[2026-09-04 보강 — 게이트② 지적] `diff` 를 인용할 때는 «방향»도 못박는다.**
초판은 base 만 못박고 **방향을 안 못박았다** — 그래서 같은 표 안에서 「우리」 열이 역순
(`diff <ours> <merge-base>`)으로 실려 **부호가 뒤집혔다**(★이 리니지의 반려 근인 「기준을 섞었다」와 **같은 종**이다).
⇒ **정본 방향을 하나로 고정한다**:
- **우리 쪽 분기** = `git diff --numstat <merge-base> <ours>` (예: `3296139` → `origin/main`)
- **upstream 변경** = `git diff --numstat <merge-base> <cut>` (예: `3296139` → `c4665b0`)

★**둘 다 «왼쪽이 merge-base»** 다 — 그래야 `+`/`−` 가 「그 쪽이 base 에 무엇을 더했나」로 **같은 뜻**을 갖는다.
★`--numstat` 열 순서는 `추가 삭제 경로` 다.

★**근거는 「좋은 습관」이 아니라 실측이다** — 같은 컷 `ba5797b` 가 base 에 따라 이렇게 갈린다:

| base | merge-base | `ba5797b` 누적 충돌 |
|---|---|---|
| `03438b0`(2026-08-16 계획 시점 `main`) | `62cf0c6a` | **19** |
| `3a59776`(PR #18 «직전» `main`) | `62cf0c6a` | ★**112** |
| `8c1238b`(현 `main`) | `3296139c` | ★**4** |

⇒ ★★**「충돌 4건」과 「충돌 112건」은 «둘 다 참»이고, base 가 없으면 어느 쪽도 검증할 수 없다**(28배 차).

★**이 규칙이 막는 것은 가정이 아니라 «이미 일어난 사고»다.** 직전 반려의 근인이 정확히 이 병기 누락이었다 —
S2 회신(복원 후 5)과 S4 회신(복원 전 20)은 ★**각자는 정확했는데** 둘을 «한 표»로 모으는 순간 기준이 섞여
「우상향 계열」이라는 **없는 사실**이 만들어졌다. ⇒ 개별 회신의 정확성으로는 막히지 않는다. **집계 시점에 깨진다.**

★**병기가 필요한 base 축은 둘이다**(하나로는 부족하다): ⑴**복원 전** = 계보 미기록 상태(`merge-base` 가
fork 시점) ⑵**복원 후** = 계보 기록 뒤(`merge-base` 가 직전 컷). ★**둘 다 적어라** — 전자는 «계보 미수렴의
비용», 후자는 «다음 회차의 실제 작업량»이고 **용도가 다르다**.

> ★★**[2026-09-03 재측정 — PR #18 이 `--merge` 로 착지한 «뒤». 위 블록이 요구한 「전량 재측정」의 집행 결과다.]**
>
> **선행 확인**: `git merge-base origin/main upstream/main` = ★**`3296139cc7ce63822941db5180bd19a8545367e4`**(= S4 컷) ·
> `git rev-list --count origin/main..upstream/main` = **30** · PR #18 머지커밋 **`8c1238b`** 부모 **2개**.
> ⇒ ★**계보가 실제로 전진했다** — 위 블록이 「거짓이었다」고 적은 전제(「앞 회차가 착지하면 뒤 회차의
> 기준선이 바뀐다」)가 ★**이 회차부터 처음으로 참이다.**
>
> **측정 명령**(전 회차 동일): `git merge-tree --write-tree --name-only <base> <cut>` 의 **2번째 줄부터 첫 빈 줄 앞까지**를 센다
> (그 뒤는 `Auto-merging` 정보 블록이라 세면 안 된다 — 이 파싱으로 위 §5 첫 표 `0·1·2·2·7·16·17·17·17·19` 가 **그대로 재현**된다 = 방법 검증).
>
> | 컷(회차) | ⓐ계획 기준 `03438b0` | ⓑ★복원 «전» `3a59776` | ⓒ★복원 «후» `8c1238b`(현 `main`) |
> |---|---|---|---|
> | `3296139`(S4) | 누적 **16** | 누적 **10** | 누적 ★**0** |
> | `c4665b0`(**S5**) | 누적 **17** · 새 **+1** | 누적 ★**45** | 누적 **3** · 새 ★**+3** |
> | `95ebc5c`(**S6**) | 누적 **17** · 새 **+0** | 누적 ★**61** | 누적 **3** · 새 ★**+0** |
> | `ba5797b`(**S7**) | 누적 **19** · 새 **+2** | 누적 ★**112** | 누적 **4** · 새 ★**+1** |
>
> ★ⓑ의 `merge-base` 는 `62cf0c6a`(fork 시점) · ⓒ는 `3296139c`. ★**ⓑ와 ⓒ의 차이는 «계보뿐»임을 통제했다** —
> ⓒ의 트리를 ⓑ의 계보 위에 얹은 합성 커밋(`git commit-tree 4788ef2f -p 3a59776`)으로 재면 **45·61·112 로 동일**하다.
> ⇒ ★**문서 4파일 diff 는 이 수에 영향이 없고, 갈리는 원인은 100% `merge-base` 다.**
>
> **ⓒ 기준 충돌 파일**(= 다음 회차가 실제로 여는 것):
> **S5** `Cargo.lock` · `java_runtime/src/classes/java/lang/string.rs` · `java_runtime/tests/classes/java/util/test_timer.rs`
> **S6** (S5 와 동일 3건 · 신규 0) **S7** +`java_runtime/src/classes/java/lang/thread.rs`
>
> ★★**ⓒ 의 새 충돌은 «upstream 이 만드는 것»이 아니라 «우리가 앞 회차에 남긴 로컬 분기»가 만든다** — 이번 재측정의 핵심이다:
> `string.rs`(우리 **+8/−28** vs `3296139`) · `test_timer.rs`(우리 **+8/−1** = **S4 가 넣은 500→2000ms 여백**).
> ★**방향은 위 상시 규칙대로 `diff <merge-base> <ours>` 다**(초판의 `+28/−8`·`+1/−8` 은 역순 출력이라 **폐기**).
> 둘 다 ⓐ 기준에는 **없던** 파일이다(그때 우리 `main` 은 그 자리가 merge-base 와 동일했다).
> ⇒ ★**「upstream 물량이 크면 충돌도 크다」가 아니다** — S7 은 diff 가 32만 줄대인데 새 충돌 **1건**이다.
>
> ★**예측 대 실측 — ★같은 기준끼리 짝지었다**(§5 표의 예측은 ⓐ에서 나온 수이므로 ⓐ와 짝짓는다):
> S5 예측 `0`(`Cargo.lock`만) ↔ ⓐ실측 **+1**(`Cargo.lock`) = **일치**(표기 차이) ·
> S6 `0` ↔ **+0** = 일치 · S7 `+3` ↔ ⓐ실측 ★**+2** = **예측이 과대**.
> S7 이 빗나간 이유는 셋이고 **전부 실측된다**:
> ⑴`Cargo.lock` 은 **S5 에서 이미 충돌**한다 ⇒ S7 의 «새»가 아니다(표가 **이중 계상**했다).
> ⑵표가 적은 `classfile/src/class_format_error.rs` 는 ★**경로가 틀렸다** — 실제 파일은
> `java_runtime/src/classes/java/lang/class_format_error.rs` 이고, ★**우리 쪽이 `3296139` 와 바이트 동일**
> (블롭 `0dbd369a`)이라 upstream 의 **+4/−3** 이 깨끗이 적용된다.
> ★★**[2026-09-04 정정] 초판이 「`3296139..ba5797b` 에서 upstream 변경 0」이라고 적은 것은 «거짓»이고 «0 인 쪽이 반대»였다** —
> `git diff --numstat 3296139 ba5797b -- <그 경로>` 는 **`4 3`**(= +4/−3)이고, **0 줄인 것은 `3296139` → `origin/main`**(우리 쪽)이다.
> ⇒ ★**이 구별이 계획에 직결된다**: 「upstream 이 안 건드리는 파일」로 읽으면 «영구 무충돌»이지만, 참인 술어로 읽으면
> ★**우리가 그 파일을 손대는 «순간» 충돌한다** — 바로 아래 ⑶ 및 `string.rs`·`test_timer.rs` 가 그 기전의 실례다.
> ⑶`throwable.rs` 는 upstream 이 **+81/−29** 로 고쳤는데 ★**우리 쪽이 `3296139` 와 바이트 동일**이라 깨끗이 적용된다
> ⇒ ★**S3 가 upstream 의 오류 분류 축을 «채택»해 수렴시킨 결과다** — 앞 회차의 설계 판단이 뒤 회차의 충돌을 **지웠다**.
>
> ★★**「예측은 하한이다」 — 대체로 맞지만 «절대적이지 않다». S7 에서 처음 깨졌다.**
> 각 회차를 «그 회차가 실제로 돈 base» 에서 보면:
> S1 `2`↔**2**(일치) · S2 `+5`↔**5**(복원 후 · 일치) · S3 `+9`↔**11**(초과) · S4 `0`↔**2**(복원 후 · 초과) ·
> S5 `0`↔ⓒ**+3**(초과) · S6 `0`↔**+0**(일치) · **S7 `+3`↔ⓒ+1 · ⓐ+2** ★**미달**.
> ⇒ ★**하한으로 «쓰되» 근거로 삼지 마라.** 초과는 «우리 로컬 분기»가 만들고(S5), 미달은 «앞 회차가 수렴시켜서» 난다(S7).
> ★**티켓 size/timeout 은 여전히 하한 쪽으로 잡아라** — 미달은 공짜지만 초과는 회차를 넘긴다.
>
> **S5~S7 재조정 판정**(수만 바뀌고 순서·축은 불변):
> - **S5** = 「충돌 0 물량」이 **아니다.** `string.rs` 는 **설계 판단**(우리 charset/`value_range` 분기 ↔ upstream +285/−31)이 붙는다.
>   ~~`test_timer.rs` 는 **S4 여백(2000ms)을 upstream 재작성 위에 다시 얹는** 기계 작업이고~~ → ★**착지로 반증됐다(아래 S5 착지 기록)**,
>   `Cargo.lock` 은 **재생성**이다.
> - **S6** = 진짜 「새 충돌 0」이다(S5 를 닫으면 남는 게 없다).
> - **S7** = `thread.rs` 1건. ★**STATE ①의 「`thread.rs` 는 S5~S7 도 「또 충돌한다」를 기본값으로 잡아라」가 «절반» 맞았다** —
>   S5·S6 은 **충돌하지 않고** S7 에서만 다시 열린다. 해소 전략은 불변(upstream 본문 + `#[tracing::instrument]` 한 줄만 수동 span 치환).
> - ★**착수 시 재측정 의무는 «남는다»** — 다만 근거가 바뀌었다. 이제 `merge-base` 는 전진하므로 이유는 「계보가 끊겨서」가
>   아니라 ★**「upstream/main 이 계속 전진하기 때문」**이다(2026-08-16 `ba5797b` 였던 헤드가 지금 `bd42427`, behind **30**).
>
> ★★**부수 발견 — 「7회차」는 더 이상 upstream 헤드에 닿지 않는다. S8 이 필요하고, 그것이 남은 회차 중 «제일 크다».**
> 컷 간 커밋 수 실측(★**전부 «증분» = 앞 컷을 왼쪽 끝점으로 둔 구간**):
> `3296139..c4665b0` **6** · `c4665b0..95ebc5c` **11** · `95ebc5c..ba5797b` **1** · ★**`ba5797b..upstream/main` 12**(합 **30** = behind).
> ★**초판은 뒤 셋을 `..95ebc5c` 처럼 왼쪽 끝점을 생략해 적어 «`3296139..` 누적»으로 오독될 수 있었다**(정정 2026-09-04).
> 참고로 누적은 `3296139..95ebc5c` **17** · `3296139..ba5797b` **18** 이다 — **증분과 다른 수다**.
> 그 12커밋 구간의 ⓒ 기준 누적 충돌 = ★**11**(S7 의 4 대비 **+7**)이고, 성격이 앞 회차들과 **다르다**:
> `java_runtime/` → **`rustjava-runtime/`** · `test_data/` → **`test-data/`** 로 **크레이트·디렉터리가 개명**됐다.
> ⇒ 새로 걸리는 7건이 ★**우리 고유 산출물에 정확히 꽂힌다**: `rustjava-runtime/src/charset.rs`(PR #5) ·
> `test-data/UnsupportedCharset.class` · `test-data/UnsupportedCharset.txt` · ★**`test-data/src/UnsupportedCharset.java`**(PR #5 픽스처) ·
> `test-data/TimeApi.class` · `test-data/TimeApi.txt`(PR #2 픽스처) · `java_runtime/tests/classes/java/lang/test_string.rs`.
> ★★**[2026-09-04 정정] 초판의 `test-data/UnsupportedCharset.{java,class,txt}` 는 «경로 오기»다** — `.java` 만
> `test-data/src/` 아래에 있다. ★**S8 의 첫 조치가 `--find-renames` 대응표 고정이라 이 목록의 경로는 정확해야 한다**
> (★이 회차가 §5 에서 정정한 결함이 정확히 «경로 오기»였다 — 같은 자리를 두 번 밟았다).

### ★★[2026-09-04] S5 착지 기록 — 예측 3건 중 **2건은 맞고 1건은 반증됐다**

**착수 시 재측정**(§5 상시 규칙 · base = 당시 `main` **`1983d9f`** · `merge-base` **`3296139c`**):
충돌 **3건**으로 위 재측정 표와 **일치**(그 사이 #19·#20 이 착지했으나 둘 다 문서 회차라 수가 안 바뀌었다).

| # | 파일 | 예측 | ★실제 처분 |
|---|---|---|---|
| 1 | `Cargo.lock` | 재생성 | ✔ **재생성**(`cargo build` · 손 머지 0) |
| 2 | `string.rs` | ★설계 판단 | ✔ **설계 판단이 맞았다 — 단 «형태»가 예상과 달랐다**(아래) |
| 3 | `test_timer.rs` | 여백 «되얹기» | ★**반증 — 되얹을 자리가 사라졌다**(아래) |

★**2 `string.rs` — 진짜 판단은 「어느 쪽을 취하나」가 아니라 「upstream 이 되살린 것을 버릴 것인가」였다.**
upstream 은 `copyValueOf` 2종을 신설하면서 **`decode_str`/`encode_str` 하드코딩 charset 표를 «되살렸다»**.
그런데 그 표를 쓰던 **4개 호출부는 충돌 없이 자동병합돼 우리 `Charset` 라우팅을 유지**했다.
⇒ upstream 블록을 통째로 취했으면 **표 2함수가 dead code 로 남아** §5 의 S3 완료조건
(「`charset.rs` 배선으로 dead code 0」)을 깼을 것이다. ⇒ ★**신규 API 는 취하고 표는 버렸다.**
★**이것이 「union 자동병합 경계」의 실례다** — 충돌면은 «표»에만 났고, 의미가 갈린 곳은 «충돌하지 않은 호출부»였다.

★★**3 `test_timer.rs` — 예측이 «되얹기»였는데, upstream 이 «벽시계 의존 자체»를 없앴다.**
upstream `c4665b0` 은 우리 2개 벽시계 테스트를 ★**manual clock(`new_with_queued_spawns_and_manual_clock`) +
queued spawn + monitor notification** 기반 **결정성 스위트 12건**으로 **대체**했다(`Thread.sleep` 기반 단정 **0건** ·
`TEST_BARRIER_TIMEOUT` 은 행 방지용이지 타이밍 단정이 아니다). ⇒ **`upstream 채택`.**
★★**S4 가 남긴 「우리 테스트의 시간 의존」 별 축은 이로써 «소멸»했다 — 발권하지 마라.**
★**그러나 «왜 2000ms 였는지»는 지우지 않는다**(그 근거가 사라지면 다음 사람이 같은 자리를 다시 넓힌다):
> 500ms 창은 **컷 이전부터** 기대 ~10회 대비 `run_count` **3~4**만 냈고(조건 맞춘 교대 실행에서 컷 전후 mean **3.5** 동일),
> upstream 이 같은 여백을 **두 번**(`895d67d` 2025-08-20 · `ad8b477` 2025-10-04) 이미 넓혔다.
> `> 2` 단정은 불변이고 창만 넓혔으며, 대가는 감도(red 문턱 1회전 ~167ms → ~667ms · 약 4배 둔화)였다.

★★**충돌 «목록에 없던» 파손 1건 — §4 가 경고한 바로 그 형태가 실제로 났다.**
upstream 신규 io 테스트 **3파일 5곳**이 `System.setProperty` 를 **`)Ljava/lang/Object;`** 로 부르는데,
우리는 PR #5 에서 **JDK 규격대로 `)Ljava/lang/String;`** 으로 고쳐 뒀다 ⇒ ★**충돌 마커 0줄인데 `NoSuchMethodError` 3건.**
⇒ **서술자만 `String` 으로 맞췄다**(값은 `_` 로 버려지므로 바인딩 타입 무접촉).
★**새 처분이 아니다** — `test_boolean`·`test_integer`·`test_long` 이 **앞 회차에 이미 같은 처분**을 받았고
그 3파일은 이번에 자동병합으로 통과했다. ★`java/util/Properties.setProperty` 의 `Object` 반환은 **JDK 규격상 옳아 무접촉**이다.

★★**`Cargo.lock` 재생성 함정 1건 — CI beta 셀 3개를 red 로 만들었다(로컬 stable 은 green 이었다).**
첫 시도는 충돌한 `Cargo.lock` 을 **upstream 판본으로 취한 뒤** `cargo build` 를 돌렸는데, 그 lock 이 이미
유효해서 cargo 가 **아무것도 올리지 않았다** ⇒ ★**`async-trait` 이 우리 `0.1.92` → upstream `0.1.91` 로 «내려갔다».**
★**0.1.91 의 매크로가 `#[must_use]` 를 이중으로 달아** beta clippy 의 `double_must_use` 가 `jvm/` 의
`#[async_trait]` 트레이트 **7자리**를 물었다(`array_class_definition`·`class_definition`·`class_loader`×2·`method`·`lib`).
★**그 트레이트들은 `main` 과 바이트 동일**이고 `main` 의 beta 셀은 **green** 이었다 ⇒ ★**코드가 아니라 lock 이 원인**이다.
⇒ **처방**: `main` 의 lock 에서 출발해 다시 `cargo build`(= §2 의 「도구가 다시 만든다」 그대로).
`async-trait` **0.1.92 유지** · lock 델타는 **`libm` 추가 1건**뿐. ★**`#[allow]` 를 뿌리지 않았다** — PR #14 가
그 처방을 쓴 자리는 `async_recursion` 매크로였고, 이번 건은 **버전만 되돌리면 사라진다**.
★**재현·검증은 로컬 beta 툴체인으로 했다**: `cargo +beta clippy --all -- -D warnings` **rc=1 → rc=0** ·
`cargo +beta test --all` **427 passed / 0 failed / 1 ignored**(stable 과 동수).

**착지 실측**: `merge-base origin/main upstream/main` **`3296139c` → `c4665b0`** · behind **30 → 24** ·
머지커밋 **부모 2개** · `c4665b0` 대비 **삭제 파일 0** ·
`cargo test --all` **427 passed / 0 failed / 1 ignored**(S4 261 → **+166**) · green 4종 rc=0 ·
`git grep 'tracing::instrument\|tracing-attributes'` 실사용 **0**(주석 1건) · `tests/test_class_format.rs` **4/4**.
> ★**개명 충돌은 내용 충돌보다 위험하다** — 3-way 가 rename 을 놓치면 우리 픽스처가 «삭제 대 수정»으로 나타나 **조용히 사라질 수 있다.**
> ⇒ S8 은 「물량」이 아니라 **별도 판정 회차**로 잡아라(발권 전 `--find-renames` 로 대응관계를 먼저 고정).

### ★★[2026-09-04] S6 착지 기록 — ★**「새 충돌 0」은 «델타»였다. 「풀 것이 없다」가 아니다**

**착수 시 재측정**(★§5 상시 규칙 · base 병기): base = `origin/main` **`a0b5d3c`** · `merge-base` **`c4665b0`**
⇒ 컷 `95ebc5c` 충돌 ★**1건** — `java_runtime/src/classes/java/lang/string.rs`.

★★**예측(0)과 어긋난 것이 «아니다» — 축이 다르다. 이 구별을 여기 못박는다**:

| 무엇을 잰 수인가 | base | 값 |
|---|---|---|
| 2026-09-03 재측정의 「S6 새 충돌 **0**」 = ★**델타**(새로 «나타난» 파일 수) | `8c1238b`(merge-base `3296139c`) | 누적 `c4665b0` **3** → `95ebc5c` **3** ⇒ 델타 **0** |
| 이번 착수 재측정 = ★**누적**(그 base 에서 실제로 열리는 파일 수) | `a0b5d3c`(merge-base `c4665b0`) | ★**1** |

⇒ ★★**둘 다 참이다.** `string.rs` 는 S5 에서 «이미» 충돌 집합에 있었으므로 S6 에서 «새로» 나타나지 않았고(델타 0),
S5 착지로 base 가 옮겨간 뒤에도 **우리 쪽 분기가 남아 있어**(`c4665b0`→`origin/main` **+8/−28** = S5 의 설계 판단 산물)
upstream 이 그 파일을 만지는 한(이 구간 **+402/−121**) **계속 열린다**.
★★**§5 상시 규칙에 한 줄 보탠다 — 충돌 수를 적을 때는 base 와 «함께» ★«델타인가 누적인가»도 밝혀라.**
base 만 병기하면 「0」이 「풀 것이 없다」로 읽힌다 — 이번이 정확히 그 형태였다.

**해소 — `string.rs` import 블록 «합집합»**(충돌면은 import 한 곳뿐):
우리 `charset::Charset` **유지** + upstream 의 재구조화 `classes::java::{lang::{Object, System},
util::{Formatter, Locale, regex::{Matcher, Pattern}}}` **채택**.
검증: `Charset::from_name`·`Charset::resolve` 호출부 **4곳 생존** · ★**S5 가 버린 `decode_str`/`encode_str` 재유입 0**.

★★**계약4⒝ 정독이 「충돌 0으로 들어온」 파손 1건을 «테스트를 돌리기 전에» 잡았다 — 이 형태는 이번이 «세 번째»다.**
upstream 이 이 구간에 **새로** 넣은 `java_runtime/tests/classes/java/util/regex/test_pattern_syntax_exception.rs` 가
`System.setProperty` 를 **`)Ljava/lang/Object;`** 로 **3곳** 부른다(우리는 PR #5 에서 JDK 규격대로 `String`).
★**신규 파일이라 충돌이 «날 수가 없다»** — `merge-tree` 가 원리적으로 못 보는 자리다.
⇒ 서술자만 `String` 으로 맞췄다(S5 가 5곳에 적용한 **확립된 처분**과 같다 · `Properties.setProperty` 의 `Object` 는 JDK 규격상 옳아 **무접촉**).
★**전례**: S3 `tests/test_class_format.rs` 3건 → S5 io 테스트 5곳 → ★**S6 regex 테스트 3곳.**
⇒ ★**「우리가 JDK 규격에 맞춘 것 ↔ upstream 이 안 맞춘 것」이 매 회차 새 파일로 재유입된다.**

**계약4⒞ `Cargo.lock` — S5 를 문 자리를 먼저 봤다**: ★**내려간 크레이트 0건**(`async-trait` **0.1.92 유지** ·
upstream `95ebc5c` 의 lock 도 이미 0.1.92) · 올라간 3(`event-listener` 5.4.1→5.4.2 · `regex-automata` 0.4.14→0.4.16 ·
`regex-syntax` 0.8.10→0.8.11) · 추가 `regex` · 제거 `concurrent-queue`·`crossbeam-utils`.

**착지 실측**: `merge-base origin/main upstream/main` ★**`c4665b0` → `95ebc5c`** · behind **24 → 13** ·
머지커밋 **부모 2개**(`a0b5d3c` + `95ebc5c`) · `95ebc5c` 대비 **삭제 파일 0** ·
`cargo test --all` ★**554 passed / 0 failed / 1 ignored**(S5 427 → **+127**) · stable 4종 + ★**beta 2종 전건 rc=0** ·
`git grep 'tracing::instrument'` 실사용 **0**(주석 1) · `charset.rs` 실재 · 픽스처 4파일 · `test_class_format.rs` **4/4**.

★**S7 참고(이 회차는 «안 했다»)**: 같은 base 에서 `ba5797b` 누적 충돌 = **2건**(`string.rs` · `thread.rs`) ·
구간 `95ebc5c..ba5797b` **1커밋**. ★**그 수도 S6 착지로 base 가 또 바뀌므로 S7 착수 시 다시 재라.**


### ★★[2026-09-04] S7 착지 기록 — 계획 **7회차 완주** · `thread.rs` 는 «직교»였다

**착수 시 재측정**(★신 서식 그대로): 컷 `ba5797b` 충돌 = ★**누적 1 · 델타 +1**
(base `origin/main` **`3e02f8c`** · merge-base **`95ebc5c`**) — `java_runtime/src/classes/java/lang/thread.rs`.
★**「예측대로였다」가 아니라 「재서 1 이었다」**(§5 예측도 1 이었으나 그것을 전제로 쓰지 않았다).
★**`string.rs` 는 이번 집합에서 빠졌다** — S6 이 해소했고 이 **1커밋** 구간에서 upstream 이 안 건드렸다
⇒ ★**누적은 줄어들 수도 있다**(위 서식 절의 본보기가 이것이다).

### `thread.rs` — ★**「어느 쪽이 이기나」가 아니라 «직교»다**

| | 규모(`merge-base` = `95ebc5c` 기준) | 한 일 |
|---|---|---|
| upstream `95ebc5c`→`ba5797b` | **+23/−10** | ★`invoke_virtual` 에 **«선언 클래스» 인자 추가**(#201 가상 디스패치 해석) + `<init>(Z)V` 에 `PRIVATE` |
| 우리 `95ebc5c`→`origin/main` | **+50/−42** | ★PR #4 의 **수동 span**(`#[tracing::instrument]` 금지 · tokio-rs/tracing#3388). ★**의미 변경은 3줄**이고 나머지는 `async {}` 로 감싸며 생긴 **들여쓰기**다 |

⇒ ★★**두 변경은 «호출 인자»와 «그 호출들을 감싸는 span» 이라 의미가 겹치지 않는다 — 우열 판정 대상이 아니다.**
텍스트가 같은 자리에 있어 충돌했을 뿐이다.
**처분**: 우리 구조(수동 span·들여쓰기)를 **뼈대**로 두고 upstream 의 새 인자 **3곳**을 그 안에 얹었다
(`"java/lang/Thread"` · `&exception.class_definition().name()` · `"java/io/StringWriter"`).
★**S1·S3·S4 에서 3회 확립된 전략과 «같다»**(upstream 본문 + 수동 span 재적용) — 이번이 **4회째**다.
**검증**: 수동 span 2요소(`info_span!` `:269` · `.instrument(span)` `:329`) **생존** ·
`invoke_virtual` **9곳 전건 새 시그니처** · `#[tracing::instrument]` 어트리뷰트 실사용 **0**(주석 1건뿐).

### ★★「충돌 0으로 들어온」 파손 — **4회째**이고 ★**축은 «처음»이다**

upstream 이 `invoke_virtual` 시그니처를 바꾸자 ★**«우리 고유 테스트» 5곳**(PR #5 자산)이
**구식 4인자**로 남아 **컴파일 실패**했다(`E0061` × 5):
`test_input_stream_reader.rs:134·157`(`read`) · `test_string.rs:1035`(`getBytes`)·`:1042`·`:1117`(`getMessage`).
★★**«우리 줄»이라 upstream 이 안 건드렸고 ⇒ 충돌이 «날 수가 없다».**
⇒ ★**이 축은 앞 세 번과 «반대 방향»이다**: S3·S5·S6 은 「upstream 이 «새 파일»을 들여와 우리 규격을 안 지킨」 것이었고,
이번은 「upstream 이 «공용 API 시그니처»를 바꿔 «우리 파일»이 낡은」 것이다.
⇒ ★**그래서 「신규 파일을 훑는다」로는 못 잡는다 — 잡은 것은 «컴파일»이다.**
**처분**: upstream 자신의 관용구를 **그대로 채택**했다 —
`read`·`getMessage` 는 `&x.class_definition().name()` · `getBytes` 는 `"java/lang/String"`
(`ba5797b` 의 upstream 테스트가 쓰는 형태를 실측해 맞췄다). ★**단언은 한 줄도 안 건드렸다.**

### 「해소분 0」 · `Cargo.lock` · 스위트

`ba5797b` 대비 **삭제 파일 0건** · 다른 파일 **52건 전수가 우리 fork 고유 자산**.
★**`Cargo.lock` — 내려간 0 · 올라간 0 · 추가 0 · 제거 0**(`async-trait` **0.1.92 유지** · 이 컷은 의존을 안 건드린다).
정독은 **양쪽이 둘 다 만진 15건**만(우리 52 ∩ upstream 319) — `charset` 라우팅(`string.rs` 4 · `input_stream_reader.rs` 3) ·
`setProperty` 서술자(`system.rs`·`jvm.rs`) · `double_must_use` allow **7곳** 전건 생존.

`cargo test --all` ★**554 / 0 / 1** · beta 도 **554 동수** · stable 4종 + beta 2종 **전건 rc=0**.
★★**시험 수 증감 «0» 이고 그것이 «맞다»** — upstream 자신의 테스트 함수도 `95ebc5c` **547** → `ba5797b` **547** 이다
(`ba5797b` 는 78개 테스트 파일을 +14,730/−3,536 로 만지지만 **시그니처 스윕**이지 테스트 «추가»가 아니다).
★약화 축 **0**: `#[ignore]` **1 → 1** · 우리 테스트 함수 **558 → 558** · 단언 삭제 **0**.

**착지 실측**: `merge-base origin/main upstream/main` ★**`95ebc5c` → `ba5797b`** · behind **13 → 12** ·
머지커밋 **부모 2개**(`3e02f8c` + `ba5797b`).

★★**계획 「7회차」가 이로써 완주됐다**(S1~S7). ★**남은 behind 12 는 «개명 스윕» 구간**이고
그것은 **S8**(총괄 보류분 `2026-09-03-upstream-sync-s5-s7-remeasure#p0`)의 몫이다 — 이 회차는 **집행하지 않았다**.


### ★★★[2026-09-04] S8 착지 기록 — **개명 스윕** · ★**behind 12 → 0**(upstream 완전 따라잡음)

**착수 재측정**(신 서식): 컷 `bd42427` = ★**누적 8 · 델타 +8**(base `3fb08a8` · merge-base `ba5797b`).
★**「재서 8 이었다」** — 그 12커밋은 전부 **crates.io 공개 준비**(크레이트 개명) 스윕이다.

### 개명 대응표 — ★upstream 이력에서 «떴다»(추측 0)

`git diff --find-renames --name-status ba5797b upstream/main` ⇒ 상태 분포 ★**R 642 · M 22 · A/D 0**.

| 전 | 후 | 건수 | 최소 유사도 |
|---|---|---|---|
| `java_runtime/` | ★**`rustjava-runtime/`** | 379 | R079 |
| `test_data/` | ★**`test-data/`** | 243 | R083 |
| `jvm_rust/` | `jvm-bytecode/` | 12 | R066 |
| `java_class_proto/` | `jvm-class-proto/` | 4 | R056 |
| `java_constants/` | `jvm-types/` | 2 | R061 |
| `test_utils/` | `test-utils/` | 2 | R061 |

★**`java_runtime` 은 «2홉»이다**: `7b966e7`(→`java-runtime` · R100 399/399) → `653f543`(→`rustjava-runtime`).
★**미확인 0** — 상태 분포에 **A/D 가 0** 이므로 「표에 없는 경로」가 없다(upstream 은 **아무것도 지우지 않았다**).
★**유사도가 낮은 셋**(R056·R061)은 `Cargo.toml`·`lib.rs` 처럼 **패키지명 자체가 본문에 든 파일**이라 낮게 나온 것이고,
같은 디렉터리의 나머지가 R100 이라 «삭제+신규»가 아니다.

### ★★픽스처 판정 — modify/delete 가 **«0건»이었다**(위험 예측이 «좋은 쪽»으로 빗나갔다)

★**이 회차가 두려워한 형태는 나오지 않았다.** git 이 `CONFLICT (file location)` + **`AU`** 로 처리해
우리 고유 파일을 **새 경로로 «이미 옮겨» 두고 «이동 확인»만 요구**했다. ⇒ **`DU`/`UD`/`DD` 0건.**

| # | 파일(새 경로) | 기원 | 처분 | 근거 |
|---|---|---|---|---|
| 1 | `rustjava-runtime/src/charset.rs` | ★**우리**(PR #5 `7fd0ad8`) | ★**간다** | upstream 에 대응물 **없음** — 대체되지 않았다 |
| 2 | `test-data/TimeApi.class` | ★**우리**(PR #2 `13ab950`) | ★**간다** | 〃 |
| 3 | `test-data/TimeApi.txt` | ★**우리**(PR #2) | ★**간다** | 〃 |
| 4 | `test-data/UnsupportedCharset.class` | ★**우리**(PR #5) | ★**간다** | 〃 |
| 5 | `test-data/UnsupportedCharset.txt` | ★**우리**(PR #5) | ★**간다** | 〃 |
| 6 | `test-data/src/UnsupportedCharset.java` | ★**우리**(PR #5) | ★**간다** | 〃 |

★**「남는다/버린다」는 하나도 없다** — 여섯 다 **우리 것**이고 upstream 판본으로 **대체된 것이 없다**
(「우리 것이면 지우지 마라」가 기본값이고, 지울 근거가 나오지 않았다).
★★**보존 증명은 «블롭»으로 했다** — `origin/main:<옛 경로>` 블롭 ↔ 새 경로 `hash-object` 가 **6건 전부 동일**.
★**픽스처 수 5 → 5** · 옛 디렉터리 **6개 전부 소멸**(개명 완료).

### ★★개명이 «우리 자산 2건»을 낡게 만들었다 — 「우리 자산이 낡는」 **5회째**

★**축은 S7 과 같은 방향**(upstream 변경 → 우리 파일이 낡음)인데 **대상이 처음**이다 — **CI 설정과 경로 문자열**:

| 자산 | 무엇이 낡았나 | 처분 |
|---|---|---|
| `.github/workflows/rust.yml:55` | `--exclude test_utils` 가 **옛 크레이트 이름** ⇒ wasm32 셀이 깨진다(★실측: 옛 이름 **rc=101** ↔ 새 이름 **rc=0**) | `test-utils` 로 |
| `tests/test_class_format.rs`(PR #3) | `"test_data/Hello.class"` **경로 문자열 4곳** | `test-data/` 로 |

★**둘 다 «우리 파일»이라 충돌이 «날 수가 없다»** — 개명 대응표를 그대로 적용했을 뿐이고 **upstream 코드는 안 건드렸다**.
★**전례**: S3 3건 → S5 5곳 → S6 3곳 → S7 5곳(공용 API) → ★**S8 2건(CI·경로)**.
⇒ ★**이제 방향이 셋이다**: ⑴upstream 신규 파일 ⑵upstream 공용 API 변경 ⑶★**upstream 개명**.
★**⑶은 컴파일이 «절반만» 잡는다** — `test_class_format.rs` 는 런타임 실패이고 `rust.yml` 은 **CI 에서만** 드러난다.

### ★`Cargo.lock` — 계약6⒞ 가 «또» 잡았다(S5 와 같은 형태)

`--theirs` + `cargo build` 가 ★**3개를 «내렸다»**: `tracing` **0.1.44 → 0.1.41** ·
`tracing-subscriber` 0.3.23 → 0.3.20 · `syn` 3.0.4 → 3.0.2.
★★**`tracing` 하강은 PR #4 를 되돌리는 것이다** — 그 PR 의 산출물이 정확히
「`tracing-attributes` 상한 핀 제거 ⇒ tracing **0.1.41 → 0.1.44 언프리즈**」였다(upstream `bd42427` 의 lock 은 **0.1.41**).
⇒ **처방은 S5 와 같다**: `origin/main` 의 lock 에서 출발해 다시 `cargo build`
⇒ ★**내려간 것 0** · `tracing` **0.1.44 유지** · `tracing-attributes` **부재 유지**.
남는 변화는 **워크스페이스 멤버 개명**(6 제거 / 6 추가)과 `classfile`·`jvm` **0.0.1 → 0.1.1**(upstream 이 올린 판 번호)뿐이다.

### `thread.rs` — 유일 내용 충돌 · «직교» 합집합

upstream 은 크레이트 개명 import(`java_class_proto`→`jvm_class_proto` · `java_constants`→`jvm_types`),
우리는 PR #4 의 `use tracing::Instrument;` ⇒ ★**의미가 겹치지 않는다** ⇒ 합집합. 수동 span 2요소 **생존**.

### green · 착지 실측

stable 4종 + ★beta 2종 **전건 rc=0** · `cargo test --all` ★**554 / 0 / 1** · beta 도 **554 동수**.
★**증감 0 이고 그것이 맞다** — upstream 테스트 함수도 `ba5797b` **547** → `bd42427` **547**(개명 스윕이라 추가 0).
★약화 0: `#[ignore]` **1 → 1** · 우리 테스트 함수 **558 → 558**.
우리 자산 전수 생존: `charset.rs` 1 · `Charset::` 4곳 · 수동 span 2 · `tracing-attributes` **0** ·
`setProperty` `String` 6곳 · `double_must_use` allow **9곳** · 픽스처 **5** · `test_class_format.rs` **4/4**.

★★**계보**: `merge-base origin/main upstream/main` ★**`ba5797b` → `bd42427`** ·
behind ★**12 → 0** · 머지커밋 **부모 2개**(`3fb08a8` + `bd42427`).
⇒ ★★★**upstream 을 «완전히» 따라잡았다.** ★`--squash` 였다면 부모가 접히며 `merge-base` 가 `ba5797b` 로
되돌아가고 behind 가 **0 → 12** 였다.


★**S1~S3 이 이 동기화의 «전부»다** — 세 회차가 판단을 다 쓰고, 각각 **한 축씩만** 다룬다.
검수자가 한 회차에서 읽어야 하는 것은 **우리 해소분**이지 upstream 원본 diff 가 아니다:

- **S1~S3**: 충돌 해소가 실물이다. 검수 대상 = 해소 hunk(각각 2·5·9파일) + §4 위험항 대응.
- **S4~S6**: 우리 해소분이 **0**이다. 검수 대상 = ⑴green ⑵★**「해소가 정말 0인가」의 증명**:
  `git diff <merge-result> <cut> -- <우리 고유 파일들>` 이 예상분(STATE·REPORT·`charset.rs`·
  `tests/test_class_format.rs`·`test_data/*`)만 남기는지. ⇒ **한 회차가 커도 검수는 짧다.**
- **S7**: 충돌은 3건뿐이나 diff 가 32만 줄대다. 위와 같은 «해소분만 읽기» 규율로 처리한다.

**각 회차 green 기준(전 회차 공통 · CI `rust.yml` 과 동일)**:
`cargo fmt --all -- --check` · `cargo clippy --all -- -D warnings` ·
`cargo clippy --workspace --exclude test_utils --target wasm32-unknown-unknown -- -D warnings` ·
`cargo test --all`. 기준선 = **149 passed / 0 failed**(현 `origin/main` 실측).

**회차별 추가 완료 조건**:

| 회차 | 추가 조건 |
|---|---|
| S1 | `git grep -n 'tracing::instrument\|tracing-attributes'` → **0건**(§4-C) |
| S2 | `test_data/UnsupportedCharset` 통과 + `test_isr_iso_8859_1` 이식 + US-ASCII Reader 잠금 신규(§4-B) |
| S3 | `tests/test_class_format.rs` 4건 통과(문구 단정 완화 · 종류 단정 유지) + `charset.rs` 배선으로 dead code 0(§4-A·§4-B) |
| S4~S7 | 우리 해소분 0 증명(위) |

---

## 6. 이 접근안이 **하지 않는** 것

- upstream(`dlunch/RustJava`) 발신 0. `classfile::ClassFileError` 에 진단 변형을 되돌려 넣는 안은
  upstream PR 이 필요하므로 **이 리니지 밖**이다.
- PR #8(`feat/rustjava-claude-md-prune`) 무접촉. `CLAUDE.md` 는 충돌 19파일에 **없다** ⇒ 독립 처분 가능.
- `wie-ktf-hardening` 잔존 2건(STATE ②) 무접촉. 단 ★**S7 착지 후 재판정 필요** — `ba5797b` 가
  광범위 스윕이라 `System.arraycopy`·`String.<init>` null 가드가 삼켜졌을 수 있다.
- STATE ④ invokedynamic 축 무접촉. **S3 이 `jvm_rust/src/class_definition.rs` 에 `verifier::verify` 를
  들여오는 순간 패닉 축이 소멸**하므로, 그 티켓의 전제는 **S3 착지 후 다시 세운다**.
