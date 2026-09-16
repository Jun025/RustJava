# 2026-09-16 — `ldc` 태그 15/16/17 을 «실제 바이트코드 생성기»가 내는가

티켓 `rustjava-ldc-tags-15-16-17-real-world-generator-survey` — 채택 제안 `2026-09-16-ldc-tags-15-16-17#p2`.
선행 회차는 **javac 은 내지 않는다**를 증명했고 ★**「ASM·Kotlin·Scala·Lombok 은 못 쟀다」**를 남겼다. 이 회차가 그 칸을 채운다.

## 결론 한 줄

★**ASM 은 «낸다»(실증했다).** Kotlin·Scala·Lombok **산출물 표본 5,479 클래스 · ldc 자리 17,819 곳에서는 0** 이다 —
★그러나 그 0 은 「이 상수를 안 쓴다」가 **아니다**: 같은 산출물의 상수 풀에는 MethodHandle·MethodType 이
**가득하다**(scala-library 하나에 1,604 + 723). ★**그것들은 전부 «부트스트랩 인자»이고 «`ldc` 피연산자»가 아니다** —
javac 에서 관측된 그 형태가 Kotlin·Scala 에서도 그대로다.

## 표 — 도구별 (★「못 쟀다」 칸을 비우지 않았다)

| 도구 | 버전 | 축 | 결과 | 근거(그대로 쳐서 재현) |
|---|---|---|---|---|
| ★**ASM** | 9.7.1 | ★**동적(직접 생성)** | ★**낸다** — 태그 15·16·17 **전부** | `EmitLdc.java`(15줄) → `LdcByAsm.class` → 스캐너: `{MethodHandle 1, MethodType 1, Dynamic 1}` |
| ASM | 9.7.1 | 정적(공개 API) | **받는다** | `javap -cp asm-9.7.1.jar org.objectweb.asm.MethodVisitor` → `public void visitLdcInsn(java.lang.Object)` · 인자 타입 `Handle`·`Type.getMethodType`(오버로드 2)·`ConstantDynamic` **전부 공개 클래스** |
| Kotlin | kotlinc 산출물 `kotlin-stdlib 2.0.21` | 동적(코퍼스) | **0** | 994 클래스 · ldc 11,107 · 풀에는 `MethodHandle 10 · MethodType 5` |
| Kotlin | 〃 `kotlinx-coroutines-core-jvm 1.9.0` | 동적(코퍼스) | **0** | 826 · 1,791 · 풀 `MethodHandle 68 · MethodType 59` |
| Scala 3 | scalac 산출물 `scala3-library_3 3.5.2` | 동적(코퍼스) | **0** | 583 · 651 · 풀 `MethodHandle 310 · MethodType 143` |
| Scala 2.13 | 〃 `scala-library 2.13.15` | 동적(코퍼스) | **0** | 2,889 · 3,755 · 풀 `MethodHandle 1,604 · MethodType 723` |
| Lombok | 1.18.48 × javac 26 | ★**동적(직접 컴파일)** | **0** | `@Data @Builder @Log` 산출물 2클래스 · ldc 2 · 풀 `MethodHandle 2` |
| Lombok | 1.18.48 자기 jar | 동적(코퍼스) | **0** | 183 · 507 |
| (보너스) J2ME | `wie/test_data/draw_j2me.jar` | 동적 | **0**(구조적으로 불가) | 2 · 6 — 태그 15·16·18 은 major ≥ 51, 17 은 ≥ 55 |
| ★**Kotlin·Scala 컴파일러 자체를 «타깃 형상»으로 몰아 본 시험** | — | — | ★**못 쟀다** | kotlinc·scalac 미설치(이 맥에 JVM 툴체인 0) · 설치 비용이 이 회차 예산 밖 — **아래 「좁힘의 근거」** |
| ★**ASM 기반 «실사용» 도구**(ByteBuddy·Mockito·Groovy…) | — | — | ★**못 쟀다** | ASM 이 그 셋을 내는 것은 실증했으나, **그 위에 선 도구들이 실제로 그 API 를 부르는지**는 이 회차에서 재지 않았다 |

