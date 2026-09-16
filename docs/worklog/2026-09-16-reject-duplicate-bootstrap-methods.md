# 2026-09-16 — `BootstrapMethods` 를 두 번 선언한 클래스를 거부한다

티켓 `rustjava-adopt-bound-bootstrap-method-attr-index-p0` — 채택 제안
`2026-09-16-bound-bootstrap-method-attr-index#p0`(운영자가 tower 패널에서 채택).

## ⓐ 제안이 «지금도» 참인가 — 재서 확인했다

원문 `why`: 「`bootstrap_method_indices_resolve` uses `find_map`, which takes the first and never
looks for a second」. ★**착수 시 재측: 그 `find_map` 은 `classfile/src/validation.rs:117` 에 그대로 있다.**
⇒ 제안은 **유효**하다(그 worklog 를 낸 회차 이후 이 자리는 움직이지 않았다).

JVMS 4.7.23 은 ClassFile 속성표에 `BootstrapMethods` 를 ★**최대 한 개**만 허용한다.

## ⓑ 이미 같은 것을 하는 축이 있는가 — 없다. 단 «모양»은 이미 있다

`validate_class` 는 이미 **부재/중복 개수**를 세는 규칙을 둘 갖고 있다:
필드의 `ConstantValue` (`constant_values.len() > 1` → 거부) · 메서드의 `Code` (`code_attributes != 1` → 거부).
★**클래스 «자신»의 속성표에는 그런 규칙이 하나도 없었다.** ⇒ 새 관용을 발명하지 않고 **그 모양을 그대로** 썼다.

## ⓒ 제안이 틀린 부분 — ★**`tradeoff` 의 한 문장은 «과했다»**

원문: 「It also needs a fixture **the generator cannot currently build**, since the attribute list is
assembled per fixture.」 ★**그것은 사실이 아니다.** 속성 목록은 빌더에게 `attributes` 리스트로 **그대로 전달**되므로,
기존 빌더를 감싸 **그 빌더가 쓴 속성을 한 번 더 append** 하는 래퍼 **8줄**이면 된다
(`duplicate_bootstrap_methods(inner)`). 생성기 구조를 바꿀 필요가 없었다.
⇒ ★**제안의 관측은 맞았고, 비용 추정이 틀렸다.** 그 차이를 적어 둔다 — 다음 사람이 같은 이유로 미루지 않게.

## 고친 자리 — 한 곳

`classfile/src/validation.rs` 에 술어 하나를 더하고 `validate_class` 의 거부 조건에 이었다:

```rust
fn at_most_one_bootstrap_methods_attribute(class: &ClassInfo) -> bool {
    class.attributes.iter().filter(|a| matches!(a, AttributeInfo::BootstrapMethods(_))).count() <= 1
}
```

★**`bootstrap_method_indices_resolve` 를 «고치지» 않고 «옆에» 뒀다** — 그 함수의 doc 이 스스로
「인덱스가 실재 항목을 가리키는가」라는 **한 문장**임을 선언하고 있고, 「표가 몇 개인가」는 **다른 문장**이다.
안에 접어 넣으면 그 함수를 **이름까지 바꿔야** 하는데 이 회차의 경계가 그것을 금한다.

## 픽스처 — ★**결함이 «하나»가 되도록 지었다**

`test-data/ldc/LdcDynamicDuplicateBSM.class`(생성기 `make_ldc_fixtures.py` · 신규 래퍼 `duplicate_bootstrap_methods`).
★**같은 유효한 표를 «바이트 동일»하게 두 번** 쓴다 — 어느 한 표만 있어도 **정상 파일**이므로
★**거부의 원인이 «둘이라는 사실» 하나로 고정**된다(둘째 표를 다르게 만들면 다른 규칙이 먼저 물어서
테스트가 «이름과 다른 이유»로 통과한다).
구조 확인(측정): 클래스 속성 = `['BootstrapMethods', 'BootstrapMethods']` · **두 본문 바이트 동일 = True**.
재생성 **멱등**: 기존 11개 **전건 바이트 동일** · 신규 1개.

## 전/후 — 실제로 무엇이 바뀌나 (end-to-end)

| 파일 | 고치기 «전» | 고친 «후» | ★참조 JVM(OpenJDK 26.0.1) |
|---|---|---|---|
| `LdcDynamicDuplicateBSM`(표 2개) | `UnsupportedOperationException: ldc of a dynamically-computed constant` | ★**`ClassFormatError: Invalid class file`** | ★**`ClassFormatError: Multiple BootstrapMethods attributes in class file`** |
| `LdcDynamic`(표 1개 · 대조군) | `UnsupportedOperationException` | ★**불변** | 로드 성공(rc=0 · 무출력) |

⇒ ★**이 리니지의 문장 그대로다**: 「이 런타임이 아직 못 한다」 → **「이 파일이 깨졌다」**, 그리고 그 판정이
참조 JVM 과 **일치**한다. ★대조군이 불변이라는 것이 **변경의 좁음**을 보인다.
※`AGENTS.md` 의 허용 축(observable behavior)만 썼다 — OpenJDK **소스 미참조**.

## 개악 대조 (양방향 · 제품 «호출부»)

| 개악 | 결과 |
|---|---|
| **M1** — 호출부에서 `|| !at_most_one_bootstrap_methods_attribute(class)` **제거**(= 제안 이전 상태) | ★**red** — `expected ClassFormatError, got: … UnsupportedOperationException` |
| **M2** — 술어 본문을 **`true`(상수 통과)** 로 | ★**red** — 같은 자리에서 |
| 정상 | **green**(`test_class_format` 12 passed) |

★**M2 를 따로 돌린 이유**: M1 만으로는 「호출을 지웠다」만 잡고 ★**「검사가 상수로 뭉개졌다」는 안 잡힌다**
(이 저장소가 반복해 지적한 「상수 대 상수」 형태). 두 축이 다 red 여야 그 단언이 **실제로 무는 것**이다.

## 잃는 것 (계약 2⒜) — 숨기지 않는다

- ★**지금까지 로드되던 파일 하나가 거부된다.** 다만 그 형상은 ★**어떤 컴파일러도 내지 않는다** —
  이 저장소가 바로 어제 잰 바로는 javac·kotlinc·scalac·Lombok 산출물 **5,479 클래스에 0**이고,
  ASM 으로도 «일부러» 만들어야 나온다. ⇒ 실사용자를 놀라게 할 확률이 낮다.
- ★**파스 시점 비용**: 클래스 속성표 1회 선형 순회(속성 수는 보통 한 자릿수). 상수 풀은 건드리지 않는다.
- ★**오탐 여지**: 규칙이 «개수»뿐이라 해석 여지가 없다 — JVMS 4.7.23 문면이 그대로 술어다.

## 안 하면 무엇이 나쁜가 (계약 2⒝)

★**두 표 중 «어느 것이 진짜인가»를 파서가 임의로 정한다.** `find_map` 이 첫 번째를 쓰므로
`bootstrap_method_attr_index` 는 **첫 표**에 대해서만 경계 검사되고 둘째 표는 **조용히 무시**된다.
⇒ 같은 파일이 「첫 표 기준으로는 유효」할 수 있고, 그 사실이 **아무 데도 드러나지 않는다**.
그리고 우리는 그 파일을 **「미지원」**이라 불렀다 — ★**참조 JVM 이 «깨졌다»고 답하는 파일에 대해서.**
