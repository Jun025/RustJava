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

★**태그 17(Dynamic)은 풀에도 0** 이다 — ⇒ ★**이 형상들에서는 두 컴파일러 모두 condy 를 내지 않는다.** 둘 중 더 센 진술이고,
★**한정은 이 문서의 나머지 수와 «같다»** — 컴파일러당 프로그램 **1개**라 위에 나열한 **기능**을 한정할 뿐 **언어**를 한정하지 않는다.

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
실측: ★**openjdk 26 은 2026-07-22 부터 homebrew 로 설치돼 있었다**(`INSTALL_RECEIPT` 의 **`time`** 필드 =
epoch `1784711495` = 2026-07-22T09:11:35Z = KST 18:11:35) — 조사 회차(**09-16**)보다 **56일**(약 2개월) 앞선다.
★**초판의 날짜는 같은 receipt 의 «다른 필드»에서 왔다** — `source_modified_time`(= `1773157346` = formula 가
마지막으로 수정된 시각 · UTC 2026-03-10T15:42:26Z)을 **로컬(KST)로 환산한 값**이다. ★**날짜는 그 필드에서,
epoch 는 `time` 에서** 가져와 한 문장에 붙였기 때문에 **인용한 수가 그 문장을 스스로 반증**했다.
⇒ 그래서 위 문장은 **어느 필드인지**를 함께 적는다(같은 혼동의 재발 방지가 이 한정의 목적이다). ⇒ 한계 설치 비용은 **formula 하나씩**이었다
(`kotlin → openjdk` · `scala → openjdk, scala-cli`).

## ★대가 — 숨기지 않는다

- ★★**머신 상태가 바뀌었다**: `kotlin 2.4.20` · `scala 3.9.0`(+`scala-cli`)이 **설치됐다**.
  에이전트 레인 약 28개가 공유하는 맥이다. ★**일부러 남겼다**(재측정 가능성). **아무도 따로 요청하지 않은 변경**이라 여기 적는다.
- ★★**그리고 초판이 빠뜨린 것 하나 — `openjdk` 기본 링크가 승격됐다**: `26.0.1 → 26.0.2.1`
  (`/opt/homebrew/opt/openjdk` · `INSTALL_RECEIPT.time 2026-09-17T12:48:50Z` · `installed_on_request false` = **의존성으로** 들어왔다).
  ★**초판의 되돌리기 레시피(`brew uninstall kotlin scala`)는 이 링크를 되돌리지 «않는다».**
  ★**그리고 Homebrew 7.0.3 에는 판본을 되돌리는 명령이 없다** — `brew switch` 는 제거됐다(실측: 미지원 명령).
  ⇒ ★**실효 처방은 «전역 되돌리기»가 아니라 «소비자 핀»이다**: 26.0.1 keg 가 **그대로 있으므로**
  `JAVA_HOME=/opt/homebrew/Cellar/openjdk/26.0.1` 로 고정하면 된다(★**실행 확인** — 그 경로의 `java -version` = `26.0.1`).
  ★**전역 `opt` 심링크를 손으로 되돌리는 것(`ln -sfn`)도 기전상 가능하지만 이 회차는 «치지 않았다»** —
  28레인 공유 머신의 전역 상태이고, brew 가 다음 `link`/`upgrade` 때 다시 옮긴다(기전만 스크래치 심링크로 확인).
  ★**범위를 정확히**: openjdk 는 **keg-only** 라 `/opt/homebrew/bin` 에 링크되지 않는다 ⇒ `PATH` 의 `java` 는
  여전히 `/usr/bin/java` 이고 **바뀌지 않았다**. 영향은 `/opt/homebrew/opt/openjdk` 를 **명시적으로** 쓰는 소비자
  (`JAVA_HOME` 지정 등 — **이 회차가 바로 그랬다**)뿐이다.
- **컴파일러당 프로그램 «하나»** ⇒ 위에 나열한 **기능들**을 한정할 뿐 **언어**를 한정하지 않는다.
- ★**CI 에서 못 돈다** — `setup-java` **0건**인데 이제 컴파일러가 둘 더 필요하다.
  `test-data/src/verify-javac-fixtures.sh` 와 같은 계급: **사람이 돌리는 검사**다.
- ★**답은 제안이 예상한 그 0 이다.** 산 것은 **오차 막대**뿐이고, 그것이 이 회차 값의 정직한 회계다.
