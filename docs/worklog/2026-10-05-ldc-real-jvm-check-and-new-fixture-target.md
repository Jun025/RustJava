## [2026-10-05] ldc 양성 픽스처 실 JVM 검사 + 새 픽스처 target 규칙 (rustjava-ldc-fixtures-real-jvm-check-and-new-fixture-target-rule)

- 무엇을: `test-data/src/verify-javac-fixtures.sh` 가 javac 재빌드 뒤 ldc 양성 픽스처 4개(`LdcMethodHandle`·`LdcMethodType`·`LdcDynamic`·`Ldc2WDynamic`)를 찾은 javac 옆의 `java` 로 실행한다 — 하나라도 거부되면 rc=1. `test-data/class-file-versions.txt` 머리에 새 픽스처 규칙(`javac --release 21` = 65.0 · 다른 major 는 `.java` 머리 주석에 이유)을 적었다.
- 왜: 열린 카드 2장(`2026-09-17-ldc-asm-regeneration-declined#p0` · `2026-09-18-root-fixture-target-decision#p0`)의 남은 이유가 각각 «스크립트에 ldc 줄 없음» · «새 픽스처 규칙 미정»이었다.
- 사용자 영향: 없음(제품 코드 0줄). 손조립 ldc 바이트의 «진짜 JVM 이 받는다»는 주장이 다시 칠 수 있는 명령이 됐다.

실측(OpenJDK 26.0.2.1 · homebrew):
- 정상: `33 rebuilt: 33 reproduced, 0 differed; 0 could not be rebuilt; 0 ldc positive(s) refused` rc=0. ldc 13개 전수 실행 — 양성 4 rc=0 · 음성 대조 9 rc=1.
- 개악: `LdcDynamic.class` major 55→52 → `✗ ldc/LdcDynamic: … java.lang.ClassFormatError: Class file version does not support constant tag 17` · rc=1. 복원 후 rc=0.
- 로캘: 한국어 런처는 원인을 «main 메소드 없음» 문장 안에 섞어 낸다 → `-Duser.language=en` + `java.lang.` 줄을 보여 준다.
- 규칙 근거 계수: root 115행 중 65.0 이 62 · indy javac 픽스처 33 중 31(나머지 2 = `LambdaCapturingThis` --release 8, REF_invokeSpecial 때문 — 규칙이 요구하는 «이유 주석»의 본보기).

한계: CI 에는 JDK 가 없어 이 검사는 여전히 수동이다(형제 검사와 같다). JVM 수락은 «적법»의 증거이지 «어떤 생성기가 그 인코딩을 낸다»의 증거가 아니다.
