# REPORT
## [2026-09-18] 부트스트랩 인자 거부가 «어느 인자·무엇을 찾았는지» 말한다 (rustjava-bootstrap-argument-diagnostic-names-index-and-tag)
- 무엇을: 채택 제안 `2026-09-18-bootstrap-argument-diagnostic-sequencing#p0`(worklog json 기록). ★**새 기능이 아니라 «이미 계산된 값을 버리지 않는 것»** — 술어가 `false` 를 내는 그 자리에 index 와 찾은 항목이 **손에 있었다**.
- ★**ⓑ 대안을 만들지 않았다** — 같은 enum 의 `UnsupportedVersion(u16)` 이 **이미 3층을 관통해 경계에서 `format!` 되는 패턴**이라 그것을 **복제**했다(새 진단 체계 **0**).
- ★**전/후**(같은 픽스처 `test-data/ldc/LdcDynamicBSMArgPastEnd.class` · CLI 실제 출력):
  `BEFORE  java.lang.ClassFormatError: a bootstrap method argument names nothing or is not a loadable constant`
  `AFTER   java.lang.ClassFormatError: bootstrap method #0 argument #0 names no constant pool entry`
- ★**세 요구 3/3**: expected(문면) · index(`argument_index` ★+`method_index` — OpenJDK 는 그걸 안 말해 여럿일 때 모호하다) · actual(가리킨 상수의 **종류 이름**). ★태그를 **번호가 아니라 이름**으로 나른다 — 태그 바이트가 파싱 후 남지 않아 번호는 «아무도 분기 안 하는 두 번째 표»를 만들게 된다.
- ★★**양방향의 급소** — `StringConcat` 은 인자가 **1개**라 index 0 이 **계산이든 하드코딩이든 통과한다**. 그래서 정적 인자 **3개**짜리 `Lambda.class` 의 **#0 과 #2** 를 각각 망가뜨려 ★**보고된 index 가 0 ↔ 2 로 따라가는 것**을 잠갔다.
- ★**소비자 전수**: `InvalidFormat` 34사용처 중 **구조적 소비자는 1개**(`jvm-bytecode` 의 `From` match) — **arm 을 더했을 뿐 고치지 않았다**. 나머지 14규칙 무변.
- ★**대가**(재서 적는다): ★**타입이 «커지지 않았다»** — `Copy` 유지 · 크기 불변(시험으로 잠금) ⇒ 할당 0. ※제안이 경계한 「소유 데이터로 커진다」는 **이 설계에선 일어나지 않았다**. ★**두 경로 공존**의 혼동은 변형 docstring 한 줄로 못박았다 · ★이 규칙의 **메시지 문면이 바뀌어** 그것을 단언하던 시험 2곳과 grep 습관이 깨진다.
- 검증: `cargo test -p classfile` **16 passed**(전 13) · `--test test_class_format` **22 passed** · `cargo test --all` rc=0.
- ★후속 추천: ⑴**다른 14규칙 중 «수를 아는데 버리는» 것을 세라**(S — 아직 그 수를 모른다) ⑵`ClassDefinitionError` 의 `&'static str` 두 변형도 같은 한계(M · 이 회차는 나란히 더해 **우회**했다). 상세 = `docs/worklog/2026-09-18-bootstrap-argument-index-and-tag.md`.

## [2026-09-18] 「어느 bootstrap argument 가 왜 나빴나」 — ★**대전제 ⓒ 에서 끝난다: 그 일을 하는 축이 이미 떠 있다**(rustjava-adopt-loadable-bootstrap-arguments-diagnostic)
- 무엇을: 채택 제안 `2026-09-17-loadable-bootstrap-arguments#p0` 의 처분(worklog json `adoptedProposals` 기록). ★**코드 0행** — `classfile/src/{error,validation}.rs` **무접촉**.
- ★**제안의 전제는 참이다 — CLI 로 돌려서 봤다**(`main` @ `8c7b473f`): 서로 다른 세 규칙(`LdcDynamicBSMArgPastEnd` 나쁜 argument · `LdcDynamicDuplicateBSM` 중복 속성 · `LdcDynamicOldMajor` 버전 게이트)이 ★**글자 하나 다르지 않은 `java.lang.ClassFormatError: Invalid class file`** 를 낸다.
- ★★**겹침을 «두 축»으로 갈라야 한다**(초판은 「전부」로 뭉쳤고 ★그 낱말이 카드의 과소 산정을 낳았다): ★**편집 영역은 전부 겹친다** — **PR #67** 이 제안의 `target` **바로 그 두 파일**을 고치고(merge-base 대비 `error.rs` **10/1** · `validation.rs` **61/27**), ★**이 술어에 이미 사유를 준다**(`"a bootstrap method argument names nothing or is not a loadable constant"`), ★밋밋한 문면이 박힌 **두 경계 자리**(`src/runtime.rs:189`·`test-utils/src/lib.rs:334`)도 **둘 다** 고쳤다. ⇒ main 에서 시작하면 **같은 enum 을 세 번째로 고치고** 같은 커버리지 구멍을 다시 발견한다.
  ★**제안 자신이 그렇게 적어 두었다** — `tradeoff`: 「the two should be done together rather than twice」.
