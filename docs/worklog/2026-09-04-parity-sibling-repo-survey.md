# [2026-09-04] 형제 repo(`wie`·`qts`) 파리티 락 필요성 조사 — ★**둘 다 «조건부 필요»**

**티켓**: `rustjava-dod-ci-parity-sibling-repo-survey`
**채택 제안**: `2026-09-04-dod-ci-parity-lock#p1`
**성격**: ★**읽기 전용 조사 + 판정** — 형제 repo **쓰기 0**(`git`·PR·파일 수정 전부 0) · 구현 **0**

★**형제 repo 는 GitHub API 로만 읽었다**(`gh api repos/<slug>/contents/...`) — 그 워킹트리·`.git` 을 건드리지 않았다.
⇒ 읽은 것은 **`origin/main` 의 내용**이고, 그 레인의 미커밋 작업은 이 조사에 들어오지 않는다.

---

## 1. ★repo 당 «정본 실측» — 구조가 «같지 않다»

| | **RustJava**(기준) | **wie** | **qts** |
|---|---|---|---|
| DoD 정본 | `CLAUDE.md` §Definition of Done 의 **첫 코드블록** | ★**`AGENTS.md`** §Definition of Done → 「The four gates」 코드블록(```sh · 4줄) | `CLAUDE.md` §Definition of Done — ★**코드블록이 «아니라» 산문**이고 내용이 **`make` 타깃 이름** |
| 간접층 | 없음(명령이 곧 명령) | 없음 | ★★**`Makefile` 이 있다** — `make lint`/`test`/`guardrail`/`sync` |
| CI 파일 | `rust.yml` **1개**(+`worklog_json`·`dod_parity` 동거) | `rust.yml` + ★**7개 더**(`coverage`·`engine-contract`·`web`·`publish-artifact`·…) | `ci.yml` + ★**11개 더**(배포 다수) |
| CI job | 3 | rust.yml 1(6셀) + 타 워크플로 다수 | ci.yml **7**(`gitleaks`·`lint`·`test`·`guardrail`·`phase0`·`go-watchdog`·`build`) |
| ★**toolchain 매트릭스** | `rust: [stable, beta]` | ★**`rust: [stable, beta]`**(동일) | ★★**없다**(python 3.12 단일) ⇒ **축 B 자체가 없다** |
| 조건부 step 의 성격 | ★**설정뿐**(windows `git config`) | ★★**«진짜 게이트»가 조건부**(`cargo test` 가 windows/비-windows 2 step) | 설정 + 배포(`build` job) |

⇒ ★★**「구조가 같은가」의 답: wie 는 «비슷하나 같지 않다» · qts 는 «다르다».**

## 2. ★두 집합의 대칭차 — «지금» 쟀다(원소 나열)

★**전제**: 「대칭차가 0 이면 «지금은 안 어긋나 있다»이지 «어긋날 수 없다»가 아니다」 —
★**그런데 두 repo 다 «0 이 아니다».** 그래서 이 구별을 쓸 자리가 아니었다.

### ⒜ `wie` — RustJava 파서 규칙을 «그대로» 적용했을 때

**축 A(명령)** · CI 비조건부 `- run:` **3** ↔ DoD **4**:

| 명령 | CI | DoD |
|---|---|---|
| `cargo fmt --all -- --check` | y | y |
| `cargo clippy --all -- -D warnings` | y | y |
| `cargo clippy --target wasm32-unknown-unknown -- -D warnings` | y | y |
| ★`RUST_MIN_STACK=4194304 cargo test --all` | ★**n**(조건부 2 step · 블록 스칼라) | y |

⇒ 대칭차 **1** — ★★**그런데 이것은 «거짓 양성»이다.** CI 는 그 검사를 **정말로 친다**. 다만
⑴`if:` 로 windows/비-windows 두 갈래이고 ⑵`export RUST_MIN_STACK=…` 를 **별 줄**에 쓴 **블록 스칼라**라
RustJava 의 「조건부 = OS 축 = 제외」 규칙과 「한 줄 문자열 동일성」에 둘 다 걸린다.

**축 B(toolchain)** · CI `{stable, beta}` ↔ DoD `{stable}`:

| toolchain | CI | DoD |
|---|---|---|
| stable | y | y |
| ★**beta** | ★**y** | ★★**n** |

⇒ 대칭차 **1** — ★★**이것은 «진짜»다.** `cargo +beta` 는 wie 의 **4 gates 에 없고**,
★**`beta` 라는 문자열이 `AGENTS.md`·`CLAUDE.md` 어디에도 «0건»** 이다(`rust.yml` 주석에만 있다).

### ⒝ `qts` — ★**문자열 대조가 원리적으로 안 맞는다** ⇒ «의미»로 폈다

`make` 타깃을 Makefile 로 펼친 DoD 집합:
`uv sync --all-packages` · `uv run ruff check .` · `uv run pytest` · `uv run pytest -m guardrail`
(+ `go vet ./...` · `go test ./...` — ★**둘 다 `command -v go` 조건부라 이 맥북에서는 «조용히 skip»**)

`ci.yml` 의 검사 집합 ↔ 위:

| CI 검사 | 로컬 DoD 에 있나 |
|---|---|
| `uv run ruff check .` | y |
| ★**`uv run ruff format --check .`** | ★★**n — 진짜 gap** |
| `uv run pytest -ra` | y(플래그만 다름) |
| `uv run pytest -m guardrail -ra` | y(〃) |
| ★`uv run pytest -m "phase0 and not integration" -ra` | ★**n** |
| ★`gitleaks-action`(★`run:` 이 아니라 **action**) | ★**n** — `run:` 파서가 **원리적으로 못 본다** |
| `go vet` / `go test`(job `go-watchdog`) | ★**조용히 skip**(qts `CLAUDE.md` 가 이미 그 사실을 적어 뒀다) |

⇒ ★**CI 에만 있는 것 = 최소 3종**(`ruff format --check` · `phase0` · `gitleaks`).
★**`ruff format --check` 는 `CLAUDE.md`·`AGENTS.md` 어디에도 «없다»**(「ruff」만 1회 언급).

## 3. ★실익을 «이력»으로 쟀다 — 두 곳 다 «실제로 났다»

RustJava 회차가 「이력 0건이면 안 넣는다」로 판정했으므로 **같은 자로** 쟀다.

### ⒜ `wie` — `rust.yml` 최근 **200 run**(성공 **166** · 실패 **34**) · ★**실패 34건 전수 분해**

| 형태 | 건수 | 로컬 DoD 가 잡나 |
|---|---|---|
| `cargo clippy --all` 이 **stable** 에서 실패 | **24** | ✔ 잡는다 |
| ★★`cargo clippy --all` 이 ★**beta 에서만** 실패 | ★**9** | ✘ ★**못 잡는다**(DoD 에 `+beta` 가 없다) |
| windows `RUST_MIN_STACK` test 실패 | 1 | ✘(OS 축 — 로컬 재현 불가) |

⇒ ★★**「로컬 green ↔ CI red」가 «9건 / 34건 = 26%»** 이고, ★**한 줄(`cargo +beta clippy --all -- -D warnings`)이 그 전부를 로컬로 끌어온다.**
★**RustJava 는 이 구멍을 이력 «2건»에서 발견해 고쳤는데, wie 는 «9건»이 났는데도 열려 있다.**

### ⒝ `qts` — `ci.yml` 최근 **실패 60건**(2026-07-18 ~ 2026-09-03) 전수 분해

| 형태 | 건수 | 로컬 DoD 가 잡나 |
|---|---|---|
| `uv run ruff format --check .` 가 실패 step 에 **포함** | **38 / 60 (63%)** | ✘ |
| ★★그중 ★**그것이 «유일한» 실패** | ★**10 / 60 (17%)** | ✘ ★**로컬 DoD 를 전부 돌려도 green 이었다** |
| `uv run ruff check .`(= `make lint` 가 치는 것) | 7 | ✔ |
| `phase0` 만 실패 | **0** | — |

★**모집단을 흐리지 않는다**: 같은 도구의 「최근 **200 run**」 창에서는 성공 188 · 실패 2 · 취소 9 다 —
★**두 창의 모집단이 다르다**(위 60건은 «실패만» 60건까지 거슬러 올라간 창이다). 비율은 **그 창 안에서만** 읽어라.

## 4. ★판정 — repo 별

### ⒜ `wie` = ★**⒞ 조건부 «필요»**

★★**단 «먼저 할 일»은 파리티 락이 «아니다» — DoD 한 줄이다.**
> ★**`AGENTS.md` §Definition of Done 의 four gates 에 `cargo +beta clippy --all -- -D warnings` 를 더하라.**
> 그 한 줄이 이력 **9/34(26%)** 를 로컬로 끌어온다. ★**파리티 락은 그 «다음»이다.**

★**파리티 락을 포팅하려면 파서를 «세 군데» 고쳐야 한다** — 복사로는 안 된다:
⑴★**조건부 step 이 «진짜 게이트»일 수 있다**(RustJava 는 설정뿐이었다) ⇒ 「조건부 = 제외」가 **거짓 red** 를 만든다
⑵★**env 접두 정규화**(`RUST_MIN_STACK=… cargo test` ↔ `export …` + 별 줄 블록 스칼라)
⑶★**다중 워크플로**(검사가 `coverage`·`engine-contract`·`web` 에도 있다) ⇒ 단일 파일 파서 부족

### ⒝ `qts` = ★**⒞ 조건부 «필요»**

★★**RustJava 검사기는 «포팅 불가»다** — 구조가 다르다:
⑴★**`Makefile` 간접층** ⇒ 「DoD 줄 ↔ CI `run:` 줄」 문자열 대조가 **원리적으로 성립하지 않는다**
⑵★**toolchain 매트릭스가 없다** ⇒ **축 B 가 아예 없다**(검사기의 절반이 무의미)
⑶★**action 기반 검사**(`gitleaks-action`)는 `run:` 이 아니라 **어떤 `run:` 파서도 못 본다**

★★**단 «먼저 할 일»은 여기서도 한 줄이다.**
> ★**`make lint` 에 `uv run ruff format --check .` 를 더하라**(또는 DoD 가 그것을 직접 부르게 하라).
> 그 한 줄이 이력 **10/60(17%)의 «로컬 green ↔ CI red»** 를 없앤다.

그 뒤에도 기계로 잠그려면 ★**Makefile 을 펼쳐 CI 와 대조하는 «다른» 검사기**가 필요하다(이 회차 범위 밖).

### ⒞ ★공통 — 이 조사가 확인한 «일반 사실» 하나

★★**세 repo 가 전부 「로컬 체크리스트 ↔ CI」 어긋남을 갖고 있었고, 어긋난 자리가 전부 «규범 문서에 적혀 있지 않았다».**
RustJava = wasm32 줄 + beta 축 · wie = **beta 축**(문자열 `beta` 가 규범 문서에 **0건**) ·
qts = **`ruff format --check`**(문자열 `format` 이 규범 문서에 **0건**).
⇒ ★**「사람이 문서를 최신으로 유지한다」는 세 repo에서 «각각» 실패했다.** 이것이 기계 대조의 일반 근거다.

## 5. ★다음 티켓의 «축과 합격선»(구현은 이 회차 밖 · ★그 repo 레인 몫)

★**이 회차는 형제 repo 를 한 바이트도 고치지 않았다.** 아래는 **총괄이 그 레인에 발권할 때** 쓸 재료다.

### T1 — `wie`: four gates 에 beta 축 추가 (**XS** · 선행)
- **축**: `AGENTS.md` §Definition of Done 의 코드블록에 `cargo +beta clippy --all -- -D warnings` 1줄.
- **합격선**: ⑴그 줄 rc=0 ⑵★**이력 재계산** — 「beta 셀에서만 실패한 run」이 이제 로컬로 잡히는지
  회신에 **9/34 를 인용**하고 ⑶`rustup component add --toolchain beta clippy` 전제를 **명시**(RustJava 실측: beta 에 `rustfmt` 가 없었다).
- ★**넓히지 마라** — `+beta fmt`·`+beta test` 는 RustJava 가 **이력 0건**을 근거로 기각했다(`…-cross-product-decision`).
  wie 에서 넣으려면 **wie 이력으로 다시 재라**(그 repo 의 windows test 실패 1건은 OS 축이지 toolchain 축이 아니다).

### T2 — `qts`: `make lint` 에 포맷 검사 추가 (**XS** · 선행)
- **축**: `Makefile` `lint:` 에 `uv run ruff format --check .` 1줄(★`make fmt` 는 «고치는» 것이라 게이트가 아니다).
- **합격선**: ⑴`make lint` rc=0 ⑵★**이력 인용** 10/60 ⑶★**`make fmt` 와 혼동하지 않도록 CLAUDE.md DoD 에 한 줄 명시.**

### T3 — 파리티 락 포팅 (**S~M** · ★T1·T2 «뒤»)
- **합격선(공통)**: ⒜정본 위치를 **코드에 박을 것**(문서 금지 — RustJava 계약 3) ⒝**성공에도 원소를 찍을 것**(조용한 통과 금지)
  ⒞**못 보는 것을 그 자리에서 찍을 것** ⒟★**개악 대조 ≥3건 red · 무개악 green**(공허하지 않음 증명).
- **wie 전용 축**: 조건부 step 을 «게이트/설정»으로 가를 술어 · env 접두 정규화 · **다중 워크플로 스캔**.
- **qts 전용 축**: **Makefile 타깃 전개** · 축 B 부재 처리 · action 기반 검사를 «못 본다»고 **명시 출력**.
- ★**T3 를 T1·T2 «앞»에 두지 마라** — 락은 «어긋남을 막는» 것이지 «이미 난 어긋남을 고치는» 것이 아니다.

## 6. 경계

★**형제 repo 쓰기 0** — `git` 명령 0 · PR 0 · 파일 수정 0(전부 `gh api … /contents` 읽기).
★`.rs` **0줄** · 이 repo 의 **검사기·`rust.yml` 무접촉** · **DoD 블록 무접촉** ·
머지 **0** · force-push **0** · `main` 직접 push **0** · 시크릿 출력 **0** · upstream 발신 **0** · 맨 `grep` **0**.
