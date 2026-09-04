# [2026-09-04] 파리티 락 «교차곱» 확장 판정 — 넓히지 «않는다»(비용을 먼저 쟀다)

**티켓**: `rustjava-dod-ci-parity-cross-product-decision`
**채택 제안**: `2026-09-04-dod-ci-parity-lock#p0`
**성격**: ★**판정 회차** — 검사기 로직 변경 **0** · `.rs` **0줄** · `rust.yml` **무접촉** · DoD 블록 **무접촉**

---

## 1. ★비용을 «먼저» 쟀다 — 추정치 0

★**제안 문면의 「시간이 두 배」도 «주장»이다.** 재서 적는다(모두 이 머신 · 2026-09-04 · 벽시계):

### ⒜ 현행 로컬 DoD 7줄

| 명령 | warm | ★**소스 1줄 편집 후** |
|---|---|---|
| `cargo fmt --all -- --check` | 0s | 0s |
| `cargo clippy --all -- -D warnings` | 1s | 7s |
| `cargo +beta clippy --all -- -D warnings` | 0s | 6s |
| `cargo clippy --workspace --exclude test-utils --target wasm32-… -- -D warnings` | 1s | 10s |
| `cargo test --all` | 17s | **66s** |
| `python3 scripts/check-worklog-json.py` | 0s | 0s |
| `python3 scripts/check-dod-ci-parity.py` | 0s | 0s |
| **합계** | ★**19s** | ★**89s** |

※★**티켓 문면은 「현행 DoD 6줄」이라 적혀 있으나 실측은 «7줄»이다** — PR #26(`61d5bf3f`)이 파리티 검사기 줄을 더했다.
★**이 리니지의 지병(낡은 수)이라 그대로 넘기지 않고 실측 수로 잰다.**

### ⒝ 교차곱으로 넓혔을 때 «추가되는» 3줄

| 명령 | ★**콜드**(최초) | warm | ★**소스 1줄 편집 후** |
|---|---|---|---|
| `cargo +beta fmt --all -- --check` | 7s | 1s | 3s |
| `cargo +beta clippy --workspace --exclude test-utils --target wasm32-… -- -D warnings` | 62s | 0s | 19s |
| `cargo +beta test --all` | 102s | 18s | ★**215s** |
| **합계** | **171s** | **19s** | ★**237s** |

### ⒞ 결론 수치

| 시나리오 | 현행 | 교차곱 | 배수 |
|---|---|---|---|
| 무변경 재실행(warm) | 19s | 38s | **2.00×** |
| ★**소스 1줄 편집 후**(회차가 실제로 겪는 케이스) | 89s | ★**326s** | ★**3.66×** |

⇒ ★★**「두 배」는 «무변경 재실행»에서만 참이다.** 회차는 언제나 «편집 후»에 DoD 를 돌린다 ⇒ **3.66배**가 실제 값이다.
★**지배항** = `cargo +beta test --all` **215s**(같은 편집에서 stable `cargo test --all` **66s** 의 **3.3배**).

### ⒟ ★돈이 아닌 비용 둘 — 재고 나서야 보였다

- ★**전제 미충족**: `rustfmt` 가 beta 에 **설치돼 있지 않다**. `cargo +beta fmt` 는 오늘 그대로 **rc=1**
  (`error: 'cargo-fmt' is not installed for the toolchain 'beta-aarch64-apple-darwin'`).
  설치 자체는 3s 지만 ★**모든 머신·에이전트가 갖춰야 하고, 안 갖추면 DoD 가 즉시 rc=1** 이다.
- ★**시간을 서로 뺏지는 «않는다» — 대신 디스크를 쓴다**: beta 3줄을 돌린 «직후» stable `clippy` **2s** ·
  stable `test` **25s** ⇒ ★**툴체인 교대 축출 0**(두 벌이 `target/` 에 공존한다).
  `target/` **46G** · 이 측정 45분 산출물 **5.43 GiB**.
  ⇒ ★**「두 배」의 흔한 기전(서로 캐시를 밀어낸다)은 «없다»** — 대신 순수 가산이다.

