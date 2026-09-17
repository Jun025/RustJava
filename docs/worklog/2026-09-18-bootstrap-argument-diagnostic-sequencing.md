# 2026-09-18 — 「어느 bootstrap argument 가 왜 나빴나」 — ★**이 회차는 대전제 ⓒ 에서 끝난다** (rustjava-adopt-loadable-bootstrap-arguments-diagnostic)

채택 제안 `2026-09-17-loadable-bootstrap-arguments#p0` 의 처분. ★**코드 0행**(`classfile/src/error.rs`·`validation.rs` **무접촉**).

## 제안의 전제는 «참»이다 — 내가 실행으로 확인했다

읽어서가 아니라 **CLI 로 돌려서** 봤다(`main` @ `8c7b473f`):

```
rust_java -cp ./test-data/ldc LdcDynamicBSMArgPastEnd   → java.lang.ClassFormatError: Invalid class file
rust_java -cp ./test-data/ldc LdcDynamicDuplicateBSM    → java.lang.ClassFormatError: Invalid class file
rust_java -cp ./test-data/ldc LdcDynamicOldMajor        → java.lang.ClassFormatError: Invalid class file
```

★**서로 다른 세 규칙**(나쁜 argument · 중복 속성 · 버전 게이트)이 **글자 하나 다르지 않은 같은 문장**을 낸다.
제안이 말한 「밋밋한 `Invalid class file`」은 정확한 서술이다.

## ★그런데 대전제 ⓒ 가 «발화한다» — 이미 그 일을 하는 축이 «떠 있다»

**PR #67**(`rustjava-adopt-classfile-error-cause-decision-p0` 리니지)이 **바로 그것**을 한다.
겹침이 «부분»이 아니라 «전부»다:

| 축 | 제안 | PR #67 |
|---|---|---|
| `target` | `classfile/src/error.rs` · `classfile/src/validation.rs` | ★**정확히 그 둘**(merge-base 대비 `error.rs` **10/1** · `validation.rs` **61/27**) |
| 이 술어 | 「어느 argument 가 왜」를 말하게 하라 | ★**이미 사유를 준다** — `"a bootstrap method argument names nothing or is not a loadable constant"` |
| 밋밋한 문면 | 없애라 | ★**두 경계 자리**(`src/runtime.rs:189` · `test-utils/src/lib.rs:334`)를 **둘 다** 고쳤다 |

★**제안 자신이 이것을 적어 두었다**: `tradeoff` 에 「The same follow-up is already proposed from the p2-fix round,
so the two should be done together rather than twice.」 ⇒ ★**제안이 «따로 하지 마라»고 말한 그 상황이 지금이다.**

★**계약 2(문면을 match 하는 곳 전수 조회)도 같은 결론을 가리킨다**: `"Invalid class file"` 리터럴은 **두 곳**에 박혀 있고,
#67 이 그 둘을 이미 고쳤다. main 에서 시작하면 **같은 두 곳을 다시 고치고**, #67 회차가 이미 보고한
「마지막 홉이 두 번 쓰여 있고 한 쪽만 테스트가 본다」는 **커버리지 구멍을 다시 발견**하게 된다.

## ★남는 것은 «있다» — 다만 제안이 말한 것보다 좁다

★**`#67` 의 payload 는 `&'static str` 이라 «런타임 인덱스»를 «구조적으로» 담을 수 없다.**

| 제안이 요구한 것 | #67 착지 후 |
|---|---|
| 「기대」(무엇이어야 했나) | ★**달성** — 규칙 문면이 그것을 말한다 |
| 「인덱스」(몇 번째 argument) | ★**미달** |
| 「실제」(실제로 무엇이 있었나) | ★**미달** |

⇒ 그래서 **새 카드**를 좁혀서 냈다(`proposals[0]` · effort **S**) — 원 제안은 `adoptedProposals` 로 처분하고,
**남은 절반만** 정확한 범위로 다시 세운다. ★**그렇게 하지 않으면 카드가 사라지면서 잔여도 함께 사라진다.**

## ★제안이 «적지 않은» 설계 제약 하나 — 후속이 다시 발견하지 않도록

`ClassFileError` 는 ★**`Copy` 를 derive 한다**. 인덱스를 담겠다고 `String` 을 넣으면 **`Copy` 가 깨지고**
그것은 네 크레이트의 소비자에게 전파된다(`classfile/tests/test.rs` **13** · `validation.rs` **8** · `class.rs` **7** ·
`jvm-bytecode/src/error.rs` **5**).
★**깨지 않고도 된다**: 정적 사유 + `u16` 인덱스 + 발견된 `u8` 태그면 **셋 다 `Copy`** 다. 그 모양을 새 카드에 적었다.

## 왜 #67 브랜치 «위에» 쌓지 않았나 — 선택이지 누락이 아니다

⑴#67 은 **아직 approve 가 아니다**(게이트② 재검이 진행 중) ⑵그 head 는 회차마다 **움직인다**
⑶그 위에 PR 을 내면 **자식 PR** 이 되고, ★**base 가 사라지면 자식이 자동으로 닫힌다**(게이트③ 계약 5 가 있는 이유).

## 잃는 것 — 「없다」로 적지 않는다

- ★**main 은 #67 이 착지할 때까지 밋밋한 채로 남는다.** 오늘 거부된 클래스 파일을 든 사람은
  **규칙 이름조차** 못 받는다. 그것이 «중복하지 않기»의 대가이고, 숨기지 않는다.
- ★**이 회차는 제안의 값을 «전혀» 전달하지 않았다** — 전달한 것은 **순서**다.
  그 판단이 틀렸다면(예: #67 이 폐기되면) 이 회차는 **한 회차를 버린 것**이 된다.
- ★**「#67 이 착지하면 잔여가 정확히 둘」은 «오늘의 #67»** 기준이다. 그 PR 은 아직 움직이고 있다.