**전 corpus 합계(ASM 산출물 제외)**: **5,479 클래스 · ldc 자리 17,819 · 타깃 태그 «0»** ·
★스캐너 오차 막대(불가능 피연산자) **전건 0.00%**.

## ⓐ 조사 비용 — 표본을 좁혔고, 그 근거를 적는다

★**도구를 «설치»하지 않았다.** 대신 ★**「컴파일러의 stdlib 은 그 컴파일러 자신의 산출물이다」**를 썼다 —
`kotlin-stdlib` 은 kotlinc 가, `scala-library` 는 scalac 가 컴파일한 **실물 대량 코퍼스**다(Maven Central 다운로드 6개 · 총 12.6MB).
⇒ 툴체인 설치(수백 MB · 머신 상태 변경) 없이 **수천 클래스**를 쟀다.

★**그 대가는 «좁음»이고 숨기지 않는다**: 각 컴파일러의 **한 프로젝트**를 본 것이라,
그 언어의 **모든 기능**이 그 코퍼스에 나타나지는 않는다. 「Kotlin 은 절대 안 낸다」가 아니라
★**「kotlinc 가 자기 stdlib·coroutines 를 컴파일한 산출물 1,820 클래스에는 0」**이다.

## ⓑ 축을 갈라 적었다 — 동적 ≠ 정적

- **동적**: 산출물 바이트를 직접 스캔(코퍼스) 또는 도구를 실제로 돌려 산출물을 만들어 스캔(ASM·Lombok). **증거력 상**.
- **정적**: 공개 API 표면을 `javap` 로 확인(ASM). ★**「낼 수 있다」이지 「낸다」가 아니다** — 그래서 ASM 은 **동적으로도** 쟀다.

## ⓒ 계측기를 «대조군»으로 먼저 검증했다

`scripts/survey-ldc-constant-tags.py`(신규)를 이 저장소 `test-data/` 에 먼저 돌렸다:
★**심어 둔 양성 8건을 8/8 찾았다**(`LdcMethodHandle`→15 · `LdcMethodType`·`Ldc2WMethodType`→16 · Dynamic 5건→17).
★**「불가능 피연산자」 1건은 오차가 아니라 «심어 둔 음성»** 이다 — `LdcUnknownTag.class`(태그 19 Module 을 가리키는 고의 불량 픽스처)
⇒ ★**실 오차 0**. 선행 회차 계측기의 **0.28%** 와 대비된다(그쪽은 명령어 보행이 간혹 어긋났다 · 이쪽은 opcode 길이표 + `wide`/`tableswitch`/`lookupswitch` 정렬을 정확히 처리한다).
※`test-data/` 클래스 수가 선행 회차 127 → **148** 인 것은 그 사이 픽스처가 늘었기 때문이다(ldc·cp·indy).

## ★풀에 «있다»와 `ldc` 가 «가리킨다»를 갈랐다 — 이 구분이 결론의 절반이다

계측기에 **풀 인구조사**를 넣었다. 그러지 않았으면 「0」이
⑴「이 상수를 아예 안 쓴다」와 ⑵「쓰지만 `ldc` 로는 안 쓴다」를 **구별하지 못한다**.
실측은 **전부 ⑵**다 — Kotlin·Scala 는 람다/SAM 때문에 `invokedynamic` 을 대량으로 쓰고 그 **부트스트랩 인자**로
MethodHandle·MethodType 을 채운다. ★**태그 17(condy)은 풀에도 «0»** 이다(우리 합성 픽스처와 ASM 산출물만 갖는다).

## 남기는 목록 (★이 회차에서 고치지 않는다 — 계약 3)

- ★**ASM 이 낸다 ⇒ 이 형상은 «가설»이 아니다.** 우리 파서가 그 셋을 「미지원」으로 답하는 것(선행 회차 산출물)의
  값이 이 측정으로 **확정**됐다 — 「아무도 안 내는 형상을 위한 코드」가 아니다.
- 형제 티켓 `…-bound-bootstrap-method-attr-index` 는 이미 착지했다(PR #47). 이 결과는 그 긴급도를 **사후 추인**한다.
