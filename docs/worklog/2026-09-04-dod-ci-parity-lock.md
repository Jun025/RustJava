# [2026-09-04] 로컬 DoD ↔ CI 매트릭스 «기계 대조» — 사람 손을 뺐다

**티켓**: `rustjava-local-dod-vs-ci-matrix-mechanical-check`
**성격**: 검사기 신설 + CI 배선 · 코드(`.rs`) 변경 **0** · 신설 1 · 수정 4
**정본 위치**: ★**문서가 아니라 스크립트 안**(`scripts/check-dod-ci-parity.py` 의 `CI_FILE`·`DOD_FILE`·`DOD_SECTION`)

---

## 1. 왜 — 사람 손 대조가 «다섯 번» 실패했다

`…-going-stale` 리니지는 초판 → fix → fix2 → fix3 으로 **5회차**를 돌았고, ★**매 회차가 「전건 동기」를 주장했고
매번 «또 한 자리»가 나왔다**(1 → 2 → 3 → 4 → 5, 게이트②에서 **6건째**). 전부 같은 종류다 —
「로컬 DoD 가 CI 검사 몇 개를 재현하는가」라는 **수가 `rust.yml` 과 어긋난 채 문서에 굳었다**.

★★**그런데 「낡은 문자열 스캐너」는 만들지 않았다 — 설계 제약이 그것을 금한다.**
게이트② 검수자가 실측으로 세운 것: ★**F1 이 다섯 회차의 sweep 을 통과한 이유는 「수가 아니라 «말»로 쓰인
낡은 주장」이고 그것은 문자열 검사기로도 안 잡힌다.** ⇒ 문서의 **문장**을 검사하면 F1 형태는 여전히 샌다.
⇒ ★**대상은 «수»가 아니라 «사실»이다** — 두 집합을 각각 파싱해 대조하고, ★**문서가 틀리는 «원인»**을 잡는다.

## 2. 두 집합 — ★**원소를 나열한다**(요약 수만 적지 않는다)

### ⒜ 정본 위치 (계약 1 · 실측해서 적는다)

| 축 | 정본 | 실측 근거 |
|---|---|---|
| 로컬 DoD | ★`CLAUDE.md` §Definition of Done 의 **첫 코드블록** | 이 repo 에 DoD 를 실행하는 **스크립트는 없다** — DoD 는 그 코드블록이 전부다(`scripts/` = `check-worklog-json.py` 뿐이었다) |
| CI | `.github/workflows/rust.yml` | job **2 → 3**(`rust_ci` 6셀 · `worklog_json` · ★신설 `dod_parity`) |

### ⒝ 축 A — 명령 집합 (검사기 출력 그대로)

| 명령 | CI | DoD |
|---|---|---|
| `cargo fmt --all -- --check` | y | y |
| `cargo clippy --all -- -D warnings` | y | y |
| `cargo clippy --workspace --exclude test-utils --target wasm32-unknown-unknown -- -D warnings` | y | y |
| `cargo test --all` | y | y |
| `python3 scripts/check-worklog-json.py` | y | y |
| ★`python3 scripts/check-dod-ci-parity.py` | y | y |

★**대칭차 «0»** — CI 6 · DoD 6.
※`cargo +beta clippy …` 는 접두를 벗겨 `cargo clippy --all -- -D warnings` 로 정규화된다(축 B 로 간다).

### ⒞ 축 B — toolchain 집합

| | 값 | 원천 |
|---|---|---|
| CI | `beta` · `stable` | `strategy.matrix.rust: [stable, beta]` |
| DoD | `beta` · `stable` | `cargo +beta …` 1줄 + 맨 `cargo` 5줄(★`rust-toolchain.toml` 부재 ⇒ 기본 = stable) |

★**대칭차 «0»**.

### ⒟ ★검사기가 «못 보는 것» — 침묵시키지 않고 «그 자리에서» 찍는다

- **조건부 step 1건** — `if: startsWith(matrix.os, 'windows')` / `<셸 블록> git config --global core.autocrlf false`.
  ⇒ ★**OS 축은 로컬 재현 불가라 축 A 에서 제외**하되 **출력에 남긴다.**
