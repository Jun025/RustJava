# REPORT

## [2026-09-04] 「우리 자산이 낡는다」 상시 조항 판정 — 넣지 않고 «구멍 하나»를 막았다 (rustjava-sync-contract-standing-clause-for-our-assets-going-stale)
- 무엇을: 운영자 채택 제안 둘(`2026-09-04-upstream-sync-s6#p1`·`…-s7#p1`)에 대한 **결정**이다.
  ★**결론: 상시 조항을 «넣지 않는다».** 대신 ★**로컬 DoD 가 CI 검사 5종 중 «wasm32 clippy» 한 줄을
  빠뜨리고 있던 것**을 고쳤다 — `CLAUDE.md` DoD 가 이제 **CI 명령 5줄을 축약 없이 그대로** 싣는다.
  ★**코드 변경 0 · 문서 2파일**(`CLAUDE.md` · `docs/upstream-sync-approach.md`).
- 왜: ★**목록 문서화로 시작하지 않고 «먼저 셌다»**(티켓이 그렇게 요구했다). 우리 자산 **8건**을
  ★**돌연변이로 깨뜨려** 무엇이 잡는지 실측했다 — 경로 문자열·`setProperty` 서술자·charset 라우팅·
  `ClassFormatError` 종류 단정은 **`cargo test` RED**, 수동 span 은 **clippy RED**,
  `double_must_use` allow 는 **깨져도 무해**, 워크로그 스크립트는 **로컬 DoD 와 CI 둘 다** 잡는다.
  ⇒ ★**「아무것도 없음」 칸이 «1개»**(CI 워크플로의 `--exclude <크레이트 이름>`)이고,
  계약이 정한 대로 ★**1개면 조항이 아니라 «그 하나를 고치는 것»이 처방**이다.
  ★★**게다가 그 1개도 «조용히» 실패하지 않는다** — cargo 가 `warning: excluded package(s) … not found`
  를 찍고 빌드가 깨진다 ⇒ ★**문제는 «침묵»이 아니라 «늦음»**(push 후 CI 에서만)이고,
  ★**근인은 「자산 목록이 없다」가 아니라 «로컬 DoD 가 CI 한 줄을 빠뜨렸다»** 였다.
- 사용자 영향: 런타임 동작 **무변경**. 회차가 로컬에서 CI 와 **같은 5줄**을 돌리게 되어
  ★**「로컬 green 인데 CI red」 부류가 이 축에서 사라진다**(S5·S7·S8 이 실제로 그 부류였다).
- 검증: CI 5줄을 **워크플로 문면 그대로** 실행 — 전건 **rc=0** · `cargo test --all` **554 / 0 / 1**(baseline 동수 · 새 red 0).
  ★**돌연변이는 전부 복구**했고 워킹트리는 문서 2파일만 바뀌었다.
- ★★**사료 — 「세 번」·「두 방향」을 확정하되 결론은 «한 조항으로 못 덮는다»**:
  S3(문구 단정 3건 · 테스트) · S5(io 5곳 · 테스트) · S6(regex 3곳 · 정독) = **⑴신규/판본 교체** ·
  S7(우리 테스트 5곳 · **컴파일**) = **⑵공용 API 변경** · S8(`rust.yml`·경로 4곳 · **CI만/테스트**) = **⑶개명**.
  ⇒ ★**잡는 그물이 각각 다르므로**, 조항 하나로 덮으면 **이미 그물이 있는 다섯 자리에까지 사람 확인을 얹게 된다**.
- ★★**재개 조건**(결정에 재개 조건이 없으면 «영구 종결»로 읽힌다):
  「**CI 가 치는 검사 중 로컬 DoD 에 없는 것이 1건이라도 생기면 다시 연다**」 ·
  ★**세는 명령을 `docs/upstream-sync-approach.md` §4 에 박았다** · ★**오늘의 값 = 0**(착수 시 **5**).
- 후속 추천: ⑴**게이트③은 `--merge`**(등재 repo). ⑵★**upstream 동기 «정기 축» 판정은 아직 열려 있다**
  (S8 워크로그 `proposals[1]` · behind 0 인 지금이 적기다) — 이 회차는 그것을 «건드리지 않았다».

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
