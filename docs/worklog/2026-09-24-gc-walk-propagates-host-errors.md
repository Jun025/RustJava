# 2026-09-24 — GC 도달성 순회가 호스트 오류를 `Result` 로 돌려준다

티켓 `wie-2026-09-22-lgt-object-reference-gate-adopt-p0` · 채택 `2026-09-22-lgt-object-reference-gate#p0`
(wie 원 제안 — 원 worklog 무접촉) · `2026-09-24-unraisable-error-variant#p0`(같은 일을 가리키는 이 repo 의 후속 카드).

## 반증 먼저
- ⒜ wie `origin/main`(`7e40b2f4`, 2026-09-24 01:12 +0900) `Cargo.toml:73` `jvm = { version = "^0.1.1" }` — 아직 crates.io 에서 소비한다.
  wie 쪽 게이트(`object_from_raw -> Option`)가 이미 패닉을 막고 있다 ⇒ 이 변경이 wie 에 주는 가치는 «진단 승격»이고,
  효력은 이 PR 착지 + crates.io 릴리스 뒤다.
- ⒝ 변종 절반은 #94(`5cc461f9`, `JavaError::Unraisable`)가 이미 냈다 ⇒ **새 변종 0**. 이 회차는 순회 쪽 절반만 한다.

## 변경
- `jvm/src/garbage_collector.rs`: `determine_garbage`·`find_reachable_objects`·`find_static_reachable_objects`·`find_all_fields`
  → `Result`. `get_field`·`get_static_field`·`load` 의 `.unwrap()` 3곳 → `?`(호스트 오류를 그대로 전달).
  내부 불변식 `.unwrap()` 2곳(`as_array_instance` · 슈퍼클래스 `get_class`) → `Unraisable`(대상을 이름으로 적는다).
- `jvm/src/jvm.rs` `collect_garbage`: `determine_garbage(..)?`. 서명(`Result<usize>`)은 그대로다.
- ★**중단하고, 건너뛰지 않는다**: 읽지 못한 객체가 다른 객체의 유일한 참조를 쥐고 있을 수 있으므로 «나머지만 garbage 로 보고»하면 산 객체를 해제한다.
  순회 중에는 destroy 가 없으므로 오류가 나면 힙은 그대로다.

## 파열 범위 (실측)
- 공개 API 변경 **0** — `garbage_collector` 는 `mod garbage_collector;`(비공개)이고 `collect_garbage` 서명 불변.
- `JavaError` match 지점 신규 파열 **0**(변종 추가 없음). 호출부 `collect_garbage()?` 는 워크스페이스에 원래 `?` 로 쓰여 있어 무수정.

## 양방향
- 새 시험 `jvm/tests/test_gc_host_error.rs`: `get_field` 가 `Unraisable` 을 내는 호스트 `ClassInstance` 를 global ref 로 두면 `collect_garbage` 가 그 오류를 돌려주고,
  ref 를 내린 뒤 GC 는 `Ok(0)` 이며 감싼 `Vector` 는 여전히 쓸 수 있다 → **ok**.
- 개악(`get_field(..)?` → `.unwrap()` 되돌림) → `panicked at jvm/src/garbage_collector.rs:103:55` **FAILED**.

## 한계
- `ClassInstance::class_definition` 은 여전히 `Result` 가 아니다(트레이트 서명 변경 = 모든 impl 파열) — 이번 범위 밖.
- `collect_garbage` 의 `self.destroy(object).unwrap()` 은 그대로다(순회가 아니라 해제 단계 · 중간 실패 시 일부만 해제되는 의미를 따로 정해야 한다).
- `get_static_field`·배열 `load` 경로는 같은 `?` 이지만 전용 시험이 없다(get_field 경로만 잠갔다).
