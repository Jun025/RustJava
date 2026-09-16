# 2026-09-16 — `tests/test_class_format.rs` 변이 저항 감사

티켓 `rustjava-adopt-cp-tag-passthrough-detectable-p0` — 채택 제안 `2026-09-16-cp-tag-passthrough-detectable#p0`.

★**결론: 고칠 것이 없다.** 11개 테스트 **전건**이 «자기가 이름 붙인 가지»의 개악에 **죽는다**.
★**그리고 제안의 전제는 «쓰인 시점에 이미 거짓»이었다**(아래 ⓒ). ⇒ **코드 변경 0** · 산출물은 **이 기록**이다.

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
