# STATE

## 진행중
- [rustjava-upstream-sync-s3] upstream 컷 `822504b`(#180 오류 분류) 머지 — 충돌 **11** 해소.
  ★**S2(PR #13)가 아직 열려 있어 그 브랜치 «위에» 쌓았다** — `main` 을 base 로 잡으면 S2 의 충돌 5건을
  다시 만난다. 조상 무손상(`merge-base` = `af4f6f8`)이라 `-s ours` 는 **불필요**. **PR 대기 — 게이트③ 미착지.**
- [rustjava-upstream-sync-s2] upstream 컷 `af4f6f8`(#177 CLDC 1.1) 머지 — 충돌 **5** 해소.
  ★**PR #11 이 스쿼시 머지돼 upstream 조상이 끊겨 있었다** — `-s ours` 로 `1f356ae` 를 부모로 기록해
  복원한 뒤 머지했다(트리 무변경). 복원 전 충돌 **15** → 복원 후 **5**. **PR #13 · 게이트② approve · 게이트③ 대기.**
- [rustjava-coverage-workflow-codecov-token-red] `coverage` 상시 red 해소 —
  `fail_ci_if_error: false`. ★**실증: 착지 전 브랜치에서 «이 저장소 최초의 green coverage»**
  (25번째 run, 앞선 24건 전부 red). **PR 대기 — 게이트③ 미착지.**

## 완료
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

### ①(최우선) upstream 동기화 — ★**S3 착지 대기(2026-08-27)**. 정본 = `docs/upstream-sync-approach.md`

★**S3(`822504b` · 오류 분류 축)까지 머지 완료 · PR 대기 중이다. 다음은 S4(`3296139` · 물량 회차).**

**S3 실측(2026-08-27)**: ★**조상은 끊기지 않았다** — S2 가 PR 로만 열려 있고 아직 스쿼시되지 않아
`merge-base HEAD upstream/main` = `af4f6f8` 그대로였다. ⇒ ★**`-s ours` 복원은 «불필요»했고 하지 않았다.**
★**대신 브랜치를 `feat/rustjava-upstream-sync-s2` 위에 쌓았다**(base = `main` 으로 잡으면 S2 의 충돌 5건을
다시 만난다). `origin/main` 의 신규 2커밋(#14 beta clippy · #15 worklog json)은 따로 머지해 얹었다.
충돌 **11** — 계획서 예측 **+9** 에 **2건이 더 붙었다**: ⑴`AGENTS.md`(#15 가 만든 워크로그 절 ↔ upstream
`Testing Boundaries` 절 · 계획서 작성 시점에 없던 파일) ⑵★**`thread.rs` 가 «다시» 충돌했다** — S1 이 이미
해소한 자리인데 `822504b` 가 같은 함수를 재작성했다. ⇒ ★**「앞 회차가 닫은 파일은 다시 안 나온다」는 전제는 틀렸다.**
green 전건 rc=0 · `cargo test --all` **216 passed / 0 failed / 1 ignored**(S2 191 → +25).
★**계획서 §4-A 가 예고한 대로 `tests/test_class_format.rs` 가 «충돌 0으로» 깨질 뻔했다** — 문구 단정 3건
(`"Truncated"`·`"tag 18"`·`"magic"`)을 **삭제**하고 `ClassFormatError` **종류 단정은 유지**해 4/4 통과.
★**§4-B 의 `charset.rs` dead-code red 도 발동하지 않았다** — upstream 의 `decode_str`/`encode_str` 중복
표를 **지우고** 우리 `charset::Charset` 으로 라우팅했다(호출자 7건 유지). 단 ★**기본 charset 경로는
upstream 의미를 취했다** — JDK 는 `new String(byte[])`·`getBytes()` 에서 미지원 charset 에 예외를 던지지
않는다(명시 charset 경로만 던진다). 우리 구판은 네 경로 전부에서 던졌다.

★★**S3 착수자에게 — 조상 복원을 먼저 확인하라.** S2 의 PR 도 스쿼시로 착지하면 `1f356ae`·`af4f6f8`
둘 다 다시 조상에서 끊긴다. 착수 시 `git merge-base origin/main upstream/main` 이 `af4f6f8` 가 아니면
S2 가 한 것과 같은 `git merge -s ours <직전 컷>` 을 **먼저** 하라. 안 하면 `merge-tree` 가 base 부터
전부 재생해 충돌 수가 3배로 부풀고, 이미 해소한 자리를 다시 해소하게 된다(S2 실측 **15 → 5**).

**S2 실측(2026-08-24)**: 충돌 **5** — S1 이 예고한 파일명과 **정확히 일치**
(`io.rs`·`input_stream_reader.rs`·`unsupported_encoding_exception.rs`·`loader.rs`·`test_input_stream_reader.rs`).
green 전건 rc=0 · `cargo test --all` **191 passed / 0 failed / 1 ignored**(S1 169 → +22).
★**`charset.rs` dead-code red 예측은 «발동하지 않았다»** — 우리 `Charset`(4종)이 upstream 의 인라인
2종보다 넓어 정본으로 남았고, 호출자는 오히려 **5 → 7건**으로 늘었다. 예측이 전제한 「upstream 판본을
통째로 취한다」가 성립하지 않았기 때문이다.
★★**S1 이 이름 붙인 형태가 이번엔 «조용한 중복»으로 나왔다** — `Throwable::getMessage` 를 우리와 upstream 이
**바이트 동일하게, 다른 위치에** 추가해 git 이 **양쪽 다** 머지했고 `E0592 duplicate definitions` 로
빌드가 깨졌다. 충돌 마커도 clippy 도 못 잡고 **컴파일만이 잡는다.**
★**`--theirs` 로 통째 해소한 파일은 «우리 줄이 지워졌는지» 반드시 되짚어라** — `loader.rs` 에서
`ClassFormatError::as_proto()` 등록 **1줄**이 그렇게 사라져 `test_class_format` 3건이 죽었다.
S2 는 이후 「base 이후 우리가 추가한 전 줄이 머지 트리에 살아 있는가」를 기계로 훑어 확인했다.

**S1 실측(2026-08-17 13:1x)**: `merge-tree` 충돌 **2 그대로**(`lang.rs`·`thread.rs`) — 계획서 예측과 일치.
green 전건 rc=0 · `cargo test --all` **169 passed / 0 failed / 1 ignored**.
★**계획서가 이름 붙인 3위험 중 S1 에서 실제로 터진 것은 tracing 하나뿐**이다 —
`tests/test_class_format.rs` 4/4 통과(upstream `classfile/src/error.rs` 재작성은 S3 컷 `822504b` 에 온다) ·
`charset.rs` 호출자 2건 생존(clippy green).
★★**대신 계획서가 «몰랐던» 파손이 하나 나왔다 — `java/lang/System.setProperty` 서술자**:
우리 PR #5 가 `…)Ljava/lang/String;` 로 고쳤고(**실제 javac 바이트코드**
`test_data/UnsupportedCharset.class` 상수풀이 그 서술자다 — JDK 규격상 우리가 옳다),
upstream 은 여전히 `…)Ljava/lang/Object;` 다. 충돌 0으로 우리 쪽이 머지되는데 upstream PR #176 이
새로 들여온 wrapper 테스트 6개 호출부가 `Object` 서술자를 박아 두어 **`NoSuchMethodError` 3건**이 났다.
⇒ 우리 서술자를 유지하고 **upstream 테스트 호출부 6곳을 고쳤다.**
★**교훈: 「충돌 목록 밖 파손」은 우리 «테스트»만이 아니라 우리 «프로덕션 서술자 변경»에서도 나온다.
그리고 그것은 upstream 이 «앞으로» 들여올 테스트에 의해 뒤늦게 터진다 — S2~S7 에서도 같은 형태를 예상하라.**

아래는 접근안 문서의 요약이고, 착수 전 **문서를 읽어라**.

**재실측(2026-08-16)**: `rev-list --left-right --count origin/main...upstream/main` → **`10  33`**
(선행 08-15 의 `9 32` 는 낡았다) · 공통조상 `62cf0c6` · origin tip `85f294a` · upstream tip `ba5797b`.
★**충돌 17 → 19파일**. 증분 2건의 원인은 upstream 신규 커밋 `ba5797b`(#201, **319파일 +20,118/−5,729**)
하나다 ⇒ ★**충돌 목록은 반감기가 짧다. 회차 착수 시 반드시 다시 재라.**
baseline green: `fmt --check` rc=0 · `cargo test --all` **149 passed / 0 failed**.

**처분 요약**: `upstream 채택` **13** · `양쪽 병합` **5** · `재생성` **1**(`Cargo.lock`) · `우리 유지` **0**.

★**선행 전제 2건을 실측으로 정정했다**:
1. 「PR #3·#5 와 upstream 이 **정면 충돌**」은 **과대평가**다. add/add 두 파일
   (`unsupported_encoding_exception.rs`·`class_format_error.rs`)은 **의미 차이 0** —
   차이는 `ba5797b` 접근플래그 스윕과 `Ok(())` 문체뿐이다. **진짜 설계 결정은
   `classfile/src/error.rs` 단 하나**이고, 거기서도 **upstream 이 이긴다**(Java 예외 4종 대 1종).
2. 「charset 퇴행」의 범위는 **`string.rs` 가 아니라 `input_stream_reader.rs` 하나**다.
   upstream 이 `String::decode_str`/`encode_str` 에서 **동일한 charset 집합·동일 별칭 정규화**를
   독립 구현했다. ★게다가 **기본 charset 경로에서 폴백**해 JDK 규격상 upstream 이 더 옳다.

★★**충돌 목록에 «없는» 파일이 더 위험하다**(문서 §4):
- `tests/test_class_format.rs` — 우리 전용이라 **충돌 0으로 머지된 뒤 4건 중 3건이 실패**한다
  (`"Truncated"`/`"tag 18"`/`"magic"` 문구 단정 ↔ upstream 의 평문 `"Invalid class file"`).
- ★**tracing 함정** — `Cargo.toml` 2개는 **조용히 우리 쪽(PR #4, `attributes` 피처 없음)으로 머지**되는데
  `thread.rs` upstream 쪽에는 `#[tracing::instrument]` 가 있다 ⇒ **그대로 취하면 컴파일 파괴**,
  피처를 되살려 고치면 **PR #4 통째 되돌림**. 답은 «upstream 뼈대 + 수동 span 재적용».
- `java_runtime/src/charset.rs` — 충돌 없이 살아남지만 호출자를 잃으면 **dead code → clippy `-D warnings` red**.

**회귀 잠금**: ★`test_data/UnsupportedCharset.class`+`.txt` 는 **이미 실재하고 이미 돈다** —
`tests/test_class.rs` 가 `test_data/*.class` 를 **디렉터리 스캔으로 자동 발견**하고 기대 출력에
**`3` / `aéb`** 가 박혀 있어 ISO-8859-1 의 Reader 통과를 종단 잠금한다(드라이버는 양쪽 동일 = 충돌 없음).
추가로 이식 3건(`test_isr_iso_8859_1` · `test_{get_bytes,new_string}_unsupported_charset_throws`) +
**US-ASCII Reader 잠금 신규 1건**.

**단계 분할 — ★커밋 수로 자르지 마라**: 컷별 `merge-tree` 실측 결과
**19충돌 중 16이 앞쪽 7커밋(#173~#180)에서 발생하고 뒤 26커밋이 더하는 것은 3뿐**이다.
⇒ **축으로 7회차**: S1 `1f356ae`(tracing/PR #4 · 새충돌 2) → S2 `af4f6f8`(charset/PR #5 · +5) →
S3 `822504b`(오류분류/PR #3 · +9) → S4 `3296139`(0) → S5 `c4665b0`(0) → S6 `95ebc5c`(0) →
S7 `ba5797b`(+3). ★**S1~S3 이 판단의 전부**이고 S4~S7 은 물량이라 **우리 해소분 0 증명 + green** 으로 검수한다.
green 기준은 전 회차 CI `rust.yml` 4종 동일(문서 §5 에 회차별 추가 조건).

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

1. ★**`rustjava-upstream-sync-s4` … `-s7`**(S1·S2·S3 **완료** · 구판 `-32-commits` **폐기**) — ①의 머지를
   `docs/upstream-sync-approach.md` §5 의 **7회차**로 쪼갠다. **한 티켓 = 한 컷**이고,
   ★**순서대로**다 — **다음은 S4(`3296139`)**. 각 회차 완료 정의 = 그 컷의 충돌 해소 + CI `rust.yml` 4종 green
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

### ⑤운영 메모 — ★2026-08-27 S3 회차 실측으로 교체(구판 「열린 PR = 2건 · #8 좌초 중」은 **낡았다**)

★**열린 PR = 1건**(`gh pr list -R Jun025/RustJava --state open`):

| PR | 브랜치 | 생성 | 상태 | 원장 |
|---|---|---|---|---|
| **#13** `[rustjava-upstream-sync-s2]` | `feat/rustjava-upstream-sync-s2` | `2026-08-23T23:37:41Z` | `OPEN` · ★게이트② **approve**(핀 `eaa5668`) | `.done.md`·`.review.md` 둘 다 有 |
| ~~#8~~ `[rustjava-claude-md-prune]` | — | `2026-08-05T07:56:55Z` | ★**MERGED**(`2026-08-18T19:26:08Z` → main `00bddf3`) | 3게이트 완주 |
| ~~#14~~ `[…beta-clippy-double-must-use-red]` | — | — | **MERGED**(→ main `dde85ce`) | 완주 |
| ~~#15~~ `[…worklog-json-proposals-convention]` | — | — | **MERGED**(→ main `b3a4cf4`) | 완주 |

★**#13 은 게이트③(`rustjava-upstream-sync-s2-merge`)만 남았다.** ★**S3(PR 신설)은 #13 위에 쌓았다** —
`main` 에 S2 가 아직 없으므로 base 를 `main` 으로 잡으면 S2 의 충돌 5건을 **다시** 해소하게 된다.

★**원격 브랜치**(`git ls-remote --heads origin` · 2026-08-27 실측) = **3건**:
`main` · `feat/rustjava-upstream-sync-s2`(PR #13 의 head) · `wie-ktf-hardening`.

| 브랜치 | 성격 | 처분 |
|---|---|---|
| `feat/rustjava-upstream-sync-s2` | PR #13 의 head — 게이트③ 대기 | 머지와 함께 삭제(`--delete-branch`) |
| `wie-ktf-hardening` | 보존 판정(2026-07-25) | 위 ②로 **잔존 가치가 2건까지 줄었다** — 브리프 ③-2 가 그 2건을 새 브랜치로 옮겨 심으면 ★**보존 근거가 소멸**한다 |

★~~`feat/rustjava-claude-md-prune`~~ 은 **더 이상 없다**(PR #8 머지와 함께 삭제) — 구판 표의 그 행은 폐기.

⇒ ★**「발권 대기 태스크 없음」이 아니다** — 게이트③ 2건(#13 · S3 PR) + 브리프 3건이 서 있다.
- ★PR 발권 시 `--repo Jun025/RustJava` 명시(2026-07-22 upstream 오발행 사고 재발 방지).
- ★upstream 발신(PR·이슈·코멘트·push)은 **티켓이 명시 허가할 때만**. 기본은 조회뿐.