- ★★**두 축은 «독립»으로 비교한다 — «교차곱»이 아니다.** CI 는 cargo 검사 4종을 stable·beta **둘 다** 치는데
  DoD 는 `clippy` 만 이중이다(= `fmt@beta`·`wasm32@beta`·`test@beta` 3쌍이 로컬에 없다).
  ★**결함이 아니라 2026-09-04 결정이 «고른 값»이다** — 로컬 `cargo +beta test` 는 두 번째 toolchain 전면
  재빌드이고, lint 를 지는 축은 clippy 뿐이며 실측 gap(`allow` 9곳 삭제 → stable 0 / beta rc=101 · 진단 6)도
  거기였다. ⇒ ★**교차곱까지 넓히는 것은 «버그 수정»이 아니라 «새 결정»이다**(제안 `#p0`).

## 3. 배선 — 계약 2 (★경고가 아니라 «막는다»)

지금 대칭차가 **0** 이므로 계약이 정한 대로 **rc=1(차단)** 으로 켰다:
- `.github/workflows/rust.yml` 에 job **`dod_parity`** 신설(단일 러너 — 6셀 매트릭스가 아니다)
- ★**DoD 블록에도 같은 줄을 넣었다** ⇒ ★**검사기가 자기 존재를 자기가 강제한다**(양쪽에 있어야 대칭차 0)
- ⇒ DoD 6줄 → **7줄**. ★**그리고 그 수를 문장에 적지 않는다** — 세는 것은 검사기 몫이다
  (이 리니지가 정확히 그 수로 다섯 번 낡았다).

## 4. 개악 대조 — ★**5건 전건 red · 무개악 green**

| # | 돌연변이 | rc | 검사기가 «무엇을» 말했나 |
|---|---|---|---|
| ⓪ | 없음 | **0** | `OK 두 축 모두 대칭차 0 — 명령 6개 · toolchain 2개로 «둘 다 일치»` |
| ⒜ | `rust.yml` 에 `- run: cargo doc --no-deps` 추가 | ★**1** | `★ CI 에만 있다 — DoD 에 이 줄을 넣어라: cargo doc --no-deps` |
| ⒝ | DoD 에서 **wasm32 줄 제거** | ★**1** | `★ CI 에만 있다 — DoD 에 이 줄을 넣어라: cargo clippy --workspace --exclude test-utils …` |
| ⒞ | 매트릭스 `[stable, beta, nightly]` | ★**1** | `★ CI 에만 있다 — DoD 에 \`cargo +nightly …\` 줄이 없다: nightly` (축 B) |
| ⒟ | DoD 에서 **`+beta` 줄 제거** | ★**1** | `★ CI 에만 있다 — DoD 에 \`cargo +beta …\` 줄이 없다: beta` (축 B) |
| ⒠ | `rust-toolchain.toml` 신설 | ★**1** | `★ … 맨 cargo 가 더는 «stable» 이 아니다 — split_toolchain() 을 함께 고쳐라` |

★**⒜⒝ 는 티켓이 요구한 두 건이고, ⒞⒟ 는 축 B 가 «자기 자리에서만» 반응함을 보이며, ⒠ 는 «축 B 매핑의 전제»를 지킨다.**
★**전부 복구 후 재확인 rc=0** · `diff` 로 두 파일 원상 동일 확인.

## 5. 문서 쪽 — ★**같은 술어를 두 벌로 두지 않았다**

- `docs/upstream-sync-approach.md` §4 의 **셸 두 토막을 지우고** 스크립트 한 줄로 바꿨다
  (★두 벌이면 다음 사람이 한쪽만 고친다 — 이 저장소가 반복해 규탄한 형태).
- **C7⒜**(「DoD 블록 개악은 자동으로 안 잡힌다 — 그 축을 돌리는 것이 사람이다」)에 ★**«닫혔다» 표지**를 달았다.
  ★**⒝(OS 축)는 그대로 열려 있다.**
- 「DoD **6줄**」이라 적힌 **present-tense 3자리**(`REPORT.md` · `STATE.md` · `approach.md`)에 정정 표지를 달았다 —
  ★**이번엔 «7» 로 갈아 끼우지 않고 «수를 문장에서 뺐다»**(그래야 다음 회차에 또 낡지 않는다).

## 6. 경계

★**「낡은 문자열」·「정정 표지」 스캐너 0**(설계 제약) · ★**F1(워크로그 `.md:54`) 무접촉**(그 회차가 아니다) ·
★upstream 동기 무접촉(behind **0** · S9 없음) · ★`allow` 9곳 무접촉 · ★`.rs` 로직 변경 **0** ·
머지 **0** · force-push **0** · `main` 직접 push **0** · 시크릿 출력 **0** · 맨 `grep` **0**.
