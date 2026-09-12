# STATE

## 진행중
(없음 — 2026-09-11 실측: 진행 티켓 0 · 열린 PR 0. ※「열린 PR 0」은 ★**이 PR(#35) 착지 시점 기준**이다 — 회신 시점에는 #35 자신이 열려 있다)

## 완료
  ★★**게이트③ 착지 — PR #38 · `--merge`**(등재 repo · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `934b3944` **불이동**(동봉 전 실측) · `ci-presence` **rc=0 CI_GREEN** ·
  자식 PR **0건** · ★**배포 워크플로 0개 ⇒ 배포 0**.
  ★**검수 minor 5건은 «고치지 않았다»**(계약 7 — 총괄 승계): ⑴⒜ 열머리 「규격이 «요구»」가 6자리 중 3자리(`readUTF`·`FileInputStream(File)`·`JarURLConnection(URL)`)에서
  **Java 8 Javadoc 보다 강하다**(그 자리 근거는 «관측된 구현 거동»이다 — ★**처분은 불변**: 현 거동이 호스트 abort 라 어느 축으로도 오답이고 회차가 근거를 «관측»이라 본문에 선언했다)
  ⑵`FileURLHandler` ⒞ 근거에 «오늘의 호출부» 논거가 섞였다 ⑶⒝⒞ 사유가 **코드에 없다**(worklog 에만 — 특히 `url.rs` 의 `handler` 는
  ★**다음 회차가 «빠진 가드»로 오인해 넣으면 그 자체가 규격 위반**이다) ⑷형제 PR #39 와 원장 2파일이 겹친다 ⑸워커 트리 미커밋 잔재.
  ★★**⑷는 이 회차가 «처분»했다** — 아래 「착지 순서」 참조.
- [rustjava-null-guard-spec-triage-for-constructor-and-collection-params] ★★**남은 20곳을 «JDK 규격»으로 삼분하고 «미충족»만 닫았다 — 산출물은 가드가 아니라 «분류»다.**
  채택 제안 `2026-09-11-null-guard-audit-and-io-buffer-guards#p0`. ★**전수표 정본 = `docs/worklog/2026-09-12-null-guard-spec-triage.md`**(여기서 다시 쓰지 않는다).
  ★**삼분**: ⒜NPE 의무 **11** · ⒝null 합법 **1** · ⒞규격 적합 게스트 코드로는 도달 불가 **8**(= 내가 다시 센 **20**).
  ★★**⒜ 11 중 «가드는 9» 다** — `Pattern.matches`·`split` 은 규격상 NPE 의무이나 **런타임이 이미 NPE 를 던진다**(감사의 **섀도잉 오탐**이었다)
  ⇒ 가드를 넣으면 **죽은 코드**다. ★**「⒜ = 가드」가 아니라 「⒜ 이면서 «미충족»일 때만」**이다.
  ★**규격 근거는 «관측»이다** — `AGENTS.md` 가 OpenJDK 소스 참조를 금지하므로(`src.zip` 로컬 실재하나 **열지 않았다**)
  허용 축인 **observable behavior** 로 **참조 JVM(OpenJDK 26.0.1)** 에 같은 호출을 넣어 쟀다.
  ★**대가를 적는다**: 측정 판본 **26** ↔ 타깃 **8** — null 계약 불변이라는 **가정**이고, 반증되면 ⒜/⒝ 가 흔들린다.
  ★★**⒞ 는 «도달 불가»가 아니라 «규격 적합 게스트 코드로는 도달 불가»다** — 이 런타임은 **접근 플래그도 `java.*` 금지도 강제하지 않는다**(실측 0건)
  ⇒ 손으로 만든 바이트코드는 package-private 멤버를 부를 수 있다. ★그 한정을 숨기지 않았다.
  ★**착수 «전» 런타임 실측이 분류를 갈랐다**(사이트별 개별 픽스처 8회): 6곳 **호스트 abort** ↔ `Pattern` 2곳 **이미 NPE**.
  ★가드별 커버리지도 **측정**(9곳 단일 개악): **6 red · 3 green** — 미커버 3은 「테스트 미작성」이 아니라 ★**픽스처 도달 불가**
  (`JarURLConnection`·`URLStreamHandler` 는 `ABSTRACT` · `ZipFile.getInputStream` 은 **`ZipOutputStream` 부재**로 zip 을 못 만든다).
  ★감사 `K` **20 → 11** — ★**그 11 을 «미해결»로 읽지 마라**(⒝1 + ⒞8 + 오탐2 ⇒ **규격상 닫을 것 0**).
  ★`cargo test --all` **554 / 0 / 1**(전체 실행) · DoD 7종 rc=0 · ⒝⒞ **무접촉**(diff 에 그 파일 0).
- [rustjava-null-guard-audit-remaining-runtime-entrypoints] ★★**런타임 전체를 «세고»(감사) 그중 «데이터 전송 버퍼» 18곳에 가드를 넣었다.**
  채택 제안 `2026-09-11-null-guard-string-init-and-arraycopy#p0`. 픽스처 `test-data/NullBufferGuards` **13케이스**(가드 **12/18** 커버 — ★**18곳 전건 단일 가드 개악으로 «측정»**).
  ★★**감사 표 — 재측 2026-09-12 · ★트리 병기**(정본 = `scripts/audit-null-guards.py`):
  **base `6da7d66f`** = **N 1,175 · M 52 · K 38** → **head** = **N 1,175 · M 34 · K 20** ⇒ **ΔM = ΔK = 18**(자기정합).
  ★★**[게이트② R2 정정] 커밋한 스크립트가 M 을 «2 낮게» 셌다**(50/32 로 신고) — `fn` 정규식이 이름 뒤 `(` 를 즉시 요구하고
  접두를 `pub `/`async ` 로만 받아 ★**제네릭 fn·`pub(crate)`/`pub(super)` 선언 34개가 통째로 안 보였다**.
  ★고친 뒤 **세 트리 × 세 독립 구현이 전건 일치**한다(아래).
  ★★**그리고 K 가 무사했던 것은 «설계가 아니라 운»이다** — 눈먼 fn 2건이 **우연히** `as_proto` 미등재였을 뿐이고,
  제네릭·`pub(crate)` 진입점이 **하나라도** 등재됐으면 K 가 **조용히** 낮아져 ★**이 감사가 「닫혔다」고 답했을 것**이다.
  ⇒ 그 «눈먼 구간»을 스크립트 docstring 에 박았다.
  ★★**[게이트② F1 정정] 초판이 적은 「M 58 · K 46」은 «다른 트리»의 수다** — **`eaad8e9c`**(= 선행 9곳 회차 «이전» main)에서 재고
  라벨만 `6da7d66f` 로 달았다. ⇒ ★**`46 − 18 = 28 ≠ 20`** 이라는 산술 모순이 그 혼입의 증상이었고, 검수자가 그것으로 잡았다.
  ★**이 저장소가 반복해 규탄한 「base 를 병기하지 않으면 수가 섞인다」의 교과서적 재현**이다 — ★그래서 이제 **두 트리를 나란히** 적는다.
  ★**판정 술어를 적어 둔다**(「보아하니」 금지): sink = `jvm/src/jvm.rs` 가 **`&Box<dyn ClassInstance>`/`impl AsClassInstance`** 로 받는 **17개 메서드**
  (`array_length`·`load_array`·`store_array`·`get_field`·`put_field`·`invoke_virtual`·`invoke_special`·`monitor_*` …) — 그 인자에 `&p`/`&mut p` 로 넘기면 **Deref/DerefMut 가 강제**된다.
  ★★**계측기를 «먼저 검증»했다 — 그리고 초판이 틀렸다.** 선행 회차가 닫은 **9곳**을 정답지로 삼아 가드 전/후 트리에 같은 자를 댔는데
  초판은 **3/9** 만 되찾았다(근인 = 정규식을 `"\n"+src` 에 돌리고 인덱스를 `src` 에 적용한 **off-by-one** — 괄호 스캔이 한 칸 밀렸다).
  고친 뒤 **8/9**. ★**남은 1건(`init_with_string`)은 `Self::value_range` «안»에서 deref 한다** ⇒ ★★**이 계측은 절차간(interprocedural)을 못 본다 — 그래서 K 는 «하한»이다.**
  ★**검수자의 「899 중 560 · 상한값」과 방향이 반대다**: 그쪽은 fn 단위 크루드 스캔(상한), 이쪽은 sink 한정 + 절차내(하한).
  ★★**세 번째 패닉 impl 을 찾았다** — `Deref`(:108)·`DerefMut`(:114) 외에 **`AsClassInstance::as_class_instance`**(`as_deref().unwrap()`)가 있다(monitor 계열이 그 경로다).
  ★**범위를 «좁게» 골랐다 — 「전부 고쳐라」가 아니다**: `java/io` **데이터 전송**(read/write 버퍼) + `String.getChars` **18곳**.
  ★★**생성자류를 «일부러» 뺐다 — 일괄 가드가 «틀리는» 자리가 실재한다**: `URL(context, spec, handler)` 는 JDK 규격상 **handler == null 이 «합법»**(기본 핸들러를 쓰라는 뜻)이다.
  ⇒ ★**잔여 20건은 «가드를 넣을지»가 아니라 «null 이 합법인지»를 규격으로 먼저 가려야 한다** — 후속의 본체가 그것이다.
  ★개악 대조 **양방향 · 4종**: ⑴18곳 전건 되돌림 → `test_class` **FAILED** ⑵**단일 가드**(`get_chars(dst)`)만 제거 → **FAILED**
  ⑶`reader.rs read(buf)` 만 · ⑷`writer.rs write_chars(chars)` 만 제거 → **각각 FAILED** ↔ 복원 전건 **ok**.
  ★★**[게이트② R1 정정 — 근인 서술이 «뒤집혀» 있었다]** 종전 문안은 「`InputStreamReader` 가 **오버라이드해서** base 에 닿지 못한다」였는데
  ★**정반대다 — 오버라이드가 «없어서» base 에 닿는다.** 실측: `InputStreamReader` 는 `read` 를 **`([CII)I` 로만** 등재하고
  `Reader` 가 **`([C)I`** 를 등재한다(`OutputStreamWriter`/`Writer` 도 같은 형상) ⇒ 1인자 호출은 **`Reader::read`** 로 해소된다.
  ⇒ ★**그래서 그때 더한 중첩 클래스 2케이스는 «이미 덮인 가드»를 또 덮었고 커버리지는 10/18 그대로였다**(검수 지적이 옳다) — **그 2케이스를 «지웠다».**
  ★**대신 «미등재 서술자»를 겨눴다** — `read(char[],int,int)`·`write(char[],int,int)` **3인자 호출**이 그 두 가드에 닿는 유일한 경로다.
  ★★**판정 축은 «클래스 이름»이 아니라 «서술자 + 등재 위치»다**(검수가 준 그 축을 그대로 썼다).
  ★★**그리고 이번엔 «추론»이 아니라 «18곳 전건 단일 가드 개악»으로 쟀다** ⇒ ★**12/18 · 미커버 6**(전부 파일 핸들 필요분 · 목록은 worklog).
  ★★**[게이트② F4] `String.getChars` 가드를 «범위 검사 뒤»로 옮겼다 — «선택»이다.**
  JDK 는 범위를 먼저 보므로 «잘못된 범위 + null dst» 면 **SIOOBE 가 이긴다**. 초판 위치는 그 선후를 **NPE 로 뒤집었다**(계약 4 「동작 의미를 바꾸지 마라」 위반).
  ★픽스처에 **선후 잠금 케이스**를 넣었고, 가드를 옛 위치로 되돌리면 그 케이스가 **red** 다(개악 ⑸).
  ★`cargo test --all` **554 / 0 / 1**(★**전체 실행** · 증감 0 이 맞다 — `test_class` 는 픽스처를 한 함수가 순회한다) · DoD 7종 rc=0.
  ★★**게이트③ 착지 — PR #37 · `--merge`**(등재 repo · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **3회차 만에 approve**(초판 → `-fix` → `-fix2`) · 핀 `77dec927` **불이동**(동봉 전 실측) · `ci-presence` **rc=0 CI_GREEN** ·
  자식 PR **0건** · ★**배포 워크플로 0개 ⇒ 배포 0**.
  ★★**반려 셋이 전부 «수»였다** — F1(두 트리 혼입 · K 46↔38) · R2(스크립트가 M 을 2 낮게) · R1(커버리지 근인 역전 · 10↔12).
  ⇒ ★**이 리니지가 남긴 규율: 「감사 회차는 «계측기»부터 정답지로 검증하라」**(그 검증이 off-by-one 과 눈먼 구간을 각각 잡았다).
  ★**검수자가 «독립 재구현 + 행 단위 대칭차 0» 로 세 트리를 재현**했고, `da3727c`(수정 전)에서 두 가드가 **green** 임을 대조군으로 확인했다.
  ★**검수 minor 3건은 이 회차가 «고치지 않았다»**(계약 7 — 총괄 승계 판정): ⒜`.gitignore` 에 `__pycache__` 부재
  ⒝새 `PARAM` 앵커가 `Option<ClassInstanceRef<…>>` 같은 **중첩 타입 파라미터를 세지 않는다** — ★**검수 실측 «현재 0건»**(기록해 둔다 · 다음 회차가 다시 재지 않도록)
  ⒞감사 스크립트 CI 미배선(= 선언된 선택이지 결함 아님).
  ★★**[게이트② F5] 감사 스크립트를 «커밋했다» — 초판의 「배선 없는 검사기는 낡는다」 판단을 뒤집는다.**
  근거: ★**작성자가 자기 수를 교차검증할 수단이 없어 F1 이 났다.** `scripts/audit-null-guards.py`(★CI 미배선 = **잠금이 아니라 감사**).
- [rustjava-null-guard-string-init-and-arraycopy-p0] ★★**null 인자 8경로의 «호스트 abort» 를 `NullPointerException` 으로 바꿨다.**
  채택 제안 `2026-09-11-s5-duplicate-issuance-stale-next-close#p0`. ★**가드 9곳**(`string.rs` 7 · `system.rs` 2) · 픽스처 `test-data/NullArgGuards` 9케이스.
  ★**제안은 「8경로」라 했는데 실측은 «9곳»이다** — `System.arraycopy` 가 `src`·`dest` **둘 다** 뚫려 있었고(제안·STATE ③-2 는 `src` 만 셌다) 그 둘은 다른 인자다.
  ★**대가 = 없음에 가깝다**(제안 문면 그대로 물려받되 재확인): upstream 관용구(진입부 `is_null()`)를 그대로 썼고 **삽입만**(`--numstat` 삭제행 **0** · 치환 아님) ·
  ★**의미 변경 0** — 가드 «앞»에서 패닉하던 입력만 예외로 바뀐다(정상 입력은 도달 경로가 동일).
  ★★**개악 대조로 공허하지 않음을 증명했다** — 가드 전건 되돌리면 `test_class` **FAILED**
  (`called Option::unwrap() on a None value` at `jvm/src/class_instance.rs:108` = STATE ② 가 지목한 `ClassInstanceRef::deref` 그 자리) ↔ 가드 복원 시 **ok**.
  ★`cargo test --all` **554 / 0 / 1** — ★**증감 0 이고 그것이 맞다**: `test_class` 는 픽스처를 «한 테스트 함수»가 순회하므로 픽스처가 늘어도 계수는 불변이다.
  ★**픽스처 컴파일 = `javac --release 8`**(major **52** = 기존 픽스처와 동일 · ★`invokedynamic`·CP 태그 15~18 **0건** — javac 9+ 함정을 피했다).
  ★**컴파일은 «스크래치»에서 했다** — `test-data/` 에서 치면 그 디렉터리의 `Exception.class` 가 `java.lang.Exception` 을 **가려** compile error 가 난다(실측).
  ⇒ ★**STATE ③-2 의 「유효 잔존 2건」이 닫혔다** · ②의 «가드 없음 7건» 표도 전건 닫혔다.
  ★**게이트③ 착지 — PR #36 · `--merge`**(등재 repo · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `9ba6db48` **불이동**(동봉 전 실측) · `ci-presence` **rc=0 CI_GREEN** ·
  자식 PR **0건** · 착지 diff 9파일(`.rs` 2 · 픽스처 3 · 원장 4) · ★**배포 워크플로 0개 ⇒ 배포 0**.
  ★★**검수자가 «자기 자리»에서 개악을 다시 쟀고, 회차보다 강한 결과를 냈다** — 가드를 **1개씩** 빼도 픽스처가 문다
  (A: `system.rs` `dest` 만 제거 → `class_instance.rs:114` `DerefMut` · B: `init_with_string_buffer` 만 제거 → `:108` `Deref`).
  ⇒ ★**위 본문이 `:108` 을 유일 지점처럼 인용한 것은 부정확하다 — 축이 둘(`Deref`/`DerefMut`)이다**(검수 minor · 문면은 사료로 보존).
  ★검수 부수 실측 1건: `ClassInstanceRef` 인자를 받는 fn **899개 중 560개**가 그 인자에 `is_null()` 가드가 없다(★**상한값** — 대부분 null 이 안 닿거나 deref 하지 않는다)
  ⇒ 후속 `rustjava-null-guard-audit-remaining-runtime-entrypoints`(P3)의 「열거가 아니라 세는 것」에 **근거가 생겼다**.
- [rustjava-upstream-sync-s5-java12-api] ★★**중복 발권 판정 — S5 는 «이미 착지»다(대전제 ⓒ 경로 · `status: blocked`).**
  실측(2026-09-11): `c4665b0` 은 `origin/main` 의 **조상** · PR #21 **MERGED**(2026-09-03T22:12:43Z · 머지커밋 `a0b5d3c`) —
  같은 일을 `rustjava-upstream-sync-s5-with-remeasured-conflicts` 가 이미 완주했다. S6(#22)·S7(#23)·S8(#24)도 착지해
  `merge-base` = **`bd42427`** · behind **1**(`2ce4717` · dependabot encoding_rs 0.8.35→0.8.40 · 임계 20 미만 ⇒ 동기 회차 불요).
  ★**근인 = 이 파일 `## 다음` 이 「다음은 S5」인 채 8일 낡아** LANE_IDLE 처방(「STATE.md 의 «다음»을 읽어라」)이 그것을 읽었다 —
  아래 ③ 절이 경고한 「이미 끝난 일을 가리키면」 형태의 ★**두 번째 재현**이다. ⇒ 이 회차가 `## 다음`·`진행중` 을 오늘 값으로 닫았다(코드 0줄 · 문서 전용).
  ★-fix 회차(게이트② F1): 초판 PR #35 가 «2일 낡은 로컬 `main`»에서 잘려 `CONFLICTING` — ★**이 회차가 고치는 병(낡은 것을 읽는다)을 자신이 밟았다.**
  `origin/main` 머지로 재해소(#33·#34 항목 보존 · force-push 0).
  ★-fix2 회차(게이트② F1-⑶): 「절차를 어떻게 고쳤는가」가 회신에만 있어 반려 — ★**다음 세션이 읽는 자리**(`AGENTS.md` §Git Workflow)에
  2줄을 박았다: ⒜브랜치는 `origin/main` 에서 자른다(로컬 `main` 신선도에 의존하지 않는 형태) ⒝PR 오픈 직후 `mergeable` 조회.
  ★근인 정정: 그 repo 는 **이미** 「sync local `main`」을 MANDATORY 로 적고 있었다 ⇒ 진짜 근인은 «현행 조항을 조회하지 않은 것».
  ★**게이트③ 착지 — PR #35 · `--merge`**(등재 repo · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② approve · 핀 `c980ef0d` **불이동**(동봉 전 실측) · `ci-presence` **rc=0 CI_GREEN** · 자식 PR **0건** ·
  착지 diff 5파일 **전건 문서** · ★**이 저장소 배포 워크플로 0개 ⇒ 배포 0** · `.rs` **0줄**.
- [rustjava-upstream-sync-squash-defeats-convergence] ★게이트③ 착지 — **PR #18 `--merge`**(2026-08-27T08:47:35Z).
  `-s ours` 계보 복원 + 「스쿼시가 족보를 접는다」 확정. 상세 = `docs/upstream-sync-approach.md`. ※구 «PR 대기» 기재 폐기(2026-09-11 정리).
- [rustjava-upstream-sync-s4] ★게이트③ 착지 — **PR #17 MERGED**(2026-08-27T01:28:44Z · 스쿼시 — 그 계보 절단을 #18 이 복원).
  컷 `3296139` 머지 · 충돌 2 해소 · `test_timer` 여백 500→2000ms(«회귀» 아님 단정 불변 · 사료는 아래 ① 절). ※구 «PR 대기» 기재 폐기(2026-09-11 정리).
- [rustjava-coverage-workflow-codecov-token-red] ★게이트③ 착지 — **PR #12 MERGED**(2026-08-17T05:34:32Z).
  `fail_ci_if_error: false` 로 coverage 상시 red 해소. ※구 «PR 대기» 기재 폐기(2026-09-11 정리).
- [rustjava-parity-unknown-tool-step-red-decision] ★**「`CHECK_TOOLS`·`SETUP_TOOLS` 둘 «다» 밖인 도구를 부르는 CI step 은 FAIL」로 정하고 집행.**
  채택 = `2026-09-08-parity-axis-a-tool-name#p0` · 선택지 **⒞ 조건부**(등록은 한 줄 — ⒜·⒝ 버린 이유는 §4 ⑺).
  ★★**발권 소견의 전제가 반증됐다** — 「목록 밖 0」이 아니라 **1**(`git config …` 셋업) ⇒ ★무조건 red 면 **첫날부터 위양성**.
  ★**지정 개악 `- run: npm test` 가 rc=0 → rc=1** · 반대 개악·우회 4종 red · ★**등록 경로 둘 다 통한다**(N4·N5) · ★**위양성 0**.
  ★**선행 술어 무손상**: `git diff --numstat f133cd30` = **33 0**(추가만). ★새 워크플로·새 잡 **0** · `.rs` **0줄**.
  ★직전 회차가 삼킨 `docs/upstream-sync-approach.md` **§5 제목 복구**(내가 낸 결함이다).
  ★**게이트③ 착지 — PR #34 · `--merge`**(등재 repo · 스쿼시는 부모 2개를 1개로 접어 계보를 잃는다).
  ★`ci-presence` **rc=0 CI_GREEN** · 핀 무이동 · 자식 PR **0건** · 배포 워크플로 **0개 ⇒ 배포 0**.
- [rustjava-parity-lock-axis-a-classify-by-tool-name] ★**파리티 락 축 A 의 소속 판정을 「`if:` 가 없는가」 →
  「어느 도구를 부르는가」(`CHECK_TOOLS = cargo·python3`)로 교체.** 채택 = `2026-09-07-…-parser-axis-design#p0`.
  ★★**지정 개악(`cargo test --all` 을 `if:` 아래로 + DoD 줄 삭제)이 rc=0 → rc=1** · 반대 개악·우회 3종 red ·
  ★**위양성 0**(무해 편집 4종 green). ★**축 A 원소 6 → 6 불변** — 순증이 아니라 «같은 6개를 다른 술어로» 잡는다.
  ★**「cargo 하나」로 좁히지 않은 이유는 수다**: 6 중 cargo 4 · python3 2 ⇒ 좁히면 **커버리지 33% 손실**.
  ★설계 문서 문면 1건 정정 — 「`if:` 를 붙이면 green」은 짧았다. **붙이기만 하면 red 이고, DoD 줄까지 지워야 green** 이다.
  ★**게이트③ 착지 — PR #33 · `--merge`**(등재 repo · 스쿼시는 부모 2개를 1개로 접어 계보를 잃는다).
  ★`ci-presence` **rc=0 CI_GREEN** · 핀 무이동 · 자식 PR **0건** · 배포 워크플로 **0개 ⇒ 배포 0**.
- [rustjava-parity-lock-per-repo-parser-axis-design] ★**파리티 락의 repo 별 «파서 축» 설계 — 결론: 공용 파서를 만들지 않고 «공용 계약»만 남긴다.**
  채택 제안 `2026-09-04-parity-sibling-repo-survey#p2`. ★**형제 repo 파일 편집 0 · 검사기 본체 변경 0 · 발권 0** · 측정 트리 = 각 repo `origin/main`.
  ★★**전제가 무너졌다** — `wie` 가 제안 «다음날»(2026-09-05) 스스로 포팅했고(`wie_cli/tests/dod_ci_parity.rs` +
  `tests/support/dod_ci_parity.rs` · 삭제는 `check-parity-lock-wired.mjs` 가 문다) 그 포팅이 ⒜~⒟ 를 우리보다 낫게 풀었다.
  ★제안 5축 판정 = **3확인 · 1부분반증(다중 워크플로 = 파서 요구가 아니라 «범위 결정») · 1정정(qts 축 B 는 «부재»가 아니라 «퇴화» — 원소 1개인데 DoD 가 침묵)**.
  ★**qts 는 집합 상등 락 자체가 틀린 도구다** — `make test`↔CI 마커 3잡 · `go` 로컬 조건부 skip↔CI 경성 게이트라 **어긋남이 정당**하다.
  ★★**이 저장소의 잠복 위음성**: 축 A 를 `if:` 로 분류하는데 지금 조건부 step 이 셋업이라 «우연히» 옳다 —
  `cargo test --all` 을 `if:` 아래로 옮기면 조용히 green 이다(wie 는 그 이동을 **이미 한** repo). ⇒ 후속 ⑴. ★★**게이트③ 착지 — PR #32 · `--merge`**(등재 repo · 스쿼시는 계보를 접는다).
  게이트② approve · 핀 `bd4c6da5` **불이동** · `ci-presence` **rc=0 CI_GREEN** · 자식 PR **0건** · 착지 diff 5파일 **전건 문서** ⇒ 배포 **0**(배포 워크플로 0개).
- [rustjava-upstream-behind-measure-scheduled-workflow] ★★**`behind` 를 «재는» 자리 신설 — `.github/workflows/upstream-behind.yml`.**
  채택 제안 `2026-09-04-upstream-sync-cadence-decision#p0`. ★임계는 정해졌는데(**`behind ≥ 20`** · PR #29 착지) ★**재는 주체가 없었다**
  (`git grep -lI 'rev-list --count' origin/main -- .github/ scripts/` → **0건**).
  ★**주기 = 주 1회**(`cron: "17 6 * * 1"`) — 선례(`rust-audit` 일간)를 **베끼지 않고** 임계 도달 속도로 정당화했다:
  behind 20 도달 **중앙 44일 · ★최소 9일** ⇒ ★**7 < 9 라 한 주기 넘게 못 놓친다** · 일간은 7배 비용에 판단 가치 0.
  ★★**알림 = ⒝ Job Summary + annotation · «항상 green»** — ★**취향이 아니라 실측으로 골랐다**:
  ⒜red 는 여기서 **안 듣는다**(선례 `rust-audit` 최근 **20 run 중 19 failure** 인데 ★**대응 티켓 0건**) ·
  예약 red 는 ★**main tip check-run 에 붙어**(실측 `8c1238b5` 에 `audit` failure **7건**) 게이트③의 「착지본 green」 읽기를 **흐린다**
  (★단 **PR 은 막지 않는다** — PR head 와 sha 가 다르다) · ⒞이슈는 ★**이 fork 가 issues 비활성이라 «불가»**.
  ⇒ ★**⒝ 가 «남은 것»이고 그 «수동성»을 알고 골랐다** — ★**재개 조건이 그 대가를 «잰다»**
  (「`behind ≥ 20` 이 7일 이상인데 회차 미개시」 · 세는 명령·오늘의 값을 §5-B ⒠ 에 박았다).
  ★**한 번 돌려 오늘의 값**: ★**behind `0`** · `merge-base` `bd42427` = upstream HEAD ⇒ 임계 미만.
  ★**upstream 발신 0**(remote 추가 + fetch 뿐 · `permissions: contents: read`) · ★**자동 발권 0** · ★**임계 20 무접촉** ·
  ★`bin/healthcheck`·orchestrator 무접촉 · `scripts/` 무접촉(파리티 rc=0) · `.rs` **0줄** · S9 미개시.
  ★게이트③ 완료: PR #31 — ★★**`--merge` 착지**(★`--squash` 아님 · 등재 repo). 머지커밋 sha 는 회신 `merged:` 참조.
  ★**착지 후 «워크플로를 실제로 돌려» 확인했다** — 구현 회차가 「본문 명령만 태웠고 워크플로 자체는 못 돌렸다」고 자인한 그 구멍이다.
  결과는 회신 참조. ★**게이트② 1회차 approve** — 반려 0.
- [rustjava-worklog-absence-machine-enforcement-decision] ★★**워크로그 «부재»를 기계로 잡을지 «판정» — 결론: 지금은 «넣지 않는다».**
  채택 제안 `2026-09-04-worklog-mandate-and-local-gate#p0`. ★**`scripts/check-worklog-json.py` 무접촉**(6축 무회귀 · rc=0) ·
  `.rs` **0줄** · 과거 backfill **0** · qts 잠금·upstream 리니지·파리티 검사기 **무접촉**.
  ★★**근거는 «사전 등록된 두 임계»가 «둘 다» 발화하지 않았다는 것 하나다** — 의무화 회차가 **데이터를 보기 «전»에**
  `AGENTS.md` 에 등록했고, ★**그 문서가 정한 재측 시점(「2026-09-04 이후 열 번째 착지 회차」)에 실제로 도달했다**(실측 **11**).
  ★**⒜ 미작성**: 창 `b3a4cf4..origin/main` first-parent **15건 중 1건**(6.7%) — 임계 **≥2** 미달.
  ★그 1건은 **규약보다 먼저 갈라진 PR #13**(`createdAt` 2026-08-23 ↔ 규약 착지 2026-08-26 · ★**인용이 아니라 실측**) ⇒
  ★**규약을 «알 수 있었던» 회차만 보면 0 / 14 = 100%**.
  ★**⒝ 열린 카드**: `.json` 15 · proposals **26** − disposed **13** = ★**13**(기준선 5) — 임계 **<5** 미달(★의무가 카드를 **늘렸다**).
  ★★**약점을 숨기지 않는다**: ⒤「100%」는 **단일 집행 주체의 습관**일 수 있다(리니지는 13개로 갈렸으나 git author 가 하나다)
  ⒥정본 술어가 **느슨**하다(「만졌다」는 옛 워크로그 한 줄 추가도 통과 — 엄격 술어로 재도 이번 창은 같은 14)
  ⒦**실패는 여전히 조용하다** ⇒ 「위험이 없다」가 아니라 「임계가 아직 발화하지 않았다」이다.
  ★**재개 조건 + 세는 명령 + 오늘의 값 `1`** 을 `AGENTS.md` §Round Worklog 에 박았다(**2 이상이면 재개** · ★엄격 술어).
  ★**재개하면 만들 것도 미리 적었다**: baseline sha 금지 · ★**PR diff 로 «조건부» 판정**(REPORT 후속 추천을 만졌으면 새 쌍 필수) ·
  정상 참작은 `createdAt` 으로(★제목·라벨 금지) · 기존 6축 보존.
  ★**측정 함정 1건 기록**: 「회차 = 부모 2개」로 세면 **스쿼시 착지를 놓친다**(그 술어로는 12/12 · 정본 술어로 15/1).
  ★게이트③ 완료: PR #30 — ★★**`--merge` 착지**(★`--squash` 아님 · 등재 repo `contracts/upstream-sync-repos.conf:22`).
  머지커밋 sha 는 회신 `merged:` 참조. ★**게이트② 1회차 approve** — 반려 0.
  ★**착지로 「재개 조건」이 `AGENTS.md` 에 살아 있다**: 엄격 술어 값이 **2 이상**이면 다시 연다(오늘 **1**).
- [rustjava-upstream-sync-cadence-decision] ★★**upstream 동기 «정기 축» 판정 — 결론: ⒝ 격차 기반 `behind ≥ 20`.**
  채택 제안 `2026-09-04-sync-contract-stale-assets-decision#p0`. ★**신설 0**(검사기·크론·워크플로) · `.rs` **0줄** · ★**S9 미개시**(behind **0**).
  ★★**제안의 전제(「벌어진 뒤에 받으면 훨씬 비싸다」)를 «먼저 재서» 반증했다**: 회차 커밋 수 ↔ 충돌의 상관계수 ★**부호가 «음»**.
  ★★**[게이트② 정정] 초판 표는 base·(델타/누적) 병기가 없어 기준이 섞여 있었다** ⇒ 정본을 **«누적»**으로 통일해 재계산:
  A 초판 **r=−0.169·합28** · B S4 정정 **r=−0.134·합30** · ★**C 정본 r=−0.155·합33**(**37커밋 → 14** ↔ **8커밋 → 19**).
  ★**세 열 모두 부호가 음 ⇒ 판정 불변** · ★**r 은 n=8 비무작위 표본이라 «부호 판정»으로만 쓴다**.
  ★**누적 충돌도 포화한다**: 고정 base 에서 **7커밋 16 → 33커밋 19**(26커밋 더 벌려도 **+3**).
  ⇒ ★**충돌을 만든 것은 «몇 개 왔는가»가 아니라 «무엇이 왔는가»**(우리 포크 지점 3곳 + 개명 스윕 같은 사건).
  ★**그런데 결론이 「아무것도 안 한다」가 되지는 않았다** — 반증된 것은 «비용이 격차에 비례한다»는 기전이고,
  「아무도 챙기지 않는다」는 위험은 그대로 참이다. ⇒ **축은 두되 임계를 포화점 «뒤»에** 놓았다.
  ★**시간 기반 기각**: upstream 이 **37% 의 주에 커밋 0**(12개월 144커밋/51주 · 중앙 **1/주**)이라 시간 축은
  ★**behind 0 인데 회차를 여는 «빈 회차»** 를 만들고, 회차 오버헤드(**티켓 3건**)가 고정이라 자주 받으면 **총비용이 는다**.
  ★**사건 기반 기각**: S3 충돌 **델타 +9**(누적 **11** · base S2 착지본 · merge-base `af4f6f8`)는 upstream **«신규» 파일**에서,
  S8 **누적 8 · 델타 +8**(base `3fb08a8` · merge-base `ba5797b`)은 **전 트리 개명**에서 왔다 ⇒ 경로 트리거가 대부분을 놓친다.
  ★**임계 20 근거**: 아래로는 **7커밋 포화점**, 위로는 **33 이 미측정 구역 입구**.
  ★★**[정정] 초판의 「실제로 아팠던 값」은 «지웠다»** — 그것은 이 판정 자신이 «금지한» 독법(8회차를 격차의 비용으로 읽는 것)이라 자기모순이었다.
  ★★**「20」은 밴드(7~33) 안에서 «고른» 수다** — 밴드는 실측이고 그 안 누적이 `16→17→17→17→19` 로 **평평**하다.
  실측 도달 **중앙 44일**(★상수 인용 금지 — 재측 **41일** · 창 정의 ±3일 · 결론 영향 0) ⇒ **연 6~8회** ·
  회차당 충돌(정본 누적) **중앙 2.5** · 게이트② 사이클 **8회차 중 7회가 1**.
  ★**방아쇠 = 기계가 «재고» 사람(총괄)이 «발권»** — 재는 자리 권고는 **이 repo 의 예약 워크플로**(선례 `rust-audit.yaml`),
  대안 `bin/healthcheck` 는 **orchestrator 소관이라 별건**. ★**구현은 이 회차가 하지 않았다.**
  ★**재측정 조건**(3회차 뒤 · 충돌 중앙 ≥5 면 임계 내리고 · 도달 간격 ≤14일이면 올린다)을 §5-B 에 오늘의 값과 함께 박았다.
  ★★**「오래됐다」는 재개 사유가 아니다 — 이 축은 «시간»이 아니라 «격차»로 열린다.**
  ★게이트③ 완료: PR #29 — ★★**`--merge` 착지**(★`--squash` 아님 · 등재 repo `contracts/upstream-sync-repos.conf:22`).
  머지커밋 sha 는 회신 `merged:` 참조. ★**게이트② 3회차 만에 approve**(초판 → fix → fix2) —
  ★**반려 둘이 모두 «수를 어떻게 적었는가»였다**: ⑴base·(델타/누적) 병기 없이 기준을 섞었다 ⑵그 정정이 «표»에만 닿고
  «산문·`summary`»로 전파되지 않았다. ⇒ ★**「표를 고쳤다고 문서가 고쳐진 것이 아니다」**가 이 리니지가 남긴 규율이다.
  ★**부모 #27·#28 착지로 두 번 `CONFLICTING` 이 됐고 그때마다 머지로 해소**했다(마지막은 `REPORT.md` 1건) —
  ★**충돌 중에는 `pull_request` 검사가 «구조적으로» 못 돈다**(검사 1건 → 10건으로 확증).
- [rustjava-dod-ci-parity-sibling-repo-survey] ★★**형제 repo(`wie`·`qts`) 파리티 락 필요성 «조사+판정» — 둘 다 «⒞ 조건부 필요».**
  채택 제안 `2026-09-04-dod-ci-parity-lock#p1`. ★**읽기 전용**(형제 repo `git`·PR·파일 수정 **0** · `gh api /contents` 로만 읽었다) · 구현 **0**.
  ★★**검사기는 «포팅 불가»다** — `wie` 는 DoD 정본이 **`AGENTS.md`** 이고 4번째 게이트(`cargo test`)가
  ★**`if:` 로 갈린 2 step + 블록 스칼라**라 우리 파서의 「조건부 = 제외」가 ★**거짓 red** 를 만든다 ·
  `qts` 는 DoD 가 **`make` 타깃 이름**(Makefile 간접층)이고 ★**toolchain 매트릭스가 없어 축 B 가 성립하지 않으며**
  `gitleaks` 는 ★**action 이라 어떤 `run:` 파서도 못 본다**.
  ★★**그런데 어긋남은 «둘 다 실재»한다 — 이력으로 쟀다**: `wie` `rust.yml` 실패 **34건 전수** 중
  ★**beta 셀에서만 실패한 clippy 9건(26%)** 인데 four gates 에 `cargo +beta` 가 **없다**(문자열 `beta` 규범 문서 **0건**) ·
  `qts` `ci.yml` 최근 실패 **60건 전수** 중 ★**`uv run ruff format --check .` «만» 실패 10건(17%)** 인데 `make lint` 는 안 친다
  (문자열 `format` 규범 문서 **0건**).
  ⇒ ★**먼저 할 일은 락이 아니라 «각 한 줄»** — 그 다음이 포팅이다. 다음 티켓 3건(T1·T2·T3)의 **축과 합격선**만 적었다(발권은 총괄 몫).
  ★★**일반 사실**: 세 repo 가 전부 이 어긋남을 갖고 있었고 **어긋난 자리가 전부 규범 문서에 없었다** —
  「사람이 문서를 최신으로 유지한다」가 **세 repo에서 각각 실패**했다.
  ★이 repo 검사기·`rust.yml`·DoD 블록 **무접촉** · `.rs` **0줄**.
  ★게이트③ 완료: PR #28 — ★★**`--merge` 착지**(★`--squash` 아님 · 등재 repo). 머지커밋 sha 는 회신 `merged:` 참조.
  ★**부모 #27 착지로 `CONFLICTING` 이 됐고 «해소 회차»가 별도로 돌았다** — 충돌 `STATE.md` 1건 ·
  ★**판단 필요 hunk 0**(두 판본 16줄 중 15줄 바이트 동일 · 차이는 «착지 상태»뿐) · 승인 내용 무접촉 ·
  ★★**검사 «1건 → 10건» 전건 성공**(충돌 중에는 `pull_request` 트리거가 «구조적으로» 못 돈다 — 그 기전이 해소로 확증됐다).
  ★**재검은 총괄이 «건너뛴다»로 판정**(원 게이트② 핀 `5c2ed7bc` · 해소 후 head `8ab6bb16`).
- [rustjava-dod-ci-parity-cross-product-decision] ★★**파리티 락 «교차곱» 확장 «판정» — 결론: 넓히지 «않는다».**
  채택 제안 `2026-09-04-dod-ci-parity-lock#p0`. ★**검사기 로직 변경 0 · `.rs` 0줄 · `rust.yml` 무접촉 · DoD 블록 무접촉.**
  ★**비용을 «먼저» 쟀다(추정 0)**: 무변경 재실행 **19s → 38s = 2.00×** ↔ ★**소스 1줄 편집 후 89s → 326s = 3.66×**.
  ⇒ ★**제안 문면의 「두 배」는 «무변경»에서만 참**이고 회차가 겪는 케이스는 3.66배다. 지배항 =
  `cargo +beta test --all` **215s**(같은 편집에서 stable test 66s 의 3.3배) · 콜드 1회 +171s.
  ★**돈이 아닌 비용 둘**: ⑴`rustfmt` 가 **beta 에 미설치**라 `cargo +beta fmt` 는 오늘 그대로 **rc=1**
  ⑵★**툴체인 교대 축출은 «없다»**(beta 3줄 직후 stable clippy 2s · test 25s) — 시간이 아니라 **디스크**를 쓴다(`target` 46G).
  ★**실익은 «따로» 쟀다**: CI 에만 있는 조합 **3개**(`fmt@beta` · `wasm32 clippy@beta` · `test@beta`)로 비어 있지 않지만,
  ★★**`rust.yml` 이력 76 run(성공 73 · 실패 3) 전수 분해에서 그 3개가 «새로» 잡았을 사건은 «0건»** 이다 —
  이 저장소의 beta 전용 실패는 **전건 `cargo clippy --all`** 이고 그 줄은 DoD 가 이미 `+beta` 로 덮는다
  (나머지 1건은 fmt 인데 **6셀 전건** 실패라 stable 이 잡는다). ⇒ ★**+237s/회차를 내고 얻는 것이 0.**
  ★**부분 확장(`fmt@beta` 만 · +3s)도 기각** — 이력 0건 · beta rustfmt 미설치가 기본 · 부분 교차곱은 모델 변경 요구.
  ★**재개 조건 + 세는 명령 + 오늘의 값 `0`** 을 `docs/upstream-sync-approach.md` §4 에 박았다
  (「`cargo clippy --all` «이외» step 이 beta 셀에서만 실패한 run 이 1건이라도 생기면 다시 연다」).
  ★검사기 **docstring 에만** 결정 포인터 1블록(그 파일이 「Widening … is a decision」이라 적고 결정 결과를 몰랐다).
  ★게이트③ 완료: PR #27 — ★★**`--merge` 착지**(★`--squash` 아님). ★**이번엔 «두 겹»으로 강제됐다**:
  ⑴등재 repo(`contracts/upstream-sync-repos.conf:22`) ⑵★**자식 PR #28 이 이 브랜치 «위에» 얹혀 있다**
  (`git merge-base --is-ancestor 204f98be <#28 브랜치>` = **YES**) ⇒ 스쿼시하면 그 커밋들이 재작성돼 #28 이 깨진다.
  ★**단 #28 의 base 는 `main`** 이라 브랜치 소멸로 닫히지는 않는다(`ripple` 형 사고와 다른 형상).
  머지커밋 sha 는 회신 `merged:` 참조. ★**착지 후 반사실**: `origin/main..upstream/main` = **0 유지**.
- [rustjava-local-dod-vs-ci-matrix-mechanical-check] ★★**로컬 DoD ↔ `rust.yml` «기계 대조» 신설 —
  `scripts/check-dod-ci-parity.py` + CI job `dod_parity`.** ★**5회차 리니지의 상수를 «사람 손»에서 뺐다.**
  ★★**설계 제약을 먼저 지켰다 — «낡은 문자열 스캐너»를 만들지 «않았다».** 게이트② 검수자가 실측으로
  「F1 은 «수»가 아니라 «말»이라 문자열 검사기로도 안 잡힌다」를 세웠기 때문이다. ⇒ 문서의 «문장»이 아니라
  ★**문서가 틀리는 «원인»(두 집합이 갈린 것)**을 잡는다 — `CLAUDE.md` DoD 코드블록과 `rust.yml` 을
  **각각 파싱해 대칭차**를 낸다(★정본 위치는 문서가 아니라 **스크립트 안**에 박혀 있다 · 계약 3).
  ★**축은 둘**: A=명령 집합 · B=toolchain 집합(`strategy.matrix.rust` ↔ `cargo +<tc>` 접두).
  ★**오늘 대칭차 0** ⇒ 계약 2 대로 **경고가 아니라 rc=1(막는다)** 로 배선했다.
  ★**검사기가 자기 줄을 «양쪽에» 넣는다** — CI job 과 DoD 블록 둘 다 `python3 scripts/check-dod-ci-parity.py`
  라 ★**자기 존재를 자기가 강제**한다. ⇒ DoD 는 6줄 → **7줄**.
  ★★**개악 5건 전건 red · 무개악 green**(DoD 줄 제거 · CI `- run:` 추가 · 매트릭스 nightly 추가 ·
  DoD `+beta` 제거 · `rust-toolchain.toml` 신설) — ★**공허하지 않다.**
  ★**침묵하지 않는다**: 성공에도 「명령 N개 · toolchain N개로 «둘 다 일치»」를 찍고,
  ★**못 보는 것**(조건부 step = OS 축 · 두 축을 «교차곱»으로 보지 않는 것)도 **그 자리에서 함께** 찍는다.
  ★**교차곱 미적용은 결함이 아니라 2026-09-04 결정이 «고른 값»이다**(`cargo +beta test` = 두 번째
  toolchain 전면 재빌드 · lint 를 지는 축은 clippy 뿐) — 넓히는 것은 «새 결정»이다.
  ★**§4 의 셸 두 토막을 «지웠다»** — 같은 술어가 두 벌이면 다음 사람이 한쪽만 고친다(정본 = 스크립트).
  ★**C7⒜(「그 축을 돌리는 것이 사람이다」)가 닫혔다** · ⒝(OS 축)는 그대로 열려 있다.
  ★`.rs` 변경 0 · `allow` 9곳 무접촉 · upstream 무접촉.
  ★게이트③ 완료: PR #26 — ★★**`--merge` 착지**(★`--squash` 아님 · 등재 repo). 머지커밋 sha 는 회신 `merged:` 참조.
  ★**착지 후 반사실 확인**: `origin/main..upstream/main` = **0 유지**(스쿼시였으면 계보가 접혀 다시 벌어진다) ·
  ★**`origin/main` 에서 신설 job `dod_parity` success** 확인.
- [rustjava-sync-contract-standing-clause-for-our-assets-going-stale] ★**「우리 자산이 낡는다」 상시 조항
  «판정» — 결론: ★★넣지 «않는다». 대신 «구멍 하나»(로컬 DoD 가 CI 매트릭스를 재현하지 않던 것)를 막았다.**
  ★코드(`.rs`) 변경 0 · 규범 문서 2 + 기록 문서 4.
  ★**먼저 셌다(목록 문서화로 시작하지 않았다)**: 자산 8건을 **돌연변이로 깨뜨려** 무엇이 잡는지 쟀다 —
  경로 문자열·`setProperty` 서술자·charset 라우팅·`ClassFormatError` 단정은 **`cargo test` RED** ·
  수동 span 은 **clippy RED** · `double_must_use` allow 는 **깨져도 무해** · 워크로그 스크립트는 **로컬+CI 둘 다**.
  ⇒ ★★**[게이트② 정정] 「아무것도 없음」은 «1개»가 아니라 «2개»**(② CI `--exclude` · ⑦ `double_must_use` allow) —
  초판이 ⑦을 «비하중» 자리(중복 crate-level attribute 1곳)에서 재 「무해」로 적었고, ★**9곳 전건 삭제로 재측정하면
  stable 0 · ★beta RED**(rc=101 · 진단 **6**건)이라 ⑦의 그물도 **CI 만**이다. ⇒ 「1개면 그 하나를 고쳐라」 규칙은 **적용되지 않는다**.
  ★**결론은 그대로 「넣지 않는다」이나 논거를 다시 세웠다**: ②와 ⑦은 «한 근인의 두 얼굴»이다 —
  ★**로컬 DoD 가 CI «매트릭스»를 재현하지 않는다**(② = 빠진 `- run:` 줄 = target 축 · ⑦ = 빠진 toolchain 축).
  ⇒ 근인 하나를 고치니 둘 다 그물을 얻었다(DoD 에 `cargo +beta clippy` 추가 ⇒ ⑦이 **로컬에서 RED**(rc=101 · 진단 **6**건)로 잡힌다).
  ★★**근인은 「자산 목록이 없다」가 아니었다** — ★**로컬 DoD 가 CI 검사 5종 중 «wasm32 clippy» 한 줄을 빠뜨렸고**
  그 줄이 그 자산이 사는 **유일한 자리**다. ⇒ `CLAUDE.md` DoD 가 이제 ★**CI 명령 5줄 + toolchain 축 1줄 = «6줄»**을
  축약 없이 싣는다(★**[게이트② 정정] 「5줄」은 계수 «1» 시절 수다 — 빠진 것은 «줄 하나»가 아니라 «두 축»이었다**).
  ★★**[후속 정정] 그 «6줄»도 낡았다 — 파리티 검사기가 더해져 «7줄»이다.** ⇒ ★**이제 줄 수를 문장에 적지 않는다**:
  `scripts/check-dod-ci-parity.py`(CI job `dod_parity`)가 DoD 코드블록과 `rust.yml` 을 각각 파싱해 대칭차를 낸다.
  ★그리고 ②는 «조용히» 실패하지도 않는다(`excluded package(s) not found` + 빌드 실패) ⇒ 문제는 침묵이 아니라 **늦음**이다.
  ★**재개 조건은 «축 둘»이다**(정정): **축① `- run:` 줄** · ★**축② toolchain 매트릭스** — 세는 명령 둘 다 §4 에 박았다.
  ★**오늘의 값 = 축① 0 · 축② 0**(착수 시 축① **5** · 축② **1**). ★세 돌연변이(beta 제거·nightly 추가·`- run:` 추가)로
  ★**각 축이 «자기 자리»에서만 반응함을 실측**했다. ★**C7 고지**: DoD 블록 «개악»과 **OS 축 3종**은 이 대조가 못 잡는다.
  ⇒ ★★**[게이트② 2차 정정 · fix2] 「고쳤다고 «신고»한 것이 산출물에 남아 있었다」 — 두 건.**
  ⑴「다섯 → 여섯」 통일이 ★**4자리 중 1자리**만 됐다 ⑵워크로그 `.json` `verification` 이 ★**diff 에 한 줄도 없이**
  철회된 라벨(「beta 도 ok — 깨져도 무해」)을 «검증 기록» 필드에 그대로 싣고 있었다.
  ★**전문 검색으로 반려 밖에서 «더» 찾았다**: 「CI 명령 5줄」 파생 수 **6자리** · 워크로그 재개 조건 블록이
  «축① 만» 실은 것 · 워크로그 검증 표에 `+beta` 행이 없던 것. ⇒ ★**전건 동기 + 마감 절차 3줄을 §4 에 규율로 박았다.**
  ⇒ ★★**[게이트② 3차 정정 · fix3] ⑴수 라벨 「beta error 7」이 «거짓»이었다 — 착지본 «18자리» 전건 정정.**
  실측(9곳 삭제 → `cargo fmt --all` → beta clippy): ★**rc=101 · `double_must_use` 진단 «6»건**
  (전건 `jvm/src/jvm.rs:140·404·438·779·791·1016`) · cargo 요약 줄이 「due to **6** previous errors」라 말한다.
  ★**「7」은 `grep -c '^error'`(진단 6 + 요약 줄 1)의 값**이고 그 필드가 내건 정의가 아니었다 ⇒ ★**화해 문장 철회.**
  ★**「6」은 «하한»이다** — `jvm-bytecode` 가 `jvm` 에 의존해 jvm 이 깨지면 그 크레이트는 **린트되지 않는다**.
  ⑵잔존 4건(워크로그 「문서 2파일」 2자리 · json `title` · 「채택 제안 둘」 3자리) + ★**PR 제목**까지 동기.
  ⑶★**«5건째»를 스스로 찾았다**(`changes[0]`) — ★**필터 걸린 sweep 이 «거짓 0»을 냈기 때문**이고(zsh 단어분할),
  그 교훈을 §4 마감 절차 **4번**으로 박았다.
  ★게이트③ 완료: PR #25 — ★★**`--merge` 착지**(★`--squash` 아님 · 등재 repo `contracts/upstream-sync-repos.conf:22`).
  머지커밋 sha 는 회신 `merged:` 참조. ★**게이트② 5회차 만에 approve**(초판 → fix → fix2 → fix3) —
  ★★**반려 넷이 전부 «내용»이 아니라 «신고한 것 ↔ 산출물»의 어긋남이었다**(계수 · 수 라벨 · 잔존 자리).
  ⇒ 그 대가로 `docs/upstream-sync-approach.md` §4 에 ★**마감 절차 4줄**이 남았다(착지본 전문 검색 ·
  분모 = 파일 × 필드 · 파생 수 · ★**필터 걸린 검색의 «거짓 0»**).
  ★**남은 minor 하나(F1 · 워크로그 `.md:54` 의 「CI 검사 «한 줄»」)는 이 회차가 «고치지 않았다»** —
  다음에 그 파일을 만지는 회차가 §4(:90)와 «같은 표지»를 달면 닫힌다.
- [rustjava-upstream-sync-s8-rename-sweep-decision] ★★**upstream 컷 `bd42427`(개명 스윕 · 12커밋) 머지 —
  ★★★`merge-base` `ba5797b` → `bd42427` · behind ★**12 → 0** · 부모 2개 ⇒ ★★**upstream 을 «완전히» 따라잡았다.**
  착수 재측정(신 서식): **누적 8 · 델타 +8**(base `3fb08a8` · merge-base `ba5797b`) — 「재서 8 이었다」.
  ★**개명 대응표**(upstream 이력 실측 · A/D **0** · 전건 R): `java_runtime`→**`rustjava-runtime`**(2홉 · 379건 R079) ·
  `test_data`→**`test-data`**(243 R083) · `jvm_rust`→`jvm-bytecode` · `java_class_proto`→`jvm-class-proto` ·
  `java_constants`→`jvm-types` · `test_utils`→`test-utils`.
  ★★**modify/delete = «0건»** — 두려워한 형태가 안 나왔다. git 이 `CONFLICT (file location)`+`AU` 로
  우리 고유 6건을 새 경로로 **이미 옮겨** 두고 이동 확인만 요구했다 ⇒ 처분 「간다」 전건 ·
  ★**`origin/main` 블롭 대조로 6건 «바이트 동일» 확인** · ★**픽스처 5 → 5**.
  ★★**개명이 «우리 자산 2건»을 낡게 만들었다(5회째 · 대상은 처음 — CI·경로)**:
  `rust.yml` 의 `--exclude test_utils`(옛 크레이트명 ⇒ wasm32 셀 red · 실측 rc=101↔0) → `test-utils` ·
  `tests/test_class_format.rs` 의 `"test_data/"` 4곳 → `test-data/`. ★둘 다 «우리 파일»이라 충돌이 날 수 없다.
  ★**`Cargo.lock` 하강 차단**: `--theirs`+build 가 `tracing` **0.1.44 → 0.1.41**(=PR #4 언프리즈 되돌림) ·
  `tracing-subscriber`·`syn` 도 내렸다 ⇒ S5 처방(main lock 에서 재생성)으로 **내려간 것 0**.
  green: stable 4종 + beta 2종 rc=0 · **554/0/1**(baseline 554 · 증감 0 이고 맞다 — upstream 도 547→547).
  ★게이트③ 완료: PR #24 — ★★**`--merge` 착지**(★`--squash` 아님 · 등재 repo). 머지커밋 sha 는 회신 `merged:` 참조.
  ★★★**이 착지가 «캠페인 종료»다 — 2026-08-16 착수 시 behind 33 → 오늘 «0».** S1~S8 컷 조상 8/8.
  ★**다음 동기는 «계획된 컷»이 아니라 «upstream 이 움직일 때»다** — 정기 축 여부는 별건 판정(워크로그 `proposals[1]`).
- [rustjava-upstream-sync-s7-and-fix-the-conflict-count-format] ★**upstream 컷 `ba5797b`(가상 디스패치 해석 ·
  1커밋 · 319파일) 머지 — 충돌 1 해소** + ★**§5 「충돌 수」 정본 서식에 «델타/누적» 축을 넣었다.**
  ★**`merge-base` `95ebc5c` → `ba5797b` · behind 13 → 12 · 부모 2개** ⇒ ★★**계획 7회차(S1~S7) 완주.**
  착수 재측정(신 서식): 컷 `ba5797b` = **누적 1 · 델타 +1**(base `3e02f8c` · merge-base `95ebc5c`).
  ★`string.rs` 는 집합에서 **빠졌다**(S6 해소 뒤 upstream 무접촉) ⇒ ★**누적은 줄어들 수도 있다.**
  ★★**`thread.rs` 는 «직교»였다** — upstream(**+23/−10**)은 `invoke_virtual` 에 «선언 클래스» 인자를 더하고
  우리(**+50/−42**, 의미 3줄 + 들여쓰기)는 그 호출들을 감싸는 **수동 span**(PR #4)이다 ⇒ 우열 판정 대상이 아니다.
  우리 구조를 뼈대로 upstream 새 인자 3곳을 얹었다(S1·S3·S4 와 같은 전략 · **4회째**).
  ★★**「충돌 0으로 들어온」 파손 4회째 · ★축은 «처음»**: upstream 이 «공용 API 시그니처»를 바꿔
  ★**우리 고유 테스트 5곳**이 구식 4인자로 남아 **컴파일 실패**(E0061×5). 우리 줄이라 충돌이 «날 수가 없다»
  ⇒ ★**앞 세 번(신규 파일)과 «반대 방향»이고, 잡은 것은 «컴파일»이다.**
  upstream 관용구를 그대로 채택(`&x.class_definition().name()` · `"java/lang/String"`) · 단언 무접촉.
  green: stable 4종 + beta 2종 rc=0 · **554 / 0 / 1**(baseline 554 → ★**증감 0 이고 그것이 맞다** —
  upstream 테스트 함수도 547 → 547 = 스윕이지 추가가 아니다) · `#[ignore]` 1 → 1 · 단언 삭제 0.
  ★게이트③ 완료: PR #23 — ★★**`--merge` 착지**(★`--squash` 아님 · 등재 repo). 머지커밋 sha 는 회신 `merged:` 참조.
  ★★**S1~S7 컷 조상 7/7** ⇒ S4 의 「전건 NO(스쿼시 3회가 족보를 원점으로)」 반대편을 **코드 회차 3연속**으로 지켰다.
  ★**남은 `behind 12` 는 «개명 스윕» 구간이고 S8 몫**(총괄 보류분 — 이 회차는 집행하지 않았다).
- [rustjava-upstream-sync-s6-cut-95ebc5c] ★**upstream 컷 `95ebc5c`(regex·Formatter·Locale · 11커밋) 머지 —
  충돌 1 해소.** ★**`merge-base` `c4665b0` → `95ebc5c` · behind 24 → 13 · 머지커밋 부모 2개**(계보 보존).
  `cargo test --all` **554 passed / 0 failed / 1 ignored**(S5 427 → **+127**) · stable 4종 + ★**beta 2종** rc=0.
  ★★**「S6 새 충돌 0」 예측은 «델타»였다 — 「풀 것이 없다」가 아니다.** 그 0 은 옛 base `8c1238b` 에서 잰
  «새로 나타난 파일 수»(누적 3 → 3)이고, 착수 재측정은 새 base `a0b5d3c`(merge-base `c4665b0`)에서 ★**누적 1**이다.
  `string.rs` 는 S5 의 설계 판단으로 우리 분기(**+8/−28**)가 남아 upstream 이 만질 때마다(**+402/−121**) 계속 열린다.
  ⇒ ★**§5 에 한 줄 보탰다: base 와 «함께» «델타인가 누적인가»도 밝혀라.**
  해소 = import 블록 **합집합**(우리 `charset::Charset` + upstream `Formatter`·`Locale`·`regex`) ·
  `Charset` 라우팅 4곳 생존 · `decode_str`/`encode_str` 재유입 **0**.
  ★★**계약4⒝ 정독이 «충돌 0으로 들어온» 파손 1건을 «테스트 전에» 잡았다 — 이 형태 «세 번째»**:
  upstream 신규 파일 `java/util/regex/test_pattern_syntax_exception.rs` 가 `System.setProperty` 를
  `)Ljava/lang/Object;` 로 **3곳** 부른다 ⇒ 서술자만 `String` 으로(S5 의 확립된 처분과 동일).
  ★**전례 S3 3건 → S5 5곳 → S6 3곳** — 매 회차 «새 파일»로 재유입된다.
  ★게이트③ 완료: PR #22 — ★★**`--merge` 착지**(★`--squash` 아님 · 등재 repo). 머지커밋 sha 는 회신 `merged:` 참조.
  ★**착지로 base 가 또 바뀌었다 ⇒ §5 의 「S7 새 충돌 1(`thread.rs`)」은 «다시 재야 한다»**(base·델타/누적 병기).
  ★**남은 구멍(검수자 지적)**: §5 «상시 규칙» 절의 정본 서식(`:268`)은 아직 `<수>(base · merge-base)` 뿐이고,
  «델타/누적» 축은 S6 착지 기록(`:444`) 안에만 있다 — ★**다음 §5 갱신 회차가 정본 서식에 한 줄 올려야 한다.**
- [rustjava-upstream-sync-s5-with-remeasured-conflicts] ★**upstream 컷 `c4665b0`(#190 Java 1.2 API 확장) 머지 —
  충돌 3 해소.** ★**`merge-base` `3296139c` → `c4665b0` · behind 30 → 24 · 머지커밋 부모 2개**(계보 보존).
  `cargo test --all` **427 passed / 0 failed / 1 ignored**(S4 261 → **+166**) · green 4종 rc=0.
  ★**착수 시 재측정**(§5 상시 규칙 · base 당시 main `1983d9f` · merge-base `3296139c`)이 재측정 표와 **일치**.
  ★★**예측 3건 중 2건 적중 · 1건 반증**:
  ⑴`Cargo.lock` **재생성** ⑵`string.rs` **설계 판단이 맞았으나 «형태»가 달랐다** — upstream 이 `copyValueOf` 2종을
  신설하며 `decode_str`/`encode_str` 표를 «되살렸는데» **4개 호출부는 자동병합으로 우리 `Charset` 라우팅을 유지**했다
  ⇒ 신규 API 는 취하고 표는 버렸다(안 그러면 dead code — S3 완료조건 위반).
  ⑶`test_timer.rs` ★**「되얹기」 예측이 반증됐다** — upstream 이 벽시계 테스트 2건을 **manual clock + monitor
  notification 기반 결정성 스위트 12건**으로 대체했다 ⇒ `upstream 채택`.
  ★★**S4 가 남긴 「우리 테스트의 시간 의존」 별 축은 소멸했다 — 발권하지 마라**(2000ms 근거는 §5 착지 기록에 보존).
  ★★**충돌 «목록에 없던» 파손 1건 — §4 가 경고한 형태가 실제로 났다**: upstream 신규 io 테스트 **3파일 5곳**이
  `System.setProperty` 를 `)Ljava/lang/Object;` 로 부르는데 우리는 PR #5 에서 JDK 규격대로 `)Ljava/lang/String;` 이라
  ★**충돌 마커 0줄인데 `NoSuchMethodError` 3건**. 서술자만 맞췄다(★`test_boolean`·`test_integer`·`test_long` 이
  **앞 회차에 이미 같은 처분**을 받았다 · `Properties.setProperty` 의 Object 반환은 JDK 규격상 옳아 무접촉).
  ★게이트③ 완료: PR #21 — ★★**`--merge` 착지**(★`--squash` 아님 · 등재 repo). 머지커밋 sha 는 회신 `merged:` 참조.
  ★**착지로 base 가 바뀌었다 ⇒ §5 의 「S6 새 충돌 0 · S7 새 충돌 1」은 «다시 재야 한다»**
  (§5 상시 규칙: 충돌 수를 적을 때는 base 를 반드시 병기하라).
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
  ★게이트③ 완료: PR #19 — ★★**`--merge` 착지**(★`--squash` 아님 · 등재 repo).
  ★착지 «전» `REPORT.md`·`STATE.md` 위치 충돌 2건을 **합집합**으로 해소했다(PR #20 이 먼저 착지 · 코드 충돌 0).
  머지커밋 sha 는 회신 `merged:` 참조.
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

### ①(최우선이 «아니게 됐다») upstream 동기화 — ★★**[2026-09-11 갱신] 캠페인 «종료». 동기 회차를 열지 마라.**

★실측(2026-09-11): **S5(#21)·S6(#22)·S7(#23)·S8(#24) 전건 `--merge` 착지** · `merge-base origin/main
upstream/main` = **`bd42427`** · behind **1**(`2ce4717` · dependabot encoding_rs bump) · ahead 87 · 열린 PR **0**.
★**behind 1 < 임계 20**(PR #29 판정) ⇒ 다음 동기는 «계획된 컷»이 아니라 임계 도달 시다 — 재는 주체는
주 1회 `.github/workflows/upstream-behind.yml`(PR #31)이고, **기계가 재고 사람(총괄)이 발권한다**.
★★**구판 「다음은 S5」는 8일 낡은 채 이 절에 남아 2026-09-11 중복 발권**(`rustjava-upstream-sync-s5-java12-api` ·
blocked)**을 만들었다** — ③ 절 「이미 끝난 일을 가리키면 레인이 조용해진다 · 닫히는 즉시 닫아라」의 두 번째 재현.
⇒ ★**다음 실작업 = ③의 null-guard 티켓**(①의 뒤라는 선행 조건이 이제 충족됐다).

---- 이하 사료(S4 회차 실측 · 타이머 여백 근거 — 단정 불변이라 보존) ----

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
| `System.arraycopy` null 가드 | ~~**부재**~~ → ★**해소**(`src`·`dest` **둘 다** · 2026-09-11) | ~~유효 잔존~~ → **닫힘** |
| `String.<init>([B)` / `([C)` null 가드 | ~~**부재**~~ → ★**해소**(아래 7건 전부 · 2026-09-11) | ~~유효 잔존~~ → **닫힘** |

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

1. ~~`rustjava-upstream-sync-s5` … `-s7`~~ → ★★**[2026-09-11] 해소 — S5~S8 전건 착지**(① 참조).
   ★**발권하지 마라** — 2026-09-11 에 실제로 중복 발권됐다(`rustjava-upstream-sync-s5-java12-api` · blocked).
2. ~~`rustjava-null-guard-string-init-and-arraycopy`~~ → ★★**[2026-09-11] 해소 — `…-p0` 로 착수해 가드 9곳 + 픽스처 9케이스로 닫았다**(완료 절 참조).
   ★**발권하지 마라.** ★아래 범위 서술은 **사료**다 — 단 ★**「가드 없음 7건」은 «8곳»이었다**: `System.arraycopy` 의
   `dest` 가 `src` 와 **별개로** 뚫려 있었고 이 절이 그것을 «1건»으로 셌다. 다음에 이런 표를 쓸 땐 **인자 단위로** 세라.
   ---- 이하 사료 ----
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

⇒ ~~다음은 S4 / 다음은 S5~~ → ★★**[2026-09-11 갱신] S8 까지 전건 착지 — 동기 캠페인 종료**(① 참조).
behind **1** · 열린 PR **0** · 원격 브랜치 = `main` · `wie-ktf-hardening`. 다음 실작업 = ③의 null-guard.

- ★PR 발권 시 `--repo Jun025/RustJava` 명시(2026-07-22 upstream 오발행 사고 재발 방지).
- ★upstream 발신(PR·이슈·코멘트·push)은 **티켓이 명시 허가할 때만**. 기본은 조회뿐.
