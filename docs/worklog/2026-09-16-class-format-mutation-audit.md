# 2026-09-16 — `tests/test_class_format.rs` 변이 저항 감사

티켓 `rustjava-adopt-cp-tag-passthrough-detectable-p0` — 채택 제안 `2026-09-16-cp-tag-passthrough-detectable#p0`.

★★**[정정 2026-09-17 · 게이트② request-changes 승계 `-fix`] 아래 「결론: 고칠 것이 없다」는 «한 자리에서 거짓»이었다.**
검수자가 **더 좁은** 개악으로 찔러 ★**`string_concat.rs` 신원 4축 중 «3축»(kind·name·descriptor)이 «전 스위트 green 인 채로» 살아남는 것**을 찾았다.
⇒ 이 회차가 그 3축의 근접 실패 픽스처를 만들어 닫았다. **전말은 맨 아래 「§정정」 절에 있다 — 그 절을 읽고 이 문단을 읽어라.**

★**원 결론(그대로 둔다)**: 11개 테스트 **전건**이 «자기가 이름 붙인 가지»의 개악에 **죽는다**.
★**그리고 제안의 전제는 «쓰인 시점에 이미 거짓»이었다**(아래 ⓒ). ⇒ **코드 변경 0** · 산출물은 **이 기록**이다.
★**그 두 문장은 여전히 참이다 — 틀린 것은 «그래서 고칠 것이 없다»는 «추론»이다**(아래 §정정).

## 감사 방법 — 제안이 말한 그 절차 그대로

제안: 「This round found one such test by **mutating the branch it named and observing green**.
The same procedure applies mechanically to the other assertions.」
⇒ 각 테스트가 **이름 붙인 제품 가지**를 하나씩 개악하고, `cargo test --test test_class_format` 를 돌려
★**어떤 테스트가 죽는지**를 기록했다. **죽지 않는 테스트 = 다른 이유로 통과하는 테스트**다.

## 개악 표 (10종 · 전부 제품 코드 · 각 1회 적용 후 복원)

| id | 파일 | 개악 | 죽은 테스트 |
|---|---|---|---|
| **M1** | `classfile/src/class.rs` | `if magic != 0xCAFEBABE {` → `if false {` | `bad_magic` |
| **M2** | `classfile/src/constant_pool.rs` | 태그 switch `_ => Err(...)` → `_ => Ok((data, Self::Integer(0)))` | `unsupported_constant_pool_tag` |
| **M3** | `classfile/src/opcode.rs` | `ldc2_w` 폭 제한 제거(무엇이든 수용) | `ldc_of_an_illegal_constant` |
| **M4** | `classfile/src/validation.rs` | `class.major_version >= minimum_major_version` → `true` | `constant_tag_below_its_minimum_class_file_version` · `ldc_of_an_illegal_constant` |
| **M5** | `classfile/src/validation.rs` | `bootstrap_method_count.is_some_and(...)` → `true` | `dynamic_constant_naming_a_missing_bootstrap_method` |
| **M6** | `jvm-bytecode/src/verifier.rs` | indy 를 `UnsupportedFeature` → `InvalidClassFile` | `every_method_handle_family_tag` · `lambda_class` · `only_the_string_concat_bootstrap_is_linked` |
| **M7** | `jvm-bytecode/src/string_concat.rs` | `StringConcatFactory` 신원 4축 검사 제거 | `only_the_string_concat_bootstrap_is_linked` |
| **M8** | `jvm-bytecode/src/verifier.rs` | `ldc`-of-MethodHandle 을 `UnsupportedFeature` → `InvalidClassFile` | `ldc_of_method_handle_family` |
| **M9** | `jvm-bytecode/src/error.rs` | `InvalidFormat => InvalidClassFile` → `UnsupportedFeature("parse")` | **6개**: `truncated` · `bad_magic` · `unsupported_constant_pool_tag` · `ldc_of_an_illegal_constant` · `constant_tag_below_its_minimum…` · `dynamic_constant_naming…` |
| **M10** | `jvm/src/jvm.rs` | 클래스 부재를 `NoClassDefFoundError` → `ClassFormatError` | `missing_class_still_raises_no_class_def_found_error` |

