# STATE

## 진행중
(없음 — 2026-09-17 실측: 착수 시 열린 PR **4건**(#56·#57·#58·#59). ★**내 경로와 겹치는 것은 둘**이다 —
 #59 가 `test-data/src/indy/make_indy_fixtures.py`·`tests/test_class_format.rs` · #56 이 `tests/test_class_format.rs`.
 ★`jvm-bytecode/src/interpreter.rs`·`rustjava-runtime/` 은 **겹침 0**. ⇒ 착지 순서는 **#56·#59 먼저, 이 PR 나중**이 맞다
 (둘 다 이것보다 오래됐고 MERGEABLE/CONFLICTING 처분이 이미 걸려 있다). 겹침은 전부 **append 형 합집합**이라 해소는 기계적이다)

## 완료
- [wie-2026-09-22-lgt-object-reference-gate-adopt-p0] GC 도달성 순회 `Result` 전파 — `get_field`·`get_static_field`·`load` unwrap 3곳 → `?`(호스트 오류 그대로) · 불변식 unwrap 2곳 → `Unraisable` · 오류 시 중단(해제 0). 새 변종 0 · 공개 서명 변경 0. 양방향: 새 시험 ok ↔ unwrap 복원 시 panic FAILED. 채택 `2026-09-22-lgt-object-reference-gate#p0` · `2026-09-24-unraisable-error-variant#p0`. wie 효력은 crates.io 릴리스 뒤. 상세 `docs/worklog/2026-09-24-gc-walk-propagates-host-errors.md`.
- [rustjava-2026-09-20-name-the-missing-bootstrap-class-adopt-p0] ★**공개 API 변경(breaking)**: `JavaError::Unraisable(String)` 신설 — Java 예외로 만들 수 없는 실패. `Jvm::new` 패닉 2곳 → `Err(Unraisable)`(빠진 클래스 이름 포함) · `Jvm::exception` 재귀 바닥(같은 스레드에서 «같은 예외»를 다시 만들거나 깊이 8 → `Unraisable`, 첫 실패 명명) · `[Ljava/lang/String;` 숨김 재현이 stack overflow → loader 질문 2회. ★외부 소비자: `let JavaError::JavaException(..) = ..` irrefutable 구조분해가 컴파일 에러가 된다(이 repo 3곳 수정 · wie 는 crates.io 0.1.1 소비라 판올림 때 발생). 채택 `2026-09-20-name-the-missing-bootstrap-class#p0` · `2026-09-20-string-array-hiding-overflows-stack#p0`. 상세 `docs/worklog/2026-09-24-unraisable-error-variant.md`.
  보정(-fix): `stream_handler.rs` `publish` 의 `if let Err(JavaException)` 2곳 → `match` + `Unraisable` 전파(삼킴 제거) · 회귀 `stream_handler_propagates_unraisable_output_failures`.
- [rustjava-2026-09-18-bootstrap-argument-index-and-tag-adopt-p0] ★**검증 규칙이 멈춘 위치를 말한다 — 11규칙, 변종 1개.** 채택 제안 `2026-09-18-bootstrap-argument-index-and-tag#p0`.
  ★**전제 반증(작게)**: `validate_class` 규칙은 15/14 가 아니라 **14 · 고정문장 13**(@origin/main `c654ae2e`).
  ★**전수**: 13 중 **11**이 표를 걸으며 멈춘 위치를 쥐고 있었다(pool 3 · field 3 · method 3 · interface 1 · class attribute 1) · `this_class`/`super_class` 2개는 가리킬 곳 없음 → `InvalidFormat` 유지.
  ★**멈춤 기준**: enum 은 «규칙»이 아니라 «표 종류»로만 자란다 ⇒ `InvalidFormatAt { cause, location: Location }` **1변종** + `Location` 5종. `ClassFileError` 크기 불변(기존 크기 테스트 무수정 통과).
  ★**문면** = `<종전 문장> (<표> #<n>)` · pool 은 `javap` 의 1-기반 `#N` · 파일 바이트 0. 경계 무이동(술어 본문 불변 · `all/any` → 첫 위반 위치).
  ★**양방향**: 인덱스 3종 개악(M1 문면에서 위치 삭제 · M2 field/method 0 고정 · M3 pool 첫 키 보고) **전건 red** · pool 인덱스(#11·#18·#34)는 **독립 바이트 워커로 교차 확인**.
- [rustjava-2026-09-23-stale-next-pointer-and-euc-kr-boundary-adopt-p0] charset 보류 판단을 `Charset::bytes_to_hold_back`(wildcard 없는 match)로 옮김 · `read()` 이름 비교 0 · 동작 불변 · 변이 양방향 확인. 채택 `2026-09-23-stale-next-pointer-and-euc-kr-boundary#p0`.
- [rustjava-2026-09-23-stale-next-pointer-and-euc-kr-boundary-adopt-p1] `## 다음` 정본 결정 = ⒝ 얇은 층(ref + 선행 사슬 + 카드 밖 항목만 · 산문 지목 금지). ⒞ 기각 근거 = tower 술어로 열린 카드 0(32건 전건 injected). 채택 `2026-09-23-stale-next-pointer-and-euc-kr-boundary#p1`. 상세 `docs/worklog/2026-09-23-next-section-canon-decision.md`.
- [rustjava-prune-declined-followup-proposals-2026-09-21] ★**추천 후속작업 2건 기각** — 운영자 지시(2026-09-21 우선순위 정리). ★제품 코드 **0줄** · 새 제안 **0** · 검사기/CI 신설 **0**.
  ★**닫은 둘**(검사기 다듬기 축): `2026-09-19-nonliteral-blind-spot-is-reported-not-gated#p0` · `2026-09-20-lock-script-output-order#p0`.
  ★**남긴 둘**(런타임 축): `2026-09-20-string-array-hiding-overflows-stack#p0` · `2026-09-20-name-the-missing-bootstrap-class#p0`.
  ★★**«선언»이고 «삭제»가 아니다** — `proposals[]` 원소 삭제 **0**(지우면 0-기반 `#pN` 이 밀려 **다른 제안의 ref 가 바뀐다**). 서식은 선례 `2026-09-17-ldc-asm-regeneration-declined.json` 그대로.
  ★**실측**: 소비자 파생식 재현 → **open 4 → 2** · 남기기로 한 둘 **그대로 열림**(과잉 차단 0) · **ref 총수 99 불변** · `check-worklog-json.py` **rc 0**.
  ★**되살리기**: 그 json 의 `declinedProposals[]` 에서 ref 줄을 빼면 다시 열린다.
- [rustjava-error-path-string-array-hiding-overflows-stack-p1] ★★**`[Ljava/lang/String;` 오버플로의 원인 — 지난 회차 기재가 «틀렸다».** 채택 제안 `2026-09-20-error-path-class-closure#p1` · ★**조사 회차 · 제품 코드 0줄**.
  ★**재현**: cap 20 · 기본 스택 → `stack overflow, aborting` rc **134**.
  ★★**반증**: 「로더로 안 돌아와 cap 이 못 끝낸다」는 **거짓** — 살아남은 전 실행에서 `asked = cap+1`(= `[C` 와 같은 모양). 원인은 ★**턴당 비용**.
  ★**두 손잡이가 따로 움직인다**(가설 분리): cap **18↔19** · 스택 **2 MiB↔4 MiB**(cap 20 고정).
  ★**순환을 백트레이스로**: `fillInStackTrace`→`instantiate_array`→`resolve_class`→`load_class`(None)→`exception`→`new_class`→`invoke_special`×4→`Throwable.<init>`→`fillInStackTrace` · ★**한 턴 ≈ 57 프레임**.
  ★**대조**: `[C` 만 같이 돈다 · `[I`·`[Ljava/lang/Object;`·`java/lang/Integer` 등은 **asked=0** ⇒ **「배열이면 난다」가 아니다**.
  ★**고침 = 스윕 헤더의 거짓 문장 하나** · 배열 제외는 유지(이유가 다르고 여전히 유효) · **개악 양방향 해당 없음**(동작 무변경).
  ★**남긴 것**: 순환은 제품에 **바닥이 없다**(오늘 닿을 수 없을 뿐) ⇒ 후속 카드 「`Jvm::exception` 에 바닥을」(M).
- [rustjava-error-path-name-the-missing-bootstrap-class-p0] ★★**없는 부트스트랩 클래스의 «이름»을 말하게 했다.** 채택 제안 `2026-09-20-error-path-class-closure#p0`.
  ★`Jvm::new` 의 `bootstrap_classes` 루프 `.unwrap()` → `let … else` · 문면 = 이름 + 여섯 중 하나 + ★**어디에 물었는지**(호스트 로더이지 `java.class.path` 가 **아니다**).
  ★**가족을 세고 골랐다**: 이 축 **1곳 고침** ↔ 같은 «모양»의 레지스트리 조회 **4곳**은 다른 축(내부 불변식)이고 스윕이 **닿지 않음을 실측**(익명 거절 0/37) ⇒ **blind spot 으로 적고 두었다**.
  ★★**스윕이 자기 눈으로 보고도 좋은 쪽에 세고 있었다** — `Panicked` 가 문면을 안 읽었다. 이제 payload 를 숨긴 이름과 대조하고 `PanickedAnonymously` 는 **그 자체가 실패**.
  ★**양방향(두 사실을 «따로»)**: 개악 → 이름 적중 **0/5** · `7 by name · 5 anonymous` **FAILED** ↔ 고침 → **5/5** · `12 · 0` **ok**. ★**rc 만 보면 둘이 같다**(양쪽 패닉).
  ★**대가**: 런타임 **0**. 패닉→오류 반환은 **하지 않았다**(`JavaError` 변종 1개 · 없는 것이 바로 예외를 만들 클래스 · 사내 3곳 + 공개 enum 파괴) ⇒ 제안 카드.
- [rustjava-string-on-the-error-path-p0] ★★**오류 경로의 클래스 집합을 한 번에 쟀다 — 답은 «둘»이 아니었다.** 채택 제안 `2026-09-19-string-on-the-error-path#p0`.
  ★**후보를 «유도»했다**(손 목록 아님): 기록 로더로 정상 구성 1회 → 요청 **51** · 서로 다른 이름 **42** · 배열 **5** 제외 ⇒ 후보 **37**.
  ★★**5개가 더 재귀한다**: `Throwable`·`Error`·`LinkageError`·`CharSequence`·`Comparable` = 기존 두 이름의 **상위형·인터페이스 폐포**.
  ★**처방 = 폐포 walk**(assert 2 → 작업목록 1 · 로더에 **직접** 질의). ★**폐포 9로 계산이 닫힌다**(막힌 2 + 재귀 5 + bootstrap 2) ⇒ 미설명 **0**.
  ★**양방향**: `0 recursed` ok ↔ `extend` 2줄 제거 → **5 recursed FAILED** ↔ 복원 ok. 기존 잠금 2건 **무수정 통과**.
  ★**대가**: 로더 질문 **44→51** · 스윕 **~15초** · ★배열 5개 제외(로더가 **합성**하므로 클래스 집합이 못 빠뜨린다 — 단 `[Ljava/lang/String;` 는 상한 20에서도 오버플로 = in-process 불가).
  ★**후속 2건**: 부트스트랩 unwrap 이 **이름을 말하지 않는다**(S) · `[Ljava/lang/String;` 의 두 번째 순환(M).
- [rustjava-checker-output-determinism-has-no-guard] ★★**검사기 출력 순서를 잠갔다 — 습관을 규칙으로.** 채택 제안 `2026-09-19-merge-drops-deterministic-order#p0` · 신설 `scripts/check-script-output-order.py`(AST) + CI 잡 `script_output_order` + DoD 10번째 줄.
  ★**불변식 한 줄**: `scripts/*.py` 의 어떤 `for`·컴프리헨션도 **`sorted(...)` 밖에서 set 을 순회하지 않는다**.
  ★**「그물 0」 재현**: 비결정성을 되돌린 채 파이썬 검사기 **4종 rc 0** · `cargo fmt` **rc 0**. ★해소 여부 선행 확인 — `scripts/`·`rust.yml` 최종 커밋은 **`35f34797`**(그 수정 자신).
  ★**정적을 고른 이유**: 두 `PYTHONHASHSEED` 재실행은 ★**회차당 약 절반 눈을 감고**(해시 순서 = 정렬 순서면 무증상), `assert sorted` 는 **다른 곳의 새 출처를 못 잡는다**(제안 자신의 약점 기술).
  ★**양방향 2×2**(제품 호출부): M1 `check-merge-dropped-symbols.py:254` 제거 → rc 1 ↔ 복원 rc 0 · M2 ★**다른 파일** `check-dod-ci-parity.py:215` → rc 1 ↔ 복원 rc 0. 정상 `7 script(s): 0` · 0.06초.
  ★**측정된 사각**: 튜플 언패킹 반환 set 은 못 본다 — `ci_runs, ci_tcs = parse_ci(...)` 에 정렬 없는 순회를 넣으니 ★**rc 0 통과**. 후속 제안 `#p0`.
  ★★**게이트² 반려 승계(`-fix`)**: **R1** `", ".join(myset)`·`print(*myset)` 이 통과했다(사고와 **같은 계급** · 미기재) ⇒ `str.join` 첫 인자 + `Starred`(Load) 를 검사에 더하고 약속 문구를 ★**«for/컴프리헨션 · str.join · \*-언팩» 세 위치로 명시** ·
  **R2** 「set→dict 는 set 에서 잡힌다」가 **거짓**(`dict.fromkeys`)이라 **삭제**. ★`fromkeys` 만 반쪽으로 잡지 «않았다» — 진짜 계급은 «컨테이너 순서 오염»이고 하나만 잡으면 **없는 프로그램을 있는 것처럼 보이게 한다**(오늘 0건 ⇒ 문안 결함) ·
  **R3** ★**python 린터·포매터·테스트 0**(추적 파일 0건 · `rust.yml` 은 검사기 5회 실행뿐) ⇒ **이 파일을 보는 기계는 CI 잡 «하나»**임을 빚으로 적었다(하네스는 만들지 «않았다»).
  ★승계 양방향: `join`·`*` 반례 **rc 1** ↔ `sorted()` 씌우면 **무검출**(오탐 0) ↔ 반례 제거 **rc 0** · 원 M1·M2 **회귀 재확인**.
- [rustjava-error-path-needs-java-lang-string-measure-first] ★★**오류 경로의 또 하나 `java/lang/String` — ⒜ «재귀한다»로 확정하고 선재 확인을 넣었다.** 채택 제안 `2026-09-19-fallback-class-absence-fails-at-construction#p0`. ★제안의 조건이 「측정이 먼저」였고 그대로 했다.
  ★**실측**: 상한 **116 생존 ↔ 117 SIGABRT «stack overflow, aborting»**(양쪽 2회 재현) · ★**상한 100000 도 abort** ⇒ 바닥이 없다.
  ★**⒞ 아님을 구조로**: `bootstrap_classes` **6개에 String 없음** · `from_rust_class` 는 이름을 **`[B`(nameBytes)** 로 넣는다 ⇒ 처음 필요한 곳은 프로퍼티 루프.
  ★**자리**: 프로퍼티 루프 **앞**(기존 `NoClassDefFoundError` 확인은 그 **뒤**라 String 형상엔 **늦다**) · 관용은 동일(로더에 **직접** 질의 후 resolve — bare `resolve_class` 는 순환 자신에게 넘긴다).
  ★**양방향**(제품 호출부): green ↔ 제거 시 ★**바이너리째 SIGABRT** ↔ 복원 green.
  ★**시작 비용**: 로더 질의 **1 → 2회**(결정론 계수). ★**벽시계는 버렸다** — 형제 레인 부하로 같은 형상 p50 이 69ms~1690ms 고 교대 8회 중 3회는 확인 있는 쪽이 더 빨랐다 ⇒ 수를 주장하지 않는다.
  ★**미측정**: 두 클래스의 생성자·정적 초기화가 닿는 나머지(후속 카드).
- [2026-09-19-partial-clone-blob-vs-absence-p0] ★★**`check-merge-dropped-symbols.py` 의 출력 순서를 고정했다 — 두 회차를 diff 할 수 있다.** 채택 제안 `2026-09-19-partial-clone-blob-vs-absence#p0`. ★**코드 1줄**(+주석 8줄) · `.rs` 0줄.
  ★**재현**: `origin/main` 판본 · `PYTHONHASHSEED=random` **10회** → 순서 **2종**(같은 6건) ⇒ 없는 차이가 diff 에 보였다.
  ★★**제안 진단은 부정확**: `findings` 는 set 이 아니라 **list** 이고 이름은 이미 정렬돼 있었다 — set 4개 중 출력에 닿는 것은 ★**`changed`(경로) 하나**다. ⇒ 「print site」가 아니라 ★**출처에서** 정렬했다(`check()` 의 **반환값**도 결정적이어야 하므로).
  ★**양방향**: 2종 ↔ **1종** ↔ 되돌리면 2종. ★찾은 것 불변(15줄 · 집합 일치 · rc 1).
  ★**대가**: 아무도 잠그지 않는다 — `scripts/` 테스트 하네스 **0** · 비결정성을 넣어도 파이썬 검사기 **4종 rc 0** · fmt **rc 0** ⇒ ★**그물 «0»**. 후속 제안으로 남겼다.
- [2026-09-18-nonliteral-exception-call-sites-p0] ★★**비리터럴 사각은 «관문»이 아니라 «보고»다 — 제안의 전제 둘 다 소멸.** 채택 제안 `2026-09-18-nonliteral-exception-call-sites#p0`.
  ★**전제 재측**: 「baseline 0」 → ★**1**(그 1자리는 **정상**이고 «비리터럴이어야만» 한다 — 리터럴이면 이 검사기가 red) · 「죽는다」 → ★**더는 안 죽는다**(#76 착지) ⇒ 해악이 «죽음»에서 «틀린 catch»로 내려갔다.
  ★**관문을 0으로 걸었으면 제안된 날 main 이 red** 였다 — ★제안이 자기 `why` 에 그 비용을 예고했고 **4시간 뒤** 현실이 됐다.
  ★**지은 것**: 매 실행에 사각을 **세어 찍는다**(never fail) · **1파일 +90/−6** · 새 DoD 명령 **0** · 새 CI 잡 **0** · 종료코드 불변.
  ★**축**: 제품 코드 주입 → **1→2**(파일:줄 지목) · 원복 → 1 · rc 양쪽 0 · 술어 민감도 **42**.
  ★**대가**: 찍힌 수는 무시할 수 있다 · 수는 술어만큼만 정확 · 제품/테스트 미구별(후속 카드).
  ★**회귀 둘을 스스로 만들고 재서 걷어냈다**(두 번 걷기 · 개행 인덱스) ⇒ 최종 비용 **유의차 없음**.
- [rustjava-assert-loadable-rederivation-did-not-come-up-short] ★★**되유도를 믿지 않고 «단언»한다 — 두 축 모두 fail-closed(exit 2).** 채택 제안 `2026-09-19-loadable-set-source-of-truth#p0`. ★**1파일 +38/−2.**
  ★**먼저 쟀다**: 같은 것을 네 가지로 세어 **전부 268** · ★**틀리게 울 조건도 0**(한 줄 둘·쉼표 없음·주석 속·중복 이름) ⇒ **green 에서 시작하는 회귀 방어**다.
  ★**두 축**: ⑴파싱 ↔ **독립 증인**(의심 대상의 자기 매치는 증명이 아니다) ⑵등재 수 ↔ 고유 이름 수(★접힌 쌍을 **이름으로** 찍는다).
  ★**양방향**: 초판 결함 «둘»을 제품 스크립트에 되살려 각각 **rc=2**(265↔268 · 268→263 + 충돌쌍 지목) · 원형상 **rc=0**.
  ★**브리프 3문**: 비교 대상 = **소스 자신**(기준선·직전값 아님) · 첫 실행/정당한 감소 = ★**생기지 않는다**(기억이 없다) · ★**죽는다(exit 2)** — 형제 회차의 「찍고 안 죽는다」와 **갈렸고 사유를 적었다**(사각 측정 ↔ 자기 입력 오독).
  ★**대가**: 등재 배열 «밖» 호출이면 false red · 진짜 중복 등재도 false red(오늘 0) · ★**바닥이지 증명 아님**(다른 이름으로 틀리면 통과) · 비용 유의차 없음.
  ★**제안 `tradeoff` 와 갈렸다**: 「줄 형태에 묶인다」를 ★**출현 계수**로 없앴다(오늘 줄 수와 동일).
- [rustjava-loadable-set-from-loader-vs-rederive-decision] ★★**적재 가능 집합은 «재유도»를 유지한다 — 로더에서 읽지 않는다.** 채택 제안 `2026-09-18-named-exception-classes-are-loadable#p2`. ★**순수 결정 · 코드 0줄** · 산출 = `docs/loadable-set-source-of-truth.md`.
  ★**전제 확인**: 「4중 3이 재유도」는 **참**(고침 커밋 `89c2e83c` 에서 갈랐다 — loadable 3 · named 1).
  ★★**그 1건이 결정한다**: `named`(코드가 무엇을 넘기는가)는 **로더가 답할 수 없다** ⇒ 로더 읽기는 **파서 하나를 없앨 뿐 파싱을 못 없앤다**(그 절반에서 형제 회차가 ★**4시간 31분** 뒤에 또 잡았다 — ★**「다른 함수 8종 41자리」**를 쓸어담는 앵커 함정이고, 세면 **33** · 맞는 답은 **0** 이다).
  ★**대가 실측**: 열거 API **0개**(268 등재가 함수 «안» 지역 배열) ⇒ **검사기 전용 생산 API** 가 필요 · 현행 **0.8초·무빌드**인데 `rust.yml` **5잡 중 4잡이 그 형상**이다.
  ★★**결정적 실측**: 초판 검사기를 지금 트리에서 돌리면 **265/263 ↔ 등재 줄 268** ⇒ ★불변식 하나로 **결함 2건이 1회차에 잡혔을 값**(현행 268/268/268). ★단 «모든 오귀속»은 못 잡는다(과소계수 계급만).
  ★**브리프 전제 1건이 거짓**: 이 repo 엔 `machine-independence-guard` 가 **없다**(다른 repo 축).
  ★**재개 조건 사전 등록**: loadable 재유도 경로에서 **세 번째** 결함이 나오면 다시 연다(「고침 이후 0」은 하루짜리라 논거로 쓰지 않았다).
- [2026-09-19-exception-reports-instead-of-aborting-p0] ★★**보고자의 부재는 보고될 수 없다 — 로더에 직접 묻고 «이름을 들어» 실패한다.** 채택 제안 `2026-09-19-exception-reports-instead-of-aborting#p0`.
  ★**이 회차가 그 순환을 «처음 실측»했다**(제안 회차는 「실측 아님」이라 적었다): 숨김 로더로 **6/21/61/121 왕복** 후 완료 · 상한 160·200 → ★**SIGABRT 스택 오버플로** ⇒ 바닥 없음 · **121~160 사이**에서 죽는다.
  ★★**측정이 설계를 두 번 기각했다**: ⑴부트스트랩 목록 추가 → **6 테스트 즉사**(해석이 «초기화»를 돌려 스레드가 필요) ⑵시스템 로더 뒤 `resolve_class` → 정상 6/6 인데 ★**숨김 로더에선 여전히 스택 오버플로**(시점만 이동).
  ★**처방**: 예외 기구를 **우회**해 로더에 직접 묻는다 — 없으면 생성 시점에 이름을 들어 실패 · 그 뒤 등재해 재질의 0.
  ★**축**: 전 **바이너리째 SIGABRT** ↔ 후 **`should_panic` 통과** · 정상 기동 **6/6 불변**.
  ★**대가**: ★패닉이고 `AGENTS.md` 와 충돌한다(대안 둘은 각각 «불가능»·«460자리 무언 변경»이라 기각 · `Jvm::new` 는 이미 unwrap 한다) · 필요보다 일찍 실패 · **이 순환만** 막는다(`String` 은 미측정 · 후속 카드).
- [rustjava-jvm-exception-throws-instead-of-unwrap] ★★**일으키려던 예외를 못 만들면 죽던 것을 «보고»로 바꿨다.** 채택 제안 `2026-09-18-named-exception-classes-are-loadable#p1`. ★시그니처 불변 · variant 0 · 호출부 편집 0.
  ★**급소**: `from_rust_string`·`new_class` 의 실패는 **이미 `JavaError`**(= 자바 예외)다 — unwrap 이 그것을 버렸다. ⇒ 그대로 돌려준다.
  ★**실측**: `panicked … unwrap() on an Err value: JavaException(java/lang/NoClassDefFoundError)` — ★올바른 보고가 **패닉 메시지 안에** 실려 사라졌다.
  ★**도달성 두 축**: 자기 호출부 **0**(named 43 전건 loadable + 43 전건 String 생성자 보유 · 이 회차 실측) ↔ ★**공개 API 로는 도달**(`wie` 가 싣는다 · 호스트가 죽는다).
  ★**양방향**(제품 함수): 전 **FAILED**(패닉) ↔ 후 **ok** · 되돌리면 red.
  ★**파급 0 의 근거**: `.exception(` **846→846** · `JavaError::` **527 편집 0** ⇒ variant 추가안이었다면 `let …else` **460자리**가 조용히 샜다(그래서 그 안을 버렸다).
  ★**대가**: 실패 시 **다른 클래스의 예외**가 온다 ⇒ 클래스로 분기하는 **제품 12자리**는 못 잡고 전파한다(죽는 것보다 낫지만 무해하지 않다).
  ★**불변**: 퇴화 경우(폴백 클래스 자체 부재)는 **무한 재귀**이고 옛 unwrap 도 못 막았다 — ★호출그래프에서 읽었고 **실측 아님**(후속 카드).
  ★**자기 diff 밖 파급 둘**: ⑴검사기 문면 3자리가 거짓이 돼 **문면만** 고쳤다(술어 무접촉 · 축 양방향 재검증 rc=1/rc=0) — 잠금의 이유가 「죽는다」에서 ★**「틀린 예외가 온다」**로 바뀐다 ⑵★형제 회차의 비리터럴 **0 → 1**(이 테스트가 그 1이다) — ★**그 회차가 관문을 «안» 건 판단이 하루 만에 값을 했다**(걸었으면 이 테스트가 막혔다).
- [rustjava-partial-clone-refusal-decision] ★★**부분 클론을 «거절하지 않는다» — 모호했던 것은 환경이 아니라 «호출 하나»였다.** 채택 제안 `2026-09-18-merge-drops-no-silent-git-failure#p0`.
  ★**진짜 blobless 클론으로 쟀다**: promisor **도달 가능**이면 답이 **완전 클론과 동일**(rc 1 · 6 dropped · 15.7s vs 2.5s) ⇒ ★거절은 «돌아가는 설정»을 막는 것.
  ★**그러나 조용한 green 은 실재**: 신선한 blobless + promisor **도달 불가** → `0 dropped` · ★**rc 0**(완전 클론은 6건).
  ★**처방**: `symbols()` 가 실패한 `show` 를 부재로 읽기 전에 **`ls-tree`** 로 트리 존재를 묻는다(부분 클론도 **트리는 갖는다**) ⇒ 문면 대조 없이 갈린다.
  ★**양방향**: 전 rc 0(거짓 green) ↔ 후 ★**rc 2 «못 쟀다»** · ★과차단 0(도달 가능 blobless 는 여전히 rc 1) · 완전 클론 불변 · **비용 유의차 없음**(구간 겹침).
  ★**위험 실현 0**: `.github` 에 `filter:` **0건**(`merge_drops` 는 `fetch-depth: 0`) — 그것이 «거절 안 함»의 근거이지 «모호함을 남길» 근거는 아니다.
  ★**선행 결함 발견(미수정)**: 출력 순서가 실행마다 다르다(set) — `origin/main` 판본 5회에 순서 2종. rc·집합 불변(후속 카드).
  ★결정을 `preflight()` docstring 에 못박았다 — 다음 회차가 같은 질문을 다시 하지 않도록.
- [rustjava-count-nonliteral-exception-call-sites] ★★**사각의 «크기»를 쟀다 — 비리터럴 exception() 호출부는 «0» 이다.** 채택 제안 `2026-09-18-named-exception-classes-are-loadable#p0`. ★**순수 측정 · `.rs` 0줄 · `scripts/` 0줄.**
  ★**수**: bare `exception(` **847** = 정의 1 + **리터럴 846** + 그 밖 리터럴 **0** + ★**비리터럴 0** ⇒ 검사기가 보는 집합 = 실제 호출부 집합(지금은 일치).
  ★★**술어를 갈라야 답이 맞는다** — `exception(` 부분일치가 `assert_exception(` 등 **다른 함수 8종 41자리**(첫 인자가 `jvm`)를 쓸어담는다. 안 갈랐으면 ★**M=33 이라는 틀린 답**이었다.
  ★**양방향으로 술어를 시험했다**: 대조군 = 한 줄 리터럴만 세면 **812** = 검사기 초판 수와 정확히 일치 · 개악 주입(변수·`format!`·`const`·raw string) → 전건 `nonliteral` **0→4**, 비-java 리터럴 → `literal_other` **0→1**, 원복 후 **0/0**·트리 클린.
  ★**단위 주의**: 매크로 본문 **3자리 × 22전개** ⇒ 전개 기준이면 **865**(소스 기준 846). 이름이 전부 리터럴이라 **답은 불변** — 사각이 아니라 단위 차이다.
  ★**잃는 것**: 토큰 붙이기 매크로는 어떤 텍스트 술어도 못 본다(이 트리 0) · 「리터럴인데 오타」는 종전 한계 그대로 · `new_class(`·`find_class(` 는 요지 밖이라 미계수.
  ★**게이트 승격은 «안 했다»** — 제안이 요구한 것은 계수이고, 베이스라인 0 관문은 별 결정이라 후속 카드로 남겼다.
- [rustjava-merge-dropped-symbols-checker-swallows-git-failures] ★★**「조용한 실패」 검사기에 «조용히 통과하는 길»이 있었다 — 닫았다.**
  ★**재현 = 진짜 얕은 클론**(`--depth 10`): 전 **rc=0** `✓ 56bb54fa (0 file(s) examined)` ↔ ★완전 클론에선 **examined 20** ⇒ 20→0 으로 접히고 green. 후 **rc=2 `cannot measure: shallow clone…`**.
  ★raise 경로도 쟀다 — 범위 오류·루프 내 diff 실패·비-git **전부 rc=2**(git stderr 동봉).
  ★★**급소**: `run()` 호출부 8곳 중 ★**`git show <rev>:<path>` 하나만 «실패가 답»**(경로 부재는 정상) ⇒ rc 로 못 가르니 **환경을 preflight 로 배제**.
  ★**ⓒ CI 는 이미 `fetch-depth: 0`** ⇒ 전 PR 을 막지 않는다(그 job 의 얕은 클론 빈도 **0**) — 그래서 처방이 «rc 올리기»로 정해졌다.
  ★**양방향**: 실패→rc=2 · ★**정상 0**(`97660921`)→**여전히 ✓ rc=0** · 탐지 회귀 0(`e53b2142` 6 · `514d5b08` 6 · `56bb54fa` 20).
  ★**잃는 것**: 얕은 트리 수동 실행은 이제 빨강 · 표시/면제 호출까지 일괄 raise 라 **더 자주 멈춘다** · ★preflight 는 «얕음»만 본다(부분 클론은 남았다) · F8 「0」의 인과는 **미증명**.
  ★`.rs` 0건 · 술어·필터 폭 무접촉 · 종료코드는 기존 `2` 재사용. ★**PR 은 #71 에 쌓여 있다**(검사기가 main 에 없다) — 계약 5 재지정 필요.
- [rustjava-lock-every-named-exception-class-is-loadable-fix] ★★**검사기의 «거짓 초록» 2건 + «거짓 빨강» 1건 정정**(게이트² 반려 승계 · PR #72).
  ★**F1** 짧은 이름 충돌(`Formatter`·`JarURLConnection` **2쌍**) ⇒ 한쪽 등재를 지워도 **전 rc=0(거짓 초록)** → **후 rc=1**. 키를 **(모듈,타입,함수)** 로.
  ★**F2** 줄 단위 스캔이 다중 줄 호출을 못 봄 ⇒ **전 rc=0(안 보임) → 후 rc=1**. ★**846 − 34 = 812** 로 검수자 수와의 차이를 설명했다.
  ★**F3** `list_proto` 3건 누락 + 한 `impl` 의 둘째 생성자 오귀속 ⇒ ★**초판의 「미등재 5건」은 틀렸다 — 실제 «0»**(5곳 정정).
  ★**수 전/후**: 이름 41→**43** · 호출부 812→**846** · loadable 263→**268**(왜인지 기재).
  ★**ⓒ 편집 «전»에 새 red 위험 측정** — 늘어나는 2이름 모두 등재 ⇒ **새 red 0**.
  ★**시간 유의차 없음**(전 1.42/1.67/1.46 ↔ 후 2.01/1.69/1.19 · 구간 겹침). ★런타임 클래스 추가 0 · `protos` 무접촉.
- [rustjava-bootstrap-argument-diagnostic-names-index-and-tag] ★★**부트스트랩 인자 거부가 «어느 인자·무엇을» 말한다.** 채택 제안 `2026-09-18-bootstrap-argument-diagnostic-sequencing#p0`.
  ★**전제 확인(코드)**: 술어가 `false` 를 내는 자리에 `index` 와 `constant_pool.get(index)` 가 **손에 있었다** — `bool` 이 둘을 버렸다.
  ★**새 체계 0** — 같은 enum 의 `UnsupportedVersion(u16)` 이 이미 3층을 관통하는 패턴이라 **복제**했다.
  ★**전/후**: `ClassFormatError: a bootstrap method argument names nothing or is not a loadable constant` → ★`ClassFormatError: bootstrap method #0 argument #0 names no constant pool entry`(CLI 실측).
  ★**세 요구 3/3**(expected·index+method_index·actual) · 태그는 **이름**으로(번호는 표가 하나 더 는다).
  ★**양방향 급소**: `StringConcat` 은 인자 1개라 index 0 이 하드코딩이어도 통과 ⇒ **인자 3개 `Lambda.class` 의 #0↔#2** 로 **index 가 따라가는 것**을 잠갔다.
  ★**소비자 1개**(`jvm-bytecode` `From`) — arm 추가뿐, 14규칙 무변. ★**타입 «안 커졌다»**(`Copy`·크기 불변 시험) · ★메시지 문면이 바뀌어 단언 2곳이 바뀐다.
  ★`-p classfile` **16 passed**(전 13) · `test_class_format` **22** · `--all` rc=0.
- [rustjava-lock-every-named-exception-class-is-loadable] ★★**이름으로 부르는 예외 클래스가 «실을 수 있는» 것인가 — 대조 한 자리.** 채택 제안 `2026-09-17-string-concat-recipe-arity#p0`(worklog json 기록).
  ★**전제 확인(코드)**: `jvm/src/jvm.rs:943-950` `new_class(...).await.`**`unwrap()`** ⇒ 못 싣는 이름은 **throw 가 아니라 패닉**. ★자기 참조 — `:842` 가 부재를 `exception("java/lang/NoClassDefFoundError")` 로 보고한다.
  ★**베이스라인 0**(★게이트² 정정 후): `exception(` 리터럴 **43 고유 / 846 호출부** ↔ 등재 **268**(`as_proto` 265 + `list_proto` 3). ★초판의 41/812/263 은 **전부 과소**였다(다중 줄 34 · `list_proto` 3 · 짧은 이름 충돌). ★제안의 「72」는 여전히 **재현 안 됨**.
  ★**실을 수 있는 집합 = 등재분** · ★**초판의 「미등재 5건」은 틀렸다 — 실제 미등재 «0»**(`name:` 268 = 등재 268).
  ★**양방향**: 현 상태 rc=0 · ★`BootstrapMethodError` 등재 제거 → **rc=1**(실제 패닉 사례 재현) · 프로토 이름 오타 → rc=1 · ★해석 불가 등재 → **rc=2(못 쟀다 · fail-closed)** · CI step 제거 → `dod_parity` rc=1.
  ★**못 보는 것**: **런타임 조립 이름 안 보임**(바닥이지 증명 아님) · `exception(` 만 · 초기화 실패 통과.
  ★**잃는 것**: DoD **6→7**(~1초 · 초판 32.7초는 `target/` 가지치기로 해소) · ★`.unwrap()` 무접촉(전환은 범위 밖).
  ★`--all` rc=0 · `dod-ci-parity` **명령 7개** rc=0.
- [rustjava-count-symbols-dropped-from-second-parent-on-resolution] ★★**충돌 해소가 떨어뜨린 정의를 센다 — 그리고 개악 시험이 «내 검사기»의 구멍을 잡았다.** 채택 제안 `2026-09-17-union-restores-silently-dropped-makeconcat#p0`. ★**제품 코드 0줄**(검사기 + 규약 + 기존 워크플로 잡 1).
  ★**ⓑ 를 먼저 쟀다** — 손실을 재구성해 기존 축 전수(생성기·단일결함 감사·두 스위트·파리티·`git status`·마커) ⇒ ★**전부 green = 아무것도 못 잡는다.** ★조용한 조건은 「떨어진 이름들이 **자기완결**일 때」이고, 한쪽을 통째로 택하는 것이 정확히 그것이다.
  ★**ⓐ 3건 → «4건»** · ★**ⓒ** 넷째가 **클래스 «안»의 메서드**라 「최상위」만 보면 **4 중 3**만 잡는다 ⇒ 한 단계 중첩까지.
  ★★**설계 교체**: 「트리==부모1 ⇒ `-s ours` 면제」가 **개악을 삼켜 rc=0** 이 됐다(그게 존재 이유인 실패다) ⇒ **선언(`*` trailer)으로 교체**.
  ★양방향 전건: 개악 **rc=1 + 이름** · 정상 **rc=0** · trailer **rc=0**/지우면 **rc=1** · 스쿼시 **rc=0** · 실사고에서 **손 분할과 동일**(4 사고 + 2 의도).
  ★★**[게이트② 승계] 필터가 좁아 «둘째 사고»(`514d5b08`)를 놓쳤다** — 근거 문장이 거짓이었다 — 초판은 **파일 4개를 보고도** findings 0 · rc=0(확장본은 같은 머지에서 **13개**) ⇒ **「theirs 가 바꾼 파일 ∪ 머지가 바꾼 파일」**로 넓혀 ★**둘 다 rc=1**. ★docstring 의 유일한 예시가 **면제되지 않는 형식**이었다 ⇒ 고치고 「출력에서 복사하라」를 계약에 박았다(당시 trailer 0건).
  ★**오탐 실측 — 이 회차가 직접 돌렸다**(창 `38b0df38..8c7b473f` 83머지): 초판 **8건/19정의** → 확장본 **10건/26정의**. ★늘어난 것은 **7정의**(`514d5b08` 6 + `37ea5a13` 1)이고 ★**진짜 4 · 오탐 3**. ★**시간은 «유의한 증가 없음»이다**(판본 차 ≈0.8s/머지 ↔ 같은 판본 재실행 편차 **2.6배** · 수 출처 = 게이트② 검수 실측 · ★재측 안 함). ★「느리지 않았다」는 틀렸다 — wide 는 narrow 의 상위집합이라 더 싸질 수 없다. ★**잃는 것**: 본문만 빈 경우·리네임을 **못 가른다** · trailer 는 **우회로**이고 **이름이 글자 그대로 대조**된다 · ★★**새 red 계급 — main 에도 있는 정의를 지운 브랜치는 다음 base 당김에서 red 가 된다**(실례 `37ea5a13` · 7 중 3).
  ★`--all` **583/0/1**(불변) · 파리티 **명령 7개 일치**(DoD 블록 동기).
- [rustjava-adopt-loadable-bootstrap-arguments-diagnostic] ★★**대전제 ⓒ 에서 끝났다 — 그 일을 하는 축(PR #67)이 이미 떠 있다.** 채택 제안 `2026-09-17-loadable-bootstrap-arguments#p0`(worklog json 기록). ★**코드 0행.**
  ★**전제는 참**(CLI 실행: 세 규칙이 전부 `ClassFormatError: Invalid class file` 동일 문면) — 그러나 ★**편집 영역은 전부 겹친다**(전달된 값은 **3 중 1** — 아래): #67 이 제안 `target` 두 파일을 고치고, 이 술어에 **이미 사유를 주며**, 밋밋한 문면 **두 자리**를 둘 다 고쳤다. ★제안 `tradeoff` 자신이 「두 번 하지 말고 함께 하라」고 적었다.
  ★**남는 잔여는 좁다** — `&'static str` 이라 **인덱스를 못 담는다** ⇒ 「기대」 달성 · 「인덱스·실제」 미달 ⇒ ★**새 카드를 냈다**(★**M** — 초판 `S` 에서 **실측 후 올렸다**: 잔여도 **같은 3층**을 건너 `target` **5파일/4크레이트**(`classfile`·`jvm-bytecode`·`RustJava`·`test-utils` — ★층 3 ↔ 크레이트 4: 경계 층이 두 크레이트에 걸친다) · `InvalidFormat` 을 넓히면 생성 17 + 매치 11 이라 **새 variant** 를 고르고 **두 갈래의 대가**를 적었다).
  ★**설계 제약 기록** — `ClassFileError` 는 `Copy`(파일 4 · 크레이트 2 의존) · 정적사유+`u16`+`u8` 로 **깨지 않고 된다**.
  ★**잃는 것**: main 은 #67 착지까지 **밋밋한 채**(규칙 이름조차 없다) · 이 회차가 전달한 것은 **값이 아니라 순서**다.
  ★`--all` **583/0/1**(불변 · base `8c7b473f`).
- [rustjava-adopt-test-data-version-freeze-uniform-target-p0] ★★**루트 fixture 를 한 target 으로 모을 것인가 — «모으지 않는다».** 채택 제안 `2026-09-17-test-data-version-freeze#p0`(worklog json 기록). ★**코드 0행 · 재컴파일 0 · `.class` 0 변경** — 산출물 = `docs/test-data-target-policy.md`.
  ★분포 **114**건 · **52×40 · 65×62 · 66×8 · 68×1 · 70×3**(fixture 자신에서 읽음).
  ★★**양방향**: ⒜**버전이 답** — `StringBuilder` 3건을 21 로 재컴파일 → ★indy 생기고 StringBuilder 사라짐(★`StringConcat` 이 루트의 **유일한** indy) · `ThreadInterruption` `access$`×10 → **0 + NestMembers**(JEP 181) ⒝**아무 버전이나** — 20건 중 ★**16건 명령 시퀀스 완전 동일**.
  ★**그 커버리지는 다른 데 없다** — 생성기 산출 64건에서 `StringBuilder` 0 · `access$` 0.
  ★**이유**: 제안의 이득(「숫자 하나로 예측」)이 뒤집힌다 — 52 는 「전-indy·전-nestmate」라는 뜻을 **실제로 갖고**, 펴면 그 구분이 사라지며 유일한 커버리지가 지워진다.
  ★**잃는 것**: 비균일 잔존(신규 target 규칙 **미수립**) · 16건은 그대로 · ★**본 것은 40 중 20** · `NativeMethod` 차이는 **미규명**.
  ★`--all` **583/0/1**(불변 · base `8c7b473f` — 앞 회차의 579 는 #61 착지 전 base 다) · 되돌릴 조건 4개를 결정 문서에 명시.
- [rustjava-adopt-javac-fixture-provenance-verified-p0] ★★**루트 픽스처 다섯이 재빌드되지 않는 이유 = `-g`.** 채택 제안 `2026-09-17-javac-fixture-provenance-verified#p0`(worklog json 기록). ★**제품 Rust 0줄 · 커밋 `.class` 바이트 0 변경.**
  ★**제안이 댄 두 설명이 둘 다 틀렸다** — 컴파일러 아니다(**26.0.1·26.0.2.1 에서 같은 다섯이 같게 행동**) · 소스 발산 아니다(`-g` 면 **지금 소스가 커밋 바이트를 낸다**).
  ⇒ ★**제안 `tradeoff` 의 「재컴파일이 동작 변경이 될 수 있다」가 «성립하지 않는다»** — 재컴파일이 불요다.
  ★고친 한 자리: 스크립트가 `--release` 를 픽스처에서 읽듯 **`-g` 도 픽스처에서**(상수풀 `LocalVariableTable`). ★배선 전 판별력 실측 **5/5 · 오탐 0**(보유 5 · 미보유 107).
  ★양방향: 정상 **109/109/0 differed** ↔ 개악(파생 한 줄 no-op) **104/5 differed**(원래 다섯과 동일).
  ★**잃는 것**: 스크립트는 **여전히 rc=1**(재빌드 불가 3건 = 제안이 미해결로 적은 별 축 · 넓히지 않았다) · 판정이 한 속성 유무에 걸린다.
  ★`--all` **579/0/1**(불변) · `check-dod-ci-parity` OK.
- [rustjava-adopt-classfile-error-cause-decision-p0] ★★**거부 사유를 세 층에 꿴다 — 「Invalid class file」 하나가 **14개** 문장이 된다(클래스 8 · 필드 3 · 메서드 3).** 채택 제안 `2026-09-17-classfile-error-cause-decision#p0`. ★**제품 동작 변경 있음**(사용자가 보는 `ClassFormatError` 메시지).
  ★제안이 **all-or-nothing** 이라 못박은 넷을 다 했다: `InvalidFormat(&'static str)` · `InvalidClassFile(&'static str)`(★`From` 이 **버리던** 자리) · 경계 2자리 · ★**`validate_class` 8항 `||` → 규칙마다 `if`**.
  ★★**사유를 꿰자 «평평한 오류가 가리던 것 둘»이 나왔다**: ⑴테스트가 **어느 층이 거부하는지를 틀리게 믿었다**(검증 아닌 **파서**) ⇒ ★단언을 실측에 맞췄다 ⑵술어 **이름이 낡아 있었다**(「in_the_pool」인데 **적재 가능성까지** 본다) ⇒ 사유는 규칙대로, ★**이름은 안 바꿨다**(리팩터 금지).
  ★**양방향 — 세 층 전부 개악**: M1 경계 · M2 `From` 이 사유 버림 · M3 두 사유를 한 문자열로 접음(★잡는 것은 `contains` `:450`(dedup 제거로 452→450) — 초판이 귀속한 dedup 단언은 **상수 대 상수라 공허**했고 **걷어냈다**) · 복원 17/0.
  ★★**대가**: ★**마지막 홉이 두 번 쓰여 있고 `test-utils` 사본은 «무검증»**(개악해도 579/0 · **합치지 않고 보고**) · 사유가 문자열이라 같은 문구 중복을 막는 것이 없다 · ★**PR #66 과 같은 함수**(충돌은 기계적).
  ★`--all` **578 → 579/0/1** · `check-dod-ci-parity` → **「OK 두 축 모두 대칭차 0 — 명령 6개 · toolchain 2개로 «둘 다 일치»」**(rc=0).
- [rustjava-adopt-link-stringconcatfactory-p2-fix2] ★★**#57 의 버전 표에 25행 등재 — 「착지 순서」가 만든 부채를 갚는다(PR #61).**
  ★**막힌 것은 CI 도 충돌도 아니었다**: 핀 `85cf0fba` 에서 rc=0 CI_GREEN · `git merge origin/main` **코드 충돌 0** 인데
  ★**합친 결과**가 #57 이 세운 「미등재 픽스처는 핀을 실패시킨다」를 어겼다(미등재 **25건** 재현).
  ★생성기(`record-class-file-versions.py`)를 **돌려서** 채웠다 — 손편집 0. ★**삭제행 0**(`25  0`) = 기존 픽스처 재생성 0 이 이 회차의 안전선.
  ★추가 25행이 이 PR 의 25개 `.class` 와 **집합 동일**(혼입 0) · ★**양방향**(한 행 제거 → red · 되돌림 → green).
  ★원장 충돌 2건은 **합집합**(줄 단위 양방향 보존 · 한쪽 통째 채택 0) · #62 기여 전건 잔존.
  ★`test_fixture_pins` **3/0** · `--all` **581/0/1** · DoD 7명령 rc=0.
  ★★**[승계 -p2-fix3] 코드 2파일 합집합 — ours 의 «삭제»가 의도가 아니었다.**
  ★선행 머지 둘(`e53b2142`·`514d5b08`)이 부모2의 makeconcat 가족(`fieldref`·`make_concat_call_site`·`LINKED`)을 **결과에서 떨어뜨렸다** ⇒ **되살렸다**.
  ★반면 `.rs` 의 16줄은 **진짜 ours 의도**(metafactory 를 링크하니 「링크 안 된다」 단언이 거짓) ⇒ **되살리지 않았다**. 회계: `.py` ↔main **156/0** · `.rs` ↔HEAD **86/0** · ↔main **182/29**.
  ★**복원분이 산 코드임을 실행으로 증명** — MakeConcat 3장을 지우고 재생성 → **바이트 동일 복구**(복원 전 생성기로는 **불가**).
  ★양방향 개악 ours 4 red / theirs 1 red · `--all` **583/0/1** · DoD 7명령 rc=0. ★그 회차는 「착지 금지 — 게이트② 재검이 먼저」로 끝났고, ★**그 재검이 approve 로 닫혔다**(아래).
  ★★**게이트③ 착지 — PR #61 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` · 티켓 `merge_strategy: merge` 선언분 ⇒ ★**계보 보존**). ★한 PR 이 `-p2`·`-fix`·`-fix2`·`-fix3` **네 회차**를 함께 싣는다.
  게이트② **approve**(리니지 최신 회신 `…-p2-fix3.review.md`) · 핀 **`5f7ce1a8`** ↔ 착수 시 PR head **동일**(불이동) · ★**`MERGEABLE/CLEAN` · base 뒤처짐 «0»** ⇒ 충돌 해소·base 당김 **둘 다 불요**(`-fix3` 이 이미 당겼다).
  ★핀에서 `ci-presence` **rc=0 CI_GREEN**(3건 전건) · 자식 PR **0건** · 배포 **0**(배포 워크플로 없음) · 주기 자동 커밋 **0건** · 라이브 실행 주체 **없음**.
  ★**선행 `-merge` 두 건은 흡수할 것이 없었다** — `…-fix-merge`(`needs-fix-ticket`)·`…-fix2-merge`(`code-conflict-out-of-scope`) 둘 다 **머지 0·커밋 0·푸시 0** 으로 멈췄다.
- [rustjava-adopt-class-format-mutation-audit-p0-fix] ★★**판정식을 `given` 에서 파생시킨다 — 게이트② 반려 승계(PR #63).**
  ★**급소 한 줄**: `repaired` 를 정본 인자로 **다시 짓고** 있어 둘째 결함을 버렸다 ⇒ 「single-defect 인가」를 묻는데 **입력이 이미 single-defect** 였다(순환 · 18중 **14건**).
  ★처방은 발명이 아니라 **옮겨오기** — 이미 옳던 `indy` 근접실패 형태를 `add()` 한 곳으로 모아 **4족 전건**이 지나게 했고,
  `given` 을 `inspect.signature(...).bind(*spec)` 로 **이름에 묶어** 생성기에 인자가 늘어도 자동 승계된다.
  ★**비대칭이 검사의 전부**(`repaired` 만 파생 · `canonical` 은 아니다) ⇒ 그 비대칭을 지키는 **`AUDIT SPEC STALE`(rc=2)** 가드 신설 — 모르면 «판정하지 않는다».
  ★★**족마다 개악**(한 족 red 의 일반화가 이번 결함을 살렸다): **M-A** ldc rc=0→**rc=1** · **M-B** cp rc=0→**rc=2(SPEC STALE)** ·
  **M-B2** 정본 고지 후 →**rc=1(1B)** · **M-C** indy 근접실패 rc=1→**rc=1**(일하던 족 유지) · **M-D** indy LINKED rc=0→**rc=1**.
  ★★**잃은 것**: 커밋 픽스처 18건은 **여전히 전건 통과**다 — 내려간 것은 «통과 수»가 아니라 **«통과 가능한 입력 집합»**(실패 불가 사례 **14 → 0**).
  대신 판정이 `CANON` 표에 의존하게 됐고(틀리면 거짓 MULTI-DEFECT), 생성기가 자라면 SPEC STALE 이 **막는다**(고의). 시험 시간 증가는 ★**실측 9.57초**(개악 5종 · 스크래치 5벌) · 감사 1회 **0.31초**.
  ★**상시 CI 검사는 만들지 않았다** — 공허한 검사를 DoD 에 박지 않으려고 판정식을 먼저 조였다. ★제품 Rust **0줄**.
  ★★**게이트③ 착지 — PR #63 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` · 티켓 `merge_strategy: merge` 선언분 ⇒ ★**계보 보존**). ★한 PR 이 `-p0`·`-p0-fix` **두 회차**를 함께 싣는다.
  게이트② **approve** · 핀 **`3472d642`** ↔ 착수 시 PR head **동일**(불이동) · ★**`CONFLICTING`** ⇒ 예외 사유 ⓕ(등재 repo 는 무조건 별 `-merge`)로 발권된 회차다.
  ★**충돌은 원장 2파일뿐**(`STATE.md`·`REPORT.md` 상단 삽입 · 제품 코드 **0**) — base 당김(뒤처짐 **15**) + 합집합 해소. 보존 증명 양방향 소실 **0** · 기여 불변 `--numstat` 정확 일치.
  ★핀에서 `ci-presence` **rc=0 CI_GREEN** · 자식 PR **0건** · 배포 **0**(배포 워크플로 없음) · 주기 자동 커밋 **0건** · 라이브 실행 주체 **없음**.
  ★★**고지(비차단 · 이 회차가 고치지 않는다 · base 를 «두 번» 당긴 뒤 재측)**: 형제 회차가 병렬로 들여온 픽스처 ★**10장이 감사 범위 밖**이다 —
  `test-data/indy/RecipeWants{AConstant,FewerArguments,MoreArguments}.class` **3장**(PR #57 계열) + `test-data/attr/Duplicate*.class` ★**7장**(PR #66 · 이 회차 중에 착지).
  병합 트리에서 감사기 재실행 = **검사 18 · 무결함 5 · 합계 23 · rc=0** 인데 그 10장은 ★**출력에 한 줄도 없고** `AUDIT SPEC STALE` 도 **울지 않았다**.
  ⇒ `CANON` 표가 수기라 ★**형제가 들여온 픽스처는 «조용히» 비어 있다**(감사기는 「자란 생성기」만 막는다). ★차단 사유 아님 — 총괄 승계 판정 대상.
- [rustjava-adopt-class-format-mutation-audit-p0] ★★**픽스처 «단일 결함» 감사.** ★**[교정 · `-fix` 회차] 종전 제목의 「18/18 통과」는 «측정»이 아니었다** — 아래 교정 줄 참조. 채택 제안
  `2026-09-16-class-format-mutation-audit#p0`(worklog json `adoptedProposals` 기록). ★**제품 코드 0줄**(감사 스크립트 1개).
  ★**판정식을 «로드»가 아니라 «바이트 동일»로** 잡았다(생성기가 결함을 인자로 받으므로 더 강하고 더 싸다):
  `generator(결함)==커밋본` **그리고** `generator(수리)==generator(표준)`.
  ★★**[교정 2026-09-17 · 게이트② request-changes] 그 「전건」은 «측정»이 아니었다** — 18건 중 **14건**이 `repaired` 를 정본 인자로
  «다시 지어» 둘째 결함을 버렸다(구조적으로 MULTI-DEFECT 불가). ★**실측된 것은 4/18**. 정정은 `-fix` 항목이 진다.
  ★**덮지 않는 것을 스크립트에 적었다**(javac 산출물 · 적법-미구현 · 테스트 안 바이트 패치분).
  ★개악 2종 red(둘째 결함 심기 **rc=1** · 커밋본 1바이트 반전 **rc=2**) · `--all` **576/0/1**(Rust 무접촉이라 불변).
- [rustjava-adopt-reject-duplicate-bootstrap-methods-p0] ★★**클래스 수준 속성 개수 규칙 — 다섯에 «예».** 채택 제안 `…-reject-duplicate-bootstrap-methods#p0`(worklog json 기록). ★**제품 동작 변경 있음.**
  ★**제안 기준만으로는 «아니오»였다**(하류 `find_map` 소비자가 있는 것은 `BootstrapMethods` 뿐 · 나머지 6종 소비자 0).
  ★★**판정을 바꾼 것은 진짜 JVM 이다** — OpenJDK 26.0.1 이 `SourceFile`·`InnerClasses`·`SourceDebugExtension`·`BootstrapMethods`(52)·`NestHost`·`NestMembers`(55) 중복을 ★**전건 `ClassFormatError`** 로 거부한다
  ⇒ ★**제안의 「오늘 로드되는 파일을 더 거부한다」는 «거짓»**(그 파일들은 진짜 JVM 에서도 안 열린다).
  ★★**통제군 둘이 표·버전게이트의 이유**: `NestHost`@**52** 는 ★**로드된다**(미정의 ⇒ 무시) · `Synthetic` 은 스펙이 하나라는데 ★**HotSpot 이 둘을 받는다**(근거로 뺐다).
  ★**개악 4종 전건 red**(M1 게이트 제거·M2 Synthetic 추가 → **통제군** red · M3 한 칸 제거 · M4 호출부 원복) · 복원 17/0.
  ★**대가**: 동작 변경 · 표는 수동 목록(늘어도 안 울린다) · Synthetic 배제는 JVM 하나에 의존 · ★**`attribute.rs` 한 줄**(제안 target 밖 — 신고).
  ★`--all` **579/0/1** · 버전 표 **+7행** · DoD 7명령 rc=0.
  ★★**게이트③ 착지 — PR #66 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` · 티켓 `merge_strategy: merge` 선언분 ⇒ ★**계보 보존**).
  게이트② **approve** · 핀 **`1da1379e`** ↔ 착수 시 PR head **동일**(불이동) · ★**`CONFLICTING`** ⇒ 예외 사유 ⓖ 로 별 `-merge` 가 발권된 회차다.
  ★**충돌은 원장 2파일뿐**(`STATE.md`·`REPORT.md` 상단 삽입 · 제품 코드 **0**) — base 당김(뒤처짐 **3**) + 합집합 해소. 보존 증명 양방향 소실 **0** · 기여 불변 `--numstat` 정확 일치.
  ★핀에서 `ci-presence` **rc=0 CI_GREEN** · 자식 PR **0건** · 배포 **0**(배포 워크플로 없음) · 주기 자동 커밋 **0건**(최다 규칙성 author cv **0.57** > 0.1) · 라이브 실행 주체 **없음**.
- [rustjava-adopt-ldc-tags-real-world-generator-survey-p0] ★★**「ldc 픽스처를 ASM 으로 재생성」 제안 — 기각.** ★**제품 코드 0줄**(`declinedProposals` 기록).
  ★**사유 ⑴ 더 센 오라클이 이미 있다** — ★**OpenJDK 26.0.1 이 양성 4건을 실행한다(전건 rc=0)** · 위법 5건은 전건 rc=1 · 「ASM 이 낸다」는 선행 회차가 이미 동적 실증.
  ★**⑵ 손의 흔적은 «옮겨갈» 뿐이다**(ASM 도 드라이버 15줄이 모양을 고른다) ★**⑶ 생성기가 «두 기구»가 된다**(음성 픽스처는 ASM 불가) ·
  값의 실체는 「CI 가 깨진다」가 **아니라**(CI 는 픽스처를 재생성하지 않는다 — 참조 0건) ★**「어디서나 한 줄」이 「네트워크+JDK+핀 jar」가 되는 것**.
  ★★**⑷ 단일결함 감사의 첫 등식(`generator==committed`)과 충돌** ⇒ `GENERATOR DRIFT rc=2`. ★단 그 감사기는 **미착지**(PR #63)라 「착지하면 충돌하는 축」으로 적었다.
  ★★**잃는 것**: `synthetic` 단서가 남고, ★우리 인코더의 체계적 편향을 파서«와» JVM 이 둘 다 관대하게 넘기는 경우는 못 잡는다(검증기는 센 필터이지 증명이 아니다).
  ★후속 카드 1건 — 재생성 대신 **양성 픽스처를 진짜 JVM 에 올리는** 현실성 검사(선례 `verify-javac-fixtures.sh`).
  ★★**게이트③ 착지 — PR #64 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` · 티켓 `merge_strategy: merge` 선언분 ⇒ ★**계보 보존**).
  게이트② **approve** · 핀 **`c0413f81`** ↔ 착수 시 PR head **동일**(불이동) · ★**`MERGEABLE/CLEAN` · base 뒤처짐 «0»** ⇒ 충돌 해소·base 당김 **둘 다 불요**.
  ★핀에서 `ci-presence` **rc=0 CI_GREEN** · 자식 PR **0건** · 배포 **0**(배포 워크플로 없음) · 주기 자동 커밋 **0건** · 라이브 실행 주체 **없음**.
  ★★**착지시킨 것은 «기각 기록»이다** — 제품 코드 **0줄**이 정상이고, 착지 diff 는 원장 4파일뿐이다.
- [rustjava-adopt-ldc-tags-real-world-generator-survey-p1] ★★**kotlinc·scalac 타깃 형상 조사 — 0. 그러나 «다른 0».** 채택 제안 `…-survey#p1`(worklog json `adoptedProposals` 기록) · ★**제품 Rust 0줄**.
  ★조사 회차가 **「못 쟀다」로 비워 둔 칸**을 채웠다 — 종전 축은 stdlib(호환성용 컴파일)이라 「이 코퍼스에서 0」과 「이 기능들에서 0」이 달랐다.
  ★**kotlinc 2.4.20 → 0 · scalac 3.9.0 → 0**. ★★**풀 수치가 그 0 을 읽을 값으로 만든다**(Kotlin `MH 7·MT 6` · Scala `11·6` = **indy 가 실제로 돌았다**) ⇒ 「안 썼다」가 아니라 **「썼는데 `ldc` 자리에 안 온다」**.
  ★**태그 17 은 풀에도 0** ⇒ ★**이 형상들에서는** 두 컴파일러 다 **condy 를 안 낸다**(한정은 나머지 수와 같다 — 컴파일러당 프로그램 1개).
  ★**양방향**: 양성 대조군 `test-data/ldc` → `{MH 1, MT 2, Dynamic 7}` ⇒ 「스캐너가 못 본다」 배제 · 오차막대 **0.00%**.
  ★**제안의 값 전제가 부분적으로 거짓**이었다 — openjdk 26 은 **2026-07-22 부터** 설치돼 있었다(`INSTALL_RECEIPT` **`time`** = `1784711495` · ★`source_modified_time` 아님) — 조사(09-16)보다 **56일** 전.
  ★★**대가**: 머신에 `kotlin`·`scala` **설치됨**(재측정 위해 남겼다) · ★**추가 신고 — `openjdk` 기본 링크 26.0.1 → 26.0.2.1 승격**(의존성 · `/opt/homebrew/opt/openjdk`).
  ★`brew uninstall kotlin scala` 로는 **안 돌아온다**(Homebrew 7 은 `brew switch` 없음) ⇒ 처방은 **26.0.1 keg 경로 핀**(`JAVA_HOME=/opt/homebrew/Cellar/openjdk/26.0.1` · 실행 확인). ★keg-only 라 `PATH` 의 `java`(=`/usr/bin/java`)는 **불변** — 영향은 opt 경로를 명시적으로 쓰는 소비자뿐.
  ★컴파일러당 프로그램 1개 · **CI 불가**.
- [rustjava-adopt-link-stringconcatfactory-p1-fix2] ★★**base 당김 — 그런데 막고 있던 코드 충돌은 «이미 없었다»(PR #60).**
  ★**전제 반증**: 「`make_indy_fixtures.py` 4구역 충돌」은 `-p1-fix` 가 **14:10 `0f06b93f`** 로 합집합 해소했고 게이트②가 **15:43 그 head 를 approve** 했다.
  발권 근거(12:12 blocked 회신)가 그 사이 낡은 것이다. ★**재발 불가**도 확인 — 뒤진 9커밋 중 그 파일을 만진 것 **0건**.
  ★**합집합이 진짜인지 다시 쟀다**: 생성기 재실행이 10픽스처 **바이트 불변**(양쪽 가족을 한 생성기가 낸다) ·
  ★**양방향 개악** — ours 산출물만 치우면 ours 만 red · theirs 만 치우면 theirs 2건만 red ⇒ 「선택」이 아니다.
  ★**실제로 남아 있던 것은 부채**다: 당기면 #57 의 표가 들어와 이 PR 픽스처 3개가 미등재 ⇒ 생성기로 **3행**(★삭제 0) 등재.
  ★원장 2파일 합집합·시간순 · `--all` **578/0/1** · DoD 7명령 rc=0.
  ★★**게이트③ 착지 — PR #60 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` · 티켓 `merge_strategy: merge` 선언분 ⇒ ★**계보 보존**).
  게이트② **approve**(리니지 최신 회신 `…-p1-fix2.review.md`) · 핀 **`632c6b06`** ↔ 착수 시 PR head **동일**(불이동) ·
  ★**`MERGEABLE/CLEAN` · base 뒤처짐 «0»** ⇒ 충돌 해소·base 당김 **둘 다 불요**(앞 회차가 이미 당겼다).
  ★핀에서 `ci-presence` **rc=0 CI_GREEN** · 자식 PR **0건** · 배포 **0**(이 저장소에 배포 워크플로 없음) · 주기 자동 커밋 **0건** · 라이브 실행 주체 **없음**.
  ★**동봉은 이 기록 한 줄뿐** — 원장(worklog 쌍·`STATE`·`REPORT`)은 구현·승계 회차가 이미 실었다.
- [rustjava-adopt-bound-bootstrap-static-arguments-p0] ★★**부트스트랩 정적 인자 = «적재 가능 상수» — 경계에서 «종류»로.**
  채택 제안 `2026-09-16-bound-bootstrap-static-arguments#p0`(worklog json `adoptedProposals` 기록). ★**제품 동작 변경 있음.**
  ★**제안이 적은 위험을 먼저 쟀다**(「틀리면 람다가 전부 corrupt」): 태그 검사는 payload 를 읽지 않으므로 `attribute.rs` 설계와 충돌하지 않고,
  ★**커밋된 클래스 파싱이 전/후 «144/12 동일»**(새로 거부 0). ★OpenJDK 26 은 같은 파일을 `ClassFormatError: argument_index 4 has bad constant type` 로 거부한다.
  ★**안 하면**: 링커에서 `UnsupportedOperationException` — 「파손」을 「미지원」이라 말하게 된다.
  ★개악 2종 red(존재만 되돌리기 · ★집합에 Utf8 한 칸 추가) · `--all` **575/0/1** · 새 픽스처 **0**(바이트 패치).
- [rustjava-adopt-link-stringconcatfactory-p2-fix] ★★**포획 «순서»를 값으로 잠그고 «호스트 abort»를 없앤다 — 게이트② 반려 승계(PR #61).**
  ★**검수자 F1·F2 둘 다 옳았다.** F1: 픽스처 전건이 포획 1개 이하라 **순서 축이 무관측**이었고 RM4(읽기 순서 역전)가 **576 green** 이었다
  ⇒ 포획 2개 람다 둘 추가(`(String,int)`=`a:7` 글자로 · `(int,int)`=`120` ★값으로만) ⇒ **RM4 red**(`7:a`·`2001`).
  F2: 서술자가 `I` 인 콜사이트가 **호스트 프로세스를 죽였다**. ★**고친 자리 = `validation.rs` 의 «사용 지점»**(JVMS 4.4.10:
  InvokeDynamic=메서드 · Dynamic=필드) — 일반 `NameAndType` 팔의 `||` 는 **옳으므로 두었다**(Fieldref·Methodref 공유 항목).
  ★근거는 실측이다: **OpenJDK 26 도 같은 파일을 `ClassFormatError`** 로 거부한다 ⇒ 「미지원」이 아니라 「파손」이 옳은 진단.
  ★**조이기 비용 선측정**: 클래스 175 · indy/condy 44건 중 새로 위법 **1건**(이 회차 픽스처)뿐.
  ★`lower()` 의 `try_parse` 는 둘째 층이고 ★**독립 관측 불가임을 명시**했다(남긴 근거 = 비용 비대칭).
  ★개악 2종 전건 red · `--all` **578/0/1** · 픽스처 재생성 멱등(형제 #59 생성기와 합친 뒤에도 바이트 불변).
- [rustjava-adopt-link-stringconcatfactory-p2] ★★**`LambdaMetafactory.metafactory` 링크 — 람다·메서드 참조가 «돈다».**
  채택 제안 `2026-09-16-link-stringconcatfactory#p2`(worklog json `adoptedProposals` 기록). ★**제품 동작 변경 있음**
  (람다 포함 클래스: 적재 거부 → 실행). ★`jvm/` 무접촉 · `java.lang.invoke` **0줄**.
  ★★**제안의 「java.lang.invoke 가 불가피 · L」은 틀렸다** — 콜사이트가 의미하는 것은 핸들 사슬이 아니라 **객체**이고,
  그걸 만들 두 축이 **이미 있었다**(`MethodBody::Rust(JvmCallback)` · `Jvm::register_class`). ⇒ 팩토리가 스핀할 클래스를 직접 만든다.
  ★**경계 = 어댑터**(박싱·언박싱·확대): 통과가 아니면 **링크하지 않는다** — 판정이 **lowering 시점**이라 「로드되거나 안 되거나」이고
  호출 «도중» 실패 경로가 없다. `LambdaBoxing.class` 가 그 경계를 잠근다(OpenJDK 는 3을 찍는다).
  ★★**관측 가능성 3종이 «처음엔 안 죽었다»** — ⑴void 버림(인터프리터 «밖» = `Thread.run()` 의 `()` 변환에서만 보인다)
  ⑵REF_invokeSpecial(javac 11+ 는 안 낸다 ⇒ **--release 8** 픽스처 · 핀을 «픽스처별»로 바꿨다) ⑶정적 인자 개수·종류(손조립 2종).
  ★**개악 14종 전건 red** · `cargo test --all` **573 → 576 / 0 failed / 1 ignored** · `LambdaKinds` 10줄이 OpenJDK 26.0.1 과 일치 ·
  DoD 7명령 rc=0 · 픽스처 재생성 멱등.
- [rustjava-adopt-link-stringconcatfactory-p1] ★★**레시피가 콜사이트와 어긋날 때 — 제안의 「싸고 옳다」가 두 겹으로 거짓이었다.**
  채택 제안 `2026-09-16-link-stringconcatfactory#p1`(worklog json `adoptedProposals` 기록). ★**제품 동작 변경 있음.**
  ★**제안의 처방은 기각**(`classfile/validation.rs`/`ClassFormatError`) — ★**OpenJDK 26.0.1 에 픽스처 3종을 직접 돌린 실측**이 근거다:
  전건 `BootstrapMethodError`(원인 `StringConcatException`) · 프레임 `linkCallSite` = **링크 시점** · **`ClassFormatError` 아님**.
  ★★**실제로 깨져 있던 둘**: ⑴`java/lang/BootstrapMethodError` 가 **런타임에 없어서** 그 가지가 던지는 대신
  `jvm.rs:948` unwrap 에서 **패닉**했다(픽스처 0 이라 아무도 밟은 적이 없다) ⑵검사가 **부족분**만 봐서
  레시피가 **짧으면** 남는 인자를 버리고 **틀린 문자열을 반환**했다(rc=0). ⇒ **상등 검사 · 변환 전 1회 · 인자/상수 두 축**.
  ★**변환 전인 이유**: `String.valueOf` 가 **사용자 `toString()`** 을 돌리므로, 먼저 변환하면 그 예외가 진단을 덮는다.
  ★**개악 4종 전건 red**(M1 검사 제거 · M2 `!=`→`>` · M3 상수 축 제거 · ★**M4 클래스 등록 제거 → 패닉 재현**) ·
  `cargo test --all` **573 → 574 / 0 failed / 1 ignored**(기준선 워크트리 실측) · DoD 7명령 rc=0 · 픽스처 재생성 멱등.
  ★★**게이트③ 착지 — PR #60 · `--merge`**(등재 repo · `merge_strategy: merge` 선언분). 게이트② **approve** ·
  핀 `532d98d7` **불이동**(착수 실측 2026-09-17T01:18:24Z) · ★**`MERGEABLE/CLEAN` · base 뒤처짐 «0»** ⇒ 충돌 해소·base 당김 **둘 다 불요**.
  ★핀에서 `ci-presence` **rc=0 CI_GREEN** · 자식 PR **0건** · 배포 **0**(이 저장소에 배포 워크플로 없음) · 주기 자동 커밋 **0건**.
  ★**동봉은 이 기록 한 줄뿐** — 원장(worklog 쌍·`STATE`·`REPORT`)은 구현 회차가 이미 실었다.
  ★★**착지 순서 고지 — 이 회차가 «새 `.class` 3개»를 들여온다**(`RecipeWants{MoreArguments,FewerArguments,AConstant}`).
  열린 PR **#57**(ci-pending)이 「`.class` 는 `test-data/class-file-versions.txt` 에 등재돼야 하고 ★**미등재는 핀이 실패시킨다**」를 세운다
  ⇒ ★**이 PR 이 먼저 착지하면 #57 의 표에 이 셋이 «없어»** 그 회차가 red 가 된다(해소 = `python3 test-data/src/record-class-file-versions.py` 재생성).
  ★**텍스트 충돌이 0 이라 `mergeable` 로는 보이지 않는 종류다.**
- [rustjava-adopt-link-stringconcatfactory-p0] ★★**`StringConcatFactory.makeConcat` 도 링크한다 — 단 «이유는 제안이 적은 것이 아니다».**
  채택 제안 `2026-09-16-link-stringconcatfactory#p0`(worklog json `adoptedProposals` 기록).
  ★**제품 동작 변경**: `makeConcat` 콜사이트가 **거부 대신 실행**된다. ★**실행기(`concat_with_constants`)는 한 줄도 안 바뀌었다.**
  ★★**ⓒ 제안의 전제가 «거짓»이다**(실측 javac 26.0.1 · 같은 소스 `a + b`):
  `--release` **9 · 11 · 17 · 21 · 26 전부 `makeConcatWithConstants`** — ★**상수 텍스트가 «없는» 경우에도** 그렇다
  (레시피가 «자리표시자 둘 · 리터럴 0»일 뿐이다). `makeConcat` 은 **비기본 내부 플래그 `-XDstringConcat=indy`** 에서만 나오고,
  `-XDstringConcat=inline` 은 `StringBuilder` 를 낸다.
  ⇒ ★**「javac 이 상수 텍스트 없이 연결할 때 쓴다」도, 「javac 산출물이 더 많이 돈다」도 거짓**이다 —
  ★**기본 javac 산출물은 이미 «전부» 링크되고 있었다.**
  ★★**그래도 한 이유를 «바꿔서» 적는다**(제안의 근거를 그대로 베끼지 않았다):
  ⑴★**만들 수 있는 형상**이다 — 그 플래그로 **직접 만들었다**. 이 리니지가 `ldc` 태그 지원을 정당화한 기준
  (「ASM 이 실제로 낸다」)과 ★**같은 기준**이다 ⑵**문서화된 공개 진입점**이라 바이트코드 생성기가 택할 수 있다
  ⑶★★**실행기에 «새 경로»가 필요 없다** — `makeConcat(n개 인자)` ≡ 레시피가 `\u{1}` **n개**인 `makeConcatWithConstants` 다
  ⇒ 콜사이트 서술자의 인자 수로 **레시피를 합성**하면 끝. ★제안이 「a recipe-free path through the same executor」로 본 것보다 **더 싸다**.
  ★**설계 — 제안의 「짧은 명시 목록을 유지하라」를 지켰다**: 상수 튜플 **둘**뿐 · **레지스트리 아님** ·
  ★**이름과 서술자를 «쌍»으로 매칭**한다 ⇒ ★**#55 가 넣은 name 축 근접 실패(`NotMakeConcatWithConstants`)가 «살아 있다»**
  (그 파일은 이름만 `makeConcat` 이고 서술자는 `makeConcatWithConstants` 의 것이라 여전히 거부된다).
  ★**레시피 합성은 «콜사이트마다»** 한다 — 한 부트스트랩 항목을 **서술자가 다른 여러 콜사이트가 공유**할 수 있어
  해석 단계에서 고정할 수 없다(그래서 `LinkedFactory::{WithConstants, NoRecipe}` 로 «인식»만 한다).
  ★★**개악 대조 4종 — 처음엔 «둘»이 살아남았고, 그것이 이 회차의 교훈이다**:
  **M1** 튜플 제거 → KILLED · **M2** 레시피 길이 `repeat(1)` → KILLED · ★**M3** 쌍 검사를 «이름만»으로 → **SURVIVED** ·
  ★**M4** 「정적 인자 없음」 가드 제거 → **SURVIVED**.
  ⇒ ★**직전 `…-p0-fix` 가 세운 「죽었다 ≠ 그 가지가 «전부» 덮였다」가 «내 새 코드»에 그대로 적용됐다** —
  축마다 근접 실패를 더해(`MakeConcatWrongDescriptor` = 서술자만 다르고 정적 인자 0 · `MakeConcatWithArgument` = 이름·서술자 맞고 정적 인자 1)
  ★**4종 전건 KILLED** 로 만들었다.
  ★**픽스처가 «출력»한다**(`ab`) — ★**레시피를 틀린 길이로 합성해도 링크되고 실행된다.** 값을 버리면 그 오류가 «안 보인다»
  (M2 가 그 증거다) ⇒ 「링크됐다」가 아니라 **「무엇이 연결됐나」**를 단언한다.
  ★**잃는 것**: 링크 수용 범위가 넓어졌다 — 「콜사이트 «하나»를 링크한다」가 이제 **둘**이다(제안이 경고한 그대로 · 명시 목록으로 억제).
  ★`cargo test --all` **573 → 574 / 0 failed / 1 ignored** · 픽스처 재생성 **멱등** · DoD **7줄 전건 rc=0**.
  ★★**게이트③ 착지 — PR #59 · `--merge`**(등재 repo · `merge_strategy: merge` 선언분). 게이트② **approve** ·
  핀 `3b04712e` **불이동**(착수 실측 2026-09-17T01:40:32Z) · ★**`MERGEABLE/CLEAN` · base 뒤처짐 «0»** ⇒ 충돌 해소·base 당김 **둘 다 불요**.
  ★핀에서 `ci-presence` **rc=0 CI_GREEN** · 자식 PR **0건** · 배포 **0**(이 저장소에 배포 워크플로 없음) · 주기 자동 커밋 **0건**.
  ★**동봉은 이 기록 한 줄뿐** — 원장(worklog 쌍·`STATE`·`REPORT`)은 구현 회차가 이미 실었다.
  ★★**착지 순서 고지 — 이 회차가 «새 `.class` 3개»를 들여온다**(`MakeConcat`·`MakeConcatWithArgument`·`MakeConcatWrongDescriptor`).
  열린 PR **#57**(ci-pending)이 「`.class` 미등재는 핀이 **실패**시킨다」(`test-data/class-file-versions.txt`)를 세우므로,
  ★**이 PR 이 먼저 착지하면 #57 의 표에 이 셋이 «없어»** 그 회차가 red 가 된다(해소 = `record-class-file-versions.py` 재생성).
  ★**텍스트 충돌 0 이라 `mergeable` 로는 보이지 않는다** — 같은 고지가 #60 회신에도 있다(그쪽은 다른 3개).
- [rustjava-adopt-indy-fixture-jdk-pin-and-slot-accounting-p0-fix] ★★**표 키의 경로 구분자를 «두 곳»에서 정규화 — 윈도우 CI red 를 고쳤다.**
  게이트② **request-changes** 승계(PR #57 · 핀 `3061edb7` · ★`ci-presence` → **`CI_RED` rc=1**). ★**제품 코드 무접촉 · 설계 무변경**
  (검수자가 「설계는 옳고 세 방향 전부 실제로 문다」로 확인했다 — ★**그 축은 다시 열지 않았다**).
  ★★**결함의 모양**: `to_string_lossy()` 가 윈도우에서 `indy\StringConcat.class` 를 만드는데 표는 `indy/…` 를 담는다 ⇒
  ★**하위 36개가 «미기록»과 «유령 기록»에 «동시에» 걸린다.** ★**루트 114개는 통과**하므로 ★**mac/linux 로는 «절대» 안 보인다**
  — 모수 실측 **150 = 루트 114 + 하위 36**(검수자 수와 일치).
  ★★**「두 곳」이 급소다**: ★**같은 결함이 기록기(`record-class-file-versions.py`)에도 있었다** — Rust 만 고치면
  ★**윈도우에서 기록한 표가 이번엔 mac/linux 를 red** 로 만들고, ★**CI 가 기록기를 안 돌려 그 축은 «영영 조용하다».**
  ⇒ 둘을 함께 고쳤다(Rust 구분자 정규화 · Python `as_posix()`). ★표 자체는 **이미 슬래시**였다(역슬래시 행 **0** 실측).
  ★★**②는 «코드»가 아니라 «순서»였고 — 그 사이 `#55` 가 먼저 착지해 «해소된 형태»로 나타났다**:
  base 를 당기니 픽스처 3개가 ★**정확히 red 로 잡혔고**(`NotFactoryDescriptor`·`NotInvokeStaticFactory`·`NotMakeConcatWithConstants`),
  ★**고친 기록기로 재기록하니 150 → 153 · diff 가 정확히 3줄**(그 외 무변).
  ⇒ ★**기계가 설계대로 «잡고», 처방이 «한 명령»이었다는 실증이다.**
  ★**남은 형제 실측**: `#56` 은 test-data 접촉 **0** · `#58` 은 `test-data/src/verify-javac-fixtures.sh` **스크립트뿐** ⇒
  ★**둘 다 표와 상호작용하지 않는다** — 착지 순서 제약 **없음**.
  ★**③ 오기 정정**: `STATE.md` 에 착지해 있던 「**#55 는 원장 3파일만 만진다**」가 **거짓**이었고
  ★**그 한 줄 때문에 ②가 보이지 않았다** ⇒ 그 자리에 반증을 붙였다. ★**교훈: 「어느 파일을 만지나」는 «PR 파일 목록»으로 확인하라.**
  ★★**이 설계의 «본래 대가»를 이제 적어 뒀다** — 「기록에 없는 픽스처 = red」는 **의도된 엄격함**이고
  그 비용(픽스처를 더하는 회차가 표를 함께 만진다)이 **어디에도 없었다**(실측: `AGENTS.md` 관련 문장 0)
  ⇒ ★**`AGENTS.md` §Testing Boundaries 에 한 줄**로 남겼다.
  ★**개악 대조(⒞)**: 정규화 한 줄을 되돌리면 ★**mac 에서도** `table_keys_use_forward_slashes_on_every_platform` 가 **진다** · 복원 green.
  ★★**그래서 «문자열 치환»으로 구현했다** — `components()` join 이면 Unix 에서 역슬래시 입력이 **그대로 통과**해
  ★**개악이 mac 에서 안 잡힌다**(대가: Unix 파일명의 진짜 역슬래시는 바뀐다 — 픽스처엔 없고 표는 우리 것이다).
  ★Python 축은 **윈도우 없이** 실증했다(`PureWindowsPath`: `str()` 역슬래시 ↔ `as_posix()` 슬래시).
  ★★**게이트③ 착지 — PR #57 · `--merge`**(등재 repo · `merge_strategy: merge` 선언분). 게이트② **approve**(★`-fix2` 승계분) ·
  ★**핀 `888d8821` 불이동**(착수 실측 2026-09-17T06:21:43Z · `MERGEABLE/CLEAN` · base 뒤처짐 «0»).
  ★★**게이트③이 «두 번» 돌았다 — 첫 회차는 «옳게» 막혔다**: base 를 당기면 이 PR 자신의 핀 테스트가 red 였다
  (형제 #59 가 들여온 `MakeConcat*` 3개가 표에 없었다) ⇒ 머지 티켓은 코드를 고치지 않으므로 `-fix2` 로 넘겼고,
  그 회차가 base 당김 + 표 **3행**(`record-class-file-versions.py` 1회 · 추가만 · 삭제 0)을 넣어 풀었다.
  ⇒ ★**이 PR 이 세운 규율이 이 PR 에 처음 적용된 사례**이고, 검사기를 완화하지 않고 표를 고쳐 통과했다.
  ★핀에서 `ci-presence` **rc=0 CI_GREEN** · 이 형상에서 `cargo test --test test_fixture_pins` **3/3 green**(이 PR 이 만든 그 축).
  ★**동봉은 이 기록 한 줄뿐**이다 — 원장(worklog 쌍·`STATE`·`REPORT`)은 구현 회차가 이미 실었다.
  ★배포 **0**(이 저장소에 배포 워크플로 없음) · 자식 PR **0건** · 주기 자동 커밋 **0건** · 낡음 판별 도구 **0건**.
  ★★**착지 순서 고지** — 이 PR 이 세운 규율(「`.class` 를 더하면 `test-data/class-file-versions.txt` 에 같은 커밋으로 행을 넣는다 ·
  미등재 픽스처는 핀이 **실패**시킨다」)은 ★**열린 PR #60(신규 `.class` 3개)·#61(17개)에 «소급 적용»된다** —
  그쪽이 표에 행을 넣지 않고 착지하면 **main 이 red** 가 된다. 텍스트 충돌이 없어 `mergeable` 로는 보이지 않는 종류다.
- [rustjava-adopt-indy-fixture-jdk-pin-and-slot-accounting-p0] ★★**`test-data` 전체(150 클래스)의 클래스 파일 버전을 «동결»했다 — «통일»이 아니라.**
  채택 제안 `2026-09-16-indy-fixture-jdk-pin-and-slot-accounting#p0`(worklog json `adoptedProposals` 기록).
  ★★**제안의 «비용 산정»을 바꿨다 — 기각이 아니라 «설계 교체»다.** 제안은 「**decide the intended target** per fixture or
  per directory … recompiled … ★**The decision is the work**」라며 **통일**을 전제했는데, 정작 제안이 적은 이득은
  「a regenerated fixture **cannot quietly start testing a different Java version's** bytecode shapes」 = ★**«드리프트 탐지»**다.
  ⇒ ★**목표를 «동결»로 바꾸면 그 「work」가 통째로 사라진다**: 지금 값을 기록하고 혼자 움직이면 red —
  ★**재컴파일 0 · 바이트 변경 0 · 「어느 타깃이 옳은가」 결정 자체가 불요.**
  ★**대가는 정직하게**: 이 축은 ★**섞임을 «고치지» 않고 «굳힌다»**(루트에 52·65·66·68·70 이 그대로 남는다).
  그것이 옳은 이유는 ★**통일 = 재컴파일 = 바이트 변경 = 「그 픽스처가 무엇을 시험하는지」의 변경**이기 때문이고(제안 자신의 경고),
  그 판정은 **별 축(후속 L)** 으로 넘겼다.
  ★★**ⓑ 이미 핀하는 축이 «둘» 있었고, 그 둘이 못 덮는 곳이 이 회차의 대상이었다**:
  ⑴`tests/test_fixture_pins.rs` 는 **`test-data/indy` 만** ⑵**생성기 코드**가 자기 산출물 **16개**의 버전을 소스에 박는다
  (`make_ldc_fixtures.py` 의 `major=52` + 항목별 오버라이드) ⇒ ★**남는 132개 = 「`.java` 소스가 있어 손수 재컴파일 가능한 것」**이고,
  ★**루트의 66×8 · 68×1 · 70×3 이 그 사고가 «이미 일어난» 지문**이다.
  ★**만든 것**: `test-data/class-file-versions.txt`(150행 + 머리 주석에 「이것은 타깃이 아니라 동결이다」) ·
  `test-data/src/record-class-file-versions.py`(머리 주석 **보존** · 없으면 **거부** · docstring 이 「실패를 잠재우려고 돌리지 마라」를 못박는다) ·
  테스트 `committed_fixtures_keep_their_recorded_class_file_version`.
  ★★**세 방향을 «전부» 검사한다 — 이것이 급소다**: ⑴기록과 다름 ⑵★**기록에 없는 새 픽스처**
  (★없으면 핀이 «옵트인»이 되어 **내일 추가되는 픽스처는 조용히 미보호** — 이 저장소가 반복해 잡은 그 형태의 변종) ⑶**유령 기록**.
  ★**개악 3종 전건 red**: M1 `Hello.class` 버전 바이트 `65→70`(= 다른 JDK 재생성과 **같은 형상**) ·
  M2 미기록 `.class` 투입 · M3 표에만 있는 행 ⇒ 복원 **green**(`test_fixture_pins` **2 passed**).
  ★실패 문면이 **파일·두 버전·해소 명령**을 함께 말한다(`Hello.class: recorded 65.0, found 70.0`).
  ★**기록기 멱등**(재실행 시 표 **바이트 동일**) · `cargo test --all` **572 → 573 / 0 failed / 1 ignored** · DoD **7줄 전건 rc=0**.
  ★**알고 남긴 값**: 생성기 산출물 16개는 **이중 잠금**(생성기 + 표)이다 — ★**예외 목록을 두는 규칙보다 «전건 단일 규칙»이 덜 썩는다.**
- [rustjava-adopt-indy-fixture-jdk-pin-and-slot-accounting-p1] ★★**「어느 javac 이 만들었나」를 «기록»에서 «검증»으로 바꿨다.**
  채택 제안 `2026-09-16-indy-fixture-jdk-pin-and-slot-accounting#p1`(worklog json `adoptedProposals` 기록). ★**제품 코드 무접촉.**
  ★★**제안의 결론 «둘»이 실측으로 반증됐다 — 그래서 제안이 «불가능»하다고 적은 쪽을 만들었다**:
  ⑴「**Nothing offline can verify** a recorded compiler version … buys **provenance, not enforcement**」 → ★**거짓**:
  javac 은 같은 소스·플래그·컴파일러에 **결정적**이라, 기록된 도구(`javac 26.0.1 --release 21`)로 재빌드하니
  ★**`test-data/indy` 의 6개가 «바이트 단위로 동일»**했다. ⇒ ★**기록이 «재현»으로 검증된다.**
  ⑵「강제 가능한 축은 `constant_pool.rs` 의 상수 개수뿐이고 **한 픽스처만** 덮는다」 → ★**거짓**:
  ★**javac 산출 indy 픽스처 «3/3»이 형상 단언 보유** — `ConstantKinds`(MethodType 1·Dynamic 3·MethodHandle 7·InvokeDynamic 3) ·
  `StringConcat`(부트스트랩 **4축** + 인자가 «가리키는 값» `"a\u{1}"` + ★**바이트 창** `[15,6,0,35]` · 「layout changed」로 실패) ·
  `Lambda`(`LambdaMetafactory.metafactory` · 인자 3 · ★`args[0]==args[2]!=args[1]`).
  ★**제안이 든 위험(「현대 javac 은 enum switch 를 condy 로 낸다」)은 `ConstantKinds` 의 Dynamic **3** 이 이미 잠그고 있다.**
  ★**만든 것**: `test-data/src/verify-javac-fixtures.sh` — ⑴★`--release` 를 **픽스처 자신의 major − 44** 에서 읽어
  ★**외부 표(형제 PR #57 의 버전 표)에 의존하지 않는다**(동기화할 것이 없다 · 미착지 의존도 없다)
  ⑵★명시한 `JAVAC` 가 안 되면 **조용히 다른 컴파일러로 대체하지 않고 rc=2** — 「무엇이 검증했나」가 흐려지면 안 된다
  ⑶★**「못 만들었다」와 「만들었는데 다르다」를 «가른다»** — 합치면 발견을 과장한다.
  ★**CI 에 배선하지 «않았다»**: `.github/workflows/rust.yml` 에 JDK 가 없고 PATH 에도 없다 ⇒
  JDK 를 요구하는 테스트는 ★**어디서나 실패하거나 어디서나 건너뛴다.** 이건 «재생성했을 때 사람이 돌리는» 검사다(doc 주석에 명시).
  ★★**실패담 둘을 남긴다 — 이 회차가 실제로 밟았다**: ⑴`command -v javac` 이 macOS **스텁**(실행되는데 「JDK 없음」)을 고른다
  ⇒ 경로가 아니라 **실행해서** 판별한다 ⑵★**`-sourcepath` 를 넣었다가 «더 나빠졌다»** — `test-data/src` 에 **`Exception.java`**·
  `Array.java`·`Method.java` 가 있어 javac 이 `Exception` 을 ★**`java.lang.Exception` 이 아니라 그 픽스처로** 해석했다
  (멀쩡히 재현되던 파일들이 `incompatible types` 로 무너졌다) ⇒ **되돌리고 그 대가**(형제 참조 소스는 홀로 재빌드 불가)를 **따로 보고**한다.
  ★**개악 대조**: 커밋본 **마지막 1바이트 반전** → ★**✗ 감지** · 복원 → **6 reproduced** · 명시 `JAVAC` 부재 → ★**rc=2 「nothing was verified」**(통과 아님).
  ★★**일반화 — 시켜 보고 «나온 값»만 적었다(고치지 않았다)**: 루트 `sh … test-data` →
  **109 rebuilt · 104 재현 · ★5 상이 · 3 재빌드 불가**. 상이 5건 = `MonitorSemantics`(+내부 2) · `NativeMethod`(`--release 8`) · `OddEven`(`--release 21`).
  ★**원인은 단정하지 않는다** — 「다른 컴파일러」와 「빌드 뒤 소스 수정」이 **둘 다 이 관측과 맞는다**. ★범위 밖이라 후속(M)으로 넘겼다.
  ★**그래도 적는 이유**: ★**제안이 걱정한 드리프트가 «실재»한다는 첫 «직접» 증거**다(그전까지는 버전 분포에서의 추론이었다).
  ★**직전 회차가 «커밋하지 않기로» 한 개악 하네스와 다른 종류다** — 그건 **제품 소스를 치환**해 죽으면 트리를 오염시켰고,
  이건 **읽고 비교만** 한다(실패해도 트리 무변) ⇒ 그래서 **남겼다.**
  ★`cargo test --all` **572 / 0 failed / 1 ignored**(doc 주석만 바꿔 **불변**) · DoD **7줄 전건 rc=0**.
  ★★**게이트③ 착지 — PR #58 · `--merge`**(등재 repo · `merge_strategy: merge` 선언분). 게이트② **approve** ·
  핀 `38c02a2d` **불이동**(착수 실측 2026-09-17T00:27:39Z · 핀에서 `ci-presence` **rc=0 CI_GREEN**).
  ★**충돌은 원장 2파일뿐**(`REPORT.md`·`STATE.md`) — 형제 **#55** 착지분과 겹쳤고 코드 파일 충돌 **0**.
  해소는 전건 보존·합집합·**시간순**: 이 회차(`38c02a2d` 05:22)가 main 쪽 최신 항목(`30a31bda` 04:04)보다 **뒤**라 위에 얹었다.
  ★줄 소실 **0**(양방향) · 합집합 밖 신규줄 **0** · ★계약 12 착지 diff numstat **해소 전후 동일**(6파일 · 해소면 밖 변경 0).
  ★배포 **0** — 이 저장소에 배포 워크플로가 **없다**(CI 2종 + 스케줄 2종 + PR 댓글 1종) · 자식 PR **0건** · 주기 자동 커밋 **0건**.
  ★★**게이트③ 2회차 — 형제 #59 가 그 사이 착지(`66bc49e8`)해 base 를 다시 당겼다.** 충돌은 또 **원장 2파일뿐**(코드 충돌 0).
  ★**이번엔 시간순이 «뒤집혔다»** — main 쪽 항목(`3b04712e` 06:52)이 이 회차(`38c02a2d` 05:22)보다 **뒤**라 **위**에 얹었다.
  ★두 번의 base 당김을 거쳐도 이 PR 의 기여 numstat 은 **불변**(`94/0` 검증 스크립트 · `14/2` 핀 테스트 · worklog 2건).
- [rustjava-adopt-cp-tag-passthrough-detectable-p1] ★★**판정 — `ClassFileError` 에 «원인»을 실을 값은 있다. 단 제안의 이름·이유·범위가 «셋 다» 틀렸다.**
  채택 제안 `2026-09-16-cp-tag-passthrough-detectable#p1`(worklog json `adoptedProposals` 기록). 낱말이 **`Decide`** 다
  ⇒ ★**코드 변경은 «틀린 주석 한 곳» 정정뿐** · 구현은 **범위를 바로잡아 후속으로** 넘겼다.
  ★★**⑴역사가 거짓이다**: `822504b` 는 `error.rs` 를 **«자르지» 않고 «만들었다»**(`new file` · 지금과 동일한 2변형)이고,
  그 이전 `ClassInfo::parse` 는 **`Option<Self>`**(실패에 정보 **0** · `.unwrap()` 투성이)였다 ⇒ ★**그 커밋은 «개선»이었다.**
  ⇒ ★**「carry a cause **again**」·「restoring」은 성립하지 않는다 — 이 리니지에 원인이 실렸던 시기는 «없다».**
  ★★**⑵「upstream 이 해야 한다」도 거짓**: `upstream/main` 기준 **5커밋 뒤** · 그중 이 파일들을 만지는 것 **0건** ·
  upstream 의 `error.rs` 접촉은 **1건(생성)** 뿐(이 crate 에서 가장 안정된 파일) · ★**우리는 이미 이 crate 에서 크게 갈렸다**
  (`constant_pool.rs` +211/−6 · `validation.rs` +137/−0 · `attribute.rs` +129/−3 · `opcode.rs` +83/−7).
  ※`AGENTS.md` read-only 는 **upstream 으로 «보내는 것»** 금지이지 로컬 변경 금지가 아니다.
  ★★**⑶범위도 틀렸다 — `target: classfile/src/error.rs` 는 1파일인데 평탄화는 «3층»이다**:
  `InvalidFormat`(생산 9곳) → `ClassDefinitionError::InvalidClassFile`(**From 이 원인을 버린다**) →
  ★**`"Invalid class file"` 하드코딩 2곳**(`src/runtime.rs:189` · `test-utils/src/lib.rs:334`).
  ⇒ ★**`error.rs` 만 고치면 «관측 변화 0»** — 아무도 원인을 넣지 않고 아무도 읽지 않는, 이 저장소가 규탄하는 그 형태다.
  ★★**진짜 비용은 `validate_class` 의 «8항 `||` 사슬»을 쪼개는 것**이다(원인이 갈리는 유일한 자리) —
  ★그것은 이 티켓이 **명시적으로 금지한 리팩터**(계약 3)라 ★**여기서 구현하지 않았다.**
  ★★**ⓑ 그런데 설계는 «한 enum 건너» 이미 증명돼 있다** — `ClassDefinitionError::UnsupportedFeature(&'static str)` 가
  **5곳**에서 쓰이며 `"ldc of a method handle"` 같은 문장을 낸다 ⇒ ★**새 발명이 아니라 «일관성 회복»**이고 이것이 「할 값 있다」의 근거다.
  ★**이득을 과장하지 않는다**: kind-only 단언 **8곳**이 원인을 이름 부를 수 있고 참조 JVM 격차가 준다.
  ★**그러나 제안의 「픽스처보다 강한 자물쇠」는 «절반만» 참** — 원인은 «어느 검사가 울렸나», 픽스처는 «그 검사가 관측 가능한가»를 잠근다.
  ★**증거**: 직전 `-fix` 가 찾은 구멍(신원 4축 중 3축 미관측)은 **링크 축**이라 ★**원인을 실었어도 안 잡혔다** ⇒ **대체가 아니라 «더하기»다.**
  ★**지금 고친 것**: `tests/test_class_format.rs` 머리 주석 — ★**제안이 근거로 인용한 바로 그 문장**이 거짓이었다
  (「cut 822504b」·「Restoring it needs upstream variants」) ⇒ 그 자리에서 정정했다.
  ★**착지한 트리가 거짓을 나르면 다음 사람이 같은 전제로 같은 제안을 다시 만든다.**
  ★두 번째 언급(「`ClassFileError` 가 평탄화한다」)은 **참**이라 **건드리지 않았다**(과잉 편집 0).
  ★`cargo test --all` **572 / 0 failed / 1 ignored**(주석만 바꿔 **불변**) · DoD **7줄 전건 rc=0**.
  ★★**게이트③ 착지 — PR #56 · `--merge`**(등재 repo · `merge_strategy: merge` 선언분). 게이트② **approve** ·
  핀 `53ca3409` **불이동**(착수 실측 2026-09-17T00:42:56Z · 핀에서 `ci-presence` **rc=0 CI_GREEN**).
  ★**충돌은 원장 2파일뿐**(`REPORT.md`·`STATE.md`) — 형제 **#55** 착지분과 겹쳤고 **코드 파일 충돌 0**.
  ★★**`tests/test_class_format.rs` 는 «자동 병합»됐고 그것을 믿지 않고 쟀다** — 양측 델타가 둘 다 살아 있다
  (이쪽 **10/3** · main 측 **32/0** · 추가·삭제줄 다중집합 **전건 일치** · 스위트 14/14 green).
  원장 해소는 전건 보존·합집합·**시간순**: 이 회차(`53ca3409` 04:31)가 main 쪽 최신 항목(`30a31bda` 04:04)보다 **뒤**라 위에 얹었다.
  ★줄 소실 **0**(양방향) · 합집합 밖 신규줄 **0** · 계약 12 착지 diff numstat **해소 전후 동일**.
  ★배포 **0**(이 저장소에 배포 워크플로 없음) · 자식 PR **0건** · 주기 자동 커밋 **0건**.
  ★★**게이트③ 2회차 — 형제 #59(`66bc49e8`)·#58(`6e016a27`)이 그 사이 착지해 base 를 다시 당겼다.** 충돌은 또 **원장 2파일뿐**(코드 충돌 0).
  ★**시간순이 이 회차를 «맨 아래»로 보낸다**(06:52 · 05:22 > 04:31) — 「내 것이 위」가 아니라 «잰 시각»이 규칙이다.
  ★`tests/test_class_format.rs` 는 두 번 다 **자동 병합**됐고 두 번 다 양방향으로 쟀다(이쪽 **10/3 불변** · main 델타 **누락 0**).
- [rustjava-adopt-cp-tag-passthrough-detectable-p0-fix] ★★**신원 4축을 «각각» 관측 가능하게 했다 — 감사의 「고칠 것이 없다」를 정정한다.**
  게이트② **request-changes** 승계(PR #55 · 핀 `ab13a3c7`). ★**제품 코드 무접촉** — 없던 것은 **픽스처**다.
  ★★**무엇이 틀렸나**: 직전 감사의 **M7**(「신원 4축 검사 제거」)은 네 비교를 ★**한꺼번에** 지운다 ⇒ 그 red 가 증명하는 것은
  ★**「4축 중 «적어도 하나»가 관측된다」**뿐인데, 회신은 그것을 **「이 가지는 덮여 있다」**로 읽었다.
  검수자가 **축을 하나씩** 지워 재니 ★**kind·name·descriptor 는 «전 스위트 green 인 채로» 살아남았다**(class 축만 죽었다).
  ★**근인은 픽스처의 수**다 — 근접 실패가 `NotStringConcatFactory`(class 축) **하나뿐**이라 나머지 세 비교는 **어떤 파일도 관측 못 했다**.
  ★생성기 docstring 이 이미 그 문장을 적어 뒀는데(「Without such a fixture the identity check is not observable」)
  **한 축에만 이행**돼 있었고, 감사는 그것을 「저항한다」로 덮었다.
  ★★**감사 방법론에 «역»을 남겼다 — 이것이 이 회차의 진짜 산출물이다**:
  원 회신의 「죽지 **않았다** ≠ 테스트가 약하다」에 더해 ⇒ ★**「죽**었다** ≠ 그 가지가 «전부» 덮였다」.**
  ★**다축 술어(`A || B || C || D`)를 통째로 지우는 «굵은» 개악은 «가장 잘 덮인 축»이 red 를 내고 나머지 축에 대해선 아무 말도 하지 않는다**
  ⇒ ★**축이 여럿인 검사는 «축 하나씩» 찔러라.**
  ★**만든 것**: `make_indy_fixtures.py` 에 `bootstrap_kind`(기본 6 = 종전 하드코딩값 ⇒ **기존 산출물 불변**) + 픽스처 3개 —
  `NotMakeConcatWithConstants`(name · `makeConcat` 은 **실재하는** StringConcatFactory 부트스트랩) ·
  `NotFactoryDescriptor`(descriptor · 끝의 varargs 만 제거 — **여전히 적법한 서술자**) ·
  `NotInvokeStaticFactory`(kind **7 = REF_invokeSpecial** — JVMS 4.4.8 상 Methodref 와 **적법**).
  ★**셋 다 «적법한 클래스 파일»이라 신원 검사까지 도달한다**(실측: 넷 다 `UnsupportedOperationException: invokedynamic` ·
  `ClassFormatError` 아님) — 상류가 먼저 거부하면 그 픽스처는 «다른 이유»로 통과하는 것이고 그것이 이 리니지가 고치려는 형태다.
  ★**전/후**: kind·name·descriptor 단독 삭제 **SURVIVED**(`--all` 570/0/1 green) → ★**KILLED**(신규 테스트) ·
  class 축 **여전히 KILLED**(이제 **2개**를 죽인다 ⇒ 옛 커버리지 유지) · 회귀 표본 2종(BSM off-by-one · `Ldc2W` arm 제거) **여전히 KILLED**.
  ★`test_class_format` **11 → 12** · `cargo test --all --no-fail-fast` **570 → 571 / 0 failed / 1 ignored** · 픽스처 재생성 **멱등** · DoD 7줄 rc=0.
  ★★**하네스가 실제로 워킹트리를 오염시켰다** — 러너 시간 상한 **SIGKILL** 로 `finally` 복원이 안 돌아 name 축이 `false` 로 남았고,
  그 상태의 측정 2건이 **오염**됐다. ★**계약 ⒡ 의 「치환 1건 단언」이 다음 축에서 «앵커 0건»으로 즉시 잡았다** ⇒ 복원 후 **4축 전부 재측**.
  ⇒ ★**이 사건이 「소스를 치환하는 하네스를 커밋하지 않는다」는 직전 회차 결정의 «실증»이다**(죽는 순간 트리를 오염시킨다).
  ★★**게이트③ 착지 — PR #55 · `--merge`**(등재 repo). 게이트② **approve**(★이 PR 은 감사 1회차가 **request-changes** 를 받고
  이 `-fix` 회차가 승계해 통과한 것이다) · 핀 `30a31bda` **불이동**(착수 실측 20:30:58Z · 워밍 후 재조회).
  ★**충돌은 원장 2파일뿐**(측정 20:31:08Z) — 형제 **#53·#54** 착지분과 겹쳤고 `tests/test_class_format.rs` 는 **자동 병합**됐다.
  해소는 전건 보존·합집합·**시간순**: 이쪽 두 항목(`30a31bd` 04:04 · `ab13a3c` 02:28)이 main 쪽 둘(01:49 · 01:16)보다 **뒤**라 위에 얹었다.
  ★줄 소실 **0** · 부활·조작 **0**.
  ★★**계약 12 에서 «ours 축 불일치»가 떴고 뭉개지 않았다** — 실체는 ★**diff 정렬 artifact**(같은 4줄 `);`·`}`·`}`·빈 줄이
  다른 hunk 에 귀속)였고, ★**정렬 후 다중집합이 «완전히 동일»**함을 보여 양측 델타가 둘 다 살아 있음을 확인했다.
  ⇒ ★**「hunk 문자열이 다르다」를 곧바로 「델타가 빠졌다」로 읽지 마라** — 이 저장소에서 두 번째로 만난 형태다.
- [rustjava-adopt-cp-tag-passthrough-detectable-p0] ★★**변이 저항 감사 — ★그 「고칠 것이 없다」는 «한 자리에서» 거짓이었다(위 `-fix` 가 정정).**
  채택 제안 `2026-09-16-cp-tag-passthrough-detectable#p0`(worklog json `adoptedProposals` 기록). ★**코드 변경 «0»**.
  ★★**결과: `tests/test_class_format.rs` 의 단언 11개 «전건»이 자기가 이름 붙인 가지의 개악에 죽는다**
  (개악 10종 · 표는 worklog). ⇒ 「통과하지만 아무것도 재지 않는 단언」은 ★**더 없다**.
  ★★**중간 함정을 남긴다 — 이 회차에서 가장 값진 기록이다**: 1차 개악 목록(M1~M7)에서 **3개가 살아남았고**,
  그때 「약한 단언 3건 발견」이라 적었으면 **거짓 보고**였다. 실제 원인은 ★**내 목록에 그 가지가 빠진 것**이었다
  (ldc 미지원 arm · 파스 실패 매핑 · not-found 경로) — M8·M9·M10 을 더하니 전부 죽었다.
  ⇒ ★**「죽지 않았다」는 «테스트가 약하다»와 «내가 그 가지를 안 건드렸다»를 구별하지 못한다.** 변이 감사를 하는 다음 사람은 이것부터 의심하라.
  ★★**ⓒ 제안 전제의 반증**: 「several ... corrupting a **referenced slot of `Hello.class`**」는 제안이 실린 커밋
  (`eb8b4eb` = PR #49) 시점에 **이미 거짓**이다 — 그 파일의 `Hello.class` 파생 3곳은 **절단**·**매직 바이트**·**무손상 들러리**이고
  ★**참조 슬롯을 덮는 것은 0**이다(`bytes[6..8]` 버전 개악은 **실물 픽스처**에 적용된다). ⇒ 「several」은
  ★**그 회차가 방금 고친 «그 하나»의 일반화**였다. ★**그래도 감사는 값했다** — 「없다」를 «재서» 아는 것과 «추측»하는 것은 다르다.
  ★**도구를 일부러 남기지 않았다**: 개악 하네스는 제품 **소스 문자열**을 매칭해 치환하므로 리팩터 뒤 ★**조용히 그 개악을 건너뛴다**
  ⇒ ★**하네스 자체가 「통과하지만 아무것도 재지 않는」 산출물**이 된다(이 제안이 사냥하는 바로 그 형태). 대신 **개악 표를 문서로** 남겼다.
  ※데이터를 읽는 `scripts/survey-ldc-constant-tags.py` 를 남긴 판단과 갈리는 이유가 이것이다(그쪽은 소스가 아니라 클래스 파일을 읽어 낡지 않는다).
  ★**경계**: 답한 질문은 「각 테스트가 «자기 가지» 개악에 죽는가」이지 「어떤 구멍도 없다」가 아니다 ·
  **픽스처의 유일 결함성**은 별도 축이라 안 봤다(후속 추천) · 진행 중 PR **#53**·**#54** 의 새 테스트 2개는 범위 밖이다
  (각 회차가 자기 라운드에서 양방향 개악 대조를 이미 붙였다).
  ★`cargo test --all` **570 / 0 failed / 1 ignored**(27 스위트 전건 합산) · 개악 10종 적용·복원 후 워킹트리 **청결** · DoD **7줄 전건 rc=0**.
- [rustjava-adopt-bound-bootstrap-method-attr-index-p1] ★★**부트스트랩 «정적 인자» 인덱스를 경계 검사한다 — 「감지되나 판정되지 않던」 자리를 닫았다.**
  채택 제안 `2026-09-16-bound-bootstrap-method-attr-index#p1`(worklog json `adoptedProposals` 기록).
  ★**전/후**: `UnsupportedOperationException` → ★`ClassFormatError`. ★참조 JVM(OpenJDK 26.0.1) →
  **`ClassFormatError: argument_index 65535 has bad constant type`**.
  ★★**제안의 급소는 「해석으로 흐르지 마라」였다** — `attribute.rs` 가 길게 적어 둔 그 후퇴(람다 보유 클래스가
  «미지원» → «파손»으로 되돌아가는 것)를 만들지 않는 것. 지킨 방법 둘: ⑴술어가 **`contains_key` 하나**라
  ★**항목을 읽지 않는다**(페이로드도 종류도 안 본다) ⑵★**그 차이를 술어 doc 에 적었다** — 제안이 「a future reader
  may not see the difference」라고 경고한 그 독자를 위한 것이다.
  ★★**그리고 «지켰다»를 주장하지 않고 쟀다**: BSM 정적 인자를 **실제로 가진** 클래스들 —
  `Lambda`·`ConstantKinds` → ★여전히 `UnsupportedOperationException: invokedynamic`(불변) ·
  `StringConcat` → ★**여전히 실행된다**(`a0` · PR #48 링크 경로 정상). ⇒ **후퇴는 일어나지 않았다.**
  ★★**ⓑ 실측이 처방을 바꿨다** — `arguments` 를 읽는 제품 코드는 `jvm-bytecode/src/string_concat.rs:92` **한 곳**뿐이고,
  거기서는 잘못된 인덱스를 ★**«조용히 링크 포기»**로 처리한다(`string_constant(...)?`) ⇒ ★**감지는 되는데 «판정»되지 않는다.**
  그래서 처방이 「그곳 수정」이 아니라 ★**「파스 시점 거부」**다(그곳을 고치면 그 콜사이트 하나만 달라지고 파일은 계속 산다).
  ★**제안은 «절반»만 요구했고 그 절반만 했다**: JVMS 4.7.23 은 ⑴유효 인덱스 ⑵**loadable constant** 둘을 요구한다.
  ⑵는 «종류» 검사라 **다른 문장**이고 넓히면 다른 티켓이다 ⇒ 인자가 `Utf8` 를 가리키면 **여전히 통과**한다(술어 doc 에 명시 · 후속 추천).
  ★참조 JVM 의 문면(`bad constant type`)이 정확히 그 ⑵의 언어라, ★**우리가 하지 않은 절반을 스스로 가리킨다.**
  ★**픽스처**: `LdcDynamicBSMArgPastEnd.class` — 생성기 `dynamic()` 에 `static_arguments=` 를 더했다(기존 `attr_index=` 와 **같은 모양**).
  ★인덱스를 **`0xFFFF`** 로 고른 것은 ★**«부재»로만 실패하게** 하기 위해서다 — 풀 «안»의 종류 틀린 항목을 가리키면
  ⑵축과 섞여 테스트가 «이름과 다른 이유»로 통과한다. 구조 측정: 인자 `[65535]` · 풀 유효 **1..19** · 재생성 **멱등**(기존 11 전건 동일).
  ★**개악 양방향**: 호출부 제거 **red** · 술어 본문 `true`(상수 통과) **red** · 정상 **green**.
  ★`cargo test --all` **571 / 0 failed / 1 ignored**(27 스위트 **전건 합산**) · DoD **7줄 전건 rc=0**.
  ★**잃는 것 — 오탐 여지가 «한 자리» 있다**: long/double 은 슬롯을 둘 먹고 **둘째 슬롯은 이 맵에 없어** 그것을 가리키는
  인자가 거부된다. ★**맵의 우연이 아니라 의도된 읽기**다(그 슬롯에서는 어떤 상수도 적재할 수 없다 — JVMS 4.4.5) · 술어 doc 에 적었다.
  ★**범위**: `bootstrap_method_indices_resolve` **무접촉**(그것은 `bootstrap_method_attr_index` 한 문장이다) ·
  `attribute.rs` **무접촉**(`arguments` 는 여전히 **원시 인덱스** — 제안이 지키라고 한 그 설계) · 형제 `#p0` 무접촉.
  ★★**게이트③ 착지 — PR #54 · `--merge`**(등재 repo). 게이트② **1회차 approve** · 핀 `4d2820f3` **불이동**(18:04:52Z · 워밍 후 재조회).
  ★**충돌은 원장 2파일뿐**(측정 18:05:01Z) — 형제 **#53** 착지분과 겹쳤고, 해소는 전건 보존·합집합·시간순
  (`4d2820f` 01:49:47 > `f2c8317` 01:16:29) · 줄 소실 **0** · 부활·조작 **0**.
  ★★**공유 «코드» 3파일(`validation.rs`·`make_ldc_fixtures.py`·`test_class_format.rs`)은 «전부 자동 병합»됐다** —
  ★그 회차가 착수 때 삽입 위치를 일부러 갈라 둔 것(술어 호출을 `bootstrap_method_indices_resolve` «위»에 · 테스트를 파일 «앞»에 ·
  생성기 항목을 다른 칸에)이 **여기서 값을 했다**. 계약 12 로 **양방향 hunk 동일성** 확인(3파일 × 2축 전건 일치).
  ⇒ ★**게이트③ 계약 2-c⒝(코드 충돌 = blocked)에 걸리지 않았다.**
  ★**병합 결과를 «의미»로도 확인했다**: `validate_class` 사슬에 두 회차의 술어가 **둘 다** 있다
  (`…static_arguments_are_in_the_pool` + `at_most_one_bootstrap_methods_attribute`) · `cargo test --all` **572**
  (= main 570 + `#p0` 1 + 이 회차 1) ⇒ **두 회차의 테스트가 모두 살아 있다.**
- [rustjava-adopt-bound-bootstrap-method-attr-index-p0] ★★**`BootstrapMethods` 를 «두 번» 선언한 클래스를 거부한다 — 임의 선택을 없앴다.**
  채택 제안 `2026-09-16-bound-bootstrap-method-attr-index#p0`(운영자 tower 패널 채택 · worklog json `adoptedProposals` 기록).
  ★**JVMS 4.7.23 = 최대 한 개.** 종전에는 `find_map` 이 **첫 표**를 쓰고 나머지를 **조용히 무시**했다 ⇒
  ★**`bootstrap_method_attr_index` 가 «어느 표»에 대해 경계 검사되는지가 임의**였고 그 사실이 아무 데도 드러나지 않았다.
  ★**전/후**: `UnsupportedOperationException`(「아직 못 한다」) → ★`ClassFormatError`(「이 파일이 깨졌다」).
  ★★**참조 JVM 이 근거다**(observable behavior · OpenJDK 소스 미참조): OpenJDK 26.0.1 →
  **`ClassFormatError: Multiple BootstrapMethods attributes in class file`** · ★**표 하나짜리 대조군은 rc=0 로드**.
  ★**고친 자리 «한 곳»** — `validate_class` 에 술어 `at_most_one_bootstrap_methods_attribute` 를 이었다.
  ★`bootstrap_method_indices_resolve` **무접촉**(그 doc 이 스스로 「인덱스가 실재 항목을 가리키는가」라는 한 문장임을
  선언한다 — 「표가 몇 개인가」는 다른 문장이고, 접어 넣으면 **이름까지 바꿔야** 한다) · ★**새 관용 0**
  (필드 `ConstantValue`·메서드 `Code` 가 이미 쓰는 **개수 세기** 모양 그대로).
  ★★**픽스처를 «결함이 하나»가 되게 지었다** — `LdcDynamicDuplicateBSM.class` = **같은 유효한 표를 바이트 동일하게 두 번**.
  어느 한 표만 있어도 정상 파일이라 ★거부 원인이 «둘이라는 사실»로 **고정**된다(둘째를 다르게 하면 다른 규칙이 먼저 물어
  테스트가 «이름과 다른 이유»로 통과한다 — 이 저장소가 #49 에서 세운 그 규율).
  구조를 **측정**했다: 속성 `['BootstrapMethods','BootstrapMethods']` · 두 본문 **바이트 동일 True**.
  ★★**제안의 한 문장은 «과했다»** — 「생성기가 만들 수 없는 픽스처가 필요하다」는 **거짓**이고 ★**8줄 래퍼**로 됐다
  (속성 목록이 빌더에 그대로 전달된다). ⇒ **관측은 맞았고 «비용 추정»이 틀렸다** — 다음 사람이 같은 이유로 미루지 않게 적는다.
  ★**개악 대조 양방향**: ⑴호출부에서 술어 제거(= 제안 이전 상태) **red** ⑵술어 본문을 **`true`(상수 통과)** 로 **red** ·
  정상 **green**. ★**⑵가 없으면 「검사가 상수로 뭉개진」 축을 못 잡는다**(⑴만으로는 호출 삭제만 잡힌다).
  ★`cargo test --all` **571 / 0 failed / 1 ignored**(27 스위트 **전건 합산** — 꼬리만 세지 않았다) ·
  픽스처 재생성 **멱등**(기존 11 전건 바이트 동일 · 신규 1) · DoD **7줄 전건 rc=0**.
  ★**잃는 것**: 지금까지 «로드되던» 파일 하나가 거부된다 — 다만 그 형상은 어제 이 저장소가 잰 대로
  **javac·kotlinc·scalac·Lombok 산출물 5,479 클래스에 0**이고 ASM 으로도 «일부러» 만들어야 나온다.
  ★★**게이트③ 착지 — PR #53 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` — 스쿼시는 부모 2개를 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `f2c83174` **불이동**(착수 실측 17:39:54Z · ★워밍 후 재조회 `MERGEABLE/CLEAN`).
  ★**충돌 0 · base 당김 0**(`merge-tree` rc=0) — 이 브랜치가 `origin/main` 위에서 갈렸고 그 뒤 착지한 형제가 없다.
  ★**여파**: 이 착지가 형제 **#54**(BSM 정적 인자)·**#55**(변이 감사)의 base 를 낡게 만든다.
  ★**#54 는 «코드 파일이 자동 병합»되도록 그 회차가 삽입 위치를 미리 갈라 뒀고**(그 done 회신의 `merge-tree` 실측),
  **#55 는 원장 3파일만 만진다** ⇒ 두 형제 모두 충돌은 **원장 계열에 국한**된다(게이트③ 계약 2-c⒜ 범위).
  ★★**[정정 2026-09-17 · `…-jdk-pin-…-p0-fix`] 바로 윗줄의 「#55 는 원장 3파일만 만진다」는 «거짓»이었다.**
  실측: `#55` 는 `test-data/indy/` 에 **`.class` 3개를 추가**하고 생성기·테스트도 만진다 ⇒ ★**원장만이 아니다.**
  ★**그 오기가 «무해하지 않았다»** — 같은 시기 `#57`(버전 동결 표)이 「픽스처가 늘면 표도 함께」를 요구하는데,
  ★**「#55 는 원장만」이라고 읽으면 그 상호작용이 «보이지 않는다».** 실제로 그 둘은 **텍스트 충돌 0인데 나중에 착지하는 쪽이 main 을 red** 로 만들었다.
  ⇒ ★**교훈: 「어느 파일을 만지나」는 «PR 파일 목록»으로 확인하라 — 요약에서 추론하지 마라.**
- [rustjava-ldc-tags-15-16-17-real-world-generator-survey] ★★**「못 쟀다」를 «쟀다»로 바꿨다 — ASM 은 태그 15/16/17 을 «낸다».**
  채택 제안 `2026-09-16-ldc-tags-15-16-17#p2`(worklog json `adoptedProposals` 기록). ★**조사 회차 · 크레이트 무접촉**(파서·테스트 0).
  ★★**ASM 9.7.1 = 낸다(실증)** — `visitLdcInsn(Handle)`·`(Type.getMethodType)`·`(ConstantDynamic)` 15줄로 만든 클래스에서
  `{MethodHandle 1, MethodType 1, Dynamic 1}` 실측. ★**정적(공개 API `javap`)과 동적(직접 생성) «둘 다»** 잡았다.
  ★**Kotlin·Scala·Lombok 산출물 = 이 표본에서 0**(5,479 클래스 · ldc 17,819 자리):
  `kotlin-stdlib 2.0.21` 994/11,107 · `kotlinx-coroutines 1.9.0` 826/1,791 · `scala3-library 3.5.2` 583/651 ·
  `scala-library 2.13.15` 2,889/3,755 · `lombok 1.18.48 × javac 26` 산출물 2/2 · lombok 자기 jar 183/507.
  ★★**그 «0» 을 「이 상수를 안 쓴다」로 읽지 마라 — 계측기에 «풀 인구조사»를 따로 넣어 갈랐다**:
  같은 산출물 상수 풀에 MethodHandle·MethodType 이 **가득하다**(scala-library **1,604 + 723** · scala3 **310 + 143** ·
  coroutines **68 + 59**). ⇒ ★**전부 «부트스트랩 인자»이고 «`ldc` 피연산자»가 아니다** — javac 형태가 그대로 재현된다.
  ★**태그 17(condy)은 풀에도 0**(우리 합성 픽스처와 ASM 산출물만 갖는다).
  ★★**계측기를 «대조군»으로 먼저 검증했다** — `test-data/` 에서 ★**심어 둔 양성 8/8 적중**(`LdcMethodHandle`→15 ·
  `LdcMethodType`·`Ldc2WMethodType`→16 · Dynamic 5→17)이고, 「불가능 피연산자」 1건은 오차가 아니라
  ★**심어 둔 음성**(`LdcUnknownTag` = Module 을 가리키는 고의 불량) ⇒ ★**실 오차 0**(선행 회차 계측기는 **0.28%**).
  ★**비용 절감의 정체**: 툴체인을 **설치하지 않았다** — ★**「컴파일러의 stdlib 은 그 컴파일러 자신의 산출물」**을 써서
  Maven Central jar **6개(12.6MB)** 만으로 수천 클래스를 쟀다. ★**대가 = 좁음**(컴파일러당 «한 프로젝트»)이고 그것을 결론에 박았다.
  ★★**「못 쟀다」 칸을 «비우지 않고 이름 붙였다»**: ⑴kotlinc·scalac 을 **타깃 형상**으로 몰아 본 시험(미설치)
  ⑵**ASM 위에 선 도구들**(ByteBuddy·Mockito·Groovy…)이 그 API 를 실제로 부르는지.
  ★**여파**: 선행 회차가 「합성 픽스처라 javac 은 안 낸다」로 정직하게 닫아 둔 자리가 ⇒ ★**「실물 생성기가 내는 형상」**으로 승격됐다.
  형제 `…-bound-bootstrap-method-attr-index`(PR #47 착지)의 긴급도가 **사후 추인**된다.
  ★**도구는 `scripts/survey-ldc-constant-tags.py` 로 «남겼다»** — 선행 회차 계측기가 ad hoc 이라 재현 불가였던 것이
  이 티켓이 생긴 이유의 절반이다(Acceptance 의 「다음 사람이 그대로 쳐서 같은 답」).
  ★★**게이트③ 착지 — PR #52 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22`).
  게이트② **1회차 approve**(반려 0) · 핀 `fd05b9a4` **불이동**(착수 실측 15:14:25Z · 워밍 후 재조회 `CONFLICTING/DIRTY`).
  ★**순서를 적는다**(티켓 절차 2): 같은 큐의 **#51 이 먼저 착지**했고(`1fe7d75`) 그것이 이 PR 의 base 를 흔들었다 ⇒
  이 회차가 **그 뒤를 잇는다**. 착지 후 이 repo 의 **열린 PR 0**.
  ★**충돌은 `STATE.md` «하나»뿐**(측정 15:14:36Z) — `REPORT.md` 는 **자동 병합**됐고 믿지 않고 쟀다(계약 12):
  ★**양방향 hunk 동일**(`base..theirs` == `ours..merged` · `base..ours` == `theirs..merged`) · 줄 소실 **0** · 부활·조작 **0**.
  해소 = 전건 보존·합집합·**시간순**(이 회차 `fd05b9a` **20:53:17** > `#51` `b10b0fd` **20:25:15**) ⇒ REPORT 최종 순서
  **survey → methodhandlekind → indy-fixture**(실측 확인).
  ★**병합 형상에서 계측기를 다시 돌려** 대조군이 그대로임을 확인했다 — `test-data/` **148 클래스 · ldc 515 ·
  심어 둔 양성 8/8**(`{MethodHandle 1, MethodType 2, Dynamic 5}`). `cargo test --all` **570 불변** · DoD 7줄 rc=0.
- [rustjava-methodhandlekind-placement-revisit-after-pr44] ★★**`MethodHandleKind` 위치 «재결정» — 답은 「그대로 둔다」.**
  채택 제안 `2026-09-16-bootstrap-methods-and-method-handle#p2`(worklog json `adoptedProposals` 기록).
  ★**낱말이 `Re-decide` 다 — 「아니오」도 정당한 답이고, 이 회차의 답이 그것이다.** 코드 변경은
  **판단과 재개 조건을 doc 주석에 적은 것뿐**(타입·`impl`·`lib.rs` re-export **무접촉** · 공개 API 변화 **0** · 동작 변경 **0**).
  ★★**제안이 적은 배치 사유 «둘» 중 «하나»만 만료됐다** — 그 구분이 이 회차의 전부다:
  ⑵「형제 회차가 `constant_pool.rs` 를 **열어 두고** 있었다」 = ★**만료**(PR **#44** `MERGED` ·
  `2026-09-16T02:13:39Z` · 머지커밋 `dc03593` · ★**그 PR 파일 목록에 `classfile/src/constant_pool.rs` 가 실재**) ·
  ⑴「**유일한 소비자**가 `BootstrapMethods` 속성이다」 = ★**여전히 참**(전수: `MethodHandleRef::resolve` 호출부
  **`attribute.rs:116` 단 1곳**·`resolve` 는 `pub` 도 아니다 · `MethodHandleKind` 읽기는 **전건 `bootstrap.method.kind` 경유**).
  ★★**제안이 스스로 건 조건이 이미 충족됐고 그 조건이 «declined» 를 가리킨다** — 「⒝(콜사이트 링크) 착지 «후»에 정하라」의
  그 ⒝ 가 **오늘 PR #48 로 착지**했는데 ★**그 소비자마저 `bootstrap.method.kind` 로 읽는다**(`jvm-bytecode/src/string_concat.rs:81`).
  ★★**그러나 그 escape 절에 기대지 않았다 — 코드에서 «더 나은» 이유를 찾았다**: `constant_pool.rs` 는 **이미**
  CONSTANT_MethodHandle 의 해독 깊이를 정해 뒀다 — `ConstantPoolReference::MethodHandle` = ★**피연산자 0**
  (그 자리 주석: 「resolve 하려면 없는 기구가 필요하고 유일한 소비자 verifier 는 "not implemented" 로 바꾼다」).
  ⇒ ★**옮기면 «같은 상수의 두 해독»이 한 파일에 나란히 서서 «모순»으로 읽힌다** — 이동이 없애려던 오해를 **키운다**.
  ★**잃는 것을 숨기지 않는다**: 「속성 파일이 상수 풀 타입을 갖는다」는 오해는 **남는다**. 이동이 아니라 **주석**으로 갚았고,
  ★**재개 조건**을 함께 박았다 — 「이 속성 «밖»의 무언가가 method handle 을 해독하면 다시 열어라」(유력 후보 `ldc` ·
  지금은 `jvm-bytecode/src/verifier.rs:32` 가 `UnsupportedFeature` 로 답한다).
  ★**불변 증명**(이동 0이라 개악 대조가 아니라 «불변»이 그 자리를 진다): `cargo test --all` **570 / 0 failed / 1 ignored**
  = ★직전 형상 `b3a20ae` 와 **동일** · `numstat` **10+/0−**(★**삽입 의도**이므로 「치환인데 삭제행 0이면 의심하라」 **비해당** — 선언해 둔다) ·
  ★「옮기기 전 위치 참조 0」 축은 **해당 없음**(이동이 없다) · DoD **7줄 전건 rc=0**.
  ★**후속 제안 카드 0** — 「X 가 생기면 다시 열어라」를 cockpit 열린 추천으로 띄우면 **집행 불가한 카드가 영구히 남는다.**
  ★★**게이트③ 착지 — PR #51 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22`).
  게이트② **1회차 approve**(반려 0) · 핀 `b10b0fdc` **불이동**(착수 실측 12:35:43Z · ★워밍 후 재조회 `MERGEABLE/CLEAN`).
  ★★**발권 사유 ⓒ(CONFLICTING)가 «실현되지 않았다»** — 오늘 형제 셋(#48·#49·#50)이 base 를 세 번 흔들었는데도
  이 PR 은 깨끗했다. 근인은 **분기 시점**이다: 이 브랜치는 #50 착지분(`b3a20ae`) «위»에서 갈렸고,
  그 뒤 착지한 형제가 **없다**(그 사이 열린 #52 는 아직 미착지) ⇒ `merge-tree` **rc=0** · 충돌 해소 **0줄**.
  ⇒ ★**base 당김 0 · 핀 이동은 원장 동봉 1건뿐**이다.
  ★**여파(숨기지 않는다)**: 이 착지가 **PR #52**(`…-real-world-generator-survey` · 게이트② 대기)의 base 를 낡게 만든다 —
  원장 2파일이 겹친다(코드 의존 0). 해소는 그쪽 회차 몫이고 여기서 만지지 않았다.
- [rustjava-indy-fixture-jdk-pin-and-slot-accounting-test] ★★**시험 위생 둘 — indy 픽스처 JDK 핀을 «검사»로 박고, 슬롯 회계를 파서 단위에서 직접 물게 했다.**
  채택 제안 **둘**을 한 회차가 닫았다(worklog json `adoptedProposals` 전건 기록):
  `2026-09-16-cp-tags-16-17-execution-fixtures#p0` · `#p1`.
  ★★**⒜ 핀은 «기록»이 아니라 «검사»다** — 이 repo 엔 `rust-toolchain.toml` 부재 · CI `setup-java` **0건** ·
  PATH 에 `javac` 부재(실물은 `/opt/homebrew/opt/openjdk/bin/javac` **26.0.1**, PATH 밖) ⇒
  ★**컴파일러에게 묻는 검사는 원리적으로 불가능**하고, 잴 수 있는 것은 **커밋된 바이트**뿐이다.
  `tests/test_fixture_pins.rs` 가 `test-data/indy` 의 javac 산출물에 **65.0**(=`--release 21`)을 요구한다.
  ★**핀 값과 강제 지점이 같은 파일**이라 「기록했는데 검사가 다른 값을 본다」가 성립하지 않는다.
  ★★**핀 대상을 «`.java` 짝이 있는 것»으로 구조적으로 골랐다** — 형제 **PR #48** 이 같은 디렉터리에
  **합성 픽스처**(`NotStringConcatFactory.class` · major **52**)를 넣는다 ⇒ ★디렉터리 전수 핀이었으면
  **#48 착지 순간 red** 였다(그 파일을 실제로 받아 버전을 재서 확인했다). 새 javac 픽스처는 `.java` 를 넣는 순간 자동 편입된다.
  ★**양방향**: 핀 값 오기 → red · ★**실제 사고 재현**(`javac` 26.0.1 을 `--release` 없이 → major **70**) → red ·
  ★**개악**(검사를 상수 통과로) → 그 major 70 파일이 **green** ⇒ red 의 출처가 검사임이 선다.
  ※재현 실험 픽스처는 `git checkout` 원복(status 0건) — ★**커밋된 픽스처 재생성 0.**
  ★★**⒝ 판정은 「예」인데, 제안의 전제는 «절반만» 참이었다** — 「직접 시험 0」은 거짓이다
  (`long_must_fit_in_two_constant_pool_slots` 실재). ⇒ 추가하기 전에 **개악으로 «무엇이 안 잡히나»를 쟀다**:
  ★**Double 을 1칸으로 바꾸면 red 는 `test_class` 단 1건**(전 JVM · 40초)이고,
  「long·double **만**」 축(Integer 를 2칸으로)은 **실물 클래스 파일을 통째로 읽는 시험**에서만 잡혔다.
  ⇒ `only_long_and_double_consume_two_constant_pool_slots` 를 넣어 개악 **3종 전건**을 **0.01초 파서 시험**이 잡게 했다.
  ★공개 API `parse_all` 의 **결과 인덱스**(1·2·4)를 보므로 구현 세부에 결합하지 않는다.
  ★★**계측 함정을 남긴다 — `cargo test` 는 «첫 실패 바이너리에서 멈춘다».** `--no-fail-fast` 없이 센 첫 측정은
  「전건 `test_class` 1건만 red」라는 **과소계상**이었다. 개악 대조를 세는 회차는 그 플래그를 반드시 붙여라.
  ★**범위**: `parse_all` 구현 무접촉 · 픽스처 재생성 0 · 형제 셋 무접촉(★시험을 `tests/test_class_format.rs` 꼬리가 아니라
  **새 파일**에 두어 #48·#49 와 충돌 0 — 그 둘이 그 파일 꼬리를 만진다).
  ★**남긴 것**: 핀은 **목표 버전**을 고정하지 **컴파일러 바이너리**를 고정하지 않는다(javac 21·26 둘 다 65.0) ·
  핀 범위는 `test-data/indy` 뿐이고 **루트는 65×61·52×40·66×8·70×3·68×1 로 다섯 버전이 섞여 있다** — 둘 다 후속 추천.
  ★★**게이트③ 착지 — PR #50 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22`).
  게이트② **1회차 approve**(반려 0) · 핀 `eb4d296b` **불이동**(착수 실측 10:39Z · 워밍 후 재조회).
  ★**충돌은 원장 2파일뿐**(측정 10:39:37Z) — 그 사이 형제 **둘**(#49·#48)이 착지했는데도 그렇다.
  ★★**코드 파일은 «양측 교집합이 0» 이었다** — 이 회차가 시험을 `tests/test_class_format.rs` 꼬리가 아니라
  **새 파일**(`tests/test_fixture_pins.rs`)에 둔 **설계 판단이 실제로 값을 했다**(#48·#49 는 둘 다 그 파일을 만졌다).
  ⇒ 계약 12(자동 병합 코드 파일 hunk 대조)는 ★**대상 자체가 없었다.**
  해소 = 전건 보존·합집합·시간순(이 회차 17:47:55 > #49 16:20 > #48 15:46) · 소실 3줄은 전건
  **상대가 base 대비 지운 줄**(#48·#49 가 각자 닫은 구멍의 옛 문장) · 부활·조작 **0**.
  ★★★**그리고 «설계 주장»이 실물로 검증됐다** — #48 이 같은 디렉터리에 넣은 합성 픽스처
  **`test-data/indy/NotStringConcatFactory.class`(major 52)** 가 병합 형상에 실제로 들어왔고,
  ★**`.java` 짝이 없어 핀 대상에서 자동 제외**되어 `indy_javac_fixtures_keep_the_pinned_class_file_version` **green**
  (나머지 6개는 전부 `.java` 짝 보유 · major 65 = 핀 대상). ⇒ ★**「디렉터리 전수 핀이었으면 #48 착지 순간 red」가
  가정이 아니라 «지금 이 트리에서» 확인된 사실이다.**
  ★`cargo test --all` **570 / 0 failed / 1 ignored** · DoD **7줄 전건 rc=0** · `--delete-branch` 미사용.
- [rustjava-cp-tag-switch-passthrough-mutation-detectable] ★★**「알 수 없는 태그를 거부한다」는 테스트가 그것을 «지키지 않았다» — 지키게 했다.**
  채택 제안 `2026-09-16-ldc-tags-15-16-17#p1`(worklog json `adoptedProposals` 기록).
  ★**제품 코드 변경 «0»** — 개악은 실증용 임시이고 전부 되돌렸다(`git status classfile/ jvm-bytecode/` **0건**으로 확인).
  ★★**대전제를 «먼저» 실증했다**(티켓 ⓐ): 태그 switch 의 `_ => Err(...)` 를 `_ => Ok(Integer(0))` 로 개악하니
  ★**그 테스트는 «ok»** 였고 스위트에서 무는 것은 `constant_pool::tests::tags_outside_the_accepted_set_are_still_rejected`
  **단 1건**이었다 ⇒ ★**end-to-end 층에는 그 가지를 무는 것이 «없었다»**(직전 회차가 「M5 층 어긋남」으로 남긴 그것).
  ★★**근인 — 판별 실험으로 좁혔다(추측 아님)**: 옛 테스트는 `Hello.class` **상수풀 1번**의 태그를 덮는데
  그 슬롯은 ★**코드가 참조하는 Methodref** 라 덮는 순간 파일이 **여러 경로로 동시에** 깨진다. 그리고 `ClassFileError` 가
  모든 파싱 실패를 ★**「Invalid class file」로 평탄화**하므로(그 테스트 파일이 스스로 적어 둔 사실) 단언이
  ★**「태그가 미지라 거부」와 「클래스가 무너져 거부」를 구별하지 못한다.**
  ★**바이트 어긋남(desync)은 근인이 «아니다»** — 4바이트를 정확히 소비하는 개악(B)으로도 **여전히 green** 이었다.
  ⇒ ★**두 가설 중 하나를 실험으로 기각했다.**
  ★★**그러므로 처방이 «단언 조이기»가 아니다** — 문면이 평탄해 조일 것이 없다. 티켓 계약 1 이 예측한 대로
  ★**입력이 그 가지에 «유일한 결함»으로 도달하게** 만들었다: `test-data/cp/UnreferencedTag{13,14,19}.class`
  (생성기 `test-data/src/cp/make_cp_fixtures.py` 신규) = ★**참조되지 않고 · 페이로드 0 · 상수풀 «맨 끝»** 인 엔트리 하나.
  ★**세 성질이 전부 값한다** — 맨 끝 + 페이로드 0 이라야 pass-through 개악이 ★**«정상 동작하는» 클래스**를 만들고,
  그래야 테스트가 red 가 된다(그렇지 않으면 «다르게 깨진» 파일이 되어 또 green 이다).
  ★★**전/후 — 같은 개악, 다른 결과**: **전** = 그 테스트 **ok** / **후** = ★**red**
  (실패 문면 `a tag that cannot appear in a class file must be rejected: ""` — ★빈 출력 = 클래스가 «성공적으로 실행»됐다).
  ★**⒞ 다른 가지 개악**(태그 16 거부) → **6 테스트 red** ⇒ 스위트가 여전히 switch 전체를 지킨다(이 회차가 좁히지 않았다).
  ★`cargo test --all` **568 / 0 failed / 1 ignored**(★수 불변 — 테스트 1개 «치환») · DoD **7줄 전건 rc=0** · 픽스처 재생성 **멱등**.
  ★**계약 2⒜ 전수 확인**: 옛 픽스처(`BadTag*`)를 쓰던 다른 테스트 **0건** · `hello_class()`·`fixture()` 헬퍼는 여전히 **4·5회** 쓰여 고아 0.
  ★**계약 2⒝ 오탐**: 새 단언은 ★**오탐이 늘지 않는다** — 픽스처가 «유일한 결함»만 갖도록 지어져 있어 다른 변경이 이 테스트를 흔들 경로가 좁다.
  ★★**게이트③ 착지 — PR #49 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` — 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `eb8b4eb6` **불이동**(착수 실측 09:03Z — 로컬·원격·PR head 일치).
  ★★**그러나 «충돌 해소»가 이 회차의 본체였다** — 검수 «도중» #47 이 착지해 PR 이 `CONFLICTING/DIRTY` 가 됐다.
  ★**충돌은 원장 2파일뿐**(측정 09:03:56Z · `REPORT.md`·`STATE.md` 의 «맨 위 새 항목») — 선행 PLAN 의 예측
  「코드 충돌은 없다 — 파일이 갈린다」가 **맞았다**. 해소 = 전건 보존·합집합·시간순(`eb8b4eb` 16:20 > `3b3667d` 14:45).
  ★★**`tests/test_class_format.rs` 는 «자동 병합»됐고 그것을 믿지 않고 쟀다**(계약 12): 양방향 hunk 동일성 —
  `base..theirs` 델타 == `ours..merged` 델타 · `base..ours` == `theirs..merged` **둘 다 일치**.
  ★★**그리고 «줄 소실 3건»을 발견해 전건 해명했다** — 둘 다 **상대가 base 대비 «지운» 줄**이다(`deleted_by_other=True`):
  ⑴우리가 남긴 「`bootstrap_method_attr_index` 는 여전히 경계 검사되지 않는다」를 ★**#47 이 그것을 구현하며 고쳐 썼다**
  ⑵#47 이 남긴 「M5 층 어긋남 — pass-through 개악을 `test_class_format` 이 못 잡는다」를 ★**이 회차가 닫으며 고쳐 썼다**.
  ⇒ ★**소실이 아니라 «각자 자기가 닫은 구멍을 갱신»한 것**이고, 부활·조작 줄은 **0**이다.
  ★**해소 외 변경 0** · `--delete-branch` 미사용 · 형제 PR **#48·#50** 은 만지지 않았다(각자 base 당김이 필요하다).
- [rustjava-link-stringconcatfactory-makeconcatwithconstants] ★★**javac 의 문자열 `+` 가 «실제로 돈다» — ④-1 의 ⒝ 를 «한 칸만» 닫았다.**
  채택 제안 `2026-09-16-bootstrap-methods-and-method-handle#p0`(worklog json `adoptedProposals` 기록 · 이 배치의 유일한 **L**).
  ★**PLAN 선회신 게이트를 탔다** — `reports/<id>.plan.md` 를 먼저 내고 같은 회차에서 착수했다(헌장: PLAN 은 통지 후 즉시 착수).
  ★★**전/후를 «실행»으로 갈랐다**: 전 = `UnsupportedOperationException: … invokedynamic` — ★그것도 `defineClass` 프레임에서 났다
  (= **실행에 도달조차 못 했다**) / 후 = ★**`a0` 출력 후 정상 종료**(`test-data/StringConcat.txt` 와 **바이트 대조**).
  ★★**이 티켓의 L 은 문자열 접합이 아니라 «어디서 잇는가»였다 — 그 사실을 여기 박는다.**
  `BootstrapMethods` 는 **클래스** 속성인데 `Interpreter::run(jvm, code_attribute, args, return_type)` 은
  ★**메서드의 `Code` 만** 받는다 ⇒ ★**실행 시점에 부트스트랩 테이블에 닿을 길이 «없다».**
  ⇒ 둘을 «다» 쥔 유일한 자리 **`ClassDefinitionImpl::from_classfile`** 에서 **정의 시점에 내려쓴다**
  (`Opcode::Invokedynamic` → ★`Opcode::InvokedynamicStringConcat`). ★**`Interpreter::run` 시그니처 불변**(호출부 0곳 변경).
  ★**순서가 설계다** — 내려쓰기를 **verifier «앞»**에 둔다 ⇒ ★**verifier 를 한 줄도 고치지 않았다**:
  「그때까지 `Invokedynamic` 으로 남아 있는 것 = 우리가 링크하지 않는 부트스트랩」이 그대로 거부 규칙이 된다.
  ★★**`java.lang.invoke` 를 «세우지 않았다»** — `MethodHandle`·`CallSite` **0줄**(그 디렉터리는 여전히 부재).
  팩토리의 계약이 «문자열 템플릿»이라 레시피를 직접 걸으면 되고, ★**그것이 「한 호출 지점」과 「링크 기구」를 가르는 선**이다.
  값→문자열은 런타임의 `String.valueOf` 를 `jvm.invoke_static` 으로 부른다(null·toString·부동소수 서식이 한 자리에 남는다).
  ★★★**「전부 열어 버린」 변경과 구별되는 축을 «만들어» 넣었다 — 이것이 이 회차에서 가장 값진 실측이다.**
  인식은 **kind·class·name·descriptor 4축 완전일치**인데, ★**그 검사를 지워도 기존 픽스처가 «전부 green» 이었다**:
  `Lambda`·`ConstantKinds` 는 정적 인자가 String 이 아니라 ★**신원 검사에 도달하기 «전»에 다른 이유로 걸린다.**
  ⇒ ★**신원 검사가 아무 테스트에도 물려 있지 않았다**(= 지워도 아무도 모른다). 그래서 ★**근접 오답 픽스처**를 새로 만들었다:
  **`NotStringConcatFactory`**(`test-data/src/indy/make_indy_fixtures.py` · ★**소유 클래스 한 축만** 다르고 kind·이름·서술자·String 정적 인자는 전부 일치)
  ⇒ ★**오직 신원 검사만이 그것을 거부할 수 있다.**
  ★**개악 2종(양방향)**: ⑴**링크 끊기** → `test_class`(출력 대조) + `test_only_the_string_concat_bootstrap_is_linked` **red**
  ⑵**무차별 링크**(4축 제거) → ★**근접 오답이 링크돼 red** — ★**그 픽스처를 넣기 «전»에는 이 개악이 green 이었다.**
  ★`cargo test --all` **568 / 0 failed / 1 ignored**(★수 불변 — 테스트 1개 «치환» + 픽스처 1쌍 추가) · DoD **7줄 전건 rc=0** ·
  ★`make_indy_fixtures.py` 재생성 **멱등**(기존 indy 픽스처 변경 0).
  ★★**잃는 것 — 위험의 «종류»가 바뀌었다**: 종전 = 「안 돈다」(거부) → 신규 = ★**「잘못 돌 수 있다」.** ★후자가 더 나쁘다 ⇒
  그래서 판정을 「거부되지 않는다」가 아니라 ★**출력값 대조**로 잡았다(`test_class` 의 `.txt` 규약).
  ★**범위 압력을 선으로 막았다** — `makeConcat`·`LambdaMetafactory`·condy 는 **무접촉**이고 후속 추천으로 넘겼다.
  ★★**게이트③ 착지 — PR #48 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` — 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `0113b0a6` **불이동**(착수 실측 10:12Z).
  ★★**착지 전에 «형제 둘»이 먼저 들어와 있었다** — `#47`(14:45) · `#49`(16:20)가 이미 main 이라 이 PR 은 `CONFLICTING/DIRTY` 였다.
  ★**충돌은 원장 2파일뿐**(측정 10:12:58Z) — 선행 PLAN 의 「코드 충돌은 없다 — 파일이 갈린다」가 ★**형제가 «둘»로 늘어난 뒤에도 유지됐다.**
  ★★**해소는 «시간순 끼워넣기»다 — 한쪽을 통째로 얹는 것이 아니었다**: main 이 이미 [#49, #47] 순으로 갖고 있어
  이 회차(15:46)를 ★**그 «사이»에** 넣었다(`theirs[:i] + ours + theirs[i:]`). ⇒ 최종 순서 **#49 → #48 → #47**.
  ★★**`tests/test_class_format.rs` 는 자동 병합됐고, 그것을 믿지 않고 «줄 단위»로 쟀다**(계약 12):
  비어있지 않은 줄 기준 소실 **35건이 전건 «상대가 base 대비 지운 줄»**(각자 자기가 닫은 구멍의 옛 문장) · 부활·조작 **0**.
  ⇒ ★#49 가 태그 테스트를 다시 지은 것과 이 회차가 StringConcat 주석을 고쳐 쓴 것이 **둘 다 살아 있다**.
  ★**해소 외 변경 0** · `--delete-branch` 미사용 · 형제 **#50**(게이트② 대기)은 만지지 않았다 — 그쪽은 자기 base 를 당긴다.
- [rustjava-bound-bootstrap-method-attr-index] ★★**`bootstrap_method_attr_index` 가 «실재하는» 부트스트랩 메서드를 가리키게 했다 — 「파손」을 되찾았다.**
  채택 제안 **둘**을 한 회차가 닫았다(worklog json `adoptedProposals` 에 **전건** 기록):
  `2026-09-16-bootstrap-methods-and-method-handle#p1` · `2026-09-16-ldc-tags-15-16-17#p0` —
  ★**서로 다른 회차가 «독립으로» 같은 결함에 닿았다**(그래서 총괄이 둘을 합쳐 발권했다).
  ★**전/후**: `UnsupportedOperationException`(「이 런타임이 아직 못 한다」) → ★`ClassFormatError`(「이 파일이 깨졌다」).
  ★**JVMS 근거**: **4.4.10**(Dynamic/InvokeDynamic 의 `bootstrap_method_attr_index` 는 `BootstrapMethods` 의
  `bootstrap_methods` 배열에 대한 «유효한 인덱스»여야 한다) · **4.7.23**(그 상수를 가진 클래스는 그 속성을 «가져야» 한다).
  ★**참조 JVM**: OpenJDK 26 은 부재 형상에 `ClassFormatError: Missing BootstrapMethods attribute` 를 낸다.
  ★★**두 축이 «한 술어»다 — 분기를 둘로 만들지 않았다**(티켓 계약 1): 속성 부재 = «항목 0개짜리 표»라
  어떤 인덱스도 못 가리킨다 ⇒ `bootstrap_method_count.is_some_and(|count| (index as usize) < count)` 한 줄이 둘을 다 문다.
  ★**자리 = `validate_class` 의 `bootstrap_method_indices_resolve`** — `validate_constant_pool` 이 아닌 이유는
  ★**풀과 «클래스 속성»을 둘 다 쥔 유일한 자리**이기 때문이고, 그 교차가 이 검사가 여태 없던 이유였다
  (그 자리의 낡은 주석이 스스로 그렇게 적고 「원하는 회차에 맡긴다」고 했다 — ★**이 회차가 그 회차다**).
  ★**새 에러 타입 0** — 기존 `ClassFileError::InvalidFormat` 에 접었다(계약 2⒞: 늘리는 대신 접을 수 있으면 그쪽이 낫다).
  ★**픽스처 +1**(`LdcDynamicBSMIndexPastEnd` = 1항목 표에 인덱스 1) — ★**생성기를 통해서만 만들 수 있다**(표를 쓰는 자리가 거기뿐).
  ★**재생성 멱등 확인**: 기존 10개 **바이트 불변** · 신규 1개만 추가.
  ★★**개악 3종 — `false` 하나로는 ⒝⒟ 가 «겹친다»**(그래서 셋을 돌렸다):
  상수 `true` → 새 테스트 red(**⒜⒞**) · 상수 `false` → **24 테스트** red(⒝⒟ 혼재) ·
  ★**내부 술어만 `false`** → **8 테스트** red(전건 dynamic 보유 클래스 = **⒝**)이고 `test_hello`·`test_switch`·
  `test_odd_even`·`test_superclass` 는 **green**(= **⒟** 무영향) ⇒ ★**두 축이 실제로 갈렸다.**
  ★`cargo test --all` **568 / 0 failed / 1 ignored**(★**수 불변** — 테스트를 «치환»했다. 수로는 안 보이므로 개악으로 물었다) · DoD **7줄 전건 rc=0**.
  ★★**남긴 것**: `BootstrapMethods` **중복 선언**은 여전히 거부하지 않는다(JVMS 4.7.23 은 «최대 1개» · 지금은 `find_map` 이 첫 것만 본다) —
  ★**이 회차 범위 밖이고 후속 추천에 적었다.**
  ★★**게이트③ 착지 — PR #47 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `3b3667d6` **불이동**(동봉 전 실측 — 로컬·원격·PR head·리뷰 줄2 **4값 일치**) ·
  `ci-presence` **rc=0 CI_GREEN** · `mergeable` **MERGEABLE/CLEAN** · 자식 PR **0건** ·
  ★**배포 워크플로 0개 ⇒ 배포 0**(착지 diff 8파일 · `.github/workflows/` 6개 전건 deploy 어휘 0건).
  ★★**묶지 «못한» 이유가 이 리니지의 산물이다** — `rustjava` 는 upstream 동기 등재라 묶음 경로에 `merge_strategy:` 를
  담을 파일이 없고, 그러면 `bin/queue-lint` 검사22 와 집행 STOP **두 방어선이 «둘 다» 사라진다**
  (`orch-upstream-sync-repos-cannot-bundle-gate3-ever`). ⇒ 별 `-merge` 티켓이 그 «선언을 담을 파일»이다.
  ★★**형제 «둘»이 열려 있다 — 이 착지가 그 둘을 깬다**: **#48**(`StringConcatFactory` 링크 · 게이트② 대기) ·
  **#49**(상수풀 태그 pass-through 개악 탐지). ★**셋 다 `tests/test_class_format.rs` + 원장 2파일을 만진다**
  (코드 파일은 갈린다 ⇒ **기능 의존 0**). ⇒ ★**그 둘은 각자 base 당기기가 필요하다** — 해소는 그쪽 회차 몫이고 여기서 만지지 않았다.
- [rustjava-invokedynamic-bootstrapmethods-and-methodhandle] ★★**`BootstrapMethods` 를 «구조»로 읽는다 — ④-1 의 ⒜ 를 닫았다. ★콜사이트 링크 0줄.**
  채택 제안 `2026-09-16-cp-tags-15-18-parse#p0`(worklog json `adoptedProposals` 에 기록).
  ★**`AttributeInfo::BootstrapMethods(Vec<u8>)` → `Vec<BootstrapMethod>`** · 신규 공개 타입 3종
  (`BootstrapMethod` · `MethodHandleRef` · `MethodHandleKind` 9종) — ★**전부 `classfile/src/attribute.rs` 한 파일**.
  ★★**「MethodHandle 결정」의 «경계»를 이름으로 적었다 — ★「전부 된다」가 아니다**(④-1 에 7항목으로 박았다).
  요지: 되는 것은 **클래스파일에 적힌 (종류·클래스·이름·서술자) 해독**뿐이고, ★**`java.lang.invoke` 런타임 클래스는 «0개»다**
  (실측: `rustjava-runtime/src/classes/java/` 에 `invoke` 디렉터리 **부재** · `ledger-grep` 참조 **0건**) ⇒ **`MethodHandle` 객체 생성 불가**.
  ★★**이 회차의 급소 = «정적 인자를 풀지 않는다»** — ★그리고 그것을 «측정»했다(M3).
  「인덱스를 `ConstantPoolReference` 로 풀어라」는 자연스러운 다음 줄인데, `LambdaMetafactory.metafactory` 의 인자가
  **MethodType·MethodHandle·MethodType** 이라 ★**람다가 든 클래스가 «파싱»에서 죽는다** ⇒
  `java.lang.UnsupportedOperationException: … invokedynamic` → ★**`java.lang.ClassFormatError: Invalid class file`**
  (= 직전 두 회차가 만든 「파손 ↔ 미지원」 구분의 **소실**). ★**M3 에서 `StringConcat` 은 «그대로 green»** 이다 —
  ⇒ ★**인자가 String 하나뿐인 픽스처만으로는 이 회귀가 «안 보인다».** 그래서 픽스처 `test-data/indy/Lambda.class` 를 새로 넣었다.
  ★**개악 4종 전건 red · 복원 green**: M1 `num_bootstrap_arguments` 를 u8 로(필드 폭 1개) · M2 `bootstrap_method_ref` 인덱스 +1 ·
  ★M3 정적 인자 해석 강제(위) · M4 verifier 분기 제거 → ★`panicked at jvm-bytecode/src/interpreter.rs:631`(**호스트 abort**).
  ⇒ ★**`todo!()` 는 «여전히 도달 불가»이고 그것을 «측정»했다**(추론 아님) · ★`verifier.rs`·`interpreter.rs` **무접촉**.
  ★**단언마다 무는 축을 붙였다**: 구조 = 필드 전수 단언(카운트만 재면 M1·M2 를 통과한다) ·
  인자 인덱스 = `Lambda` 두 층(classfile 파스 + end-to-end 문장) · 종류 집합 = 0·10·255 red ·
  ★**짝짓기는 «내가 아니라 `validation.rs` 가 진다»는 주장까지 테스트로 잠갔다**(kind 1·4·9 red ↔ kind 5 green — 공허하지 않음).
  ★**낡은 주석 전수 정정**(`ledger-grep -rn 'BootstrapMethods'` 로 세어 닫았다): `constant_pool.rs`(「still kept as raw bytes」) ·
  `validation.rs`(「unparsed byte blob 이라 경계 지을 것이 없다」 → 이제 **있는데 이 함수가 못 본다**로 사유 교체). ★사료 구절(완료 절·REPORT 후속 추천)은 그대로 뒀다.
  ★`cargo test --all` **558 → 562 / 0 failed / 1 ignored**(신규 4 · ★감소 0) · DoD **7줄 전건 rc=0**.
  ★★**게이트③ 착지 — PR #45 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `992dfa53` **불이동**(동봉 전 실측 — PR head 와 바이트 일치) ·
  `ci-presence` **rc=0 CI_GREEN**(6셀 전건 pass) · 자식 PR **0건**(head 브랜치 `feat/rustjava-bootstrap-methods` 기준) ·
  ★**배포 워크플로 0개 ⇒ 배포 0**(착지 diff 13파일을 `origin/main...HEAD` 로 냈고, `.github/workflows/` 6개 중
  deploy·publish·release·wrangler 어휘 **0건** — coverage/CI/schedule/dependabot 뿐이다).
  ★★**착지 순서 — 형제 PR 이 «둘» 남는다**: **#44**(`ldc` 태그 15·16·17 · `-fix` 승계 얹힘) · **#46**(태그 16·17 실행 픽스처).
  ★**코드 충돌은 없을 것이다**(#44-fix 는 `validation.rs` 본문 · #46 은 `constant_pool.rs` 단위 테스트 · 이 회차는 `attribute.rs`) —
  ★**그러나 원장 2파일(`STATE.md`·`REPORT.md`) 최상단은 이 착지 뒤에 겹친다** ⇒ ★**그 둘이 합집합 해소를 진다.**
  ※`validation.rs` 는 이 회차(주석 1블록)와 #44-fix(`validate_class` 본문+새 함수)가 **다른 헌크**라 자동 병합될 것으로 보이나, 해소 주체는 뒤에 착지하는 쪽이다.
- [rustjava-ldc-tags-15-16-17-still-malformed] ★★**`ldc` 태그 15·16·17 도 «미지원»이라고 말한다 — ★④-2 를 닫았다.**
  ★**전/후 실행 출력**: `ClassFormatError: Invalid class file` → ★`UnsupportedOperationException: Unsupported class file feature: ldc of a method handle`(/`method type`/`dynamically-computed constant`).
  ★**직전 회차와 «같은 관용»을 썼다**(파싱 → verifier 거부) — 새 관용을 만들지 않았다(티켓 ①).
  ★★**⓪ 판정이 «둘»로 갈렸다 — 이 회차의 핵심이다**: ⒜**재현됐다**(4종 전건 `Malformed`)
  ⒝★**javac 은 그 형태를 «내지 않는다» — 「재현 불가」가 아니라 «측정된 부재»다.**
  JDK 자체 jmods **27,902 클래스 · `ldc`/`ldc_w`/`ldc2_w` 1,252,714 자리 · ★0건**
  (★**계측기를 대조군으로 검증**: 같은 corpus 에서 String 1,176,232 · Integer 23,292 · Long 20,006 … 를 되찾았고 디코드 드리프트 **0.28%**) ·
  `--release 21/25/26+preview` 로 문자열연결·람다·메서드참조·레코드·sealed/enum/pattern switch 등 **14종** → 태그 15·16 은
  ★**부트스트랩 «인자»로만** 등장하고 태그 17 은 **0** · 이 repo `test-data/` 127클래스 500자리 **0건**.
  ⇒ ★★**픽스처는 «합성»이고 3곳(생성기·테스트·worklog)에 그렇게 적었다.** ★**「평범한 코드에서 나온다」로 적지 않았다.**
  ★**못 잰 축도 적는다**: ASM·Kotlin 류 서드파티 jar corpus 는 이 머신에 **0개**(`$HOME`·`.m2`·`.gradle`·`.ivy2`·Cellar 전수) ⇒ **미측정**.
  ★**그럼에도 고친 정당화는 «도달성»이 아니라 «인접성»이다** — 그 셋이 ④-1(`invokedynamic` 실행)이 필요로 할 **바로 그 상수들**이다.
  ★★**참조 JVM 이 근거다**(`AGENTS.md` 허용 축 = observable behavior · OpenJDK 소스 **미참조**):
  OpenJDK 26.0.1 이 양성 픽스처 **4종을 전부 로드·실행**한다 ⇒ ★**「못 읽는 파일」이 아니라 「못 하는 파일」**임이 선다.
  ★★**그 참조 JVM 이 내 픽스처를 «두 번» 반려했고 그것이 부수 산출물이다** — 태그 17 은 **major ≥ 55** 필요 ·
  `Dynamic` 은 **`BootstrapMethods` 속성 필수**(JVMS 4.7.23). ★**우리 `validation.rs` 는 «둘 다» 검사하지 않는다**(④-2 에 남겼다).
  ★**대가(②) 미지불을 «음성 대조군»으로 보였다**: `ldc2_w` 에 MethodType(JVMS 6.5 위반) · `ldc` 대상이 태그 19
  ⇒ ★**둘 다 여전히 `ClassFormatError`**(참조 JVM 도 각각 VerifyError·ClassFormatError). ※우리는 파싱 시점에 끊어 phylum 이 다르다 — 단언하지 않았다.
  ★개악 **5종**: M1 verifier 새 분기 제거 → ★`panicked at jvm-bytecode/src/interpreter.rs:1063`(**호스트 abort** = 직전 회차가 `todo!()` 에서 잰 것과 같은 형태) ·
  M2 `from_constant_pool` 원복 · M3 opcode 분기 원복 · M4 `ldc2_w` 확장 → **전건 red** · 복원 green.
  ★★**M5 는 «내 테스트가 못 잡았다» — 숨기지 않는다**: 상수풀 태그 switch 를 pass-through 로 만들어도
  `tests/test_class_format.rs` 는 **전건 green**(내 `LdcUnknownTag` 는 «인접한 이유»로 통과한다). 실제로 무는 것은
  ★**직전 회차의 `classfile::constant_pool::tests::tags_outside_the_accepted_set_are_still_rejected`** 다.
  ⇒ ★**축은 잠겨 있으나 «내가 단언한 층»이 아니다.**
  ★`cargo test --all` **558 → 560 / 0 failed / 1 ignored**(신규 2 · ★감소 0) · DoD **7줄**(= 파리티 검사기 기준 **명령 6개**) 전건 rc=0 ·
  ★`verifier.rs` 의 `Invokedynamic` 줄 **무접촉** · ★`interpreter.rs` **무접촉**(`git diff --stat` 부재).
  ★★**[-fix 회차 2026-09-16 · 게이트② `request-changes` 승계] ★이 회차가 «구멍을 만들었다» — 검수자가 만들어서 쟀고, 내가 재현했다.**
  ★**넓힌 수용집합이 «우연한 백스톱»을 대체 없이 걷어냈다** ⇒ 참조 JVM «도» 못 읽는 파손 condy 2종이 「미지원」이라 답했다
  (`LdcDynamicOldMajor` 태그 17 @ major 52 · `LdcDynamicNoBSM` BSM 부재). ★**before `ab872b7` 는 둘 다 `ClassFormatError`** 였다 — 내가 격리 worktree 로 재측.
  ★★**그 재측에서 «계측 함정»을 하나 밟았다 — 적어 둔다**: 두 워크트리가 **`CARGO_TARGET_DIR` 를 공유**하면
  cargo 가 **낡은 테스트 바이너리를 그대로 링크**해 ★**정반대 답**(before 가 「미지원」)을 준다. 깨끗한 타깃으로 다시 재서야 `ClassFormatError` 가 나왔다.
  ⇒ ★**worktree 간 측정은 타깃 디렉터리를 «분리»하고, 빌드 결과에 그 판본의 문자열이 있는지 `strings` 로 확인하라.**
  ★**고른 갈래 = ⒜ major 버전 축**(`validation.rs` +28줄 · **속성 파싱 0** · `attribute.rs` 무접촉 ⇒ PR #45 와 겹치지 않는다).
  ★★**검수자의 1행을 «4행 표»로 넓혔다 — 추측이 아니라 실측이 시켰다**: 같은 결함이 **태그 15·16 에도** 있었다(major 50 실측).
  ⇒ **15·16·18 ≥ 51 · 17 ≥ 55**(JVMS 4.4). ★**대가 0**: jmods **27,902 클래스 위반 0**
  (★그 corpus 는 전부 major 69·70 이라 ≥51 행을 **시험하지 못한다** — 숨기지 않는다).
  ★**양방향**: M1 새 검사 제거 → 「파손」 축 **2 red**(양성 4종은 ok) · ★**M2 「전부 파손으로 되돌리기」**(수용집합 원복) → ★**양성 4종 red**
  ⇒ ★**되돌리기는 통과 방법이 아니다**(Acceptance ⑶) · 복원 green · `git diff --stat` 으로 `opcode.rs` 원복 확인.
  ★**픽스처 +4**(`LdcTag13`·`LdcTag14`·`LdcDynamicOldMajor`·`LdcDynamicNoBSM`) — ★검수자가 「태그 13·14 로는 구성 불가」가
  **거짓**임을 만들어서 보였다(같은 `_ => Err` 한 줄이 19 에도 적용되므로 자기 증거로 자기를 반증한다). ⇒ **Acceptance 문면을 글자 그대로 덮는다.**
  ★**남는 대역 = 1**(`LdcDynamicNoBSM`) · ★**문안 정정 3건**(0.28% 는 «상한»이 아니라 «탐지 가능 오디코드 관측치» · DoD 수 통일 · jmod **68개**).
  ★`cargo test --all` **560 → 562 / 0 / 1**(신규 2 · 감소 0).
  ★★**[-fix2 회차] 게이트② approve 인데 ⓒ`CONFLICTING` 이라 게이트③로 못 갔다 — 형제 #45 가 «같은 파일 꼬리»에 착지했다.**
  `git merge origin/main`(★리베이스·force-push 0)으로 base 를 당기고 **3파일을 해소**했다. ★**제품 로직 변경 0.**
  ⒜`tests/test_class_format.rs` — ★**양쪽을 «둘 다»**(이 리니지 4테스트 + #45 의 `test_lambda_…` · 픽스처가 `test-data/ldc/*` ↔ `Lambda.class` 로 달라 간섭 0).
  ⒝`REPORT.md`·`STATE.md` 완료 절 — #45 기록을 앞에, 내 기록을 뒤에(순수 합집합 · 삭제 0).
  ⒞★**`STATE.md` 「다음」 절 항목 2 는 «합집합»이 답이 아니었다 — 숨기지 않는다.** 둘 다 남기면 **모순되는 「2.」가
  두 개**(닫힘 ↔ 여전히 열림) 생긴다. ★**그런데 이건 «내 판단»이 아니라 «실측»으로 끝났다**: `origin/main` 쪽 그 블록은
  ★**머지베이스 `ab872b7` 와 바이트 동일**이고(#45 의 기록이 «아니다» — 상속 원문이다) ★**approve 핀 `e9151acf` 가 이미
  그 5줄을 지운 상태**다(`grep -cxF` 전건 0). ⇒ ★**머지가 «승인된 삭제»를 따랐을 뿐 새로 지운 줄이 0 이다.**
  ★#45 가 «실제로 더한» 1줄(항목 1 꼬리 — `verifier.rs` 무접촉 실측)은 **보존**했다.
  ★`classfile/src/validation.rs`·`constant_pool.rs` 는 **자동 병합**(충돌 아님 — #45 예측 적중).
  ★★**게이트③ 착지 — PR #44 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **`-fix2` 회차 approve** · 핀 `b5f268c0` **불이동**(동봉 전 실측 — 로컬·원격·PR head·리뷰 줄2 **4값 일치**) ·
  `ci-presence` **rc=0 CI_GREEN**(3건 전건 완료·성공) · `mergeable` **MERGEABLE/CLEAN** · `merge-tree` 충돌 **0** ·
  자식 PR **0건**(head 브랜치 `feat/rustjava-ldc-tags-15-16-17` 기준) ·
  ★**배포 워크플로 0개 ⇒ 배포 0**(착지 diff 20파일을 `origin/main...HEAD` 로 냈고, `.github/workflows/` 6개 전건
  deploy·publish·release·wrangler·pages 어휘 **0건**).
  ★★**이 리니지의 `-merge` 는 «두 번 렌더»됐다** — 1차(`…-fix-merge`)는 낡은 head `e9151acf` 기준이라 `hold:` 가 걸려
  **큐에 들어가지 못했고**(queue-lint 검사26), 2차(`…-fix2-merge`)가 새 head `b5f268c0` 로 렌더돼 이 착지를 냈다.
  ★**그 `hold:` 가 값을 했다** — 낡은 렌더본이 그대로 돌았으면 게이트③이 `CONFLICTING` 으로 섰다.
  ★★**형제 #46 이 «아직 열려 있다**(head `72db4923` · 게이트③ `blocked(ci-pending)` 재배차 중) ⇒ ★**이 착지가 그 PR 의
  `tests/test_class_format.rs` 꼬리와 원장 2파일을 «다시» 충돌시킨다** — 그쪽은 base 당기기가 한 번 더 필요하다.
  ★★**구조적 근인은 남는다** — 네 PR(#43·#45·#44·#46)이 **같은 파일의 «꼬리»에 테스트를 덧붙인다**.
  파일을 가르거나 테스트를 모듈로 쪼개지 않는 한 **다음 회차도 같은 자리에서 충돌한다**(고치지 않고 적는다).
- [rustjava-cp-tags-16-17-execution-fixtures] ★★**javac 은 태그 17(condy)을 «낸다» — 직전 회차의 「못 찾았다」를 뒤집었다.** `.rs` 런타임 **0줄**.
  채택 제안 `2026-09-16-cp-tags-15-18-parse#p2`(worklog json `adoptedProposals` 에 기록).
  ★★**직전 회차 기록 정정이 아니라 «승계»다** — 그 회차는 「javac 가 그 둘을 내는 평범한 코드를 **찾지 못했다**」고
  ★**정직하게** 적었다(거짓 단언이 아니다). ⇒ ★**이 회차가 찾았다.**
  ★**태그 17 의 산출 조건(이 원장에 처음 박는다)**: ★**`switch` 의 case 라벨이 «정규화된 enum 상수»이고 선택자가 enum 이 아닐 때**(JEP 441).
  javac 이 각 상수를 `java/lang/Enum$EnumDesc` 로 기술하며 `ConstantBootstraps.invoke` condy 를 낸다.
  ★**평범한 enum switch 도, sealed 인터페이스 pattern switch 도 «0» 이다**(둘 다 실측 — 그 둘만 보고 「javac 은 condy 를 안 낸다」로 닫으면 틀린다).
  ★★**희소도를 쟀다**: OpenJDK 26 자체 jmods **27,902 클래스 중 태그 17 보유 = «1»**(`jdk/jpackage/internal/PackageBuilder` · 3항목) ↔
  태그 16 은 **1,747 클래스 · 8,434 항목**. ⇒ ★**16 은 흔하고 17 은 «사실상 없다»** — 그래서 실물 픽스처가 값을 한다.
  ★**픽스처 `test-data/indy/ConstantKinds.class`**(javac `--release 21` · 소스 동봉) — ★**한 파일이 네 태그를 전부** 낸다
  (MethodHandle **7** · MethodType **1** · Dynamic **3** · InvokeDynamic **3**). ★참조 JVM 이 **끝까지 실행**한다(`h3` · rc=0).
  ★★**제안이 예측한 실패 형태가 «실재한다» — 그것을 측정한 것이 이 회차의 값이다.**
  제안 문면: 「오프셋 실수가 **단위 테스트를 통과하고 실물에서만** 드러나는 형태가 이 파서에서 가능하다」.
  ⇒ ★**가능하다**: `parse_all` 의 `is_double_entry` 에 `Dynamic`(또는 `MethodType`)을 더하면
  ★**격리 단위 테스트 `parses_method_handle_family_tags` 는 «ok»**(그것은 `parse_tagged` 를 직접 부른다)인데
  ★**실물 풀은 전 항목이 밀려 `ClassFormatError` 로 죽는다.**
  ★★**양방향 — 출력으로**: ⒜**픽스처 테스트 제거 + 같은 개악** → 전 스위트 ★**558 passed / 0 failed(green)** = ★**그 축을 무는 것이 아무것도 없었다**
  ⒝**픽스처 복원 + 같은 개악** → ★**2층 red**(`constant_pool` 단위 + end-to-end `ClassFormatError: Invalid class file`) · 복원 green.
  ★**단언마다 무는 축**: 「실행이 미지원이라 말한다」 = end-to-end · ★**「픽스처가 그 태그를 «실제로 갖고 있다»」 = `constant_pool.rs` 단위 계수**
  (이것이 없으면 javac 판올림으로 condy 가 사라져도 end-to-end 는 **초록인 채 아무것도 단언하지 않는다**).
  ★**태그 15 는 이 회차 몫이 아니다**(형제 `rustjava-ldc-tags-15-16-17-still-malformed`) — 겹치는 단언을 쓰지 않았다.
  ★`cargo test --all` **558 → 560 / 0 failed / 1 ignored**(신규 2 · ★감소 0) · DoD **7줄 전건 rc=0** ·
  ★`verifier.rs`·`interpreter.rs` **무접촉** · ★`BootstrapMethods` 파싱 **0줄**(형제 L 티켓 몫).
  ★★**[-fix 회차] 게이트③가 `blocked`(code-file-conflict)로 섰다 — 형제 #45 가 «같은 파일 꼬리»에 착지했다.**
  `git merge origin/main`(★리베이스·force-push 0)으로 base 를 당기고 **3파일을 합집합**으로 해소했다:
  `tests/test_class_format.rs`(★**두 테스트를 둘 다** — 픽스처가 `ConstantKinds.class` ↔ `Lambda.class` 로 달라 간섭 0) ·
  `REPORT.md`·`STATE.md`(먼저 착지한 #45 기록을 앞에, 내 기록을 뒤에). ★**제품 로직 변경 0** ·
  ★`constant_pool.rs` 는 **자동 병합**(충돌 아님). ⇒ ★**해소가 만든 «절 구분 빈 줄» 1곳을 REPORT.md 에서 복원했다** —
  `=======` 마커가 그 구분 역할을 하고 있었다(이 repo 에서 두 번째다).
  ★★**게이트③ 착지 — PR #46 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  ★★**이 리니지의 게이트③은 «두 번»이다** — 1회차(`…-execution-fixtures-merge`)가 `blocked`(2-c⒝ `code-file-conflict` ·
  `tests/test_class_format.rs`)로 서서 **머지 0 · 동봉 0** 이었고, 그것이 `-fix`(base 당기기)를 낳았다. ★**그 거부가 옳았다.**
  게이트② **-fix 회차 approve**(반려 0) · 핀 `25bb796f` **불이동**(동봉 전 실측 — 로컬·원격·PR head **4값 일치**) ·
  `ci-presence` **rc=0 CI_GREEN**(3건 전건 완료·성공) · 자식 PR **0건**(head 브랜치 `feat/rustjava-cp-tags-16-17-fixtures` 기준) ·
  ★**배포 워크플로 0개 ⇒ 배포 0**(착지 diff 10파일을 `origin/main...HEAD` 로 냈고, `.github/workflows/` 6개 전건
  deploy·publish·release·wrangler·pages 어휘 **0건**).
  ★★**착지 순서 — 형제 #44 가 «다시» 겹친다**(`ldc` 태그 15·16·17 · `-fix`+`-fix2` 얹힘). ★**#44 도 이 착지 직전까지
  `origin/main = 20a6aa21` 을 base 로 합집합했으므로**, 이 커밋이 들어가면 그쪽 `tests/test_class_format.rs` 꼬리와
  원장 2파일이 **다시 충돌한다** ⇒ ★**#44 는 base 당기기 회차가 한 번 더 필요하다.**
  ★★**구조적 근인은 남는다** — 네 PR(#43·#45·#46·#44)이 **같은 파일의 «꼬리»에 테스트를 덧붙인다**.
  파일을 가르거나 테스트를 모듈로 쪼개지 않는 한 **다음 회차도 같은 자리에서 충돌한다**(고치지 않고 적는다).
  ★★**[-fix2 회차] 위 「게이트③ 착지」 블록은 «예측»이었고 방향이 반대로 실현됐다 — 정정한다.**
  그 블록은 「#46 이 먼저 착지해 #44 를 깬다」로 적었는데, 실제로는 ★**#44 가 먼저 착지(`dc03593`)해 #46 을 깼다**
  (게이트③ 2회차가 `blocked`/`code-file-conflict`). ★**틀린 것은 «구조»가 아니라 «순서»다** —
  「같은 파일 꼬리를 무는 두 PR 중 나중 쪽이 base 당기기를 치른다」는 그대로 참이었다.
  ⇒ 이 회차가 그 값을 치렀다: `git merge origin/main`(★리베이스·force-push 0)으로 base(`dc03593`)를 당기고 **3파일 합집합**.
  ⒜`tests/test_class_format.rs` — ★**#44 의 4테스트를 «전부» 받아들였다**(`…method_handle_family…` ·
  `…illegal_constant…` · `…below_its_minimum_class_file_version…` · `…no_bootstrap_methods_attribute…`) **+ 내 `…every_method_handle_family_tag…` + #45 의 `…lambda_class_reports…`** ⇒ 파일 test fn **7 → 11**.
  ⒝`REPORT.md`·`STATE.md` — 착지분을 앞에, 내 기록을 뒤에. ★**`STATE.md` 는 «양쪽이 같은 꼬리 1줄»을 공유해**
  그 줄이 충돌면 «밖»으로 접혔다(머지 템플릿 2-c⒟ 가 경고한 바로 그 형상) ⇒ ★**꼬리를 양쪽에 복제**해 두 블록을 각자 닫았다.
  ★**제품 로직 변경 0** · `classfile/src/constant_pool.rs` 는 **자동 병합**(충돌 아님).
  ★★**그리고 이번엔 «다시 겹칠» 형제가 없다** — RustJava 열린 PR 은 **#46 하나뿐**이다(실측).
  ★★**게이트③ 착지 — PR #46 · `--merge`**(등재 repo `contracts/upstream-sync-repos.conf:22` · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  ★★★**이 리니지의 게이트③은 «세 번»이었다 — 그 사료를 남긴다**:
  1차 `…-execution-fixtures-merge` **blocked**(`code-file-conflict` · **#45 착지**) → `-fix` base 당기기 ·
  2차 `…-fix-merge` **blocked ×2**(`ci-pending` 자동 재배차 2회 → green → 그 뒤 `code-file-conflict` · **#44 착지**) → `-fix2` base 당기기 ·
  3차(이 회차) **착지**. ★**1·2차의 「해소를 시도하지 않고 blocked」가 둘 다 옳았다** — 게이트③이 게이트②를 삼키지 않게 한 값이다.
  ★★**근인은 «워커 판단»이 아니라 «배열»이었다**(총괄이 원장에 자인): 같은 파일을 무는 형제 PR 둘의 게이트③를 **동시에** 열면
  ★**먼저 착지하는 쪽이 나머지를 «반드시» 깬다 — 확률이 아니라 구조**다. ⇒ 규율 = 「같은 파일을 무는 형제 PR 의 게이트③는 **직렬**로 연다」.
  게이트② **`-fix2` 회차 approve** · 핀 `02628dbe` **불이동**(동봉 전 실측 — 로컬·원격·PR head·리뷰 줄2 **4값 일치**) ·
  `ci-presence` **rc=0 CI_GREEN** · `mergeable` **MERGEABLE/CLEAN** · 자식 PR **0건** ·
  ★**배포 워크플로 0개 ⇒ 배포 0**(착지 diff 10파일 · `.github/workflows/` 6개 전건 deploy 어휘 0건).
  ★**착지 후 검증**: `tests/test_class_format.rs` 테스트 **11개**(기존 5 + 이 회차 1 + #44 의 4 + #45 의 1) ·
  `cargo test --all` **568** 에서 줄지 않음(= `562(origin/main) + 2(#46) + 4(#44)` 가산 검증값).
- [rustjava-cp-tags-15-18-parse-and-honest-diagnosis] ★★**javac 9+ 클래스가 «파손»이 아니라 «미지원»이라고 말한다 — ★실행은 0줄.**
  ★**전/후 실행 출력**: `ClassFormatError: Invalid class file` → ★`UnsupportedOperationException: Unsupported class file feature: invokedynamic`.
  픽스처 `test-data/indy/StringConcat.class` = `System.out.println("a" + args.length);` **한 줄**(`javac --release 21` · major **65**).
  ★★**티켓 ① 의 급소 지목이 «한 칸 모자랐다» — 이 회차의 가장 값진 산출물이다.**
  상수풀 태그 15~18 을 **전부 살려 둔 채** `classfile/src/opcode.rs` 의 `0xba` 분기(`map_res(…, |_| Err(()))`)만
  옛 판본으로 되돌리면 픽스처는 ★**다시 `Malformed`** 로 죽는다(개악 **M3**) ⇒ ★**파서 수정은 «둘»이고 티켓은 «하나»를 적었다.**
  ★**분할 판단 자체는 «지지된다»**(⓪) — 「거부 → 지원」 앞에 칸이 하나 더 있다는 것은 맞고, 그 칸의 **내용**이 한 항목 넓었다.
  ★★**`todo!()` 미도달을 «측정»했다 — 추론이 아니다**(⓪⒝): verifier 의 `Opcode::Invokedynamic(_)` 분기만 일시 제거하면
  ★`panicked at jvm-bytecode/src/interpreter.rs:631: not yet implemented`(**호스트 abort**) ↔ 현 트리는 게스트 예외(**M4**).
  ⇒ ★**그 `todo!()` 와 픽스처 사이에 선 것은 «정확히 verifier 한 줄»이고, 그것이 서 있다**(유일 진입점 `from_classfile` 의 두 번째 문장).
  ★★**대가를 치르지 않았다**(②): `parse_tagged` 의 `_ =>` 분기 **불변** · `validate_constant_pool` 에 새 태그 3종 정합성 검사 추가(JVMS 4.4.8).
  ★**기존 회귀 테스트가 «태그 18 = 미지원»을 사례로 쓰고 있었다** — 이 회차가 그것을 지원하게 만들었으므로 **13·14·19** 로 바꿔 같은 단언을 유지했다.
  ★개악 **4종 전건 red**(M1 태그15 제거 · M2 태그18 제거 · M3 `0xba` 원복 · M4 verifier 분기 제거) · **복원 green**.
  ★`cargo test --all` **554 → 558 / 0 failed / 1 ignored**(신규 4 · ★**감소 0**) · DoD **6종 rc=0** · ★**`interpreter.rs` 무접촉**(`git diff --stat` 부재).
  ★**`BootstrapMethods` 를 «파싱하지 않았다»** — 이미 `Vec<u8>` 로 받아 두고 있고, 그 인덱스를 **역참조하는 코드가 이 회차에 없다**
  (verifier 가 정의 시점에 끊으므로 링크가 일어나지 않는다). `ConstantPoolReference::InvokeDynamic` 이 인덱스를 **원문 그대로** 든다.
  ★**신규 관측 1건**: `ldc` 로 실린 태그 15·16·17 은 ★**여전히 `Malformed`** 다(④-2) — 같은 종류의 거짓말이 한 자리 더 남아 있다.
  ★**알고 남긴 잡음**: `rustfmt` 가 `ConstantPoolItem` 의 **기존 변형 6개를 여러 줄로 펼쳤다**(새 변형이 `struct_variant_width` 35 를 넘겨 enum 전체가 확장형).
  필드 이름을 줄이면 피하지만 JVMS 용어를 버리게 되어 받아들였다.
- [rustjava-test-class-parallel-and-scratch-isolation] ★★**병렬화의 «숨은 전제»를 테스트 소스에 못박았다 — ★격리는 «넣지 않았다».**
  채택 제안 `2026-09-12-null-guard-file-io-fixture#p0`. ★**변경 = `tests/` 주석 2곳뿐**(삽입만 · 런타임·픽스처 무접촉).
  ★**1순위가 «구현»이 아니라 «판단»이었다** — 제안 자신이 「오늘 얻는 것이 없다」로 유보했고 티켓이 **재판정**을 시켰다. ⇒ ★**유보는 옳다.**
  ★★**그러나 제안이 전제를 «하나»로 적은 것은 틀렸다 — 실측하니 «셋»이다**:
  ⑴`test_class` 가 그 바이너리의 **유일한** 테스트 함수 ⑵★**`tests/test_real_jvm.rs` 가 «같은 `test-data/` 를 순회»하는데 `#[ignore]`**
  ⑶`cargo test` 가 테스트 **바이너리를 순차** 실행(실행 로그로 확인).
  ★**⑵가 제안이 몰랐던 «가장 얇은» 전제다** — `#[ignore]` 한 줄을 지우면 같은 디렉터리를 도는 함수가 **둘**이 된다
  (오늘은 ⑶ 덕에 무해하나 ★`cargo nextest` 류로 옮기면 그 방어가 사라진다).
  ★**기록을 «테스트가 읽는 자리»에 뒀다**(계약 2): `test_class` 순회 함수 위(세 전제 + **깨지면 무엇이 일어나는지** + 병렬화 **선행 조건**) ·
  `test_real_jvm` 의 **`#[ignore]` 바로 위**(그 속성이 막고 있는 것) ⇒ ★**병렬화하려는 사람이 «반드시 여는» 두 자리**다.
  ★**대조쌍 «없음»**(Acceptance 가 허용한 경로) — 모의 병렬을 세우려면 계약 3 이 금지한 병렬화를 먼저 해야 해 **억지 픽스처를 만들지 않았다**.
  ★`cargo test --all` **554 / 0 / 1**(전체 실행) · DoD 7종 rc=0.
  ★**방어는 «주석»이지 기계 강제가 아니다** — 알고 고른 값이고(관용만 늘고 지킬 축이 없다), **병렬 러너 도입 회차**가 선행 조건으로 함께 진다.
  ★★**게이트③ 착지 — PR #42 · `--merge`**(등재 repo · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `8f8443ea` **불이동**(해소·동봉 전 실측) · `ci-presence` **rc=0 CI_GREEN** ·
  자식 PR **0건** · ★**배포 워크플로 0개 ⇒ 배포 0**.
  ★**착수 시 `CONFLICTING`** — PR #40·#41 선착지로 원장 2파일이 겹쳤다(★**코드 충돌 0**) · **합집합** 해소(양방향 소실 **0** · 항목 **바이트 동일**).
  ★★**직전 회차가 남긴 «마커=구분선» 규율이 곧바로 값을 했다** — 해소 «전/후» 경계를 **세어** 이 회차가 만든 결손 **1곳**을 찾아 복원했고,
  ★**선재 4곳은 그대로 뒀다**(계약 7). ⇒ 해소 후 경계 결손 **4 = 선재 그대로 · 신규 0**.
  ★★**이 착지로 열린 PR 이 «0» 이 된다** — null 가드 축(감사 → 규격 삼분 → 파일 IO → zip → net)과 그 부산물이 전부 닫혔다.
- [rustjava-fixture-subclassing-abstract-runtime-classes] ★★**게스트 서브클래싱은 «된다» — `java/net` 가드 2곳을 잠갔다(`.rs` 0줄).**
  채택 제안 `2026-09-12-null-guard-spec-triage#p1`. 픽스처 `test-data/NullNetGuards`(2케이스).
  ★**이 티켓은 «조사»가 1순위였다** — 제안 자신이 「**안 되면 그 사실이 산출물**」이라 적었다. ⇒ ★**판정 = «된다».**
  ★**실측**: `URLStreamHandler` 를 상속해 추상 `openConnection(URL)` 만 구현한 **중첩 클래스**로 `protected setURL(...)` 호출
  ⇒ 인스턴스화·도달·**NPE** 전부 관측. `JarURLConnection` 도 `super(null)` + 추상 `getJarFile()` 구현으로 같은 형태가 선다.
  ★★**선례가 이미 있었다** — 기존 픽스처가 **`ClassLoader`**(런타임 제공 클래스)를 상속한다
  ⇒ ★**진짜 미확인은 「런타임 클래스 상속」이 아니라 «추상 + `protected` 진입점» 조합뿐이었다**(조사 전제를 좁혔어야 더 쌌다).
  ★접근 제어는 장애가 아니었다(이 런타임은 접근 플래그 미강제) — 막힐 수 있던 **클래스 링크·추상 메서드 구현**이 통과했다.
  ★★**양방향 — «전/후»가 뒤집힌다**: 픽스처를 **일시 제거**하면 두 개악이 **둘 다 green**(안 잠김) ↔ 픽스처가 있으면 **둘 다 RED**(`class_instance.rs:108`) · 복원 **ok**.
  ★`cargo test --all` **554 / 0 / 1**(전체 실행) · DoD 7종 rc=0 · ★`.rs` **무접촉** · 런타임 기능 추가 **0**(계약 3·대전제 ⓑ).
  ★**감사 수 불변**(N 1,175 · M 25 · K 11) — 당연하다: 이 회차는 가드를 더하지 않고 **시험**만 더한다.
  ★**미커버 나머지 1곳(`ZipFile.getInputStream`)은 끌어들이지 않았다**(대전제 ⓒ) — **형제 PR #40** 몫이고 파일이 겹치지 않는다.
  ★★**게이트③ 착지 — PR #41 · `--merge`**(등재 repo · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `5bedc1bb` **불이동**(해소·동봉 전 실측) · `ci-presence` **rc=0 CI_GREEN** ·
  자식 PR **0건** · ★**배포 워크플로 0개 ⇒ 배포 0**.
  ★**착수 시 `CONFLICTING`** — PR #40 이 먼저 착지해 원장 2파일 최상단이 겹쳤다(★**코드 충돌 0** ⇒ 2-c⒜ 승인 범위).
  **합집합**으로 해소(줄 단위 양방향 소실 **0** · 항목 바이트 동일).
  ★**해소가 만든 «절 구분 빈 줄» 1곳을 복원했다** — `=======` 마커가 구분 역할을 하고 있었다.
  ★**같은 형태의 «선재» 결손 4곳(2026-08-24 까지 거슬러 간다)은 «손대지 않았다»**(계약 7 — 총괄 승계 소재).
  ★★**이 착지로 null 가드 축의 «미커버»가 «0»** 이 된다(#40 의 1곳 + 이 회차 2곳이 마지막이었다).
- [rustjava-zip-output-stream-minimal-for-fixture-reachability] ★★**`ZipFile.getInputStream` 가드를 잠갔다 — 그런데 ★`ZipOutputStream` 을 «만들지 않았다»(런타임 0줄).**
  채택 제안 `2026-09-12-null-guard-spec-triage#p0`. 픽스처 `test-data/ZipGuards`.
  ★★**제안의 «근인 진단»이 틀렸다** — 「근인은 `ZipOutputStream` 부재」였으나, `ZipFile` 인스턴스를 만들려면
  ★**zip 이 «하나 있으면» 되고 그것을 «런타임이 만들» 필요가 없다.** ⇒ 구현 **0줄**.
  ★**티켓이 이 결말을 미리 허용했다**(대전제 ⓑ: 「다른 경로가 있으면 **런타임을 구현하지 않는 것이 더 싼 답**」).
  ★★**더 싼 경로를 «두 단계» 내려갔다**: ⑴미리 만든 zip 을 커밋하니 왕복 성립 ⇒ `ZipOutputStream` 불요 판명
  ⑵★**그런데 새 바이너리조차 불요** — `test-data/test.jar` 가 **이미 있고 jar 은 zip 이다** ⇒ 임시 `ziptest.zip` 을 **지웠다**(추가 바이너리 **0**).
  ★**아카이브 «내용»에 의존하지 않는다** — `getEntry` 를 부르지 않고 **열리기만** 하면 된다(결합도를 일부러 낮췄다).
  ★★**양방향**: 착수 시 같은 개악이 ★**green**(안 잠김) ↔ 이 회차 뒤 ★**red**(`class_instance.rs:108` 호스트 abort) · 복원 **ok**.
  ★`cargo test --all` **554 / 0 / 1**(전체 실행) · DoD 7종 rc=0 · ★`.rs` **무접촉** · deflate·`ZipEntry` 메타 **무접촉**(계약 3).
  ★**남은 한계는 «다른 축»이다** — 게스트가 zip 을 «쓰는» 경로는 여전히 없다. 그것이 필요해지는 것은
  「게스트가 zip 을 만든다」는 요구가 생길 때이지 **가드 잠금 때문이 아니다**.
  ★**부수 관측(고치지 않았다)**: `ZipFile.close()` 가 **미등재**라 픽스처 초판이 `NoSuchMethodError` 를 맞았다 ⇒ 후속 제안.
  ★★**게이트③ 착지 — PR #40 · `--merge`**(등재 repo · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `c7874993` **불이동**(동봉 전 실측) · `ci-presence` **rc=0 CI_GREEN** ·
  자식 PR **0건** · ★**배포 워크플로 0개 ⇒ 배포 0**.
  ★**형제 PR #41·#42 가 이 착지로 원장 2파일에서 충돌한다** — 계약 2-c 의 «정상» 형태(코드 충돌 0 · 셋 다 `.rs` 무접촉).
  그 두 회차가 **base 당김 + 합집합**으로 해소한다.
- [rustjava-null-guard-fixture-for-file-backed-io] ★★**파일 기반 IO 가드 6곳을 «회귀로 묶었다» — `.rs` 변경 0.**
  채택 제안 `2026-09-11-null-guard-audit-and-io-buffer-guards#p1`. 픽스처 `test-data/NullFileIoGuards`(**7케이스** = 6곳 + ★**정리 단언 1**).
  ★★**「가드가 있다」와 「가드가 잠겨 있다」는 다른 문장이다** — 그 6곳은 가드가 들어간 뒤에도 **지워도 아무도 모르는** 상태였다
  (픽스처가 **살아 있는 파일 핸들**을 만들 수 없었다). ⇒ ★**산출물은 «가드»가 아니라 «가드를 지키는 시험»이다.**
  ★**전수(내가 셌다 · 트리 `b50d7d0c`)**: `FileInputStream.read([BII)` · `FileOutputStream.write([BII)` ·
  `RandomAccessFile` 의 `read([B)`·`read([BII)`·`write([B)`·`write([BII)` = **6** (가드 유무는 소스 · 시험 유무는 개악으로 «측정»).
  ★★**개악 대조를 «자리마다» 했다 — 6/6 전건 red**(한 자리로 일반화하지 않았다). 매 회차 뒤 트리 복원 확인.
  ★★**정리를 «시험의 일부»로 만들었다** — 끝에서 `delete()` 후 `exists()` 를 출력하고 기대값이 `gone` 이라
  ★**스크래치 파일이 남으면 그 자체로 시험이 진다**(사람이 따로 볼 필요가 없다). 실행 후 잔여 **0**.
  ★**흔들림 0**(계약 5): `SKIP`·`command -v`·OS 분기·절대경로 **전건 0** · ★**3회 재실행 동일**.
  ★`cargo test --all` **554 / 0 / 1**(전체 실행 · 계수 불변이 맞다) · DoD 7종 rc=0 · 기존 픽스처 **감소 0**.
  ★**알고 남긴 것**: 이 픽스처가 «파일을 쓰는 첫 시험»이고 **고정 이름**을 쓴다 ⇒ `test_class` 를 병렬화하면 충돌한다
  (오늘은 한 함수가 순회하므로 무해). ★`.gitignore` 에 **넣지 않았다** — 넣으면 「남아도 안 보인다」가 되어 위 단언이 무력해진다.
  ★★**게이트③ 착지 — PR #39 · `--merge`**(등재 repo · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `1270ff93` **불이동**(동봉 전 실측) · `ci-presence` **rc=0 CI_GREEN** ·
  자식 PR **0건** · ★**배포 워크플로 0개 ⇒ 배포 0**.
  ★★**착수 시 `CONFLICTING` 이었다 — 그러나 «정상»이다**(계약 2-c): PR #38 이 먼저 착지해 `STATE.md`·`REPORT.md` **최상단**이 겹쳤다.
  ★**코드 충돌 «0»**(원장 2파일 밖 충돌 0 ⇒ 2-c⒜ 승인 범위) · 해소 = **합집합**(양쪽 항목 전건 보존 · 줄 단위 양방향 소실 **0**).
  ★★**그 해소 중에 «내가 앞 회차에 낸 결함»을 찾아 고쳤다** — #38 머지 회차가 게이트③ 블록을 `- [` 헤더 **«위»에** 끼워
  `## 완료` 바로 밑에 **떠 있었다**(origin/main 에 이미 그 형상). 합집합 뒤에는 그것이 ★**#39 항목의 꼬리처럼 읽혀** 더 나빴다.
  ⇒ **#38 항목 «안»으로 옮겼다**(다른 항목의 게이트③ 기록과 같은 자리). ★**충돌 hunk 의 `theirs` 쪽이 바로 그 블록이라 «해소»의 일부다.**
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
  ★★**게이트③ 착지 — PR #38 · `--merge`**(등재 repo · 스쿼시는 부모 2개를 1개로 접어 계보를 지운다).
  게이트② **1회차 approve**(반려 0) · 핀 `934b3944` **불이동**(동봉 전 실측) · `ci-presence` **rc=0 CI_GREEN** ·
  자식 PR **0건** · ★**배포 워크플로 0개 ⇒ 배포 0**.
  ★**검수 minor 5건은 «고치지 않았다»**(계약 7 — 총괄 승계): ⑴⒜ 열머리 「규격이 «요구»」가 6자리 중 3자리(`readUTF`·`FileInputStream(File)`·`JarURLConnection(URL)`)에서
  **Java 8 Javadoc 보다 강하다**(그 자리 근거는 «관측된 구현 거동»이다 — ★**처분은 불변**: 현 거동이 호스트 abort 라 어느 축으로도 오답이고 회차가 근거를 «관측»이라 본문에 선언했다)
  ⑵`FileURLHandler` ⒞ 근거에 «오늘의 호출부» 논거가 섞였다 ⑶⒝⒞ 사유가 **코드에 없다**(worklog 에만 — 특히 `url.rs` 의 `handler` 는
  ★**다음 회차가 «빠진 가드»로 오인해 넣으면 그 자체가 규격 위반**이다) ⑷형제 PR #39 와 원장 2파일이 겹친다 ⑸워커 트리 미커밋 잔재.
  ★★**⑷는 이 회차가 «처분»했다** — 아래 「착지 순서」 참조.
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

### ⓪규칙 — ★**이 절은 «카드 ref + 선행관계 + 카드 밖 항목»만 적는다**(2026-09-23 결정 · `…-adopt-p1`)

★**항목은 전부 ref(`<worklog>#pN`) 또는 PR 번호로 적는다 — 「다음 실작업 = X」 같은 산문 지목 금지.** 할 일의 본문은 카드(`docs/worklog/*.json`)가 갖는다.
★**닫힘 판정 = 그 ref 가 어느 worklog 의 `adoptedProposals`/`declinedProposals` 에 있다**(PR 은 `state`) — 한 줄당 조회 1회로 끝난다. 닫힌 줄은 **닫는 회차가** 지운다.
★**카드 «열림»은 tower 술어(`… − injected`)로 세지 마라** — 이 레인은 그 술어로 **0**이다(주입 = 발권 요청일 뿐 착지가 아니다). 여기 술어는 `전체 − adopted − declined` 다.

1. **선행 사슬**(카드가 표현 못 하는 유일한 것): `2026-09-17-link-lambdametafactory#p1`(결정) → `#p0`(어댑터) → `java.lang.invoke` 패키지(카드 없음 · 근거 = `rustjava-runtime/src/classes/java/lang/invoke` **부재**) → `2026-09-17-string-concat-recipe-arity#p1`.
2. **순서 없음**: `2026-09-18-bootstrap-argument-index-and-tag#p0`(`queue/rustjava` 발권됨) · `2026-09-24-unraisable-error-variant#p0` · `2026-09-12-zip-getinputstream-guard-lock#p0` · `2026-09-12-test-class-scratch-premise#p0`.
3. **카드 밖**: PR **#81** — `CONFLICTING`(2026-09-23 워밍 후 재조회) · 충돌 해소 선행.

---- 이하 ⓪(2026-09-23 전수 재측 판)·①~⑤ 는 사료다. ★**「다음」으로 읽지 마라** ----


### ⓪-사료 — 2026-09-23 전수 재측 판(위 ⓪규칙으로 대체됨)

★★★**이 절이 자기 규율(「닫히는 즉시 닫아라」)을 «세 번» 어겼다** — ⑴③-0 `…claude-md-prune-disposition`(2026-08-27 해소)이
최우선에 남아 레인이 조용해졌고 ⑵①의 「다음은 S5」가 8일 낡은 채 남아 2026-09-11 **중복 발권**을 만들었고
⑶그 자리를 고치며 ①의 꼬리를 「다음 실작업 = ③의 null-guard」로 바꿨는데 **그 null-guard 는 같은 날 이미 닫혀 있었다**
⇒ ★**`LANE_IDLE rustjava` 1088분**(2026-09-23 13:41 · 큐 0 · running 0). ★**네 번째를 만들지 마라 — 닫으면 그 턴에 여기서 지워라.**

★**닫힌 것을 «측정»으로 확인했다**(`git merge-base --is-ancestor <sha> origin/main` · 전건 ANCESTOR):

| ①~⑤ 가 「다음」이라 부르던 것 | 닫은 커밋 | 판정 |
|---|---|---|
| ① 꼬리 「다음 실작업 = ③의 null-guard」 | `6da7d66f` | ★**닫힘** — 지목 자체가 낡았다 |
| ② `wie-ktf-hardening` 잔존 2건(`arraycopy`·`String.<init>`) | `6da7d66f` | ★**전건 닫힘** — 유효 잔존 **0** |
| ③-1 `rustjava-upstream-sync-s5…s8` | (S5~S8 전건 착지) | 닫힘 |
| ③-2 `rustjava-null-guard-string-init-and-arraycopy` | `6da7d66f` | 닫힘 |
| ③-3 / ④-1 ⒝ 의 「`makeConcat` 남음」 | `e94cfe91`(#60 리니지) | ★**닫힘** — `jvm-bytecode/src/string_concat.rs` 의 `FACTORY_NAME_NO_RECIPE` |
| ④-1 ⒝ 의 「`LambdaMetafactory` 런타임 남음」 | `8c7b473f` | ★**닫힘** — `jvm-bytecode/src/lambda.rs` · `Opcode::InvokedynamicLambda` |
| ④-2 의 「★남는 대역 = 1 · `LdcDynamicNoBSM`」 | `d9f45ebf` | ★**닫힘** — `validation.rs` 의 `bootstrap_method_indices_resolve` |
| ④-3 `InputStreamReader` 디코더 경계 | 아래 참조 | ★**절반이 틀렸다** — 사료의 「완화책은 ①로 들어온다」는 **들어왔고**(필드 `endOfInput` 실재), 그러나 EUC-KR 축은 **깨진 채**였다 |

★**열려 있음을 «근거와 함께» 적는다**(「PR 제목·개설일만 보고 썼다가 둘 다 틀린」 선례가 이 레인에 있다):

1. ★★**`java.lang.invoke` 패키지 — 여전히 «0»**(④-1 ⒝ 의 «마지막» 칸).
   근거 = `ls rustjava-runtime/src/classes/java/lang/` 에 `invoke` **부재**(디렉터리 0 · 파일 0).
   ⇒ 링크는 **콜사이트를 opcode 로 내려써서** 돌고 있고(`string_concat.rs`·`lambda.rs`), `MethodHandle`/`MethodType`/`CallSite`
   **객체**는 아직 만들 수 없다. 열린 카드가 그 축에 셋 붙어 있다 —
   `2026-09-17-link-lambdametafactory#p0`(M · 어댑터를 박싱부터 넓혀라) · `#p1`(S · 람다 클래스를 리플렉션에 보일지 «결정») ·
   `2026-09-17-string-concat-recipe-arity#p1`(S · **선행 = `java.lang.invoke` 실재**).
   ★**순서는 `#p1`(결정) → `#p0`(어댑터) → 패키지** — 결정이 패키지의 크기를 정한다.
2. `2026-09-18-bootstrap-argument-index-and-tag#p0`(S) — 직전 착지 `1ec83a14`(#73)의 승계. 「이미 손에 든 수를 버리는」
   나머지 `validation.rs` 규칙을 **센다**. ★계수 회차라 위험 0 이고 다음 진단 회차의 크기를 정한다.
3. `2026-09-12-zip-getinputstream-guard-lock#p0`(S) · `2026-09-12-test-class-scratch-premise#p0`(M) — 런타임/테스트 축.
   이 둘은 ①~⑤ 어디에도 적혀 있지 않았다 ⇒ ★**「다음」의 정본은 이 절이 아니라 `docs/worklog/*.json` 의 열린 카드**임을 적어 둔다
   (2026-09-23 실측 **열린 카드 30건** · 그중 upstream 캠페인·형제 repo(wie·qts) 것이 **11건**이라 이 레인이 칠 수 있는 것은 그보다 적다).
4. ★**게이트③ 미착지 1건**: PR **#81**(`feat/rustjava-sorted-findings`) — ★`mergeable=CONFLICTING`·`mergeStateStatus=DIRTY`
   (2026-09-23 워밍 후 재조회). 충돌 해소가 선행이다. ★**열린 PR 은 이 1건뿐**이다(⑤의 표는 낡았다).

★**④-3 은 이 회차가 닫았다** — `InputStreamReader` 의 EUC-KR 경계. 사료는 「디코더를 read 마다 새로 만든다 = 경계 유실」이라고만
적고 「①로 완화책이 들어오니 다시 재라」고 남겼는데, **재 보니 완화책은 두 축 중 «하나»만 옳았다**:
UTF-8 은 역주사(연속 바이트 `0x80..=0xbf` 가 선두 바이트와 **서로소**라 성립)로 옳고 테스트로도 잠겨 있었는데,
EUC-KR 은 「마지막 바이트 >= 0x81 이면 한 바이트 보류」였다 — ★**EUC-KR 의 후행 바이트는 선두 범위(`0x81..=0xfe`)와 «겹친다»**
⇒ **완성된 쌍**의 후행 바이트도 보류돼 그 쌍의 선두 바이트가 홀로 남고, 매 read 마다 새로 만드는 디코더가 그것을 **자기 상태로 삼켜** 버렸다.
실측 = `"12345678한"`(EUC-KR 10바이트) → ★**`"12345678\u{FFFD}"`**. 처방은 전방 주사(선두면 2, 아니면 1)이고 테스트 3형상으로 잠갔다.


### ①(최우선이 «아니게 됐다») upstream 동기화 — ★★**[2026-09-11 갱신] 캠페인 «종료». 동기 회차를 열지 마라.**

★실측(2026-09-11): **S5(#21)·S6(#22)·S7(#23)·S8(#24) 전건 `--merge` 착지** · `merge-base origin/main
upstream/main` = **`bd42427`** · behind **1**(`2ce4717` · dependabot encoding_rs bump) · ahead 87 · 열린 PR **0**.
★**behind 1 < 임계 20**(PR #29 판정) ⇒ 다음 동기는 «계획된 컷»이 아니라 임계 도달 시다 — 재는 주체는
주 1회 `.github/workflows/upstream-behind.yml`(PR #31)이고, **기계가 재고 사람(총괄)이 발권한다**.
★★**구판 「다음은 S5」는 8일 낡은 채 이 절에 남아 2026-09-11 중복 발권**(`rustjava-upstream-sync-s5-java12-api` ·
blocked)**을 만들었다** — ③ 절 「이미 끝난 일을 가리키면 레인이 조용해진다 · 닫히는 즉시 닫아라」의 두 번째 재현.
~~⇒ ★**다음 실작업 = ③의 null-guard 티켓**(①의 뒤라는 선행 조건이 이제 충족됐다).~~
★★**[2026-09-23 닫음] 그 지목은 «쓰인 날 이미 낡아 있었다»** — ③-2 는 같은 2026-09-11 에 `…-p0` 로 닫혔다(`6da7d66f`).
★**이 한 줄이 세 번째 재발이고 `LANE_IDLE` 1088분의 근인이다.** 살아 있는 후보는 ⓪ 블록에 있다.

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

### ②`wie-ktf-hardening` 잔존분 — ★★**[2026-09-23 닫음] 2건 → «0». 이 절 전체가 사료다.**
★아래 표의 「유효 잔존」 두 행(`System.arraycopy` null 가드 · `String.<init>` null 가드)은 **둘 다 `6da7d66f` 로 닫혔고**
그 사실이 표 안에 이미 적혀 있었는데 ★**절 제목만 「2건」으로 남아** 다음 사람이 잔존이 있다고 읽게 돼 있었다.
⇒ ★**제목과 본문이 어긋나면 제목을 고쳐라** — 본문만 고치면 목차만 보는 다음 회차가 속는다.
---- 이하 사료(2026-08-15 재판정 당시 기재) ----
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

### ③~~다음 회차 발권 후보(우선순위 순)~~ → ★★**[2026-09-23] 1·2·3 «전건 해소». 발권 후보 «0» — 사료다.**

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
3. ~~`rustjava-invokedynamic-cp-tags-15-18-support`~~ → ★★**[2026-09-16] 절반 해소 — «파손 → 거부» 칸을 닫았다**
   (`rustjava-cp-tags-15-18-parse-and-honest-diagnosis` · 완료 절·`docs/worklog/2026-09-16-cp-tags-15-18-parse.md`).
   ★**태그 15~18 파싱 «만»으로는 안 됐다** — `classfile/src/opcode.rs` 의 `0xba` 분기가 **opcode 파싱에서 무조건 실패**해
   그대로 `Malformed` 로 이어졌다(개악 M3 이 그 자리에서 증명). ⇒ ★**「한 티켓으로 묶는다」는 «세 칸»이었다**:
   ⒜상수풀 태그 ⒝opcode `0xba` ⒞실행. **⒜⒝ 착지 · ⒞ 미착수.**
   ★**`todo!()` 는 도달하지 않는다 — 측정했다**(verifier 분기만 빼면 `interpreter.rs:631` 호스트 abort · 넣으면 게스트 예외).
   ★★**[2026-09-16 갱신] 칸은 «셋»이 아니라 «넷»이었다** — `rustjava-ldc-tags-15-16-17-still-malformed` 가
   ⒜′**`ldc` 계열이 그 상수를 «집을» 때**를 닫았다(⒜ 는 상수풀 파싱이고 이것은 opcode 수용이다 — `0xba` 와 같은 형태의 별 칸).
   ⇒ ★**⒜⒜′⒝ 착지 · ⒞ 미착수.** ★남은 것은 ④ 의 1번 항목뿐이다.
   ---- 이하 사료 ----
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

★★**[2026-09-16 갱신] 아래 절의 «태그 15~18 이 없다»·«크레이트 경로 `jvm_rust/`» 는 «낡았다» — 사료로 읽어라.**
경로는 **`jvm-bytecode/src/`** 로 개명됐고, 태그 15~18 은 **파싱된다**(아래 1번). 살아 있는 것은 **디코더 축**(3번)뿐이다.

1. ★★**`invokedynamic` «실행» — ⒜ 착지 · ★남은 것은 ⒝ «콜사이트 링크»뿐**(⇐ ③-3 의 ⒞ 칸).
   ★★**[2026-09-16 갱신 · `rustjava-invokedynamic-bootstrapmethods-and-methodhandle`] ⒜ 를 닫았다.**
   `AttributeInfo::BootstrapMethods` 가 **`Vec<u8>` → `Vec<BootstrapMethod>`**(JVMS 4.7.23 구조) ·
   `MethodHandleRef`(`MethodHandleKind` 9종 + `FieldMethodref`)로 **CONSTANT_MethodHandle 을 «해독»한다**.
   ★★**「결정」의 «경계»를 이름으로 적는다 — 「된다」로 읽지 마라**: 되는 것은 **클래스파일에 적힌 (종류·클래스·이름·서술자)를 꺼내는 것**뿐이다.
   ★**안 되는 것(전부 이름으로)**: ⑴클래스 로드 0 ⑵멤버 조회 0(그 메서드가 실재하는지 아무도 안 본다) ⑶접근 검사 0(JVMS 5.4.3.5) ·
   ⑷★**`java.lang.invoke` 런타임 클래스가 «0개»다**(실측: `rustjava-runtime/src/classes/java/` 에 `invoke` 디렉터리 **부재** · 문자열 참조 **0건**) ⇒ **`MethodHandle` «객체»는 만들 수 없다** ·
   ⑸종류↔대상 짝짓기는 **`validation.rs` 가 진다**(파서는 일부러 중복하지 않는다 — 테스트로 잠갔다) ·
   ⑹`Dynamic`/`InvokeDynamic` 의 `bootstrap_method_attr_index` 는 ★**[2026-09-16 닫힘 · `rustjava-bound-bootstrap-method-attr-index`] 경계 검사된다**(부재 = 0항목 표로 함께 문다) ·
   ⑺★**부트스트랩 «정적 인자»는 «인덱스 그대로»** 둔다(아래 ★).
   ★★**⑺ 이 이 회차의 급소다 — 「인덱스를 `ConstantPoolReference` 로 풀어라」는 «회귀»다**(M3 로 측정):
   `LambdaMetafactory.metafactory` 의 인자는 **MethodType·MethodHandle·MethodType** 이라 해석을 강제하면
   ★**람다가 든 모든 클래스가 «파싱»에서 죽어 `ClassFormatError: Invalid class file` 로 되돌아간다**
   (= 직전 두 회차가 만든 「파손 ↔ 미지원」 구분을 그대로 잃는다). ⇒ **`test-data/indy/Lambda.class` 가 그 축을 «두 층»에서 문다.**
   ~~★**미착수(= ⒝)**: `makeConcatWithConstants` 는 링크됐다 — 남은 것은 `makeConcat` · `LambdaMetafactory` 런타임 · `java.lang.invoke` 패키지 신설.~~
   ★★**[2026-09-23 재측] 셋 중 «둘이 닫혔다»** — `makeConcat` = `string_concat.rs` 의 `FACTORY_NAME_NO_RECIPE`(`e94cfe91` 리니지) ·
   `LambdaMetafactory` = `jvm-bytecode/src/lambda.rs` + `Opcode::InvokedynamicLambda`(`8c7b473f`).
   ★**남은 것은 `java.lang.invoke` 패키지 «하나»다**(실측: `rustjava-runtime/src/classes/java/lang/` 에 `invoke` 부재) ⇒ ⓪-1 로 옮겼다.
   ★★**착수 전 실측 의무 — «그대로 유효»하다**: `verifier.rs` 의 `Opcode::Invokedynamic(_)` 분기를 **언제 뺄지**가 그 회차의 게이트다.
   그것을 빼면 `interpreter.rs:631` 의 `todo!()` 가 **도달 가능해진다** — ★**2026-09-16 에 «두 번» 측정됐다**
   (태그 회차 M4 · 이 회차 M4: 분기 제거 시 `panicked at jvm-bytecode/src/interpreter.rs:631` **호스트 abort** ↔ 현 트리는 게스트 예외).
   ⇒ ★**분기 제거와 인터프리터 구현은 «같은 커밋»이어야 한다.** 따로 하면 그 사이에 호스트 abort 가 산다.
   ★**이 회차는 그 분기를 «건드리지 않았다»**(`git diff --stat jvm-bytecode/src/verifier.rs` **빈 출력**).
2. ★★**[닫힘 2026-09-16 · `rustjava-ldc-tags-15-16-17-still-malformed`] `ldc` 태그 15·16·17 — «미지원»이라고 말한다.**
   진단이 `ClassFormatError: Invalid class file` → `UnsupportedOperationException: Unsupported class file feature: ldc of a …`.
   ★**판정은 «둘»이다 — 하나로 접지 마라**: ⒜**재현됐다** ⒝★**javac 은 그 형태를 내지 않는다**(측정된 부재 —
   JDK jmods **27,902 클래스 · `ldc` 1,252,714 자리 · 0건** · `--release 21/25/26+preview` 14종 소스 0건).
   ⇒ ★**픽스처는 «합성»이다**(`test-data/src/ldc/make_ldc_fixtures.py` · 6종). ★**그렇게 적었다 — 「평범한 코드」로 적지 마라.**
   ★**정당화는 «도달성»이 아니라 «인접성»이다**: 그 셋이 ④-1 이 필요로 할 바로 그 상수들이다.
   ★**참조 JVM 이 양성 4종을 전부 로드·실행**한다 ⇒ 「못 읽는 파일」이 아님이 실측으로 선다.
   ★★**[갱신 2026-09-16 · `-fix` · 게이트② request-changes] 위 「남긴 것 ⑴」은 «미검사»가 아니라 «오진»이었다 — 이 회차가 만든 구멍이다.**
   ★**수용집합을 넓히면서 «우연한 백스톱»을 대체 없이 걷어냈다**: 전에는 `Dynamic → None → opcode 파싱 실패` 라서
   **참조 JVM «도» 못 읽는 파손 condy** 가 `ClassFormatError` 로 끊겼는데, 넓힌 뒤 ★**「미지원」이라고 답한다**
   ⇒ ★**이 리니지의 문장(「못 «읽는» 파일이 아니라 못 «하는» 파일이다」)이 정확히 반대로 뒤집힌 대역**이다.
   ★**공정하게**: before 가 옳았던 것은 «검사해서»가 아니다 — 검사는 애초에 없었다. 그래도 **커버리지 삭제**다.
   ★★**고른 갈래 = ⒜ «major 버전 축»**(`validation.rs` 의 `constant_pool_tags_fit_the_class_file_version` · **속성 파싱 0**).
   ★**그리고 이 회차가 「검수자의 1행」을 «4행 표»로 넓혔다 — 실측이 시켰다**: 같은 결함이 태그 **15·16** 에도 있었다
   (major 50 에서 `ldc of a method handle`/`method type` ↔ 참조 JVM 은 `Class file version does not support constant tag 15/16`).
   ⇒ 표 = **15·16·18 ≥ 51 · 17 ≥ 55**(JVMS 4.4). ★**대가 «0»**: OpenJDK 26 jmods **27,902 클래스 중 위반 0**
   (★단 그 corpus 는 전부 major 69·70 이라 ≥51 행을 «시험하지 못한다» — 숨기지 않는다) · 이 repo `test-data` 위반은 **이 회차가 만든 픽스처 1건뿐**.
   ★★**[2026-09-23 닫음] 아래 「남는 대역 = «1»」은 «0» 이다** — `d9f45ebf`(`rustjava-bound-bootstrap-method-attr-index`)의
   `validation.rs::bootstrap_method_indices_resolve` 가 **「속성 부재 = 0항목 표」**로 그 칸을 함께 물었다(그 회차 worklog 의 표 ⒞행이 `rejected`).
   ★**그 회차가 닫았는데 이 절이 안 지워졌다** — ⓪ 블록의 「세 번 어겼다」와 **같은 계급**이다.
   ---- 이하 사료 ----
   ★★**남는 대역 = «1»**: **`LdcDynamicNoBSM`**(`bootstrap_method_attr_index` 가 가리키는 `BootstrapMethods` 가 **없다**) —
   참조 JVM 은 `Missing BootstrapMethods attribute` 인데 우리는 **여전히 「미지원」**이다.
   ★그 경계 검사는 **속성 파싱이 정말로 필요**하므로 ④-1(PR #45 리니지) 몫이다 — ★**픽스처와 테스트로 «현재 답»을 잠가 뒀으니
   그 회차가 닫으면 그 단언이 «시끄럽게» 진다.**
   ★**남긴 것 둘 더**: ⑵★**M5 층 어긋남**: ★**[2026-09-16 닫힘 · `rustjava-cp-tag-switch-passthrough-mutation-detectable`] 이제 `test_class_format` 이 «잡는다»**(픽스처를 «유일한 결함»으로 다시 지었다 — 종전엔 참조되는 Methodref 슬롯을 덮어 «다른 이유»로 통과했다) ⑶서드파티 생성기 corpus **미측정**(이 머신에 jar 0개).
3. ~~★InputStreamReader 디코더 — 아래 사료 절 셋째 항목 그대로 **살아 있다**(별건).~~
   ★★**[2026-09-23 닫음] 「그대로 살아 있다」는 절반만 참이었다 — 재 보니 두 축 중 «하나»만 깨져 있었다.**
   사료가 요구한 「① 이후 상태를 다시 재라」를 실제로 했다: 완화책은 **들어왔다**(필드 `endOfInput` 실재 · UTF-8 역주사 실재 ·
   그 축은 `test_input_stream_reader_preserves_split_multibyte_and_buffered_eof` 로 **이미 잠겨 있었다**).
   ★**깨져 있던 것은 EUC-KR 축**이고 ★**테스트가 0건이라 아무도 몰랐다**. 근인 = 「마지막 바이트 >= 0x81 이면 보류」인데
   EUC-KR 후행 바이트는 선두 범위와 **겹쳐서** 완성된 쌍도 보류돼 그 선두가 홀로 남고, read 마다 새로 만드는 디코더가 그것을 삼켰다.
   ⇒ 전방 주사로 고치고 3형상을 잠갔다(이 회차). ★**「디코더를 read 마다 새로 만든다」는 구조 자체는 그대로다** —
   `encoding_rs::Decoder` 를 자바 필드에 담을 수 없어 «보류 휴리스틱»이 설계다. ★**그래서 다섯째 charset 을 더하면 이 함정이 되살아난다**(⓪ 아래 빚).

---- 이하 사료(2026-08-16 기재 · 크레이트 경로·태그 서술은 낡았다) ----
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

★★**[2026-09-23 갱신] 아래 표는 «낡았다» — #16 은 머지됐고 지금 열린 PR 은 «#81 하나»다**
(`gh pr list -R Jun025/RustJava --state open` → `feat/rustjava-sorted-findings` · ★워밍 후 재조회 `mergeable=CONFLICTING`·`DIRTY`).
★**「열린 PR = 1건」이라는 수만 우연히 맞아서** 이 표가 27일간 안 고쳐졌다 — ★**수가 같다고 내용이 같은 것이 아니다.**
---- 이하 사료(2026-08-27 실측) ----

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
