# REPORT

## [2026-08-17] upstream 동기 S1 — 컷 `1f356ae` 머지 (rustjava-upstream-sync-s1-tracing-cut-1f356ae)
- 무엇을: upstream `1f356ae`(#173~#179 · 5커밋)를 머지했다. 충돌 **2** 해소 —
  `lang.rs` 는 **양쪽 병합**(우리 `class_format_error` + upstream 의 Java 1.2 wrapper 9종),
  `thread.rs` 는 **upstream 뼈대 + PR #4 수동 span 재적용**(`#[tracing::instrument]` 한 줄만 치환,
  `Cargo.toml` 2개 무접촉). 66파일 `+5,235 / −151`.
- 왜: 접근안 §6 이 정한 7회차 중 첫 회차이고 축은 tracing 이다. upstream `thread.rs` 를 그대로 취하면
  `attributes` 피처가 꺼진 tracing 에 속성 매크로가 걸려 **컴파일이 깨지고**, 피처를 되살리면 PR #4 가
  통째로 되돌아간다. 뼈대만 취하고 span 만 수동으로 되돌려 둘 다 피했다.
  ★**충돌 목록 밖에서 하나가 더 깨졌다**: 우리 PR #5 가 JDK 규격에 맞게 고친
  `System.setProperty` 서술자(`…)Ljava/lang/String;` — 실제 javac 바이트코드가 그렇다)와
  upstream 의 구판(`…)Ljava/lang/Object;`)이 어긋나, upstream 이 새로 들여온 wrapper 테스트 3건이
  `NoSuchMethodError` 로 죽었다. 우리 서술자를 유지하고 upstream 테스트 호출부 6곳을 고쳤다.
- 사용자 영향: 없음(동작 변경 0). Java 1.2 wrapper 클래스 9종
  (`Boolean`/`Byte`/`Character`/`Double`/`Float`/`Long`/`Number`/`Short` · `ClassNotFoundException`)과
  `Thread.currentThread()` 동일객체 반환이 들어왔다. `cargo test --all` **169 passed / 0 failed / 1 ignored**
  (기준선 149 → +20, 전부 upstream 신규 + 우리 기존분).
- 후속 추천: S2(컷 `af4f6f8` · charset 축 · 새 충돌 +5). ★착수 시 충돌 재측정 필수 ·
  ★**우리 프로덕션 서술자/시그니처 변경이 upstream 신규 테스트와 어긋나는지**를 S1 과 같은 방식으로 훑어라.

## [2026-08-16] upstream 동기화 접근안 확정 (rustjava-upstream-sync-approach-plan)
- 무엇을: 격차를 오늘 값으로 다시 재고(**10 앞섬 / 33 뒤처짐** · 충돌 **17 → 19파일**), 충돌 19파일을
  처분 어휘 4종으로 분류한 표와 단계 분할안을 `docs/upstream-sync-approach.md` 로 확정했다.
  **머지 실행 0 · 충돌 해소 0 · 코드 변경 0**(문서만).
- 왜: 충돌의 성격이 「어느 쪽 구현을 남기는가」라 머지 도중 즉흥 결정이 불가능했다. 실제로 실측해 보니
  선행 전제 2건이 틀렸다 — ⑴add/add 두 파일은 **의미 차이 0**이라 «정면 충돌»이 아니고 진짜 설계
  결정은 `classfile/src/error.rs` 하나뿐이며 ⑵charset 퇴행 범위는 `string.rs` 가 아니라
  `input_stream_reader.rs` 하나다(upstream 이 동일 charset 집합을 독립 구현했고 기본 경로에선 더 옳다).