## 결과 — ★**11/11 이 «적어도 하나»의 개악에 죽는다**

| 테스트 | 죽인 개악 |
|---|---|
| `truncated_class_raises_class_format_error` | M9 |
| `unsupported_constant_pool_tag_raises_class_format_error` | M2 · M9 |
| `only_the_string_concat_bootstrap_is_linked` | M6 · M7 |
| `bad_magic_raises_class_format_error` | M1 · M9 |
| `missing_class_still_raises_no_class_def_found_error` | M10 |
| `class_carrying_every_method_handle_family_tag_is_unsupported_not_malformed` | M6 |
| `ldc_of_method_handle_family_reports_unsupported_feature_not_malformed` | M8 |
| `ldc_of_an_illegal_constant_is_still_malformed` | M3 · M4 · M9 |
| `a_constant_tag_below_its_minimum_class_file_version_is_malformed` | M4 · M9 |
| `a_dynamic_constant_naming_a_missing_bootstrap_method_is_malformed` | M5 · M9 |
| `lambda_class_reports_unsupported_feature_not_malformed` | M6 |

★**중간 함정 하나를 적어 둔다**: 1차 표(M1~M7)에서는 **3개가 살아남았다**.
그때 「약한 단언 3건 발견」이라고 적었으면 **거짓**이었다 — 실제로는 ★**내 개악 목록에 그 가지가 빠져 있었다**
(ldc 미지원 arm · 파스 실패 매핑 · not-found 경로). ⇒ M8·M9·M10 을 더하자 전부 죽었다.
★**「죽지 않았다」는 «테스트가 약하다»와 «내가 그 가지를 안 건드렸다»를 구별하지 못한다.**

## ⓒ 제안의 전제 — ★**쓰인 시점에 이미 거짓이었다**

원문 `why`: 「several of which **also rely on corrupting a referenced slot of `Hello.class`** and then
asserting only the flattened error kind」.

