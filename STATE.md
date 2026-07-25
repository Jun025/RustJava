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
- ★`origin/main` 이 `upstream/main` 보다 **20커밋 뒤처짐**(2026-07-25 실측). upstream 동기화
  티켓이 선행돼야 `wie-ktf-hardening` 잔존분 정리가 안전하다.
- (wie-ktf-hardening 유효 잔존 4건 — 전부 호스트 프로세스 abort 급 패닉/미구현, upstream 상납 가치 있음)
  `Timer.schedule(TimerTask,long)` 1회성 / `StringBuffer.insert(I,String)` /
  `StringBuffer.append([CII)` null→NPE / `System.arraycopy`·`String.<init>([B)([C)`·
  `ByteArrayInputStream.<init>([B)` null 가드 + `Integer.byteValue()/shortValue()`.
- ★위 착수 전 upstream `agent/runtime-api-gaps`(6309d47, +33k lines, 미머지)가 잔존 4건을
  삼키는지 확인할 것(중복 노동 방지).
- 발권 대기 태스크 없음 — 열린 PR 0, origin 작업 브랜치는 보존 판정된 `wie-ktf-hardening` 1건.
- ★PR 발권 시 `--repo Jun025/RustJava` 명시(2026-07-22 upstream 오발행 사고 재발 방지).
- (범위 밖 잔여) `jvm_rust/src/interpreter.rs:629` `todo!()` (invokedynamic) — 별건 티켓 필요
- (신규 발견) javac 21 산출 익명 내부 클래스(.class)가 "Malformed class file" 로 파싱 실패 —
  원인 미조사(태그 15~18 아님). 별건 티켓 필요.
- (신규 발견) InputStreamReader 가 read 마다 스트림 디코더를 새로 생성 — EUC-KR 등 multibyte
  가 버퍼 경계에 걸리면 부분 시퀀스 유실 가능(기존 문제, 이번 범위 밖). 별건 티켓 권장.
