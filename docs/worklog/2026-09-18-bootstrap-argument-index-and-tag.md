# 부트스트랩 인자 거부가 «어느 인자·무엇을 찾았는지» 말한다

채택 제안 `2026-09-18-bootstrap-argument-diagnostic-sequencing#p0`.

## ⓐ 「index 를 알고 있는데 버린다」 — ★코드에서 확인했다. 맞다.

`classfile/src/validation.rs` 의 술어는 `-> bool` 이었고 본체가 이랬다:
```rust
method.arguments.iter().all(|index| { class.constant_pool.get(index).is_some_and(...) })
```
★클로저가 `false` 를 내는 그 자리에 **`index` 가 손에 있고**, `constant_pool.get(index)` 가 **찾은 항목**이다.
`bool` 로 접는 순간 둘 다 사라지고, 호출부는 **재지 않은 산문**으로 실패를 설명해야 했다.
⇒ ★**이 회차는 «새 기능»이 아니라 «이미 계산된 값을 버리지 않는 것»이다.**

## ⓑ `&'static str` 제약 — 실재한다. 그리고 ★**대안은 «만들 필요가 없었다»**

`ClassFileError::InvalidFormat(&'static str)` 은 런타임 index 를 담을 수 없다(제안의 서술 그대로).
★**그러나 같은 enum 에 `UnsupportedVersion(u16)` 이 이미 있다** — 즉 «구조화 변형을 더해 3층으로 나르는 법»이
**이 저장소에 이미 있고 작동 중**이다:
```
ClassFileError::UnsupportedVersion(u16)
  → ClassDefinitionError::UnsupportedClassVersion(u16)          (jvm-bytecode/src/error.rs 의 From)
  → format!("Unsupported class file version {version}")          (src/runtime.rs · test-utils/src/lib.rs)
```
⇒ ★**그 패턴을 «복제»했다. 새 진단 체계 0.** 계약 1 이 금지한 것이 정확히 그것이다.

## ⓒ 소비자 전수 — 기존 경로는 무변

`InvalidFormat` 사용처 **34건**(`validation.rs` 15 · `classfile/tests/test.rs` 11 · `class.rs` 3 ·
`tests/test_class_format.rs` 2 · `error.rs` 2 · `jvm-bytecode/src/error.rs` 1).
★**구조적 소비자는 «하나»다** — `jvm-bytecode/src/error.rs` 의 `From<ClassFileError>` 의 `match`.
그 arm 을 **더했을 뿐 고치지 않았다**. 나머지 14개 규칙은 `InvalidFormat` 을 그대로 쓴다.
※이 규칙의 메시지를 단언하던 시험 **2곳**은 바뀐다 — ★그것이 이 회차의 산출물이지 회귀가 아니다.

## 전/후 — 같은 픽스처, 실제 출력

픽스처 `test-data/ldc/LdcDynamicBSMArgPastEnd.class`(인자가 풀 밖을 가리킨다) · CLI 로 실행:
```
BEFORE  java.lang.ClassFormatError: a bootstrap method argument names nothing or is not a loadable constant
AFTER   java.lang.ClassFormatError: bootstrap method #0 argument #0 names no constant pool entry
```
★**AFTER 는 이 브랜치에서 실제로 받은 출력이다.** BEFORE 는 `origin/main` 의 자기 시험이 단언하던 문면이다
(`tests/test_class_format.rs` — `err.contains("java.lang.ClassFormatError")` + 그 cause).

## 세 요구 충족 — ★**3/3**

| 요구 | 충족 | 어디에 |
|---|---|---|
| **expected** | ✔ | 「is not a loadable constant」(종류가 있을 때) · 「names no constant pool entry」(없을 때) |
| **index** | ✔ | `argument_index` — ★**그리고 `method_index` 도**(아래) |
| **actual** | ✔ | `actual: Option<&'static str>` = 실제로 가리킨 상수의 **종류 이름** |

