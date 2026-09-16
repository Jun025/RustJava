# 2026-09-17 — `test-data` 전체의 클래스 파일 버전을 «동결»한다

티켓 `rustjava-adopt-indy-fixture-jdk-pin-and-slot-accounting-p0` — 채택 제안
`2026-09-16-indy-fixture-jdk-pin-and-slot-accounting#p0`.

★**제안의 목적은 샀고, 제안이 「the work」라 부른 비용은 «치르지 않았다».**

★★**[정정 2026-09-17 · 게이트② request-changes 승계] 아래 설계는 그대로 옳지만 «키를 만드는 두 줄»이 윈도우에서 틀렸고,
형제 `#55` 와의 착지 상호작용을 아무도 적지 않았다 — 전말은 맨 아래 §정정 절에 있다.**

## ⓐ 제안이 «지금도» 참인가 — 참이다. 단 수는 움직였다

제안의 수: 루트 `65×61 · 52×40 · 66×8 · 70×3 · 68×1` · `ldc` `55×4 · 52×7`.
★**착수 시 재측**(그 회차 이후 픽스처가 늘었다):

| 디렉터리 | 지금 |
|---|---|
| `test-data/`(루트) | **52×40 · 65×62 · 66×8 · 68×1 · 70×3** |
| `test-data/ldc` | **52×7 · 55×6** |
| `test-data/indy` | **52×1 · 65×6** |
| `cp` · `dispatch/*` · `loader` · `src/jar` | 52×3 · 52×10 · 52×2 · 65×1 |

**총 150 클래스**. ⇒ 「다섯 버전이 섞여 있다」는 **여전히 참**이고, ★**65 가 61→62 로 는 것은 이 리니지가 그 사이 픽스처를 더했기 때문**이다.

## ⓑ 이미 핀하는 축 — 둘 있다. 그리고 «그 둘이 못 덮는 것»이 이 회차의 대상이다

1. `tests/test_fixture_pins.rs` — **`test-data/indy` 의 javac 산출물**만(`.java` 짝이 있는 것) `65.0` 으로 핀.
2. **생성기 코드** — `make_ldc_fixtures.py` 는 `major=52` 기본값 + 항목별 오버라이드(55)를 **소스에 박아** 둔다.
   `cp`·`indy` 합성분도 같다. ⇒ 그 **16개**는 재생성해도 버전이 안 흔들린다(생성기를 고치면 diff 에 보인다).

⇒ ★**남는 것은 «javac 로 손수 재컴파일할 수 있는» 132개**다(`.java` 소스 보유 · 실측).
★**그 132개가 정확히 사고가 일어난 자리**다 — 루트의 66×8·68×1·70×3 이 그 지문이다.

## ⓒ 제안의 «비용 산정»을 바꿨다 — 기각이 아니라 «설계 교체»

제안의 tradeoff: 「pinning means first **deciding the intended target** per fixture or per directory —
and any fixture **recompiled** to match would change bytes … **The decision is the work**; the assertion is ten lines.」

★**그 문장은 «통일»을 전제한다.** 그런데 제안이 산 것으로 적은 이득은
「A regenerated fixture **cannot quietly start testing a different Java version's** bytecode shapes」 —
즉 ★**«드리프트 탐지»**이지 «버전 통일»이 아니다.

⇒ ★★**목표를 «동결»로 바꾸면 그 「work」가 통째로 사라진다**:
**지금 값을 그대로 기록하고, 혼자 움직이면 red.** 재컴파일 **0** · 바이트 변경 **0** ·
「어느 타깃이 옳은가」라는 **결정 자체가 불요**다.
★**대가는 정직하게 적는다**: 이 축은 ★**「버전이 섞여 있다」를 «고치지» 않는다.** 섞인 채로 **굳힌다**.
그것이 옳은 이유는 — 통일하려면 재컴파일해야 하고, ★**재컴파일은 바이트를 바꾸며 그 픽스처가 «무엇을 시험하는지»를 바꿀 수 있다**(제안 자신의 경고다).
⇒ ★**섞임은 «관측된 사실»로 표에 남고, 그 사실이 보이는 자리에 놓인다.**

## 만든 것

