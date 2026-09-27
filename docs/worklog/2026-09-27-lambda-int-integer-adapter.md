## [2026-09-27] 람다 링커에 int↔Integer 어댑터 — 한 쌍만 (rustjava-2026-09-17-link-lambdametafactory-boxing-adapter-adopt-p0)
- 무엇을: `jvm-bytecode/src/lambda.rs` 의 통과 전용 술어 `adapts` 를 위치별 계획 `plan` 으로 바꿨다. 통과(pass-through)는 종전 술어 `passes` 를 그대로 먼저 묻고, 그 밖에는 `int`↔`Integer` 한 쌍만 삽입한다 — 나갈 때 `Integer.valueOf`, 들어올 때 `Integer` 검사 후 `intValue`(null → NPE · 다른 타입 → CCE, 실제 팩토리와 같은 예외). 링크 여부와 실행 시 변환이 **같은 함수**에서 나온다. 부트스트랩 셋째 인자 `instantiatedMethodType` 을 `LambdaCallSite` 에 실었다 — 지워진 `Object` 가 `Integer` 인지 `Short` 인지는 그것만 말한다.
- 왜: `2026-09-17-link-lambdametafactory#p0` 채택. `Supplier<Integer>`·`Function<T,Integer>` 모양이 원시값 구현에 닿는 자리가 전부 거부되고 있었다. 한 쌍으로 좁힌 이유 = 카드의 tradeoff(잘못 허용한 쌍은 거부를 틀린 답으로 바꾼다).
- 사용자 영향: 박싱이 필요한 람다·메서드 참조가 로드·실행된다(OpenJDK 26.0.2.1 과 출력 일치). 그 밖의 쌍(`long` 박싱 · 언박싱 후 확장 · `Number` 로 박싱)은 여전히 거부된다.

### 실측
| 무엇 | 결과 |
|---|---|
| `LambdaBoxing`(종전 거부 픽스처) | 수정 전 거부 → 수정 후 `3`(OpenJDK 와 동일) |
| `LambdaBoxingKinds`(신규 · 위치 5 + 예외 3) | `4 / 42 / 7 / obj:5 / integer:6 / npe in / cce / npe out` — OpenJDK 26.0.2.1 출력과 줄 단위 일치 |
| `LambdaBoxingLong`·`LambdaUnboxingShort`(신규 · 거부 유지) | 둘 다 `UnsupportedOperationException … invokedynamic` (OpenJDK 는 `5000000000`·`42`) |
| 통과 경우 기존 시험(`Lambda`·`LambdaKinds`·`LambdaCapturingThis`) | 불변 green |
| `verify-javac-fixtures.sh`(javac 26.0.2.1) | 33/33 재현 |

### 되돌리면 red (변이 6종 · 전부 1건 red)
| 변이 | red 가 된 시험 |
|---|---|
| M1 인자 박싱 제거 | int↔Integer 시험 |
| M1b 반환 박싱 제거 | int↔Integer 시험 (`LambdaBoxing`) |
| M2 언박싱에서 instantiated 가 `Integer` 인지 안 봄 | 거부 시험 (`LambdaUnboxingShort` 가 링크된다) |
| M3 `long` 도 박싱 허용 | 거부 시험 (`LambdaBoxingLong`) |
| M4 언박싱 전 `Integer` 검사 제거 | int↔Integer 시험 (`cce` 줄) |
| M5 반환 언박싱 제거 | int↔Integer 시험 |

### 범위 밖(카드 없음)
- 다른 쌍은 쌍마다 따로 추가한다. 지금은 필요로 하는 게스트가 없다. wie 게스트 jar 는 major 47 이라 `invokedynamic` 에 닿지 않는다(`2026-09-26-lambda-class-reflection-visibility`). 그래서 제안 카드를 만들지 않았다.
- 캡처 값은 어댑트하지 않는다(통과가 아니면 거부). javac 는 구현 타입 그대로 캡처한다.