★**OpenJDK 보다 한 칸 더 말한다**: OpenJDK 는 `argument_index 4 has bad constant type` 로 **어느 bootstrap
method 인지 말하지 않는다** — `BootstrapMethods` 항목이 여럿이면 모호하다. 그래서 `method_index` 를 함께 싣는다.

★**«태그»를 번호가 아니라 «이름»으로 나른다**: 파싱 후 태그 바이트가 보존되지 않아 번호를 실으려면
**아무도 분기하지 않는 숫자를 담을 두 번째 표**를 만들어야 한다. 이름은 `ConstantPoolItem` 변형에서 **파생**되므로
표가 낡을 수 없다. ⇒ 「무엇을 찾았나」는 채워지고 유지보수면은 늘지 않는다.

## 양방향

| 축 | 결과 |
|---|---|
| ⒜ ★**index 가 «따라 바뀌는가»** — `Lambda.class`(정적 인자 **3개**)의 **#0** 과 **#2** 를 각각 Utf8 로 망가뜨림 | ★**`argument_index` 가 0 ↔ 2 로 따라간다**(상수 박힘 아님) |
| ⒝ 정상 입력 | 전 픽스처 파싱 불변 · `cargo test --all` green |
| ⒞ 기존 경로 회귀 | `InvalidFormat` 14규칙·`UnsupportedVersion` 시험 전건 green |
| 종류 축 | Utf8 을 가리키면 `actual: Some("Utf8")` · 풀 밖이면 `actual: None` |

★**⒜ 가 이 축의 전형적 거짓 통과를 막는 자리다** — `StringConcat` 은 인자가 **1개**라 index 가 0 이고,
**계산해도 하드코딩해도 통과한다.** 인자 3개짜리 람다가 그 둘을 가른다.

## 대가 — 「없다」로 적지 않는다

- ★**타입은 «커지지 않았다» — 재서 적는다**: `ClassFileError` 는 여전히 **`Copy`** 이고 크기도 그대로다
  (`size_of::<ClassFileError>() == size_of::<&'static str>() + size_of::<usize>()` 를 시험으로 잠갔다).
  `u16` 둘 + `&'static str` 이 `InvalidFormat` 이 이미 쓰던 자리에 들어간다 ⇒ **할당 0 · 핫 경로 영향 0.**
  ※제안이 경계한 「소유 데이터로 커진다」는 **이 설계에서는 일어나지 않았다** — 소유 문자열을 쓰지 않아서다.
- ★**두 경로가 공존한다** — 어느 쪽을 쓸지 헷갈릴 자리다. ⇒ ★**변형 docstring 에 한 줄로 못박았다**:
  「`InvalidFormat` 은 가리킬 것이 없는 규칙용이고 그것이 여전히 대부분이다. 이 변형은 **수가 이미 손에 있을 때만**」.
  ★그 주석이 코드와 어긋나지 않는지 **양방향 시험으로 확인했다**(위 ⒜ — 실제로 수를 싣는다).
- ★**메시지 문면이 바뀐다** — 이 규칙의 메시지를 단언하던 시험 2곳과, 그 문구로 grep 하던 사람의 습관이 깨진다.
- ★**`method_index` 는 OpenJDK 와 다른 문면**이다 — 로그를 기계로 대조하는 쪽이 있으면 그쪽이 깨진다(이 repo 엔 없다).

## 후속 추천

⑴★**나머지 규칙 중 «수를 아는데 버리는» 것을 세라**(S) — 이 회차는 **한 규칙**만 고쳤다. `validation.rs` 의
   다른 14규칙 중 몇이 같은 형상인지 **아직 모른다**. 세고 나서 값하면 같은 패턴으로 옮기면 된다.
⑵★**`ClassDefinitionError` 의 `&'static str` 두 변형**(`InvalidClassFile`·`UnsupportedFeature`)도 같은 한계를
   갖는다(M) — 이 회차는 **우회**했다(새 변형을 나란히 더했다). 층을 통째로 구조화할지는 별 판단이다.
