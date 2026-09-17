# 2026-09-17 — kotlinc·scalac 를 «타깃 형상»으로 몰았다 (rustjava-adopt-ldc-tags-real-world-generator-survey-p1)

채택 제안 `2026-09-16-ldc-tags-real-world-generator-survey#p1`.
조사 회차는 **컴파일러의 stdlib**(= 그 컴파일러 자신의 산출물)을 쟀고, ★**「그 컴파일러에게 «문제의 기능»을
직접 컴파일시키지는 않았다」를 자기 표에 「못 쟀다」로 적어 두었다.** 그 칸을 채운다.

## 결과 — ★**0. 그러나 «다른 0» 이다**

| 컴파일러 | 버전 | 클래스 | ldc 자리 | ★**ldc 피연산자 15/16/17** | 풀에 «존재» |
|---|---|---|---|---|---|
| Kotlin | kotlinc-jvm **2.4.20** (JRE 26) | 7 | 23 | ★**0** | MethodHandle **7** · MethodType **6** |
| Scala 3 | scalac **3.9.0** | 7 | 18 | ★**0** | MethodHandle **11** · MethodType **6** |

★★**풀 수치가 이 0 을 읽을 값으로 만든다** — 0 이 아니다. 즉 **indy 경로가 실제로 돌았고 상수도 실제로 만들어졌다.**
다만 그것들이 **`ldc` 피연산자에 닿지 않는다.** ⇒ 「안 썼다」가 아니라 ★**「썼는데도 그 자리에 안 온다」**이다.

★**태그 17(Dynamic)은 풀에도 0** 이다 — ⇒ ★**두 컴파일러 모두 condy 를 «아예» 내지 않는다.** 둘 중 더 센 진술이다.

## 무엇을 «타깃 형상»으로 골랐나

두 파일(`scripts/ldc-tag-survey-targets/Targets.{kt,scala}`) — 태그 15·16 이 컴파일러 산출물에서
**사는 유일한 자리**가 invokedynamic 이므로, **indy 로 가는 기능을 전부** 넣었다:

람다 · **언바운드/바운드 메서드 참조** · SAM 변환(**네이티브 인터페이스 + Java 인터페이스 둘 다**) ·
enum 주어(`when`/`match`) · 문자열 연결 · lazy · (Kotlin) reified 타입 파라미터 · (Scala) **eta 확장 · inline def · 구조적 타입**.

★**플래그도 «더 많이» indy 로 보내는 쪽으로 골랐다**:
```
kotlinc -jvm-target 21 -Xlambdas=indy -Xsam-conversions=indy -Xstring-concat=indy-with-constants -d <out> …/Targets.kt
scalac  -release 21 -d <out> …/Targets.scala
python3 scripts/survey-ldc-constant-tags.py <out>
```
★**이 플래그를 빼고 재면 「질문에서 비껴 설정된 컴파일러」를 재는 것**이고, 그것이 코퍼스 축이 피할 수 없던 약점이다
(stdlib 은 **호환성**을 위해 컴파일되지 백엔드를 훑으려고 컴파일되지 않는다).

## ★양방향 — 이 0 이 「스캐너가 못 본다」가 아님을 먼저 보였다

```
python3 scripts/survey-ldc-constant-tags.py test-data/ldc
  ★ tags 15/16/17 as an ldc operand: {'MethodHandle': 1, 'MethodType': 2, 'Dynamic': 7}
```
같은 스캐너 · 같은 세션에서 **우리 픽스처는 «보인다»** ↔ **타깃 형상은 0**.
스캐너 오차 막대(불가능 피연산자)는 두 타깃 실행 모두 **0 (0.00%)**.

## ★제안의 «값» 전제를 다시 쟀다 — 부분적으로 거짓이었다

제안의 `tradeoff` 는 「**JVM 툴체인이 의도적으로 없는 맥**에 컴파일러 둘을 설치하는 값」이라 했다.
실측: ★**openjdk 26 은 2026-03-11 부터 homebrew 로 설치돼 있었다**(`INSTALL_RECEIPT` epoch `1784711495`) —
조사 회차(**09-16**)보다 **반년 앞선다**. ⇒ 한계 설치 비용은 **formula 하나씩**이었다
(`kotlin → openjdk` · `scala → openjdk, scala-cli`).

## ★대가 — 숨기지 않는다

- ★★**머신 상태가 바뀌었다**: `kotlin 2.4.20` · `scala 3.9.0`(+`scala-cli`)이 **설치됐다**.
  에이전트 레인 약 28개가 공유하는 맥이다. **가산적이고 되돌릴 수 있다** — `brew uninstall kotlin scala`.
  ★**일부러 남겼다**(재측정 가능성). 그러나 **아무도 따로 요청하지 않은 변경**이라 여기 적는다.
- **컴파일러당 프로그램 «하나»** ⇒ 위에 나열한 **기능들**을 한정할 뿐 **언어**를 한정하지 않는다.
- ★**CI 에서 못 돈다** — `setup-java` **0건**인데 이제 컴파일러가 둘 더 필요하다.
  `test-data/src/verify-javac-fixtures.sh` 와 같은 계급: **사람이 돌리는 검사**다.
- ★**답은 제안이 예상한 그 0 이다.** 산 것은 **오차 막대**뿐이고, 그것이 이 회차 값의 정직한 회계다.
