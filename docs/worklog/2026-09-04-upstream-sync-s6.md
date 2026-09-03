# [2026-09-04] upstream 동기 S6 — 컷 `95ebc5c` (regex · Formatter · Locale · 충돌 1)

**티켓**: `rustjava-upstream-sync-s6-cut-95ebc5c`
**컷**: `95ebc5c0c7560ae062a49a67e42d62563b8ee01b`(2026-08-15 · **11커밋** · 142파일 +17,593/−483)
**채택 제안**: `2026-09-04-upstream-sync-s5#p0`

---

## 1. ★착수 시 재측정 — 「새 충돌 0」을 «전제»로 쓰지 않았다

| 축 | 값 |
|---|---|
| base(착수 시 `main`) | **`a0b5d3c`** |
| `merge-base origin/main upstream/main` | **`c4665b0`** · behind **24** · 열린 PR **0** |
| **컷 `95ebc5c` 충돌** | ★**1건** — `java_runtime/src/classes/java/lang/string.rs` |

★**「재서 0 이었다」가 아니라 「재서 1 이었다」**로 적는다. 예측은 0 이었다.

### ★★그 차이가 이 회차의 «첫 산출»이다 — 예측과 실측은 «둘 다 참»이고 축이 다르다

| 무엇을 잰 수인가 | base | 값 |
|---|---|---|
| 2026-09-03 재측정의 「S6 새 충돌 **0**」 = ★**델타**(새로 «나타난» 파일 수) | `8c1238b`(merge-base `3296139c`) | 누적 `c4665b0` **3** → `95ebc5c` **3** ⇒ 델타 **0** |
| 이번 착수 재측정 = ★**누적**(그 base 에서 실제로 열리는 파일 수) | `a0b5d3c`(merge-base `c4665b0`) | ★**1** |

⇒ `string.rs` 는 S5 에서 **이미** 충돌 집합 안에 있었으므로 S6 에서 «새로» 나타나지 않았다(델타 0).
그런데 S5 착지로 base 가 옮겨간 뒤에도 **우리 쪽 분기가 남아 있다** —
`c4665b0`→`origin/main` **+8/−28**(= S5 의 설계 판단 산물: `Charset` 라우팅 유지 + upstream 표 폐기).
upstream 이 그 파일을 이 구간에 **+402/−121** 로 만졌으므로 **계속 열린다**.

★★**§5 상시 규칙에 한 줄 보탰다 — 충돌 수에는 base 와 «함께» ★«델타인가 누적인가»도 밝혀라.**
base 만 병기하면 **「0」이 「풀 것이 없다」로 읽힌다** — 이번이 정확히 그 형태였고, 티켓조차
「새 충돌 0 · 부딪힐 것이 없다」로 읽었다.

---

## 2. 충돌 1건 해소 — `string.rs` import 블록 «합집합»

충돌면은 ★**import 한 곳(hunk 1개)** 뿐이었다:

| | 내용 |
|---|---|
| HEAD(우리) | `charset::Charset,` + `classes::java::lang::{Object, System},` |
| upstream `95ebc5c` | `classes::java::{ lang::{Object, System}, util::{ Formatter, Locale, regex::{Matcher, Pattern} } },` |

⇒ ★**합집합**: 우리 `charset::Charset` 을 **유지**하고 upstream 의 **재구조화된 `classes` 블록을 채택**했다.
★**한쪽 통째 채택이 아니다** — `--theirs` 였으면 `charset::Charset` 이 사라져 아래 4곳이 컴파일 실패한다.

**검증**(해소가 의미를 지켰는가):
- `Charset::from_name`(`:340`·`:536`) · `Charset::resolve`(`:888`·`:931`) ★**4곳 생존**
- ★**S5 가 버린 `decode_str`/`encode_str` 재유입 0건**(upstream 이 또 들고 오지 않았다)

---

## 3. ★★계약4⒝ — 「충돌 0으로 들어온」 파손 1건을 «테스트를 돌리기 전에» 잡았다

### 정독 대상을 «좁히는» 방법 (142파일을 다 읽지 않았다)

