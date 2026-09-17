# 2026-09-17 — 다른 클래스 수준 속성에도 «개수 규칙»이 필요한가 (rustjava-adopt-reject-duplicate-bootstrap-methods-p0)

채택 제안 `2026-09-16-reject-duplicate-bootstrap-methods#p0` — 제목이 **「Decide whether …」**,
즉 이 회차가 지는 것은 **판정**이다. 제안은 자기 `why` 에 「**아무도 읽지 않는 속성에 대해서는 답이
«아니오»일 수 있다**」고 적어 두었다.

★**재서 보니 답은 «다섯에 대해 예»다. 그리고 제안이 적은 값 전제가 «거짓»이었다.**

## 1. 제안의 기준을 먼저 적용해 봤다 — 그 기준만으로는 «아니오»가 나온다

기존 검사의 주석이 스스로 기준을 적어 두었다: 「중복이 ★**하류에서 «임의의 선택»을 관측 가능하게 만드는가**」.
실측(소비자 전수 · `attribute.rs` 제외):

| 속성 | `find_map` 류 선택자로 읽는 소비자 |
|---|---|
| `BootstrapMethods` | ★**있다** — `validation.rs` + `jvm-bytecode/src/string_concat.rs:110 resolve_bootstrap_methods` |
| `SourceFile` · `InnerClasses` · `SourceDebugExtension` · `NestHost` · `NestMembers` · `Synthetic` | **0** |

⇒ ★**그 기준만 쓰면 제안의 추측대로 «아니오»다.** 판정을 바꾼 것은 **다른 오라클**이다.

## 2. ★진짜 JVM 에 물었다 — 그리고 값 전제가 뒤집혔다

OpenJDK **26.0.1** 에 한 속성씩 물었다. ★**런처 메시지(「기본 메소드를 찾을 수 없습니다」)는 진짜 원인을 가린다** —
`Class.forName` 으로 잡아 진단문을 받았다.

| 중복시킨 속성 | major | OpenJDK 26.0.1 |
|---|---|---|
| `SourceFile` · `InnerClasses` · `SourceDebugExtension` · `BootstrapMethods` | 52 | ★**`ClassFormatError: Multiple … attributes`** |
| `NestHost` · `NestMembers` | 55 | ★**`ClassFormatError`** |
| ★`NestHost` | **52** | ★**로드된다** — 그 버전엔 속성이 **정의되지 않아 무시**된다(JVMS 4.7.1) |
| ★`Synthetic` | 52 | ★**로드된다** — JVMS 4.7.8 은 at-most-one 인데 **HotSpot 은 둘을 받는다** |

⇒ ★★**제안의 `tradeoff` 「Rejecting more files that load today」는 다섯에 대해 «거짓»이다.**
그 파일들은 **오늘도 진짜 JVM 에서 로드되지 않는다.** 받아들이던 쪽이 **우리**였다.

## 3. 그래서 «표»이고 «버전 게이트»다 — 두 줄이 그 이유다

⒜★**버전 게이트가 하중을 받는다**: `NestHost` 를 major 52 에서 세면 ★**모든 JVM 이 받아들이는 파일을 거부**한다.
정의되지 않은 속성은 «중복»이 아니라 «미인식»이고, 미인식은 무시한다.
⒝★**`Synthetic` 은 «빠뜨린» 것이 아니라 «근거로 뺀» 것이다**: 스펙은 하나라는데 HotSpot 은 둘을 받는다.
넣으면 **우리가 맞추려는 그 JVM 보다 엄격**해지고, 읽는 곳도 없어 관측될 임의 선택도 없다.

★반대 방향도 의도적이다 — 필드·메서드·Code 소속 속성(`ConstantValue`·`Code`·`Exceptions`·`MethodParameters`·
`StackMapTable`·`LineNumberTable`·`LocalVariableTable`)은 클래스 속성표에 나타나도 **세지 않는다**.
거기서 그것들은 «정의되지 않은 자리의 속성» = 무시 대상이다.

## 4. 양방향 — 통제군이 «하중을 받는지»까지 쟀다

| 개악(제품 호출부) | 결과 |
|---|---|
| **M1** 버전 게이트 제거 | ★**red** — 통제군 `DuplicateNestHostOldMajor` |
| **M2** `Synthetic` 을 표에 추가 | ★**red** — 통제군 `DuplicateSynthetic` |
| **M3** `SourceFile` 한 칸 제거 | ★**red** — 그 픽스처만 |
| **M4** 호출부를 종전(BootstrapMethods 전용)으로 되돌림 | ★**red** — 다섯 건 |
| 복원 | **green** 17/0 |

★**M1·M2 가 이 회차의 요지다** — 통제군이 없으면 「전부 세면 된다」가 통과해 버린다.

## 5. ★대가

- ★**제품 동작이 바뀐다**: 그 다섯을 둘씩 가진 클래스가 **로드되지 않는다**. 전부 OpenJDK 도 거부하는 파일이지만,
  **동작 변경은 동작 변경이다**.
- ★**표는 «적어 둔 목록»이지 «유도된 것»이 아니다** — 내일 `AttributeInfo` 에 속성이 늘어도 행을 더하기 전엔 덮이지 않고,
  그것을 **울어 주는 것이 없다**.
- ★**`Synthetic` 배제는 «JVM 하나의 행동»에 걸려 있다** — HotSpot 이 조이거나 다른 JVM 이 이미 거부한다면
  그 통제군 픽스처가 **틀린 답을 고정**하게 된다.
- ★**`attribute.rs` 를 만졌다** — 제안의 `target` 은 `validation.rs` 뿐이다. 팔 **한 줄**이고, JVM 이 거부하는 속성을
  덮기 위한 **최소**였지만 **명시한 파일 밖**이라 그대로 신고한다.

## 6. 안 하면 무엇이 나쁜가

「같은 정직함을 일관되게」가 **한 속성에서 멈춘다.** 스펙이 하나라는 것을 둘 가진 파일이,
★**진짜 JVM 은 거부하는데 우리만 실행**한다 — 이 리니지가 반복해 가른 「미지원 ↔ 파손」의 반대편 실수다.
