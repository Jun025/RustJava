# [2026-09-04] upstream 동기 S8 — 개명 스윕 판정 · ★**behind 12 → 0**

**티켓**: `rustjava-upstream-sync-s8-rename-sweep-decision`
**채택 제안**: `2026-09-03-upstream-sync-s5-s7-remeasure#p0`
**컷**: `bd42427fea794dd3f4ee93194c4d990d1ef693ae`(**12커밋** · crates.io 공개 준비 개명 스윕)

⇒ ★★★**upstream 을 «완전히» 따라잡았다** — 2026-08-16 계획 착수 시 behind **33** → 오늘 **0**.

---

## 1. 착수 재측정 (신 서식)

> 컷 `bd42427` 충돌 = ★**누적 8 · 델타 +8**(base `3fb08a8` · merge-base `ba5797b`)

★**「재서 8 이었다」.** 12커밋은 전부 **공개 준비 개명**이다
(`Rename crates for publication` · `Align crate directories with package names` ·
`Rename test data directory` · `Rename Java runtime crate` · `Remove test-util from workspace` 등).

## 2. ★개명 대응표 — upstream 이력에서 «떴다»(추측 0)

`git diff --find-renames --name-status ba5797b upstream/main` ⇒ ★**R 642 · M 22 · A/D 0**.

| 전 | 후 | 건수 | 최소 유사도 |
|---|---|---|---|
| `java_runtime/` | ★**`rustjava-runtime/`** | 379 | R079 |
| `test_data/` | ★**`test-data/`** | 243 | R083 |
| `jvm_rust/` | `jvm-bytecode/` | 12 | R066 |
| `java_class_proto/` | `jvm-class-proto/` | 4 | R056 |
| `java_constants/` | `jvm-types/` | 2 | R061 |
| `test_utils/` | `test-utils/` | 2 | R061 |

★**`java_runtime` 은 «2홉»이다**: `7b966e7`(→`java-runtime` · **R100 399/399**) → `653f543`(→`rustjava-runtime` · R100 307/379).
★**「없다」가 아니라 실측이다** — 상태 분포에 ★**A/D 가 0** 이므로 **표에 없는 경로가 없다**(upstream 은 아무것도 지우지 않았다).
⇒ ★**「미확인」으로 남길 경로 0건.**
★**유사도가 낮은 셋(R056·R061)은 «삭제+신규»가 아니다** — `Cargo.toml`·`lib.rs` 처럼 **패키지명이 본문에 든 파일**이라
낮게 나온 것이고, 같은 디렉터리의 나머지가 R100 이다.

## 3. ★★픽스처 판정 — modify/delete 가 **«0건»이었다**

★**이 회차가 두려워한 형태가 «나오지 않았다».** git 이 **`CONFLICT (file location)`** + **`AU`** 로 처리해
우리 고유 파일을 **새 경로로 «이미 옮겨» 두고 «이동 확인»만 요구**했다.
⇒ 병합 중 **`DU`/`UD`/`DD` 0건**.

| # | 파일(새 경로) | ⒜ 기원 | ⒝ 새 경로 | ⒞ 처분 · 왜 |
|---|---|---|---|---|
| 1 | `rustjava-runtime/src/charset.rs` | ★**우리** — PR #5 `7fd0ad8` | `java_runtime/src/` → `rustjava-runtime/src/` | ★**간다** — upstream 에 **대응물 없음**(대체되지 않았다) |
| 2 | `test-data/TimeApi.class` | ★**우리** — PR #2 `13ab950` | `test_data/` → `test-data/` | ★**간다** — 〃 |
| 3 | `test-data/TimeApi.txt` | ★**우리** — PR #2 | 〃 | ★**간다** — 〃 |
| 4 | `test-data/UnsupportedCharset.class` | ★**우리** — PR #5 | 〃 | ★**간다** — 〃 |
| 5 | `test-data/UnsupportedCharset.txt` | ★**우리** — PR #5 | 〃 | ★**간다** — 〃 |
| 6 | `test-data/src/UnsupportedCharset.java` | ★**우리** — PR #5 | `test_data/src/` → `test-data/src/` | ★**간다** — 〃 |

