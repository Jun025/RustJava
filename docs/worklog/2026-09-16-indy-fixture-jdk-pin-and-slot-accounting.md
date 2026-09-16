# 2026-09-16 — indy 픽스처 JDK 핀 + 상수풀 슬롯 회계 직접 시험

티켓 `rustjava-indy-fixture-jdk-pin-and-slot-accounting-test` — 채택 제안 **둘**을 한 회차가 닫는다
(`2026-09-16-cp-tags-16-17-execution-fixtures#p0` · `#p1`).

## ⒜ JDK 핀 — 「기록」이 아니라 「검사」다

**전제 확인(ⓑ·ⓒ)**: 이 repo 에 도구 버전을 고정하는 관용은 **없다**(`rust-toolchain.toml` 부재 ·
`.github/workflows/rust.yml` 에 `setup-java` **0건** · `java|jdk` 문자열 **0건**). 그리고 **CI 는 Java 를 깔지 않고**
이 맥의 PATH 에도 `javac` 가 없다(`javac -version` → `Unable to locate a Java Runtime`;
실물은 `/opt/homebrew/opt/openjdk/bin/javac` = **26.0.1** 로 PATH 밖에 있다).
⇒ ★**컴파일러에게 물어보는 검사는 원리적으로 불가능하다. 검사할 수 있는 것은 «커밋된 바이트»뿐이다.**

**핀 값**: `test-data/indy` 의 javac 산출물 **6개 전건이 65.0**(= `javac --release 21`).
출처는 그 픽스처를 넣은 세 회차의 worklog·REPORT 가 일치해 기록한다(`--release 21` · OpenJDK 26.0.1 javac).

**검사**: `tests/test_fixture_pins.rs` 가 `test-data/indy/*.class` 중 **`test-data/src/indy/<Outer>.java` 가
있는 것만** 골라 `65.0` 을 요구한다. 실패 메시지가 **재컴파일 명령을 그대로** 싣는다.

★**ⓒ「재생성하는 사람이 핀을 보는가」의 답 = 「빨개져서 본다」.** 문서 한 줄이 아니라 red 다 —
핀 값과 그 값을 강제하는 자리가 **같은 파일**이라 기록과 검사가 어긋날 수 없다.

★**«javac 산출물만» 고르는 것이 구조적으로 필요하다**: 형제 PR #48 이 같은 디렉터리에 **합성 픽스처**
`NotStringConcatFactory.class`(**major 52** · 생성기가 바이트로 쓴다)를 넣는다 ⇒ ★디렉터리 전수 핀이었으면
**#48 이 착지하는 순간 red** 가 됐다(그 파일을 실제로 받아 버전을 재서 확인했다). `.java` 짝 유무로 가르면
그 파일은 «핀 대상이 아님»으로 자동 분류되고, 새 javac 픽스처는 `.java` 를 넣는 순간 자동 편입된다.

**양방향 실증**
| 조작 | 결과 |
|---|---|
| 정상 | **green** |
| 핀 값을 `70` 으로 틀리게 | **red** — `StringConcat.class is class file 65.0, not the pinned 70.0` |
| ★**실제 사고 재현**: `javac`(26.0.1)를 `--release` **없이** 돌려 `Lambda.class` 재생성(major **70**) | **red** — `Lambda.class is class file 70.0, not the pinned 65.0` |
| ★**개악**: 검사를 상수 통과(`let _ = (major, minor)`)로 바꾸고 위 major 70 파일을 그대로 둠 | ★**green** — red 가 사라진다 ⇒ 그 red 는 «검사가» 만든 것이다 |

※재현 실험의 픽스처는 `git checkout` 으로 원복했고(`git status` 0건) **커밋된 픽스처는 재생성하지 않았다**.

★**핀이 «덮지 못하는» 축을 숨기지 않는다**: `--release 21` 은 **목표 버전**을 고정하지 **컴파일러 바이너리를
고정하지 않는다**. javac 21 과 26 은 같은 `--release 21` 에서 둘 다 65.0 을 낸다. 그 축은 같은 픽스처에 걸린
**상수 개수 단언**(`MethodType 1 · Dynamic 3 · MethodHandle 7 · InvokeDynamic 3`)이 부분적으로 진다. 후속 제안 `#p1`.

## ⒝ 슬롯 회계 직접 시험 — 판정: **「예」**(제안의 전제는 절반만 참이었다)

제안 문면은 「직접 시험이 **없다**」였는데 ★**그것은 틀렸다** — `long_must_fit_in_two_constant_pool_slots`
가 이미 `parse_all` 을 직접 문다. ⇒ **그대로 「추가」로 가지 않고 «무엇이 안 잡히는지»를 개악으로 쟀다.**

| 개악(two-slot 집합 변경) | 회차 «전» 무엇이 red 였나 |
|---|---|
| `M1` Long 을 1칸으로 | `constant_pool::tests::long_must_fit_in_two_constant_pool_slots` + end-to-end 3건 |
| ★`M2` **Double** 을 1칸으로 | ★**`test_class` 단 1건** — 전 JVM 을 돌리는 가장 무거운 스위트뿐 |
| ★`M3` **Integer** 를 2칸으로 | `test_array_clone_method_owner_is_a_valid_class_constant` · `test_class` — 둘 다 **실물 클래스 파일을 통째로 읽는** 시험 |

★**계측 함정 하나를 적어 둔다**: 첫 측정이 「전건 `test_class` 1건만 red」로 나왔는데 그것은 ★`cargo test` 가
**첫 실패 바이너리에서 멈추기** 때문이었다. `--no-fail-fast` 를 붙이고서야 위 표가 나왔다.

⇒ **판정 「예」**: Double 축과 「long·double **만**」 축이 **파서에서 멀리 떨어진 자리**에서만 잡힌다.
`only_long_and_double_consume_two_constant_pool_slots` 를 넣었다 — 공개 API `parse_all` 에
`[Integer, 2칸짜리, Integer]` 를 먹이고 **결과 인덱스가 `1·2·4`** 이며 **4번이 그 Integer** 인지 본다
(구현 내부가 아니라 «인덱스»를 보므로 Contract 2 의 「구현 세부 결합」 위험을 피한다).

**개악 대조(회차 «후»)**: M1·M2·M3 **전건**이 이 시험 하나로 red 가 된다(`cargo test -p classfile --lib`).
★특히 M2 는 **40초짜리 전-JVM 스위트 → 0.01초짜리 파서 단위 시험**으로 내려왔다.

## 범위

`parse_all` **구현 무접촉** · 픽스처 **재생성 0** · 형제 티켓 셋 무접촉
(★파일도 겹치지 않게 골랐다: 시험을 `tests/test_class_format.rs` 꼬리가 아니라 **새 파일**에 넣어 #48·#49 와 충돌 0).