- **`test-data/class-file-versions.txt`** — 150행의 `<major>.<minor> <경로>` + 머리 주석
  (★「이것은 «타깃»이 아니라 «동결»이다」·왜 통일하지 않는가·무엇을 잡는가·어떻게 갱신하는가).
- **`test-data/src/record-class-file-versions.py`** — 표 재기록기. ★머리 주석을 **보존**하고(없으면 **거부**),
  docstring 이 ★**「실패를 잠재우려고 돌리지 마라 — 왜 움직였는지 보고, 재컴파일과 «같은 커밋»에 표를 넣어라」**를 못박는다.
- **`tests/test_fixture_pins.rs`** — 새 테스트 `committed_fixtures_keep_their_recorded_class_file_version`.

## ★세 방향을 «전부» 검사한다 — 이것이 이 설계의 급소다

| 방향 | 없으면 생기는 구멍 |
|---|---|
| 기록과 **다르다** | 드리프트를 못 잡는다(본래 목적) |
| **기록에 없는** 픽스처가 있다 | ★**핀이 «옵트인»이 되어 내일 추가되는 픽스처는 조용히 미보호** |
| **파일이 없는** 기록이 남았다 | 표가 썩어 「무엇이 보호되는지」를 아무도 못 믿는다 |

★**둘째가 특히 중요하다** — 이 저장소가 반복해 잡은 「통과하지만 아무것도 재지 않는」 형태의 정확한 변종이다.

**개악 대조(3방향 · 전건 red · 복원 green)**
- **M1** `Hello.class` 의 버전 바이트만 `65 → 70`(= 다른 JDK 로 재생성한 것과 **같은 형상**) → ★**red**:
  `Hello.class: recorded 65.0, found 70.0` + 「재컴파일이 의도였다면 기록기를 돌려 표를 «같은 커밋»에 넣어라」
- **M2** 기록 없는 새 `.class` 투입 → ★**red**(`fixtures with no recorded version: [...]`)
- **M3** 표에만 있는 유령 항목 추가 → ★**red**(`the table records fixtures that are no longer here: [...]`)
- 복원 → **green**(`test_fixture_pins` **2 passed**)

★기록기 **멱등** 확인: 다시 돌려도 표가 **바이트 동일**.

## 잃는 것 / 안 하면 무엇이 나쁜가

⒜**잃는 것**: ⑴**150행짜리 데이터 파일**이 는다 — 픽스처를 **의도적으로** 재컴파일할 때마다 **한 줄을 같이 고쳐야** 한다
(★그것이 비용이자 «목적»이다: 그 한 줄이 리뷰에 보이는 유일한 흔적이다) ⑵`cargo test --all` **572 → 573**(테스트 1개) ·
`test_fixture_pins` 소요 **0.04s**(파일 150개의 8바이트만 읽는다 — 파싱 없음) ⑶★**«섞여 있음»을 고치지 않는다**(위 ⓒ).
⑷★**생성기 산출물 16개는 «이중 잠금»이 된다**(생성기 코드 + 표) — 규칙을 단순하게 유지하려고 **일부러** 전건을 넣었다.
⒝**안 하면**: 제안이 적은 그 사고가 계속 가능하다 — `javac Switch.java` 를 `--release` 없이 돌리면 **오늘의 JDK(70)** 가 찍히고,
★**이진 파일 안 2바이트 변화는 리뷰에서 아무도 못 본다.** ★그리고 그것은 가정이 아니다 — ★**루트의 66·68·70 이 그 사고가 «이미 일어난» 흔적**이다.

---

# §정정 (2026-09-17 · `rustjava-adopt-indy-fixture-jdk-pin-and-slot-accounting-p0-fix`)

## ① 경로 구분자 — 윈도우에서 하위 36개가 전부 어긋났다

★**설계가 아니라 «키를 만드는 두 줄»이 틀렸다.** 표는 `indy/StringConcat.class` 를 담는데
윈도우의 `to_string_lossy()` 는 `indy\StringConcat.class` 를 만든다 ⇒ ★**하위 디렉터리 36개가
«미기록»과 «유령 기록» 양쪽에 «동시에» 걸린다.** ★**루트 114개는 통과**하므로 mac/linux 에서는 **절대 보이지 않는다**
— `rust_ci (windows-latest, {stable,beta})` 만 잡았다(모수 실측: **150 = 루트 114 + 하위 36**).