★**「남는다/버린다」는 하나도 없다.** 「우리 것이면 지우지 마라」가 기본값이고,
★**지울 근거(= upstream 판본으로 대체됐음)가 «한 건도» 나오지 않았다** — upstream 트리에 대응 파일이 없다.

★★**보존 증명은 «블롭»으로 했다**(「존재한다」가 아니라 「같다」):
`origin/main:<옛 경로>` 의 블롭 ↔ 새 경로 `git hash-object` ⇒ ★**6건 전부 동일**
(`0199513a` · `5a394ad6` · `b10f2546` · `b8748f08` · `89833293` · `4ecd8f4f`).
★**우리 픽스처 수: 전 5 → 후 5** · 옛 디렉터리 **6개 전부 소멸**(개명 완료).

## 4. ★★개명이 «우리 자산 2건»을 낡게 만들었다 — 「우리 자산이 낡는」 **5회째**

★**축은 S7 과 같은 방향**(upstream 변경 → 우리 파일이 낡음)이나 ★**대상이 처음**이다 — **CI 설정과 경로 문자열**.

| 자산 | 무엇이 낡았나 | 처분 |
|---|---|---|
| `.github/workflows/rust.yml:55` | `--exclude test_utils` 가 **옛 크레이트 이름** ⇒ wasm32 셀이 깨진다 | `test-utils` 로(주석도) |
| `tests/test_class_format.rs`(PR #3) | `"test_data/Hello.class"` **경로 문자열 4곳** | `test-data/` 로 |

★**실측으로 확정했다**(문면 추측 아님):
`cargo clippy --workspace --exclude test_utils --target wasm32-unknown-unknown` → ★**rc=101** ↔
`--exclude test-utils` → ★**rc=0**.
★**둘 다 «우리 파일»이라 충돌이 «날 수가 없다»** — 개명 대응표를 그대로 적용했을 뿐이고 **upstream 코드는 무접촉**이다.

★★**전례를 이어 붙이면 방향이 «셋»이 된다**:

| 회차 | 방향 | 무엇이 잡나 |
|---|---|---|
| S3 · S5 · S6 | upstream **신규 파일**이 우리 규격 미준수 | 정독 / 자산 grep / 테스트 |
| S7 | upstream **공용 API 변경**으로 우리 파일이 낡음 | 컴파일 |
| ★**S8** | upstream **개명**으로 우리 **CI·경로 문자열**이 낡음 | ★**컴파일이 «절반만»** — 경로는 런타임, CI 설정은 **CI 에서만** |

## 5. `charset.rs`·`test_string.rs` — ★**티켓 계약 3 의 전제가 어긋났다**(정직 고지)

티켓은 이 둘의 **내용 충돌** 해소를 요구했으나 **실측은 다르다**:
- `charset.rs` — ★**위치 충돌뿐**(`AU`). ★**내용 충돌 0** — upstream 에 그 파일이 **없으므로**(순수 우리 것) 내용이 갈릴 수가 없다.
- `test_string.rs` — ★**자동 병합**됐다(상태 `A ` = rename 경유 스테이지). 충돌 0.

⇒ ★**⒜⒝⒞ 형식으로 답할 «내용 충돌»이 이 둘엔 없었다.** 실제 내용 충돌은 **`thread.rs` 하나**이고 §6 이 그것이다.

## 6. `thread.rs` — 유일 내용 충돌 · «직교» 합집합

| | 한 일 |
|---|---|
| ⒜ upstream(**+3/−3**) | 크레이트 개명 import — `java_class_proto`→`jvm_class_proto` · `java_constants`→`jvm_types` |
| ⒝ 우리 | PR #4 의 `use tracing::Instrument;`(수동 span) |

⒞ ★**직교다** — 한쪽은 «개명된 크레이트를 가리키는 import», 다른 쪽은 «span 트레이트 import» 로 **의미가 겹치지 않는다**
⇒ **합집합**. **수동 span 2요소 생존**(`info_span!` + `.instrument(span)`).

## 7. ★`Cargo.lock` — 계약6⒞ 가 «또» 잡았다(S5 와 같은 형태)

`git checkout --theirs Cargo.lock` + `cargo build` 가 ★**3개를 «내렸다»**:

| 크레이트 | 우리 | upstream 쪽으로 내려감 |
|---|---|---|
| ★`tracing` | **0.1.44** | **0.1.41** |
| `tracing-subscriber` | 0.3.23 | 0.3.20 |
| `syn` | 3.0.4 | 3.0.2 |

★★**`tracing` 하강은 PR #4 를 «되돌리는» 것이다** — 그 PR(`fa92ef9`)의 산출물이 정확히
「`tracing-attributes` 상한 핀 제거 ⇒ tracing **0.1.41 → 0.1.44 언프리즈**」였다(upstream `bd42427` 의 lock 은 **0.1.41**).
⇒ **처방은 S5 와 같다**: `origin/main` 의 lock 에서 출발해 다시 `cargo build`
⇒ ★**내려간 것 0** · `tracing` **0.1.44 유지** · `tracing-attributes` **부재 유지**.
남는 변화는 **워크스페이스 멤버 개명**(6 제거/6 추가)과 `classfile`·`jvm` **0.0.1 → 0.1.1**(upstream 이 올린 판 번호)뿐이다.

## 8. green · 시험 수 증감 출처 (계약 5)

| 명령 | stable | ★beta |
|---|---|---|
| `cargo fmt --all -- --check` | **0** | — |
| `cargo clippy --all -- -D warnings` | **0** | **0** |
| `cargo clippy --workspace --exclude test-utils --target wasm32-unknown-unknown -- -D warnings` | **0** | **0** |
| `cargo test --all` | **0** — **554 / 0 / 1** | **0** — **554 / 0 / 1** |
| `python3 scripts/check-worklog-json.py` | **0** | — |

★**새 red 0**: baseline **554/0/1** → 착지 **554/0/1**.
★★**증감 «0» 이고 그것이 «맞다»** — upstream 테스트 함수도 `ba5797b` **547** → `bd42427` **547**
(★**개명 스윕이라 테스트 «추가»가 0** 이다).
★**약화 0**: `#[ignore]` **1 → 1** · 우리 테스트 함수 **558 → 558** · 스킵·단언 완화 **0**.

**우리 자산 전수 생존**: `charset.rs` **1** · `Charset::` 호출부 **4곳** · 수동 span **2곳** ·
`tracing-attributes` **0** · `setProperty` `String` **6곳** · `double_must_use` allow **9곳** ·
픽스처 **5** · `tests/test_class_format.rs` **4/4**.

## 9. ★계보 4축 (계약 6)

| 축 | 값 |
|---|---|
| 머지커밋 부모 | ★**2개** — `3fb08a8` + `bd42427` |
| `merge-base origin/main upstream/main` | ★**`ba5797b` → `bd42427`** |
| behind | ★**12 → 0** |
| `-s ours` | **쓰지 않았다**(S5 이후 **4회 연속** 불필요) |

★★**반증 계산(S7 에서 물려받았다)**: `--squash` 였다면 부모가 접히며 `merge-base` 가 **`ba5797b` 로 되돌아가고**
behind 가 ★**0 → 12** 였다. ⇒ **스쿼시의 대가가 «수»로 보인다.**

## 10. 경계 준수

머지 **0**(PR 제출까지) · 새 PR **1**(base `main` · 스택 아님) · force-push **0** · 리베이스 **0** ·
`main` 직접 push **0** · ★**S9 이후 앞당김 0**(behind 0 이라 앞당길 것도 없다) ·
upstream 코드 «개선» **0**(수렴만 — 고친 2건은 **우리 자산**이다) · ★**§5 과거 기록 소급 수정 0** ·
시크릿 출력 **0** · upstream 발신 **0** · 맨 `grep` **0**(전건 `/usr/bin/grep`).
