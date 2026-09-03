# STATE

## 진행중
- [rustjava-upstream-sync-remeasure-s5-s7-and-lock-restore-basis] ★**PR #18 착지 «뒤» 기준으로 S5~S7 재측정
  + 「충돌 수에 base 병기」를 §5 상시 규칙으로 채택.** ★**코드 변경 0 · 문서 전용.**
  세 base 병기: 같은 컷 `ba5797b` 가 **19**(계획 `03438b0`) · **112**(복원 전 `3a59776`) · **4**(복원 후 `8c1238b`) —
  ★**28배 차이. base 없는 충돌 수는 검증 불가다.**
  ★**새 충돌(복원 후)**: **S5 +3** · **S6 +0** · **S7 +1**. ★**S5 는 「충돌 0 물량」이 아니다** —
  그 3건은 `Cargo.lock`(재생성) · `string.rs`(★**설계 판단**) · `test_timer.rs`(**S4 가 넣은 2000ms 여백**)이고
  ★★**전부 upstream 이 아니라 «우리가 앞 회차에 남긴 로컬 분기»가 만든다.**
  ★**「예측은 하한」이 S7 에서 처음 깨졌다**(예측 +3 ↔ ⓐ+2 · ⓒ+1) — `Cargo.lock` 이중 계상 ·
  `class_format_error.rs` 경로 오기 · ★**그 파일과 `throwable.rs` 둘 다 «우리 쪽»이 `3296139` 와 바이트 동일**이라
  upstream 변경(**+4/−3** · **+81/−29**)이 깨끗이 적용된다(S3 가 수렴시켰다).
  ★★**[2026-09-04 정정] 초판이 `class_format_error.rs` 를 「upstream 변경 0」으로 적은 것은 «거짓»이고 «0 인 쪽이 반대»였다** —
  `diff 3296139 ba5797b` = **+4/−3** · `diff 3296139 origin/main` = **0줄**(블롭 `0dbd369a`).
  ⇒ ★**「upstream 이 안 건드린다」가 아니라 «우리가 손대는 순간 충돌한다»** 이다.
  ★**부수 발견 — S8 이 필요하고 남은 것 중 제일 크다**: `ba5797b..upstream/main` **12커밋** · ⓒ 누적 **11**(+7) ·
  `java_runtime/`→`rustjava-runtime/` · `test_data/`→`test-data/` **개명 스윕**이라 우리 픽스처에 꽂힌다.
  **PR #19 대기 — 게이트③ 미착지.**
- [rustjava-upstream-sync-squash-defeats-convergence] ★**S1~S4 가 착지하고도 fork 가 upstream 에
  한 걸음도 가까워지지 않은 근인을 확정하고 계보를 기록했다.** 근인 = 게이트③ 제품 repo **`--squash`**.
  증명은 **머지커밋 부모 수**다 — `6bfe97c4`·`11ef5010`·`4bb796de`·`3a597768` **전건 1개**(커밋 7·10·15·21이
  각각 1로 접힘). ★**반증 시도는 실패했다(= 가설이 맞다)**: 브랜치 `34a4235` 는 부모 **2개**
  (`c80638a`+`3296139`)인 진짜 머지 ⇒ **계보는 브랜치에 있었고 스쿼시가 버렸다**(cherry-pick 가설 기각).
  처방 = `origin/main` 위 `git merge -s ours 3296139` — **트리 오브젝트 SHA 동일**(`c4f57d10…`)로 트리 변경 0.
  ★`git diff --stat` 빈 출력은 `-s ours` 정의상 항상 참이라 근거로 쓰지 않았다(S4 교훈).
  효과: `merge-base` **`62cf0c6a` → `3296139c`** · behind **33 → 18** · 컷 조상 **0/4 → 4/4**.
  ★★**이 PR 이 `--squash` 로 착지하면 위 전부가 무효다** — 반드시 `gh pr merge --merge`.
  ★**회차마다 `-s ours` 를 다시 넣는 지금 방식은 러닝머신이다** — S4 의 `c80638a` 가 정확히 그 처방이었는데
  PR #17 의 스쿼시에 함께 지워졌다(이 리니지에서 네 번 반복). **PR 대기 — 게이트③ 미착지.**
