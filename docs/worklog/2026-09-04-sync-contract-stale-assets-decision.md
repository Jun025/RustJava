# [2026-09-04] 「우리 자산이 낡는다」 상시 조항 «판정» — 넣지 않고 «구멍 하나»를 막았다

**티켓**: `rustjava-sync-contract-standing-clause-for-our-assets-going-stale`
**채택 제안 2건**: `2026-09-04-upstream-sync-s6#p1` · `2026-09-04-upstream-sync-s7#p1`
**성격**: 결정 회차 · 코드 변경 **0** · 문서 **2파일**

---

## 1. ★먼저 «셌다» — 목록 문서화로 시작하지 않았다

티켓이 못박은 순서 그대로: ★**「그물이 없는 자리」의 «수»가 이 결정을 정한다.**
자산 8건을 ★**돌연변이로 깨뜨려**(추론 0) 무엇이 잡는지 쟀다.

| # | 우리 자산(출처) | 돌연변이 | build | clippy | test | ★그물 |
|---|---|---|---|---|---|---|
| ① | 픽스처 경로 문자열(PR #3) | `test-data/` → `test_data/` | ok | ok | ★**RED** | 있음 |
| ② | ★**CI 워크플로 `--exclude test-utils`**(S8) | `test-utils` → `test_utils` | — | — | — | ★★**없음(CI 만)** |
| ③ | `System.setProperty` 서술자(PR #5) | `String` → `Object` | ok | ok | ★**RED** | 있음 |
| ④ | charset 라우팅(PR #5) | `Charset::resolve` → 폴백 우회 | ok | ok | ★**RED** | 있음 |
| ⑤ | 수동 span(PR #4) | `.instrument(span)` 삭제 | ok | ★**RED** | ok | 있음 |
| ⑥ | `ClassFormatError` 종류 단정(PR #3) | `ClassFormatError` → `Throwable` | ok | ok | ★**RED** | 있음 |
| ⑦ | `double_must_use` allow(PR #14) | allow 삭제 | ok | ok(★beta 도 ok) | ok | ★**해당 없음 — 깨져도 무해** |
| ⑧ | 워크로그 잠금 스크립트 경로(PR #15) | 스크립트 이동 | — | — | — | 있음(로컬 DoD **와** CI 둘 다 · rc=2) |

★**⑦은 「그물 없음」이 아니라 「위험 없음」이다** — allow 를 지워도 stable·beta 둘 다 통과한다
(upstream 이 그 자리를 바꿔 allow 가 불필요해졌다). ⇒ 낡아도 아무것도 깨지지 않으므로 이 결정의 대상이 아니다.

⇒ ★★**「아무것도 없음」 칸은 «1개»(②)다.**

## 2. ★★그 1개도 «조용히» 실패하지 않는다 — 그 정정이 처방을 더 좁혔다

낡은 이름으로 치면 cargo 가 이렇게 말한다:

```
warning: excluded package(s) `test_utils` not found in workspace `…/RustJava`
error: Only features sync,macros,io-util,rt,time are supported on wasm.
error: could not compile `tokio` (lib) due to 1 previous error      ← rc≠0
```

⇒ ★**문제는 «침묵»이 아니라 «늦음»이다** — push 후 CI 에서만 난다.
⇒ ★★**근인은 「자산 목록이 없다」가 «아니라» — 로컬 DoD 가 CI 검사 한 줄을 빠뜨린 것이다.**

**대조 실측**:

| | 검사 |
|---|---|
| CI(`rust.yml`) | `fmt --all -- --check` · `clippy --all` · ★**`clippy --workspace --exclude test-utils --target wasm32`** · `test --all` · `check-worklog-json.py` = **5** |
| 로컬 DoD(종전) | `cargo fmt --check` · `cargo clippy` · `cargo test` · `check-worklog-json.py` = **4** |

★**빠진 하나가 정확히 wasm32 줄**이고, ★**그 줄이 `--exclude <크레이트 이름>` 이 사는 «유일한» 자리**다.
⇒ S8 의 개명(`test_utils`→`test-utils`)이 **로컬 어디에서도 안 드러난** 이유가 이것이다.

## 3. 사료 확정 — 「세 번」·「두 방향」 (계약 2)

| 회차 | 낡은 것 | ⒝ 방향 | 잡은 것 |
|---|---|---|---|
| **S3** | `test_class_format.rs` 문구 단정 3건 | ⑴upstream **신규/판본 교체** | 테스트 |
| **S5** | io 테스트 **5곳** `setProperty` 서술자 | ⑴ | 테스트 |
| **S6** | regex 테스트 **3곳** 〃 | ⑴ | ★정독(테스트 «전») |
| **S7** | 우리 고유 테스트 **5곳** `invoke_virtual` | ⑵**공용 API 시그니처 변경** | ★컴파일 |
| **S8** | `rust.yml` 크레이트명 · `test_class_format.rs` 경로 4곳 | ⑶**개명** | ★CI 만 / 테스트 |

⒜ **「세 번」 = S3·S5·S6**(제안 `#p1` 이 센 것) · ⒝ **「두 방향」 = ⑴·⑵**(S7 이 가른 것) ·
★**S8 이 ⑶을 더해 «셋»이 됐다.**

⒞ ★★**같은 형태인가 → «아니다».** 잡는 그물이 각각 다르다(테스트 · 컴파일 · CI).
⇒ ★**조항 하나로 못 덮는다** — 덮으려 하면 **이미 그물이 있는 다섯 자리에까지 사람 확인을 얹는다**. 그것이 비용이다.

## 4. ★★결정문 · 적은 «자리» · 이유 (계약 3)

> ★**동기 회차 계약에 「우리 자산 서술자 목록을 문서화하고 머지 후 재확인한다」 상시 조항을 «넣지 않는다».**
> ★**대신 «그물이 없던 자리 하나»를 막는다 — `CLAUDE.md` §Definition of Done 이 CI 명령 5줄을 «축약 없이 그대로» 싣는다.**

**⒜ 왜 이것이 «가장 좁은» 형태인가**:
- 계약 자체가 정했다 — 「**1개면 계약 조항이 아니라 «그 하나를 고치는 것»이 처방**」.
- ★**사람 기억에 맡기지 않는다** — 추가한 것은 «체크리스트 항목»이 아니라 ★**실제로 도는 명령**이다.
  회차가 DoD 를 돌리면 **cargo 가 스스로 낡음을 말한다**(`excluded package(s) … not found`).
- ★**목록을 만들지 않았다** — 목록은 사람이 유지해야 하고, 위 표대로 **여섯 자리는 이미 기계가 지킨다**.

**⒞ 적은 자리 — 워크로그에만 적지 않았다**:
| 자리 | 무엇을 | 왜 그 자리인가 |
|---|---|---|
| ★`CLAUDE.md` §Definition of Done | **CI 5줄 verbatim** + 「축약하지 마라」와 그 이유(S8 실사고) | ★**모든 회차가 반드시 읽는다**(DoD) |
| ★`docs/upstream-sync-approach.md` §4 뒤 | **결정문 · 돌연변이 표 · 사료 표 · 재개 조건과 세는 명령** | ★**동기 회차가 반드시 읽는 계약 파일** |

## 5. ★★재개 조건 + 세는 명령 + 오늘의 값 (계약 4)

> ★**CI 가 치는 검사 중 로컬 DoD 에 없는 것이 «1건이라도» 생기면 이 결정을 다시 연다.**

```sh
cd "$(git rev-parse --show-toplevel)"
DOD=$(sed -n '/## Definition of Done/,/^- 착수·완료마다/p' CLAUDE.md)
LC_ALL=C /usr/bin/grep -E '^[[:space:]]+- run: ' .github/workflows/rust.yml | sed 's/^[[:space:]]*- run: //' \
| while IFS= read -r c; do printf '%s' "$DOD" | LC_ALL=C /usr/bin/grep -qF -- "$c" || echo "MISSING: $c"; done \
| /usr/bin/grep -c .
```

| | 값 |
|---|---|
| 착수 시 | ★**5**(DoD 가 축약본이라 CI 5줄 «전건»이 문자열로 안 맞았고, 그중 wasm32 는 **의미로도** 없었다) |
| ★**오늘(착지본)** | ★**0** |

★**1 이상이 되면**: ⑴그 줄을 DoD 에 넣거나 ⑵넣을 수 없는 이유를 적고 **이 결정을 재검토**하라.
★**「조항이 필요하다」로 바로 가지 마라 — 그때도 «먼저 세라»**(위 §1 돌연변이 표를 다시 만들면 된다).

## 6. 계약 5 — CI 크레이트 이름 1건은 ★**이미 고쳐져 있었다**

티켓은 「이 회차에서 고쳐도 된다(XS)」고 열어 뒀으나 ★**S8 회차가 이미 고쳐 착지시켰다**
(`.github/workflows/rust.yml:55` = `--exclude test-utils`). ⇒ **이 회차는 그 파일을 안 건드렸다.**
★**그리고 티켓 경고대로 «고쳤으니 조항 결정을 건너뛴다»로 가지 않았다** — 위 §4 가 그 별도 산출이다.
★**이 회차가 한 것은 «그 사고가 로컬에서 안 잡힌 이유»를 막은 것**이고, 그 둘은 다른 층이다
(전자는 **증상 1건**, 후자는 **그 증상이 늦게 드러난 경로**).

## 7. 계약 6 — 기존 스위트 전건 · 새 red 0

★**CI 5줄을 «워크플로 문면 그대로»** 실행:

| 명령 | rc |
|---|---|
| `cargo fmt --all -- --check` | **0** |
| `cargo clippy --all -- -D warnings` | **0** |
| `cargo clippy --workspace --exclude test-utils --target wasm32-unknown-unknown -- -D warnings` | **0** |
| `cargo test --all` | **0** — ★**554 / 0 / 1** |
| `python3 scripts/check-worklog-json.py` | **0** |

★**새 red 0**(baseline **554/0/1** 동수) · ★**돌연변이는 전부 복구**했고 워킹트리는 **문서 2파일**만 바뀌었다.

## 8. 경계 준수

★**upstream 동기 회차를 «새로 열지» 않았다**(behind **0** · S9 없음) · 포크 정책 무접촉 ·
★**「목록 통째 문서화」로 시작하지 않았다**(먼저 셌다) · 머지 **0** · force-push **0** · 리베이스 **0** ·
`main` 직접 push **0** · 시크릿 출력 **0** · 코드(`.rs`) 변경 **0** · 맨 `grep` **0**(전건 `/usr/bin/grep`).
