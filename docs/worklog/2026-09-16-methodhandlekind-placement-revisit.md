# 2026-09-16 — `MethodHandleKind` 위치 재결정 (PR #44 착지 후)

티켓 `rustjava-methodhandlekind-placement-revisit-after-pr44` — 채택 제안
`2026-09-16-bootstrap-methods-and-method-handle#p2`.

## 결정: **옮기지 않는다.** `MethodHandleKind`·`MethodHandleRef` 는 `classfile/src/attribute.rs` 에 남는다

제안 낱말이 `Re-decide` 였고, ★**재결정의 답이 「그대로」다.** 근거는 취향이 아니라 아래 실측이다.

## ⓐ 「이유가 만료됐다」 — 반은 맞고 반은 아니다

제안이 적은 배치 사유는 **둘**이다(원문 `why`):
1. 「the **only consumer today** is the BootstrapMethods attribute」
2. 「the sibling round had `constant_pool.rs` **open with edits**」 ← 제안 자신이 「PR 이 머지되는 순간 사라진다」고 적은 그것

**⑵는 만료됐다**(착수 시 재측): PR **#44** `state=MERGED` · `mergedAt=2026-09-16T02:13:39Z` ·
머지커밋 `dc03593699cd3ca8d4ef41109ca8371d01b4c4c8` · ★**그 PR 의 파일 목록에 `classfile/src/constant_pool.rs` 가 실제로 있다**
(사유가 가리킨 그 파일이 맞다 — 이름만 같은 다른 파일이 아니다).

**⑴은 살아 있다.** 이 회차의 전수 실측:

| 참조 | 자리 | 경로 |
|---|---|---|
| `MethodHandleRef::resolve` **호출** | `attribute.rs:116` **단 1곳** | `BootstrapMethod` 파서 안(`resolve` 는 `pub` 도 아니다) |
| `MethodHandleKind` 읽기 | `classfile/tests/test.rs:223,270` | `bootstrap_methods(&class)[0].method.kind` |
| 〃 | `jvm-bytecode/src/string_concat.rs:81` | `bootstrap.method.kind != MethodHandleKind::InvokeStatic` |

★**제안이 조건으로 건 「⒝(콜사이트 링크)가 착지한 뒤에 정하라」는 이미 충족됐다** — 그 ⒝ 가 **PR #48**
(`rustjava-link-stringconcatfactory-…`)로 오늘 착지했고, ★**그 소비자조차 `BootstrapMethod` 를 통해 읽는다.**
⇒ 제안 자신의 `tradeoff` 가 적은 처분이 그대로 발동한다:
「if ⒝ lands and **still only reads them through BootstrapMethod**, leaving them alone is correct and this
proposal should be **declined** rather than done.」

## ⓑ 「옮기지 않는다」의 근거는 그 escape 절 «하나»가 아니다 — 코드에서 더 나은 이유를 찾았다

`constant_pool.rs` 는 **이미** CONSTANT_MethodHandle 을 어디까지 해독할지 정해 두었다:
`ConstantPoolReference::MethodHandle` — ★**피연산자 0**. 그 자리의 주석이 이유를 적는다(요지):
「method handle 을 resolve 하려면 없는 기구가 필요하고, 유일한 소비자인 verifier 는 그것을 **"not implemented"** 로 바꾼다」.

⇒ ★**거기로 옮기면 «같은 상수의 서로 다른 두 해독»이 한 파일에 나란히 선다** —
「아무도 resolve 하지 않는다」는 무-페이로드 해독과 「이 멤버를 가리킨다」는 완전 해독이 이웃한다.
★**그것은 읽는 사람에게 모순으로 보이고, 이 이동이 없애려던 바로 그 오해를 «더 크게» 만든다.**

## 잃는 것 — 숨기지 않는다

티켓 계약 2⒝ 가 적은 「그대로 두면 **「속성 파일이 상수 풀 타입을 갖는다」는 오해가 남는다**」는 **참이고, 남는다.**
⇒ 그 비용을 «이동»이 아니라 ★**그 자리의 주석**으로 갚았다 — `MethodHandleKind` doc 에 ⑴왜 여기인지
⑵`constant_pool.rs` 가 이미 내린 결정 ⑶★**언제 이 판단을 다시 열어야 하는지**(= 이 속성 «밖»의 무언가가
method handle 을 해독할 때 · 유력 후보는 `ldc` 경로이고 지금은 `UnsupportedFeature` 로 답한다)를 적었다.
★**다음 사람이 같은 질문을 «처음부터» 다시 하지 않게 하는 것이 이 회차의 실제 산출물**이다.

★**후속 제안 카드를 새로 만들지 않았다** — 「X 가 생기면 다시 열어라」는 조건부 항목이고, 그것을 cockpit
열린 추천으로 띄우면 **아무도 집행할 수 없는 카드가 영구히 남는다**. 재개 조건은 **코드 주석**에 뒀다(고칠 사람이 보는 자리).

## 불변 증명 (Acceptance ⒝ — 이동이 없으므로 «불변»이 그 자리를 진다)

- `cargo test --all` **570 passed / 0 failed / 1 ignored** — ★직전 회차(`#50` 착지 형상 `b3a20ae`)와 **동일**.
- `git show --numstat` = **10 insertions / 0 deletions**(한 파일) ⇒ ★**doc 주석 «삽입»이고 «치환»이 아니다** —
  계약의 「치환 의도인데 삭제행 0이면 의심하라」는 **의도가 삽입이므로 해당하지 않는다**(그 사실을 여기 적어 둔다).
- ★**「옮기기 전 위치를 참조하는 코드 0」 축은 «해당 없음»이다**(이동 0) — 조용히 빼지 않고 선언한다.
- 타입 정의·`impl`·re-export(`classfile/src/lib.rs:15`) **무접촉** ⇒ 공개 API 변화 **0**.
