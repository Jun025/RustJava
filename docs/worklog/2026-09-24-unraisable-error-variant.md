# 2026-09-24 — `JavaError::Unraisable` : `Jvm::new` 는 `Err`, `Jvm::exception` 에는 바닥

티켓 `rustjava-2026-09-20-name-the-missing-bootstrap-class-adopt-p0` · 채택
`2026-09-20-name-the-missing-bootstrap-class#p0` · `2026-09-20-string-array-hiding-overflows-stack#p0`.

## 반증 먼저 (@origin/main `d5a1d2f0`)
- ⒜ `AGENTS.md:15` 「never panic in library code」 — 문면 그대로.
- ⒝ `JavaError` 단일 변종 · irrefutable `let JavaError::JavaException(..) = ..` **3곳**
  (`jvm/src/jvm.rs` `<clinit>` 래핑 · `jvm/tests/test_exception_construction.rs` · `rustjava-runtime/tests/.../test_throwable.rs`) — 수 일치.
  추가로 exhaustive `match` 2곳(`jvm-bytecode/src/interpreter.rs` · `.../logging/stream_handler.rs`)이 깨졌다.
- ⒞ 재귀: `Jvm::exception` 에 가드 0 — 순환은 닫혀 있다.

## 결정 = ⒜ 비-Java 변종 추가
- 근거: ⑴AGENTS.md 조항 ⑵`Jvm::new` 는 이미 `Result` 를 돌려준다 — 패닉은 서명과 어긋난다 ⑶하류 임베더 실재 —
  wie `2026-09-22-lgt-object-reference-gate#p0` 이 같은 변종을 요청(타이틀 2건이 호스트를 죽였다) ⑷breaking 범위:
  이 repo 3곳 + match 2곳 · wie 는 `jvm` 을 crates.io 0.1.1 로 소비하므로 **판올림 때까지 깨지지 않는다**(wie irrefutable 다수 — 그때 치를 비용).
- 이름 `Unraisable(String)`: 「Java 예외로 만들 수 없다」는 **사실**을 이름으로 — 원인이 호스트(클래스 집합)든 런타임(재귀)이든 같다.
- `#[non_exhaustive]` 는 **안 붙였다** — 소비자에게 `_` 팔을 강제해 새 변종이 조용해진다. 다음 변종이 생기면 그때 판단.

## 티켓과 다르게 한 것 — 「깊이 1」 가드는 틀렸다 (실측)
깊이 1로 넣자 `test_exception_reports_unloadable_class_instead_of_aborting` 이 red:
`jvm.exception("java/lang/NoSuchClassAnywhere", …)` 는 **정상적으로** 안쪽에서 NoClassDefFoundError 를 만든다(1단 중첩 = JVM 동작).
⇒ 규칙: **같은 스레드에서 «같은 예외(타입+메시지)»를 이미 만들고 있으면** 거부(= 순환) + 반복하지 않는 순환의 바닥으로 **깊이 8**.
정상 중첩은 2단이다.

## 측정
- `[Ljava/lang/String;` 숨김 cap 20: 종전 2 MiB 스택 overflow(rc 134) → `Unraisable`, loader 질문 **2회**, 메시지가 첫 실패
  `java/lang/NoClassDefFoundError ([Ljava/lang/String;)` 명명.
- 스윕: `37 candidate(s): 0 recursed · 12 refused by name · 0 refused anonymously · 25 failed cleanly · 0 not needed` — 종전과 동일
  (거부를 패닉이 아니라 `Unraisable` 메시지에서 읽게만 바꿨다).
- 변이(전건 원복 `cmp`): 가드 무력화 → `has overflowed its stack` · 부트스트랩 루프에 익명 패닉 복원 → `…bootstrap_class_is_an_error_naming_it` FAILED ·
  변종 삭제 → `jvm` 컴파일 에러 8.
- `cargo test --all` **594 passed · 0 failed**(592 + 신규 2) · clippy stable/beta/wasm32 rc=0 · fmt rc=0 · python 5종 rc=0.

## 후속
- p0: GC 도달성 순회(`garbage_collector::find_reachable_objects`)의 `.unwrap()` 들을 `Result` 로 — wie 제안의 둘째 반. 변종이 생겼으니 이제 전파할 곳이 있다.
