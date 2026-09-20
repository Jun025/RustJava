# 2026-09-20 — 오류 경로의 클래스 집합을 «한 번에» 쟀다 · 답은 «둘이 아니었다»

채택 제안 `2026-09-19-string-on-the-error-path#p0`.
티켓 `rustjava-string-on-the-error-path-p0` · 인용 트리 `origin/main 5d732b13`.

## 제안이 물은 것

> 「This round and the one before it each closed exactly one class, and each found the next by
> **reading the code and guessing** … A sweep that **hides each bootstrap-reachable name in turn**
> and records which ones recurse would answer the whole question once.」
> tradeoff — 「If it turns out the answer is **still just these two**, the result is a **recorded
> negative** rather than a new check.」

## 답 — ★**「still just these two」가 아니다**

기록 로더로 정상 구성을 한 번 돌려 후보를 «유도»했다(손으로 적지 않았다): 요청 **51건** · 서로 다른 이름 **42** ·
배열 이름 **5** 제외 ⇒ 후보 **37**. 그 37개를 하나씩 숨겨 재구성했다.

| 분류 | 고침 전 | 고침 후 |
|---|---|---|
| ★**재귀(바닥 없음)** | **5** | ★**0** |
| 이름을 들어 거절 | 7 | **12** |
| 한 번 묻고 깨끗이 실패 | 25 | 25 |
| 없어도 구성됨 | 0 | 0 |

★**재귀한 5개** = `java/lang/Throwable` · `java/lang/Error` · `java/lang/LinkageError` ·
`java/lang/CharSequence` · `java/lang/Comparable`.
★**그 5개는 우연이 아니다** — 기존에 막아 둔 두 이름의 **상위형·인터페이스 폐포(closure)** 다.
클래스를 resolve 하면 상위형도 resolve 되고, ★**거기서 난 결손은 «아직 resolve 중인 바로 그 두 클래스»로 보고된다.**

## 처방 — 목록이 아니라 «폐포»를 본다

`Jvm::new` 의 손으로 적은 assert 두 개를 **작업목록 루프 하나**로 바꿨다. 씨앗은 그 두 이름이고,
얻은 정의의 `interface_names()`·`super_class_name()` 을 계속 밀어 넣는다. 로더에 **직접** 묻는다 —
`resolve_class` 로 물으면 그 질문이 `Jvm::exception` 으로 가고, 그것이 곧 순환이다.

★**폐포는 9개이고 계산이 «닫힌다»**: `NoClassDefFoundError · LinkageError · Error · Throwable ·
Object · Serializable · String · CharSequence · Comparable`
= **이미 막혀 있던 2** + **재귀한 5** + **`bootstrap_classes` 가 먼저 잡는 2**(`Object`·`Serializable`).
⇒ 설명되지 않는 이름이 **0** 이다.

## 양방향 — 제품 호출부에서(사본 아님)

| | 개악 | 결과 |
|---|---|---|
| **정상** | 없음 | `37 candidate(s): 0 recursed · 12 refused by name · 25 failed cleanly · 0 not needed` · **ok** |
| **개악** | `jvm/src/jvm.rs` 의 `pending.extend(...)` 두 줄 제거(= 두 이름만 보던 고침 전 동작) | ★**`5 recursed` · FAILED** |
| **복원** | 되돌림 | **0 recursed · ok** |

★기존 잠금 `jvm/tests/test_exception_fallback_recursion.rs` **2건은 손대지 않고 그대로 통과**한다 —
새 패닉 문안이 `has no java/lang/String` · `has no java/lang/NoClassDefFoundError` 를 그대로 담기 때문이다.

## 대가 — 숨기지 않는다

- ★**시작 비용**: 로더 질문 **44 → 51**(폐포 9회 ↔ 종전 2회). 벽시계는 재지 «않았다» —
  전 회차가 같은 자리에서 재고 「이 호스트의 노이즈 아래」로 버렸다. **결정론적 수로만 말한다.**
- ★**`cargo test --all` 에 스윕 ~15초**가 붙는다. 일반화의 값이고, 깎지 않았다.
- ★**배열 이름 5개는 스윕에서 제외**했고 그 이유는 «의견이 아니라 측정»이다: 부트스트랩 로더가
  배열을 **합성**한다(`define_array_class`) ⇒ 어떤 클래스 집합도 배열을 «빠뜨릴» 수 없다.
  그래도 재 봤다 — `[C` 는 **재귀**(상한 20에 21질문) · ★**`[Ljava/lang/String;` 는 «그 상한에서도»
  스택 오버플로**다(= 그 순환은 로더로 돌아오지 않아 상한이 끝내지 못한다). ⇒ ★**이 스윕이 in-process 로
  돌릴 수 없는 유일한 후보**이고, 제안 `#p1` 로 남겼다.
- ★**부트스트랩 6개를 숨기면 `called Option::unwrap() on a None value`** 로 죽는다 — 구성 시점에
  멈추니 옳지만 **이름을 말하지 않는다**. 12건 중 **5건**이 그것이다. 이 회차의 결함이 아니라 별 계급이라
  제안 `#p0` 으로 남겼다.

## 왜 지금인가

전 두 회차가 **각각 한 클래스씩** 닫았고 다음 이름을 **읽고 추측해** 찾았다. 그 속도로는 폐포 9개를 닫는 데
회차 7번이 더 든다 — 그리고 ★**그 방식은 「이게 마지막인가」에 영원히 답하지 못한다.** 이번에 기계가 답했다.