- 사용자 영향: 아직 없다(문서 전용). 다만 ★**충돌 목록에 없는 파일 3곳이 조용히 깨진다**는 것을
  머지 전에 잡았다 — `tests/test_class_format.rs` 3건 실패 · `thread.rs`×`Cargo.toml` tracing 함정
  (그대로 취하면 컴파일 파괴, 되살리면 PR #4 되돌림) · `charset.rs` dead code 로 clippy red.
- 후속 추천: `rustjava-upstream-sync-s1`(컷 `1f356ae` · 새 충돌 2 · tracing 축) 발권. ★구판
  `-32-commits` 단일 티켓은 폐기 — 컷별 실측상 **19충돌 중 16이 앞쪽 7커밋에 몰려** 있어 커밋 수
  분할은 무의미하다. S1~S3(설계) → S4~S7(물량) 순서로 7회차.

## [2026-08-15] upstream 격차 재실측 + 잔존분 재판정 (rustjava-lane-restart-upstream-sync-precondition)
- 무엇을: `origin/main` ↔ `upstream/main` 격차를 오늘 값으로 다시 재고(**9 앞섬 / 32 뒤처짐** —
  구판 「20 뒤처짐」은 낡았다), 선행조건이던 upstream `agent/runtime-api-gaps` 의 상태를 확정하고,
  `wie-ktf-hardening` 유효 잔존을 **4건 → 2건**으로 재판정해 STATE.md `## 다음` 을 전면 갱신했다.
  코드 변경 0(문서만).
- 왜: 이 레인이 25시간 무배차로 멈춰 있었고 근인이 **낡은 `## 다음`** 이었다. 특히 선행조건으로
  걸려 있던 `agent/runtime-api-gaps` 는 「미머지」가 아니라 **PR #190 으로 2026-07-25 04:59Z
  스쿼시 머지**(`c4665b0`)돼 브랜치까지 삭제된 상태였다 — 즉 잔존분 판정 기준이 통째로 바뀌어
  있었는데 아무도 다시 재지 않았다. 재판정 결과 Timer·StringBuffer 2건·BAIS·Class.forName·
  Integer.byteValue/shortValue 는 upstream 이 **삼켰고**, `System.arraycopy` 와
  `String.<init>([B)/([C)` 의 null 가드 **2건만** 유효하게 남았다.
- 사용자 영향: 런타임 동작 변화 없음. 다음 회차 작업이 무효 6건을 중복 구현하는 낭비가 사라졌다.
- 후속 추천: ①`rustjava-upstream-sync-32-commits`(P1·L) — 충돌 예상 **17파일** 실측 완료.
  ★머지 시 upstream 의 `input_stream_reader.rs`(UTF-8/EUC-KR 하드코딩)를 그대로 취하면 우리
  PR #5 의 charset 일반화가 **퇴행**하니 `test_data/UnsupportedCharset` 로 잠그고 진행할 것.
  ②그 뒤 null 가드 2건(형제 `String.<init>([BII)` 포함) — 원인은 `ClassInstanceRef::deref` 의
  `unwrap()` 이라 전역 수리가 불가하고 진입부 가드가 정답이다.
  ③invokedynamic `todo!()` + 상수풀 태그 15~18 미지원은 ★**아직 미해결**이다(양쪽 브랜치에서
  실측 확인). 「이미 처리됐다」는 통설을 STATE 에서 정정했다.
- ※**[2026-08-16 게이트② 반려 반영]** 같은 PR 위에 문서 4곳을 고쳤다(코드 여전히 0).
  ⒜★**「열린 PR 0」이 거짓이었다** — 실측 **2건**(#9 · ★**#8 `[rustjava-claude-md-prune]` 이 11일째
  좌초 · `.review.md` 부재**). ⑤를 실측 표로 교체하고 **#8 처분 브리프를 목록 맨 위에** 넣었다.
  ⒝★invokedynamic 은 upstream `jvm_rust/src/verifier.rs` 가 `UnsupportedFeature` 로 **거부**한다 ⇒
  **①머지로 패닉 축이 소멸**하므로 「한 티켓으로 묶는 이유」를 다시 썼다. ⒞null 가드 형제 열거를
  **전수 7건**으로 확대(`([BLjava/lang/String;)`·`([BIILjava/lang/String;)`·`(Ljava/lang/String;)`·
  `(Ljava/lang/StringBuffer;)` 추가). ⒟★`system.rs` 는 충돌 목록에 **없다** — 선행 근거를 `string.rs`
  하나로 정정.

## [2026-07-25] 원격 잔존 브랜치 위생 판정 (2026-07-25-rustjava-branch-hygiene)
- 무엇을: 포크(Jun025/RustJava)에 남아 있던 원격 브랜치 2건을 실측 대조로 판정했다.
  `dependabot/cargo/tracing-attributes-0.1.31` 은 삭제하고, `wie-ktf-hardening` 은
  혼재 판정으로 보존 + 커밋별 판정표를 제출했다. 코드 변경은 없다.
- 왜: dependabot 브랜치는 PR #4(`fa92ef9`)가 `tracing-attributes` 직접 의존을 통째로
  제거해 패치 대상 라인 자체가 사라졌다 — 머지해도 적용되지 않는 죽은 패치다.
  `wie-ktf-hardening` 은 `git cherry` 가 12커밋 전부를 미반영(`+`)으로 표시했지만, 실제로는
  내용이 upstream 에 스쿼시 머지(#174·#175·#176·#177·#180·#182)돼 있었고 우리 `origin/main`
  이 upstream 보다 20커밋 뒤처져 있어 생긴 착시였다. 12건 중 8건 반영/대체, 4건만 유효 잔존.
- 사용자 영향: 원격 브랜치 목록이 `main` + 판정 보류 1건으로 정리됐다. 런타임 동작 변화 없음.
- 후속 추천: ① `origin/main` ← `upstream/main` 20커밋 동기화가 선행돼야 한다(그래야 잔존분이
  4건으로 확정되고, upstream 이 `GlobalRef`(#182)로 다르게 푼 스레드 루팅과의 이중 적용을 피한다).
  ② 이후 잔존 4건(Timer 1회성 schedule, StringBuffer.insert, append([CII) null→NPE,
  arraycopy·String.<init>([B)([C)·BAIS.<init>([B) null 가드 + Integer.byteValue/shortValue)만
  추려 PR 1건 — 전부 호스트 프로세스를 죽이는 실제 패닉이라 upstream 상납 가치도 있다.
  ③ 착수 전 upstream `agent/runtime-api-gaps`(미머지, +33k lines) 중복 여부 확인.

## [2026-07-22] 미지원 charset 패닉 → UnsupportedEncodingException (rustjava-unsupported-charset-exception)
- 무엇을: `String.getBytes(charset)`/`new String(byte[], charset)`/`InputStreamReader.read()`의
  `unimplemented!()` 패닉을 `java.io.UnsupportedEncodingException`(신설, IOException 하위) throw 로
  전환하고, String↔Reader 의 지원 charset 목록을 공용 `charset::Charset` 으로 일원화했다.
- 왜: charset 이름은 자바 코드가 넘기는 완전한 사용자 입력인데 미지원 이름 한 줄에 호스트
  프로세스가 죽었다. Reader 쪽은 ISO-8859-1 조차 못 받는 String 쪽과의 불일치도 있었다.
- 사용자 영향: `"hi".getBytes("UTF-16")` 류가 이제 try/catch 로 잡히는 자바 예외가 되고,
  `file.encoding=ISO-8859-1` 후 InputStreamReader 도 정상 동작한다. 부수 교정:
  `System.setProperty` 반환 시그니처를 JDK 규격(`...)Ljava/lang/String;`)으로 수정,
  `Throwable.getMessage()` 신설.
- 후속 추천: ① `Charset` 공용화를 계기로 UTF-16/Shift_JIS 등 실제 인코딩 추가는 별건 티켓으로.
  ② InputStreamReader 가 read 마다 스트림 디코더를 새로 만들어 버퍼 경계의 multibyte 부분
    시퀀스가 유실될 수 있는 기존 문제(EUC-KR)가 남아 있다 — 별건 조사 권장.

## [2026-07-22] tracing-attributes 상한 핀 제거 (rustjava-tracing-attributes-pin-removal)
- 무엇을: 워크스페이스 유일의 `#[tracing::instrument]`(thread.rs, "java thread" span)를
  `tracing::info_span!` + `Instrument` 수동 span 으로 대체하고, `java_runtime` 의
  `tracing-attributes <0.1.29` 직접 의존 핀과 workspace `tracing` 의 `attributes` 피처를 제거.
  Cargo.lock 은 tracing 계열만 국소 갱신(tracing 0.1.41→0.1.44, subscriber 0.3.20→0.3.23,
  tracing-attributes 그래프에서 소멸). wasm32 clippy CI 의 누락 커버리지도 교정
  (`--workspace --exclude test_utils` — test_utils 는 tokio rt-multi-thread 라 wasm 불가).
- 왜: 한 줄의 attribute macro 가 no_std 빌드를 깨는 탓(tokio-rs/tracing#3388)에 tracing 계열
  전체가 동결됐고 dependabot PR 이 해석 불가로 계속 죽었음.
- 사용자 영향: tracing 계열 업데이트 재개 가능(보안 패치 포함). span 출력("java thread{id=N}"
  이름·필드·레벨·타깃)은 실행 대조로 동일함을 확인 — 관측 회귀 0.
- 후속 추천: ① dependabot 재시도 유도(다음 주기에 자동), ② javac 21 익명 내부 클래스 파싱
  실패(Malformed) 원인 조사 별건, ③ wasm32 에서 test_utils 대체 테스트 전략 검토.

## [2026-07-22] 클래스파일 파싱 실패 → ClassFormatError 전파 (rustjava-classfile-parse-error-propagation)
- 무엇을: `ClassInfo::parse` 를 `Option` → `Result<_, ParseError>` 로 바꿔 실패 원인(절단/매직
  불일치/미지원 상수풀 태그 N/기타 손상)을 담고, `from_classfile` 의 `unwrap()`/`assert_eq!` 를
  제거해 `define_class` 에서 기존 예외 관례(`jvm.exception`)대로 `java.lang.ClassFormatError` 로
  올림. `java/lang/ClassFormatError` 런타임 클래스(부모 LinkageError) 신설.
- 왜: 손상되거나 미지원 항목(javac 9+ 가 기본으로 심는 invokedynamic 계열 태그 15~18)을 가진
  class 파일을 여는 순간 Rust 패닉으로 프로세스(임베딩 호스트 포함)가 즉사했음. "클래스 못 찾음"
  은 예외인데 "못 읽음"만 패닉인 비대칭.
- 사용자 영향: 잘못된 class 파일이 진단 가능한 자바 예외(원인 메시지 포함)로 보고되고 프로세스는
  살아남음. 미지원 태그는 명확히 거절(구현 아님). `tests/test_class_format.rs` 4케이스(절단/태그
  18/매직/못찾음 대조군)가 회귀 잠금.
- 후속 추천: ① invokedynamic/MethodHandle 실제 지원(별건 대형), ② 상수풀 인덱스 참조
  (`.get().unwrap()` 계열) 손상 대응(별건), ③ UnsupportedClassVersionError 도입 검토(major
  version 기반).

## [2026-07-22] 시간 API 패닉 제거 + 회귀 잠금 (rustjava-runtime-time-todo-impl)
- 무엇을: `src/runtime.rs`의 `RuntimeImpl` 에서 `now()`/`sleep()`/`r#yield()` 의 `todo!()` 를 실제
  구현(UNIX epoch ms·`tokio::time::sleep`·`tokio::task::yield_now`)으로 교체하고, `test_utils`
  `TestRuntime::r#yield` 의 `todo!()` 도 동형으로 구현. 루트 `Cargo.toml` tokio 에 `time` 피처 추가.
- 왜: 배포 바이너리가 `System.currentTimeMillis()`·`Thread.sleep()`·`new Date()` 등 시간 API 를
  부르는 순간 Rust 패닉으로 즉사했으나, 테스트 코퍼스가 해당 API 를 0건 사용해 CI 가 초록이었음.
- 사용자 영향: 시간 API 를 쓰는 모든 자바 프로그램이 이제 정상 동작. `test_data/TimeApi`
  픽스처(currentTimeMillis/yield/sleep/Date, 결정론적 단언)가 `RuntimeImpl` 경로 통합 테스트로
  상시 회귀 감시.
- 후속 추천: ① `jvm_rust/src/interpreter.rs:629` 의 잔여 `todo!()` 제거(별건), ② Timer/Object.wait
  경로도 픽스처 확장, ③ 픽스처 .java 소스 보관 체계(현재 .class+.txt 만 커밋하는 관례).