- [rustjava-upstream-sync-s4] upstream 컷 `3296139`(#184 GlobalRef · CLI classpath · CDC text) 머지 —
  충돌 **2** 해소(`java/lang/thread.rs` · `jvm/src/jvm.rs`). ★**첫 조치가 `git merge -s ours --no-ff 822504b`**
  — 그것이 ★**충돌 20 → 2**를 만들었다. **PR 대기 — 게이트③ 미착지.**
  ★★**계획서의 「S4 새 충돌 0」 예측은 틀렸다 — 실측 2건**이고, `thread.rs` 는 **S1·S3 에 이어 세 번째**다.
  ★**`test_timer_periodic` 여백을 넓혔다(500→2000ms) — ★«회귀»가 아니라 «만성 경계 테스트»다**(단정 불변).
  ★**컷 양쪽이 같은 비율로 흔들린다**(조건 맞춘 교대 실측 · ①절) — 전 판본의 「컷이 들여왔다」는 **틀렸다**.
- [rustjava-coverage-workflow-codecov-token-red] `coverage` 상시 red 해소 —
  `fail_ci_if_error: false`. ★**실증: 착지 전 브랜치에서 «이 저장소 최초의 green coverage»**
  (25번째 run, 앞선 24건 전부 red). **PR 대기 — 게이트③ 미착지.**

## 완료
- [rustjava-worklog-mandate-decision-and-local-gate] ★**워크로그 작성을 DoD «의무»로 결정 + 형식 잠금을
  로컬 DoD 4번째 명령으로 편입.** ★**코드 변경 0 · 새 도구 0 · CI 워크플로 무접촉.**
  ★**먼저 쟀다**: 규약 착지(`b3a4cf4` · 2026-08-26) 이후 착지 **4회차 중 3건(75%)** 작성 ·
  유일한 미작성(PR #13)은 **부모가 정확히 `b3a4cf4`** 라 규약을 알 수 없었다(알 수 있었던 회차만 **3/3**).
  ★★**관측이 높은데도 의무화한 이유 = 실패가 «조용하고 잠글 수 없다»** — 잠금은 **존재하는 `.json` 만**
  검사하므로(비소급 설계) 아예 안 쓴 회차는 **red 0 · 카드만 0** 이다. 표본 3건 · 전건 같은 리니지.
  ★**되돌릴 수**(`AGENTS.md` §Round Worklog · 측정 명령 동봉): **10회차 착지 시점** 재측정 —
  ⒜미작성 ≥ 2 ⇒ 문안을 빼거나 **기계 강제로** ⒝열린 카드 < 5 ⇒ **의무 재검토**.
  ★**`cargo test` 안으로 안 넣었다**: `serde_json` 이 `Cargo.lock` 에 **부재**(새 의존성 + 6셀 빌드) ·
  `python3` shell-out 은 python 없는 머신에서 **오탐 red** ⇒ 「오탐 0」 절대 조건 위반.
  대가 = **+0.10s**(3명령 warm 합계 60.2s 대비 **+0.17%** · 완전 캐시 회차 9.36s 대비 +1.1%).
  ★게이트③ 완료: PR #20 — ★★**`--merge` 착지**(★`--squash` 아님. `rustjava` 는 `upstream-sync-repos.conf`
  등재 repo 라 스쿼시가 계보를 지운다 — #11·#13·#16·#17 이 그 형태였다). 머지커밋 sha 는 회신 `merged:` 참조.
- [rustjava-upstream-sync-s3] upstream 컷 `822504b`(#180 오류 분류) 머지 — 충돌 **11** 해소.
  ★**S2(PR #13) 브랜치 «위에» 쌓았다** — 당시 `main` 에 S2 가 없어 base 를 `main` 으로 잡으면 S2 의 충돌
  5건을 다시 만나기 때문이다. ★게이트③ 완료: PR #16 스쿼시 머지 → main **`4bb796d`**(2026-08-26).
  ★★**착수 시 「upstream 조상 무손상이라 `-s ours` 불필요」로 적었는데, «축을 하나 놓쳤다»** —
  #13 이 스쿼시로 착지하자 **upstream 조상(`822504b`)은 그대로인데 `origin/main` 과의 조상이 끊겼다**
  (`merge-base` = `b3a4cf4` · `11ef501` 이 조상 **아님**) ⇒ main 과 **6충돌**(원장 1 + 코드 5, 내용은 전부 동일).
  ⇒ 게이트③이 `git merge -s ours --no-ff 11ef501`(트리 무변경 실측)로 복원해 **충돌 0**으로 만들었다.
  ★**교훈: 조상은 «upstream 축»과 «origin/main 축» 둘이다. 스쿼시가 끊는 것은 후자다.**
- [rustjava-upstream-sync-s2] upstream 컷 `af4f6f8`(#177 CLDC 1.1) 머지 — 충돌 **5** 해소.
  ★**PR #11 이 스쿼시 머지돼 upstream 조상이 끊겨 있었다** — `-s ours` 로 `1f356ae` 를 부모로 기록해
  복원한 뒤 머지했다(트리 무변경). 복원 전 충돌 **15** → 복원 후 **5**.
  ★게이트③ 완료: PR #13 스쿼시 머지 → main `11ef501`(2026-08-26). ★착지 전 base 를 `main` 으로 당겨
  **#14(beta clippy)를 들여와** CI red 를 풀었다(핀 `eaa5668` rc=1 CI_RED → `df3b04a` rc=0 CI_GREEN).
- [rustjava-worklog-json-proposals-convention] 회차 워크로그 `docs/worklog/` `.md`+`.json` 한 쌍 규약 이식.
  ★게이트③ 완료: PR #15 스쿼시 머지 → main `b3a4cf4`.
- [rustjava-ci-beta-clippy-double-must-use-red] beta clippy `double_must_use` 13건 red 해소.
  ★게이트③ 완료: PR #14 스쿼시 머지 → main `dde85ce`.
- [rustjava-claude-md-prune] `CLAUDE.md` 프룬(autonomous-sop 삭제 + Goal/Constraints/DoD 신설).
  ★게이트③ 완료: PR #8 스쿼시 머지 → main `00bddf3`(2026-08-18). ※구판 「좌초 중」 기재는 폐기.
- [rustjava-upstream-sync-s1-tracing-cut-1f356ae] upstream 컷 `1f356ae` 머지(충돌 2 해소 · tracing 축 ·
  `System.setProperty` 서술자 파손 1건 추가 처리). ★게이트③ 완료: PR #11 스쿼시 머지 →
  main `6bfe97c`(2026-08-17). ※원격 브랜치는 repo 설정 `deleteBranchOnMerge=true` 로 자동 삭제됨.
- [rustjava-runtime-time-todo-impl] RuntimeImpl 시간 API `todo!()` 3건 제거(now/sleep/yield) +
  test_utils `r#yield` 구현 + tokio `time` 피처 추가 + 회귀 잠금 픽스처(`test_data/TimeApi`).
  ★게이트③ 완료: PR #2 스쿼시 머지 → main `13ab950`(2026-07-23), 브랜치 정리 완료.
- [rustjava-classfile-parse-error-propagation] 클래스파일 파싱 실패를 패닉 대신
  `java.lang.ClassFormatError` 로 전파(절단/매직 불일치/미지원 상수풀 태그 구분).
  ★게이트③ 완료: PR #3 스쿼시 머지 → main `549b9eb`(2026-07-23), 브랜치 정리 완료.
- [rustjava-tracing-attributes-pin-removal] `#[tracing::instrument]` 1건을 수동 span 으로 대체,
  `tracing-attributes` 상한 핀 제거(tracing 0.1.41→0.1.44 언프리즈), wasm32 clippy CI 커버리지
  교정. ★게이트③ 완료: PR #4 approve 핀 `0a19f38` 확인 → main 충돌 해소(docs-only) 후
  스쿼시 머지(2026-07-23), 브랜치 정리 완료.
- [rustjava-unsupported-charset-exception] 미지원 charset `unimplemented!()` 패닉 3지점을
  `java.io.UnsupportedEncodingException`(신설) throw 로 전환, String↔InputStreamReader 지원
  charset 을 공용 `charset::Charset` 으로 일치(ISO-8859-1/US-ASCII 가 Reader 에서도 동작).
  부수: `System.setProperty` 반환 시그니처 JDK 규격화(Object→String, jvm 부트스트랩 포함),
  `Throwable.getMessage()` 신설, 픽스처 `test_data/UnsupportedCharset`.
  ★게이트③ 완료: PR #5 approve 핀 `dd9fcdf` 확인(이후 이동분은 문서화된 main 동기화 머지
  3건뿐, diff-of-diffs 로 코드 동일성 검증) → 스쿼시 머지(2026-07-23), 브랜치 정리 완료.
- [2026-07-25-rustjava-branch-hygiene] 원격 잔존 브랜치 2건 판정. ①
  `dependabot/cargo/tracing-attributes-0.1.31` → PR #4 로 `tracing-attributes` 의존 자체가
  소멸(Cargo.toml/lock 전무)해 패치 대상 라인 부재 → origin 에서 삭제 완료. ②
  `wie-ktf-hardening` → 12커밋 중 8건이 upstream/main 에 스쿼시 반영/대체(#174~#182),
  4건 유효 잔존 → **혼재 판정으로 보존**, 커밋별 판정표를 총괄에 제출(코드 변경 0).

## 다음

### ①(최우선) upstream 동기화 — ★**S1~S4 착지 완료 · 계보 복원 완료(2026-09-03)**. 정본 = `docs/upstream-sync-approach.md`

★★**[2026-09-03 갱신] 구판 「S4 착지 대기」는 낡았다** — PR #17(S4)·#18(계보 기록)이 **전건 머지**됐고
★**#18 은 `--merge` 로 착지해** `merge-base origin/main upstream/main` = **`3296139c`** · behind **30** ·
열린 PR **0**. ⇒ ★**「앞 회차가 착지하면 뒤 회차의 기준선이 바뀐다」가 이제 처음으로 참이다.**

★**다음은 S5(`c4665b0` · Java 1.2 API 확장). 단 「충돌 0 물량」이 «아니다»** — 2026-09-03 재측정(base = 현 `main`
`8c1238b` · `merge-base 3296139c`)으로 **새 충돌 3건**: `Cargo.lock`(재생성) ·
`java_runtime/src/classes/java/lang/string.rs`(★**설계 판단** · upstream `3296139`→`c4665b0` **+285/−31** ↔
우리 `3296139`→`origin/main` **+8/−28**) ·
`java_runtime/tests/classes/java/util/test_timer.rs`(**S4 가 넣은 500→2000ms 여백을 다시 얹는 기계 작업**).
**S6 새 충돌 0 · S7 새 충돌 1**(`thread.rs`). ★상세·근거·세 base 대조표 = `docs/upstream-sync-approach.md` §5 「[2026-09-03 재측정]」.
★★**충돌 수를 적을 때는 base 를 반드시 병기하라 — §5 상시 규칙이다.**

**S4 실측(2026-08-27)**: 착수 시 `merge-base origin/main upstream/main` = ★**`62cf0c6`**(최초 공통조상) ·
`1f356ae`·`af4f6f8`·`822504b` 가 `origin/main` 의 조상 **전건 NO** — ★**스쿼시 3회가 족보를 원점으로 되돌렸다.**
⇒ 첫 조치 `git merge -s ours --no-ff 822504b` → `merge-base` **`822504b`** 복원.
★**무해성의 근거는 «`git diff --stat origin/main HEAD` 빈 출력»이 «아니다»** — `-s ours` 는 정의상 우리 트리를
유지하므로 그 출력은 **항상 참**이고 아무것도 증명하지 않는다. 근거는 ★**`--diff-filter=D` 0 + 양방향 전문 대조**다.
★**충돌 20 → 2**(`java/lang/thread.rs` · `jvm/src/jvm.rs`).
green 전건 rc=0 · `cargo test --all` **261 passed / 0 failed / 1 ignored**(S3 216 → +45).

★★**타이머 테스트 여백 1건 — ★«회귀»가 아니다. 전 판본의 「upstream 회귀를 들여왔다」 서술은 «틀렸다».**
`test_timer_periodic` 은 ★**컷 이전부터** 500ms 창에서 기대 10회 대비 **3~4회**만 도는 **만성 경계 테스트**다.
★★**측정 조건을 «섞지 마라» — 전 판본이 틀린 이유가 그것이다**(단독 결과와 병렬 결과를 나란히 놓았다).
조건을 맞춘 **교대 실행** 실측:

| 조건 | `4bb796d`(컷 **전**) | `3296139`(컷 **후**) |
|---|---|---|
| **단독 실행** · 교대 10회 | `3 3 3 3 4 4 4 4 3 4` · mean **3.5** | `4 3 4 3 4 3 4 3 4 3` · mean **3.5** |
| **전 스위트 병렬** · 교대 8회 | `4 4 4 4 3 4 3 4` | `4 3 3 6 4 4 4 4` |

⇒ ★**두 조건 어디서도 차이가 없다.** 「1회전 ~110~150ms」는 컷이 만든 값이 아니라 **양쪽 공통의 기존 값**이다.
★**사료가 그 자체로 반증이다**: upstream 이 같은 자리를 넓힌 `895d67d`(**2025-08-20**)·`ad8b477`(**2025-10-04**)는
★**둘 다 이미 `origin/main` 의 조상**이고, 근인으로 지목했던 `e557673`(GlobalRef)은 **2026-07-18** 이다
⇒ ★**지목된 커밋보다 «11개월 앞서» 이미 만성 flaky 였다.**
처분은 그대로다(`sleep 500 → 2000ms` · `run_count > 2` **불변** · `#[ignore]` 0 · 삭제 0) — 성격만 정정한다:
★**「가리는 여백」이 아니라 «만성 경계 테스트에 정상 여백을 준 것»이다.**
★**대가**: 감도가 내려간다 — red 문턱 1회전 **~167ms → ~667ms**(약 4배 둔화). 「5.6배 저하」는 여전히 red,
「2.4배 저하」는 이제 통과한다. 그 상한을 테스트 주석에 박았다.

★~~**「예측은 하한」이 이제 3회 연속 실측됐다**~~ → ★★**[2026-09-03 정정] 하한은 «절대적이지 않다» — S7 에서 처음 깨졌다**
(예측 `+3` ↔ 실측 계획기준 **+2** · 복원 후 **+1**). 초과는 «우리 로컬 분기»가 만들고(S5 `0↔+3`),
미달은 «앞 회차가 수렴시켜서» 난다(S3 가 upstream 오류 분류를 채택해 `throwable.rs` 충돌이 사라졌다).
⇒ ★**티켓 size/timeout 은 여전히 하한 쪽으로 잡되, 「하한이다」를 근거로 쓰지 마라.**
★**`thread.rs` 는 S1·S3·S4 «세 회차 연속» 충돌**한다 — upstream 이 `ThreadStartProxy::call` 을 반복 재작성하기 때문이다.
★★**[2026-09-03 정정] 「S5~S7 도 기본값으로 잡아라」는 «절반» 맞았다** — 실측상 **S5·S6 은 충돌하지 않고 S7 에서만** 다시 열린다.
해소 전략은 불변이다:
**upstream 본문을 뼈대로 취하고 `#[tracing::instrument]` 한 줄만 PR #4 의 수동 span 으로 치환**한다.

### ②`wie-ktf-hardening` 잔존분 — 2026-08-15 재판정으로 **4건 → 2건**
★**선행 확인 종결**: upstream `agent/runtime-api-gaps`(`6309d47`)는 **미머지가 아니다** —
**PR #190 로 2026-07-25 04:59Z 스쿼시 머지**(머지커밋 `c4665b0`, +33,109/−1,040)됐고
그래서 브랜치가 upstream 에서 **삭제**됐다. 즉 「삼키는지」는 이제 **upstream/main 에 직접 묻는다**.
항목별 판정(전부 `upstream/main` 트리 실측):

| 항목 | upstream/main 현재 | 판정 |
|---|---|---|
| `Timer.schedule(TimerTask;J)V` 1회성 | `timer.rs` `schedule_once` 존재 | **삼킴 → 무효** |
| `StringBuffer.insert(ILjava/lang/String;)` | `insert_string` 존재 | **삼킴 → 무효** |
| `StringBuffer.append([CII)` null→NPE | null 가드 **+ 범위 가드**까지 존재 | **삼킴 → 무효** |
| `ByteArrayInputStream.<init>([B)` null 가드 | `is_null()` 가드 존재(`be77dc6`) | **삼킴 → 무효** |
| `Class.forName` null/not-found 가드 | 둘 다 존재 | **삼킴 → 무효** |
| `Integer.byteValue()/shortValue()` | `java/lang/Number` 가 **구현 제공**, Integer 가 상속(`#176`) | **삼킴 → 무효** |
| `System.arraycopy` null 가드 | **부재** | ★**유효 잔존** |
| `String.<init>([B)` / `([C)` null 가드 | **부재** | ★**유효 잔존** |

★유효 잔존 2건의 **파괴력 근거**(추정 아님): `ClassInstanceRef::deref` 가
`self.instance.as_ref().unwrap()` 이라 **null 참조를 넘기면 Rust 패닉 = 호스트 프로세스 abort**다
(`jvm/src/class_instance.rs`). `System::arraycopy` 는 `jvm.load_array(&src, …)`,
`String::init_with_byte_array`/`init_with_char_array` 는 `jvm.array_length(&value)` 로 곧장 deref 한다.
★★**형제 호출부 «전수»**(2026-08-16 재실측 · `java/lang/String` 의 `<init>` 오버로드 **10건 전건** 대조).
가드가 **없어서 null 을 넘기면 호스트 abort** 인 것이 **7건**이다:

| `<init>` 서술자 | 구현 | null 이 닿는 자리 | 가드 |
|---|---|---|---|
| `([B)V` | `init_with_byte_array` | `jvm.array_length(&value)` | **없음** |
| `([C)V` | `init_with_char_array` | `jvm.array_length(&value)` | **없음** |
| `([BII)V` | `init_with_partial_byte_array` | `jvm.load_array(&value, …)` | **없음** |
| `([BLjava/lang/String;)V` | `init_with_byte_array_charset` | `jvm.array_length(&value)` | **없음** |
| `([BIILjava/lang/String;)V` | `init_with_partial_byte_array_charset` | `jvm.load_array(&value, …)` | **`charset_name` 만** 검사 · `value` **없음** |
| `(Ljava/lang/String;)V` | `init_with_string` | `value_range` → `jvm.get_field(this, …)` | **없음** |
| `(Ljava/lang/StringBuffer;)V` | `init_with_string_buffer` | `jvm.invoke_virtual(&value, "toString", …)` | **없음** |
| `([CII)V` | `init_with_partial_char_array` | — | ★있음 |
| `(II[C)V` | `init_with_shared_char_array` | — | ★있음 |
| `()V` | `init_empty` | 인자 없음 | 해당 없음 |

★**`get_field`/`invoke_virtual` 도 `&Box<dyn ClassInstance>` 를 받는다** — 배열 인자만의 문제가 아니다.
⇒ ★**가드가 «짝이 안 맞는» 것이 핵심이다**: `([CII)`·`(II[C)` 만 막혀 있고 `[B` 계열은 전부 뚫려 있다.
※**전역 수리는 불가** — `Deref` 는 `Result` 를 못 돌려준다. upstream 방식(진입부 `is_null()` 가드)이 정답이다.

### ③다음 회차 발권 후보(우선순위 순)

★★**0번 항목(`rustjava-pr8-claude-md-prune-disposition`)은 «해소됨» — 2026-08-27 S3 회차가 닫았다.**
구판은 「`reports/rustjava-claude-md-prune.review.md` 가 **없다** ⇒ 게이트②가 아예 돌지 않고 좌초」를
근거로 이 항목을 **최우선**에 뒀는데, 그 근거가 **둘 다 사실이 아니게 됐다**(실측):
- `Jun025/RustJava#8` = ★**`MERGED`**(`mergedAt` **2026-08-18T19:26:08Z** · 머지커밋 `00bddf3`).
  head `feat/rustjava-claude-md-prune` 는 머지와 함께 **삭제**됐다(`git ls-remote --heads origin` 잔존 0).
- `reports/rustjava-claude-md-prune.review.md` **실재**(2026-08-19 03:42) · 승계 `-fix` 리니지의
  `.done.md`/`.review.md` 도 **둘 다 실재** ⇒ ★**게이트②는 돌았고 3게이트를 완주했다.**
- 「현 `main` 의 `CLAUDE.md` 에 그 변경이 없다」도 해소 — `origin/main` 의 `CLAUDE.md` 에
  `## Goal` 절이 실재한다(= 프룬 판본이 정본).
★**교훈: 이 절이 «이미 끝난 일»을 최우선으로 가리키면 레인이 조용해진다** — 실제로 발권이 멈춘 채
18시간(`LANE_IDLE rustjava`)이 지났다. 「다음」 절 항목은 **닫히는 즉시** 닫아라.

⇒ **재부여된 순서**(위 0번이 빠지고 1→3 이 한 칸씩 올라온다):

1. ★**`rustjava-upstream-sync-s5` … `-s7`**(S1~S4 **완료** · 구판 `-32-commits` **폐기**) — ①의 머지를
   `docs/upstream-sync-approach.md` §5 의 **7회차**로 쪼갠다. **한 티켓 = 한 컷**이고,
   ★**순서대로**다 — **다음은 S5(`c4665b0`)**. ★**착수 첫 조치는 `git merge -s ours --no-ff 3296139`**
   (S4 착지가 또 스쿼시라 족보가 다시 끊긴다 — S2·S3·S4 가 전부 같은 형태였다). 각 회차 완료 정의 = 그 컷의 충돌 해소 + CI `rust.yml` 4종 green
   + 문서 §5 의 회차별 추가 조건(S1 tracing 0건 / S2 charset 잠금 / S3 `test_class_format.rs` /
   S4~S7 해소분 0 증명). ★**착수 시 충돌을 재측정하라** — 앞 회차 착지로 기준선이 바뀐다.
2. **`rustjava-null-guard-string-init-and-arraycopy`**(P2·S·low) — ②의 유효 잔존 2건 + 형제 전수.
   ★**①의 뒤**여야 한다 — ★**근거 정정(2026-08-16)**: 선행 이유는 **`string.rs` 가 충돌 목록에 있기 때문**이다.
   ★**`system.rs` 는 충돌 목록에 «없다»**(`merge-tree` 출력에서 `Auto-merging` 만 있고 `CONFLICT` 줄이 없다) —
   구판이 두 파일 다 충돌이라고 적은 것은 오류이니 **충돌 해소 대상으로 잡지 마라.**
   - **범위**: `System.arraycopy` + ②의 표에서 **가드 없음 7건 전부**
     (`([B)` · `([C)` · `([BII)` · `([BLjava/lang/String;)` · `([BIILjava/lang/String;)` ·
     `(Ljava/lang/String;)` · `(Ljava/lang/StringBuffer;)`).
   - **재현**: `System.arraycopy(null,0,dst,0,0)` · `new String((byte[])null)` · `new String((char[])null)` ·
     `new String((byte[])null,0,0)` · `new String((byte[])null,"UTF-8")` · `new String((byte[])null,0,0,"UTF-8")` ·
     `new String((String)null)` · `new String((StringBuffer)null)` → 전부 현재 **패닉**, 기대 `NullPointerException`.
   - **완료 정의**: 8케이스 픽스처 잠금 + 3종 green. ★가드 스타일은 upstream 기존 방식(진입부 `is_null()`)에 맞춘다.
3. **`rustjava-invokedynamic-cp-tags-15-18-support`**(P2·M·med) — 아래 ④ 참조.
   ★**「한 티켓으로 묶는 이유」가 2026-08-16 로 바뀌었다.** 구판 논거(「파서만 고치면 인터프리터가
   `todo!()` 로 죽는다」)는 ★**①머지 뒤 성립하지 않는다** — upstream 이 `jvm_rust/src/verifier.rs` 로
   **패닉 축을 이미 제거**했기 때문이다(④ 참조). 그리고 ③은 ① 뒤에 도는 티켓이다.
   ⇒ ★**새 논거**: ①머지 뒤 남는 것은 **「깔끔한 거부」에서 「실제 지원」으로 올리는 일**이고,
   그것은 **파서(태그 15~18 수용) 없이는 시작조차 못 하고, 부트스트랩 실행(`verifier`·인터프리터) 없이는
   끝나지 않는다** — ★**두 끝이 «같은 기능 하나»의 앞뒤라서 한 티켓이다**(구판처럼 「어느 쪽만 고치면
   반대쪽에서 죽는다」가 아니다).
   - ★**착수 전 필수**: ①머지 뒤 `jvm_rust/src/{verifier,error}.rs` 와 `classfile/src/constant_pool.rs` 의
     **실제 착지 상태를 다시 재라**. `jvm_rust/src/class_definition.rs` 는 **충돌 17파일에 이미 들어 있다** —
     즉 ①이 반드시 손대는 자리다.
   - ★**범위 축소 권고 불변**: 크면 「파서 태그 수용 + 명확한 미지원 예외」까지로 자른다.

### ④미해결 — ★「이미 처리됐다」는 통설은 실측으로 **거짓**이다
- ★`jvm_rust/src/interpreter.rs` `Opcode::Invokedynamic(_) => todo!()` 는
  **origin/main·upstream/main 양쪽에 그대로 살아 있다** — ★**문자열로는 참이지만 «도달 가능성»이 다르다**
  (2026-08-16 정정. 구판은 여기서 멈춰 «양쪽 동일»로 읽었는데 **틀렸다**).
  ★★**upstream 에는 `jvm_rust/src/verifier.rs` 가 새로 있고**(origin 에는 **없다**),
  거기서 `Opcode::Invokedynamic(_)` 를 만나면 `ClassDefinitionError::UnsupportedFeature("invokedynamic")`
  로 **거부**한다. 이 `verify` 는 `jvm_rust/src/class_definition.rs` 가 **클래스 정의 시점에** 부르므로,
  upstream 에서는 ★**인터프리터의 `todo!()` 에 닿기 전에 깔끔한 오류로 끊긴다.**
  방증: 같은 테스트가 origin 은 `test_invokedynamic_consumes_reserved_bytes`,
  upstream 은 ★`test_invokedynamic_is_rejected` 로 **이름부터 바뀌어 있다**(`classfile/src/opcode.rs`).
  ⇒ ★★**①머지가 끝나면 «패닉 축»은 소멸한다.** 남는 것은 **«거부» → «실제 지원»** 이다.
  ※`rustjava-classfile-parse-error-propagation` 이 이 축을 처리했다는 통설은 **여전히 거짓**이다 —
  그 티켓은 **클래스파일 파싱** 경로만 고쳤고 인터프리터·verifier 와 무관하다.
- ★javac 21 익명 내부 클래스 "Malformed class file": 미해결. 원인 후보가 좁혀졌다 —
  `classfile/src/constant_pool.rs` 의 태그 분기가 **origin·upstream 모두 1~12 까지뿐**이고
  **15(MethodHandle)·16(MethodType)·17(Dynamic)·18(InvokeDynamic) 분기가 없다.**
  javac 9+ 는 문자열 `+` 연결조차 invokedynamic 으로 낸다.
  ⇒ STATE 구판의 「태그 15~18 아님」은 **근거 없이 배제한 것**으로 보인다.
  ★**이 절반은 ①머지 뒤에도 그대로 남는다** — verifier 는 파싱이 끝난 뒤에 도는데, 태그 15~18 은
  **파싱 단계에서 먼저 막힌다.** 위 invokedynamic 축과 **한 티켓**(브리프 ③)으로 묶는 근거가 이것이다.
- ★InputStreamReader 디코더: **origin/main 에서 미해결**(read 마다
  `Charset::resolve(...).new_stream_decoder()` 로 새로 만든다). `rustjava-unsupported-charset-exception`
  이 고친 것은 **미지원 charset 패닉**이지 **경계 유실**이 아니다 — 별개 결함이다.
  ※단 upstream 은 완화책을 갖고 있다(`endOfInput` 필드 + UTF-8 lead-byte 역주사 · EUC-KR `>=0x81`
  홀드백). ⇒ **①의 머지로 함께 들어온다.** 별건 발권 전에 ① 이후 상태를 다시 재라.

### ⑤운영 메모 — ★2026-08-27 S3 게이트③ 실측으로 교체(구판 「열린 PR = #13 하나」는 **낡았다**)

★**열린 PR = 1건**(`gh pr list -R Jun025/RustJava --state open`):

| PR | 브랜치 | 상태 | 원장 |
|---|---|---|---|
| **#16** `[rustjava-upstream-sync-s3]` | `feat/rustjava-upstream-sync-s3` | `OPEN` · ★게이트② **approve**(핀 `3cf944d`) · 게이트③ 집행 중 | `.done.md`·`.review.md` 둘 다 有 |
| ~~#13~~ `[rustjava-upstream-sync-s2]` | — | ★**MERGED**(`2026-08-26T21:51:38Z` → main `11ef501`) | 3게이트 완주 |
| ~~#8~~ · ~~#14~~ · ~~#15~~ | — | **MERGED**(→ `00bddf3` · `dde85ce` · `b3a4cf4`) | 완주 |

★★**스택 PR 을 다룰 때 반드시 기억할 것**(2026-08-27 실사고급 근접): #16 의 base 가
`feat/rustjava-upstream-sync-s2` 였고 이 저장소는 `deleteBranchOnMerge=true` 다 ⇒ ★**#13 을 그냥 머지했으면
#16 이 base 소멸로 자동 CLOSED 될 자리**였다(reopen 불가). S2 게이트③이 **머지 «전»에**
`gh pr edit 16 --base main` 으로 선제 재타깃해 막았다. ⇒ ★**스택 PR 은 부모 머지 «전»에 자식 base 를 옮겨라.**

★**원격 브랜치**(`git ls-remote --heads origin` · 2026-08-27 실측) = **3건**:
`main` · `feat/rustjava-upstream-sync-s3`(PR #16 의 head) · `wie-ktf-hardening`.

| 브랜치 | 성격 | 처분 |
|---|---|---|
| `feat/rustjava-upstream-sync-s3` | PR #16 의 head — 게이트③ 집행 중 | 머지와 함께 자동 삭제(`deleteBranchOnMerge`) |
| `wie-ktf-hardening` | 보존 판정(2026-07-25) | 위 ②로 **잔존 가치가 2건까지 줄었다** — 브리프 ③-2 가 그 2건을 새 브랜치로 옮겨 심으면 ★**보존 근거가 소멸**한다 |

⇒ ~~★**다음은 S4(`3296139`)** — 남은 upstream 커밋 **26**~~ → ★★**[2026-09-03 갱신] 다음은 S5(`c4665b0`)** —
남은 upstream 커밋 **30**(`ba5797b` 뒤로 **12** 가 더 붙었다 · 헤드 `bd42427`). 열린 PR **0** · 원격 브랜치 = `main` · `wie-ktf-hardening`.
★**「7회차」로는 헤드에 닿지 않는다 — S8 발권이 필요하고, 그것이 남은 회차 중 제일 크다**
(ⓒ 누적 충돌 **11** · `java_runtime/`→`rustjava-runtime/` · `test_data/`→`test-data/` **개명 스윕**이 우리 픽스처에 꽂힌다).

- ★PR 발권 시 `--repo Jun025/RustJava` 명시(2026-07-22 upstream 오발행 사고 재발 방지).
- ★upstream 발신(PR·이슈·코멘트·push)은 **티켓이 명시 허가할 때만**. 기본은 조회뿐.
