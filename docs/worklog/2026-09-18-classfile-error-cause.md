# 2026-09-18 — 거부된 클래스 파일이 «왜»를 말한다 (rustjava-adopt-classfile-error-cause-decision-p0)

채택 제안 `2026-09-17-classfile-error-cause-decision#p0`.
제안은 **all-or-nothing** 이라고 스스로 못박았다 — 타입만 고치면 관측되는 것이 없고,
`||` 사슬을 안 쪼개면 **평평함이 사라지는 게 아니라 옮겨갈 뿐**이다. 넷 다 했다.

## 한 일 — 세 층을 관통한다

```
classfile::ClassFileError::InvalidFormat(&'static str)
   → jvm_bytecode::ClassDefinitionError::InvalidClassFile(&'static str)   ← From 이 «버리던» 자리
      → jvm.exception("java/lang/ClassFormatError", cause)                ← 두 자리 모두
```
그리고 `validate_class` 의 **8항 `||` 사슬**을 **규칙마다 `if` 하나**로 쪼갰다 —
클래스 8 · 필드 3 · 메서드 **3** = ★**사유 14개**(계수 = `grep -c 'ClassFileError::InvalidFormat(' classfile/src/validation.rs`)(필드의 `ConstantValue` 는 「몇 개냐」와 「타입이 맞냐」가
**한 조건에 묶여** 있었고, 그 둘을 갈랐다).

★**왜 변형이 아니라 문자열인가**: 집합이 **열려 있고**(규칙이 늘 때마다 하나씩) **아무도 분기하지 않는다**.
그리고 이 저장소에 **선례가 있다** — `ClassDefinitionError::UnsupportedFeature(&'static str)`.

## ★사유를 꿰자마자 «숨어 있던 것 둘»이 튀어나왔다

**⑴ 테스트가 «어느 층이 거부하는지»를 틀리게 믿고 있었다.**
`classfile/tests/test.rs` 의 「인덱스가 **엉뚱한 종류**를 가리킨다」 케이스는 **검증이 거부한다**고 적혀 있었는데,
사유는 **`"truncated or unparsable class file"`** — ★**파서가 거부한다**.
★**코드를 내 추측에 맞추지 않고 단언을 실측에 맞췄다**(주석에 「measured, not assumed」를 박았다).
※바로 옆 루프(`reference kind` 1·4·9)는 **반대로** 검증이 거부한다 — ★그 대비가 이제 **사유로 보인다**.

**⑵ 술어의 «이름»이 낡아 있었고 아무도 몰랐다.**
`bootstrap_method_static_arguments_are_in_the_pool` 은 이름과 달리 **「적재 가능 상수인가」까지** 요구한다
(자기 docstring 이 그렇게 적고 OpenJDK 의 `bad constant type` 까지 인용한다 — 직전 회차가 규칙을 **넓혔다**).
⇒ 사유는 **규칙 그대로** 적었다: `"a bootstrap method argument names nothing or is not a loadable constant"`.
★**함수 이름은 바꾸지 않았다** — 리팩터는 이 회차 범위 밖이다(계약 3). 그 사실을 코드 주석에 남겼다.

★★**둘 다 «평평한 오류»가 가리고 있던 것**이다. 사유가 없을 땐 **어느 것도 틀릴 수 없었다** — 물을 수가 없었으니까.

## 양방향 — 세 층 전부에 개악을 놓았다

| 개악 | 결과 |
|---|---|
| **M1** `src/runtime.rs` 가 다시 `"Invalid class file"` 를 박는다 | ★**red** |
| **M2** `From` 이 다시 사유를 **버린다**(제안이 지목한 바로 그 버그) | ★**red** |
| **M3** 서로 다른 두 사유를 **한 문자열**로 접는다 | ★**red** — 줄마다의 `assert!(err.contains(cause))` 가 잡는다(`tests/test_class_format.rs:452`) |
| 복원 | **green** 17/0 |

★**M3 이 없으면** 「전부 같은 문자열로 되돌려도 통과」가 가능하다 — 그래서 테스트가 **사유들이 서로 다름**까지 단언한다.

## ★대가 — 실측한 구멍 하나를 포함해서

- ★★**마지막 홉이 «두 번» 쓰여 있고 한 쪽만 테스트가 본다**(실측): `src/runtime.rs` ↔ `test-utils/src/lib.rs`.
  ★**test-utils 사본만 개악하면 `cargo test --all` 이 `579 passed / 0 failed`** — **아무것도 울지 않는다**.
  ★**합치는 것은 리팩터라 하지 않았고**, 대신 **구멍을 보고한다**.
- 사유가 **문자열**이라 두 규칙에 같은 문구를 주는 것을 막는 것이 없다. ★**그리고 그것을 «막는다»고 적었던 dedup 단언은 공허했다** — `seen` 에 담기던 것이 제품의 출력이 아니라 **표의 기대 리터럴**이라 상수끼리 비교했고, 제품이 무엇을 내든 결과가 같았다. ⇒ **걷어냈다.** 남는 보장은 `contains` 가 덮는 **그 세 픽스처**뿐이다.
- ★**픽스처 규율을 대체하지 않는다**(제안이 이미 적었다) — 사유는 「어느 검사가 울었나」이지
  「다축 검사의 각 축이 관측되나」가 아니다.
- ★**PR #66 과 같은 함수를 만진다** — 뒤에 착지하는 쪽이 base 를 당겨 그 항을 다시 쪼갠다. 충돌은 실재하지만 **기계적**이다.

## 검증
`cargo test --all` **578 → 579 / 0 failed / 1 ignored** · `classfile` **15+13/0** · `test_class_format` **17/0** · `check-dod-ci-parity` → **「OK 두 축 모두 대칭차 0 — 명령 6개 · toolchain 2개로 «둘 다 일치»」**(rc=0).