실측 — 제안이 실린 커밋(`eb8b4eb` = PR #49) 시점의 같은 파일에서 `Hello.class` 파생 픽스처는 **3곳**뿐이고,
그중 ★**«참조되는 상수풀 슬롯»을 덮는 것은 «0»** 이다:
- `hello_class()[..60]` — **절단**(슬롯 덮어쓰기가 아니다)
- `bytes[0] = 0x00` — **매직 바이트**(헤더이지 풀 슬롯이 아니다)
- `fixture("Unrelated.class", &hello_class())` — 손상 **0**(부재 테스트의 들러리)
※`bytes[6..8]`(버전)은 **실물 픽스처**에 적용되지 `Hello.class` 에 적용되지 않는다.

⇒ ★**「several」은 그 회차가 «방금 고친 그 하나»의 일반화였다.** 그 회차는 유일한 사례를 없애면서
「나머지도 그럴 것」이라고 적었고, **그 나머지는 애초에 없었다.**
★**그렇다고 감사가 헛되지 않다** — 「없다」를 «재서» 아는 것과 «추측»하는 것은 다르고, 그 차이가 이 회차의 산출물이다.

## ⓑ 이미 같은 축이 있는가 — 없다

이 저장소에 변이 테스트 도구(`cargo-mutants` 등) 설정 **0건**(실측). 변이 저항은 **회차마다 손으로** 확인해 왔다
(이 리니지의 done 회신들이 그 기록이다).

## ★도구를 «남기지 않았다» — 그 이유가 이 제안의 주제와 같다

개악 하네스는 **제품 «소스 문자열»을 매칭해 치환**한다. 그 문자열은 리팩터마다 바뀌고, 안 맞으면 하네스는
★**조용히 「그 개악을 건너뛴다」** — 즉 ★**하네스 자체가 «통과하지만 아무것도 재지 않는» 산출물**이 된다.
그것은 이 제안이 사냥하는 바로 그 형태다. ⇒ **표를 문서로 남기고 스크립트는 남기지 않는다**(위 표에 파일·치환이
그대로 있어 손으로 재현된다). ※데이터를 읽는 도구(`scripts/survey-ldc-constant-tags.py`)와 다른 판단인 이유가 이것이다 —
그쪽은 소스가 아니라 **클래스 파일**을 읽어 낡지 않는다.

## 감사의 «경계» — 무엇을 재지 않았나

- ★이 감사가 답한 질문은 **「각 테스트가 «자기가 이름 붙인 가지»의 개악에 죽는가」** 다.
  ★**「어떤 개악에도 죽지 않는 구멍이 없다」는 아니다** — 개악 목록은 내가 골랐고, 위 함정이 보여 주듯 목록은 늘 불완전할 수 있다.
- ★**픽스처의 «유일 결함성»은 별도 축**이다(#49 가 세운 그 규율). 이번엔 단언 축만 봤다.
- ★진행 중인 PR **#53**(중복 `BootstrapMethods`)·**#54**(BSM 정적 인자)의 새 테스트 2개는 **이 감사 범위 밖**이다 —
  각 회차가 **자기 라운드에서 이미 양방향 개악 대조를 붙였다**(그 done 회신에 표가 있다).


---

# §정정 (2026-09-17 · `rustjava-adopt-cp-tag-passthrough-detectable-p0-fix`)

## 무엇이 틀렸나 — 「11/11 죽었다」는 맞고, 「그래서 덮여 있다」가 틀렸다

위 표의 **M7**(「신원 4축 검사 제거」)은 `string_concat.rs` 의 네 비교를 ★**한꺼번에** 지운다.
그 개악이 죽었다는 사실이 증명하는 것은 ★**「네 축 중 «적어도 하나»가 관측된다」**뿐인데,
회신은 그것을 ★**「이 가지는 덮여 있다」**로 읽었다.

검수자가 **축을 하나씩** 지워 재니:

| 개악(축 하나만 `false` 로) | 이 회차 «전» | 이 회차 «후» |
|---|---|---|
| **R6** owning class | **KILLED** (`only_the_string_concat_bootstrap_is_linked`) | **KILLED** (2개 — 위 테스트 **+** 신규) |
| **R8** reference kind | ★**SURVIVED** — `--test` 11/0 · `--all` **570/0/1 green** | ★**KILLED** (`each_axis_of_the_factory_identity_is_observable`) |
| **R11** method name | ★**SURVIVED** — 동일 | ★**KILLED** (동일) |
| **R7** descriptor | ★**SURVIVED** — 동일 | ★**KILLED** (동일) |

★**근인은 픽스처의 수**다: 근접 실패 픽스처가 `NotStringConcatFactory`(**owning class 축**) **하나뿐**이라
나머지 세 비교는 **어떤 파일도 관측하지 못했다**. ★**생성기 docstring 이 이미 그 문장을 적어 뒀다** —
「Without such a fixture the identity check is not observable … would leave every test green while
silently linking anything」 — ⇒ 그 문장은 **한 축에만** 이행돼 있었고, 이 감사는 그 사실을 「저항한다」로 덮었다.

## ★★감사 방법론에 남기는 한 줄 — 위 교훈의 «역»

원 회신은 ★**「죽지 않았다 ≠ 테스트가 약하다」**(내 개악 목록이 불완전할 수 있다)를 값지게 적었다.
★**그 «역»이 빠져 있었고, 그 빈칸이 이 오류를 만들었다**:

> ★★**「죽었다 ≠ 그 가지가 «전부» 덮였다».**
> ★**다축 술어(`A || B || C || D`)를 «통째로» 지우는 굵은 개악은 «어느 한 축이 덮였다»만 증명한다.**
> ⇒ ★**축이 여럿인 검사는 «축 하나씩» 찔러라.** 굵은 개악의 red 는 «가장 잘 덮인 축»이 낸 것이고,
> 나머지 축에 대해서는 ★**아무 말도 하지 않는다.**

## 이 회차가 만든 것 — 픽스처 3개(제품 코드 무접촉)

`make_indy_fixtures.py` 에 `bootstrap_kind` 파라미터를 더하고(기존 기본값 6 = REF_invokeStatic) 항목 3개를 추가했다:
`NotMakeConcatWithConstants`(name 축 · **`makeConcat`** 은 실재하는 StringConcatFactory 부트스트랩이라 «있을 법한» 근접 실패다) ·
`NotFactoryDescriptor`(descriptor 축 · 끝의 `[Ljava/lang/Object;` 만 제거 — **여전히 적법한 메서드 서술자**) ·
`NotInvokeStaticFactory`(kind 축 · **7 = REF_invokeSpecial** — JVMS 4.4.8 상 Methodref 와 짝지어도 **적법**).
★**셋 다 «적법한 클래스 파일»이라 신원 검사까지 도달한다** — 상류가 먼저 거부하면 그 픽스처는 «다른 이유»로 통과하는 것이고,
그것이 바로 이 리니지가 고치려는 형태다(실측: 넷 다 `UnsupportedOperationException: invokedynamic` 로 거부 · `ClassFormatError` 아님).
★**제품 코드는 고치지 않았다** — 제품은 이미 4축을 «본다». 없던 것은 **그것을 관측 가능하게 하는 픽스처**다.

## ★하네스가 워킹트리를 오염시켰다 — 실제로 일어났고, 앵커 단언이 잡았다

이 회차의 1차 개악 하네스가 러너 시간 상한에 **SIGKILL** 돼 `finally` 복원이 **돌지 않았고**,
`string_concat.rs` 의 **name 축이 `false` 로 남았다**. 그 상태에서 돌린 R6·R8 측정은 ★**2축이 꺼진 오염된 값**이었다.
★**그것을 잡은 것이 계약 ⒡ 의 「치환 1건 단언」이다** — 다음 축을 치환하려다 `앵커 0건` 으로 즉시 죽었다.
⇒ 복원 후 **4축을 전부 재측**했고 위 표는 **청결한 트리에서의 값**이다.
★**이 사건은 위 「도구를 남기지 않았다」 결정의 «실증»이기도 하다** — 제품 소스를 치환하는 하네스는
**죽는 순간 트리를 오염시킨다**. 그래서 이번에도 **커밋하지 않았다**(스크래치에서만 썼다).

## 검증 (이 회차)

- ⒜ R8·R11·R7 **전건 KILLED**(각각 `each_axis_of_the_factory_identity_is_observable`) ·
  R8 은 `cargo test --all` 에서도 **green 이 아니다**(1 failed).
- ⒝ R6(class) **여전히 KILLED** — 게다가 **2개**를 죽인다(옛 커버리지가 밀려나지 않았다).
- ⒞ 회귀 표본 2종: **R3**(BSM 경계 off-by-one `<` → `<=`) → KILLED · **R9**(`Ldc2W` arm 제거) → KILLED.
- ⒟ 기준선: `cargo test --test test_class_format` **11 → 12 passed**(신규 `test_each_axis_of_the_factory_identity_is_observable`) ·
  `cargo test --all --no-fail-fast` **570 → 571 passed / 0 failed / 1 ignored**(27 스위트 전건 합산).
- ⒠ 픽스처는 **생성기 산출물**이다 — `python3 test-data/src/indy/make_indy_fixtures.py` 재실행으로 재생산되고,
  ★**기존 `NotStringConcatFactory.class` 는 바이트 동일**(멱등).
- ⒡ **치환 1건 단언 하네스** 사용(위 오염 사건이 그 값어치의 증거다).

## 잃는 것 (이 회차)

- ★**픽스처 3개(각 약 0.4KB) + 테스트 1개**가 늘었다. `cargo test --all` **570 → 571**,
  `test_class_format` 소요 **0.25s → 0.74s**(클래스 3개를 더 로드한다) · 전체 스위트 체감 변화는 **측정 오차 수준**이다.
- ★**생성기에 파라미터가 하나 늘었다**(`bootstrap_kind`) — 기본값이 종전 하드코딩 값(6)이라 **기존 산출물은 불변**이다.
- ★**안 만들면**: 생성기 docstring 이 예고한 그 상태 — ★**「모든 테스트가 green 인 채로 아무것이나 링크된다」**가 그대로 남는다.
  (그 상태는 가정이 아니라 **이 회차 전까지의 사실**이었고, 위 표의 «전» 열이 그 증거다.)