### ⒠ CI 셀 수(계약 1⒞)

`rust_ci` = OS **3** × toolchain **2** = ★**6셀** × 4명령 = **24 실행** + `worklog_json` **1** + `dod_parity` **1** = ★**26**.
로컬 DoD 는 **7**. ★**OS 축은 선언상 범위 밖**(로컬 재현 불가)이라 아래 gap 은 그 축을 뺀 값이다.

## 2. ★«CI 에만 있는 조합» — 목록 (수만 적지 않는다)

CI 조합(OS 축 제외) = 4명령 × 2 toolchain + 2 = **10** ↔ DoD = **7** ⇒ ★**빠진 «3»**:

| # | 명령 | toolchain |
|---|---|---|
| ① | `cargo fmt --all -- --check` | **beta** |
| ② | `cargo clippy --workspace --exclude test-utils --target wasm32-unknown-unknown -- -D warnings` | **beta** |
| ③ | `cargo test --all` | **beta** |

★**비어 있지 않다** ⇒ 「실익 0 이라 자동 기각」 갈래는 **해당하지 않는다.** 실익은 아래에서 «따로» 쟀다.

## 3. ★실익 실측 — 그 3개가 «이력에서» 잡았을 사건

`rust.yml` 전체 **76 run**(성공 **73** · 실패 **3**). ★**실패 3건 «전수» 분해**(추출이 아니라 전건):

| run | sha | 실패 셀 | 실패 step | beta 전용? |
|---|---|---|---|---|
| `33824237441` | `0a2992a7` | ★**6셀 전건** | `cargo fmt --all -- --check` | ✗ — **stable 도 실패** ⇒ 현행 DoD 가 잡는다 |
| `33798515798` | `d8af846d` | beta **3셀** | `cargo clippy --all -- -D warnings` | ✓ — ★**DoD 가 이미 `+beta` 로 덮는 줄** |
| `32674170282` | `eaa56689` | ubuntu **beta** | `cargo clippy --all -- -D warnings` | ✓ — ★**같음** |

⇒ ★★**이 저장소의 «beta 전용 실패»는 전건 `cargo clippy --all` 이고, 그 한 줄은 DoD 가 이미 이중으로 친다.**
⇒ ★★**교차곱 3개가 «새로» 잡았을 사건 = «0건 / 76 run».**

★**주의 — ①의 fmt 실패가 «반례처럼 보이지만 아니다»**: 그 run 은 **6셀 전건** 실패라 stable 이 이미 잡는다
(그것은 S7 회차의 순서 실수였고, 그 회차 회신이 「편집했으면 4종을 다시 돌려라」로 이미 처분했다).
★**교차곱이 필요했으려면 «beta 만» 빨개졌어야 한다.**

## 4. ★판정 — ⒝ **넓히지 않는다**

> ★**`scripts/check-dod-ci-parity.py` 의 두 축(A=명령 · B=toolchain)은 «독립»으로 비교한다 — 교차곱으로 넓히지 않는다.**
> ★**부분 확장(`fmt@beta` 만)도 지금은 채택하지 않는다.**

★**근거는 «비용 대 실익» 두 수뿐이다**: **+237s/회차(3.66×)** ↔ **이력상 실익 0건 / 76 run**.
★**「안 넓힌다」를 «기록»으로 남긴다** — 다음 사람이 같은 제안을 다시 올릴 때 재논의가 아니라 **재측정**으로 가게.

