# 람다 클래스는 리플렉션에 보여야 하는가 — 결정: **현 상태 유지(보인다)**

2026-09-26 · 티켓 `rustjava-2026-09-17-link-lambdametafactory-reflection-visibility-decision` ·
제안 `2026-09-17-link-lambdametafactory#p1` 의 처분.

## 지금 어떻게 동작하나 (코드에서 읽은 것)

- `jvm-bytecode/src/lambda.rs` `lower` 가 호출 지점마다 `<Host>$$Lambda$<n>` 이름을 붙이고,
  `instantiate` 가 **처음 실행될 때** `jvm.register_class` 로 일반 클래스 레지스트리에 넣는다.
- `getClass().getName()` → `Host$$Lambda$0`.
- `Class.forName` (`rustjava-runtime/src/classes/java/lang/class.rs` `for_name`) → `jvm.resolve_class` →
  레지스트리를 **먼저** 본다(`jvm/src/jvm.rs` `resolve_class_internal`). 그래서
  - 그 호출 지점이 한 번이라도 실행된 **뒤**에는 그 이름으로 클래스를 찾는다.
  - 실행되기 **전**에는 로더에 그런 파일이 없으므로 `ClassNotFoundException`.

## 실제 JVM 은 (공개 규격)

- `LambdaMetafactory` Javadoc 은 생성 클래스의 이름·정체를 규정하지 않는다.
- JDK 15+ 는 hidden class(JEP 371)로 만든다 — `Class.forName` 으로 찾을 수 **없고**, `getName()` 에 `/` 가 섞인다
  (그 이전 판도 익명 클래스라 이름으로 찾을 수 없었다).
- 즉 차이는 둘이다: ⑴이름 모양 ⑵실행 후 `forName` 이 찾는다. ⑴은 규격상 미정이라 차이가 아니다. 남는 것은 ⑵ 하나.

## 이 런타임의 게스트가 그 차이에 닿는가

- **wie(J2ME 타이틀): 닿지 않는다 — 구조적으로.** `invokedynamic`·`MethodHandle`·`MethodType` 상수는 class 파일
  major 51 이상에서만 합법이고, `classfile/src/validation.rs` `constant_pool_tags_fit_the_class_file_version` 이
  그 미만을 로드 단계에서 거부한다. J2ME/CLDC 클래스는 49 이하다.
  실측(wie `origin/main` `5434ab0a` 의 jar 46개 · class 12,954개): 게스트 jar(`test_data/*_j2me.jar`)는 major **47** ·
  `LambdaMetafactory` 참조 **0**. 참조 365건은 전부 `game_lab/vendor_sdk/…/KEmulator*`·SWT·LWJGL — 데스크톱 JDK 에서
  도는 **벤더 에뮬레이터 도구**이고 이 런타임이 실행하지 않는다.
- **`rust_java` CLI 로 최신 클래스를 돌리는 경우: 닿을 수는 있다.** 그러나 차이 ⑵를 관측하려면 프로그램이
  `getClass().getName()` 결과를 다시 `Class.forName` 에 넣어야 한다 — 실제 JVM 에서는 늘 실패하는 코드라
  그렇게 쓰는 프로그램은 없다고 보아도 된다. 테스트·픽스처 중 람다에 `getClass`/`forName` 을 쓰는 것은 **0**
  (`test-data/src/indy/*`).

## 선택지와 대가

| | 무엇을 | 대가 |
|---|---|---|
| **A. 유지(채택)** | 지금 그대로 | 호출 지점당 레지스트리 1칸(JVM 수명 동안). 실행 후 `forName` 이 찾는다는 차이 1개 |
| B. `forName` 에서만 가린다 | `for_name` 에서 `$$Lambda$` 이름을 거부 | 3줄. 그러나 이름 모양으로 판정하므로 그 이름을 가진 **정상 게스트 클래스**도 못 찾게 된다(`$` 는 합법 문자) |
| C. hidden class 로 등록 | 레지스트리 밖 두 번째 등록 경로 | 새 등록 경로 하나 — 아무도 필요로 하지 않은 성질을 위한 기구 |

## 결정

**A.** 차이에 닿는 게스트 경로가 wie 에는 없고(버전 게이트), CLI 에서도 실제 JVM 에서 실패하는 코드만 닿는다.
B·C 는 아무도 필요로 하지 않은 성질을 위해 비용이나 새 오답을 산다. 제품 코드 변경 0.

**알고 두는 것**: 게스트가 `Host$$Lambda$0` 이라는 이름의 클래스를 **직접 싣고** 그것이 먼저 로드되면,
`instantiate` 의 `has_class` 가 참이 되어 람다 대신 그 클래스를 인스턴스화한다. 누가 일부러 그렇게 짓지 않는 한
생기지 않는다.

## 다시 열 조건

- `rust_java` 또는 다른 임베더가 major ≥ 51 게스트를 돌리는데, 람다 클래스 이름·`forName` 결과에 **의존하는** 프로그램이 실제로 관측될 때.
- 또는 위 「알고 두는 것」의 이름 충돌이 실제로 관측될 때.
「오래됐다」는 사유가 아니다.