- ★**남는 것은 있다 · 다만 좁다**: `#67` 의 payload 는 **`&'static str`** 이라 ★**런타임 인덱스를 구조적으로 못 담는다** ⇒ 「기대」는 **달성**, 「인덱스」·「실제」는 **미달**. ⇒ ★**새 카드를 좁혀 냈다**(effort **S**) — 원 제안은 처분하고 잔여만 정확한 범위로 다시 세운다(안 그러면 카드와 함께 잔여도 사라진다).
- ★**제안이 적지 않은 설계 제약**: `ClassFileError` 는 **`Copy`** 이고 ★**파일 4 · 크레이트 «2»**(`classfile` 3 + `jvm-bytecode` 1)가 그것을 쓴다 — ★초판의 「네 크레이트」는 **명사가 틀렸다**(수는 맞다 · 루트 2건은 **주석**이다)(`tests/test.rs` 13 · `validation.rs` 8 · `class.rs` 7 · `jvm-bytecode/src/error.rs` 5). ★**깨지 않고도 된다** — 정적 사유 + `u16` 인덱스 + `u8` 태그면 셋 다 `Copy`. 후속이 다시 발견하지 않도록 새 카드에 적었다.
- ★**#67 위에 쌓지 않은 이유**(선택이지 누락 아님): ⑴아직 approve 아님(게이트② 재검 중) ⑵head 가 회차마다 움직임 ⑶**자식 PR** 이 되어 base 소멸 시 자동으로 닫힌다(게이트③ 계약 5).
- ★**잃는 것**: ★**main 은 #67 착지까지 밋밋한 채로 남는다**(오늘 사용자는 **규칙 이름조차** 못 받는다) · 이 회차는 제안의 값을 **전혀 전달하지 않았고** 전달한 것은 **순서**다 · #67 이 폐기되면 이 판단은 **한 회차를 버린 것**이 된다.
- 검증: `cargo test --all` **583 passed / 0 failed / 1 ignored**(불변 — 코드 무접촉 · base `8c7b473f`) · `check-dod-ci-parity` → **「OK 두 축 모두 대칭차 0 — 명령 6개 · toolchain 2개로 «둘 다 일치»」**.
- ★후속 추천: 새 카드 「**구조화된 variant 로** bootstrap argument 의 인덱스와 태그를 말한다」(★**M** · ★초판은 `S` 였다 — **실측 후 올렸다**: 잔여도 #67 과 **같은 3층**을 건넌다(중간층 `jvm-bytecode` 도 `&'static str` · 경계는 `&str`) ⇒ `target` **5파일 / 4크레이트**(`classfile`·`jvm-bytecode`·`RustJava`·`test-utils` — ★**층은 3인데 크레이트는 4다**: 경계 층 하나가 두 크레이트에 걸친다). ★`InvalidFormat` 을 넓히면 생성 **17**곳 + 값 매치 **11**곳이라 **새 variant** 를 고르되 ★**두 갈래가 생기는 대가**를 카드에 적었다) — ★**#67 «뒤»에** · 상세 = `docs/worklog/2026-09-18-bootstrap-argument-diagnostic-sequencing.md`.
## [2026-09-18] 루트 fixture 를 한 target 으로 모을 것인가 — ★**모으지 않는다**(rustjava-adopt-test-data-version-freeze-uniform-target-p0)
- 무엇을: 채택 제안 `2026-09-17-test-data-version-freeze#p0` 의 **결정**(worklog json `adoptedProposals` 기록). ★**코드 0행 · 재컴파일 0 · `.class` 바이트 0 변경** — 산출물은 `docs/test-data-target-policy.md` 와 그 근거다.
- ⑴**분포**(동결 파일이 아니라 fixture 자신에서 읽었다): 루트 **114**건 · major **52×40 · 65×62 · 66×8 · 68×1 · 70×3** — 동결 파일 머리주석과 일치.
- ⑵★★**양방향으로 갈랐다 — 이것이 이 회차의 실질이다**:
  ⒜★**버전이 곧 시험 대상**: **문자열 연결**(52 의 `StringBuilder` 3건을 21 로 재컴파일하면 ★`BootstrapMethods` 가 생기고, 그중 **둘**은 `StringBuilder` 가 사라진다 — ★**셋째 `$FailingAppendable` 은 남는다**(`FormatterIntegration.java:36` 의 명시적 필드 선언 = 낮춤 산물이 아니다) · ★`StringConcat.class` 는 루트에서 indy 를 가진 **유일한** fixture ⇒ **낮춤 전략마다 하나씩**) · **nestmate**(`ThreadInterruption` `access$`**×10** → 21 에서 **0 + `NestMembers`** · `MonitorSemantics` ×4 동일 · JEP 181).
  ⒝★**아무 버전이나 되는 것**: 「`StringBuilder`·indy 둘 다 없고 단독 재빌드 가능」한 20건을 21 로 재컴파일해 **명령 시퀀스 전체 대조** → ★**16건 완전 동일**.
  ⇒ ★**본 20건 중 «3건»이 버전이 답이고(nestmate 2 + `NativeMethod`) 1건은 단독 재빌드 불가, 16건은 아무래도 좋다.** ★초판이 적은 「5건」은 **자기 산술과 어긋났다**(5+16=21≠20) — 선별에서 «이미 배제한» `StringBuilder` 2건을 얹어야 나오는 수이고, ★**결론을 더 세게 보이게 하는 방향의 오차**였다(결론은 이 비가 아니라 「0·0」에 선다).
- ★★**그 커버리지는 다른 데 없다**: `test-data/{cp,indy,ldc,attr}` 생성기 산출 **64건 전수**에서 `StringBuilder` **0** · `access$` **0** ⇒ 루트 52 무리가 **유일한 시험면**이다.
- ★**결론의 근거**: 제안의 이득(「숫자 하나로 예측」)이 실측에 **뒤집힌다** — 지금 major 52 는 「전-indy·전-nestmate」라는 **뜻을 실제로 갖고**, 전부 펴면 그 구분이 사라지며 **런타임이 아직 구현해야 하는 두 경로의 유일한 커버리지**가 지워진다. 대가도 실재한다(핀 `test_fixture_pins` · 루트 **66건**의 `.txt` 출력 대조).
- ★**잃는 것**: 비균일은 그대로 남고(신규 fixture 의 target 규칙은 **세우지 않았다** — 별 축) · 16건은 「아무래도 좋은 채」로 남으며 · ★**본 것은 40 중 20 이다**(16/20 을 40 의 비로 읽지 마라) · ★`NativeMethod` 의 명령 차이는 **원인을 못 밝혔다**.
- 검증: `cargo test --all` **583 passed / 0 failed / 1 ignored**(불변 — 코드 무접촉 · ★이 브랜치 base `8c7b473f` 기준이다. 같은 날 앞 회차들의 **579** 는 PR #61 착지 «전» base 의 수라 다르다) · `check-dod-ci-parity` → **「OK 두 축 모두 대칭차 0 — 명령 6개 · toolchain 2개로 «둘 다 일치»」**.
- ★후속 추천(★**worklog `.json` `proposals[]` 에 카드 2장으로 «기계 채널»에 실었다** — 초판은 `REPORT` 에만 적어 cockpit 에 **0장**이었다): ⑴**신규 fixture 의 target 규칙**을 세울 것인가(M) ⑵**되돌릴 조건에 «관측자»를 붙인다**(S · 넷 중 셋은 문서를 열어야만 발화한다 — 게이트② 실측). ★초판이 ⑴로 적은 「`NativeMethod` 제3 축 여부」는 ★**검수자가 규명해 닫혔다**(축 2 의 다른 얼굴) ⇒ 카드로 내지 않는다. 상세 = `docs/worklog/2026-09-18-root-fixture-target-decision.md`.

## [2026-09-18] 루트 픽스처 다섯이 재빌드되지 않는 이유 — ★**`-g` 다. 제안이 댄 두 설명은 «둘 다» 틀렸다** (rustjava-adopt-javac-fixture-provenance-verified-p0)
- 무엇을: 채택 제안 `2026-09-17-javac-fixture-provenance-verified#p0`(worklog json `adoptedProposals` 기록). ★**제품 Rust 0줄 · 커밋된 `.class` 바이트 «0 변경»** — 고친 것은 검증 스크립트 한 자리다.
- ★**답**: 커밋본은 **디버그 정보를 달고**(`-g`) 컴파일됐고 스크립트는 **그것 없이** 재빌드했다. javac 기본은 `-g:lines,source` 라 `LocalVariableTable` 이 안 나온다. 단서는 `javap -v -p` 대조(커밋본 902B 에만 `LocalVariableTable` 과 `this`·`args`·`oe`…).
- ★★**제안의 두 설명을 측정으로 반증했다**: ⒜**다른 컴파일러 아니다** — 같은 다섯이 **26.0.1 과 26.0.2.1 에서 똑같이** 다르고 둘 다 `-g` 면 **똑같이 동일**하다(두 판본 각각 직접 실행 · 26.0.1 keg 잔존) ⒝**소스 발산 아니다** — `-g` 만 주면 **지금 소스가 커밋 바이트를 정확히 낸다**.
  ⇒ ★★**제안이 가장 걱정한 대가가 사라진다** — `tradeoff` 의 「재컴파일이 동작 변경이 될 수 있다」는 ★**재컴파일 자체가 불요**라 성립하지 않는다.
- ★**고친 한 자리**: 스크립트가 이미 `--release` 를 픽스처에서 읽으므로 **`-g` 도 같은 자리에서** 읽게 했다(상수풀의 `LocalVariableTable` 유무). ★**배선 전에 판별력을 쟀다** — 보유 **5** · 미보유 **107** · ★**5/5 · 오탐 0**.
- ★**양방향**: 정상 **109 rebuilt / 109 reproduced / 0 differed** ↔ ★개악(`-g` 파생 한 줄 no-op) **104 / 5 differed**(✗ 목록이 원래 다섯과 동일) · 복원 0.
- ★**잃는 것**: ⒜스크립트는 **여전히 `rc=1`** — 재빌드 불가 **3건**(형제 참조 소스 · `-sourcepath` 미사용)은 제안이 미해결로 적은 **별 축**이라 넓히지 않았다 ⒝판정이 **한 속성의 유무**에 걸린다(`-g:none` 재생성은 조용히 드리프트로 읽힌다).
- 검증: `cargo test --all` **579 passed / 0 failed / 1 ignored**(불변 — Rust 무접촉) · `check-dod-ci-parity` → **「OK 두 축 모두 대칭차 0 — 명령 6개 · toolchain 2개로 «둘 다 일치»」**.
- ★후속 추천: 재빌드 불가 3건의 컴파일 방법 결정(M) — 상세 = `docs/worklog/2026-09-18-five-fixtures-were-built-with-g.md`.

## [2026-09-18] 거부된 클래스 파일이 «왜»를 말한다 — 세 층을 관통하는 사유 (rustjava-adopt-classfile-error-cause-decision-p0)
- 무엇을: 채택 제안 `2026-09-17-classfile-error-cause-decision#p0`. ★**제품 동작 변경 있음** — `ClassFormatError` 메시지가 **모든 거부에 같던 「Invalid class file」** 에서 **사유별 문장**으로 바뀐다.
- ★**제안이 스스로 all-or-nothing 이라 못박았다** — 타입만 고치면 **관측되는 것이 없고**, `||` 사슬을 안 쪼개면 **평평함이 사라지는 게 아니라 옮겨갈 뿐**이다. 넷 다 했다:
  `ClassFileError::InvalidFormat(&'static str)` → `ClassDefinitionError::InvalidClassFile(&'static str)`(★`From` 이 **버리던** 자리) → 두 경계 자리 모두 사유를 그대로 던진다 · `validate_class` 의 **8항 `||` 사슬 → 규칙마다 `if` 하나**(사유 **14개**(클래스 8 · 필드 3 · 메서드 3 — `grep -c 'ClassFileError::InvalidFormat('` 로 센 값) · 필드 `ConstantValue` 는 「몇 개냐」와 「타입이 맞냐」가 **한 조건에 묶여** 있어 갈랐다).
- ★**왜 «변형»이 아니라 «문자열»인가**: 집합이 **열려 있고**(규칙마다 하나) **아무도 분기하지 않는다**. 선례도 있다 — `ClassDefinitionError::UnsupportedFeature(&'static str)`.
- ★★**사유를 꿰자마자 «평평한 오류가 가리고 있던 것 둘»이 나왔다**:
  ⑴**테스트가 «어느 층이 거부하는지»를 틀리게 믿고 있었다** — 「인덱스가 엉뚱한 종류를 가리킨다」는 **검증**이 아니라 ★**파서**가 거부한다(`truncated or unparsable class file`). ★**코드를 추측에 맞추지 않고 단언을 실측에 맞췄다**(주석에 「measured, not assumed」).
  ⑵**술어 이름이 낡아 있었다** — `bootstrap_method_static_arguments_are_in_the_pool` 은 이름과 달리 **「적재 가능 상수인가」까지** 요구한다(직전 회차가 넓혔고 자기 docstring 이 그렇게 적는다). 사유는 **규칙 그대로** 적고 ★**함수 이름은 바꾸지 않았다**(리팩터 = 범위 밖).
- ★★**양방향 — 세 층 «전부»에 개악**: **M1** `src/runtime.rs` 가 다시 문자열을 박는다 → red · **M2** `From` 이 다시 사유를 버린다(제안이 지목한 그 버그) → red · **M3** 두 사유를 한 문자열로 접는다 → red · 복원 **17/0**. ★★**M3 을 잡는 것은 줄마다의 `assert!(err.contains(cause))` 다**(`tests/test_class_format.rs:450` — 실행이 루프 끝에 **도달조차 하지 않는다** · ★초판은 `:452` 라 적었으나 dedup 2줄 제거로 **:450 으로 옮겨졌다**). ★**초판은 이것을 시험 말미의 dedup 단언에 귀속시켰는데 틀렸다** — 그 벡터에 담기던 것은 제품의 출력이 아니라 **표의 기대 리터럴**이라 **상수끼리 비교**했고 제품이 무엇을 내든 결과가 같았다. ⇒ ★**주석만 고치지 않고 그 블록을 걷어냈다**(잃는 것은 아래 대가에 적는다).
- ★★**대가 — 실측한 구멍 하나를 포함해 적는다**: ⒜★**마지막 홉이 «두 번» 쓰여 있고 한 쪽만 테스트가 본다** — `test-utils/src/lib.rs` 사본만 개악하면 `cargo test --all` 이 **579 passed / 0 failed**(아무것도 안 운다). ★**합치는 것은 리팩터라 하지 않았고 구멍을 보고한다.** ⒝사유가 문자열이라 **두 규칙에 같은 문구**를 주는 것을 막는 것이 ★**아무것도 없다** — 초판이 그것을 막는다고 적은 dedup 단언은 공허했고 **걷어냈다**(:10) ⇒ 남는 보장은 `contains` 가 덮는 **그 세 픽스처**뿐이다 ⒞★**픽스처 규율을 대체하지 않는다**(제안이 이미 적었다) ⒟★**PR #66 과 같은 함수를 만진다** — 뒤에 착지하는 쪽이 base 를 당겨 그 항을 다시 쪼갠다(충돌은 실재하나 **기계적**).
- 검증: `cargo test --all` **578 → 579 / 0 failed / 1 ignored** · `classfile` **15+13/0** · `check-dod-ci-parity` → **「OK 두 축 모두 대칭차 0 — 명령 6개 · toolchain 2개로 «둘 다 일치»」**(rc=0 · ★수를 직접 세지 않는다 — `CLAUDE.md` §DoD 규율).

## [2026-09-17] 코드 2파일 합집합 — ★**그런데 ours 의 «삭제»는 의도가 아니라 선행 머지의 «조용한 롤백»이었다** (rustjava-adopt-link-stringconcatfactory-p2-fix3)
- 무엇을: 게이트③이 `code-conflict-out-of-scope` 로 세운 PR #61 의 충돌 4파일(원장 2 + 코드 2)을 합집합으로 해소. ★제품 Rust **0줄**(테스트·픽스처 생성기만).
- ★★**브리프의 전제 하나가 반증됐다** — 「ours 가 «의도적으로» 지운 54·16줄을 되살리지 마라」였는데, 두 파일의 성격이 **정반대**였다:
  ⒜`make_indy_fixtures.py` 의 54줄은 ★**전건이 makeconcat 가족**(`fieldref`·`MAKECONCAT_DESCRIPTOR`·`make_concat_call_site`·`LINKED` 표·쓰기 루프)이고,
  ★**ours 가 지운 적이 없다** — 선행 회차의 머지 «둘»(`e53b2142` `-p2-fix` · `514d5b08` `-p2-fix2`)이 **부모2에 있던 것을 결과에서 떨어뜨렸다**(양쪽 다 부모2=1 → 결과=0).
  ⒝`tests/test_class_format.rs` 의 16줄은 ★**진짜 ours 의도** — metafactory 를 링크하게 만들었으니 「LambdaMetafactory 는 링크되지 않는다」 단언 2개가 **거짓이 됐다**.
- ★**그래서 처분을 갈랐다**: ⒜는 **되살리고** ⒝는 **되살리지 않았다**. 회계로 보인다 — `.py` 해소본↔main **156/0**(삭제 0) · `.rs` 해소본↔HEAD **86/0**(ours 무손실) · `.rs` 해소본↔main **182/29**(그 29줄 = ours 가 지운 2함수 전문).
- ★★**되살린 쪽이 «죽은 코드»가 아님을 실행으로 보였다**: `MakeConcat{,WrongDescriptor,WithArgument}.class` **3장을 지우고 생성기를 재실행**하니 **바이트 동일하게 복구**됐다. ★대조 — 복원 «전» 생성기에는 `LINKED`·`make_concat_call_site` 가 **0건**이라 그 3장을 낼 수 없었다. ⇒ 방치했으면 **생성기가 설명하지 못하는 커밋 픽스처 3장**이 그대로 착지했다.
- ★**양방향 개악**: ours 픽스처만 치우면 **ours 4건만 red**(theirs ok) · theirs 픽스처만 치우면 **theirs 1건만 red**(ours ok) ⇒ 두 축이 **독립**이다.
- 검증: `cargo test --all` **583 passed / 0 failed / 1 ignored** · 픽스처 핀 **3/0** · 기존 픽스처 **바이트 불변** · DoD 7명령 rc=0.
- ★후속 추천: 「머지 결과에서 «부모2에만 있던 심볼»이 사라졌는지」를 세는 검사(S) — 이번 손실은 **충돌 표시가 전혀 없는 깨끗한 자동 병합**이라 사람도 `mergeable` 도 못 본다. 상세 = `docs/worklog/2026-09-17-union-restores-silently-dropped-makeconcat.md`.

## [2026-09-17] #57 의 버전 표에 이 PR 의 픽스처 25행을 등재한다 — ★**착지 «순서»가 만든 부채** (rustjava-adopt-link-stringconcatfactory-p2-fix2)
- 무엇을: `test-data/class-file-versions.txt` 에 **25행 추가**(생성기 실행 · 손편집 0) + base 당김. ★제품 Rust **0줄** · 픽스처 재생성 **0** · 테스트 코드 **무접촉**.
- 왜: PR **#57** 이 「미등재 픽스처는 핀을 실패시킨다」를 **의도적으로** 세우고 06:37 에 착지했다. #61 의 25개 픽스처는 그보다 **먼저** 만들어졌으므로, 착지한 그 순간부터 표에 25행을 빚졌다. ★**CI 도 충돌도 아니다** — 핀에서 rc=0 CI_GREEN · `git merge origin/main` 코드 충돌 0인데 **합친 결과**가 규율을 어긴다.
- 사용자 영향: **없다**(테스트 데이터 표). 있는 것은 게이트③ 해금.
- ★**안전선 = 삭제행 0**: `git diff --numstat` → **`25  0`**. 기존 156행 무변경 = 픽스처가 재생성되지 않았다는 뜻이다(삭제행이 있었으면 «다른 사건»이라 멈췄을 자리).
- ★**추가 25행 = 이 PR 이 만든 25개 `.class` 와 집합이 «정확히» 같다**(파일명 대조 · 남의 픽스처 혼입 0).
- ★**양방향으로 쟀다**: 표에서 `65.0 indy/LambdaKinds.class` 한 행을 지우면 **red**(그 파일명을 정확히 지목) · 되돌리면 **green** ⇒ 표가 실제로 규율을 집행한다(빈 표로 통과하지 않는다).
- ★**base 당김의 원장 충돌 2건은 «합집합»으로 풀었다** — `REPORT.md`·`STATE.md` 최상단 삽입 충돌. 한쪽 통째 채택 0 · 줄 단위 양방향 보존 증명(양측 고유줄 결손 **0** · 결과에만 있는 줄 **0**). ★코드 충돌은 **0**이었고, 같은 파일(`classfile/src/validation.rs`)을 다투던 #62 의 기여는 자동 병합 뒤에도 **전건 잔존**(loadable 집합 · 서술자 팔 «둘 다» 살아 있다).
- 검증: `cargo test --test test_fixture_pins` **3 passed / 0 failed** · `cargo test --all` **581 / 0 / 1**(#62 착지분 +3) · DoD **7명령 전건 rc=0**.
- ★후속: 「착지한 규율이 진행 중 PR 을 소급으로 빚지게 하는데 아무도 말해 주지 않는다」 — `docs/worklog/2026-09-17-fixture-version-table-backfill.json`.

## [2026-09-17] 「전건 single-defect」는 «측정»이 아니었다 — 판정식을 `given` 에서 파생시킨다 (rustjava-adopt-class-format-mutation-audit-p0-fix)
- 무엇을: 게이트② **request-changes** 승계(PR #63 · 핀 `377d58b1`). ★**검수자 지적이 옳았다** — 고친 것은 감사 스크립트 **1파일**이고 제품 Rust 는 **0줄**이다.
- ★★**급소는 한 줄이다**: `repaired` 를 **정본 인자로 다시 짓고** 있었다 ⇒ 픽스처에 둘째 결함이 무엇이 들어오든 `repaired` 와 `canonical` 이 **똑같이 버려서** 항상 같았다.
  판정식이 「single-defect 인가」를 묻는데 **입력이 이미 single-defect 로 만들어져 있었다**(순환). ★18건 중 **14건**이 그 형태였다.
- ★**처방은 «발명»이 아니라 «옮겨오기»다** — `indy` 근접실패 4건이 이미 옳은 형태(`dict(given, **{축: 정본})`)였다.
  그것을 `add()` 한 곳으로 모아 **4족 전건**이 지나게 했고, `given` 은 `inspect.signature(...).bind(*spec)` 로 **이름에 묶는다**
  ⇒ 생성기에 인자가 하나 늘어도 `repaired` 가 **자동으로 물려받는다**(종전엔 튜플 인덱스라 조용히 빠졌다).
- ★★**비대칭이 이 검사의 전부다**: `repaired` 는 `given` 에서 파생하고 `canonical` 은 **파생하지 않는다**.
  둘 다 파생하면 둘째 결함이 양쪽에 실려 다시 상쇄된다. ⇒ 그 비대칭을 지키는 가드로 **`AUDIT SPEC STALE`(rc=2)** 을 넣었다 —
  스펙이 «정본을 모르는 인자»를 설정하면 **판정하지 않고 멈춘다**(추측하지 않는다).
- ★★**족마다 개악을 놓았다**(한 족의 red 를 전체로 일반화하지 않는다 — 그 일반화가 이번 결함을 살렸다):

  | 개악(둘째 결함) | 종전 | ★이 회차 |
  |---|---|---|
  | **M-A** `ldc` · `LdcDynamicOldMajor` 에 중복 BSM | rc=0 통과 | ★**rc=1 MULTI-DEFECT**(13B) |
  | **M-B** `cp` · 생성기에 `major` 추가 + `UnreferencedTag13` major 45 | rc=0 통과 | ★**rc=2 AUDIT SPEC STALE**(정본 미상 ⇒ 판정 거부) |
  | **M-B2** 위 + 감사기에 `major` 정본을 알려 줌 | — | ★**rc=1 MULTI-DEFECT**(1B) |
  | **M-C** `indy` 근접실패 · `NotInvokeStaticFactory` 에 잘못된 메서드명 | rc=1 | rc=1 (★**일하던 족은 그대로 일한다**) |
  | **M-D** `indy` LINKED · `MakeConcatWrongDescriptor` 에 정적 인자 | rc=0 통과 | ★**rc=1 MULTI-DEFECT**(4B) |

- ★★**잃은 것을 숨기지 않는다 — 그리고 «수가 내려가지 않았다»는 사실도 그대로 적는다.**
  ⒜커밋된 픽스처 18건은 **여전히 전건 single-defect**(rc=0)다. ★**내려간 것은 «통과 수»가 아니라 «통과할 수 있는 입력의 집합»**이다 —
  구조적으로 실패 불가이던 사례가 **14 → 0**. ⒝대신 판정이 **`CANON` 표가 옳다는 것에 의존**하게 됐다: 표가 틀리면 종전엔 조용했을 자리가 이제 **거짓 MULTI-DEFECT** 로 운다.
  ⒞생성기가 자라면 `AUDIT SPEC STALE` 이 **게이트를 막는다**(고의다 — 「모르면 멈춘다」). ⒟시험 시간이 늘었다 — ★**추정이 아니라 실측이다**: 개악 5종(스크래치 사본 5벌 · 각 벌에서 픽스처 재생성 + 감사 1회) = **9.57초**(`/usr/bin/time -p`) · 감사 1회 단독 **0.31초**. 전/후 대조까지 돌리면 그 두 배 남짓이다.
- ★**상시 CI 검사는 «이 회차에서 만들지 않았다»** — 공허한 검사를 DoD 에 박는 것이 가장 비싼 결말이라, 판정식을 먼저 조이고 검사 신설은 다음 회차로 둔다.
- 검증: 정상 형상 **검사 18 · 무결함 5 · rc=0** · 개악 5종 전건 red · `cargo test --all` **Rust 무접촉이라 불변**.

## [2026-09-17] 픽스처가 «정확히 한 곳만» 망가졌는가 — 감사의 «나머지 반쪽» (rustjava-adopt-class-format-mutation-audit-p0)
- 무엇을: 채택 제안 `2026-09-16-class-format-mutation-audit#p0`. ★**제품 코드 0줄**(감사 스크립트 1개).
- 왜: 종전 감사는 「테스트가 자기 가지의 개악에 죽는가」를 물었다. 이 회차는 ★**「픽스처가 정확히 한 곳만 망가졌는가」**를 묻는다.
  ★**두 곳이 망가진 픽스처는 한 곳을 고쳐도 테스트가 계속 green** 이고, 나머지 하나가 «더는 덮이지 않는다»는 사실이 **조용히** 묻힌다.
- ★★**판정식을 «로드»가 아니라 «바이트 동일»로 잡았다** — 제안은 「수리 후 로드되는가」를 제시했으나
  생성기가 결함을 **인자로 받으므로** 더 강한 검사가 더 싸게 된다:
  `generator(결함) == 커밋본`(생성기가 여전히 그 파일을 설명한다) **그리고** `generator(수리) == generator(표준)`(그 밖에 차이가 없다).
  ★**강한 이유**: 우리 로더가 «마침» 신경 쓰지 않는 둘째 결함은 로드 검사를 통과하지만 이 검사는 통과하지 못한다. ★**싼 이유**: JVM 이 필요 없다.
- ★★**[교정 2026-09-17 · 게이트② request-changes · `-fix` 회차] 이 줄의 «전건»은 «측정»이 아니었다.**
  당시 판정식은 18건 중 ★**14건에서 `repaired` 를 정본 인자로 «다시 지어»** 둘째 결함을 버렸다 ⇒ 구조적으로 MULTI-DEFECT 를 낼 수 없었다.
  ★**실측된 것은 4/18**(`indy` 근접실패)이고 나머지 14는 공허하게 통과했다. 정정된 결과와 근거는 **맨 위 `-fix` 항목**에 있다.
  (족 구성은 그대로다: `indy` 근접실패 6 · `ldc` 9 · `cp` 3 / 무결함 5 = `MakeConcat`·`LdcMethodHandle`·`LdcMethodType`·`LdcDynamic`·`Ldc2WDynamic`)
- ★**덮지 않는 것을 스크립트 머리에 «적었다»**(조용히 건너뛰지 않았다): javac 산출물(결함 0) · 적법하나 미구현(결함 0) ·
  ★**테스트 «안»에서 바이트 패치로 만드는 픽스처**(디스크에 없어 읽을 수 없다 — 그쪽 근거는 각 주석이고 **더 약하다**).
- ★**개악 2종 전건 red**: **M1** 둘째 결함 심기 → `MULTI-DEFECT` **rc=1** · **M2** 커밋본 1바이트 반전 → `GENERATOR DRIFT` **rc=2**.
  ★**M2 가 첫 등식을 지킨다** — 생성기가 커밋본을 설명하지 못하면 위 판정은 전부 공허해지고, 스크립트는 통과 대신 그렇게 «말한다».
- 검증: `cargo test --all` **576 passed / 0 failed / 1 ignored**(★Rust 무접촉이라 불변) · worklog 46/0 · parity 대칭차 0.
- ★후속: 이 감사를 ★**상시 검사로 올릴지 «결정»하라**(S) — 안 돌리는 감사는 썩는다 ↔ DoD 8번째 명령 + parity 갱신이 대가다.

## [2026-09-17] 다른 클래스 수준 속성에도 개수 규칙이 필요한가 — ★**다섯에 «예». 그리고 제안의 값 전제가 거짓이었다** (rustjava-adopt-reject-duplicate-bootstrap-methods-p0)
- 무엇을: 채택 제안 `2026-09-16-reject-duplicate-bootstrap-methods#p0`(제목이 **「Decide whether …」** = 이 회차가 지는 것은 **판정**). ★**제품 동작 변경 있음** — 아래.
- ★**제안의 기준만으로는 «아니오»가 나온다**: 기존 검사 주석이 적은 기준은 「중복이 **하류에서 임의의 선택을 관측 가능하게 만드는가**」이고, 소비자 전수 실측 결과 그런 속성은 ★**`BootstrapMethods` 하나뿐**이다(`validation.rs` + `string_concat.rs:110 resolve_bootstrap_methods` 의 `find_map`). 나머지 6종은 `attribute.rs` 밖 소비자 **0**.
- ★★**판정을 바꾼 것은 «진짜 JVM»이다** — OpenJDK **26.0.1** 에 한 속성씩 물었다(★런처 메시지는 원인을 가리므로 `Class.forName` 으로 진단문을 받았다):
  `SourceFile`·`InnerClasses`·`SourceDebugExtension`·`BootstrapMethods`(52) · `NestHost`·`NestMembers`(55) → ★**전건 `ClassFormatError: Multiple … attributes`**.
  ⇒ ★**제안의 `tradeoff` 「Rejecting more files that load today」는 그 다섯에 대해 «거짓»이다** — 그 파일들은 **오늘도 진짜 JVM 에서 로드되지 않는다.** 받아들이던 쪽이 **우리**였다.
- ★★**그래서 «표»이고 «버전 게이트»다 — 두 통제군이 그 이유다**:
  ⒜★`NestHost` **major 52** 에서는 ★**로드된다**(그 버전엔 정의되지 않아 무시 · JVMS 4.7.1) ⇒ 게이트 없이 세면 **모든 JVM 이 받는 파일을 거부**한다.
  ⒝★`Synthetic` 은 JVMS 4.7.8 이 at-most-one 이라는데 ★**HotSpot 은 둘을 받는다** ⇒ ★**빠뜨린 것이 아니라 «근거로 뺐다»**(넣으면 맞추려는 JVM 보다 엄격해진다).
- ★**반대 방향도 의도적이다** — 필드·메서드·Code 소속 속성은 클래스 속성표에 나타나도 **세지 않는다**(정의되지 않은 자리의 속성 = 무시).
- ★★**양방향 개악 4종 전건 red**: **M1** 버전 게이트 제거 → 통제군 `DuplicateNestHostOldMajor` red · **M2** `Synthetic` 추가 → 통제군 red · **M3** `SourceFile` 한 칸 제거 → 그 픽스처만 red · **M4** 호출부를 종전으로 되돌림 → 다섯 red · 복원 **17/0 green**. ★**M1·M2 가 요지다** — 통제군이 없으면 「전부 세면 된다」가 통과한다.
- ★**대가**: ⒜**제품 동작 변경** — 그 다섯을 둘씩 가진 클래스가 **로드되지 않는다**(전부 OpenJDK 도 거부하지만 변경은 변경이다) ⒝**표는 «적어 둔 목록»** 이라 속성이 늘어도 행을 더하기 전엔 안 덮이고 **울어 주는 것이 없다** ⒞`Synthetic` 배제는 **JVM 하나의 행동**에 걸려 있다 ⒟★**`attribute.rs` 를 만졌다**(제안 `target` 은 `validation.rs` 뿐) — `SourceDebugExtension` 파싱 팔 **한 줄**로 기존 **dead variant** 를 살린 것이고, 없으면 중복이 `Unknown` 에 섞여 **구별 불가**다. 최소였지만 **명시 파일 밖**이라 신고한다.
- 검증: `cargo test --all` **578 → 579 / 0 failed / 1 ignored** · `test_fixture_pins` **3/0** · 픽스처 재생성 **멱등** · 버전 표 **+7행**(AGENTS.md 의무) · DoD 7명령 rc=0.

## [2026-09-17] ldc 픽스처를 ASM 으로 재생성하자 — ★**기각**(rustjava-adopt-ldc-tags-real-world-generator-survey-p0)
- 무엇을: 운영자가 tower 에서 채택한 제안 `2026-09-16-ldc-tags-real-world-generator-survey#p0` 의 처분. ★**제품 코드 0줄** — 산출물은 «판정과 그 근거»다.
- ★**기각 사유 ⑴ 제안이 사려는 것이 이미 있다 — 그것도 더 센 오라클로**: ★**OpenJDK 26.0.1 이 양성 픽스처 4건을 «실행»한다**(`LdcMethodHandle`·`LdcMethodType`·`LdcDynamic`·`Ldc2WDynamic` **전건 rc=0**) ⇒ 진짜 JVM 의 **검증기**가 우리 손조립 바이트를 받아들인다. 대조군(위법 5건)은 **전건 rc=1**. 그리고 「ASM 이 이 태그를 내는가」는 **선행 조사 회차가 이미 동적으로 실증**했다.
- ★**⑵ 손의 흔적은 사라지지 않고 «옮겨간다»**(제안이 적지 않은 축): ASM 경로는 `visitLdcInsn(new Handle(…))` 을 부르는 **드라이버 15줄**이 있어야 성립한다 ⇒ **모양은 여전히 우리가 고르고**, 바뀌는 것은 «인코딩의 출처» 하나다.
- ★**⑶ 값은 제안 자신이 셋을 적었고 지금도 참이다**: JDK 가 PATH 에 없다(★정정 — **설치는 돼 있다**: `/opt/homebrew/opt/openjdk` 26.0.1 · 「부재」가 아니라 「기본 경로로 안 닿는다」) · asm.jar 를 네트워크로 들여와야 한다(repo 에 **0건**) · 음성 픽스처는 ASM 이 못 만들어 ★**생성기가 «두 기구»가 된다**.
  ★**과장하지 않는다** — CI 는 오늘 픽스처를 **재생성하지 않는다**(`make_*_fixtures|verify-javac` 참조 **0건** · `setup-java` 도 **0건**). 진짜 비용은 ★**「어디서나 한 줄로 되는 재생성」이 「네트워크+JDK+핀된 jar」 의식이 되는 것**이다.
- ★★**⑷ 상시 검사와 정면 충돌한다**: `audit-fixture-single-defect.py` 의 **첫 등식** `generator(**given)==committed` 는 `no defect` 스킵 «전»에 돌므로, ASM 바이트면 그 셋이 **`GENERATOR DRIFT` rc=2** 가 된다. ★**단서**: 그 감사기는 **아직 미착지**(PR #63 진행 중 · `bin/landed` **UNLANDED**)라 「오늘의 사실」이 아니라 **「착지하면 즉시 충돌하는 축」**이다.
- ★**⑸ 버전도 손으로 고를 수밖에 없다**: 표의 `55.0 LdcDynamic` 은 우연이 아니라 JVMS 4.4(태그 17 ⇒ major ≥ 55)이고, ASM 은 **드라이버가 정한 값**을 쓴다. `tests/test_class_format.rs` 가 그 major 를 50/54 로 낮춰 버전 규칙을 증명하므로 하중이 **둘**이다.
- ★★**잃는 것 — 기각은 공짜가 아니다**: 생성기 머리의 `These are **synthetic**` 단서가 양성 픽스처에도 남고, ★**우리 인코더의 체계적 편향을 우리 파서«와» OpenJDK 가 «둘 다» 관대하게 넘기는 경우**는 ASM 이라면 드러났을 것이다(JVM 검증기는 **센 필터이지 증명이 아니다**). 어느 쪽이든 «현실성»의 상시 보장은 없다.
- ★후속: 더 싼 대안을 카드로 남겼다 — **재생성 대신 양성 픽스처를 진짜 JVM 에 올려 현실성 검사로 삼는다**(선례 `verify-javac-fixtures.sh`) · `docs/worklog/2026-09-17-ldc-asm-regeneration-declined.json`.

## [2026-09-17] kotlinc·scalac 를 «타깃 형상»으로 몰았다 — ★**0. 그러나 «다른 0»** (rustjava-adopt-ldc-tags-real-world-generator-survey-p1)
- 무엇을: 채택 제안 `2026-09-16-ldc-tags-real-world-generator-survey#p1`. 조사 회차가 자기 표에 **「못 쟀다」**로 적어 둔 칸을 채웠다. ★**제품 Rust 0줄**(조사 입력 2 + 스캐너 docstring).
- 왜: 종전 축은 **컴파일러의 stdlib**(= 그 컴파일러 산출물)이었다. ★**stdlib 은 호환성을 위해 컴파일되지 백엔드를 훑으려고 컴파일되지 않는다** — 그래서 「이 코퍼스에서 0」과 「이 기능들에서 0」이 다르다.
- ★**결과**: kotlinc **2.4.20** → 7클래스·23 ldc 자리 · ★**피연산자 15/16/17 = 0** / scalac **3.9.0** → 7·18 · ★**0**.
  ★★**풀 수치가 이 0 을 읽을 값으로 만든다** — Kotlin `MethodHandle 7 · MethodType 6` · Scala `11 · 6` **(0 이 아니다)** ⇒ ★**indy 경로가 실제로 돌았고 상수도 만들어졌는데 `ldc` 자리에 «닿지 않는다»**. 「안 썼다」가 아니라 「썼는데 안 온다」다.
  ★**태그 17(Dynamic)은 풀에도 0** ⇒ ★**이 형상들에서는 두 컴파일러 모두 condy 를 내지 않는다**(둘 중 더 센 진술 · ★한정은 나머지 수와 «같다» — **컴파일러당 프로그램 1개**라 기능을 한정하지 언어를 한정하지 않는다).
- ★**타깃 형상**: 람다 · 언바운드/바운드 메서드 참조 · SAM 변환(네이티브+Java 둘 다) · enum 주어 · 문자열 연결 · lazy · (Kotlin) reified · (Scala) eta 확장·inline def·구조적 타입. ★**플래그도 «더 많이» indy 로 보내는 쪽으로 골랐다**(`-Xlambdas=indy`·`-Xsam-conversions=indy`·`-Xstring-concat=indy-with-constants` · `scalac -release 21`).
- ★**양방향**: 같은 스캐너·같은 세션에서 **양성 대조군** `test-data/ldc` → `{MethodHandle 1, MethodType 2, Dynamic 7}` ⇒ ★**이 0 은 「스캐너가 못 본다」가 아니다.** 오차 막대(불가능 피연산자) 두 실행 **0.00%**.
- ★**제안의 값 전제를 다시 재서 «부분적으로 거짓»임을 찾았다**: 「JVM 툴체인이 의도적으로 없는 맥」이라 했으나 ★**openjdk 26 은 2026-07-22 부터 설치돼 있었다**(`INSTALL_RECEIPT` 의 **`time`** 필드 = epoch `1784711495` · KST 18:11:35 — ★**`source_modified_time`(=formula 최종 수정)이 아니다**)(조사 회차 **09-16** 보다 **56일**, 약 2개월 앞선다) ⇒ 한계 설치는 **formula 하나씩**이었다 — ★**단 «하나씩»이 공짜는 아니다**: 그 의존 해결이 위 **openjdk 판본 승격을 동반**했다(대가 절).
- ★★**대가**: **머신 상태가 바뀌었다** — `kotlin 2.4.20`·`scala 3.9.0`(+`scala-cli`) 설치 · ★**그리고 «신고에서 빠져 있던» 한 가지 — `openjdk` 기본 링크가 «의존성으로» 26.0.1 → 26.0.2.1 로 승격됐다**(`/opt/homebrew/opt/openjdk` · 같은 순간 `time 2026-09-17T12:48:50Z` · `installed_on_request false`). ★**되돌리기**: `brew uninstall kotlin scala` 는 **이 링크를 되돌리지 않는다**. 그리고 ★**Homebrew 7 에는 판본을 되돌릴 명령이 없다**(`brew switch` 제거). 실효 처방은 ★**26.0.1 keg 가 그대로 있으므로 소비자가 경로를 핀하는 것**이다 — `JAVA_HOME=/opt/homebrew/Cellar/openjdk/26.0.1`(실행 확인: `java -version` → `26.0.1`). ★**범위**: openjdk 는 **keg-only** 라 `PATH` 의 `java` 는 `/usr/bin/java` 로 **안 바뀐다** — 영향은 `/opt/homebrew/opt/openjdk` 를 **명시적으로** 쓰는 소비자(이 회차가 그랬다)뿐이다. 레인 약 28개가 공유하는 맥이고, **가산적·가역**(`brew uninstall kotlin scala`)이라 **재측정 가능성을 위해 일부러 남겼다**. ★컴파일러당 프로그램 «하나»라 **기능**을 한정할 뿐 **언어**를 한정하지 않는다. ★**CI 에서 못 돈다**(`setup-java` 0건) — 사람이 돌리는 검사다.
- ★**답은 제안이 예상한 그 0 이다** — 산 것은 **오차 막대**뿐이고, 그것이 이 회차 값의 정직한 회계다.

## [2026-09-17] base 를 당겼다 — ★**막고 있던 코드 충돌은 «이미 없었다»**(rustjava-adopt-link-stringconcatfactory-p1-fix2)
- 무엇을: `origin/main` 당김(뒤처짐 **9**) + 그 당김이 만든 `test-data/class-file-versions.txt` **3행**. ★제품 코드 **0줄** · 픽스처 바이트 **불변**.
- ★★**전제가 반증됐다**: 이 회차는 「`make_indy_fixtures.py` 4구역 코드 충돌」을 풀라고 발권됐는데, 지금 당기면 그 파일은 **충돌하지 않는다**. `-p1-fix` 회차가 **14:10 에 `0f06b93f` 로 이미 합집합 해소**했고 게이트②가 **15:43 에 그 head 를 approve** 했다 — 발권 근거였던 12:12 blocked 회신이 그 사이 **낡았다**.
- ★**재발도 불가능하다**(그냥 「지금은 없다」가 아니다): 뒤진 9커밋 중 `make_indy_fixtures.py` 를 만진 것이 **0건**이다.
- ★**그래도 합집합이 «진짜»인지 다시 쟀다** — 합집합의 전형적 실패는 「한쪽 의도가 조용히 빠지는 것」이라서다.
  ⒜생성기를 **실제로 돌려** 10개 픽스처가 전부 **바이트 불변**(양쪽 가족 — theirs `MakeConcat*` 3 · ours `RecipeWants*` 3 — 을 **한 생성기**가 낸다).
  ⒝★**양방향 개악**: ours 산출물 3개만 치우면 **ours 만 red**(theirs 2건 green) · theirs 3개만 치우면 **theirs 2건 red**(ours green). ⇒ 「선택」이 아니라 합집합이다.
- ★**남은 것은 충돌이 아니라 «부채»였다**: base 를 당기면 #57 의 버전 표가 들어오고 이 PR 의 픽스처 3개가 미등재라 `test_fixture_pins` 가 red 가 된다(게이트② 검수가 「`-merge` 회차가 표 3행을 함께 진다」고 이미 지목한 그것). 생성기로 채웠다 — ★**`3  0`(삭제 0)**.
- 원장 충돌 2건(`REPORT.md`·`STATE.md`)은 **합집합·시간순**. ★`STATE.md` 「진행중」 한 줄은 3-way 에서 **ours 가 이겼다**(base == theirs ⇒ 정상) — 결손이 아니다.
- 검증: `test_class_format` **16/0** · `test_fixture_pins` **3/0** · `cargo test --all` **578 / 0 / 1** · DoD **7명령 전건 rc=0**.

## [2026-09-17] 부트스트랩 정적 인자는 «적재 가능 상수»여야 한다 — 경계에서 «종류»로 (rustjava-adopt-bound-bootstrap-static-arguments-p0)
- 무엇을: 채택 제안 `2026-09-16-bound-bootstrap-static-arguments#p0`. ★**제품 동작이 바뀐다** — 인자가 적재 불가 상수를 가리키는 클래스 파일이 **`ClassFormatError`** 로 거부된다.
- 왜: JVMS 4.7.23 이 요구하는 것은 «인덱스가 어딘가에 닿는다»가 아니라 ★**「적재 가능 상수」**다(Integer·Float·Long·Double·Class·String·MethodHandle·MethodType·Dynamic).
  직전 회차는 «경계»까지만 하고 «종류»를 남겨 두었고, 그 사실을 **함수 주석이 스스로 적어 두었다**.
- ★★**제안이 스스로 적은 위험을 «먼저» 쟀다** — 「틀리면 람다 클래스가 전부 corrupt 가 된다」.
  그 위험의 실체는 `attribute.rs` 의 설계(인자를 **해석하지 않는다**)이고, ★**태그 검사는 «어느 변형인가»만 묻고 «안을 읽지» 않는다** ⇒ 그 설계를 어기지 않는다.
  ★**말이 아니라 수로**: 커밋된 클래스 전건 파싱이 **전 144 / 실패 12 → 후 144 / 실패 12** 로 ★**동일**(새로 거부되는 파일 **0** · 람다 포함).
- ★**OpenJDK 26.0.1 실측이 근거다**: 인자를 Utf8 로 돌리자 ★**`ClassFormatError: argument_index 4 has bad constant type`** ⇒ 그 파일은 «미지원»이 아니라 **«파손»**이다.
- ★**안 하면 무엇이 나쁜가**: 파일이 통과한 뒤 **나중에·틀린 낱말로** 실패한다 — 링커가 `None` 을 받아 링크를 포기하고
  검증기가 **`UnsupportedOperationException`**(= 「우리가 지원하지 않는다」)을 낸다. ★**이 저장소가 반복해 가른 그 두 낱말**이다.
- ★**개악 2종 전건 red**: M1 «존재만»으로 되돌리기 · ★**M2 적재 가능 집합에 Utf8 한 칸 추가** — M2 가 요지다(단언이 «경계»에 걸려 있음을 보인다).
- 검증: `cargo test --all` **575 passed / 0 failed / 1 ignored** · DoD 7명령 rc=0 · ★**새 픽스처 0**(기존 파일 바이트 패치 · 길이 필드 불변).
- ★**여기서 더 갈 수 없는 자리도 적는다**: `Dynamic` 인자의 서술자가 필드 서술자인지, `MethodHandle` 인자가 실제 멤버로 해석되는지는 **payload 가 필요**해 이 술어의 밖이다(설계이지 누락이 아니다).
- ★후속: `ClassFormatError` 에 **사유를 실어라**(M) — OpenJDK 는 인덱스와 이유를 말한다. ★p2-fix 회차가 낸 같은 제안과 **묶어서** 하는 편이 낫다.
## [2026-09-17] 포획 «순서»를 값으로 잠그고, 클래스 파일이 «프로세스를 죽이는» 자리를 닫는다 (rustjava-adopt-link-stringconcatfactory-p2-fix)
- 무엇을: 게이트② **request-changes** 승계(PR #61 · 핀 `87ef6a70`). ★검수자 지적 **F1·F2 둘 다 옳았고 둘 다 받았다.**
- ★**F1 — 포획 «순서»가 전 스위트에 무관측이었다**: `LambdaBody::call` 의 읽기 순서를 뒤집어도(검수자 RM4) **576 green**.
  거부가 아니라 ★**조용히 틀린 답**이다. 근인은 단언 방식이 아니라 **픽스처**다 — 전건이 포획 **1개 이하**라 «순서»라는 축이 존재하지 않았다.
  ⇒ 포획 2개 람다 **둘**을 넣었다: `(String,int)` → `a:7`(순서가 **글자**에 보인다) · `(int,int)` → `120`(★**값에만** 보인다 — 어떤 타입 검사로도 못 잡는 축).
  ★**RM4 가 이제 죽는다**: `a:7→7:a` · `120→2001`. ★`(a, b) -> a + b` 는 javac 이 내는 «가장 평범한» 람다다.
- ★★**F2 — 적법한 클래스 파일이 «호스트 프로세스»를 죽였다**(`jvm/src/type.rs:74` panic · 게스트 예외가 아니다).
  ★**고친 자리를 «골랐고 왜인지 적는다»**: 검수자 제안(`lambda.rs` 2줄)만 쓰면 진단이 `UnsupportedOperationException` 이 되는데,
  ★**OpenJDK 26 은 같은 파일을 `ClassFormatError` 로 거부한다**(`Method "run" … has illegal signature "I"`) — 그 파일은 «미지원»이 아니라 **«파손»**이다.
  JVMS 4.4.6 상 `NameAndType` 은 필드·메서드 서술자 **둘 다** 적법해야 하고(Fieldref·Methodref 가 같은 항목을 공유한다),
  ★**어느 쪽인지는 «참조하는 태그»가 정한다(4.4.10)** ⇒ 일반 `NameAndType` 팔은 **그대로 두고**
  `InvokeDynamic`=메서드 서술자 · `Dynamic`=필드 서술자를 **그 팔에서** 요구하게 했다(`Methodref` 는 이미 그렇게 한다).
- ★**조이기 «비용»을 먼저 쟀다**(티켓 요구): 커밋된 클래스 **175개** · indy/condy 참조 **44건** 중 새로 위법이 되는 것은 ★**이 회차가 만든 픽스처 1건**뿐.
- ★**`lower()` 의 `try_parse` 는 «둘째 층»이고, 독립 관측이 «안 된다»는 것을 숨기지 않는다** — 검증을 통과하면서 `try_parse` 가 실패하는 입력을 만들지 못했다.
  남긴 이유는 측정이 아니라 **비용의 비대칭**이다: 바깥 층이 틀리면 게스트 예외, 안쪽이 없으면 **프로세스 사망**.
- ★**덤**: 같은 panic 이 `origin/main` 의 string concat 경로(`extract_invoke_params`)에도 있었는데 **같은 규칙에 함께 막힌다**(검수자가 「별 티켓」이라 한 자리).
  넓힌 것이 아니라 **규칙을 옳은 자리에 둔 결과**다.
- 검증: 개악 **2종 전건 red**(RM4 · F2 규칙 되돌리기) · `cargo test --all` **578 passed / 0 failed / 1 ignored** ·
  픽스처 재생성 **멱등**(★형제 #59 의 생성기와 **한 파일로 합친 뒤**에도 기존 픽스처 바이트 불변).
- ★후속: `ClassFormatError` 에 **사유를 싣자**(M) — 이 회차가 세운 「미지원 ↔ 파손」 구분이 정작 파손 쪽에서 「Invalid class file」 한 줄로 뭉개진다.

## [2026-09-17] `LambdaMetafactory.metafactory` 를 링크했다 — ★**람다와 메서드 참조가 «돈다»** (rustjava-adopt-link-stringconcatfactory-p2)
- 무엇을: 채택 제안 `2026-09-16-link-stringconcatfactory#p2`. ★**제품 동작이 바뀐다** — 람다·메서드 참조를 담은 클래스가
  **적재 거부**에서 **실행**으로 바뀐다. `jvm/` 은 **무접촉**, `java.lang.invoke` 는 **한 줄도 추가하지 않았다**.
- ★★**제안의 비용 추정이 틀렸다 — 그것이 이 회차의 요지다.** 제안은 「string concat 의 지름길을 쓸 수 없다 ·
  `java.lang.invoke` 가 **불가피**해진다 · 노력도 **L**」이라 했다. ★**호출 사이트가 «의미»하는 것은 핸들 사슬이 아니라 «객체»다.**
  그리고 그 객체를 만들 두 축이 ★**이미 있었다**: `MethodBody::Rust(JvmCallback)`(본문이 러스트인 메서드) ·
  `Jvm::register_class`(런타임에 만든 정의를 이름으로 등재). ⇒ 팩토리가 스핀할 클래스를 **직접** 만든다.
- ★**설계에서 «떨어져 나온» 것 둘**(만든 게 아니다): ⒜**GC 가 이미 추적한다** — `find_all_fields` 가
  `ClassDefinition::fields` 를 걷으므로 포획값을 «필드»로 두면 그대로 살아 있다(그래서 필드다)
  ⒝**등재가 멱등** — `register_class_internal` 이 `.or_insert` 라 두 스레드가 같은 콜사이트에 닿아도 먼저 것이 이긴다(락 0).
- ★★**경계 = «어댑터»**: 실 팩토리는 박싱·언박싱·확대를 끼워 넣는다. 여기엔 그 축이 없으므로 **통과**(동일 프리미티브 ·
  양쪽 레퍼런스)가 아니면 ★**링크하지 않는다**. 판정이 서술자만으로 되므로 **lowering 시점**에 끝난다 ⇒ 클래스는
  로드되거나 안 되거나이고 ★**호출 «도중»에 실패하는 경로가 없다.** 그 경계는 `LambdaBoxing.class` 가 **잠근다**
  (OpenJDK 26.0.1 은 3을 찍고 우리는 거부한다).
- ★★**관측 가능성이 어려웠던 자리 셋 — 전부 «처음엔 안 죽었다»**:
  ⑴**void 버림**: 지워도 전 스위트 green 이었다 — 남은 값은 오퍼랜드 스택 «아래»에 쌓이고 정상 바이트코드가 다시 꺼내지 않는다.
  ⇒ 인터프리터 «밖»에서만 보인다: `Thread.run()` 이 `Runnable.run()V` 를 러스트에서 부르고 `From<JavaValue> for ()` 로 변환한다
  (Void 가 아니면 **panic**). 픽스처를 그 경로로 통과시키자 개악이 `Expected void, got Int(7)` 로 죽었다.
  ⑵**REF_invokeSpecial**: javac 은 Java 11(nestmates)부터 그 종류를 «내지 않는다» — 같은 소스 실측으로
  `--release 8` 은 kind 7 · `--release 21` 은 kind 5. ★그렇게 오래된 클래스 파일이 이 런타임의 «대상»이므로 가지를 남기고
  픽스처를 **8로 컴파일**했다. `test_fixture_pins.rs` 를 **픽스처별 핀**으로 바꿨다 — «면제»로 뺐으면 그 픽스처의 요지가 무검증이 된다.
  ⑶**정적 인자 «개수·종류»**: 신원 4축엔 근접실패가 있었는데 이 둘엔 **없었다** ⇒ 손조립 2종 추가.
- ★★**개악 14종 전건 red**(정상 576 green): lowering 미호출 · 신원 4축 각각 · 정적인자 2축 · 통과검사 · 포획 저장 ·
  수신자 처리 · void 버림 · REF_invokeSpecial · REF_newInvokeSpecial · 릴리스 8 핀.
- 검증: `cargo test --all` **573 → 576 passed / 0 failed / 1 ignored**(기준선 `origin/main` 워크트리 실측) ·
  `LambdaKinds` 출력 **10줄이 OpenJDK 26.0.1 과 글자대로 일치** · DoD 7명령 rc=0 · 픽스처 재생성 **멱등**.
- ★후속: **박싱 어댑터**(M · `LambdaBoxing` 이 이미 그 자리를 잠그고 있다) · **람다 클래스의 리플렉션 가시성 결정**(S) —
  `docs/worklog/2026-09-17-link-lambdametafactory.json`.
## [2026-09-17] 레시피가 콜사이트와 어긋날 때 — ★**「싸고 옳다」는 두 겹으로 거짓이었다** (rustjava-adopt-link-stringconcatfactory-p1)
- 무엇을: 채택 제안 `2026-09-16-link-stringconcatfactory#p1`(「진단의 자리를 정하라」). ★**제품 동작이 바뀐다** —
  레시피와 콜사이트의 «합의»를 **변환 전에 한 번** 재고, 없던 `java/lang/BootstrapMethodError` 를 런타임에 추가했다.
- 왜: 제안은 「현 런타임 검사는 싸고 옳다, 문제는 «자리»뿐」이라 했다. ★**둘 다 틀렸다.**
  ⑴★**그 가지는 애초에 던지지 못했다** — `java/lang/BootstrapMethodError` 가 이 런타임에 **없어서**
  `jvm.rs:948` 의 unwrap 에서 **NoClassDefFoundError 로 패닉**했다. 이 형상의 픽스처가 **하나도 없어** 아무도 밟은 적이 없다.
  ⑵★**검사가 «부족분»만 봤다** — 레시피가 콜사이트보다 **짧으면** 남는 인자를 조용히 버리고 **틀린 문자열**(`a`)을 돌려주고 rc=0 이었다.
  ⇒ 부등호를 **상등**으로 바꾸고 인자·상수 두 축을 함께 잰다.
- 사용자 영향: 손상·수제 클래스 파일이 **패닉이나 조용한 오답 대신** `BootstrapMethodError` 를 받는다. 정상 javac 산출물은 **무영향**.
- ★★**제안의 처방(`classfile/validation.rs` 로 옮겨 `ClassFormatError`)은 기각한다 — 추측이 아니라 실측이다.**
  OpenJDK 26.0.1 에 세 픽스처를 **직접 돌렸다**: 전건 `BootstrapMethodError`(원인 `StringConcatException`) · 프레임은 `linkCallSite` =
  ★**링크 시점**이고 ★**`ClassFormatError` 가 아니다**. 파일은 파싱되고, 부트스트랩 정적 인자의 «의미»는 클래스파일 형식의 소관이 아니다.
  ⇒ 제안이 스스로 적은 비용(「검증 단계에서 부트스트랩 인자를 걷는 것 = `attribute.rs` 가 일부러 피한 해결」)도 함께 면했다.
- ★**왜 «변환 전»인가**: 여기엔 `CallSite` 가 없어 링크가 첫 실행에 접힌다 ⇒ 그 순서에 가장 가까운 것이 「무엇도 변환하기 전에 잰다」이다.
  먼저 변환하면 `String.valueOf` 를 통해 **사용자 `toString()` 이 돌고**, 그 예외가 이 진단을 덮는다.
- ★★**개악 4종 전건 red**(정상 574 green): M1 합의 검사 제거 · M2 `!=`→`>`(부족분만) · M3 상수 축 제거 ·
  ★**M4 `loader.rs` 에서 클래스 등록 제거 → `jvm.rs:948` 패닉이 «되살아난다»**(= 새 클래스가 하중을 진다).
- 검증: `cargo test --all` **573 → 574 passed / 0 failed / 1 ignored**(기준선은 `origin/main` 워크트리에서 실측) ·
  DoD 7명령 rc=0 · 픽스처 재생성 **멱등**(기존 4개 바이트 불변).
- ★후속: **72개** `java/…Error|Exception` 이름이 이 워크스페이스의 오류 경로에 있고, 이번 회차 전까지 그중 **1개**(이 건)가 proto 없이 있었다.
  ★**지금 baseline 이 0** 이라 잠그기 가장 싼 시점이다 — `docs/worklog/2026-09-17-string-concat-recipe-arity.json`.

## [2026-09-17] `StringConcatFactory.makeConcat` 도 링크한다 — 단 «이유는 제안이 적은 것이 아니다» (rustjava-adopt-link-stringconcatfactory-p0)
- 무엇을: 레시피 없는 진입점 `makeConcat` 을 링크한다. ★**실행기 무접촉** — 콜사이트 인자 수로 **레시피를 합성**한다.
- 왜: 채택 제안 `2026-09-16-link-stringconcatfactory#p0`.
- 사용자 영향: ★`makeConcat` 에 묶인 콜사이트가 **거부 대신 실행**된다.
- ★★**제안의 전제는 «틀렸다»**(실측 javac 26.0.1 · 같은 소스 `a + b`): `--release` **9·11·17·21·26 전부
  `makeConcatWithConstants`** 를 낸다 — ★**상수 텍스트가 «없어도»** 그렇다(레시피가 자리표시자 둘일 뿐).
  `makeConcat` 은 **비기본 내부 플래그 `-XDstringConcat=indy`** 에서만 나온다.
  ⇒ ★**「javac 이 상수 텍스트 없이 연결할 때 쓴다」·「javac 산출물이 더 많이 돈다」는 둘 다 거짓**이다(기본 산출물은 **이미 전부 링크된다**).
- ★**그래도 한 이유를 «바꿔서» 적는다**: ⑴**만들 수 있는 형상**이고(그 플래그로 직접 만들었다 — 이 리니지가 `ldc` 지원을
  정당화한 「ASM 이 실제로 낸다」와 **같은 기준**) ⑵**문서화된 공개 진입점**이며 ⑶★**실행기에 새 경로가 필요 없다**
  (`makeConcat(n)` ≡ 레시피 `\u{1}`×n 인 `makeConcatWithConstants`).
- ★**설계**: 상수 튜플 **둘**뿐이고 **레지스트리로 만들지 않았다**(제안 tradeoff 준수) ·
  ★**이름과 서술자를 «쌍»으로** 맞춰 ★**#55 의 name 축 근접 실패 픽스처가 «살아 있다»**(그 파일은 이름만 `makeConcat`) ·
  레시피 합성은 **콜사이트마다**(한 부트스트랩을 서술자가 다른 콜사이트들이 공유할 수 있다).
- ★★**개악 4종 중 «둘»이 처음엔 살아남았다** — M3(쌍 검사를 이름만으로) · M4(정적 인자 가드 제거).
  ★**직전 회차가 세운 「죽었다 ≠ 그 가지가 전부 덮였다」가 내 새 코드에 그대로 적용됐다** ⇒ 축마다 근접 실패를 더해
  **4종 전건 KILLED** 로 만들었다(`MakeConcatWrongDescriptor` · `MakeConcatWithArgument`).
- ★**픽스처가 «출력»한다**(`ab`) — ★레시피를 틀린 길이로 합성해도 **링크되고 실행된다**. 값을 버리면 그 오류가 안 보인다(M2 가 그 증거).
- 검증: `cargo test --all` **573 → 574 / 0 failed** · 픽스처 재생성 **멱등**(기존 4개 바이트 동일) · DoD 7명령 rc=0.
## [2026-09-17] 표 키의 경로 구분자를 «두 곳에서» 정규화한다 — 윈도우 CI red 를 고친다 (rustjava-adopt-indy-fixture-jdk-pin-and-slot-accounting-p0-fix)
- 무엇을: 표 키를 만드는 **두 곳**(Rust 테스트 · Python 기록기)을 **슬래시**로 맞췄다. ★**제품 코드 무접촉 · 설계 무변경.**
- 왜: 게이트② **request-changes**(PR #57 · 핀 `3061edb7`) — ★**윈도우 CI 가 red 였다**(`ci-presence` → `CI_RED` rc=1).
- 사용자 영향: 없다(시험 위생). ★바뀐 것은 **표 핀이 «모든 OS 에서» 같은 키를 쓴다**는 것이다.
- ★★**결함의 모양**: `to_string_lossy()` 가 윈도우에서 `indy\StringConcat.class` 를 만드는데 표는 `indy/…` 를 담는다 ⇒
  ★**하위 36개가 «미기록»과 «유령 기록» 양쪽에 «동시에» 걸린다.** ★**루트 114개는 통과**하므로 ★**mac/linux 로는 절대 안 보인다**
  (모수 실측: **150 = 114 + 36**).
- ★★**「두 곳」이 급소다** — ★**같은 결함이 기록기에도 있었고**, Rust 만 고치면 ★**윈도우에서 기록한 표가 이번엔 mac/linux 를 red** 로 만든다.
  ★**그 축은 CI 가 기록기를 안 돌려서 «영영 조용하다»** ⇒ 둘을 함께 고쳤다(`as_posix()`).
- ★**형제 상호작용(②)은 «코드»가 아니라 «순서»였고, 그 사이 `#55` 가 먼저 착지해 해소된 형태로 나타났다**:
  base 를 당기니 그 3개가 **정확히 red 로 잡혔고**, 고친 기록기로 재기록하니 **150 → 153**(diff 정확히 3줄).
  ⇒ ★**기계가 설계대로 잡고 처방이 한 명령이었다는 실증.**
- ★**정정 하나 더(③)**: `STATE.md` 에 착지해 있던 「#55 는 원장 3파일만 만진다」가 **거짓**이었고,
  ★**그 오기 때문에 ②가 보이지 않았다** ⇒ 그 자리에서 반증을 붙였다.
- ★**이 설계의 «본래 대가»를 이제 «적어 뒀다»**: 「기록에 없는 픽스처 = red」는 의도된 엄격함이고,
  그 비용(픽스처를 더하는 회차가 표를 함께 만진다)이 **어디에도 없었다** ⇒ **`AGENTS.md` §Testing Boundaries 에 한 줄**.
- 검증: ⒞ 정규화를 되돌리면 ★**mac 에서도** 단위 시험이 진다(그래서 `components()` 대신 **문자열 치환**으로 구현했다 —
  전자는 Unix 에서 역슬래시 입력을 그냥 통과시켜 ★개악이 안 잡힌다) · Python 축은 `PureWindowsPath` 로 **윈도우 없이** 실증 ·
  `cargo test --all` **0 failed** · DoD 7명령 rc=0.

## [2026-09-17] `test-data` 전체의 클래스 파일 버전을 «동결»했다 — 통일이 아니라 ★**[위 `-fix` 가 윈도우 경로 축을 정정]** (rustjava-adopt-indy-fixture-jdk-pin-and-slot-accounting-p0)
- 무엇을: 커밋된 픽스처 **150개**의 `<major>.<minor>` 를 표에 기록하고, ★**혼자 움직이면 red** 가 되게 했다.
- 왜: 채택 제안 `2026-09-16-indy-fixture-jdk-pin-and-slot-accounting#p0`.
- 사용자 영향: 없다(시험 위생). ★바뀐 것은 **「다른 JDK 로 재생성했는데 아무도 모르는」 경로가 닫힌 것**이다.
- ★★**제안의 «비용 산정»을 바꿨다 — 기각이 아니라 «설계 교체»**: 제안은
  「**decide the intended target** per fixture … recompiled … **The decision is the work**」라며 **통일**을 전제했는데,
  정작 제안이 산 이득은 「a regenerated fixture **cannot quietly start testing a different Java version's** shapes」 =
  ★**«드리프트 탐지»**다. ⇒ 목표를 **동결**로 바꾸면 ★**그 「work」가 통째로 사라진다** — 재컴파일 **0** · 바이트 변경 **0** ·
  「어느 타깃이 옳은가」 **결정 불요**.
- ★**대가를 숨기지 않는다**: 이 축은 ★**섞임을 «고치지» 않고 «굳힌다»**. 통일하려면 재컴파일해야 하고,
  ★**재컴파일은 바이트를 바꿔 그 픽스처가 «무엇을 시험하는지»를 바꿀 수 있다**(제안 자신의 경고) ⇒ 별 축으로 넘겼다(후속 L).
- ★**ⓑ 이미 핀하는 축이 둘 있었고, 그 둘이 못 덮는 곳이 대상이었다**: `test_fixture_pins.rs` 는 **indy 만** ·
  생성기는 **자기 산출물 16개**의 버전을 소스에 박는다 ⇒ ★**남는 132개가 「javac 로 손수 재컴파일 가능」한 것**이고,
  루트의 **66×8 · 68×1 · 70×3** 이 그 사고의 지문이다.
- ★**세 방향을 전부 검사한다**: ⑴기록과 다름 ⑵★**기록에 없는 새 픽스처**(없으면 핀이 «옵트인»이 되어 내일 추가분이 조용히 미보호) ⑶유령 기록.
  개악 **3종 전건 red**(M1 버전 바이트 `65→70` · M2 미기록 픽스처 · M3 유령 행) · 복원 **green**.
  ★실패 문면이 **파일·두 버전·해소 명령**을 함께 말한다(`Hello.class: recorded 65.0, found 70.0`).
- 검증: 재측 인구조사(총 150 · 루트 5버전) · 기록기 **멱등**(재실행 시 표 바이트 동일) ·
  `cargo test --all` **572 → 573 / 0 failed / 1 ignored** · DoD 7명령 rc=0.
- 후속 추천: 루트 픽스처를 **하나의 타깃으로 재컴파일할지 «판정»**(L) — ★동결은 「움직였나」를 답하지 「우리가 원하는 값인가」를 답하지 않는다.
  상세 = `docs/worklog/2026-09-17-test-data-version-freeze.md`.
## [2026-09-17] 「어느 javac 이 만들었나」를 «기록»이 아니라 «검증»으로 바꿨다 (rustjava-adopt-indy-fixture-jdk-pin-and-slot-accounting-p1)
- 무엇을: 기록된 도구로 **다시 빌드해 바이트를 비교**하는 검사를 만들었다. ★**제품 코드 무접촉**(Rust 변경은 doc 주석 1곳).
- 왜: 채택 제안 `2026-09-16-indy-fixture-jdk-pin-and-slot-accounting#p1`.
- 사용자 영향: 없다(시험 위생). ★바뀐 것은 ★**「javac 26.0.1 로 만들었다」가 «주장»에서 «검증된 사실»이 된 것**이다.
- ★★**제안의 결론 «둘»이 실측으로 반증됐다**:
  ⑴「**Nothing offline can verify** a recorded compiler version … buys **provenance, not enforcement**」 →
  ★**거짓**. javac 은 같은 소스·플래그·컴파일러에 **결정적**이라 ★**`test-data/indy` 6개가 바이트 단위로 재현된다**.
  ⑵「강제 가능한 축은 `constant_pool.rs` 의 상수 개수뿐이고 **한 픽스처만** 덮는다」 → ★**거짓**.
  ★**javac 산출 indy 픽스처 3/3 이 형상 단언을 갖는다** — `ConstantKinds`(태그별 정확한 개수) ·
  `StringConcat`(신원 4축 + 인자가 «가리키는 값» + ★**바이트 창** `[15,6,0,35]`) · `Lambda`(`args[0]==args[2]!=args[1]`).
  ★제안이 든 위험(「현대 javac 이 enum switch 를 condy 로 낸다」)은 ★**`ConstantKinds` 의 Dynamic 개수 3 이 이미 잠근다.**
- ★**만든 것**: `test-data/src/verify-javac-fixtures.sh` — ★`--release` 를 **픽스처 자신의 major − 44** 로 읽어
  **외부 표에 의존하지 않고**, ★명시한 `JAVAC` 가 안 되면 **조용히 대체하지 않고 rc=2** 로 멈추며,
  ★**「못 만들었다」와 「만들었는데 다르다」를 «가른다»**(합치면 발견을 과장한다).
  ★**CI 에 배선하지 «않았다»** — 워크플로에도 PATH 에도 JDK 가 없어 **어디서나 실패하거나 어디서나 건너뛴다**.
- ★**실패담 둘(밟은 대로 적는다)**: ⑴`command -v javac` 이 macOS **스텁**을 고른다 ⇒ **실행해서** 판별 ⑵★`-sourcepath` 를 넣었다가
  **더 나빠졌다** — `test-data/src/Exception.java` 가 `java.lang.Exception` 을 가려 멀쩡하던 재현이 타입 오류로 무너졌다 ⇒ **되돌리고 그 대가를 따로 보고**.
- ★**개악 대조**: 커밋본 **1바이트 반전** → ★**✗ 감지** · 복원 → 6/6 재현 · 명시 `JAVAC` 부재 → ★**rc=2(통과 아님)**.
- ★**일반화 — 값만 적고 고치지 않았다**: 루트에 돌리니 **109 rebuilt · 104 재현 · 5 상이 · 3 재빌드 불가**.
  상이 5건(`MonitorSemantics`×3 · `NativeMethod` @8 · `OddEven` @21)의 ★**원인은 단정하지 않는다**(다른 컴파일러 ↔ 빌드 후 소스 수정이 둘 다 맞는다).
  ★그래도 적는 이유: ★**제안이 걱정한 드리프트가 «실재»한다는 첫 직접 증거**다.
- 검증: `cargo test --all` **572 / 0 failed / 1 ignored**(doc 주석만 바꿔 **불변**) · DoD 7명령 rc=0.
- 후속 추천: 루트 5건이 **왜** 재현되지 않는지 규명(M) — 상세 = `docs/worklog/2026-09-17-javac-fixture-provenance-verified.md`.
## [2026-09-17] `ClassFileError` 가 «원인»을 실어야 하는가 — ★**판정: 할 값 있다. 단 제안의 이름·이유·범위가 셋 다 틀렸다** (rustjava-adopt-cp-tag-passthrough-detectable-p1)
- 무엇을: 낱말이 **`Decide`** 인 제안을 **판정**했다. ★**코드 변경은 «틀린 주석 한 곳» 정정뿐** — 구현은 후속으로 넘겼다.
- 왜: 채택 제안 `2026-09-16-cp-tag-passthrough-detectable#p1`.
- 사용자 영향: 없다(판정). ★바뀐 것은 ★**착지한 트리가 «거짓 전제»를 나르지 않게 된 것**이다.
- ★★**제안의 «역사»가 거짓이다 — 두 겹으로**: ⑴`822504b` 는 `classfile/src/error.rs` 를 **«자르지» 않고 «만들었다»**
  (`new file` · 내용이 지금과 **동일한 2변형**) ⑵그 이전에는 `ClassInfo::parse` 가 **`Option<Self>`** 를 돌려줬다
  (실패에 정보 **0**) ⇒ ★**그 커밋은 «후퇴»가 아니라 «개선»이었고, 「again」·「restore」는 성립하지 않는다.**
- ★★**「upstream 이 해야 한다」도 거짓**: `upstream/main` 기준 **5커밋 뒤**이고 그중 이 파일들을 만지는 것 **0건** ·
  upstream 이 `error.rs` 를 만진 커밋은 **1건(생성)** 뿐 · ★**우리는 이미 이 crate 에서 크게 갈렸다**
  (`constant_pool.rs` **+211/−6** · `validation.rs` **+137/−0** · `attribute.rs` **+129/−3** · `opcode.rs` **+83/−7**).
  ※`AGENTS.md` 의 read-only 규율은 **upstream 으로 «보내는 것»**을 금할 뿐 로컬 변경을 금하지 않는다.
- ★★**ⓑ 설계는 «한 enum 건너» 이미 증명돼 있다**: `ClassDefinitionError::UnsupportedFeature(&'static str)` 가
  **5곳**에서 쓰이며 `"ldc of a method handle"` 같은 문장을 낸다 ⇒ ★**새 발명이 아니라 «일관성 회복»**이고, 모양도 자명하다.
- ★**이득(과장하지 않는다)**: kind-only 단언 **8곳**이 원인을 이름 부를 수 있게 되고, 참조 JVM 격차가 줄어든다
  (OpenJDK 는 `Multiple BootstrapMethods attributes…`·`argument_index 65535 has bad constant type` 를 내는데 우리는 전부 `Invalid class file`).
  ★**그러나 「픽스처보다 강한 자물쇠」는 «절반만» 참이다** — 원인은 「어느 검사가 울렸나」를, 픽스처는 「그 검사가 관측 가능한가」를 잠근다.
  ★**증거**: 직전 `-fix` 가 찾은 구멍(신원 4축 중 3축 미관측)은 **링크 축**이라 원인을 실었어도 **안 잡혔다**.
- ★★**비용 — 제안의 `target: classfile/src/error.rs` 는 틀렸다(1파일이 아니라 3층)**: 평탄화가 아래로 두 번 더 일어난다
  (`InvalidFormat` → `ClassDefinitionError::InvalidClassFile`(From 이 원인을 버린다) → ★**하드코딩 문자열 2곳**).
  ⇒ ★**`error.rs` 만 고치면 관측 변화가 «0»** 이다. ★**진짜 비용은 `validate_class` 의 8항 `||` 사슬을 쪼개는 것**이고,
  그것은 이 티켓이 **금지한 리팩터**다 ⇒ ★**여기서 구현하지 않고 «범위를 바로잡아» 넘겼다.**
- 검증: `cargo test --all` **572 / 0 failed / 1 ignored**(주석만 바꿔 불변) · DoD 7명령 rc=0.
- 후속 추천: 원인을 **3층에 관통**시키고 `validate_class` 를 **검사마다 반환**으로 쪼갠다(M) —
  ★「⑴만 하고 멈추면 관측 변화 0 · ⑷ 없이 하면 평탄함이 «이사»할 뿐」까지 제안에 적었다.
  상세 = `docs/worklog/2026-09-17-classfile-error-cause-decision.md`.
## [2026-09-17] 신원 4축을 «각각» 관측 가능하게 했다 — 감사의 「고칠 것이 없다」를 정정한다 (rustjava-adopt-cp-tag-passthrough-detectable-p0-fix)
- 무엇을: `string_concat.rs` 의 부트스트랩 신원 **4축**(kind·class·name·descriptor) 중 ★**3축이 «관측되지 않고» 있었다** —
  근접 실패 픽스처가 **class 축 하나**뿐이었기 때문이다. 나머지 3축의 픽스처를 만들었다. ★**제품 코드 무접촉.**
- 왜: 게이트② **request-changes**(PR #55 · 핀 `ab13a3c7`). 검수자가 **축을 하나씩** 지워 ★**kind·name·descriptor 개악이
  «전 스위트 green 인 채로» 살아남는 것**을 찾았다(직전 감사 회차의 굵은 개악 M7 은 4축을 한꺼번에 지워 그것을 못 봤다).
- 사용자 영향: 없다(테스트 픽스처). ★바뀐 것은 ★**「아무것이나 링크해도 테스트가 green」인 상태가 사라진 것**이다.
- ★★**전/후**: kind·name·descriptor 축 단독 삭제 → **SURVIVED**(`--all` 570/0/1 green) ⇒ ★**KILLED**
  (신규 `test_each_axis_of_the_factory_identity_is_observable`) · class 축은 **여전히 KILLED**(이제 2개를 죽인다 — 옛 커버리지 유지).
- ★**픽스처는 «적법한 파일»이어야 값한다**: `NotMakeConcatWithConstants`(name · `makeConcat` 은 실재 부트스트랩) ·
  `NotFactoryDescriptor`(descriptor · 여전히 적법한 서술자) · `NotInvokeStaticFactory`(kind **7** · JVMS 4.4.8 상 Methodref 와 적법).
  ⇒ 넷 다 **신원 검사까지 도달**해 `UnsupportedOperationException` 로 거부된다(상류가 먼저 거부하면 «다른 이유»로 통과하는 것이다).
- ★★**감사 방법론에 «역»을 남겼다**: 원 회신은 「죽지 않았다 ≠ 테스트가 약하다」를 적었는데 그 역이 빠져 있었다 ⇒
  ★**「죽었다 ≠ 그 가지가 «전부» 덮였다」 — 다축 술어를 통째로 지우는 굵은 개악은 «어느 한 축이 덮였다»만 증명한다.**
- ★**하네스가 실제로 트리를 오염시켰다**(러너 SIGKILL → `finally` 미실행) — ★계약 ⒡ 의 **「치환 1건 단언」이 그것을 잡았다**.
  복원 후 4축 전부 재측했고, 이 사건이 「하네스를 커밋하지 않는다」 결정의 **실증**이다.
- 검증: 회귀 표본 2종(BSM 경계 off-by-one · `Ldc2W` arm 제거) **여전히 KILLED** ·
  `test_class_format` **11 → 12** · `cargo test --all --no-fail-fast` **570 → 571 / 0 failed** · 픽스처 재생성 **멱등** · DoD 7명령 rc=0.

## [2026-09-16] `test_class_format.rs` 변이 저항 감사 — ★**«고칠 것이 없다»는 «한 자리에서» 거짓이었다**(위 `-fix` 가 정정) (rustjava-adopt-cp-tag-passthrough-detectable-p0)
- 무엇을: 이 스위트의 단언 **11개**가 「자기가 이름 붙인 가지」의 개악에 **실제로 죽는지** 쟀다. ★**코드 변경 0**(감사 회차).
- 왜: 채택 제안 `2026-09-16-cp-tag-passthrough-detectable#p0` — 「통과하지만 아무것도 재지 않는 단언이 더 있는지 보라」.
- 사용자 영향: 없다(측정). 바뀐 것은 ★**「없다」를 «추측»에서 «실측»으로 옮긴 것**이다.
- ★★**결과: 11/11 이 «적어도 하나»의 개악에 죽는다** — 개악 **10종**(매직 검사 해제 · 태그 pass-through 수용 ·
  `ldc2_w` 폭 제한 제거 · 버전 하한 `true` · BSM 인덱스 `true` · indy 를 「파손」으로 · 신원 4축 제거 ·
  ldc 미지원 arm 을 「파손」으로 · 파스 실패를 「미지원」으로 · not-found 를 「파손」으로). 표는 worklog 에.
- ★★**중간 함정을 기록한다** — 1차 목록(M1~M7)에서는 **3개가 살아남았다.** 그때 「약한 단언 3건」이라 적었으면 **거짓**이다:
  실제로는 ★**내 개악 목록에 그 가지들이 빠져 있었다**(ldc 미지원 arm · 파스 실패 매핑 · not-found 경로).
  M8·M9·M10 을 더하자 **전부 죽었다**. ⇒ ★**「죽지 않았다」는 «테스트가 약하다»와 «내가 그 가지를 안 건드렸다»를 구별하지 못한다.**
- ★★**ⓒ 제안의 전제는 쓰인 시점에 이미 거짓이었다**: 원문은 「several of which **also rely on corrupting a
  referenced slot of `Hello.class`**」라고 했으나, 제안이 실린 커밋(`eb8b4eb` = PR #49)의 같은 파일에서
  `Hello.class` 파생은 **3곳**뿐이고 ★**«참조 슬롯»을 덮는 것은 «0»** 이다(절단 · 매직 바이트 · 무손상 들러리).
  ⇒ 「several」은 ★**그 회차가 «방금 고친 그 하나»의 일반화**였다. ★그렇다고 감사가 헛되지 않다 — 「없다」를
  **재서** 아는 것과 **추측**하는 것은 다르다.
- ★**도구를 «남기지 않았다»**: 개악 하네스는 제품 **소스 문자열**을 매칭하므로 리팩터 뒤 ★**조용히 개악을 건너뛴다** —
  ★**하네스 자체가 「통과하지만 아무것도 재지 않는」 산출물**이 된다(이 제안이 사냥하는 그 형태). ⇒ **표를 문서로** 남겼다.
  ※`scripts/survey-ldc-constant-tags.py` 를 남긴 판단과 다른 이유: 그쪽은 **클래스 파일(데이터)** 을 읽어 낡지 않는다.
- 검증: 개악 10종 적용·복원 후 워킹트리 청결 · `cargo test --all` **570 / 0 failed / 1 ignored**(27 스위트 전건 합산) · DoD 7명령 rc=0.
- ★**감사의 경계**: 답한 질문은 「각 테스트가 «자기 가지»의 개악에 죽는가」이지 「어떤 구멍도 없다」가 아니다 ·
  **픽스처의 «유일 결함성»은 별도 축**(후속 추천) · 진행 중 PR #53·#54 의 새 테스트 2개는 범위 밖(각자 라운드에서 개악 대조 완료).
- 후속 추천: **픽스처**의 유일 결함성을 같은 방식으로 감사(M) — 상세 = `docs/worklog/2026-09-16-class-format-mutation-audit.md`.
## [2026-09-16] 부트스트랩 메서드의 «정적 인자» 인덱스를 경계 검사한다 (rustjava-adopt-bound-bootstrap-method-attr-index-p1)
- 무엇을: JVMS 4.7.23 의 `bootstrap_arguments` 는 상수 풀 인덱스인데 ★**아무도 그것이 실재하는지 보지 않았다.**
  이제 풀에 «없는» 인덱스를 가리키면 **거부**한다.
- 왜: 채택 제안 `2026-09-16-bound-bootstrap-method-attr-index#p1`.
- 사용자 영향: ★**진단이 바뀐다** — `UnsupportedOperationException` → ★`ClassFormatError`.
- ★★**급소는 「해석으로 흐르지 마라」였다**(제안 `tradeoff`). 지킨 방법 둘: ⑴술어가 **`contains_key` 하나**라
  항목을 **읽지 않는다** ⑵그 차이를 **술어 doc 에 적었다**(다음 사람이 보라고).
  ★**그리고 지켰다고 «주장»하지 않고 쟀다** — BSM 정적 인자를 **실제로 가진** 클래스들: `Lambda`·`ConstantKinds` →
  ★여전히 `UnsupportedOperationException: invokedynamic`(불변) · `StringConcat` → ★**여전히 실행된다**(`a0`).
  ⇒ 우려한 후퇴(람다 보유 클래스가 corrupt 로 되돌아가는 것)는 **일어나지 않았고 그것이 실측이다.**
- ★★**참조 JVM**: OpenJDK 26.0.1 → **`ClassFormatError: argument_index 65535 has bad constant type`**.
  ★그 문면이 ★**우리가 «하지 않은» 절반을 가리킨다** — JVMS 는 ⑴유효 인덱스 ⑵**loadable constant** 둘을 요구하고,
  제안·이 회차는 ⑴만 했다(⑵는 «종류» 검사 = 다른 문장이라 넓히지 않았다 · 후속 추천).
- ★**ⓑ 실측이 처방을 바꿨다**: `arguments` 를 읽는 제품 코드는 `string_concat.rs:92` **한 곳**뿐인데,
  거기서는 잘못된 인덱스를 ★**«조용히 링크 포기»로 처리**한다(`?`) ⇒ **감지는 되나 판정되지 않는다.**
  그래서 처방이 「그곳 수정」이 아니라 **「파스 시점 거부」**다.
- ★**픽스처**: `LdcDynamicBSMArgPastEnd.class`(생성기에 `static_arguments=` 추가 — 기존 `attr_index=` 와 같은 모양).
  인덱스를 **`0xFFFF`** 로 고른 이유는 ★**«부재»로만 실패하게** 하려는 것이다(풀 안의 «종류 틀린» 항목을 가리키면
  ⑵축과 섞여 «이름과 다른 이유»로 통과한다). 구조 측정: 인자 `[65535]` · 풀 유효 1..19 · 재생성 **멱등**(기존 11 동일).
- 검증: 개악 **양방향**(호출 제거 **red** · 상수 통과 **red** · 정상 green) ·
  `cargo test --all` **571 / 0 failed / 1 ignored**(27 스위트 **전건 합산**) · DoD 7명령 rc=0.
- ★**잃는 것**: 오탐 여지가 **한 자리** 있다 — long/double 의 **둘째 슬롯**은 이 맵에 없어 그것을 가리키는 인자는 거부된다.
  ★**의도된 읽기**다(그 슬롯에서는 어떤 상수도 적재할 수 없다 · JVMS 4.4.5) — 술어 doc 에 적었다.
- 후속 추천: 인자가 **loadable constant 종류**인지까지 볼 것인가 **판정**(위 ⑵ · OpenJDK 문면이 그 언어를 쓴다).
  상세 = `docs/worklog/2026-09-16-bound-bootstrap-static-arguments.md`.
## [2026-09-16] `BootstrapMethods` 를 두 번 선언한 클래스를 거부한다 (rustjava-adopt-bound-bootstrap-method-attr-index-p0)
- 무엇을: JVMS 4.7.23 은 `BootstrapMethods` 를 **최대 한 개**만 허용한다. 두 개를 실은 파일을 ★**거부**한다
  (종전에는 `find_map` 이 **첫 표**를 쓰고 나머지를 조용히 무시했다).
- 왜: 채택 제안 `2026-09-16-bound-bootstrap-method-attr-index#p0`(운영자 tower 패널 채택).
- 사용자 영향: ★**진단이 바뀐다** — `UnsupportedOperationException`(「이 런타임이 아직 못 한다」) →
  ★`ClassFormatError`(「이 파일이 깨졌다」). ★**참조 JVM 과 같은 판정**이 된다.
- ★★**참조 JVM 이 근거다**(observable behavior · OpenJDK 소스 미참조): OpenJDK 26.0.1 은 같은 파일에
  **`ClassFormatError: Multiple BootstrapMethods attributes in class file`** 를 내고, ★**표가 하나뿐인 대조군은
  rc=0 으로 로드**한다 ⇒ 픽스처의 결함이 «둘이라는 사실» 하나임이 참조 구현으로 확증된다.
- ★**고친 자리는 «한 곳»**: `validate_class` 에 술어 `at_most_one_bootstrap_methods_attribute` 를 이었다.
  ★`bootstrap_method_indices_resolve` 는 **무접촉** — 그 함수의 doc 이 스스로 「인덱스가 실재 항목을 가리키는가」라는
  **한 문장**임을 선언하고, 「표가 몇 개인가」는 **다른 문장**이다(접어 넣으면 이름까지 바꿔야 한다).
  ★**새 관용 0** — 필드 `ConstantValue`·메서드 `Code` 가 이미 쓰는 **개수 세기** 모양 그대로다.
- ★**픽스처는 «결함이 하나»가 되게 지었다**: `LdcDynamicDuplicateBSM.class` = 같은 유효한 표를 ★**바이트 동일**하게 두 번.
  어느 한 표만 있어도 정상 파일이라 ★**거부 원인이 «둘»로 고정**된다(둘째 표를 다르게 하면 다른 규칙이 먼저 물어
  테스트가 «이름과 다른 이유»로 통과한다). 구조 실측: 속성 `['BootstrapMethods','BootstrapMethods']` · 본문 동일 `True`.
- ★★**제안의 한 문장은 «과했다»**: 「생성기가 만들 수 없는 픽스처가 필요하다」 — ★**8줄 래퍼로 됐다**
  (속성 목록이 빌더에 그대로 전달된다). **관측은 맞았고 비용 추정이 틀렸다.**
- 검증: 개악 **양방향** — ⑴호출부에서 술어 제거(=제안 이전 상태) **red** ⑵술어 본문을 **`true`(상수 통과)** 로 **red**
  (★⑵가 없으면 「검사가 상수로 뭉개진」 축을 못 잡는다) · 정상 **green** ·
  `cargo test --all` **571 passed / 0 failed / 1 ignored**(27 스위트 **전건 합산**) · 픽스처 재생성 **멱등**(기존 11 바이트 동일) · DoD 7명령 rc=0.
- 후속 추천: 클래스 수준의 **다른 「최대 1개」 속성**(SourceFile·EnclosingMethod·Signature…)도 같은 규칙을 받아야 하는지 **판정**
  — ★이번 건이 검사를 얻은 이유는 「중복이 하류에 임의 선택을 만든다」이고, 파서가 무시하는 속성엔 그 논거가 **전이되지 않는다**.
  상세 = `docs/worklog/2026-09-16-reject-duplicate-bootstrap-methods.md`.

## [2026-09-16] `ldc` 태그 15/16/17 — ★**ASM 은 «낸다»**(Kotlin·Scala·Lombok 산출물은 0) (rustjava-ldc-tags-15-16-17-real-world-generator-survey)
- 무엇을: 선행 회차가 남긴 **「못 쟀다」**(ASM·Kotlin·Scala·Lombok)를 **쟀다**. 조사 회차 — ★**크레이트 무접촉**(파서·테스트 0).
- 왜: 채택 제안 `2026-09-16-ldc-tags-15-16-17#p2`. javac 은 안 낸다가 이미 증명됐고, **직접 바이트코드를 짜는 도구**가 남아 있었다.
- 사용자 영향: 없다(측정). ★**바뀐 것은 «우리 파서가 왜 그 셋을 받는가»의 근거**다 — 가설이 아니라 **실물 생성기가 내는 형상**이 됐다.
- ★★**결론**: **ASM 9.7.1 은 태그 15·16·17 을 «전부» 낸다** — 15줄짜리 프로그램(`visitLdcInsn(Handle/Type/ConstantDynamic)`)이
  만든 클래스에서 `{MethodHandle 1, MethodType 1, Dynamic 1}` 을 **실측**했다(정적 API 확인 + 동적 생성 **둘 다**).
- ★**Kotlin·Scala·Lombok 산출물은 5,479 클래스 · ldc 자리 17,819 곳에서 «0»** — 그러나 ★**그 0 을 「안 쓴다」로 읽지 마라**:
  같은 산출물 상수 풀에는 MethodHandle·MethodType 이 **가득하다**(`scala-library` 하나에 **1,604 + 723**).
  ★**전부 «부트스트랩 인자»이고 «`ldc` 피연산자»가 아니다** — javac 에서 관측된 형태가 Kotlin·Scala 에서도 그대로다.
  ★**태그 17(condy)은 풀에도 0** 이다.
- ★**계측기를 «대조군»으로 먼저 검증했다**: 이 저장소 `test-data/` 에서 **심어 둔 양성 8건을 8/8** 잡았고,
  「불가능 피연산자」 1건은 오차가 아니라 ★**심어 둔 음성**(`LdcUnknownTag`)이라 **실 오차 0**(선행 회차 계측기는 0.28%).
- ★**비용을 이렇게 줄였다**: 툴체인을 **설치하지 않고** ★**「컴파일러의 stdlib 은 그 컴파일러 자신의 산출물」**을 이용해
  Maven Central 에서 6개 jar(12.6MB)만 받아 **수천 클래스**를 쟀다. ★대가 = **좁음**: 컴파일러당 «한 프로젝트»다.
- ★**못 쟀다고 적은 칸 둘**(비워 두지 않았다): ⑴kotlinc·scalac 을 **타깃 형상으로 몰아 본 시험**(미설치)
  ⑵**ASM 위에 선 도구들**(ByteBuddy·Mockito·Groovy…)이 실제로 그 API 를 부르는지.
- 검증: 도구 `scripts/survey-ldc-constant-tags.py`(신규 · 크레이트 밖) · 스캐너 오차 막대 **전 corpus 0.00%** ·
  `cargo test --all` **570 / 0 failed / 1 ignored**(불변 — 크레이트 무접촉) · DoD 7명령 rc=0.
- 후속 추천: ⑴`test-data/ldc` 양성 픽스처를 **손으로 짠 바이트 → ASM 실물 산출물**로 교체
  ⑵kotlinc·scalac 을 설치해 **타깃 형상 축**을 채우기. 상세 = `docs/worklog/2026-09-16-ldc-tags-real-world-generator-survey.md`.


## [2026-09-16] `MethodHandleKind` 위치 재결정 — **옮기지 않는다**(재결정의 답이 「그대로」다) (rustjava-methodhandlekind-placement-revisit-after-pr44)
- 무엇을: `MethodHandleKind`·`MethodHandleRef` 를 `constant_pool.rs` 로 옮길지 **재결정**했다. ⇒ **`attribute.rs` 에 남긴다.**
  코드 변경은 **그 판단과 «재개 조건»을 doc 주석에 적은 것뿐**(타입·`impl`·re-export 무접촉 · 동작 변경 0).
- 왜: 채택 제안 `2026-09-16-bootstrap-methods-and-method-handle#p2`. ★낱말이 `Re-decide` 이고 ★**「아니오」도 정당한 답**이다.
- 사용자 영향: 없다(배치 판정 · 공개 API 변화 0).
- ★**제안이 적은 배치 사유 «둘» 중 하나만 만료됐다**: ⑵「형제 회차가 `constant_pool.rs` 를 열어 두고 있었다」는
  **만료**(PR **#44** `MERGED` · `2026-09-16T02:13:39Z` · 머지커밋 `dc03593` · ★그 PR 파일 목록에 그 파일이 실제로 있다) ·
  ⑴「유일한 소비자가 `BootstrapMethods` 속성이다」는 ★**여전히 참**이다.
- ★★**제안이 건 조건이 이미 충족됐고, 그 조건이 «declined» 를 가리킨다**: 「⒝(콜사이트 링크) 착지 후에 정하라」의
  그 ⒝ 가 오늘 **PR #48** 로 착지했는데 ★**그 소비자마저 `bootstrap.method.kind` 로 읽는다**
  (`jvm-bytecode/src/string_concat.rs:81`). `MethodHandleRef::resolve` 호출부는 ★**`attribute.rs:116` 단 1곳**이다.
- ★★**그러나 escape 절에 기대지 않았다 — 코드에서 더 나은 이유를 찾았다.** `constant_pool.rs` 는 **이미**
  CONSTANT_MethodHandle 을 어디까지 해독할지 정해 뒀다 — `ConstantPoolReference::MethodHandle` = ★**피연산자 0**
  (「아무도 resolve 하지 않는다」). ⇒ 옮기면 ★**같은 상수의 «두 해독»이 한 파일에 나란히 서서 모순으로 읽힌다** —
  이동이 없애려던 오해를 오히려 키운다.
- ★**잃는 것**: 「속성 파일이 상수 풀 타입을 갖는다」는 오해는 **남는다**. 그 비용을 이동이 아니라 **주석**으로 갚았고,
  ★**재개 조건**(이 속성 «밖»의 무언가가 method handle 을 해독할 때 — 유력 후보 `ldc`, 지금은 `UnsupportedFeature`)을 함께 박았다.
- 검증: `cargo test --all` **570 / 0 failed / 1 ignored** — ★직전 형상과 **동일**(불변 증명) ·
  `numstat` **10+/0−**(★삽입 의도이므로 「삭제행 0」 규율 비해당) · DoD 7명령 rc=0.
- 후속 추천: ★**새 카드 0** — 「X 가 생기면 다시 열어라」를 cockpit 열린 추천으로 띄우면 **집행 불가한 카드가 영구히 남는다**.
  재개 조건은 **코드 주석**에 뒀다(고칠 사람이 보는 자리). 상세 = `docs/worklog/2026-09-16-methodhandlekind-placement-revisit.md`.

## [2026-09-16] indy 픽스처에 JDK 핀을 «검사»로 박고, 슬롯 회계 직접 시험을 넣었다 (rustjava-indy-fixture-jdk-pin-and-slot-accounting-test)
- 무엇을: ⒜`test-data/indy` 의 javac 픽스처가 **class file 65.0**(= `--release 21`)을 유지하는지 **커밋된 바이트로** 검사한다.
  ⒝`parse_all` 의 「long·double **만** 상수풀 2칸」 규칙을 **파서 단위에서 직접** 문다.
- 왜: 채택 제안 `2026-09-16-cp-tags-16-17-execution-fixtures#p0`·`#p1`.
- 사용자 영향: 직접 없음(시험 위생). ★**픽스처를 다른 JDK 로 재생성하면 그 자리에서 빨개진다** —
  종전에는 아무 신호도 없었다. ★슬롯 회계 개악은 **40초짜리 전-JVM 스위트** 대신 **0.01초짜리 파서 시험**이 잡는다.
- ★**검사할 수 있는 것은 바이트뿐이다**: 이 repo 엔 `rust-toolchain.toml` 이 없고 CI 에 `setup-java` **0건**,
  이 맥의 PATH 에도 `javac` 가 없다(실물은 `/opt/homebrew/opt/openjdk/bin/javac` 26.0.1 로 PATH 밖) ⇒
  ★**컴파일러에게 묻는 검사는 원리적으로 불가능**하다. 핀 값과 그것을 강제하는 자리를 **한 파일**에 두어 기록·검사 드리프트를 없앴다.
- ★**핀 대상을 «`.java` 짝이 있는 것»으로 골랐다** — 형제 PR #48 이 같은 디렉터리에 **합성 픽스처**(major **52**)를 넣는다.
  디렉터리 전수 핀이었으면 ★**#48 착지 순간 red** 였다(그 파일을 받아 버전을 재서 확인했다).
- ★**제안의 전제 하나는 «틀렸다»**: ⒝의 「직접 시험 0」은 절반만 참이다(`long_must_fit_in_two_constant_pool_slots` 실재).
  ⇒ 그대로 추가하지 않고 **개악으로 «무엇이 안 잡히는지»를 쟀다**: ★**Double 을 1칸으로 바꾸면 `test_class` 단 1건**만 red 였다.
  그것이 「예」의 근거다.
- 검증: 핀 — 값 오기 **red** · ★`javac --release` **없이** 재생성(major 70) **red** · 검사를 상수 통과로 개악하면 그 red **소멸**.
  슬롯 — 개악 3종(`long→1칸`·`double→1칸`·`integer→2칸`) **전건 red**. `cargo test --all` **570 passed / 1 ignored** · DoD 7명령 rc=0.
  ※계측 함정: `cargo test` 는 **첫 실패 바이너리에서 멈춘다** — `--no-fail-fast` 없이 센 첫 측정은 과소계상이었다.
- 후속 추천: ⑴핀을 `test-data` 나머지로 확대(★근거: 루트가 **65×61·52×40·66×8·70×3·68×1** 로 다섯 버전이 섞여 있다)
  ⑵`--release` 가 아니라 **javac 바이너리**를 기록·검사하는 축(같은 `--release 21` 에서 javac 21·26 은 둘 다 65.0 이다).

## [2026-09-16] 「알 수 없는 태그를 거부한다」는 테스트가 ★**그것을 지키지 않았다** — 지키게 했다 (rustjava-cp-tag-switch-passthrough-mutation-detectable)
- 무엇을: `test_unsupported_constant_pool_tag_raises_class_format_error` 가 ★**상수풀 태그 switch 의 pass-through 가지를
  개악해도 green** 이었다. 그 가지가 «끝에서 끝까지» 관측되도록 **픽스처를 바꿔** 이제 **red** 가 되게 했다.
  ★**제품 코드 변경 0**(개악은 실증용 임시 · 전부 되돌렸다 · `git status` 로 확인).
- 왜: 채택 제안 `2026-09-16-ldc-tags-15-16-17#p1`. ★**「소비되지 않는 경보는 장식이다」의 테스트판** — 상수 pass 로 바꿔도
  통과하는 단언은 «없는 것과 같다».
- 사용자 영향: **없다**(테스트만 바뀐다). 바뀐 것은 ★**그 단언이 실제로 무엇을 잠그는가**다.
- ★★**근인 — 「통과한 진짜 이유」를 판별 실험으로 좁혔다.** 옛 테스트는 `Hello.class` **상수풀 1번**의 태그 바이트를 덮었는데,
  그 슬롯은 ★**코드가 `invokespecial` 로 «참조»하는 Methodref** 다 ⇒ 덮는 순간 파일이 **여러 경로로 동시에** 깨진다.
  `ClassFileError` 는 모든 파싱 실패를 ★**「Invalid class file」로 평탄화**하므로(그 파일이 스스로 적어 둔 사실)
  단언이 ★**「태그가 미지라 거부」와 「클래스가 무너져 거부」를 구별하지 못한다.**
  ★**바이트 폭 어긋남(desync)은 근인이 «아니었다»** — 4바이트를 정확히 소비하는 개악으로도 **여전히 green** 이었다(판별 실험 B).
- ★**처방은 단언 조이기가 «아니다»**(문면이 평탄해 불가능하다) — ★**입력이 그 가지에 «유일한 결함»으로 도달하게** 했다:
  `test-data/cp/UnreferencedTag{13,14,19}.class`(신규 생성기 `test-data/src/cp/make_cp_fixtures.py`) =
  ★**참조되지 않고 · 페이로드가 없고 · 상수풀 «맨 끝»**인 엔트리 하나만 미지 태그다.
  ★그 세 성질이 «전부» 값한다 — 맨 끝 + 페이로드 0이라야 pass-through 개악이 **정상 동작하는 클래스**를 만들고, 그래야 red 가 된다.
- ★★**전/후 — 같은 개악, 다른 결과**: ⑴**전**: pass-through 개악 → 그 테스트 **ok**(스위트에서 무는 것은 `classfile` 단위 테스트 1건뿐)
  ⑵**후**: 같은 개악 → ★**red**(실패 문면이 `must be rejected: ""` = 클래스가 «성공적으로 실행»됐다는 뜻).
  ⑶**다른 가지 개악**(태그 16 거부) → **6 테스트 red** ⇒ 스위트가 여전히 switch 전체를 지킨다.
- 검증: `cargo test --all` **568 / 0 failed / 1 ignored**(수 불변 — 테스트 1개 치환) · DoD 7줄 rc=0 · 픽스처 재생성 멱등 ·
  ★**옛 픽스처(`BadTag*`)를 쓰던 다른 테스트 0건**(전수 확인) · `hello_class()`·`fixture()` 헬퍼는 여전히 4·5회 쓰인다(고아 0).
- 후속 추천: ⑴같은 자를 다른 «조용한» 단언에 대 보기(개악 내성 감사) ⑵`ClassFileError` 에 원인 변종을 되살릴지 판정(상류 과제).
  상세 = `docs/worklog/2026-09-16-cp-tag-passthrough-detectable.md`.
## [2026-09-16] javac 의 문자열 `+` 가 «실제로 돈다» — 호출 지점 «하나»만 이었다 (rustjava-link-stringconcatfactory-makeconcatwithconstants)
- 무엇을: `invokedynamic` 중 ★**`StringConcatFactory.makeConcatWithConstants` 한 부트스트랩만** 링크한다.
  javac 9+ 가 문자열 `+` 를 내리는 그 형태다. ★**나머지 부트스트랩은 전부 종전대로 거부**된다.
- 왜: 채택 제안 `2026-09-16-bootstrap-methods-and-method-handle#p0`(이 배치의 유일한 **L**).
- 사용자 영향: ★**전/후가 «실행»으로 갈린다** — 전: `UnsupportedOperationException: … invokedynamic`(★`defineClass` 단계에서
  거부돼 **실행에 도달조차 못 했다**) → 후: ★**`a0` 을 출력하고 정상 종료**(`test-data/StringConcat.txt` 와 바이트 대조).
- ★★**급소는 접합이 아니라 «어디서 잇는가»였다**: `BootstrapMethods` 는 **클래스** 속성인데 `Interpreter::run` 은
  **메서드의 `Code` 만** 받는다 ⇒ 실행 시점에 부트스트랩 테이블에 닿을 길이 **없다**. ⇒ 둘을 «다» 쥔 유일한 자리인
  `ClassDefinitionImpl::from_classfile` 에서 **정의 시점에 내려쓴다**(`Opcode::InvokedynamicStringConcat`).
  ★`Interpreter::run` **시그니처 불변** · ★`java.lang.invoke`(MethodHandle·CallSite) **0줄** — 레시피를 직접 집행한다.
- ★★**「전부 열어 버린」 변경과 구별되는 축을 «만들어» 넣었다**: 인식은 **kind·class·name·descriptor 4축 완전일치**다.
  ★그런데 그 검사를 지워도 기존 픽스처는 **전부 green 이었다**(Lambda·ConstantKinds 는 정적 인자가 String 이 아니라
  «다른 이유»로 먼저 걸린다) ⇒ ★**신원 검사가 아무 테스트에도 안 물려 있었다.** 그래서 ★**근접 오답 픽스처**
  `NotStringConcatFactory`(소유 클래스 한 축만 다르다 · `test-data/src/indy/make_indy_fixtures.py`)를 새로 만들었다.
- ★**개악 2종**: ⑴링크 끊기 → `test_class`(출력 대조)·`test_only_the_string_concat_bootstrap_is_linked` **red**
  ⑵무차별 링크(4축 제거) → ★**근접 오답 픽스처가 링크돼 red**(그 픽스처 «전»에는 green 이었다 — 위).
- 검증: `cargo test --all` **568 / 0 failed / 1 ignored**(수 불변 — 테스트 1개 치환 · 픽스처 1쌍 추가) · DoD 7줄 rc=0.
- ★**잃는 것**: 위험의 «종류»가 「안 돈다」 → ★**「잘못 돌 수 있다」**로 바뀐다. 그래서 판정을 **출력값 대조**로 잡았다.
- 후속 추천: ⑴`makeConcat`(인자 없는 형제 팩토리) ⑵부트스트랩 정적 인자가 String 이 아닌 레시피 형태 ⑶`LambdaMetafactory`(L).
  상세 = `docs/worklog/2026-09-16-link-stringconcatfactory.md`.
## [2026-09-16] `bootstrap_method_attr_index` 가 «실재하는» 부트스트랩 메서드를 가리키게 했다 (rustjava-bound-bootstrap-method-attr-index)
- 무엇을: `Dynamic`/`InvokeDynamic` 상수의 `bootstrap_method_attr_index` 가 **BootstrapMethods 테이블 안**을 가리키는지
  검사한다(JVMS 4.4.10·4.7.23). ★**두 축이 한 술어다** — 속성이 «아예 없는» 경우는 «항목 0개짜리 표»여서 어떤 인덱스도 못 가리킨다.
- 왜: 채택 제안 `2026-09-16-bootstrap-methods-and-method-handle#p1` · `2026-09-16-ldc-tags-15-16-17#p0`(서로 다른 회차가 **독립으로** 같은 결함에 닿았다).
- 사용자 영향: ★**진단이 바뀐다** — 그 두 형상이 `UnsupportedOperationException`(「이 런타임이 아직 못 한다」) →
  ★`ClassFormatError`(「이 파일이 깨졌다」). ★**어떤 JVM 도 못 읽는 파일을 «미지원»이라 부르던 것을 그만둔다.**
- ★★**의도된 «거부 확대»다 — 하류에는 회귀로 보인다.** 지금까지 조용히 「미지원」으로 넘어가던 클래스 파일이 **거부**된다.
  ★그 전환이 이 회차의 산출물 자체이고, OpenJDK 26 은 같은 파일에 `ClassFormatError: Missing BootstrapMethods attribute` 를 낸다.
- ★**검증 지점을 하나로 모았다**(계약 1): `validate_class` 안의 `bootstrap_method_indices_resolve` **한 함수** ·
  ★**새 에러 타입 0**(기존 `ClassFileError::InvalidFormat` 에 접었다 — 호출부 변종 증가 0).
  ★`validate_constant_pool` 이 아니라 `validate_class` 인 이유 = **풀과 클래스 속성을 «둘 다» 쥔 유일한 자리**이고,
  그 교차가 이 검사가 여태 없던 이유였다(그 자리의 낡은 주석이 그렇게 적고 「원하는 회차에 맡긴다」고 했다 — 이 회차가 그것이다).
- ★**개악 3종**: 상수 `true` → 새 테스트 red(축 ⒜⒞) · 상수 `false` → **24 테스트** red(축 ⒝⒟) ·
  ★내부 술어만 `false` → **8 테스트** red이고 `test_hello` 류는 **green** ⇒ ★**⒝ 와 ⒟ 가 갈린다**(`false` 하나로는 둘이 겹친다).
- 검증: `cargo test --all` **568 / 0 failed / 1 ignored**(수 불변 — 테스트 1개를 «치환»했다) · DoD 7줄 rc=0 ·
  ★픽스처 재생성 **멱등**(기존 10개 바이트 불변 · 신규 1개만 추가).
- 후속 추천: ⑴`BootstrapMethods` 중복 선언 거부(JVMS 4.7.23 은 «최대 1개» — 지금은 첫 것만 본다)
  ⑵`MethodHandleKind` 의 위치 재판정(형제 티켓) ⑶`StringConcatFactory` 콜사이트 링크(L · 별 티켓).
  상세 = `docs/worklog/2026-09-16-bound-bootstrap-method-attr-index.md`.

## [2026-09-16] `BootstrapMethods` 를 «구조»로 읽는다 — 콜사이트 링크는 0줄 (rustjava-invokedynamic-bootstrapmethods-and-methodhandle)
- 무엇을: `AttributeInfo::BootstrapMethods` 를 **`Vec<u8>` → `Vec<BootstrapMethod>`**(JVMS 4.7.23)로 파싱하고,
  `CONSTANT_MethodHandle` 을 `MethodHandleRef`(`MethodHandleKind` 9종 + 클래스·이름·서술자)로 **해독**한다.
  ★**`STATE.md` ④-1 이 「최소 둘로 갈라라」고 못박은 그 ⒜ «만»** — ⒝(콜사이트 링크·`StringConcatFactory` 런타임)는 **0줄**.
- 왜: 채택 제안 `2026-09-16-cp-tags-15-18-parse#p0`. 직전 두 회차가 「파손 → 미지원」을 닫았고, 남은 칸이 「미지원 → 지원」이다.
  그 첫 삽이 **부트스트랩 메서드를 «읽는» 것**이고, `ConstantPoolReference::InvokeDynamic` 이 인덱스를 원문 그대로 들고 있어 파서를 다시 고칠 필요가 없었다.
- 사용자 영향: ★**없다 — 동작 불변**(그 클래스는 오늘도 `invokedynamic` 미지원으로 거부된다). 바뀐 것은 **다음 회차가 쓸 자료구조**다.
- ★★**「MethodHandle 결정」의 «경계»를 이름으로 적었다 — ★「전부 된다」로 적지 않았다.**
  되는 것 = **클래스파일에 적힌 (종류·클래스·이름·서술자) 해독**. 안 되는 것 = 클래스 로드 **0** · 멤버 조회 **0** ·
  접근 검사 **0** · ★**`java.lang.invoke` 런타임 클래스 «0개»**(실측 — 디렉터리 부재 · 참조 0건) ⇒ **`MethodHandle` 객체 생성 불가** ·
  종류↔대상 짝짓기는 **`validation.rs` 몫** · `bootstrap_method_attr_index` **경계 미검사** · 정적 인자 **미해석**.
- ★★**급소 — 「정적 인자를 풀지 않는다」가 «선택»이고 그것을 측정했다**(개악 M3):
  「인덱스를 `ConstantPoolReference` 로 풀어라」는 가장 자연스러운 다음 줄인데, `LambdaMetafactory.metafactory` 의 인자가
  **MethodType·MethodHandle·MethodType** 이라 강제하면 ★**람다가 든 모든 클래스가 «파싱»에서 죽는다** —
  `UnsupportedOperationException: … invokedynamic` → ★`ClassFormatError: Invalid class file` 로 **되돌아간다**.
  ★★**그런데 M3 에서 `StringConcat` 은 «green» 이다**(그 인자는 String 하나뿐) ⇒ ★**기존 픽스처만으로는 이 회귀가 보이지 않는다.**
  그래서 픽스처 **`test-data/indy/Lambda.class`**(javac `--release 21`)를 새로 넣고 **두 층**(파스·end-to-end)에서 물게 했다.
- ★**개악 4종 전건 red · 복원 green**: M1 필드 폭 1개(`num_bootstrap_arguments` u16→u8) · M2 `bootstrap_method_ref` 인덱스 +1 ·
  M3 정적 인자 해석 강제 · M4 verifier 분기 제거 → ★`panicked at jvm-bytecode/src/interpreter.rs:631`(**호스트 abort**)
  ⇒ ★**`todo!()` 는 여전히 «도달 불가»이고 그것을 «측정»했다.**
- 검증: `cargo test --all` **558 → 562 / 0 / 1**(신규 4 · 감소 0) · DoD 7줄 rc=0 ·
  ★`verifier.rs`·`interpreter.rs` **무접촉**(`git diff --stat` 빈 출력) · ★낡은 주석 2곳을 **전수 계수로** 닫았다.
- 후속 추천: ⑴**⒝ 콜사이트 링크**(`StringConcatFactory` 한 종류 · `java.lang.invoke` 패키지 신설 · L)
  ⑵`bootstrap_method_attr_index` 경계 검사(④-2 의 「태그↔major 버전」과 한 회차)
  ⑶`MethodHandleKind` 를 `constant_pool.rs` 로 옮길지 재판정(오늘은 소비자가 하나라 attribute.rs 에 뒀다).
  상세 = `docs/worklog/2026-09-16-bootstrap-methods-and-method-handle.md`.

## [2026-09-16] `ldc` 의 태그 15·16·17 도 «파손»이 아니라 «미지원»이라고 말한다 (rustjava-ldc-tags-15-16-17-still-malformed)
- 무엇을: `ldc`/`ldc_w`/`ldc2_w` 가 **MethodHandle(15)·MethodType(16)·Dynamic(17)** 을 만났을 때의 진단을
  `ClassFormatError: Invalid class file` → ★`UnsupportedOperationException: Unsupported class file feature: ldc of a …`
  로 바꿨다. 직전 회차(PR #43)가 `invokedynamic` 에 쓴 것과 ★**같은 관용**(파싱 → verifier 거부) · ★**`interpreter.rs` 무접촉.**
- 왜: `STATE.md` ④-2 가 직전 회차 자신의 산출물로 지목한 「같은 종류의 거짓말이 한 자리 더」다.
  ★**그리고 이 셋은 ④-1(`invokedynamic` 실행)이 «필요로 할» 바로 그 상수들**이라 거기 닿는 사람이 먼저 만난다.
- 사용자 영향: 정상 동작 **불변**(그 클래스는 오늘도 못 돈다). ★**바뀐 것은 «왜 못 도는지»를 말하는 문장이다.**
- ★★**⓪ 판정이 «둘»로 갈렸다 — 흐리지 마라**: ⒜**재현됐다**(픽스처 4종 전건 `Malformed`)
  ⒝★**그러나 javac 은 그 형태를 «내지 않는다» — 재현 불가가 아니라 «측정된 부재»다.**
  JDK 자체 jmods **27,902 클래스 · `ldc` 계열 1,252,714 자리 · 0건**(계측기는 공허하지 않다 — 같은 corpus 에서
  String 1,176,232 · Integer 23,292 … 를 되찾았고 디코드 드리프트는 **0.28%**) · `--release 21/25/26+preview` 로
  문자열 연결·람다·메서드참조·레코드·패턴 switch 등 **14종** 컴파일 → 태그 15·16 은 **부트스트랩 인자로만** 등장 · 태그 17 은 **0**.
  ⇒ ★**픽스처는 «합성»이고 그렇게 «적었다»**(생성기·테스트·worklog 3곳). ★**「평범한 코드에서 나온다」로 적지 않았다.**
  ★**못 잰 것도 적는다**: ASM·Kotlin 류 서드파티 jar corpus 는 **이 머신에 0개**라 그쪽은 **미측정**이다.
- ★★**참조 JVM 이 «근거»다**(소스 참조 아님 — 허용 축인 observable behavior): OpenJDK 26.0.1 이
  양성 픽스처 **4종을 전부 로드·실행**한다 ⇒ ★**「못 읽는 파일」이 아니라 「못 하는 파일」임이 실측으로 선다.**
  ★**그 참조 JVM 이 내 픽스처를 «두 번» 반려했고 그것이 부수 산출물이다**: 태그 17 은 **major ≥ 55** 필요 ·
  `Dynamic` 은 **`BootstrapMethods` 속성 필수**(JVMS 4.7.23). ★**우리 `validation.rs` 는 «둘 다» 검사하지 않는다** ⇒ 후속 ⑴.
- ★★**대가(②)를 지불하지 않았음을 «음성 대조군»으로 보였다**: `ldc2_w` 에 MethodType(JVMS 6.5 위반) ·
  `ldc` 가 가리키는 자리에 태그 19 ⇒ ★**둘 다 여전히 `ClassFormatError`**(참조 JVM 도 각각 VerifyError·ClassFormatError 로 거부).
- ★**개악 5종**: M1 verifier 새 분기 제거 → ★`panicked at jvm-bytecode/src/interpreter.rs:1063`(**호스트 abort** —
  직전 회차가 `todo!()` 에서 잰 것과 **같은 형태**) · M2 `from_constant_pool` 원복 · M3 opcode 분기 원복 · M4 `ldc2_w` 확장 → **전건 red** ·
  ★★**M5 는 «내 테스트가 못 잡았다»** — 상수풀 태그 switch 를 pass-through 로 만들어도 `test_class_format` 은 **전건 green** 이었고,
  실제로 무는 것은 **직전 회차의 `classfile` 단위 테스트**다. ★**축은 잠겨 있으나 «내가 단언한 층»이 아니다 — 그대로 적는다.**
- 검증: `cargo test --all` **558 → 560 / 0 / 1**(신규 2 · 감소 0) · DoD **7줄**(= 파리티 검사기 기준 **명령 6개**) 전건 rc=0 ·
  ★`verifier.rs` 의 `Invokedynamic` 줄 **무접촉** · `interpreter.rs` **무접촉**(`git diff --stat` 부재).
- ★★**[-fix 2026-09-16 · 게이트② `request-changes` 승계] 위 후속 ⑴ 은 «후속»이 아니라 «이 회차가 만든 구멍»이었다 — 같은 PR 에서 닫았다.**
  넓힌 수용집합이 ★**우연한 백스톱**(`Dynamic → None → opcode 파싱 실패`)을 **대체 없이** 걷어내, ★**참조 JVM «도» 못 읽는**
  파손 condy 2종이 「미지원」이라 답했다 ⇒ ★**이 회차의 문장이 정확히 반대로 뒤집힌 대역**. before `ab872b7` 는 둘 다 `ClassFormatError` 였다(재측).
  ★**처방 = major 버전 축**(`validation.rs` · 속성 파싱 0) — ★**검수자의 1행을 4행 표로 넓혔다**(실측: 같은 결함이 **태그 15·16 에도** 있었다)
  ⇒ **15·16·18 ≥ 51 · 17 ≥ 55**(JVMS 4.4) · ★**대가 0**(jmods 27,902 클래스 위반 0 · ★단 전부 major 69·70 이라 ≥51 행은 미시험).
  ★**남는 대역 «1»** = `LdcDynamicNoBSM`(BSM 경계 검사는 속성 파싱이 필요해 ④-1 몫) — ★픽스처·테스트로 **현재 답을 잠갔다**.
  ★**양방향**: M1 검사 제거 → 파손 축 2 red · ★M2 「전부 파손으로 되돌리기」 → **양성 4종 red**(되돌리기는 통과 방법이 아니다) · 복원 green.
  ★픽스처 **+4**(태그 13·14 포함 — 「13·14 로는 구성 불가」가 거짓임이 확인됐다) · 문안 정정 **3건** · `cargo test --all` **560 → 562 / 0 / 1**.
- 후속 추천: ⑴★**`Dynamic` ↔ `BootstrapMethods` 경계 검사**(남는 대역 1건 · ④-1/PR #45 리니지 몫)
  ⑵M5 가 드러난 층 어긋남 표기 ⑶서드파티 bytecode 생성기 corpus 측정(저우선).
  상세 = `docs/worklog/2026-09-16-ldc-tags-15-16-17.md`.

## [2026-09-16] javac 은 태그 17(condy)을 «낸다» — 실물 픽스처로 잠갔다 (rustjava-cp-tags-16-17-execution-fixtures)
- 무엇을: 상수풀 태그 **16(MethodType)·17(Dynamic)** 을 **실제 javac 산출물**에서 읽는 픽스처
  `test-data/indy/ConstantKinds.class`(소스 동봉)를 커밋하고, **두 층**(실행 · 상수풀 계수)에서 잠갔다. ★런타임 `.rs` **0줄**.
- 왜: 채택 제안 `2026-09-16-cp-tags-15-18-parse#p2`. 그 회차는 네 태그를 더했는데 **실물로 검증된 것은 15·18 뿐**이었고,
  16·17 은 «손으로 만든 바이트»로만 시험됐다. 제안의 논거는 「**오프셋 실수가 단위 테스트를 통과하고 실물에서만 드러난다**」였다.
- 사용자 영향: **없다**(제안 자신이 「없음 — 회귀 방어의 깊이만 는다」고 적었다). 바뀐 것은 **그 방어의 깊이**다.
- ★★**직전 회차의 「못 찾았다」를 «뒤집었다» — 그러나 그것은 «정정»이 아니라 «승계»다.**
  그 회차는 「javac 가 그 둘을 내는 평범한 코드를 **찾지 못했다**」고 정직하게 적었다(거짓 단언이 아니다). ⇒ **이 회차가 찾았다.**
- ★**태그 17 의 산출 조건**: ★**`switch` case 라벨이 «정규화된 enum 상수»이고 선택자가 enum 이 아닐 때**(JEP 441) —
  javac 이 각 상수를 `Enum$EnumDesc` 로 기술하며 `ConstantBootstraps.invoke` condy 를 낸다.
  ★**평범한 enum switch·sealed pattern switch 는 둘 다 «0»** 이다(실측 — 그 둘만 보면 「javac 은 condy 를 안 낸다」로 오답한다).
- ★★**희소도를 쟀다**: OpenJDK 26 자체 jmods **27,902 클래스 중 태그 17 보유 «1»**(`jdk/jpackage/…/PackageBuilder`) ↔
  태그 16 은 **1,747 클래스 · 8,434 항목**. ⇒ **16 은 흔하고 17 은 사실상 없다** — 그래서 실물 픽스처가 값을 한다.
- ★★**제안이 예측한 실패 형태가 «실재한다» — 측정했다**: `parse_all` 의 `is_double_entry` 에 `Dynamic`(또는 `MethodType`)을 더하면
  ★**격리 단위 테스트는 «ok»**(그것은 `parse_tagged` 를 직접 부른다) ↔ ★**실물 풀은 전 항목이 밀려 `ClassFormatError`** 로 죽는다.
- ★★**양방향(출력으로)**: ⒜픽스처 테스트 제거 + 같은 개악 → 전 스위트 ★**558 passed / 0 failed(green)** ⇒ **그 축을 무는 것이 아무것도 없었다**
  ⒝픽스처 복원 + 같은 개악 → ★**2층 red** · 복원 green.
- 검증: `cargo test --all` **558 → 560 / 0 / 1**(신규 2 · 감소 0) · DoD 7줄 rc=0 ·
  ★`verifier.rs`·`interpreter.rs` **무접촉** · `BootstrapMethods` 파싱 **0줄**(형제 L 티켓 몫) · 태그 15 축 **무접촉**(형제 회차 몫).
- 후속 추천: ⑴JDK 판올림 시 `ConstantKinds` 재컴파일 여부 판정(condy 산출 조건은 preview 가 아니라 정식이라 안정적이나 «재지 않았다»)
  ⑵`parse_all` 의 슬롯 회계를 단위 테스트가 직접 덮게 할지 판정 ⑶서드파티 생성기 corpus 미측정(형제 회차와 같은 한계).
  상세 = `docs/worklog/2026-09-16-cp-tags-16-17-execution-fixtures.md`.

## [2026-09-16] javac 9+ 클래스가 «파손»이 아니라 «미지원»이라고 말한다 (rustjava-cp-tags-15-18-parse-and-honest-diagnosis)
- 무엇을: 상수풀 태그 **15·16·17·18** 과 ★**opcode `0xba`** 를 파싱하게 해서, javac 9+ 산출물의 진단을
  `ClassFormatError: Invalid class file` → ★`UnsupportedOperationException: Unsupported class file feature: invokedynamic`
  으로 바꿨다. ★**`invokedynamic` 실행은 0줄**(범위 밖) · ★**`interpreter.rs` 무접촉.**
- 왜: 두 실패가 한 이름을 쓰고 있었다. 「파일이 깨졌다」(내 잘못)와 「런타임이 못 한다」(도구의 한계)는
  ★**사용자에게 전혀 다른 문장**이고, javac 9+ 는 **문자열 `+` 한 줄**조차 invokedynamic 으로 낸다.
- 사용자 영향: 정상 동작 **불변**(오늘도 그 클래스는 못 돈다). ★**바뀐 것은 «왜 못 도는지»를 말하는 문장이다.**
- ★★**티켓 ① 의 급소 지목이 «한 칸 모자랐다» — 이 회차의 가장 값진 실측**: 태그 15~18 을 **전부 살려 둔 채**
  `0xba` 분기(`map_res(…, |_| Err(()))`)만 옛 판본으로 되돌리면 픽스처는 ★**다시 `Malformed`** 로 죽는다(개악 M3).
  ⇒ 「파손 → 거부」 칸에 필요한 파서 수정은 **둘**이고 티켓은 **하나**를 적었다. ★**분할 판단 자체는 옳다.**
- ★★**`todo!()` 미도달을 «측정»했다**(추론 아님): verifier 의 `Opcode::Invokedynamic(_)` 분기만 일시 제거하면
  ★`panicked at jvm-bytecode/src/interpreter.rs:631: not yet implemented` **호스트 abort** 가 난다(M4) ↔ 현 트리는 게스트 예외.
  ⇒ 그 `todo!()` 와 픽스처 사이에 선 것은 **정확히 verifier 한 줄**이고, 그것이 서 있다.
- 검증: 픽스처 `test-data/indy/StringConcat.class`(`javac --release 21` · major 65) · 개악 **4종 전건 red · 복원 green** ·
  ★**미지 태그 축 유지**(13·14·19 로 `ClassFormatError` 재단언 — 기존 테스트가 «태그 18 = 미지원»을 사례로 쓰고 있었다) ·
  `cargo test --all` **554 → 558 / 0 / 1**(신규 4 · 감소 0) · DoD 6종 rc=0.
- 후속 추천: ⑴**invokedynamic 실행**(BootstrapMethods 파싱 + 콜사이트 링크 · L · 갈라야 한다)
  ⑵`ldc` 로 실린 태그 15·16·17 은 ★**여전히 `Malformed`** — 같은 종류의 거짓말이 한 자리 더 남아 있다(재현 픽스처를 먼저 재라)
  ⑶태그 16·17 은 실행 픽스처가 없다(단위 테스트만). 상세 = `docs/worklog/2026-09-16-cp-tags-15-18-parse.md`.

## [2026-09-12] 병렬화의 «숨은 전제»를 못박았다 — 격리는 넣지 않았다 (rustjava-test-class-parallel-and-scratch-isolation)
- 무엇을: 파일을 쓰는 픽스처가 «고정 이름»을 쓰는 것이 **안전한 이유**(전제 3개)와 **깨지면 무엇이 일어나는지**를
  ★**테스트 소스 «안»에** 적었다(`tests/test_class.rs`·`tests/test_real_jvm.rs` 주석 2곳). ★**격리 구현 0 · 런타임 무접촉.**
- 왜: 제안 자신이 「오늘 얻는 것이 없다」로 유보했고 티켓이 **재판정**을 시켰다 ⇒ ★**유보는 옳다**
  (지금 격리를 넣으면 픽스처 관용만 늘고 **지킬 축이 없어** 다음 사람이 또 고정 이름을 쓴다).
- 사용자 영향: 없음(주석 전용). ★**병렬화하려는 다음 회차가 «선행 조건»을 그 자리에서 읽는다.**
- 검증: 전제 3개를 **코드·실행 로그로** 확인 · `cargo test --all` **554/0/1**(전체 실행) · DoD 7종 rc=0.
  ★**대조쌍 «없음»** — 구현하지 않았고, 모의 병렬을 세우려면 이 회차가 금지당한 병렬화가 선행한다(Acceptance 허용 경로).
- ★**제안의 전제가 «하나»가 아니라 «셋»이었다**: 그중 `tests/test_real_jvm.rs` 가 **같은 디렉터리를 순회**하는데
  `#[ignore]` 로만 막혀 있다는 것이 ★**내가 그 제안을 쓸 때 확인하지 않은 자리**다.
- 후속 추천: 병렬 러너를 **실제로 도입하는 회차**가 **선행 조건으로** 픽스처별 작업 디렉터리를 함께 넣는다.

## [2026-09-12] 게스트 서브클래싱은 «된다» — `java/net` 가드 2곳 잠금 (rustjava-fixture-subclassing-abstract-runtime-classes)
- 무엇을: 「게스트 픽스처가 **추상 런타임 클래스**를 상속할 수 있는가」를 **먼저 쟀고**(미확인이었다), ★**된다**로 판정해
  `URLStreamHandler.setURL` · `JarURLConnection(URL)` 두 가드를 픽스처 `test-data/NullNetGuards` 로 **잠갔다**. ★**`.rs` 변경 0줄.**
- 왜: 그 2곳은 가드가 있어도 **지워도 아무도 모르는** 상태였다 — `protected` 진입점이라 **서브클래스 없이는 도달할 수 없다**.
- 사용자 영향: 런타임 동작 **불변**(시험만 늘었다). ★두 가드가 사라지면 이제 **CI 가 말한다**.
- 검증: ★**양방향 · 자리마다** — 픽스처를 **일시 제거**하면 두 개악이 **둘 다 green**(안 잠김) ↔ 있으면 **둘 다 RED**.
  `cargo test --all` **554/0/1**(전체 실행) · DoD 7종 rc=0.
- ★**부수 발견**: 기존 픽스처가 이미 **`ClassLoader`** 를 상속하고 있었다 ⇒ 진짜 미확인은 「런타임 클래스 상속」이 아니라
  **«추상 + `protected` 진입점» 조합**뿐이었다. 조사 전제를 좁혔으면 더 쌌다.
- 후속 추천: 이 축의 **미커버가 0 이 됐다**는 사실 자체를 기록(다음 사람이 같은 20곳을 다시 세지 않도록).

## [2026-09-12] `ZipFile.getInputStream` 가드 잠금 — ★`ZipOutputStream` 을 만들지 않았다 (rustjava-zip-output-stream-minimal-for-fixture-reachability)
- 무엇을: 규격 근거로 넣었으나 **잠기지 않던** `ZipFile.getInputStream(ZipEntry)` 가드를 픽스처 `test-data/ZipGuards` 로 **잠갔다**.
  ★**런타임(`.rs`) 변경 0줄 · `ZipOutputStream` 구현 0줄 · 추가 바이너리 0.**
- 왜: ★**제안의 근인 진단이 틀렸다** — 「근인은 `ZipOutputStream` 부재」였으나 필요한 것은 **«zip 파일 하나»**이지 «zip 을 만드는 런타임»이 아니었다.
  ⇒ 이미 있는 `test-data/test.jar`(**jar 은 zip 이다**)를 열어 살아 있는 `ZipFile` 을 얻었다.
- 사용자 영향: 런타임 동작 **불변**(시험만 늘었다). ★그 가드가 사라지면 이제 **CI 가 말한다**.
- 검증: ★**양방향** — 착수 시 같은 개악이 **green**(안 잠김) ↔ 이 회차 뒤 **red**(호스트 abort) · 복원 **ok**.
  `cargo test --all` **554/0/1**(전체 실행) · DoD 7종 rc=0.
- ★**남은 한계를 «섞지 마라»**: 게스트가 zip 을 «쓰는» 경로는 여전히 없다(`ZipOutputStream` 부재).
  그것은 「게스트가 zip 을 만든다」는 요구가 생길 때의 일이지 **가드 잠금 때문이 아니다**.
- 후속 추천: `ZipFile.close()` 등재 — 게스트가 흔히 부르는데 `NoSuchMethodError` 가 난다(이 회차가 픽스처에서 실제로 밟았다).

## [2026-09-12] 파일 기반 IO 가드 6곳을 회귀로 묶었다 (rustjava-null-guard-fixture-for-file-backed-io)
- 무엇을: 살아 있는 파일 핸들이 있어야 도달하는 가드 **6곳**(`FileInputStream`·`FileOutputStream`·`RandomAccessFile`)을
  임시 파일 픽스처 `test-data/NullFileIoGuards`(7케이스)로 **잠갔다**. ★**`.rs` 는 한 줄도 바꾸지 않았다.**
  채택 제안 `2026-09-11-null-guard-audit-and-io-buffer-guards#p1`.
- 왜: ★**「가드가 있다」와 「가드가 잠겨 있다」는 다른 문장이다** — 그 6곳은 **지워도 아무도 모르는** 상태였다.
- 사용자 영향: 런타임 동작 **불변**(시험만 늘었다). ★그 6개 가드가 나중에 사라지면 이제 **CI 가 말한다**.
- 검증: ★**개악 대조를 «자리마다»** — 가드를 하나씩 지워 **6/6 전건 red**. `cargo test --all` **554/0/1**(전체 실행) · DoD 7종 rc=0.
  ★**흔들림 0**: `SKIP`·`command -v`·OS 분기·절대경로 **0** · **3회 재실행 동일**.
- ★**정리를 시험의 일부로 만들었다**: 끝에서 `delete()` 후 `exists()` 를 단언하므로 스크래치 파일이 남으면 **시험이 진다**. 실행 후 잔여 **0**.
- 후속 추천: `test_class` 병렬화 시 스크래치 경로 격리(오늘은 단일 함수 순회라 무해 — ★그 «숨은 전제»를 기록해 둔다).
## [2026-09-12] 남은 20곳 «규격 삼분» — ⒜ 11 · ⒝ 1 · ⒞ 8, 그중 «미충족» 9곳만 닫았다 (rustjava-null-guard-spec-triage-for-constructor-and-collection-params)
- 무엇을: 감사가 남긴 20곳을 **JDK 규격 기준 세 갈래**로 가르고 ★**⒜ 이면서 런타임이 «미충족»인 9곳만** 닫았다.
  ★**주 산출물은 전수 분류표**(`docs/worklog/2026-09-12-null-guard-spec-triage.md`)이고 가드는 그 뒤다.
  회귀 잠금 = 픽스처 `test-data/NullSpecGuards`(6케이스). 채택 제안 `2026-09-11-null-guard-audit-and-io-buffer-guards#p0`.
- 왜: ★**null 이 «합법»인 자리에 가드를 넣으면 규격 위반이고, 도달 불가에 넣으면 죽은 코드다.**
  실제로 `URL(context, spec, handler)` 의 `handler` 는 참조 JVM 에서 **null 이 정상 통과**한다 ⇒ 손대지 않았다.
- 사용자 영향: 게스트가 `FileInputStream`·`ZipFile`·`readUTF` 등에 null 을 넘겨도 **에뮬레이터가 죽지 않고** `NullPointerException` 이 난다.
  정상 입력 동작 **불변** · ⒝⒞ 갈래 **무접촉**.
- 검증: ★**착수 «전»에 런타임을 먼저 쟀다**(사이트별 개별 픽스처 8회) — 6곳 **호스트 abort** ↔ `Pattern` 2곳 **이미 NPE**.
  ⇒ ★**후자는 가드를 넣지 «않았다»**(감사의 섀도잉 오탐 · 넣으면 죽은 코드). 가드별 커버리지도 단일 개악으로 **측정**: **6 red · 3 green**.
  DoD 7종 rc=0 · `cargo test --all` **554/0/1**(전체 실행).
- ★**규격 근거를 «관측»으로 댔다** — `AGENTS.md` 가 OpenJDK 소스 참조를 금지하므로 `src.zip` 을 **열지 않고** 참조 JVM(26.0.1) 거동을 쟀다.
  ★**대가**: 측정 판본 26 ↔ 타깃 8(가정을 명시했다).
- 후속 추천 2건: `ZipOutputStream` 최소 구현(미커버 가드 1개 잠금 + zip 왕복) · 추상 클래스 서브클래싱 픽스처(미커버 2개 잠금).

## [2026-09-11] null 가드 «감사» — 세고, 그중 데이터 전송 버퍼 18곳을 닫았다 (rustjava-null-guard-audit-remaining-runtime-entrypoints)
- 무엇을: 런타임 전체에서 `ClassInstanceRef` 인자를 받아 **곧바로 역참조하는** 진입점을 **셌고**
  (★**트리 병기 · 재측 2026-09-12** — base `6da7d66f`: **N 1,175 · M 52 · K 38** → head: **M 34 · K 20** · **ΔK 18**),
  그중 **`java/io` 데이터 전송 버퍼 + `String.getChars` 18곳**에 선행 회차와 **같은 모양**의 진입부 `is_null()` 가드를 넣었다.
  회귀 잠금 = 픽스처 `test-data/NullBufferGuards`(**13케이스** · 가드 **12/18** 커버). 채택 제안 `2026-09-11-null-guard-string-init-and-arraycopy#p0`.
- 왜: 선행 회차가 닫은 것은 **손으로 열거한** 9곳이었고, ★그 열거 자신이 `arraycopy` 의 `dest` 를 빠뜨렸다.
  ⇒ 이번엔 **열거가 아니라 술어로 셌다**. sink 는 `jvm.rs` 가 `&Box<dyn ClassInstance>`/`impl AsClassInstance` 로 받는 **17개 메서드**다.
- 사용자 영향: 게스트가 스트림·Writer·`String.getChars` 에 **null 버퍼**를 넘겨도 에뮬레이터가 죽지 않고 `NullPointerException` 이 난다.
  정상 입력 동작 **불변**(가드 앞에서 패닉하던 입력만 예외로 바뀐다) · 예외 종류·메시지 형태는 선행 회차와 동일.
- 검증: ★★**커버리지를 «추론»하지 않고 «측정»했다 — 가드 18곳을 «하나씩» 빼고 각각 빌드·실행**(2×18 회차).
  ⇒ **12 곳은 red(덮인다) · 6 곳은 green(★안 덮인다 — 전부 파일 핸들 필요분)**. 그 밖에 `getChars` **가드 위치** 원복 개악도 red.
  DoD 7종 rc=0 · `cargo test --all` **554/0/1**(★전체 실행 · 계수 불변이 맞다).
- ★★**계측기를 먼저 검증했고, 초판이 틀렸다**: 선행 9곳을 정답지로 대니 **3/9** 만 잡혔다(근인 = `"\n"+src` 에 정규식을 돌리고 인덱스는 `src` 에 쓴 **off-by-one**).
  고쳐서 **8/9**. 남은 1건은 helper **안**에서 deref 하므로 ★**이 계측은 절차간을 못 보고, 따라서 K 는 «하한»이다**(검수자의 「899/560 상한」과 방향이 반대다).
- ★★**[게이트② 정정 3건] 초판이 낸 수 중 «둘»이 틀렸고, 둘 다 «수를 섞은» 형태다.**
  ⑴**K 46** 은 base 가 아니라 **`eaad8e9c`**(두 회차 전) 트리의 수였다 — 라벨만 `6da7d66f` 였고, `46 − 18 = 28 ≠ 20` 이 그 증상이었다.
  ⇒ ★**base·head 를 나란히 적는 형태로 바꿨다.** ⑵**픽스처 12/18** 도 그 시점엔 **10/18** 이었고,
  ★**내가 적은 근인이 «뒤집혀» 있었다** — `InputStreamReader` 가 오버라이드해서가 «아니라» **오버라이드가 없어서** `Reader::read([C)` 로 해소된다
  (등재 서술자로 확인: `InputStreamReader` 는 `([CII)I` 만, `Reader` 가 `([C)I` 를 등재한다).
  ⇒ 그때 더한 중첩 클래스 2케이스는 **이미 덮인 가드를 또 덮었다** ⇒ **지웠고**, 대신 **3인자 호출**로 그 두 가드를 실제로 덮었다.
  ⑶`String.getChars` 가드가 **JDK 의 예외 선후를 뒤집고 있었다** ⇒ **범위 검사 뒤로 옮기고** 선후 잠금 케이스를 넣었다.
- ★**감사 스크립트를 커밋했다**(`scripts/audit-null-guards.py` · ★CI 미배선 = **잠금이 아니라 감사**) — 초판의 「배선 없는 검사기는 낡는다」를 뒤집는다.
  ★**근거는 이 회차 자신이다**: 작성자가 자기 수를 교차검증할 수단이 없어 ⑴이 났다.
- ★★**[게이트② R2] 그런데 그 커밋된 스크립트가 M 을 «2 낮게» 셌다** — `fn` 정규식이 **제네릭·`pub(crate)` 선언 34개를 못 봤다**.
  ★고쳤고 **세 트리 × 세 독립 구현이 전건 일치**한다(N **1,175** / M **60·52·34** / K **46·38·20**).
  ★★**「내 수가 맞고 검수 구현이 세부에서 다르다」던 전 회차 문안을 «철회»한다 — 틀린 것은 내 쪽이었다.**
  ★★**K 가 무사했던 것은 «설계가 아니라 운»이다** — 눈먼 fn 2건이 우연히 `as_proto` 미등재였을 뿐이고,
  등재된 제네릭 진입점이 **하나만** 있었어도 K 가 조용히 낮아져 ★**이 감사가 「닫혔다」고 답했을 것**이다. ⇒ 그 눈먼 구간을 docstring 에 박았다.
- ★**잔여 20건을 «일괄 가드»로 닫지 마라** — `URL(context, spec, handler)` 는 JDK 규격상 **null handler 가 합법**이다.
  ⇒ 후속의 본체는 「가드를 넣는 것」이 아니라 **「null 이 합법인지 규격으로 가르는 것」**이다.
- 후속 추천: `rustjava-null-guard-spec-triage-for-constructor-and-collection-params`(P3) — 잔여 20건을 **규격 기준으로 삼분**
  (NPE 의무 / null 합법 / 도달 불가)하고 첫 갈래만 닫는다.

## [2026-09-11] null 인자가 호스트를 죽이던 9경로에 가드 (rustjava-null-guard-string-init-and-arraycopy-p0)
- 무엇을: `String.<init>` 7개 오버로드와 `System.arraycopy`(`src`·`dest`)에 진입부 `is_null()` 가드를 넣어,
  null 을 넘겼을 때 **Rust 패닉(= 호스트 프로세스 abort)** 대신 **`NullPointerException`** 이 나게 했다.
  회귀 잠금 = 픽스처 `test-data/NullArgGuards`(9케이스). 채택 제안 `2026-09-11-s5-duplicate-issuance-stale-next-close#p0`.
- 왜: `ClassInstanceRef::deref` 가 `self.instance.as_ref().unwrap()` 이라 **null 이 닿는 순간 되돌릴 수 없다** —
  게스트 Java 코드의 흔한 실수가 JVM 전체를 죽였다. 가드가 **짝이 안 맞는** 것이 급소였다:
  `([CII)`·`(II[C)` 만 막혀 있고 `[B` 계열은 전부 뚫려 있었다.
- 사용자 영향: 에뮬레이터가 게스트의 null 실수로 **죽지 않는다**. 정상 입력의 동작은 **불변**(가드 «앞»에서
  패닉하던 입력만 예외로 바뀐다) · API 시그니처·의미 변경 **0**.
- 검증: ★**개악 대조** — 가드 전건 되돌리면 `test_class` **FAILED**(`Option::unwrap()` on `None` at
  `jvm/src/class_instance.rs:108`) ↔ 복원 시 **ok** ⇒ 픽스처가 공허하지 않다.
  DoD 7종 rc=0 · `cargo test --all` **554/0/1**(★`test_class` 가 픽스처를 한 함수로 순회하므로 **계수 불변이 맞다**).
- ★**제안의 「8경로」는 «9곳»이었다**: `System.arraycopy` 는 `src`·`dest` **둘 다** 뚫려 있었는데 STATE ③-2 가
  그것을 1건으로 셌다. ⇒ 다음에 그런 표를 쓸 땐 **인자 단위로** 세라.
- 후속 추천: `rustjava-null-guard-audit-remaining-runtime-entrypoints`(P3) — 이 회차는 **STATE ②가 열거한 범위만** 닫았다.
  같은 형태(`ClassInstanceRef` 를 받아 곧장 deref)가 런타임 전체에 몇 개나 남았는지는 **아무도 세지 않았다**.

## [2026-09-11] S5 중복 발권 판정 + 낡은 «다음» 절 폐쇄 (rustjava-upstream-sync-s5-java12-api)
- 무엇을: 티켓이 요구한 S5(`c4665b0`) 동기화는 **이미 착지돼 있었다** — `c4665b0` 은 `origin/main` 의 조상이고
  PR #21 이 `rustjava-upstream-sync-s5-with-remeasured-conflicts` 로 2026-09-03 에 `--merge` 착지했다(S6~S8 도 완주 ·
  behind **1** < 임계 20). ⇒ 티켓 대전제 ⓒ 경로로 **blocked** 종료. 이 회차의 실변경은 **문서 전용**:
  `STATE.md` 의 「다음은 S5」(8일 낡음)와 «PR 대기» 진행중 3건을 오늘 값으로 닫았다.
- 왜: 낡은 `## 다음` 이 LANE_IDLE 처방(「STATE.md 의 «다음»을 읽어라」)을 타고 **중복 발권을 실제로 만들었다** —
  STATE.md 자신이 경고한 형태(「이미 끝난 일을 가리키면 레인이 조용해진다」)의 두 번째 재현이다.
- 사용자 영향: 없음(코드 0줄). 다음 발권자가 같은 중복을 다시 밟지 않는다.
- 후속 추천: `rustjava-null-guard-string-init-and-arraycopy`(P2·S) 발권 — 「①의 뒤」 선행 조건이 충족됐고
  레인이 굶고 있다(케이스 8건·완료 정의는 STATE.md ③-2 에 이미 확정돼 있다).
- ★**[-fix · 게이트② F1·F2] 이 회차가 «자기가 고치는 병»을 밟았다** — 초판 PR #35 를 **2일 낡은 로컬 `main`**
  (`4959d0f3`)에서 잘라 `CONFLICTING` 이 됐고, done 이 `origin/main = 2cb03af7` 을 정확히 적고도 그 어긋남을
  화해시키지 않았다(`mergeable` 축 미조회 = **몰랐다**). 처방은 리베이스가 아니라 **`origin/main` 머지**(등재 repo · 계보 보존):
  #33·#34 의 STATE 16행·REPORT 39행을 **바이트 동일로 보존** 확인 후 해소. F2 = DoD 증빙이 «옛 파서»(조건부 기준)에서
  나왔던 것 — 머지로 현행 판본(도구 이름 기준)이 들어와 **재실행 rc=0**, 분류축 줄이 실제로 바뀌었다.
## [2026-09-08] 「모르는 도구」를 부르는 CI step 은 red 다 — ★등록 한 줄로 지나간다 (rustjava-parity-unknown-tool-step-red-decision)
- 무엇을: 채택 제안 `2026-09-08-parity-axis-a-tool-name#p0` 의 **결정 + 집행** 회차다. 세 선택지 중 ★**⒞ 조건부**를 골랐다 —
  `CHECK_TOOLS` 에도 `SETUP_TOOLS` 에도 «없는» 도구를 부르는 `run:` step 은 **FAIL**, 등록은 **한 줄**.
  ★**새 워크플로 0 · 새 잡 0 · `.rs` 0줄** — 판정은 기존 `dod_parity` 잡의 rc 에 얹었다.
- 왜: 선행 회차가 축 A 를 «도구 이름»으로 바꾸면서 ★**목록 «밖»은 «찍기»만 하게** 남겼다. `npm test` 한 줄이 들어와도
  rc=0 이라 ★**아무도 안 읽으면 아무 일도 일어나지 않는다** — 이 리니지가 다섯 번 낡은 방식이 정확히 그 형태다.
- ★★**수가 설계를 고쳤다 — 발권 소견의 전제가 반증됐다**: 소견은 「목록 밖이 **0** 이면 ⒞ 가 값한다」였는데
  ★**재보니 «1»이다**(`git config --global core.autocrlf false`). ⇒ 「목록 밖 = 즉시 red」(⒜의 소박한 형태)로 갔으면
  ★**착지 첫날부터 «옳은 워크플로»가 red** 다. 「목록 밖」은 한 가지가 아니라 **둘**(몰라야 할 셋업 ↔ 배웠어야 할 검사)이었고,
  그것을 가르는 것이 `SETUP_TOOLS` 다 ⇒ ★**⒞ 는 «⒜의 완화»가 아니라 «⒜가 성립하는 유일한 형태»다.**
- 사용자 영향: **없다**(런타임 무변경). 검사기 출력이 한 줄 늘었고(`셋업 등록분`), 실패 문면이 ★**등록 경로 2갈래를 그 자리에 찍는다**.
- 실측(개악 대조 12종 · 스크래치 클론 + 40자 핀 `f133cd30…`):
  ★**지정 개악 N1(`- run: npm test`) rc=0 → rc=1** · 원복 green · N2 `make` red · N3 `if:` 아래 숨김 red ·
  N6 **DoD 쪽 거울** red · N7 `sh -c` 겹 red(도구=`sh`) · ★**반대 개악 M2(cargo step 삭제) red** ·
  선행 회차 개악 M1b **여전히 red** · ★**등록 경로 «둘 다» 통한다**(N4 `SETUP_TOOLS` / N5 `CHECK_TOOLS`+DoD → 명령 7개) ·
  ★**위양성 0**(무해 편집 3종 green).
- ★**선행 술어 무손상 증명**: `git diff --numstat f133cd30 -- scripts/check-dod-ci-parity.py` = ★**`33  0`**(추가만 · 삭제 0).
- ★**내 결함 하나를 고쳤다**: 직전 회차(PR #33)가 §4 를 편집하며 `## 5. 단계 분할` **제목을 삼켰다**. 이 회차가 복구했다.
- 후속 추천: **없다**(천장 3건은 «알고 두는» 값으로 §4 ⑺ 에 적었다 — 발권 대상이 아니다).

## [2026-09-08] 파리티 락 축 A 를 «도구 이름»으로 분류한다 (rustjava-parity-lock-axis-a-classify-by-tool-name)
- 무엇을: 채택 제안 `2026-09-07-parity-per-repo-parser-axis-design#p0` 의 **집행 회차**다.
  `scripts/check-dod-ci-parity.py` 의 축 A 소속 판정을 **「`if:` 가 없는가」 → 「어느 도구를 부르는가」**
  (`CHECK_TOOLS = ("cargo", "python3")` · 도구 = 명령줄 첫 셸 낱말)로 바꿨다. ★**CI 쪽과 DoD 쪽에 «같은» 술어를 건다.**
  ★**새 워크플로·새 잡 0 · CI 를 막는 새 게이트 0 · `.rs` 변경 0.**
- 왜: `if:` 는 그 step 이 **무엇을 하는가와 무관**해서 «빠져나갈 문»이었다. ★**그 빠져나감의 형태를 실측했고,
  설계 문서의 문면이 한 걸음 짧았다** — `if:` 를 붙이기만 하면 **red 다**(DoD 쪽에 줄이 남는다).
  ★**green 이 되는 것은 «그다음 한 걸음», DoD 줄까지 지웠을 때**이고 그 조합이 오늘 **rc=0** 이었다.
  ⇒ `if:` 가 하는 일은 «green 만들기»가 아니라 ★**«DoD 줄 삭제를 허가하기»**다. 새 술어에서 그 조합은 **red** 다.
- 사용자 영향: **없다**(런타임 무변경). 검사기 출력이 한 줄 늘었다(분류축 명시) — 그리고 ★**제외 사유가
  «조건부라서»에서 «`git` 이라서»로 바뀌어**, 무엇을 왜 안 보는지가 **도구 이름으로** 찍힌다.
- 실측(개악 대조 10종 · 스크래치 클론 + 40자 핀 `4959d0f3…`):
  ★**지정 개악 M1b(`if:` + DoD 줄 삭제) rc=0 → rc=1** · 반대 개악 M2(cargo step 삭제) red ·
  M3(블록 스칼라 우회) red · ★**M4(python3 락 우회) green → red** · ★**위양성 0**(무해 편집 4종 전건 green).
  ★**빠진 것도 수로 적는다**: 축 A 원소 **6 → 6**(불변) · «제외» **1건 → 1건**(같은 step, 사유가 바뀌었다) ·
  ★**M1a(`if:` 만 붙임)가 red → green** — 이것이 유일한 «빠짐»이고 **위양성이었으므로 회수하지 않는다**.
- 후속 추천 1건: `#p0` 「모르는 도구(`npm`·`make`)를 부르는 CI step 을 «찍기»에서 «red»로 올릴지 결정」 —
  ★**새 게이트라 이 회차 범위 밖**이었다. 오늘 그 형상은 **0건**이지만, `if:` 구멍도 어제까지 0건이었다.

## [2026-09-07] 파리티 락의 repo 별 «파서 축» 설계 — ★공용 파서를 만들지 않는다 (rustjava-parity-lock-per-repo-parser-axis-design)
- 무엇을: 채택 제안 `2026-09-04-parity-sibling-repo-survey#p2` 의 **설계 회차**다. 형제 repo(`wie`·`qts`)를
  각 `origin/main` 에서 실측해 ⑴제안이 말한 «막는 것» 5축을 확인/반증하고 ⑵파서 축 ⒜~⒟ + 합격선 2개를 정하고
  ⑶repo 별로 «포팅/다른 형태/안 함»을 골랐다. 산출은 `docs/upstream-sync-approach.md` §4 의 설계 절 **하나**다.
  ★**형제 repo 파일 편집 0 · 검사기 본체 변경 0 · 발권 0**(설계까지가 계약이다).
- 왜: 「포팅 불가」는 «지금 검사기 그대로는»이라는 뜻이었고, **무엇을 바꾸면 성립하는가**가 미정이었다.
  재보니 ★**전제가 무너졌다** — `wie` 는 제안이 쓰인 **다음날(2026-09-05) 스스로 포팅했고**, 그 포팅이 ⒜~⒟ 를
  우리보다 낫게 풀었다(`if:` 가 아니라 **도구 이름**으로 분류 · 위치가 아니라 **이름 있는 마커** · `flatten_shell` 로 env 보존).
  `qts` 는 반대로 ★**집합 상등 락 자체가 틀린 도구**다 — `make test`↔CI 마커 3잡, `go` 의 로컬 조건부 skip↔CI 경성 게이트라
  **어긋남이 «정당»하고**, 상등을 요구하면 «옳은 문서»가 red 가 된다.
- 사용자 영향: **없다**(이 repo 코드 무변경). ★**이 저장소의 «잠복 위음성» 하나가 드러났다** — 축 A 를 `if:` 로 분류하는데
  지금 조건부 step 이 셋업(`git config …`)이라 **우연히** 옳을 뿐이고, 누가 `cargo test --all` 을 `if:` 아래로 옮기면
  축 A 에서 **조용히 빠지고 green** 이 된다. ★wie 는 그 이동을 **이미 한** repo 다.
- 후속 추천 3건(★이 회차는 발권하지 않는다 — 소관은 각 레인):
  ⑴`rustjava-parity-axis-a-classify-by-tool-not-conditional`(P2·이 repo) — 축 ⒜ 분류 교체 + 개악 대조
  ⑵`qts-ci-job-reachability-lock`(P3·qts·`depends_on: [qts-make-lint-add-ruff-format-check]`) — 상등이 아니라 «도달 가능성» 락
  ⑶`wie-parity-lock-multi-workflow-scope-decision`(P3·wie) — 천장 ③ 확대 여부(「넓히지 않는다」도 정답)

## [2026-09-05] `behind` 를 «재는» 예약 워크플로 신설 (rustjava-upstream-behind-measure-scheduled-workflow)
- 무엇을: `.github/workflows/upstream-behind.yml` **1개 신설**. 주 1회 upstream 을 **read-only fetch** 해
  `rev-list --count` 를 찍고 **Job Summary + annotation** 으로 보고한다. ★**코드(`.rs`) 0줄 · `scripts/` 무접촉.**
- 왜: 2026-09-04 판정이 트리거를 **`behind ≥ 20`** 으로 못박았는데 ★**그 수를 «재는 주체»가 없었다**
  (`rev-list --count` 를 가진 파일 **0건**). 임계만 있고 관측이 없으면 「아무도 안 챙긴다」가 그대로 남는다.
- 사용자 영향: 런타임·CI 게이트 **무변경**(새 워크플로는 예약 전용이라 PR 검사에 들어가지 않는다).
  ★**다음 동기 회차의 시점이 «사람 기억»에서 «주간 관측»으로 옮겨졌다.**
- 검증: ★**한 번 돌려** 오늘의 값을 냈다 — **behind `0`** · `merge-base` `bd42427` = upstream HEAD ⇒ 임계 미만.
  무회귀: `check-dod-ci-parity.py` **rc=0** · `check-worklog-json.py` **rc=0**.
- ★★**알림 방식을 «실측으로» 골랐다**(이 회차 결정의 대부분): ⒜red 는 여기서 **안 듣는다**
  (선례 `rust-audit` 최근 **20 run 중 19 failure** · ★**대응 티켓 0건** · `coverage` 리니지도 만성 red 를 «green 으로» 끝냈다)
  이고 예약 red 는 **main tip check-run 을 오염**시킨다(★단 **PR 은 막지 않는다** — 확인했다) ·
  ⒞이슈는 ★**이 fork 가 issues 비활성이라 불가** ⇒ ★**⒝ 가 «남은 것»이고 그 수동성은 재개 조건으로 잰다.**
- ★**후속 추천**: ⑴`behind ≥ 20` 이 **7일 이상**인데 회차가 안 열리면 **알림 방식을 다시 열어라**(세는 명령은 §5-B ⒠).
  ⑵★**임계 20 과 「발권은 사람이 한다」는 이 회차가 건드리지 않았다** — 바꾸려면 새 결정이다.

## [2026-09-05] 워크로그 «부재» 기계 강제 판정 — 넣지 않는다 (rustjava-worklog-absence-machine-enforcement-decision)
- 무엇을: 채택 제안 `2026-09-04-worklog-mandate-and-local-gate#p0` 에 대한 **판정**이다.
  ★**결론: 워크로그 «부재»를 잡는 기계 강제를 지금은 넣지 않는다** — `check-worklog-json.py` 의
  「존재하는 `.json` 만 검사한다」 설계와 «소급 금지»를 그대로 둔다. ★**검사기 무접촉 · `.rs` 0줄 · backfill 0**.
- 왜: ★★**의무화 회차가 «데이터를 보기 전»에 등록한 두 임계가 «둘 다» 발화하지 않았다.**
  그 문서가 정한 재측 시점(「2026-09-04 이후 열 번째 착지 회차」)에 ★**실제로 도달했고**(실측 **11**),
  ⒜ 미작성 **1 / 15**(임계 ≥2 미달 · ★그 1건은 규약보다 먼저 갈라진 PR #13 이라 **0 / 14 = 100%**) ·
  ⒝ 열린 카드 **13**(임계 <5 미달 — 의무가 카드를 **늘렸다**).
  ⇒ ★**데이터를 본 뒤 기계를 넣는 것은 «골대 옮기기»**다. 등록한 규칙을 지켰다.
- 사용자 영향: **없다**(CI·DoD·검사기 무변경). 회차 절차가 지금 그대로 유지된다.
- 검증: 창·분모를 밝혀 **전수**로 쟀고(`git rev-list --first-parent b3a4cf4..origin/main`),
  ★**정상 참작도 인용이 아니라 실측**했다(PR #13 `createdAt` 2026-08-23 ↔ 규약 착지 2026-08-26).
  ★**술어를 둘로 재서**(느슨/엄격) 같은 **14** 임을 확인했고, ★**문서에 박은 재개 조건 명령을 실행해 값 `1` 을 확인**했다.
  무회귀: `check-worklog-json.py` **rc=0** · `check-dod-ci-parity.py` **rc=0**.
- ★**약점을 숨기지 않는다**: 「100%」는 **단일 집행 주체**의 습관일 수 있고(리니지 13개지만 git author 는 하나),
  정본 술어는 **느슨**하며, ★**실패는 여전히 조용하다** ⇒ 「위험이 없다」가 아니라 「임계가 아직 발화하지 않았다」이다.
- ★**후속 추천**: ⑴재개는 **엄격 술어 값이 2 이상**일 때만(오늘 **1**) ⑵재개 시 설계는 이미 적어 뒀다 —
  ★**baseline sha 를 쓰지 말고 «PR diff 로 조건부» 판정**하라(의무가 조건부이므로 검사도 조건부여야 한다).

## [2026-09-04] upstream 동기 «정기 축» 판정 — 격차 기반 `behind ≥ 20` (rustjava-upstream-sync-cadence-decision)
- 무엇을: 채택 제안 `2026-09-04-sync-contract-stale-assets-decision#p0` 에 대한 **판정**이다.
  ★**결론: 동기 회차는 `origin/main..upstream/main` 이 «20 이상»일 때 연다** — 시간(격주·월간) 기반은 기각.
  ★**신설 0**(검사기·크론·워크플로) · `.rs` **0줄** · ★**S9 미개시**(behind **0**). 바뀐 것은 `docs/upstream-sync-approach.md` **§5-B** 뿐이다.
- 왜: ★★**전제를 «먼저» 재서 «반증»했다.** 회차 커밋 수 ↔ 충돌의 상관계수 ★**부호가 «음»**이다.
  ★★**[게이트② 정정] 초판 표는 base·(델타/누적) 병기가 없어 기준이 섞여 있었다** ⇒ 정본을 **«누적»**으로 통일해 다시 계산했다:
  A 초판(혼재) **r=−0.169 · 합 28** · B S4 만 정정 **r=−0.134 · 합 30** · ★**C 정본 «전부 누적» r=−0.155 · 합 33**.
  ★**세 열 모두 부호가 음**이고 「많은4 < 적은4」가 유지된다(정본 C: **37커밋 → 14** ↔ **8커밋 → 19**) ⇒ ★**판정 불변**.
  ★**단 r 은 n=8 비무작위 표본이라 «비례하지 않는다»는 «부호 판정»으로만 쓴다.**
  ★**누적 충돌도 포화한다**(고정 base: **7커밋 16 → 33커밋 19**) ⇒ ★**충돌을 만든 것은 «양»이 아니라 «무엇이 왔는가»**다.
  ★**그래도 「아무것도 안 한다」로 가지 않았다** — 반증된 것은 «비용이 격차에 비례한다»는 기전이고,
  제안이 지목한 «아무도 챙기지 않는다»는 위험은 그대로 참이라서 **축은 두되 임계를 포화점 뒤**에 놓았다.
- 사용자 영향: 런타임 **무변경**. ★**다음 동기를 «언제» 여는지가 처음으로 정해졌다** — 그 전엔 아무 기준이 없었다.
- 검증: behind **0**(요구값) · `merge-base` `bd42427` = upstream HEAD · S1~S8 회차별 커밋/충돌 전수 ·
  게이트② 사이클 전수(`reports/*.review.md` 줄1 — 8회차 중 **7회가 1사이클**) ·
  upstream 12개월 **144커밋/51주**(주당 중앙 **1** · ★**0인 주 37%**) · 도달 시간 behind 20 = **중앙 44일**
  (★**상수로 인용하지 마라** — 게이트② 재측 **41일** · 차이는 **창 정의**에서 온다 ±3일 · **결론 영향 0**).
- ★**비용**: 회차당 새 충돌 **중앙 2.5**(0~9) · 티켓 **3건** · 연 **6~8회** ⇒ 연 **18~24 티켓**.
- ★**방아쇠**: **기계가 «재고» 사람(총괄)이 «발권»** 한다. 재는 자리 권고 = 이 repo 의 **예약 워크플로**
  (선례 `rust-audit.yaml` 이 이미 `schedule: cron` · upstream 은 **read-only fetch** 라 발신 0). ★**구현은 별건.**
- ★**후속 추천**: ⑴behind 를 «재는» 워크플로 1개(발권은 사람 유지) ⑵★**「8회차가 들었다」를 «격차 33 의 비용»으로 인용하지 마라** —
  그 덩어리의 상당 부분은 **스쿼시로 계보가 접힌 비용**이고 `merge_strategy: merge` 로 이미 닫혔다.
## [2026-09-04] 형제 repo 파리티 락 필요성 조사 — 둘 다 «조건부 필요», 검사기는 포팅 불가 (rustjava-dod-ci-parity-sibling-repo-survey)
- 무엇을: 채택 제안 `2026-09-04-dod-ci-parity-lock#p1` 에 대한 ★**읽기 전용 조사 + 판정**이다.
  `wie`·`qts` 의 **DoD 정본 위치 · CI 검사 집합 · 대칭차**를 각각 실측하고, ★**이력으로 실익까지** 쟀다.
  ★**형제 repo 변경 0**(`git`·PR·파일 수정 전부 0 — `gh api /contents` 로만 읽었다) · 구현 **0**.
- 왜: ★**같은 처방이 그대로 맞는지 «먼저» 재라**는 것이 제안 문면이었고, 재보니 ★**맞지 않았다.**
  `wie` 는 DoD 정본이 **`AGENTS.md`** 이고 CI 의 4번째 게이트(`cargo test`)가 ★**`if:` 로 갈린 2 step + 블록 스칼라**라
  우리 파서의 「조건부 = OS 축 = 제외」 규칙이 ★**거짓 red** 를 만든다. `qts` 는 DoD 가 ★**`make` 타깃 이름**이라
  Makefile 간접층이 있고, ★**toolchain 매트릭스가 없어 축 B 가 성립하지 않으며**, `gitleaks` 는 ★**action 이라 못 본다**.
- 사용자 영향: **없다**(이 repo 무변경). ★**형제 repo 에 «측정된» 개선 경로 둘이 생겼다** —
  `wie` 이력 **9/34(26%)** · `qts` 이력 **10/60(17%)** 이 각각 «한 줄»로 로컬에 들어온다.
- 검증: `wie` `rust.yml` 최근 200 run(성공 166 · 실패 34) **실패 전수 분해** — stable-only clippy 24 ·
  ★**beta-only clippy 9** · windows test 1. `qts` `ci.yml` 최근 실패 **60건 전수 분해** —
  `ruff format --check` 포함 38 · ★**그것만 10** · `ruff check` 7 · phase0-only 0.
  ★문서 실측: 문자열 `beta` 가 `wie` 규범 문서에 **0건** · 문자열 `format` 이 `qts` 규범 문서에 **0건**.
- ★★**일반 사실 하나**: 세 repo 가 **전부** 이 어긋남을 갖고 있었고 ★**어긋난 자리가 전부 규범 문서에 «적혀 있지 않았다»**
  ⇒ ★**「사람이 문서를 최신으로 유지한다」가 세 repo에서 «각각» 실패했다** — 기계 대조의 일반 근거다.
- ★**후속 추천**: ⑴`wie` four gates 에 `cargo +beta clippy --all -- -D warnings` ⑵`qts` `make lint` 에
  `uv run ruff format --check .` ⑶★**락 포팅은 그 «뒤»**(락은 어긋남을 «막는» 것이지 «고치는» 것이 아니다).
  ★셋 다 **그 repo 레인 소관**이라 여기서 고치지 않았다 — 축과 합격선만 워크로그에 적었다.

## [2026-09-04] 파리티 락 «교차곱» 확장 판정 — 넓히지 않는다 (rustjava-dod-ci-parity-cross-product-decision)
- 무엇을: 채택 제안 `2026-09-04-dod-ci-parity-lock#p0` 에 대한 **판정**이다. ★**결론: 교차곱으로 넓히지 «않는다»**
  (부분 확장 `fmt@beta` 만도 기각). ★**검사기 로직 변경 0 · `.rs` 0줄 · `rust.yml` 무접촉 · DoD 블록 무접촉** —
  바뀐 것은 `docs/upstream-sync-approach.md` §4 의 판정 절과 검사기 **docstring 1블록**뿐이다.
- 왜: ★**비용과 실익을 «각각» 실측했다(추정 0).**
  **비용** — 무변경 재실행 19s → 38s(**2.00×**) ↔ ★**소스 1줄 편집 후 89s → 326s(3.66×)**.
  제안 문면의 「두 배」는 «무변경»에서만 참이고, 회차는 언제나 «편집 후»에 돌린다. 지배항은
  `cargo +beta test --all` **215s**(같은 편집에서 stable test 66s 의 3.3배).
  **실익** — CI 에만 있는 조합은 **3개**로 비어 있지 않지만, ★★**`rust.yml` 이력 76 run(성공 73 · 실패 3)
  전수 분해에서 그 3개가 «새로» 잡았을 사건은 0건**이다. 이 저장소의 beta 전용 실패는 **전건 `cargo clippy --all`**
  이고 그 한 줄은 DoD 가 이미 `+beta` 로 덮는다. ⇒ **+237s/회차를 내고 얻는 것이 0.**
- 사용자 영향: **없다**(런타임·CI·DoD 무변경). 회차 소요가 지금 그대로 유지된다.
- 검증: 벽시계 실측 3세트(warm · 편집 후 · 콜드) · CI 이력 전수 분해 · 재개 조건 명령을 실행해 **오늘의 값 0** 확인 ·
  측정용 소스 편집은 `git checkout` 으로 복구 · `check-dod-ci-parity.py` **rc=0**(대칭차 0 유지).
- ★**재고 나서야 보인 것 둘**: ⑴`rustfmt` 가 **beta 에 미설치**라 `cargo +beta fmt` 는 오늘 그대로 **rc=1** —
  교차곱을 채택했다면 모든 머신이 `rustup component add` 를 선행해야 했다 ⑵★**툴체인 교대 축출은 «없다»** —
  「두 배」의 흔한 기전(서로 캐시를 밀어낸다)이 이 저장소엔 없고, 대가는 시간이 아니라 **디스크**다(`target` 46G).
- ★**후속 추천**: ⑴재개 조건이 발화하면 **그 step «하나만»** 넣고 비용표를 **다시 재라**(이 표는 «그때의 값»이다).
  ⑵★「비용이 싸졌다」는 재개 사유가 **아니다** — 실익이 0 인 동안에는 싸도 넣지 않는다.

## [2026-09-04] 로컬 DoD ↔ CI 매트릭스 «기계 대조» 신설 (rustjava-local-dod-vs-ci-matrix-mechanical-check)
- 무엇을: `scripts/check-dod-ci-parity.py` 신설 + `rust.yml` job **`dod_parity`** 배선 + DoD 블록에 그 줄 추가.
  ★**`CLAUDE.md` DoD 코드블록**과 **`.github/workflows/rust.yml`** 을 «각각 파싱해» **대칭차**를 낸다 —
  축 A(명령 집합) · 축 B(toolchain 집합). 오늘 **둘 다 0** 이라 계약대로 **rc=1(막는다)** 로 켰다.
- 왜: `…-going-stale` 리니지가 **다섯 회차** 내내 「전건 동기」를 주장했고 **매번 «또 한 자리»**가 나왔다
  (게이트②에서 6건째). ★**전부 같은 종류** — 「로컬 DoD 가 CI 검사 몇 개를 재현하는가」가 `rust.yml` 과
  어긋난 채 문서에 굳었다. ⇒ ★**사람 손 대조로는 다섯 번 실패했다.**
  ★★**그런데 «낡은 문자열 스캐너»는 만들지 않았다** — 게이트② 검수자가 「F1 은 «수»가 아니라 «말»이라
  문자열 검사기로도 안 잡힌다」를 실측으로 세웠기 때문이다. ⇒ 문서의 문장이 아니라 ★**문서가 틀리는 «원인»**을 잡는다.
- 사용자 영향: 런타임 동작 **무변경**(`.rs` 변경 0). 회차가 DoD 를 축약하거나 CI 가 검사·매트릭스 차원을
  늘렸는데 DoD 를 안 고치면 ★**그 PR 이 그 자리에서 red** 다 — 종전엔 «사람이 축을 돌려야» 드러났다.
- 검증: ★**개악 5건 전건 red · 무개악 green** — ⒜DoD wasm32 줄 제거 ⒝CI `- run: cargo doc --no-deps` 추가
  ⒞매트릭스 `nightly` 추가 ⒟DoD `+beta` 줄 제거 ⒠`rust-toolchain.toml` 신설(축 B 매핑 붕괴 가드).
  DoD 7줄 전건 rc=0 · `cargo test --all` **554 / 0 / 1**(새 red 0).
- ★**한계를 숨기지 않는다 — 검사기가 «그 자리에서 함께» 찍는다**: ⒜**OS 축**(조건부 step)은 로컬 재현 불가라
  여전히 CI 가 유일한 그물 ⒝두 축을 **«교차곱»으로 보지 않는다** — CI 는 cargo 검사 4종을 stable·beta 둘 다
  치는데 DoD 는 `clippy` 만 이중이다. ★**이것은 2026-09-04 결정이 «고른 값»**이고(로컬 `cargo +beta test` =
  두 번째 toolchain 전면 재빌드 · lint 를 지는 축은 clippy 뿐), 넓히는 것은 «새 결정»이다.
- ★**후속 추천**: ⑴교차곱(모든 검사 × 모든 toolchain)까지 넓힐지 — ★**비용을 먼저 재고** 결정하라.
  ⑵같은 형태의 파리티 락이 형제 repo(wie·qts)에도 필요한지 판정.

## [2026-09-04] 「우리 자산이 낡는다」 상시 조항 판정 — 넣지 않고 «구멍 하나»를 막았다 (rustjava-sync-contract-standing-clause-for-our-assets-going-stale)
- 무엇을: 운영자 채택 제안 ★**셋**(`2026-09-04-upstream-sync-s6#p1` · `…-s7#p1` · ★`…-s8#p0`)에 대한 **결정**이다.
  (★**[fix3 정정] 초판은 「둘」** — 같은 제안의 세 번째 판이 도착해 `adoptedProposals` 는 **세 ref** 다.)
  ★**결론: 상시 조항을 «넣지 않는다».** 대신 ★**로컬 DoD 가 CI 의 «매트릭스»를 재현하지 않던 것**을 고쳤다 —
  `CLAUDE.md` DoD 가 이제 **CI 명령 5줄 + toolchain 축 1줄 = «6줄»**을 축약 없이 싣는다.
  ★**코드(`.rs`) 변경 0** · 규범 문서 **2파일**(`CLAUDE.md` · `docs/upstream-sync-approach.md`) + 기록 문서 4
  (`REPORT.md` · `STATE.md` · 워크로그 `.md`/`.json`).
  ★**[게이트② 정정] 초판은 「CI 검사 5종 중 wasm32 clippy 한 줄」이라 적었다** — 계수 «1» 시절 문면이고,
  계수 **2** 정정 후에는 **두 축**(target · toolchain)이라 DoD 도 **6줄**이다.
  ★★**[후속 정정] 그 «6줄»도 낡았다 — 파리티 검사기가 더해져 «7줄»이다.**
  ⇒ ★**줄 수는 이제 문장이 아니라 `scripts/check-dod-ci-parity.py` 가 센다**(CI job `dod_parity`).
- 왜: ★**목록 문서화로 시작하지 않고 «먼저 셌다»**(티켓이 그렇게 요구했다). 우리 자산 **8건**을
  ★**돌연변이로 깨뜨려** 무엇이 잡는지 실측했다 — 경로 문자열·`setProperty` 서술자·charset 라우팅·
  `ClassFormatError` 종류 단정은 **`cargo test` RED**, 수동 span 은 **clippy RED**,
  `double_must_use` allow 는 **깨져도 무해**, 워크로그 스크립트는 **로컬 DoD 와 CI 둘 다** 잡는다.
  ⇒ ★★**[게이트② `request-changes` 정정] 「아무것도 없음」은 «2개»다**(② CI `--exclude` · ⑦ `double_must_use` allow).
  초판이 ⑦을 «비하중» 자리에서 재 「무해」로 적었으나, ★**9곳 전건 삭제로 재측정하면 stable 0 · ★beta RED**(rc=101 · 진단 **6**건)
  ⇒ ⑦의 그물도 **CI 만**이다. ⇒ ★**「1개면 그 하나를 고쳐라」 규칙은 적용되지 않는다.**
  ★★**게다가 그 1개도 «조용히» 실패하지 않는다** — cargo 가 `warning: excluded package(s) … not found`
  를 찍고 빌드가 깨진다 ⇒ ★**문제는 «침묵»이 아니라 «늦음»**(push 후 CI 에서만)이고,
  ★**근인은 「자산 목록이 없다」가 아니라 «로컬 DoD 가 CI «매트릭스»를 재현하지 않는다»** 였다 —
  ②는 빠진 `- run:` 줄(**target** 축) · ⑦은 빠진 **toolchain** 축(beta) ⇒ ★**한 근인의 두 얼굴**이다.
  ⇒ ★**결론은 그대로 「넣지 않는다」이나 논거가 바뀌었다**: 근인 하나를 고치면 둘 다 그물을 얻는다
  (DoD 에 `cargo +beta clippy` 를 넣자 ⑦이 **로컬에서 RED**(rc=101 · 진단 **6**건)로 잡힌다 · 실측).
- 사용자 영향: 런타임 동작 **무변경**. 회차가 로컬에서 CI 와 **같은 6줄**(★**toolchain 축 포함**)을 돌리게 되어
  ★**「로컬 green 인데 CI red」 부류가 이 축에서 사라진다**(S5·S7·S8 이 실제로 그 부류였다).
- 검증: **DoD 6줄을 문면 그대로** 실행 — 전건 **rc=0**(★`cargo +beta clippy --all -- -D warnings` **rc=0** 포함) ·
  `cargo test --all` **554 / 0 / 1**(baseline 동수 · 새 red 0). ★**돌연변이는 전부 복구**했고 `.rs` 변경은 **0**이다.
- ★★**사료 — 「세 번」·「두 방향」을 확정하되 결론은 «한 조항으로 못 덮는다»**:
  S3(문구 단정 3건 · 테스트) · S5(io 5곳 · 테스트) · S6(regex 3곳 · 정독) = **⑴신규/판본 교체** ·
  S7(우리 테스트 5곳 · **컴파일**) = **⑵공용 API 변경** · S8(`rust.yml`·경로 4곳 · **CI만/테스트**) = **⑶개명**.
  ⇒ ★**잡는 그물이 각각 다르므로**, 조항 하나로 덮으면 **이미 그물이 있는 여섯 자리에까지 사람 확인을 얹게 된다**
  (★**세는 법**: 돌연변이 표 **8행** − 그물이 「CI 만」인 **2행**(② · ⑦) = **6** · 명령은 `docs/upstream-sync-approach.md` §4.
  ★**[게이트② 정정] 초판의 「다섯」은 계수 «1» 시절의 파생 수였다** — 계수가 2 가 되며 **6**이 맞다).
- ★★**재개 조건**(결정에 재개 조건이 없으면 «영구 종결»로 읽힌다):
  「**CI 가 치는 검사 중 로컬 DoD 에 없는 것이 1건이라도 생기면 다시 연다**」 ·
  ★**세는 명령을 `docs/upstream-sync-approach.md` §4 에 박았고, 정정으로 «축이 둘»이 됐다**
  (축① `- run:` 줄 · ★축② **toolchain 매트릭스**) · ★**오늘의 값 = 축① 0 · 축② 0**(착수 시 축① 5 · 축② 1).
  ★세 돌연변이로 **각 축이 «자기 자리»에서만 반응**함을 실측했다.
- ★★**[정정] 초판 후속 추천 3번을 «철회한다»** — 「`double_must_use` allow 가 이제 지워도 통과한다」는
  **비하중 돌연변이에서 나온 거짓**이다. ★**9곳을 전건 지우면 beta clippy 가 RED 다**(rc=101 · 진단 **6**건) ⇒ ★**지우지 마라.**
- 후속 추천: ⑴**게이트③은 `--merge`**(등재 repo). ⑵★**upstream 동기 «정기 축» 판정은 아직 열려 있다**
  (S8 워크로그 `proposals[1]` · behind 0 인 지금이 적기다) — 이 회차는 그것을 «건드리지 않았다».
  ⑶★**C7 고지**: DoD 블록 «개악»과 **OS 축 3종**은 이 대조가 못 잡는다 — 회차가 두 축을 «실제로 돌렸는지» 적어라.

## [2026-09-04] upstream 동기 S8 — 컷 `bd42427` 개명 스윕 · ★**behind 0** (rustjava-upstream-sync-s8-rename-sweep-decision)
- 무엇을: upstream `bd42427` 까지 **12커밋**(crates.io 공개 준비 개명 스윕)을 `--merge` 로 흡수하고 충돌 **8건**을 해소했다.
  ★★**`merge-base` `ba5797b` → `bd42427` · behind `12` → ★`0`** · 부모 **2개**
  ⇒ ★★★**upstream 을 «완전히» 따라잡았다**(2026-08-16 계획 착수 시 behind 33 → 오늘 0).
- 왜: 운영자 채택 제안 `2026-09-03-upstream-sync-s5-s7-remeasure#p0` — 「개명이 우리 픽스처를 **조용히 지울 수 있다**」.
  ★**이 회차의 산출은 코드가 아니라 «판정»**이었다: 어느 픽스처가 어디로 가는가.
- 사용자 영향: **크레이트 이름이 공개용으로 바뀐다**(`java_runtime`→`rustjava-runtime` · `jvm_rust`→`jvm-bytecode` ·
  `java_class_proto`→`jvm-class-proto` · `java_constants`→`jvm-types` · `test_utils`→`test-utils` ·
  `test_data/`→`test-data/`). ★**런타임 동작 변경 0** · 우리 자산 전수 생존.
- 검증: stable 4종 + ★beta 2종 rc=0 · `cargo test --all` **554 / 0 / 1** · beta 도 **554 동수** ·
  ★증감 0 이고 그것이 맞다(upstream 테스트 함수도 **547 → 547** — 개명 스윕이라 추가 0) ·
  `#[ignore]` **1 → 1** · 우리 테스트 함수 **558 → 558**.
- ★★**두려워한 형태(modify/delete)가 «0건»이었다.** git 이 `CONFLICT (file location)` + `AU` 로 처리해
  우리 고유 **6건**을 새 경로로 **이미 옮겨** 두고 «이동 확인»만 요구했다 ⇒ 처분은 **「간다」 전건**이고
  ★**`origin/main` 블롭 ↔ 새 경로 해시가 6건 전부 동일**(바이트 보존) · ★**픽스처 수 5 → 5**.
  ★「남는다/버린다」는 **0** — 여섯 다 우리 것이고 upstream 판본으로 **대체된 것이 없다**.
- ★★**대신 개명이 «우리 자산 2건»을 낡게 만들었다 — 「우리 자산이 낡는」 5회째이고 «대상»이 처음이다**:
  ⑴`.github/workflows/rust.yml` 의 `--exclude test_utils` 가 **옛 크레이트 이름**이라 wasm32 셀이 깨진다
  (★실측: 옛 이름 **rc=101** ↔ 새 이름 **rc=0**) ⑵`tests/test_class_format.rs`(PR #3)의 `"test_data/"` **4곳**.
  ★**둘 다 «우리 파일»이라 충돌이 «날 수가 없다»** — 개명 대응표를 그대로 적용했고 upstream 코드는 무접촉.
  ⇒ ★**이제 이 형태의 방향이 셋이다**: ⑴upstream 신규 파일 ⑵공용 API 변경 ⑶★**개명**.
  ★**⑶은 컴파일이 «절반만» 잡는다** — 경로 문자열은 런타임, CI 설정은 **CI 에서만** 드러난다.
- ★**`Cargo.lock` 하강을 차단했다**(계약6⒞가 «또» 잡았다): `--theirs`+build 가 `tracing` **0.1.44 → 0.1.41** ·
  `tracing-subscriber` · `syn` 을 내렸다. ★**`tracing` 하강은 PR #4(언프리즈)를 되돌리는 것**이라
  S5 와 같은 처방(`origin/main` lock 에서 재생성)으로 ★**내려간 것 0 · `tracing` 0.1.44 유지**.
- 후속 추천: ⑴**게이트③은 반드시 `--merge`**(등재 repo). ⑵★**behind 0 이 됐으므로 다음 동기 회차는
  «upstream 이 움직일 때»다** — 정기 축이 필요하면 별건 판정. ⑶★**「우리 자산이 낡는」 축의 계약화**가
  이제 **다섯 회차 미처분**이고, 이번에 **세 번째 방향(개명)**이 드러났다.

## [2026-09-04] upstream 동기 S7 — 컷 `ba5797b` 머지 + §5 서식에 «델타/누적» 축 (rustjava-upstream-sync-s7-and-fix-the-conflict-count-format)
- 무엇을: upstream `ba5797b`(#201 가상 디스패치 해석 · **1커밋** · 319파일 +20,118/−5,729)을 `--merge` 로
  흡수하고 충돌 **1건**을 해소했다. 함께 ★**§5 「충돌 수」 정본 서식을
  `<수> <델타|누적>(base <sha> · merge-base <sha>)` 로 고쳤다.**
  ★**`merge-base` `95ebc5c` → `ba5797b`** · behind **13 → 12** · 부모 **2개** ⇒ ★★**계획 7회차(S1~S7) 완주.**
- 왜: ⑴S6 이 「예측 0 ↔ 실측 1」로 갈렸고 검수자가 근인을 **서식의 구멍**으로 지목했다 —
  base 만 병기하면 ★**「3 → 3」과 「0」이 둘 다 규칙을 지킨다.** ⑵S7 이 충돌 수를 «새로» 적는 회차라
  ★**그 서식을 이번에 고치는 것이 가장 쌌다**(같은 순간에 만난다).
  ★**착수 재측정은 신 서식으로 적었다**: 컷 `ba5797b` = **누적 1 · 델타 +1**(base `3e02f8c` · merge-base `95ebc5c`).
  ★**「예측대로였다」가 아니라 「재서 1 이었다」.** `string.rs` 는 집합에서 **빠졌다**(S6 해소 뒤 무접촉)
  ⇒ ★**누적은 줄어들 수도 있다 — 그래서 누적을 델타로 대신할 수 없다.**
- 사용자 영향: ★**가상 메서드 디스패치 해석이 JVM 규격에 맞게 고쳐진다** — `invoke_virtual` 이 «선언 클래스»를
  받아 상속·오버라이드 해석이 정확해지고, 전 클래스에 **접근 플래그**(`PUBLIC`/`PRIVATE`)가 부여된다.
  ★**우리 자산 변경 0**(charset 라우팅 7곳 · `setProperty` 서술자 · 수동 span · `double_must_use` allow 7곳 · 픽스처 4).
- 검증: stable 4종 + ★beta 2종 rc=0 · `cargo test --all` **554 / 0 / 1** · beta 도 **554 동수** ·
  ★**시험 수 증감 0 이고 그것이 맞다** — upstream 테스트 함수도 `95ebc5c` **547** → `ba5797b` **547**
  (78개 테스트 파일을 +14,730/−3,536 로 만지지만 **시그니처 스윕**이지 추가가 아니다) ·
  ★약화 0: `#[ignore]` **1 → 1** · 우리 테스트 함수 **558 → 558** · 단언 삭제 **0** ·
  「해소분 0」 = `ba5797b` 대비 **삭제 파일 0** · 다른 파일 **52건 전수 우리 자산** ·
  `Cargo.lock` **내려감 0 · 올라감 0 · 추가 0 · 제거 0**.
- ★★**`thread.rs` 는 «직교»다 — 「어느 쪽이 이기나」가 아니다.** upstream(**+23/−10**)은 `invoke_virtual` 에
  «선언 클래스» 인자를 더했고, 우리(**+50/−42** · ★**의미 변경은 3줄**이고 나머지는 들여쓰기)는
  그 호출들을 **감싸는 수동 span**(PR #4 · `#[tracing::instrument]` 금지)이다 ⇒ 의미가 겹치지 않는다.
  우리 구조를 뼈대로 upstream 새 인자 **3곳**을 얹었다(S1·S3·S4 와 같은 전략 · **4회째**).
- ★★**「충돌 0으로 들어온」 파손 «4회째» — 그런데 축은 «처음»이다.** upstream 이 **공용 API 시그니처**를 바꾸자
  ★**우리 고유 테스트 5곳**(PR #5 자산)이 구식 4인자로 남아 **컴파일 실패**(`E0061`×5).
  ★**우리 줄이라 upstream 이 안 건드렸고 ⇒ 충돌이 «날 수가 없다».**
  ⇒ ★**S3·S5·S6 과 «반대 방향»이다**(그쪽은 upstream 신규 파일이 우리 규격을 안 지킨 것) —
  ★**그래서 「신규 파일을 훑는다」로는 못 잡고, 이번에 잡은 것은 «컴파일»이다.**
  처분은 upstream 자신의 관용구를 **그대로 채택**(`&x.class_definition().name()` · `"java/lang/String"`) · 단언 무접촉.
- 후속 추천: ⑴**게이트③은 반드시 `--merge`**(등재 repo · `merge_strategy: merge` 필수).
  ⑵★**S8** — 남은 **behind 12** 가 «개명 스윕»(`java_runtime/`→`rustjava-runtime/` · `test_data/`→`test-data/`)이고
  우리 픽스처에 꽂힌다. 총괄 보류분 `2026-09-03-upstream-sync-s5-s7-remeasure#p0` — **이 회차는 집행하지 않았다.**
  ⑶★**「우리 자산이 낡는」 축을 계약에 넣을지 판정하라** — 이번 4회째로 **두 방향이 다 확인됐다**
  (upstream 신규 파일 ↔ upstream 시그니처 변경). 후자는 컴파일이 잡지만 **전자는 안 잡는다.**

## [2026-09-04] upstream 동기 S6 — 컷 `95ebc5c` 머지 (rustjava-upstream-sync-s6-cut-95ebc5c)
- 무엇을: upstream `95ebc5c`(**11커밋** · 142파일 +17,593/−483)을 `--merge` 로 흡수하고 충돌 **1건**을 해소했다.
  ★**`merge-base origin/main upstream/main` `c4665b0` → `95ebc5c`** · behind **24 → 13** · 머지커밋 **부모 2개**.
- 왜: 운영자 채택 제안 `2026-09-04-upstream-sync-s5#p0`. ★**「새 충돌 0」을 «전제»로 쓰지 않고 다시 쟀다.**
  ★★**그 0 은 «델타»였다 — 「풀 것이 없다」가 아니다.** 옛 base `8c1238b` 에서 누적 3 → 3(새로 나타난 파일 0)이고,
  새 base `a0b5d3c`(merge-base `c4665b0`)에서 **누적 1**이다. 둘 다 참이다.
  `string.rs` 는 S5 의 설계 판단으로 우리 분기(**+8/−28**)가 남아 upstream 이 그 파일을 만지는 한
  (이 구간 **+402/−121**) 계속 열린다. ⇒ ★**§5 에 한 줄 보탰다: base 와 «함께» «델타/누적»도 밝혀라.**
- 사용자 영향: **`java.util.regex`(Pattern·Matcher)·`Formatter`·`Locale`** 이 들어온다 —
  `String.format`·정규식 API 가 처음으로 동작한다. ★**우리 자산 변경 0**(charset 4종 · `setProperty` 서술자 ·
  수동 span · `ClassFormatError` 분류 · 픽스처 전건 생존).
- 검증: stable 4종 rc=0 · ★**beta 2종 rc=0**(S5 가 물린 자리를 미리 확인) ·
  `cargo test --all` ★**554 passed / 0 failed / 1 ignored**(S5 427 → **+127** · 새 red 0) · beta 도 **554 동수** ·
  「해소분 0」 = **`95ebc5c` 대비 삭제 파일 0** · 다른 파일 **50건 전수가 우리 fork 고유 자산**.
- ★**해소**: `string.rs` 충돌면은 **import 한 곳**뿐이라 **합집합**으로 풀었다 —
  우리 `charset::Charset` + upstream 의 재구조화 `classes::java::{lang, util::{Formatter, Locale, regex}}`.
  `Charset` 라우팅 **4곳 생존** · ★**S5 가 버린 `decode_str`/`encode_str` 재유입 0**.
- ★★**계약4⒝ 정독이 「충돌 0으로 들어온」 파손 1건을 «테스트를 돌리기 전에» 잡았다 — 이 형태 «세 번째»다.**
  upstream 이 이 구간에 **새로** 넣은 `java/util/regex/test_pattern_syntax_exception.rs` 가
  `System.setProperty` 를 `)Ljava/lang/Object;` 로 **3곳** 부른다(우리는 PR #5 에서 JDK 규격대로 `String`).
  ★**신규 파일이라 충돌이 «날 수가 없다»** — `merge-tree` 가 원리적으로 못 보는 자리다.
  서술자만 맞췄다(S5 가 5곳에 적용한 확립된 처분). ★**전례: S3 3건 → S5 5곳 → S6 3곳.**
- ★**`Cargo.lock`**: S5 를 문 자리를 먼저 봤다 — ★**내려간 크레이트 0건**(`async-trait` **0.1.92 유지**) ·
  올라간 3 · 추가 `regex` · 제거 2.
- 후속 추천: ⑴**게이트③은 반드시 `--merge`**(`merge_strategy: merge` 필수 — 등재 repo).
  ⑵**S7**(컷 `ba5797b` · `95ebc5c..ba5797b` **1커밋**) — 같은 base 에서 누적 **2건**(`string.rs`·`thread.rs`)이나
  ★**S6 착지로 base 가 또 바뀌므로 착수 시 다시 재라.**
  ⑶★**「충돌 목록에 없는 파손」 축을 계약에 넣을지 판정하라 — 이제 3회째다**(S5 워크로그 `proposals[1]`).

## [2026-09-04] upstream 동기 S5 — 컷 `c4665b0` 머지 (rustjava-upstream-sync-s5-with-remeasured-conflicts)
- 무엇을: upstream `c4665b0`(#190 Java 1.2 runtime API 확장) 까지 **6커밋**(171파일 +33,138/−1,058)을 머지했다.
  재측정된 충돌 **3** 해소 — `Cargo.lock` **재생성** · `string.rs` ★**설계 판단** · `test_timer.rs` **upstream 채택**.
  ★**`merge-base origin/main upstream/main` `3296139c` → `c4665b0`** · behind **30 → 24** · 머지커밋 **부모 2개**.
- 왜: §5 「[2026-09-03 재측정]」이 예고한 3건을 닫아 다음 회차(S6)의 기준선을 만든다.
  ★**착수 시 재측정 결과가 그 표와 일치**했다(그 사이 #19·#20 이 착지했으나 둘 다 문서 회차라 수가 안 바뀌었다).
- 사용자 영향: **Java 1.2 런타임 API 가 대폭 넓어진다** — `String.copyValueOf` 2종, `compareTo(Object)` 브리지,
  `PrintStream`/`PrintWriter`/`BufferedReader`·`Writer` 계열 계약 정비, `Properties`·`Boolean`·`Integer`·`Long`
  파싱 경로, `Timer` 의 fixed-rate/fixed-delay·min-heap·overflow 처리. ★**우리 자산 변경 0** —
  charset 4종 · `System.setProperty` 서술자 · `ClassFormatError` 4종 분류 · 수동 span · 픽스처 전건 생존.
- 검증: green 4종 rc=0(`fmt` · `clippy` · wasm32 `clippy` · `test --all`) ·
  `cargo test --all` **427 passed / 0 failed / 1 ignored**(S4 261 → **+166**) ·
  ★「해소분 0」 = **`c4665b0` 대비 삭제 파일 0** · 다른 파일 **47건 전수가 우리 fork 고유 자산** ·
  `git grep 'tracing::instrument\|tracing-attributes'` 실사용 **0**(주석 1건) · `tests/test_class_format.rs` **4/4**.
- ★★**`string.rs` — 진짜 판단은 「어느 쪽을 취하나」가 아니라 「upstream 이 «되살린» 것을 버릴 것인가」였다.**
  upstream 이 `copyValueOf` 2종을 신설하며 `decode_str`/`encode_str` 하드코딩 charset 표를 되살렸는데,
  그 표를 쓰던 **4개 호출부는 충돌 없이 자동병합돼 우리 `Charset` 라우팅을 유지**했다.
  ⇒ 통째로 취했으면 **표 2함수가 dead code** 로 남아 S3 완료조건(「`charset.rs` 배선으로 dead code 0」)을 깼다.
  ★**충돌면은 «표»에 났는데 의미가 갈린 곳은 «충돌하지 않은 호출부»였다** — 「union 자동병합 경계」의 실례다.
- ★★**`test_timer.rs` — 「되얹기」 예측이 반증됐다.** upstream 이 우리 벽시계 테스트 2건을
  **manual clock + queued spawn + monitor notification** 기반 **결정성 스위트 12건**으로 대체했다
  (`Thread.sleep` 기반 단정 **0건**). ⇒ S4 의 `500→2000ms` 여백은 **되얹을 자리가 사라졌다**.
  ★★**S4 가 남긴 「우리 테스트의 시간 의존」 별 축은 소멸했다 — 그 축으로 발권하지 마라.**
  ★**단 «왜 2000 이었는지»의 근거는 `docs/upstream-sync-approach.md` §5 착지 기록에 인용으로 보존했다.**
- ★★**충돌 «목록에 없던» 파손 1건 — §4 가 경고한 형태가 실제로 났다.** upstream 신규 io 테스트 **3파일 5곳**이
  `System.setProperty` 를 `)Ljava/lang/Object;` 로 부르는데 우리는 PR #5 에서 JDK 규격대로 `)Ljava/lang/String;`
  으로 고쳐 뒀다 ⇒ ★**충돌 마커 0줄인데 `NoSuchMethodError` 3건**. 서술자만 `String` 으로 맞췄다.
  ★**새 처분이 아니다** — `test_boolean`·`test_integer`·`test_long` 이 앞 회차에 이미 같은 처분을 받았고
  이번엔 자동병합으로 통과했다. ★`java/util/Properties.setProperty` 의 `Object` 반환은 JDK 규격상 옳아 **무접촉**.
- 후속 추천: ⑴**게이트③은 반드시 `--merge`** — `<id>-merge` 티켓 frontmatter 에 `merge_strategy: merge` 필수
  (`rustjava` 는 `contracts/upstream-sync-repos.conf` 등재 · 스쿼시가 `merge-base` 를 되돌린다).
  ⑵**S6**(컷 `95ebc5c` · 11커밋) — §5 예측 **새 충돌 0**이고 이번 S5 해소로 그 전제가 실제로 섰다.
  ★그래도 **착수 시 재측정**하라(upstream 헤드가 계속 전진한다).
  ⑶★**S8 은 여전히 필요하고 제일 크다** — 개명 스윕(`java_runtime/`→`rustjava-runtime/`)이 우리 픽스처에 꽂힌다.

## [2026-09-04] 워크로그 의무화 «결정» + 형식 잠금을 로컬 DoD 안으로 (rustjava-worklog-mandate-decision-and-local-gate)
- 무엇을: 운영자 채택 제안 2건을 한 회차로 처리했다. ⑴**워크로그 작성을 DoD 의무로 «결정»**
  (`CLAUDE.md` 1줄 + `AGENTS.md` 포인터) ⑵**형식 잠금 `python3 scripts/check-worklog-json.py` 를
  로컬 DoD 4번째 명령으로 편입**. ★**코드(`.rs`) 변경 0 · 새 도구 0 · CI 워크플로 무접촉.**
- 왜: ★**먼저 쟀다** — 규약 자신이 착지한 `b3a4cf4`(2026-08-26) 이후 착지한 **4회차 중 3건(75%)**이
  워크로그 쌍을 남겼고, 유일한 미작성(PR #13)은 **부모가 정확히 `b3a4cf4`** 라 규약을 알 수 없었던
  회차다(알 수 있었던 회차만 세면 **3/3**). ★★**관측이 높은데도 의무화를 고른 이유는 하나다 —
  실패가 «조용하고 잠글 수 없다».** 잠금은 **존재하는 `.json` 만** 검사하므로(규약 비소급 설계)
  아예 안 쓴 회차는 **어디서도 red 가 나지 않고 카드만 0** 이 된다. 표본은 3건이고 전건 같은
  리니지라 소형 회차 관측은 **0** 이다. ⇒ 대가 DoD 1줄로 **무방비한 침묵**을 닫았다.
  ★**잠금 위치 실측**: `.github/workflows/rust.yml:63` `worklog_json` job **단 1곳**(ubuntu 1러너) —
  6셀 매트릭스에도 로컬 3명령에도 없어 **틀린 파일은 PR 을 열어야 빨개졌다.**
- 사용자 영향: 런타임 동작 **무변경**. 회차가 남긴 후속 추천이 cockpit 「후속 작업 추천」 패널에
  **빠짐없이 도달**하고, 형식 오류를 **push 전에** 알게 된다.
- 검증: ★**변경 «적용 후»** 4명령 전건 **rc=0**(재컴파일 회차 `fmt` 1.1s · `clippy` 11.5s ·
  `test --all` 47.6s / 완전 캐시 회차 0.56s · 0.39s · 8.41s · 잠금 0.04s).
  ★**3명령은 한 글자도 안 건드렸다** ⇒ 기존 검사의 오탐 위험은 **구조적으로 0**.
  ★**대가(초) — 분모를 둘 다 적는다**: 잠금 5회 `0.10·0.04·0.13·0.13·0.08` ⇒ 중앙값 **0.10s**.
  재컴파일 회차(60.2s) 대비 **+0.17%** · ★**완전 캐시 회차(9.36s) 대비 +1.1%** ⇒ **최악 약 1%**.
- ★**`cargo test` «안»으로 넣지 않았다 — 두 경로 다 기각**: `serde_json` 은 ★**`Cargo.lock` 에 없어**
  새 의존성 + 6셀 빌드 비용이고(★스크립트 docstring 이 이미 같은 이유로 기각한 길) · `python3`
  shell-out 은 python 없는 머신에서 `cargo test` 를 **red** 로 만들어 「오탐 0」 절대 조건을 깬다.
- ★★**되돌릴 «수» — `AGENTS.md` §Round Worklog 에 측정 명령과 함께 박았다**(취향으로 재론하지 말고 다시 재라):
  **2026-09-04 이후 10회차 착지 시점**에 ⒜**미작성 회차 ≥ 2(>20%)** ⇒ 문안을 **빼거나** 기계 강제로 올려라
  (★문서에만 둔 채 유지하지 마라 — 규칙은 있고 효력은 없는 상태가 가장 나쁘다) ·
  ⒝**열린 카드 < 5** ⇒ **의무 자체를 재검토**.
- ★**후속 추천**: 워크로그 **미작성**을 «기계»로 잡을지 판정하라 — 지금 의무는 **문서에만** 있다
  (잠금은 「있으면 검사」라 「없으면 침묵」이다). 판단 재료 = 워크로그 `2026-09-04-…json` `proposals[0]`.
- ★**고지**: 열린 **PR #19** 와 `REPORT.md`·`STATE.md` **상단이 겹친다** — 나중에 착지하는 쪽이 인접 충돌한다.

## [2026-09-03] S5~S7 「새 충돌」 재측정 + 「base 병기」를 §5 상시 규칙으로 채택 (rustjava-upstream-sync-remeasure-s5-s7-and-lock-restore-basis)
- 무엇을: PR #18 이 `--merge` 로 착지해 `merge-base origin/main upstream/main` 이 **`3296139c`** 로
  전진한 «뒤» 기준으로 S5~S7 을 다시 쟀고, ★**세 base 를 «병기»** 했다(계획 기준 `03438b0` · 복원 전
  `3a59776` · 복원 후 `8c1238b`). 같은 컷 `ba5797b` 가 base 에 따라 ★**19 · 112 · 4**(28배 차)로 갈린다.
  ⇒ 그래서 「**충돌 수를 적을 때 base 를 반드시 병기한다**」를 `docs/upstream-sync-approach.md` §5 의
  **상시 규칙으로 채택**했다(문안 신설). ★**직전 반려의 근인이 정확히 이 병기 누락**이고, S2·S4 회신은
  각자는 정확했는데 «한 표»로 모으는 순간 기준이 섞였다 — **개별 회신의 정확성으로는 막히지 않는다.**
- 왜: §5 표의 S5~S7 「0·0·+3」은 **계보가 전진한다는 전제** 위의 수인데 그 전제가 S1~S4 내내 깨져 있었다.
  이제 처음으로 참이 됐으므로 그 위에서 다시 재야 티켓 size/timeout 이 맞는다.
  ★**재측정 결과 «새 충돌»(복원 후)**: **S5 +3** · **S6 +0** · **S7 +1**. S5 는 「충돌 0 물량」이 **아니다**.
  ★★**그 +3 을 만든 것은 upstream 이 아니라 «우리가 앞 회차에 남긴 로컬 분기»다** —
  `string.rs`(우리 **+8/−28**) · `test_timer.rs`(우리 **+8/−1** = **S4 가 넣은 500→2000ms 여백**) · `Cargo.lock`(재생성).
  ★**방향 정본 = `git diff --numstat <merge-base> <ours>`**(2026-09-04 정정 — 초판 `+28/−8`·`+1/−8` 은 역순 출력이라 폐기).
  ★**S7 은 diff 32만 줄대인데 새 충돌 1건**(`thread.rs`) ⇒ 「물량이 크면 충돌도 크다」는 성립하지 않는다.
- 사용자 영향: ★**코드 변경 0 · 문서 전용**(런타임 동작 무변경). 얻는 것은 **남은 회차의 크기를 실제 수로**
  잡는 것과, 다음 회차가 같은 «기준 혼합» 반려를 반복하지 않는 것이다.
- 검증: `git merge-tree --write-tree --name-only <base> <cut>` 의 2번째 줄~첫 빈 줄. ★**이 파싱으로 §5 첫 표
  `0·1·2·2·7·16·17·17·17·19` 가 그대로 재현된다**(= 방법 자체의 검증). ★**ⓑ↔ⓒ 차이가 «계보뿐»임을 통제**:
  ⓒ 트리를 ⓑ 계보에 얹은 합성 커밋(`git commit-tree 4788ef2f -p 3a59776`)이 **45·61·112 로 동일**.
- ★**「예측은 하한」은 «절대적이지 않다» — S7 에서 처음 깨졌다**(예측 +3 ↔ 실측 ⓐ+2 · ⓒ+1).
  원인 3건 전부 실측: ⑴`Cargo.lock` **이중 계상**(S5 에서 이미 충돌) ⑵`class_format_error.rs` 는 표의 **경로가 틀렸고**,
  ★**우리 쪽이 `3296139` 와 바이트 동일**(블롭 `0dbd369a`)이라 upstream **+4/−3** 이 깨끗이 적용된다
  ⑶`throwable.rs` 도 **같은 술어**다(우리 0줄 ↔ upstream +81/−29)
  ⇒ ★**S3 의 설계 판단(upstream 오류 분류 채택)이 뒤 회차의 충돌을 «지웠다».**
  ★★**[2026-09-04 정정] 초판 ⑵의 「`3296139..ba5797b` upstream 변경 0」은 «거짓»이고 «0 인 쪽이 반대»였다** —
  실측 `diff --numstat 3296139 ba5797b` = **`4 3`** · `diff --numstat 3296139 origin/main` = **0줄**.
  ⇒ ★**「upstream 이 안 건드리는 파일」로 읽으면 «영구 무충돌»이지만, 참인 술어로는 «우리가 손대는 순간 충돌»한다.**
  결론(S7 예측 과대 = ⓐ +2)은 **불변**이다.
- ★**후속 (1) — S8 이 필요하다. 그리고 남은 회차 중 제일 크다.** 「7회차」는 더 이상 upstream 헤드에 닿지 않는다:
  `ba5797b..upstream/main` **12커밋**(★**증분** — 누적 `3296139..ba5797b` 는 18 로 다른 수다) ·
  ⓒ 누적 충돌 ★**11**(S7 의 4 대비 +7). 성격이 다르다 —
  `java_runtime/` → **`rustjava-runtime/`** · `test_data/` → **`test-data/`** **개명 스윕**이라
  새 7건이 ★**우리 고유 산출물에 꽂힌다**: `rustjava-runtime/src/charset.rs` ·
  `test-data/UnsupportedCharset.class`·`.txt` · ★**`test-data/src/UnsupportedCharset.java`**(★`.java` 만 `src/` 아래 —
  2026-09-04 경로 정정) · `test-data/TimeApi.class`·`.txt` · `test_string.rs`.
  ★개명 충돌은 3-way 가 rename 을 놓치면 픽스처가 «삭제 대 수정»으로 **조용히 사라진다** ⇒ **별도 판정 회차**로 잡아라.
- ★**후속 (2)** — S5 티켓의 성격을 「물량」 → **「설계 판단 1건 포함」**으로 바꿔 발권하라(`string.rs`).

## [2026-08-27] upstream 동기 근인 확정 — 게이트③ `--squash` 가 계보를 버린다 (rustjava-upstream-sync-squash-defeats-convergence)
- 무엇을: S1~S4(PR #11·#13·#16·#17)가 **전건 `merged=true`** 인데도 `merge-base origin/main upstream/main`
  이 fork 시점 **`62cf0c6a`** 그대로이고 behind **33** 이 줄지 않던 근인을 확정했다. 근인은 게이트③
  제품 repo **`--squash`** 다 — 네 머지커밋의 **부모가 전건 1개**이고(커밋 7·10·15·21이 각각 1로 접힘),
  반면 브랜치 `34a4235` 는 부모 **2개**(`c80638a`+`3296139`)인 진짜 머지였다.
  ⇒ ★**계보는 브랜치에 있었고 게이트③이 버렸다** — cherry-pick·브랜치 재작성 가설은 실측으로 기각.
  처방(갈래 ⒞의 ⒜)으로 `origin/main` 위에 `git merge -s ours 3296139` 를 얹었다. `1f356ae`·`af4f6f8`·
  `822504b` 는 전부 `3296139` 의 조상이라 **한 머지가 네 컷을 덮는다**.
- 왜: 내용은 이미 들어와 있는데 **계보가 없어서** git 이 영원히 「33 뒤」라고 답했고, 그 결과
  **다음 회차가 앞 회차가 이미 닫은 충돌을 처음부터 다시 열었다.**
  ★**근거는 «충돌 총수»가 아니라 «재생분»이다** — 총수는 회차마다 기준(복원 전/후)이 달라 비교가 안 된다:
  ★**S2 15 중 10 재생 → S4 20 중 18 재생**(둘 다 계보가 끊긴 회차) · ★**S3 는 계보가 온전해 재생 0**.
  기준을 단 회차별 표: **S1 2**(복원 불요) · **S2 15 → 5** · **S3 11**(★계보 온전 · 복원 불요) · **S4 20 → 2**.
  ★★**S3 는 이 병의 «반례»다** — `merge-base` 가 `af4f6f8` 로 정상이었고 복원을 하지 않았다.
  계획서 §5 는 S4 를 「새 충돌 **0**」으로 예측했는데 실측은 **복원 전 20 · 복원 후 2** 로
  **어느 쪽으로 읽어도 빗나갔다**. 그 표의 수는
  **base 가 전진한다는 전제 위의 수**였고 그 전제가 깨져 있었다.
- 사용자 영향: **제품 코드 변경 0** — 트리 오브젝트 SHA 가 `origin/main` 과 **동일**(`c4f57d10…`)하므로
  런타임 동작은 1바이트도 바뀌지 않는다. 얻는 것은 **다음 동기 회차의 비용**이다:
  `merge-base` **`62cf0c6a` → `3296139c`** · behind **33 → 18** · S1~S4 컷 조상 **0/4 → 4/4**.
- 검증: 트리 변경 0 근거 = **트리 SHA 동일**(`git diff 3a59776..c118d21` **0줄**).
  ★`git diff --stat` 빈 출력은 `-s ours` 에서 **정의상 항상 참**이라 근거로 쓰지 않았다(S4 워크로그 교훈).
  `.rs` 변경 0 이므로 cargo 4종은 트리 불변으로 담보된다. `scripts/check-worklog-json.py` rc=0.
- ★**후속 추천 (1) — 총괄 몫. 이 세션은 손대지 않았다**:
  게이트③ **제품 repo `--squash` 규율에 «upstream 동기 PR 예외»를 넣을지 총괄이 판정하라**
  (`~/orchestrator/ORCHESTRATOR.md` 2곳 · `~/orchestrator/templates/merge-ticket.tpl` 스니펫 1곳 —
  ★**orchestrator 소관이라 RustJava 세션이 고치지 않는다**).
  ★★**이 PR 자체가 `--squash` 로 착지하면 위 처방이 그대로 무효가 된다** — 부모 2개가 1개로 접히며
  `3296139` 조상 관계가 사라지고 `merge-base` 는 `62cf0c6a` 로 되돌아간다. 반드시 `gh pr merge --merge`.
  ★**회차마다 브랜치에 `-s ours` 를 다시 넣는 현행 방식은 러닝머신이다** — S4 의 `c80638a` 가 정확히 그
  처방이었는데 PR #17 의 스쿼시에 함께 사라졌고, 이 리니지에서 **네 번** 반복됐다.
  ★★**`wie` 도 같은 함정 위에 있다 — behind 1067.** 예외 판정은 RustJava 단독 문제가 아니다.
- ★**후속 추천 (2)**: S5~S7 의 「새 충돌」을 **이 PR 이 `--merge` 로 착지한 «뒤»** 재측정하라 —
  `merge-base` 가 `3296139` 인 상태에서 잰 수라야 다음 회차의 실제 기준선이다(워크로그 `#p1`).

## [2026-08-27] upstream 동기 S4 — 컷 `3296139` 머지 (rustjava-upstream-sync-s4)
- 무엇을: upstream `3296139`(#184 CLI classpath) 까지 **8커밋**을 머지했다(GlobalRef · CDC text API ·
  monitor 인자 일반화 · classfile 오류 은닉 · tokio 1.53). 충돌 **2** 해소 —
  `jvm/src/jvm.rs` 는 **합집합**(upstream `load_bootstrap_class` + 우리 `double_must_use` allow),
  `java/lang/thread.rs` 는 **upstream 의 `GlobalRef` 본문 + PR #4 의 수동 span**이다.
  ★**첫 조치는 `git merge -s ours --no-ff 822504b`** — 그것이 **충돌 20 → 2**를 만들었다.
  ★**무해성의 근거는 «`git diff --stat origin/main HEAD` 빈 출력»이 «아니다»** — `-s ours` 는 정의상 우리 트리를
  유지하므로 그 출력은 **항상 참**이라 아무것도 증명하지 않는다. 근거는 ★**`--diff-filter=D` 0**(upstream 이
  들여온 것 중 잃은 파일 0)**와 충돌 2파일의 양방향 전문 대조**다.
- 왜: 스쿼시 착지 3회(#11·#13·#16)로 `origin/main` 의 upstream 조상이 ★**최초 공통조상 `62cf0c6` 까지
  되돌아가 있었다**(`1f356ae`·`af4f6f8`·`822504b` 전건 조상 아님). 그대로 재면 git 이 앞 회차가 이미 해소한
  자리를 통째로 재생해 **20충돌**을 낸다. 트리는 이미 동일하므로 부모만 기록해 base 를 복원했다.
  ★**S2 회차가 세운 방법을 그대로 썼고, 이제 이 리니지에서 네 번째 적용이다.**
- 사용자 영향: JNI 스타일 **전역 참조**(`GlobalRef`)가 들어와 스폰된 스레드가 자기 `this` 를 GC 로부터
  안전하게 붙든다. **CLI 에 classpath 옵션**이 생기고(`-cp`/`-classpath`), `java.text` 포맷팅 API
  (`DateFormat`·`DecimalFormat`·`SimpleDateFormat`·`NumberFormat`)가 추가된다.
  ★**기존 동작 변경 0** — 우리 자산(charset 4종 · `System.setProperty` 서술자 · `ClassFormatError` 4종 분류 ·
  수동 span)은 전건 생존했다.
- 검증: `cargo fmt --all -- --check` · `cargo clippy --all -- -D warnings` ·
  `cargo clippy --workspace --exclude test_utils --target wasm32-unknown-unknown -- -D warnings` ·
  `cargo test --all` **4/4 rc=0** · **261 passed / 0 failed / 1 ignored**(S3 216 → +45).
  「해소분 0」 증명 = ★**upstream `3296139` 대비 삭제된 파일 0** · 다른 파일 **39건 전수가 우리 fork 고유 자산**
  (원장·CI·worklog·charset·오류분류·tracing·픽스처·타이머 여백). 충돌 2파일은 **양방향 원본 전문 대조**로
  소실을 전건 확인했고 **의도 밖 0**이다.
- ★**타이머 테스트 여백 1건 — «회귀»가 아니다**(★전 판본의 「들여온 upstream 회귀」 서술은 **틀렸다**):
  `test_timer_periodic` 은 ★**컷 이전부터** 500ms 창에서 기대 10회 대비 **3~4회**만 도는 **만성 경계 테스트**이고,
  머신 부하가 걸리면 ★**컷 양쪽이 «같은 비율로»** 단정 아래로 떨어진다.
  ★**측정 조건을 맞춰 교대 실행한 실측**(★조건을 섞지 않는다):
  ⒜**단독 실행 · 교대 10회** — `4bb796d`(컷 전) `3 3 3 3 4 4 4 4 3 4`(mean 3.5) ↔
    `3296139`(컷 후) `4 3 4 3 4 3 4 3 4 3`(mean 3.5) ⇒ ★**차이 없음**
  ⒝**전 스위트 병렬 · 교대 8회** — 컷 전 `4 4 4 4 3 4 3 4` ↔ 컷 후 `4 3 3 6 4 4 4 4` ⇒ ★**차이 없음**
  ★**사료가 그 자체로 반증이다**: upstream 이 같은 자리를 넓힌 `895d67d`(**2025-08-20**)·`ad8b477`(**2025-10-04**)는
  ★**둘 다 이미 `origin/main` 의 조상**이고, 근인으로 지목했던 `e557673`(GlobalRef)은 **2026-07-18** 이다
  ⇒ ★**이 테스트는 지목된 커밋보다 «11개월 앞서» 이미 만성 flaky 였다.**
  ★**전 판본이 틀린 이유는 «수»가 아니라 «조건»이다** — `origin/main` **10/10**(단독)과 순정 upstream **3/8 실패**(병렬)를
  나란히 놓았다. ★**서로 다른 측정 조건의 수를 비교했다.**
  **처분은 그대로다**(`sleep 500 → 2000ms` · `run_count > 2` **불변** · `#[ignore]` 0 · 삭제 0) —
  단 성격이 「가리는 여백」이 아니라 ★**만성 경계 테스트에 정상 여백을 준 것**이다.
  ★**대가**: 창을 넓히면 감도가 내려간다 — red 문턱이 1회전 **~167ms → ~667ms**(약 4배 둔화)로, 돌연변이
  「루프 sleep 16ms → 700ms(5.6배 저하)」는 여전히 red 지만 「→ 300ms(2.4배)」는 이제 통과한다. 그 상한을 주석에 박았다.
- 후속 추천: ⑴**게이트②** — `CLAUDE.md` DoD 상 ★**머지는 검수자가 approve 와 «같은 턴»에 집행**한다
  (`<id>-merge` 는 **예외 경로**다). ⑵**S5**(컷 `c4665b0` · 171파일 +33,138) —
  ★**착수 첫 조치는 `git merge -s ours --no-ff 3296139`**(S4 도 스쿼시로 착지하면 족보가 또 끊긴다).
  ⑶★**`thread.rs` 는 S1·S3·S4 «세 회차 연속» 충돌한다** — S5~S7 도 기본값으로 잡아라. 전략은 불변
  (upstream 본문 + 수동 span 1줄 치환). ⑷★**「타이머 성능 회귀」는 «없다» — 그 축으로 발권하지 마라**
  (위 문단 참조: 컷 전후가 조건 맞춘 실측에서 동일하고, upstream 이 이미 두 번 넓힌 자리다).
  ★**남는 별 축은 «우리 테스트의 시간 의존»이다** — `test_timer_periodic` 이 벽시계에 의존하고 이번이 세 번째
  여백 확장이며 red 문턱이 약 4배 둔해졌다. ★**주인은 우리이고 upstream 발신은 «불요»다.**
  판단 재료 = worklog `2026-08-27-upstream-sync-s4.json` `proposals[0]`.

## [2026-08-27] upstream 동기 S3 — 컷 `822504b` 머지 (rustjava-upstream-sync-s3)
- 무엇을: upstream `822504b`(#180 Harden JVM runtime correctness) 1커밋을 머지했다. 충돌 **11** 해소.
  `classfile/{class,constant_pool,error,lib}.rs` · `jvm_rust/class_definition.rs` · `src/runtime.rs` ·
  `test_utils/lib.rs` 는 **upstream 채택**(우리 `ParseError` 5변형 → upstream `ClassFileError` +
  `ClassDefinitionError`). `java/lang/string.rs` 는 **upstream 골격을 우리 `charset::Charset` 으로
  라우팅**해 중복 charset 표를 지웠고, `test_string.rs` 는 **양쪽 테스트 합집합**,
  `thread.rs` 는 **upstream 본문 + PR #4 의 수동 span**, `AGENTS.md` 는 **양쪽 절 합집합**이다.
  부수: `tests/test_class_format.rs` 의 **문구 단정 3건 삭제**(종류 단정은 유지).
- 왜: 계획서 §3-B 가 판정한 대로 **Java 관측면에서 upstream 이 이긴다** — 우리 `ParseError` 는 Rust
  변형이 5종이지만 Java 예외는 `ClassFormatError` **1종**뿐이고, upstream 은 `ClassFormatError` ·
  `UnsupportedClassVersionError` · `VerifyError` · `UnsupportedOperationException` **4종**으로 나눈다.
  JVM 구현체에서 값이 큰 쪽은 관측면이다. PR #3 의 목적(「패닉 대신 `ClassFormatError`」)은 upstream
  에서도 그대로 성립한다(미지원 상수풀 태그 → `ErrorKind::Switch` → `InvalidFormat`, 패닉 0).
  ★**치른 값은 진단 문구다** — 「Truncated」·「tag 18」·「magic」이 전부 `"Invalid class file"` 로 평탄해졌고,
  §4-A 가 예고한 대로 `tests/test_class_format.rs` 3건이 **충돌 마커 없이** 그것 때문에 깨졌다.
- 사용자 영향: 클래스파일 검증이 세분화된다 — 지원하지 않는 클래스파일 버전은 이제
  `UnsupportedClassVersionError`, 바이트코드 검증 실패는 `VerifyError`, `invokedynamic` 은
  `UnsupportedOperationException` 으로 **깔끔히 거부**된다(구판은 인터프리터 `todo!()` 패닉까지 갔다).
  대신 `ClassFormatError` 메시지는 원인별 문구를 잃고 `"Invalid class file"` 평문이 된다.
  ★**charset 동작 변경 1건**: 기본 charset 경로(`new String(byte[])`·`getBytes()`)는 미지원 이름에도
  더 이상 `UnsupportedEncodingException` 을 던지지 않고 UTF-8 로 폴백한다 — **JDK 규격이 그렇다**.
  명시 charset 경로(`new String(byte[],String)`·`getBytes(String)`)는 그대로 던진다.
  ISO-8859-1·US-ASCII 는 우리 `Charset` 이 정본으로 남아 계속 동작한다(종단 픽스처 green).
- 검증: `cargo fmt --all -- --check` · `cargo clippy --all -- -D warnings` ·
  `cargo clippy --workspace --exclude test_utils --target wasm32-unknown-unknown -- -D warnings` ·
  `cargo test --all` **4/4 rc=0** · **216 passed / 0 failed / 1 ignored**(S2 191 → +25, 우리 테스트 유실 0).
  `tests/test_class_format.rs` **4/4** · `git grep 'tracing::instrument\|tracing-attributes'` **0건**(§4-C 불변).
  추가로 「base `af4f6f8` 이후 우리가 추가한 .rs 321줄이 머지 트리에 살아 있는가」를 기계로 전수 대조했고,
  부재 81줄은 **전건 의도한 해소**였다(`ParseError` 기구 · `thread.rs` 구본문 · 완화한 문구 단정 3줄).
- 후속 추천: ⑴**게이트③ 순서 주의** — `rustjava-upstream-sync-s2-merge`(#13)가 **먼저**고 S3 PR 이 그 위다.
  ★#13 이 스쿼시로 착지하면 `af4f6f8` 조상이 다시 끊기므로, S3 PR 이 `main` 으로 리타깃된 뒤
  **S2 가 한 `-s ours` 를 다시 해야 할 수 있다**(착지 후 `merge-base` 를 재라).
  ⑵**S4**(컷 `3296139`) — 계획서 예측 **새 충돌 0**. 검수는 「우리 해소분 0 증명 + green」.
  ⑶★**계획서의 「새 충돌」 예측은 하한이다** — S3 는 +9 예측에 `AGENTS.md`·`thread.rs` **2건이 더 붙었다**.
  전자는 계획서 이후 우리가 만든 파일이고, 후자는 **앞 회차가 이미 닫은 파일의 재충돌**이다.
  ⑷`charset.rs` dead-code red 축은 **닫아도 된다** — S3 에서 오히려 upstream 중복 표를 흡수했다.

## [2026-08-26] 회차 워크로그 `.json` + `proposals` 규약 이식 (rustjava-worklog-json-proposals-convention)
- 무엇을: `AGENTS.md` 에 「Round Worklog `docs/worklog/`」 절(소비처가 읽는 키 표 · 소급 없음),
  `scripts/check-worklog-json.py` 잠금 6축, `rust.yml` 에 `worklog_json` job 1개(ubuntu 단일 러너),
  `docs/worklog/` 개시 + 이 회차 `.md`/`.json` 한 쌍. **Rust 코드 변경 0.**
- 왜: cockpit 「후속 작업 추천」 커버리지 6 repo 중 채워진 것이 2개뿐이고 RustJava 는
  `docs/worklog` **디렉터리 자체가 없어**(2026-08-26 재실측 `archiveErr: pathspec … did not match`)
  구조적으로 0건이었다. qts 회차가 만든 규약을 **복제**했다 — 새 스키마·새 기구 0.
- 사용자 영향: 착지 후 RustJava 의 후속 제안이 cockpit 화면에 처음 뜬다(이 회차 2건).
  회차마다 워크로그 2파일 작성 부담이 는다.
- 후속 추천: ★**규약을 심었다 ≠ 카드가 계속 는다.** 이 repo 회차 기록 정본은 `REPORT.md` 라
  워크로그 작성이 DoD 에 없다 — 의무화 여부는 미결(짝 `.json` 의 `proposals[0]`).
  잠금이 `cargo test` 밖 CI job 이라 로컬 DoD 3명령으로는 안 돈다(`proposals[1]`).

## [2026-08-25] beta clippy `double_must_use` red 해소 (rustjava-ci-beta-clippy-double-must-use-red)
- 무엇을: `Cargo.lock` 의 `async-trait` 0.1.89→**0.1.92**, `#[async_recursion]` **7지점**에 국소
  `#[allow(clippy::double_must_use)]`, `rust.yml` matrix 에 `fail-fast: false` 1줄. **기능 변경 0.**
- 왜: `rustup run beta cargo clippy --all -- -D warnings` 가 `origin/main`(코드 무변경)과 열린 PR
  양쪽에서 **동일하게 13건** red 였다 ⇒ ★코드가 아니라 **부동 beta 채널이 움직였다**(1.99.0-beta.1,
  2026-08-17). 13건 **전부** `note: this error originates in the attribute macro …` — 우리 소스에
  `#[must_use]` 를 쓴 지점은 **0건**이고 `async_trait` 7 + `async_recursion` 6 의 매크로 확장이 찍은 것이다.
  0.1.92 의 `async-trait` 은 그 `push(#[must_use])` 를 삭제해 7건이 사라지고, `async-recursion` 은
  **1.1.1 이 최신**이라 올릴 곳이 없어 그 7지점(6+jvm_rust 1)만 국소 억제했다 — crate/워크스페이스 전역 억제는 쓰지 않았다.
- 사용자 영향: 없음(런타임 동작 무변경). `main` 과 열린 PR 전건을 막던 게이트③ 병목이 풀린다.
- 후속 추천: ★열린 PR 은 **자동으로 green 이 되지 않는다** — 이 PR 착지 후 각 PR 의 CI 재실행이 필요하다
  (PR #13 `upstream-sync-s2` 는 이미 게이트② approve 상태라 재실행만 남는다).
## [2026-08-24] upstream 동기 S2 — 컷 `af4f6f8` 머지 (rustjava-upstream-sync-s2)
- 무엇을: upstream `af4f6f8`(#177 CLDC 1.1 core API) 1커밋을 머지했다. **63파일 +3,217/−383.**
  충돌 **5** 해소 — `io.rs`·`unsupported_encoding_exception.rs`·`loader.rs` 는 upstream 이 상위집합이라
  그쪽을 취했고, `input_stream_reader.rs` 는 **우리 `Charset`(UTF-8·EUC-KR·ISO-8859-1·US-ASCII 4종)을
  정본으로 유지**한 채 upstream 의 멀티바이트 경계 처리(`decode_length`·`end_of_input`)만 얹었으며,
  `test_input_stream_reader.rs` 는 **양쪽 테스트 합집합**(우리 3 + upstream 4 = 7건 전부 통과)이다.
  부수 2건: ⑴`Throwable::getMessage` **조용한 중복** 제거 ⑵`loader.rs` 에서 `--theirs` 가 지운
  `ClassFormatError::as_proto()` 등록 1줄 복원.
- 왜: ★**PR #11(S1)이 스쿼시로 착지해 upstream 조상이 끊겨 있었다.** `origin/main` 의 코드 트리는 S1
  머지 결과와 **바이트 동일**(`git diff 0bd4f80 origin/main -- '*.rs' '*.toml' '*.lock'` 빈 출력)인데
  git 의 merge-base 는 여전히 `62cf0c6` 라, `merge-tree` 가 `1f356ae` 의 6커밋을 통째로 재생하며
  **충돌 15건**을 냈다 — S1 이 이미 해소한 자리들이었다. `git merge -s ours 1f356ae`(트리 무변경)로
  부모만 기록해 base 를 복원하니 **충돌 5건**, 즉 S1 이 예고한 파일 5개와 정확히 일치했다.
- 사용자 영향: CLDC 1.1 코어 API 가 들어온다(`InputStreamReader.ready()`·2인자 생성자,
  `OutputStreamWriter`, `PrintStream` 확장, CLDC 예외 계층, `java.util.Date`/`Random`/`Calendar` 보강).
  ★**기존 charset 동작은 그대로다** — ISO-8859-1/US-ASCII 는 upstream 인라인 판본에 없지만 우리 것이
  살아남아 계속 동작하고, PR #5 의 종단 픽스처(`test_data/UnsupportedCharset`, ISO-8859-1 `aéb`)도 green 이다.
  ★단 **2인자 생성자 `(InputStream, String)` 는 미지원 charset 을 «생성 시점»에 던진다**(upstream 신규 ·
  JDK 규격). 1인자 생성자는 JDK 가 `UnsupportedEncodingException` 을 선언하지 않으므로 **기존대로
  read() 시점에** 던진다 — 그래서 픽스처를 재컴파일하지 않고도 양쪽 테스트가 다 산다(이 맥에 JDK 부재).
- 검증: `cargo fmt --all -- --check` · `cargo clippy --all -- -D warnings` ·
  `cargo clippy --workspace --exclude test_utils --target wasm32-unknown-unknown -- -D warnings` ·
  `cargo test --all` **4/4 rc=0** · **191 passed / 0 failed / 1 ignored**(S1 169 → +22, 우리 테스트 유실 0).
  추가로 「base `1f356ae` 이후 우리가 추가한 260줄이 머지 트리에 살아 있는가」를 기계로 전수 대조했고,
  부재 2건은 **의도한 해소**임을 확인했다(디코드 호출 1줄 = upstream 인자 채택 · `io.rs` `pub use` 1줄 = rustfmt 재배치).
- 후속 추천: ⑴**게이트③ `rustjava-upstream-sync-s2-merge`**. ⑵**S3**(컷 `822504b` · 오류 분류 축) —
  ★착수 전 `git merge-base origin/main upstream/main` 을 확인하고 `af4f6f8` 가 아니면 `-s ours` 로
  조상을 먼저 복원하라(스쿼시 머지가 매 회차 이 문제를 재생산한다). S1 이 예고한
  `classfile/src/error.rs` 재작성 ↔ 우리 `ParseError` 5변형 충돌이 거기서 터진다.
  ⑶`charset.rs` dead-code red 예측은 **S2 에서 발동하지 않았고 앞으로도 발동 가능성이 낮다** —
  호출자가 5 → 7건으로 늘었다. S3 의 `string.rs` 접촉 시 한 번 더 확인하면 이 축은 닫아도 된다.

## [2026-08-17] `coverage` 상시 red 해소 (rustjava-coverage-workflow-codecov-token-red)
- 무엇을: `.github/workflows/coverage.yml` 의 `fail_ci_if_error` 를 `true` → **`false`** 로 내리고
  이유·복구법을 주석으로 박았다. **변경 파일 1개**(워크플로) + 문서 2개.
- 왜: `coverage` 는 **보고를 시작한 이래 24/24 전건 red** 였다(2026-07-22~08-17). 근인은 단 하나 —
  ★**이 저장소는 fork 라 upstream 의 시크릿을 상속하지 않는다.** `gh secret list` 는 **빈 목록**이고
  `secrets.CODECOV_TOKEN` 이 빈 문자열로 전개돼 업로더가 `Token length: 0` →
  `{"message":"Token required - not valid tokenless upload"}` 로 죽었다. `fail_ci_if_error: true` 가
  그 업로드 실패를 job 실패로 승격시켜 왔다. ★**상수 red 는 신호가 아니다** — 진짜 회귀가 섞여도 안 보이고,
  실제로 upstream 동기 매 회차가 「선재 인프라라 머지 차단 아님」이라는 **특례 문구를 손으로** 달아야 했다.
- 사용자 영향: 없음(코드 무변경). CI 신호만 회복된다. ★**빌드 검사는 약해지지 않는다** —
  `Generate code coverage` 는 별도 `run:` 스텝이고 `continue-on-error` 가 없어 빌드/테스트가 깨지면
  여전히 job 이 red 다. 비치명이 된 것은 **codecov.io 로의 «발행»뿐**이고 그 오류 문구는 스텝 로그에 그대로 남는다.
- 후속 추천: `CODECOV_TOKEN` 을 이 fork 에 등재하면 업로드까지 복구된다 — ★**human-step**(시크릿 발급·등재는
  사람 몫). 등재 전까지는 codecov.io 에 데이터가 쌓이지 않는다(단, 토큰이 없던 지금까지도 쌓인 적이 없다).

## [2026-08-17] upstream 동기 S1 — 컷 `1f356ae` 머지 (rustjava-upstream-sync-s1-tracing-cut-1f356ae)
- 무엇을: upstream `1f356ae`(#173~#179 · 5커밋)를 머지했다. 충돌 **2** 해소 —
  `lang.rs` 는 **양쪽 병합**(우리 `class_format_error` + upstream 의 Java 1.2 wrapper 9종),
  `thread.rs` 는 **upstream 뼈대 + PR #4 수동 span 재적용**(`#[tracing::instrument]` 한 줄만 치환,
  `Cargo.toml` 2개 무접촉). 66파일 `+5,235 / −151`.
- 왜: 접근안 §6 이 정한 7회차 중 첫 회차이고 축은 tracing 이다. upstream `thread.rs` 를 그대로 취하면
  `attributes` 피처가 꺼진 tracing 에 속성 매크로가 걸려 **컴파일이 깨지고**, 피처를 되살리면 PR #4 가
  통째로 되돌아간다. 뼈대만 취하고 span 만 수동으로 되돌려 둘 다 피했다.
  ★**충돌 목록 밖에서 하나가 더 깨졌다**: 우리 PR #5 가 JDK 규격에 맞게 고친
  `System.setProperty` 서술자(`…)Ljava/lang/String;` — 실제 javac 바이트코드가 그렇다)와
  upstream 의 구판(`…)Ljava/lang/Object;`)이 어긋나, upstream 이 새로 들여온 wrapper 테스트 3건이
  `NoSuchMethodError` 로 죽었다. 우리 서술자를 유지하고 upstream 테스트 호출부 6곳을 고쳤다.
- 사용자 영향: 없음(동작 변경 0). Java 1.2 wrapper 클래스 9종
  (`Boolean`/`Byte`/`Character`/`Double`/`Float`/`Long`/`Number`/`Short` · `ClassNotFoundException`)과
  `Thread.currentThread()` 동일객체 반환이 들어왔다. `cargo test --all` **169 passed / 0 failed / 1 ignored**
  (기준선 149 → +20, 전부 upstream 신규 + 우리 기존분).
- 후속 추천: S2(컷 `af4f6f8` · charset 축 · 새 충돌 +5). ★착수 시 충돌 재측정 필수 ·
  ★**우리 프로덕션 서술자/시그니처 변경이 upstream 신규 테스트와 어긋나는지**를 S1 과 같은 방식으로 훑어라.

## [2026-08-16] upstream 동기화 접근안 확정 (rustjava-upstream-sync-approach-plan)
- 무엇을: 격차를 오늘 값으로 다시 재고(**10 앞섬 / 33 뒤처짐** · 충돌 **17 → 19파일**), 충돌 19파일을
  처분 어휘 4종으로 분류한 표와 단계 분할안을 `docs/upstream-sync-approach.md` 로 확정했다.
  **머지 실행 0 · 충돌 해소 0 · 코드 변경 0**(문서만).
- 왜: 충돌의 성격이 「어느 쪽 구현을 남기는가」라 머지 도중 즉흥 결정이 불가능했다. 실제로 실측해 보니
  선행 전제 2건이 틀렸다 — ⑴add/add 두 파일은 **의미 차이 0**이라 «정면 충돌»이 아니고 진짜 설계
  결정은 `classfile/src/error.rs` 하나뿐이며 ⑵charset 퇴행 범위는 `string.rs` 가 아니라
  `input_stream_reader.rs` 하나다(upstream 이 동일 charset 집합을 독립 구현했고 기본 경로에선 더 옳다).
- 사용자 영향: 아직 없다(문서 전용). 다만 ★**충돌 목록에 없는 파일 3곳이 조용히 깨진다**는 것을
  머지 전에 잡았다 — `tests/test_class_format.rs` 3건 실패 · `thread.rs`×`Cargo.toml` tracing 함정
  (그대로 취하면 컴파일 파괴, 되살리면 PR #4 되돌림) · `charset.rs` dead code 로 clippy red.
- 후속 추천: `rustjava-upstream-sync-s1`(컷 `1f356ae` · 새 충돌 2 · tracing 축) 발권. ★구판
  `-32-commits` 단일 티켓은 폐기 — 컷별 실측상 **19충돌 중 16이 앞쪽 7커밋에 몰려** 있어 커밋 수
  분할은 무의미하다. S1~S3(설계) → S4~S7(물량) 순서로 7회차.

## [2026-08-15] upstream 격차 재실측 + 잔존분 재판정 (rustjava-lane-restart-upstream-sync-precondition)
- 무엇을: `origin/main` ↔ `upstream/main` 격차를 오늘 값으로 다시 재고(**9 앞섬 / 32 뒤처짐** —
  구판 「20 뒤처짐」은 낡았다), 선행조건이던 upstream `agent/runtime-api-gaps` 의 상태를 확정하고,
  `wie-ktf-hardening` 유효 잔존을 **4건 → 2건**으로 재판정해 STATE.md `## 다음` 을 전면 갱신했다.
  코드 변경 0(문서만).
- 왜: 이 레인이 25시간 무배차로 멈춰 있었고 근인이 **낡은 `## 다음`** 이었다. 특히 선행조건으로
  걸려 있던 `agent/runtime-api-gaps` 는 「미머지」가 아니라 **PR #190 으로 2026-07-25 04:59Z
  스쿼시 머지**(`c4665b0`)돼 브랜치까지 삭제된 상태였다 — 즉 잔존분 판정 기준이 통째로 바뀌어
  있었는데 아무도 다시 재지 않았다. 재판정 결과 Timer·StringBuffer 2건·BAIS·Class.forName·
  Integer.byteValue/shortValue 는 upstream 이 **삼켰고**, `System.arraycopy` 와
  `String.<init>([B)/([C)` 의 null 가드 **2건만** 유효하게 남았다.
- 사용자 영향: 런타임 동작 변화 없음. 다음 회차 작업이 무효 6건을 중복 구현하는 낭비가 사라졌다.
- 후속 추천: ①`rustjava-upstream-sync-32-commits`(P1·L) — 충돌 예상 **17파일** 실측 완료.
  ★머지 시 upstream 의 `input_stream_reader.rs`(UTF-8/EUC-KR 하드코딩)를 그대로 취하면 우리
  PR #5 의 charset 일반화가 **퇴행**하니 `test_data/UnsupportedCharset` 로 잠그고 진행할 것.
  ②그 뒤 null 가드 2건(형제 `String.<init>([BII)` 포함) — 원인은 `ClassInstanceRef::deref` 의
  `unwrap()` 이라 전역 수리가 불가하고 진입부 가드가 정답이다.
  ③invokedynamic `todo!()` + 상수풀 태그 15~18 미지원은 ★**아직 미해결**이다(양쪽 브랜치에서
  실측 확인). 「이미 처리됐다」는 통설을 STATE 에서 정정했다.
- ※**[2026-08-16 게이트② 반려 반영]** 같은 PR 위에 문서 4곳을 고쳤다(코드 여전히 0).
  ⒜★**「열린 PR 0」이 거짓이었다** — 실측 **2건**(#9 · ★**#8 `[rustjava-claude-md-prune]` 이 11일째
  좌초 · `.review.md` 부재**). ⑤를 실측 표로 교체하고 **#8 처분 브리프를 목록 맨 위에** 넣었다.
  ⒝★invokedynamic 은 upstream `jvm_rust/src/verifier.rs` 가 `UnsupportedFeature` 로 **거부**한다 ⇒
  **①머지로 패닉 축이 소멸**하므로 「한 티켓으로 묶는 이유」를 다시 썼다. ⒞null 가드 형제 열거를
  **전수 7건**으로 확대(`([BLjava/lang/String;)`·`([BIILjava/lang/String;)`·`(Ljava/lang/String;)`·
  `(Ljava/lang/StringBuffer;)` 추가). ⒟★`system.rs` 는 충돌 목록에 **없다** — 선행 근거를 `string.rs`
  하나로 정정.

## [2026-07-25] 원격 잔존 브랜치 위생 판정 (2026-07-25-rustjava-branch-hygiene)
- 무엇을: 포크(Jun025/RustJava)에 남아 있던 원격 브랜치 2건을 실측 대조로 판정했다.
  `dependabot/cargo/tracing-attributes-0.1.31` 은 삭제하고, `wie-ktf-hardening` 은
  혼재 판정으로 보존 + 커밋별 판정표를 제출했다. 코드 변경은 없다.
- 왜: dependabot 브랜치는 PR #4(`fa92ef9`)가 `tracing-attributes` 직접 의존을 통째로
  제거해 패치 대상 라인 자체가 사라졌다 — 머지해도 적용되지 않는 죽은 패치다.
  `wie-ktf-hardening` 은 `git cherry` 가 12커밋 전부를 미반영(`+`)으로 표시했지만, 실제로는
  내용이 upstream 에 스쿼시 머지(#174·#175·#176·#177·#180·#182)돼 있었고 우리 `origin/main`
  이 upstream 보다 20커밋 뒤처져 있어 생긴 착시였다. 12건 중 8건 반영/대체, 4건만 유효 잔존.
- 사용자 영향: 원격 브랜치 목록이 `main` + 판정 보류 1건으로 정리됐다. 런타임 동작 변화 없음.
- 후속 추천: ① `origin/main` ← `upstream/main` 20커밋 동기화가 선행돼야 한다(그래야 잔존분이
  4건으로 확정되고, upstream 이 `GlobalRef`(#182)로 다르게 푼 스레드 루팅과의 이중 적용을 피한다).
  ② 이후 잔존 4건(Timer 1회성 schedule, StringBuffer.insert, append([CII) null→NPE,
  arraycopy·String.<init>([B)([C)·BAIS.<init>([B) null 가드 + Integer.byteValue/shortValue)만
  추려 PR 1건 — 전부 호스트 프로세스를 죽이는 실제 패닉이라 upstream 상납 가치도 있다.
  ③ 착수 전 upstream `agent/runtime-api-gaps`(미머지, +33k lines) 중복 여부 확인.

## [2026-07-22] 미지원 charset 패닉 → UnsupportedEncodingException (rustjava-unsupported-charset-exception)
- 무엇을: `String.getBytes(charset)`/`new String(byte[], charset)`/`InputStreamReader.read()`의
  `unimplemented!()` 패닉을 `java.io.UnsupportedEncodingException`(신설, IOException 하위) throw 로
  전환하고, String↔Reader 의 지원 charset 목록을 공용 `charset::Charset` 으로 일원화했다.
- 왜: charset 이름은 자바 코드가 넘기는 완전한 사용자 입력인데 미지원 이름 한 줄에 호스트
  프로세스가 죽었다. Reader 쪽은 ISO-8859-1 조차 못 받는 String 쪽과의 불일치도 있었다.
- 사용자 영향: `"hi".getBytes("UTF-16")` 류가 이제 try/catch 로 잡히는 자바 예외가 되고,
  `file.encoding=ISO-8859-1` 후 InputStreamReader 도 정상 동작한다. 부수 교정:
  `System.setProperty` 반환 시그니처를 JDK 규격(`...)Ljava/lang/String;`)으로 수정,
  `Throwable.getMessage()` 신설.
- 후속 추천: ① `Charset` 공용화를 계기로 UTF-16/Shift_JIS 등 실제 인코딩 추가는 별건 티켓으로.
  ② InputStreamReader 가 read 마다 스트림 디코더를 새로 만들어 버퍼 경계의 multibyte 부분
    시퀀스가 유실될 수 있는 기존 문제(EUC-KR)가 남아 있다 — 별건 조사 권장.

## [2026-07-22] tracing-attributes 상한 핀 제거 (rustjava-tracing-attributes-pin-removal)
- 무엇을: 워크스페이스 유일의 `#[tracing::instrument]`(thread.rs, "java thread" span)를
  `tracing::info_span!` + `Instrument` 수동 span 으로 대체하고, `java_runtime` 의
  `tracing-attributes <0.1.29` 직접 의존 핀과 workspace `tracing` 의 `attributes` 피처를 제거.
  Cargo.lock 은 tracing 계열만 국소 갱신(tracing 0.1.41→0.1.44, subscriber 0.3.20→0.3.23,
  tracing-attributes 그래프에서 소멸). wasm32 clippy CI 의 누락 커버리지도 교정
  (`--workspace --exclude test_utils` — test_utils 는 tokio rt-multi-thread 라 wasm 불가).
- 왜: 한 줄의 attribute macro 가 no_std 빌드를 깨는 탓(tokio-rs/tracing#3388)에 tracing 계열
  전체가 동결됐고 dependabot PR 이 해석 불가로 계속 죽었음.
- 사용자 영향: tracing 계열 업데이트 재개 가능(보안 패치 포함). span 출력("java thread{id=N}"
  이름·필드·레벨·타깃)은 실행 대조로 동일함을 확인 — 관측 회귀 0.
- 후속 추천: ① dependabot 재시도 유도(다음 주기에 자동), ② javac 21 익명 내부 클래스 파싱
  실패(Malformed) 원인 조사 별건, ③ wasm32 에서 test_utils 대체 테스트 전략 검토.

## [2026-07-22] 클래스파일 파싱 실패 → ClassFormatError 전파 (rustjava-classfile-parse-error-propagation)
- 무엇을: `ClassInfo::parse` 를 `Option` → `Result<_, ParseError>` 로 바꿔 실패 원인(절단/매직
  불일치/미지원 상수풀 태그 N/기타 손상)을 담고, `from_classfile` 의 `unwrap()`/`assert_eq!` 를
  제거해 `define_class` 에서 기존 예외 관례(`jvm.exception`)대로 `java.lang.ClassFormatError` 로
  올림. `java/lang/ClassFormatError` 런타임 클래스(부모 LinkageError) 신설.
- 왜: 손상되거나 미지원 항목(javac 9+ 가 기본으로 심는 invokedynamic 계열 태그 15~18)을 가진
  class 파일을 여는 순간 Rust 패닉으로 프로세스(임베딩 호스트 포함)가 즉사했음. "클래스 못 찾음"
  은 예외인데 "못 읽음"만 패닉인 비대칭.
- 사용자 영향: 잘못된 class 파일이 진단 가능한 자바 예외(원인 메시지 포함)로 보고되고 프로세스는
  살아남음. 미지원 태그는 명확히 거절(구현 아님). `tests/test_class_format.rs` 4케이스(절단/태그
  18/매직/못찾음 대조군)가 회귀 잠금.
- 후속 추천: ① invokedynamic/MethodHandle 실제 지원(별건 대형), ② 상수풀 인덱스 참조
  (`.get().unwrap()` 계열) 손상 대응(별건), ③ UnsupportedClassVersionError 도입 검토(major
  version 기반).

## [2026-07-22] 시간 API 패닉 제거 + 회귀 잠금 (rustjava-runtime-time-todo-impl)
- 무엇을: `src/runtime.rs`의 `RuntimeImpl` 에서 `now()`/`sleep()`/`r#yield()` 의 `todo!()` 를 실제
  구현(UNIX epoch ms·`tokio::time::sleep`·`tokio::task::yield_now`)으로 교체하고, `test_utils`
  `TestRuntime::r#yield` 의 `todo!()` 도 동형으로 구현. 루트 `Cargo.toml` tokio 에 `time` 피처 추가.
- 왜: 배포 바이너리가 `System.currentTimeMillis()`·`Thread.sleep()`·`new Date()` 등 시간 API 를
  부르는 순간 Rust 패닉으로 즉사했으나, 테스트 코퍼스가 해당 API 를 0건 사용해 CI 가 초록이었음.
- 사용자 영향: 시간 API 를 쓰는 모든 자바 프로그램이 이제 정상 동작. `test_data/TimeApi`
  픽스처(currentTimeMillis/yield/sleep/Date, 결정론적 단언)가 `RuntimeImpl` 경로 통합 테스트로
  상시 회귀 감시.
- 후속 추천: ① `jvm_rust/src/interpreter.rs:629` 의 잔여 `todo!()` 제거(별건), ② Timer/Object.wait
  경로도 픽스처 확장, ③ 픽스처 .java 소스 보관 체계(현재 .class+.txt 만 커밋하는 관례).
