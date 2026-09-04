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
| ⑦ | `double_must_use` allow(PR #14) | ★**9곳 «전건» 삭제** | ok | ★**beta RED**(error 7) | ok | ★★**없음 — CI 만**(② 와 같은 칸) |
| ⑧ | 워크로그 잠금 스크립트 경로(PR #15) | 스크립트 이동 | — | — | — | 있음(로컬 DoD **와** CI 둘 다 · rc=2) |

★★**[2026-09-04 정정 — 게이트② `request-changes`] 초판의 ⑦행 「깨져도 무해」는 «거짓»이다.**
근인은 ★**돌연변이를 «비하중» 자리에 넣은 것**이다 — 초판은 `interpreter.rs:1` **한 곳**만 지웠는데
그것은 **같은 파일 함수별 `#[allow]` 에 가려진 «중복»**이고 `jvm/src/jvm.rs` 에 **7곳**이 그대로 남아 있었다.
★**세 갈래 재측정**(각 회 `cargo fmt --all` 정규화 후):

| 돌연변이 | stable | ★beta |
|---|---|---|
| ⓪ 무돌연변이 | 0 | **0** |
| ① `interpreter.rs:1` 만(★초판) | 0 | **0** ← 초판 결과 재현됨 |
| ② `jvm.rs:2` 만(함수별 6 존치) | 0 | **0** |
| ★③ **9곳 전건** | 0 | ★**error 7** |

⇒ ★**⑦의 그물은 «beta clippy» 뿐이고 그것이 로컬 DoD 에 없었으므로 «CI 만» — ② 와 같은 칸이다.**

⇒ ★★**「아무것도 없음」 칸은 «2개»(② · ⑦)다.**

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
⇒ ★**조항 하나로 못 덮는다** — 덮으려 하면 **이미 그물이 있는 여섯 자리에까지 사람 확인을 얹는다**. 그것이 비용이다.
★**세는 법**(수리 전 기준): §1 돌연변이 표 **8행** − 그물이 「CI 만」인 **2행**(② · ⑦) = ★**6**.
★**[게이트② 정정] 초판의 「다섯」은 계수 «1» 시절의 파생 수다** — 계수가 **2**로 정정되며 **6**이 맞다.
(`:93` 의 「여섯 자리는 이미 기계가 지킨다」와 이제 **같은 수**다.)

## 4. ★★결정문 · 적은 «자리» · 이유 (계약 3)

> ★**동기 회차 계약에 「우리 자산 서술자 목록을 문서화하고 머지 후 재확인한다」 상시 조항을 «넣지 않는다».**
> ★**대신 «구멍 하나»를 막는다 — `CLAUDE.md` §Definition of Done 이 CI 명령 5줄 + toolchain 축 1줄 = «6줄»을 축약 없이 싣는다.**

★**[2026-09-04 정정 — 게이트②] 초판 결정문은 「그물이 없던 자리 «하나» … CI 명령 5줄」이었다** —
계수 «1» 시절 문면이다. 계수가 **2**가 되며 빠진 것은 «줄 하나»가 아니라 **두 축**(target · toolchain)이고 DoD 는 **6줄**이다.

**⒜ ★계수가 «2»인데도 왜 여전히 조항이 아닌가**(★초판 논거 「1개니까」는 계수 정정으로 **무효** — 결론은 같고 이유가 다르다):
- ★★**② 와 ⑦ 은 «다른 자산»이 아니라 «한 근인의 두 얼굴»이다 — «로컬 DoD 가 CI 매트릭스를 재현하지 않는다».**
  ②는 빠진 `- run:` 줄(**target** 축) · ⑦은 빠진 **toolchain** 축(beta).
  ⇒ ★**근인 하나를 고치면 둘 다 그물을 얻는다**(실측: DoD 에 `+beta` 를 넣자 ⑦이 로컬에서 error 7 로 잡힌다).
- ★**「조항이 «여럿»이면?」도 기각한다** — 세 방향은 잡는 그물이 각각 달라 조항이 셋이 되고,
  그중 둘은 ★**이미 기계가 지키는 자리에 사람 확인을 얹는 것**이다.
- ★**사람 기억에 맡기지 않는다** — 추가한 것은 «체크리스트 항목»이 아니라 ★**실제로 도는 명령**이다.
  회차가 DoD 를 돌리면 **cargo 가 스스로 낡음을 말한다**(`excluded package(s) … not found`).
- ★**목록을 만들지 않았다** — 목록은 사람이 유지해야 하고, 위 표대로 **여섯 자리는 이미 기계가 지킨다**.

**⒞ 적은 자리 — 워크로그에만 적지 않았다**:
| 자리 | 무엇을 | 왜 그 자리인가 |
|---|---|---|
| ★`CLAUDE.md` §Definition of Done | ★**6줄 verbatim**(CI `- run:` 5줄 + `cargo +beta` 1줄) + 「축약하지 마라」·「toolchain 축을 빼지 마라」와 그 이유(S8 실사고 · beta 실측) | ★**모든 회차가 반드시 읽는다**(DoD) |
| ★`docs/upstream-sync-approach.md` §4 뒤 | **결정문 · 돌연변이 표 · 사료 표 · 재개 조건과 세는 명령** | ★**동기 회차가 반드시 읽는 계약 파일** |

## 5. ★★재개 조건 + 세는 명령 + 오늘의 값 (계약 4)

> ★**CI 가 치는 검사 중 로컬 DoD 에 없는 것이 «1건이라도» 생기면 이 결정을 다시 연다.**

```sh
cd "$(git rev-parse --show-toplevel)"
DOD=$(sed -n '/## Definition of Done/,/^- 착수·완료마다/p' CLAUDE.md)

# ★축① — CI 의 `- run:` 줄 ∖ 로컬 DoD   (0 이어야 한다)
LC_ALL=C /usr/bin/grep -E '^[[:space:]]+- run: ' .github/workflows/rust.yml | sed 's/^[[:space:]]*- run: //' \
| while IFS= read -r c; do printf '%s' "$DOD" | LC_ALL=C /usr/bin/grep -qF -- "$c" || echo "MISSING-RUN: $c"; done \
| /usr/bin/grep -c .

# ★축② — CI 의 toolchain 매트릭스 ∖ 로컬 DoD   (0 이어야 한다)
LC_ALL=C /usr/bin/grep -E '^[[:space:]]+rust: \[' .github/workflows/rust.yml \
| sed 's/.*\[//; s/\].*//; s/, */\n/g' \
| while IFS= read -r tc; do
    [ "$tc" = stable ] && { printf '%s' "$DOD" | LC_ALL=C /usr/bin/grep -qE 'cargo (fmt|clippy|test)' || echo "MISSING-TC: $tc"; continue; }
    printf '%s' "$DOD" | LC_ALL=C /usr/bin/grep -qF -- "cargo +$tc " || echo "MISSING-TC: $tc"
  done | /usr/bin/grep -c .
```

★**[2026-09-04 정정 — 게이트②] 초판은 이 블록에 «축① 만» 실어 놓고 아래 표에서는 두 축을 말했다.**
★**정본은 `docs/upstream-sync-approach.md` §4 이고, 이제 두 파일이 같은 명령을 싣는다.**

| | 값 |
|---|---|
| 착수 시 | 축① ★**5** · 축② ★**1**(beta 축이 DoD 에 없었다) |
| ★**오늘(정정 후)** | 축① ★**0** · 축② ★**0** |

★**두 축을 «둘 다» 세라 — 하나만 0 이면 나머지 차원이 조용히 빠진다**(그것이 ⑦이 CI 에서만 났던 이유다).
★**1 이상이 되면**: ⑴그 줄/축을 DoD 에 넣거나 ⑵넣을 수 없는 이유를 적고 **이 결정을 재검토**하라.
★**OS 축(3종)은 로컬 재현 불가라 대조 대상이 아니다** — ★그 차원은 **CI 가 유일한 그물**이고 «알고 두는» 값이다.
★**「조항이 필요하다」로 바로 가지 마라 — 그때도 «먼저 세라»**(위 §1 돌연변이 표를 다시 만들면 된다).

## 6. 계약 5 — CI 크레이트 이름 1건은 ★**이미 고쳐져 있었다**

티켓은 「이 회차에서 고쳐도 된다(XS)」고 열어 뒀으나 ★**S8 회차가 이미 고쳐 착지시켰다**
(`.github/workflows/rust.yml:55` = `--exclude test-utils`). ⇒ **이 회차는 그 파일을 안 건드렸다.**
★**그리고 티켓 경고대로 «고쳤으니 조항 결정을 건너뛴다»로 가지 않았다** — 위 §4 가 그 별도 산출이다.
★**이 회차가 한 것은 «그 사고가 로컬에서 안 잡힌 이유»를 막은 것**이고, 그 둘은 다른 층이다
(전자는 **증상 1건**, 후자는 **그 증상이 늦게 드러난 경로**).

## 7. 계약 6 — 기존 스위트 전건 · 새 red 0

★**정정 후 DoD «6줄»을 «문면 그대로»** 실행(★초판은 **5줄**이었다 — `+beta` 가 빠져 있었다):

| 명령 | rc |
|---|---|
| `cargo fmt --all -- --check` | **0** |
| `cargo clippy --all -- -D warnings` | **0** |
| ★`cargo +beta clippy --all -- -D warnings` | ★**0** |
| `cargo clippy --workspace --exclude test-utils --target wasm32-unknown-unknown -- -D warnings` | **0** |
| `cargo test --all` | **0** — ★**554 / 0 / 1** |
| `python3 scripts/check-worklog-json.py` | **0** |

★**새 red 0**(baseline **554/0/1** 동수) · ★**돌연변이는 전부 복구**했고 워킹트리는 **문서 2파일**만 바뀌었다.

## 8. 경계 준수

★**upstream 동기 회차를 «새로 열지» 않았다**(behind **0** · S9 없음) · 포크 정책 무접촉 ·
★**「목록 통째 문서화」로 시작하지 않았다**(먼저 셌다) · 머지 **0** · force-push **0** · 리베이스 **0** ·
`main` 직접 push **0** · 시크릿 출력 **0** · 코드(`.rs`) 변경 **0** · 맨 `grep` **0**(전건 `/usr/bin/grep`).
