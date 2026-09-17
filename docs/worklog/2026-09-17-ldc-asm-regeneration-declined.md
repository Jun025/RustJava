# 2026-09-17 — ldc 픽스처를 ASM 으로 재생성하자는 제안: ★**기각** (rustjava-adopt-ldc-tags-real-world-generator-survey-p0)

채택된 제안 `2026-09-16-ldc-tags-real-world-generator-survey#p0` —
**「태그 15/16/17 ldc 픽스처를 손조립 바이트 대신 ASM 으로 재생성하라」**.

★**기각한다.** 취향이 아니라 **실측**으로 적는다. 제품 코드 **0줄**.

---

## 1. 제안이 사려는 것 — 그리고 그것이 «이미» 있다

제안의 `userBenefit`: 「테스트가 **바이트코드 생성기가 실제로 내는 것**에 고정되어, 픽스처가
어떤 실제 도구도 내지 않는 모양으로 표류하면서 통과하는 일이 없어진다」.

★**그 질문은 이미 답이 나와 있다 — 그것도 «더 센 오라클»로.**

| 오라클 | 무엇을 말하나 | 실측 |
|---|---|---|
| ★**OpenJDK 26.0.1 이 실제로 «실행»한다** | 진짜 JVM 의 클래스 파일 **검증기**가 우리 손조립 바이트를 받아들인다 | `LdcMethodHandle`·`LdcMethodType`·`LdcDynamic`·`Ldc2WDynamic` ★**4건 전건 rc=0 · 출력 없음** |
| 대조군(위법 픽스처) | 같은 JVM 이 거부한다 | `Ldc2WMethodType`·`LdcTag13`·`LdcUnknownTag`·`LdcDynamicOldMajor`·`LdcDynamicNoBSM` **전건 rc=1** |
| 선행 조사 회차 | 「ASM 이 이 태그들을 내는가」 | ★**이미 동적으로 실증**(ASM 9.7.1 · `EmitLdc.java` 15줄 → `{MethodHandle 1, MethodType 1, Dynamic 1}`) |

⇒ ★**「이 모양이 진짜인가」는 이미 참으로 측정됐고, 「우리 인코딩이 적법한가」는 JVM 검증기가 통과시켰다.**
ASM 재생성이 더할 것은 **인코딩의 출처**뿐이다.

## 2. ★손의 흔적은 «사라지지 않고 옮겨간다» — 제안이 적지 않은 축

ASM 경로는 `visitLdcInsn(new Handle(...))` 을 **명시적으로 부르는 드라이버**(선행 회차의 `EmitLdc.java`, 15줄)가 있어야 성립한다.
⇒ ★**픽스처의 «모양»은 여전히 우리가 고른다.** 바뀌는 것은 「그 모양을 누가 «인코딩»하느냐」 하나다.
「손으로 만든 모조품」이 「손으로 쓴 Java 를 ASM 에 먹인 것」이 될 뿐, `synthetic` 이라는 성질은 남는다.

## 3. 값 — 제안 자신이 셋을 적었고, 그 셋이 «지금도» 참이다

| 제안의 비용 | 실측 |
|---|---|
| JVM 툴체인이 PATH 에 없다 | ★참 — `java -version` → `Unable to locate a Java Runtime` · `/usr/libexec/java_home` **0건**. 단 ★**정정**: homebrew 로 **설치는 돼 있다**(`/opt/homebrew/opt/openjdk` = 26.0.1) ⇒ 「부재」가 아니라 **「기본 경로로는 닿지 않는다」** |
| asm.jar 를 핀+체크섬으로 들여와야 한다 | ★참 — repo 에 jar **0건**(`test-data/test.jar` 은 무관) · 받으려면 **네트워크**가 필요하다 |
| 음성 픽스처는 ASM 으로 만들 수 없다 | ★참 — 고의로 망가진 파일이라 어떤 빌더 API 도 못 낸다 ⇒ ★**생성기가 «두 기구»가 된다** |

