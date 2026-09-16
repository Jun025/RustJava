# 2026-09-16 — 부트스트랩 메서드의 «정적 인자» 인덱스를 경계 검사한다

티켓 `rustjava-adopt-bound-bootstrap-method-attr-index-p1` — 채택 제안
`2026-09-16-bound-bootstrap-method-attr-index#p1`.

★**제품 동작이 바뀐다**: 정적 인자가 상수 풀에 «없는» 인덱스를 가리키면 이제 **거부**된다
(`UnsupportedOperationException` → **`ClassFormatError`**).

## ⓐ 제안이 «지금도» 참인가 — 재서 확인했다

`classfile/src/validation.rs` 에서 `arguments` 문자열 **0건**(착수 시 실측) ⇒ 그 인덱스를 보는 검증은 **없었다**. 제안 **유효**.

## ⓑ 이미 같은 축이 있는가 — ★**있다. 그런데 «검증»이 아니라 «포기»다**

`arguments` 를 읽는 제품 코드는 **한 곳**뿐이다 — `jvm-bytecode/src/string_concat.rs:92`(StringConcatFactory 링크, PR #48).
그곳은 `ConstantPoolReference::from_constant_pool` 이 `None` 을 내면 ★**그 콜사이트를 «링크하지 않고 조용히 넘긴다»**
(`string_constant(...)?`). 즉 ★**잘못된 인덱스가 «감지»는 되는데 «판정»되지 않는다** — 파일은 계속 살아서
「이 런타임이 아직 못 하는 기능」으로 보고된다. ⇒ ★**처방이 「그곳을 고치는 것」이 아니라 「파스 시점에 거부하는 것」인 이유다.**

## ⓒ 제안이 틀렸는가 — 아니다. ★**단 «절반»만 요구한다**

JVMS 4.7.23 은 각 `bootstrap_arguments` 항목이 ⑴**풀의 유효한 인덱스**이고 ⑵그 항목이 **loadable constant** 일 것을 요구한다.
제안이 말한 것은 ⑴뿐이고, 이 회차도 ⑴만 했다 — ★**⑵는 «종류» 검사라 다른 문장**이고, 넓히면 그것은 다른 티켓이다.
⇒ 인자가 `Utf8` 를 가리키는 파일은 **여전히 통과**한다. 그 사실을 술어 doc 에 적어 뒀다.
※참조 JVM 의 문면이 그 차이를 그대로 보여 준다 — OpenJDK 26 은 **`argument_index 65535 has bad constant type`**,
즉 ⑵의 언어로 말한다(⑴을 ⑵가 삼킨 형태). 우리 문장은 ⑴이다.

## ★★제안의 급소 — 「해석으로 흐르지 마라」를 어떻게 지켰나

원문 `tradeoff`: 「**Must not drift into resolution.** … a bounds check has to stay a bounds check,
and a future reader may not see the difference.」

지킨 방법 둘:
1. **술어가 `contains_key` 하나다** — 항목을 **읽지 않는다**. 페이로드도, 종류도 보지 않는다.
2. ★**그 차이를 코드에 적었다** — 술어 doc 이 「`attribute.rs` 가 길게 설명한 그 후퇴(람다 보유 클래스가
   unsupported → corrupt 로 되돌아가는 것)와 이것이 왜 다른지」를 말한다. **다음 사람이 보라고 거기 있다.**

★**그리고 «지켰다»를 주장하지 않고 쟀다** — BSM 정적 인자를 **실제로 가진** 클래스들이 여전히 미지원인지:

| 클래스 | 정적 인자 | 결과 |
|---|---|---|
| `test-data/indy/Lambda.class` | MethodType·MethodHandle·MethodType | ★`UnsupportedOperationException: invokedynamic`(불변) |
| `test-data/indy/ConstantKinds.class` | condy·람다 다수 | ★`UnsupportedOperationException: invokedynamic`(불변) |
| `test-data/indy/StringConcat.class` | String 레시피 | ★**여전히 «실행»된다**(`a0` 출력 · PR #48 링크 경로 정상) |

⇒ ★**우려한 후퇴는 «일어나지 않았다», 그리고 그것이 실측이다.**

## 고친 자리 — 한 곳

`validate_class` 의 거부 사슬에 술어 하나(`bootstrap_method_static_arguments_are_in_the_pool`).
★`bootstrap_method_indices_resolve` **무접촉**(그 함수는 `bootstrap_method_attr_index` 한 문장이다) ·
★`attribute.rs` **무접촉**(`arguments` 는 여전히 **원시 인덱스**다 — 제안이 지키라고 한 그 설계).

## 픽스처

`test-data/ldc/LdcDynamicBSMArgPastEnd.class` — 생성기 `dynamic(...)` 에 `static_arguments=()` 인자를 더해
`0xFFFF` 하나를 실었다(기존 `attr_index=0` 와 **같은 모양**의 확장 · 그 파라미터가 존재하는 이유도 같다).
★구조 **측정**: `BootstrapMethods` 메서드 1개 · 인자 `[65535]` · 풀 유효 범위 **1..19** ⇒ 범위 밖.
★재생성 **멱등**: 기존 **11개 전건 바이트 동일** · 신규 1개.
★인덱스를 `0xFFFF` 로 고른 이유: **부재**로만 실패하게 하려는 것이다 — 풀 안의 «종류가 틀린» 항목을 가리키면
⑵(loadable) 축과 섞여 테스트가 «이름과 다른 이유»로 통과할 수 있다.

## 전/후 (end-to-end) · 참조 JVM

| 파일 | 전 | 후 | ★OpenJDK 26.0.1 |
|---|---|---|---|
| `LdcDynamicBSMArgPastEnd` | `UnsupportedOperationException: ldc of a dynamically-computed constant` | ★**`ClassFormatError: Invalid class file`** | ★**`ClassFormatError: argument_index 65535 has bad constant type`** |

## 개악 대조 (양방향 · 제품 «호출부»)

| 개악 | 결과 |
|---|---|
| **M1** — 호출부에서 술어 **제거**(= 제안 이전 상태) | ★**red** |
| **M2** — 술어 본문을 **`true`(상수 통과)** 로 | ★**red** |
| 정상 | **green**(`test_class_format` **12 passed**) |

## 잃는 것 / 안 하면 무엇이 나쁜가

⒜**잃는 것**: ⑴지금까지 로드되던 형상 하나가 거부된다 — 다만 **어떤 컴파일러도 내지 않는다**(어제 잰 값:
javac·kotlinc·scalac·Lombok 산출물에 `ldc` 태그 15/16/17 **0**이고, 이 형상은 손으로 지어야 나온다).
⑵파스 시점 비용 = **부트스트랩 메서드 수 × 인자 수**의 `BTreeMap::contains_key`(람다 많은 클래스에서도 수십 회 · `O(log n)`).
⑶★**오탐 여지는 «있다, 한 자리»**: long/double 은 풀 슬롯을 둘 먹고 **둘째 슬롯은 이 맵에 없다**(JVMS 4.4.5 의
「유효하나 사용 불가」) ⇒ 그 둘째 슬롯을 가리키는 인자는 거부된다. ★**그것이 의도된 읽기**이고(그런 인덱스는
어떤 상수도 «적재»할 수 없다) 술어 doc 에 적어 뒀다 — 맵의 우연이 아니다.
⒝**안 하면**: 유일한 소비자가 그 인덱스를 **조용히 포기**하므로(ⓑ), 아무도 못 읽는 파일이 계속
**「이 런타임이 아직 못 한다」**로 보고된다 — ★참조 JVM 이 `ClassFormatError` 로 답하는 파일에 대해서.
