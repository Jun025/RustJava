## [2026-10-01] ZipFile.close() 등재 (rustjava-zipfile-close-not-registered)

- 무엇을: `java/util/zip/ZipFile.close()V` 를 no-op 으로 등재했다. 픽스처 `ZipClose` 가 명시 `close()` 와 try-with-resources 를 둘 다 잠근다.
- 왜: `ZipFile` 을 닫는 평범한 게스트가 `NoSuchMethodError: java/util/zip/ZipFile.close:()V` 로 죽었다(픽스처로 재현).
  javac 는 try-with-resources 를 자원의 정적 타입에 대한 `invokevirtual ZipFile.close` 로 컴파일하므로(javap 확인) `Closeable` 인터페이스 없이 이 한 메서드로 두 경로가 다 산다.
- 사용자 영향: zip/jar 을 읽고 닫는 게스트가 정상 종료한다. no-op 인 이유는 `<init>` 이 아카이브 전체를 `zipData` 로 복사해 해제할 핸들이 없어서다. 그래서 close 뒤 읽기도 계속 되며, JDK 처럼 `IllegalStateException` 을 던지지는 않는다.

양방향: 수정 전 `cargo test --test test_class` → `Test ZipClose failed … NoSuchMethodError: java/util/zip/ZipFile.close:()V` · 수정 후 ok.

카드 정리: `2026-09-12-zip-getinputstream-guard-lock#p0` 채택(이 회차). 이미 착지한 것을 재측해 닫았다:
- `2026-08-27-upstream-sync-s3#p1` · `2026-09-17-lambda-capture-order-and-host-abort#p0` → PR #67 `f19acf77`(거부 사유가 `ClassFormatError` 메시지까지 간다)
- `2026-09-16-cp-tags-15-18-parse#p1` → PR #44 `dc035936`(ldc 15/16/17 이 「파손」이 아니라 「미지원」으로 보고됨 · javac 는 그 ldc 를 내지 않음을 실측)
