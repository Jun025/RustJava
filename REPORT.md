# REPORT

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