★**부분 확장 ⒞ 기각 사유**(싼데도 안 하는 이유를 적는다): `fmt@beta` 는 편집 후 **+3s** 로 싸다. 그런데
⑴이력 실익 **0건** ⑵beta `rustfmt` **미설치가 기본**이라 DoD 가 **즉시 rc=1**(회차마다 `rustup component add` 선행)
⑶검사기가 축을 «독립»으로 비교하므로 «부분» 교차곱을 표현하려면 **모델 자체를 바꿔야 한다**(= 로직 변경 · 이 회차 금지).
★**단 재개 조건이 «fmt» 에서 발화하면 이것이 «첫 후보»다.**

## 5. ★재개 조건 + 세는 명령 + 오늘의 값

> ★**`rust.yml` 실패 이력에 「`cargo clippy --all` «이외»의 step 이 beta 셀에서만 실패한 run」이 «1건이라도» 생기면 다시 연다.**

```sh
gh run list --repo Jun025/RustJava --workflow=rust.yml --status=failure --limit 200 \
  --json databaseId --jq '.[].databaseId' \
| while read -r id; do
    gh api "repos/Jun025/RustJava/actions/runs/$id/jobs" \
      --jq '[.jobs[]|select(.conclusion=="failure")] as $f
            | ($f|map(select(.name|test("beta")))|length) as $b
            | ($f|length) as $t
            | [$b, $t, ($f[0].steps//[]|map(select(.conclusion=="failure"))|map(.name)|join(","))] | @tsv'
  done | awk -F'\t' '$1==$2 && $3 !~ /clippy --all/ {n++} END{print n+0}'
```

★**오늘의 값 = `0`**. 중간 출력(그대로):
```
3	6	Run cargo fmt --all -- --check          ← beta 3 / 전체 6 ⇒ beta 전용 아님
3	3	Run cargo clippy --all -- -D warnings   ← beta 전용이지만 DoD 가 덮는 줄
1	1	Run cargo clippy --all -- -D warnings   ← 같음
```
★**1 이상이면**: 그 step «하나만» 교차곱에 넣고(전부 넣지 마라) **위 비용표를 다시 재라** — 이 표의 수는 «그때의 값»이다.
★★**「비용이 싸졌다」는 재개 사유가 «아니다»** — 실익이 0 인 동안에는 싸도 넣지 않는다.

## 6. 적은 «자리»와 이유

| 자리 | 무엇을 | 왜 |
|---|---|---|
| `docs/upstream-sync-approach.md` §4 | 결정문 · 비용표 · 실익표 · 재개 조건 | ★**이 리니지의 결정이 사는 곳**(파리티 락 C7 처분·마감 절차와 같은 절) |
| ★`scripts/check-dod-ci-parity.py` **docstring** | 「그 결정은 2026-09-04 에 났고 답은 NO · §4 를 보라」 **1블록** | ★그 파일이 스스로 「Widening … is a decision, not a bug fix」라고 적어 두고 **결정이 났는지는 말하지 않았다** ⇒ ★**검사기를 읽는 사람이 그 문장 하나로 오도된다** |
| 이 워크로그 | 판정 전문 + `adoptedProposals` | 규약 |

★**docstring 외 코드 변경 0** — 검사기의 실행 경로는 한 줄도 건드리지 않았다(계약).

## 7. 경계

★검사기 **로직 변경 0** · `.rs` **0줄** · `rust.yml` **무접촉** · ★**DoD 블록 무접촉**(「넓힌다」가 아니므로 줄을 더하지 않았다 —
더했으면 `dod_parity` 가 즉시 red 가 되어 이 repo 의 모든 PR 을 막는다) ·
★**돌연변이·측정 흔적 전건 복구**(`jvm/src/jvm.rs` 편집 → `git checkout`) ·
머지 **0** · force-push **0** · `main` 직접 push **0** · 시크릿 출력 **0** · upstream 발신 **0** · 맨 `grep` **0**.
※★**측정 부작용 1건은 «남는다»**: `rustup component add --toolchain beta rustfmt` — ★**되돌리지 않았다**
(무해하고, 재개 조건이 fmt 에서 발화하면 바로 필요하다). 회신에 신고한다.
