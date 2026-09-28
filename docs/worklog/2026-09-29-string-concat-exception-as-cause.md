## [2026-09-29] 문자열 연결 레시피 불일치 — 상세를 cause(StringConcatException)로 싣는다 (rustjava-java-lang-invoke-package-and-linkage-error-cause)
- 무엇을: `java.lang.invoke` 패키지를 만들고 `StringConcatException` 한 클래스만 넣었다(`<init>(String)` 만 — `Jvm::exception` 이 부르는 것). `concat_with_constants`(`jvm-bytecode/src/interpreter.rs`)가 불일치 시 `BootstrapMethodError: bootstrap method initialization exception` 을 던지고 상세는 `initCause` 로 단 `StringConcatException` 에 싣는다. 인자 → 상수 순으로 보고, 문구는 OpenJDK 26 것이다.
- 왜: `2026-09-17-string-concat-recipe-arity#p1` 채택. 선행 회차(`rustjava-linkage-errors-carry-their-cause`, 2026-09-18 blocked)가 막힌 이유였던 «cause 로 지목할 클래스 부재» 를 풀었다. cause 배관(`Throwable.initCause`·`printStackTrace` 의 `Caused by:`)은 이미 있었다.
- 사용자 영향: 오류 출력이 실 JVM 과 같은 두 줄 모양이 된다. 판정 경계(무엇을 거부하나)는 그대로다.

### 실측 (OpenJDK 26.0.2.1 ↔ 이 런타임, `test-data/indy/RecipeWants*.class`)
| 픽스처 | OpenJDK cause | 이 런타임 cause |
|---|---|---|
| `RecipeWantsFewerArguments` | arguments: wants 1, provides 2 | 같음 |
| `RecipeWantsMoreArguments` | arguments: wants **1**, provides 1 | arguments: wants **2**, provides 1 |
| `RecipeWantsAConstant` | constants: wants **0**, only 0 | constants: wants **1**, only 0 |

숫자가 다른 두 줄은 일부러 맞추지 않았다. OpenJDK 는 파서가 멈춘 시점의 계수를 찍어서 파일을 설명하지 못한다. 문구와 cause 타입은 같다.

### 시험
- `test_a_recipe_that_contradicts_its_call_site_is_a_bootstrap_method_error` 가 부분문자열 대신 머리줄 3개(`Java Exception:` · 바깥 오류 · `Caused by:` 줄)를 정확히 단언한다.
- 수정 전 런타임: red(cause 줄 없음, 상세가 바깥 메시지에 있음). 변이 `initCause` 호출 제거: red(cause 줄 없음).

### 옮기지 않은 것
`lambda.rs`·`string_concat.rs` 의 `java/lang/invoke/LambdaMetafactory`·`StringConcatFactory` 는 부트스트랩 이름을 대조하는 상수이고, 클래스로 싣는 곳이 없다. 그래서 이 패키지로 옮길 것이 없다. `MethodHandle`·`CallSite` 는 쓰는 곳이 없어 만들지 않았다.