★**CI 축 실측**: `.github/workflows/` 6개에 `setup-java|java-version|temurin|zulu` **0건**.
★**단 「CI 가 깨진다」로 과장하지 않는다** — 오늘 CI 는 픽스처를 **재생성하지 않는다**
(`make_*_fixtures|verify-javac` 참조 **0건**). ⇒ 진짜 비용은 이것이다:
★**「어디서나 한 줄로 되는 재생성」이 「네트워크 + JDK + 핀된 jar」 의식이 된다.**

## 4. ★그리고 «지금» 새로 생긴 값 — 상시 검사와 정면 충돌한다

`test-data/src/audit-fixture-single-defect.py` 의 **첫 등식**은
`generator(**given) == committed` 이고, 이것은 `no defect` 로 건너뛰는 행 **«전에»** 돈다.
⇒ ASM 이 바이트를 내면 그 세 픽스처는 ★**`GENERATOR DRIFT`(rc=2)** 가 된다.

★**정직하게 단서를 단다**: 그 감사기는 ★**아직 착지하지 않았다**
(`git ls-tree -r origin/main` 일치 **0** · `bin/landed` → **UNLANDED**). **PR #63 에서 진행 중**이다.
⇒ 이 축은 「오늘의 사실」이 아니라 **「착지하면 즉시 충돌하는 축」**이다. 그래도 기각 근거로 쓰는 이유는,
그 감사기가 바로 **이 픽스처들의 자기기술성을 지키는 유일한 상시 검사**이기 때문이다.

## 5. 버전 핀도 손으로 고를 수밖에 없다

`test-data/class-file-versions.txt`(#57 착지):
```
55.0 ldc/Ldc2WDynamic.class      55.0 ldc/LdcDynamic.class
52.0 ldc/LdcMethodHandle.class   52.0 ldc/LdcMethodType.class
```
★**55 는 우연이 아니다** — JVMS 4.4 가 태그 17 을 major ≥ 55 로 묶는다. ASM 은 **드라이버가 정한 값**을 쓰므로
그 majors 는 **여전히 사람이 고른다**. 게다가 `tests/test_class_format.rs:309-330` 이 이 픽스처들의 major 를
50/54 로 낮춰 버전 규칙을 증명하므로, ★**major 는 두 번째 테스트의 하중도 받는다**.

## 6. ★잃는 것 — 「기각은 공짜」가 아니다

- 생성기 머리의 **`These are **synthetic**`** 단서가 양성 픽스처에도 그대로 남는다 ⇒ 읽는 사람은
  「모양이 진짜다」를 **조사 worklog 의 말**로 믿어야 한다(명령으로 재현하는 것이 아니라).
- ★**잔여 위험**: 우리 파이썬 인코더에 **체계적 편향**이 있는데 우리 파서«와» OpenJDK 검증기가 **둘 다 마침 관대**하다면,
  ASM 산출물은 그것을 드러내고 우리는 못 드러낸다. ★JVM 검증기는 **센 필터이지 증명이 아니다.**
- 어느 쪽이든 **상시 보장은 없다** — 누가 생성기를 「적법하지만 비현실적인」 모양으로 고쳐도 아무것도 울지 않는다.
  버전 핀도 단일결함 감사도 **자기정합성**을 볼 뿐 **현실성**을 보지 않는다.

## 7. 더 싼 대안은 제안 카드로 남겼다

`#p0` — **「재생성하지 말고, 양성 픽스처를 진짜 JVM 에 «올려서» 현실성 검사로 삼아라」**.
이 회차가 손으로 잰 그 rc=0 을 **명령으로** 굳히는 것이고, 이미 같은 형태의 스크립트
(`test-data/src/verify-javac-fixtures.sh` — 「JDK 가 필요하고 CI 엔 없으니 수동 검사다」)가 **선례로 있다**.
★그 카드도 자기 약점을 적었다: **수동이라 같은 방식으로 썩고**, 「적법하다」를 증명할 뿐 「이 인코딩을 누가 낸다」는 증명하지 못한다.