★★**같은 결함이 «기록기»에도 있었다** — `record-class-file-versions.py` 가 윈도우에서 **역슬래시 행을 써 넣는다**.
★**한쪽만 고치면 방향만 바뀐다**: Rust 만 고치면 이번엔 **윈도우에서 기록한 표가 mac/linux 를 red** 로 만들고,
★**기록기는 CI 가 돌리지 않으므로 그 축은 «영영 조용하다».** ⇒ **두 곳을 같이 고쳤다**
(Rust: 구분자 정규화 · Python: `as_posix()`).

★**표 자체는 이미 슬래시였다**(역슬래시 행 **0** · 실측) ⇒ 정규화만으로 맞는다.

## ② 형제와의 상호작용 — «코드»가 아니라 «순서»였고, 그 사이 순서가 «정해졌다»

`#55` 는 `test-data/indy/` 에 `.class` **3개를 추가**하면서 표를 만지지 않는다 ⇒ **텍스트 충돌은 0인데
나중에 착지하는 쪽이 main 을 red** 로 만든다. ★**이것은 설계가 «의도한» 동작이다**(「기록에 없는 픽스처 = red」가 이 축의 목적).

★**그 사이 `#55` 가 먼저 착지했다**(`4b89f18`) ⇒ ★**순서 문제는 «해소된 형태»로 나타났다**:
base 를 당기니 그 3개가 **정확히 red 로 잡혔고**(`fixtures with no recorded version: [NotFactoryDescriptor ·
NotInvokeStaticFactory · NotMakeConcatWithConstants]`), ★**고친 기록기로 재기록하니 3행이 더해지고 그 외는 무변**
(150 → **153** · diff 가 정확히 3줄). ⇒ ★**기계가 설계대로 «잡고», 처방이 «한 명령»이었다는 실증이다.**

★**남은 형제 실측**(이 브랜치 기준): `#56`·`#58` 과의 충돌은 **`REPORT.md`·`STATE.md` 뿐**이고
★**둘 다 `.class` 를 만지지 않는다**(`#56` test-data 접촉 0 · `#58` 은 `test-data/src/verify-javac-fixtures.sh` 스크립트뿐)
⇒ ★**표와 상호작용하지 않는다.** 착지 순서 제약 **없음**.

## ③ 형제 식별 오기 — `STATE.md` 에 적힌 「#55 는 원장 3파일만」

★**거짓이었고, 무해하지 않았다.** 그 한 줄 때문에 ②의 상호작용이 **보이지 않았다**.
⇒ 그 자리에서 정정했다(원문은 남기고 아래에 반증을 붙였다). ★**교훈: 「어느 파일을 만지나」는 «PR 파일 목록»으로 확인하라.**

## 이 설계의 «본래 대가»를 흐리지 않는다 — 그리고 이제 «적어 뒀다»

「기록에 없는 픽스처 = red」는 ★**의도된 엄격함**이고, 그래서 ★**픽스처를 더하는 모든 회차가 표를 함께 만져야 한다.**
그 비용이 **어디에도 적혀 있지 않았다**(실측: `AGENTS.md` 에 관련 문장 0) ⇒ ★**`AGENTS.md` §Testing Boundaries 에 한 줄**로 남겼다.

## 검증

- ⒝⑴ **Rust**: `table_key()` 를 함수로 빼고 단위로 시험 — `dispatch\base\PackageBase.class` → `dispatch/base/PackageBase.class`.
- ⒝⑵ **Python**: 윈도우 없이 잰다 — `PureWindowsPath` 로 `str()` 은 `dispatch\base\…`, `as_posix()` 는 `dispatch/base/…` 임을 mac 에서 실증.
- ⒞ **개악**: 정규화 한 줄을 되돌리면 ★**mac 에서도 그 단위 시험이 진다**(`table_keys_use_forward_slashes_on_every_platform` FAILED) · 복원 green.
  ★**그래서 «문자열 치환»으로 구현했다** — `components()` 로 join 하면 Unix 에서 역슬래시 입력이 그대로 통과해 ★**개악이 mac 에서 안 잡힌다.**
  ★대가: Unix 파일명에 진짜 역슬래시가 있으면 바뀐다(픽스처엔 없고 표는 우리 것이다).
- ⒠ `cargo test --all --no-fail-fast` **0 failed**(수는 아래 본문).
