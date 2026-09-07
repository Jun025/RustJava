# 2026-09-07 — 파리티 락의 repo 별 «파서 축» 설계

티켓 `rustjava-parity-lock-per-repo-parser-axis-design` · 채택 제안 `2026-09-04-parity-sibling-repo-survey#p2`
측정 트리 = 각 repo `origin/main`(2026-09-07 fetch) · ★**형제 repo 파일 편집 0 · 검사기 본체 변경 0 · 발권 0**

## 결론 — 공용 파서를 만들지 않는다. 공용 «계약»만 남긴다

전문은 `docs/upstream-sync-approach.md` §4 「[2026-09-07 설계] repo 별 «파서 축»」. 여기는 요약과 «왜 이렇게 짧아졌나»다.

## ★이 회차를 짧게 만든 사실 하나 — `wie` 는 이미 포팅했다

제안은 2026-09-04 에 쓰였고, `wie` 는 ★**2026-09-05 에 스스로 포팅했다**:
`wie_cli/tests/dod_ci_parity.rs` + `wie_cli/tests/support/dod_ci_parity.rs`,
삭제 방어는 `engine-contract.yml` 의 상시 step `node scripts/check-parity-lock-wired.mjs`.

⇒ ★**설계해야 할 것의 절반이 «이미 집행된 설계의 검증»이 됐다.** 그리고 그 포팅이 축 ⒜~⒟ 를 **우리보다 낫게** 풀었다:

| 축 | 우리(python) | wie(rust) |
|---|---|---|
| 게이트/셋업 분류 | `if:` 유무 | ★**도구 이름**(`cargo` 를 부르는가) |
| DoD 정본 지목 | §제목 뒤 «첫» 코드블록 | ★**이름 있는 마커** + 구간의 **모든** 블록 |
| 블록 스칼라 | `<셸 블록> …` 로 접어 **명령을 잃는다** | ★`flatten_shell` — `export`/`$env:` 를 **env 접두로 보존** |
| 못 보는 것 | 조건부 step 만 출력 | ★**천장 7줄을 pass/fail 무관 상시 출력** |

## ★제안 5축 판정 — 3확인 · 1부분반증 · 1정정

- ✅ `wie` 조건부 step 이 진짜 게이트 — `cargo test --all` 이 `if:` 2 step 에만 있다
- ✅ `wie` env 접두 정규화 필요 — 그 2 step 이 블록 스칼라 + `RUST_MIN_STACK`
- ⚠️ `wie` 다중 워크플로 — ★**부분 반증.** PR 워크플로가 3개인 것은 맞으나 wie 의 DoD 가 스스로 범위를 `rust.yml` 로 못박았다 ⇒ **파서 요구가 아니라 «범위 결정»**이고 wie 자신이 천장 ③으로 기각했다
- ✅ `qts` Makefile 간접층 — DoD 3토큰 ↔ CI 6명령, ★**문자열 교집합 0**
- 🔧 `qts` 축 B «부재» — ★**정정: «퇴화»다.** `strategy.matrix` 0건이지만 `PYTHON_VERSION: "3.12"`·`go-version: "1.22"` 로 원소가 **1개**이고 **DoD 가 그 버전을 아예 안 적는다**. 「없다」로 적으면 다음 사람이 «축을 만들면 된다»로 오독한다
- ✅ `qts` action 기반 검사 — `gitleaks/gitleaks-action@v2`·`docker/build-push-action@v6`

★**제안이 못 본 것 셋**: ⒳wie 자체 포팅 ⒴wie DoD 구간의 fenced block 이 2개(첫 블록만 읽으면 **beta 축 거짓 통과**) ⒵qts DoD 는 fenced block 이 아니라 **산문 불릿**(파싱 대상 자체가 없다).

## ★`qts` 는 파서 문제가 아니다 — 락의 «종류»가 틀렸다

- `make test` = `uv run pytest`(전부) ↔ CI = 마커로 갈린 **3잡**(`-ra` / `guardrail and not integration` / `phase0 and not integration`)
- `make lint` 의 `go vet` 은 ★`command -v go` **조건부 skip**(DoD 산문이 그 skip 을 **명시**한다) ↔ CI 는 전용 `watchdog` 잡의 **경성 게이트**

⇒ ★**어긋남이 «정당»하다. 집합 상등 락은 여기서 «옳은 문서»를 red 로 만든다.**
옳은 축은 **«도달 가능성»** — CI 게이트 잡마다 그것을 로컬에서 치는 `make` 타깃이 있는가.
자리는 이미 있다: `tests/guardrail/test_ci_guardrail_job_marker.py` 가 같은 형태로 `ci.yml` 문자열을 문다 ⇒ **새 층 0**.

## ★우리 저장소의 잠복 위음성 — 이 회차의 «실익»은 여기다

축 A 를 `if:` 로 분류하는데, 지금 조건부 step 1건이 `git config --global core.autocrlf false`(셋업)라 **우연히** 옳다.
누가 `cargo test --all` 을 `if:` 아래로 옮기면 ★**축 A 에서 조용히 빠지고 green** 이 된다.
★**wie 는 그 이동을 «이미 한» repo 다** — 가정이 아니라 형제가 밟은 자리다. ⇒ `proposals[0]`.

## 「세 벌이 된다」는 대가는 치르지 않는다

제안이 적은 그 대가는 실측하면 성립하지 않는다: 파서는 **둘뿐**(우리 python · wie rust)이고 둘 다 **각 repo CI 가 이미 돌리는 런타임**에 얹혀 새 의존이 0이다.
`qts` 는 파서가 아니라 **다른 축의 락**이라 애초에 세 벌째가 아니다.
⇒ ★**공용으로 뺄 것은 코드가 아니라 «파서 축 규칙 + 합격선»이고, 그것이 §4 의 그 절이다.**