union 위험은 ★**양쪽이 «둘 다» 만진 파일**에만 있다:

```
git diff --name-only <merge-base> origin/main | sort  →  우리가 만진 49
git diff --name-only <merge-base> 95ebc5c    | sort  →  upstream 이 만진 142
comm -12  →  ★교집합 9건 = 정독 대상
```

| 교집합 파일 | 정독 결과 |
|---|---|
| `string.rs` | 충돌로 이미 처리(§2) |
| `AGENTS.md` | 우리 `Git Workflow`·`Round Worklog` 절 **+56/−0 생존** |
| `java_runtime/Cargo.toml` | ★`tracing-attributes` 핀 **부재 유지**(PR #4 자산) |
| `jvm/src/jvm.rs` | `setProperty` 서술자 **`String` 유지** + `double_must_use` allow **6곳 생존** |
| `test_print_stream`·`test_print_writer`·`test_boolean` | S5 의 서술자 처분 **+3/−3 · +1/−1 · +2/−2 생존** |
| `test_string.rs` | 우리 전용 2건 **+41/−0 생존** |
| `Cargo.lock` | §4 |

### ★그런데 «교집합 밖»에서 파손이 나왔다 — 신규 파일이라 교집합에 들지 않는다

`java_runtime/tests/classes/java/util/regex/test_pattern_syntax_exception.rs` —
★**`origin/main` 에 없고 `95ebc5c` 에 있다 = upstream 이 이 구간에 «새로» 넣은 파일**.
그 안에서 `java/lang/System.setProperty` 를 **`)Ljava/lang/Object;`** 로 **3곳** 부른다.
우리는 PR #5 에서 **JDK 규격대로 `)Ljava/lang/String;`** 으로 고쳐 뒀다 ⇒ 그대로 두면 `NoSuchMethodError`.

★★**신규 파일이라 «충돌이 날 수가 없다»** — `merge-tree` 도 교집합 대조도 원리적으로 못 본다.
찾은 방법은 ★**「우리 자산 서술자를 전 테스트 트리에 대고 다시 grep」** 이었다:
```
awk '/"java\\/lang\\/System"/{sys=NR} /"setProperty"/{sp=NR} /Ljava\\/lang\\/Object;/{ … }'
```
⇒ **서술자만 `String` 으로 맞췄다 — 3곳**(값은 `let _:` 로 버려져 바인딩 타입 무접촉).
★`java/util/Properties.setProperty` 의 `Object` 반환은 **JDK 규격상 옳아 무접촉**(7곳 그대로).

★★**이 형태는 이번이 «세 번째»다 — 그래서 별 축으로 올린다**:
**S3** `tests/test_class_format.rs` 문구 단정 3건 → **S5** io 테스트 **5곳** → ★**S6 regex 테스트 3곳.**
⇒ ★**「우리가 JDK 규격에 맞춘 것 ↔ upstream 이 안 맞춘 것」이 매 회차 «새 파일»로 재유입된다.**
S5 워크로그 `proposals[1]`(「충돌 목록에 없는 파손을 잡는 축을 계약에 넣을지 판정하라」)이 **아직 미처분**이다.

---

## 4. 계약4⒞ — `Cargo.lock` 이 조용히 버전을 «내리지» 않았는가

★**S5 를 문 자리다**(그때 `async-trait` 이 0.1.92 → 0.1.91 로 내려가 beta clippy 3셀이 red 였다).
⇒ **이번엔 머지 직후 바로 쟀다**:

| 축 | 값 |
|---|---|
| ★**내려간 크레이트** | ★**0건** |
| `async-trait` | **0.1.92 유지**(★upstream `95ebc5c` 의 lock 도 이미 **0.1.92** — 이번엔 하강 압력 자체가 없었다) |
| 올라간 | `event-listener` 5.4.1→5.4.2 · `regex-automata` 0.4.14→0.4.16 · `regex-syntax` 0.8.10→0.8.11 |
| 추가 / 제거 | `regex` / `concurrent-queue`·`crossbeam-utils` |

★**그리고 beta 축을 «push 전에» 직접 돌렸다**(S5 는 CI 가 알려 줬다) — 아래 §5.

---

## 5. 계약5 — 기존 축 전건 + S4·S5 축 생존

| 명령 | stable | ★beta |
|---|---|---|
| `cargo fmt --all -- --check` | **0** | — |
| `cargo clippy --all -- -D warnings` | **0** | **0** |
| `cargo clippy --workspace --exclude test_utils --target wasm32-unknown-unknown -- -D warnings` | **0** | **0** |
| `cargo test --all` | **0** — ★**554 / 0 / 1** | **0** — ★**554 / 0 / 1** |
| `python3 scripts/check-worklog-json.py` | **0** | — |

★**새 red 0**: 착수 baseline **427/0/1** → 착지 **554/0/1**(**+127**).
S1 169 → S2 191 → S3 216 → S4 261 → S5 427 → ★**S6 554**.

**S4·S5 축 생존 실측**:

| 축 | 결과 |
|---|---|
| `tracing::instrument` 실사용 | ★**0**(`thread.rs:267` **주석 1건**뿐) · 수동 span(`info_span!`+`.instrument`) **2곳 생존** |
| `tracing-attributes` | **0**(`Cargo.lock`·`Cargo.toml`·`java_runtime/Cargo.toml`) |
| `charset.rs` 단일 출처 | **실재** · `Charset::` 호출부 **4곳** |
| 픽스처 `TimeApi`·`UnsupportedCharset` | **4파일** |
| `tests/test_class_format.rs` | **4/4** |
| ★**S4 의 500→2000ms 여백** | ★**0곳 — «없는 것이 정상»이다**(아래) |

★★**티켓 계약5 의 예시(「예: `test_timer.rs` 의 500→2000ms 여백」)는 «낡았다» — 정직하게 적는다.**
그 여백은 ★**S5 착지로 사라졌다**: upstream 이 벽시계 테스트 2건을 **manual clock 기반 결정성 스위트 12건**으로
**대체**했고(§5 S5 착지 기록), 되얹을 자리가 없어 `upstream 채택`했다. 현재 `timer_*` **12건**이 그것이다.
⇒ ★**「여백이 0곳」은 회귀가 «아니라» S5 의 확정된 처분이다.** 그 근거(왜 2000 이었나)는 §5 에 인용으로 보존돼 있다.

**「해소분 0」 증명**: `95ebc5c` 대비 ★**삭제 파일 0건** · 다른 파일 **50건 전수가 우리 fork 고유 자산**.

---

## 6. 계보 — 이 repo 의 «본체»

| 축 | 착지 전 | 착지 후 |
|---|---|---|
| `merge-base origin/main upstream/main` | `c4665b0` | ★**`95ebc5c`** |
| behind | **24** | ★**13** |
| 머지커밋 부모 | — | ★**2개**(`a0b5d3c` + `95ebc5c`) |

★**`-s ours` 는 쓰지 않았다** — `merge-base` 가 이미 `c4665b0` 로 서 있어 복원할 것이 없었다(S5 부터 2회 연속).
★★**게이트③이 `--squash` 면 이 전진이 통째로 사라진다** — `<id>-merge` 에 **`merge_strategy: merge` 필수**.

---

## 7. 경계 준수

머지 **0**(PR 제출까지) · 새 PR **1**(base `main` · 스택 아님) · force-push **0** · 리베이스 **0** ·
`main` 직접 push **0** · ★**S7 흡수 0**(`merge-base` 가 `95ebc5c` 에서 멈춘 것이 그 증거) ·
upstream 코드 «개선» **0**(수렴만 — 서술자 3곳은 «우리 자산에 맞춤»이고 S5 의 확립된 처분이다) ·
★**S8 신설 지시(`2026-09-03-…#p0`)는 집행하지 않았다**(총괄 보류분) ·
§5 재측정 표 **재작성 0**(착지 기록 절만 신설) · upstream 발신 **0** · 맨 `grep` **0**(전건 `/usr/bin/grep`).
