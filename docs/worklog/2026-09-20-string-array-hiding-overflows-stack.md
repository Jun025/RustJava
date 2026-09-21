# 2026-09-20 — `[Ljava/lang/String;` 를 숨기면 왜 스택이 넘치나 (조사)

채택 제안 `2026-09-20-error-path-class-closure#p1` (티켓 `rustjava-error-path-string-array-hiding-overflows-stack-p1`).

## 재현 — 했다
`origin/main 97707f26` · cap **20** · 기본 스택(2 MiB) ⇒
`thread 'probe' has overflowed its stack` / `fatal runtime error: stack overflow, aborting` · **rc 134**.

## cap 20 은 어디서 오는 수인가
`jvm/tests/test_error_path_class_sweep.rs` 의 **`const GIVE_UP_AFTER: u32 = 20`** →
`test_utils::test_jvm_hiding(name, give_up_after)` → `HidesOneClass.give_up_after`.
★**테스트 전용 상수**다 — 제품에는 이런 상한이 **없다**.

## ★★결론 — 지난 회차가 적은 원인은 «틀렸다»
> 지난 회차 문안: 「its recursion does not come back through the loader, so the cap cannot end it」

★**반증됐다. 매 턴 로더로 돌아오고, cap 이 끝낸다** — 살아남은 모든 실행에서 `asked = cap + 1`
(= `[C` 가 보이는 바로 그 모양). 넘치는 이유는 **모양이 아니라 턴당 비용**이다.

**두 손잡이가 «따로» 임계를 움직인다** — 이것이 「무한」과 「유한하지만 깊다」를 가른다:

| 손잡이 | 고정 | 결과 |
|---|---|---|
| cap | 스택 2 MiB | **18 생존**(asked=19) ↔ ★**19 오버플로** |
| 스택 | cap 20 | 2 MiB **오버플로** ↔ ★**4 MiB 생존**(asked=**21**) · 8/32/128 MiB 도 생존 |

무한 재귀였다면 **둘 중 어느 것도** 결과를 바꾸지 못한다.

## 순환 — 추측이 아니라 백트레이스
`Throwable::fill_in_stack_trace` 3회 진입 시점에 스크래치 패닉 + `RUST_BACKTRACE=1`(계측은 되돌렸다):

```
fill_in_stack_trace
 └ instantiate_array("Ljava/lang/String;")
    └ resolve_class → resolve_class_internal → load_class      ← 로더가 None(숨김)
       └ exception("java/lang/NoClassDefFoundError", …)
          └ new_class → invoke_special ×4
             (NoClassDefFoundError → LinkageError → Error → Throwable.<init>)
             └ init_with_message → invoke_virtual "fillInStackTrace"
                └ fill_in_stack_trace                           ← 같은 자리, 다음 턴
```
한 턴 ≈ **57 프레임**. `[C` 는 `JavaLangString::from_rust_string` **안에서** 도는 훨씬 짧은 경로라
같은 2 MiB 에 20턴이 들어간다. ⇒ ★**`[Ljava/lang/String;` 는 「배열이라서」가 아니라
「오류 경로의 «더 깊은 곳»에서 필요해서」 비싸다.**

## 이 타입이 특별한가 — 대조로 답한다 (cap 20)
| 숨긴 이름 | 결과 |
|---|---|
| `[C` | 재귀 · asked=**21** · 생존 |
| `[Ljava/lang/String;` | 재귀 · 2 MiB **오버플로** / 4 MiB asked=**21** |
| `[B` | `Option::unwrap()` 패닉 — ★`bootstrap_classes` 소속이라 **형제 `#p0` 의 결함**이다(이 회차 아님) |
| `[I` · `[Ljava/lang/Object;` · `[Ljava/lang/StackTraceElement;` · `java/lang/Integer` · `java/lang/StringBuilder` | asked=**0** · 구성 성공 — 구성이 아예 묻지 않는다 |

⇒ ★**「배열이면 난다」가 아니다.** 오류 경로가 실제로 필요로 하는 **두 배열**만 돈다.

## 고쳤나
★**제품 코드 0줄.** 고친 것은 **스윕 헤더의 거짓 문장 하나**다 — 위 측정이 그것을 참으로 만든다.
배열 제외는 **그대로 둔다**: 이유는 원래부터 「로더가 **합성**하므로 어떤 클래스 집합도 배열을
빠뜨릴 수 없다」였고, 그 이유는 이번 측정과 **무관하게** 유효하다.

## 왜 지금인가 · 대가
- **왜 지금**: 그 거짓 문장이 **제품 트리에 커밋돼** 있었고, 「이 축은 잴 수 없다」고 선언해
  ★**다음 회차가 재보지 않게 만드는** 형태였다.
- **대가**: 조사 회차라 새 잠금은 없다. 순환 자체는 **제품에 바닥이 없는 채로 남는다** —
  오늘 닿을 수 없을 뿐이다(제안 카드).
