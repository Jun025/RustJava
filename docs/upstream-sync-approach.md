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
| **S5** | `c4665b0` (#190) | 6 | 171f +33,138/−1,058 | **0**(`Cargo.lock`만) | Java 1.2 API 확장 | 물량 |
| **S6** | `95ebc5c` | 11 | 142f +17,593/−483 | **0** | regex · Formatter · logging · LinkedHashMap | 물량 |
| **S7** | `ba5797b` (#201) | 1 | 319f +20,118/−5,729 | **+3** (`class_format_error.rs`·`throwable.rs`·`Cargo.lock`) | 접근 플래그 · 가상 디스패치 스윕 | 물량+ |

★**「새 충돌」은 «해당 컷에서 처음 충돌하는 파일 수»다.** 앞 회차가 착지하면 뒤 회차의 기준선이
바뀌므로 **각 회차 착수 시 재측정이 필수**다(§0 의 교훈 그대로).

> ★★**[2026-08-27 정정 — 이 표의 「새 충돌」은 «base 가 전진할 때»의 수다. 그 전제는 S1~S4 에서 깨져 있었다.]**
> 위 「앞 회차가 착지하면 뒤 회차의 기준선이 바뀐다」는 **거짓이었다.** 게이트③이 제품 repo 를
> `--squash` 로 착지시키므로 PR 브랜치의 upstream 머지 부모가 버려지고, `origin/main` 은 **내용만**
> 받은 채 **계보는 fork 시점(`62cf0c6a`)에 머문다.** ⇒ `git merge-base origin/main upstream/main` 이
> 전진하지 않으므로 **다음 회차는 앞 회차가 이미 닫은 충돌을 처음부터 다시 연다.**
>
> 실측(S1~S4 착지 «후»): 머지커밋 `6bfe97c4`·`11ef5010`·`4bb796de`·`3a597768` **전건 부모 1개**(=squash) ·
> `merge-base` **`62cf0c6a` 불변** · behind **33** · 회차별 충돌 **2 → 5 → 11 → 20 단조증가**
> (표의 예측치 2 → +5 → +9 → **0** 과 어긋난다 — 특히 S4 는 「0」 예측에 **20**을 만났다).
>
> ⇒ ★**S5~S7 의 「0」(`Cargo.lock`만)도 같은 전제 위의 수다. 그대로 믿지 마라 — «하한»으로 읽어라.**
> 이 표를 쓰기 전에 **반드시** `git merge-base origin/main upstream/main` 을 먼저 재고, 그것이
> 직전 컷으로 전진해 있지 않으면 **해당 회차 착수 시 `merge-tree` 로 전량 재측정**한다.
>
> **처분**: 계보 기록 커밋(`git merge -s ours <컷>` · 트리 변경 0)으로 `merge-base` 를 `3296139`
> 까지 끌어올렸다(behind 33 → 18). ★**단 이 커밋 자체가 `--squash` 로 착지하면 무의미하다** —
> 게이트③ 예외 판정은 총괄 소관(`REPORT.md` 후속 추천 참조).

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
