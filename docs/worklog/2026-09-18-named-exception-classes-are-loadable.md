# 이름으로 부르는 예외 클래스가 «실을 수 있는» 것인가 — 한 자리에서 대조한다

채택 제안 `2026-09-17-string-concat-recipe-arity#p0`(운영자가 tower 「추천 후속 작업」에서 채택).

## ⓐ 주장을 «내가» 먼저 확인했다 — 재현된다

제안의 주장: 「런타임이 자기가 갖고 있지 않은 예외 클래스를 올리려다 **throw 가 아니라 panic** 했다」.

**기전은 코드에 그대로 있다**(`jvm/src/jvm.rs:943-950`):
```rust
pub async fn exception(&self, r#type: &str, message: &str) -> JavaError {
    let instance = self.new_class(r#type, "(Ljava/lang/String;)V", (message_str,)).await.unwrap();
    //                                                                              ^^^^^^^^ 
```
`new_class` 는 `Result` 를 돌려주고, 부트스트랩 로더가 이름을 못 풀면 `Err` 다 ⇒ ★**`.unwrap()` 이 패닉한다.**
Java 예외가 되는 것이 아니라 **프로세스가 죽는다.**

★**그리고 이 축은 «자기 참조»다** — `jvm/src/jvm.rs:842` 가 클래스 부재를 보고하는 방식이
`return Err(self.exception("java/lang/NoClassDefFoundError", class_name).await);` 다.
⇒ **오류 경로 자신의 클래스가 실리지 않으면 «보고»가 패닉한다.**

**실을 수 있는 집합의 정본**은 `rustjava-runtime/src/loader.rs` 의 `protos` 배열이다 —
`get_runtime_class_proto` 가 `protos.into_iter().find(|proto| proto.name == name)` 로 고르고,
부트스트랩 로더(`load_class` → `find_rustjar_class(RT_RUSTJAR, …)`)가 결국 그 함수에 닿는다
(`src/runtime.rs:173-174` · `test-utils/src/lib.rs:318-319`).

## 착수 시점 실측

| 축 | 값 |
|---|---|
| `exception(` 호출부의 리터럴 클래스명 | ★**43 고유** · 호출부 **846** (초판 41/812 — 다중 줄 34건을 잃었다) |
| `loader.rs` 등재 | ★**268 항목**(`as_proto` **265** + `list_proto` **3**) (초판은 `as_proto` 만 봐서 265) |
| 등재 타입 → `name:` 해석 | **265/265 성공**(해석 실패 0) |
| ★**못 싣는 이름(A − B)** | ★**0 — 베이스라인이 이미 0이다** |

★**제안의 「72 java/javax class names」와 내 41 은 다르다.** 내 술어는 ★**`exception(` 첫 인자의 리터럴**만
세고 **고유**로 센다 — 72 는 다른 술어(호출부 총계나 `new_class` 포함)로 보이나 **그 값을 재현하지 못했다.**
★**그래서 인용하지 않고 내 수를 적는다.** 제안의 「baseline is now 0」은 **재현된다.**

★★**[정정 — 게이트² F3] 초판이 적은 「정의됐지만 «등재되지 않은» 이름 5건」은 «틀렸다». 실제 미등재는 «0» 이다.**
3건(`ListItr` 무리)은 `list_proto()` 로 등재돼 있는데 초판 정규식이 `as_proto` 만 봤고,
2건(`logging/Formatter`·`org/rustjava/net/JarURLConnection`)은 짧은 이름 충돌로 다른 쪽에 가려졌다.
아래는 그 «틀린» 목록이다(사료로 남긴다):
`java/util/AbstractList$ListItr` · `java/util/ArrayList$ListItr` · `java/util/Vector$ListItr` ·
`java/util/logging/Formatter` · `org/rustjava/net/JarURLConnection`.
⇒ ★**그래서 검사기는 «`name:` 리터럴 전수»가 아니라 «등재분»을 실을 수 있는 집합으로 쓴다** — 그쪽이 진짜다.

## 무엇을 만들었나 — 대조 검사 «한 자리»

`scripts/check-named-exception-classes-are-loadable.py` (DoD 7번째 명령 · CI job `named_exception_classes`).
★**「그 한 클래스를 추가한다」로 끝내지 않았다** — 어느 쪽 집합이 늘어도 이 한 자리가 문다.

## ★못 보는 것 — 「증명」이 아니라 «바닥»이다

- ★**런타임에 만들어지는 이름**(`format!` · `const` · 변수 · `match` 팔이 돌려주는 `&str`)은 호출부에
  리터럴이 아니므로 **보이지 않는다.** 이것이 가장 큰 구멍이고, 제안 자신도 `tradeoff` 에 그렇게 적었다.
- ★**`exception(` 만 훑는다.** `new_class(`·`find_class(` 로 직접 부르는 이름은 범위 밖이다 —
  그쪽은 `Result` 를 호출자에게 돌려주지 «언래핑하지 않으므로» 패닉 축이 아니다.
- ★**등재된 프로토가 런타임에 «초기화»에 실패하는 경우는 통과한다** — 이름의 해석 가능성만 본다.
- ★**이름은 적힌 그대로 대조된다** — 다른 실재 클래스와 우연히 일치하는 오타는 통과한다.

## ★양방향 — 개악이 «반드시» red 다

| 개악 | 결과 |
|---|---|
| ⒜ **현 상태** | `✓ 43 named … all 268 loadable` **rc=0** (초판 41/263) |
| ⒝ ★**⑶ 실제 사례 재현** — `loader.rs` 에서 `BootstrapMethodError` 등재 1줄 제거 | **rc=1** · `✗ java/lang/BootstrapMethodError — named at jvm-bytecode/src/interpreter.rs:1109` |
| ⒞ 프로토 `name:` 을 오타로(등재는 유지) — `NullPointerException` | **rc=1** · `✗ java/lang/NullPointerException — named at …:85 and 320 more` |
| ⒟ ★**fail-closed** — 해석 불가 등재 항목 추가 | **rc=2** `cannot measure: registered entries with no resolvable name: java::lang::NoSuchTypeHere` |
| ⒠ 전건 복구 후 | **rc=0** |

★⒟ 가 이 설계의 요지다 — 등재 항목을 이름으로 되짚지 못하면 **「실을 수 있는 집합」을 조용히 줄여 false red** 를 낸다.
그래서 **줄이지 않고 «못 쟀다»(2)** 로 끝낸다.

★**배선 자체도 양방향으로 쟀다**: CI step 을 지우면 `check-dod-ci-parity.py` 가 **rc=1**(`FAIL 대칭차 있음: 축 A`),
되돌리면 **rc=0**(`명령 7개 · toolchain 2개`).

## 시간

**0.80s · 0.98s · 0.91s**(3회 · 부하 중). ★초판은 **32.7초**였다 — `rglob` 이 `target/` 를 걸어서다.
최상위에서 `target`·`.git` 을 **가지치기**해 그 비용을 없앴다(그 이유를 코드 주석에 남겼다).

## 잃는 것 — 「없다」로 적지 않는다

- ★**DoD 명령이 6 → 7 로 는다.** 로컬 회차마다 ~1초가 붙고, `dod_parity` 가 그 대칭을 **함께** 지고 간다
  (제안이 `tradeoff` 에 적은 그 대가다 — 실측치는 1초이지 32초가 아니다).
- ★**바닥이라 «안심»을 준다** — 리터럴만 보므로 「검사기가 green 이니 패닉이 없다」는 **거짓**이다.
  위 「못 보는 것」 첫 항이 정확히 그 구멍이고, 그것을 닫으려면 **호출부 쪽 표현을 리터럴로 묶는 별 축**이 필요하다.
- ★**패닉을 throw 로 바꾸지 않았다** — 이 회차는 「못 싣는 일이 없게」만 한다. `.unwrap()` 은 그대로 있고,
  위 구멍으로 새는 이름이 생기면 **여전히 패닉한다.** 그 전환은 별 축이다(범위 밖 · 제안 `#p1` 과도 다르다).
- ★**「등재 미비 5건」은 애초에 없었다**(위 정정) — 초판 검사기의 결함이 만든 허상이고, 고칠 것이 없었다.

## 후속 추천

⑴★**런타임 조립 이름을 잡는 축**(M) — 위 구멍의 정면. `exception(` 첫 인자가 리터럴이 «아닌» 호출부를
   세어 그 수를 먼저 재고, 리터럴 강제(상수 테이블)가 값하는지 판단한다. ★지금은 **그 수조차 모른다.**
⑵★**`Jvm::exception` 의 `.unwrap()` 을 throw 로 바꾸는 축**(M) — 이 회차가 명시적으로 **범위 밖**으로 둔 것.
   바닥 검사가 있어도 마지막 방어선은 여전히 없다.


---

## ★★[게이트² 반려 승계] 결함 셋 — 둘이 «거짓 초록», 하나가 «거짓 빨강»

초판 검사기는 «베이스라인 0» 이라 ★**거짓 초록 = 검사기 부재**다. 셋 다 내가 재현하고 고쳤다.

### F1 — 짧은 이름 충돌이 «진짜 미등재»를 가렸다(거짓 초록)
`name_of` 를 **타입 이름만**으로 키잉하고 `setdefault`(첫 파일 승)를 써서, 충돌쌍의 한쪽이 다른 쪽을 대신 답했다.
★**이 트리의 충돌쌍은 둘**: `Formatter`(`java/util` ↔ `java/util/logging`) · `JarURLConnection`(`java/net` ↔ `org/rustjava/net`).
```
개악: java::util::Formatter 등재 1줄 제거 + exception("java/util/Formatter", …)
  전  ✓ 42 named exception class(es) across 813 call site(s); all 263 loadable   rc=0   ← ★거짓 초록
  후  ✗ java/util/Formatter — named at …                                          rc=1
둘째 쌍도 같다: java::net::JarURLConnection 제거 → ✗ java/net/JarURLConnection   rc=1
```
★**처방**: 키를 ★**(모듈 경로, 타입, 함수)** 로. 모듈 경로는 파일 위치에서 그대로 나오고, 그것이 `loader.rs` 가 적는 `crate::classes::…` 와 같은 축이다.

### F2 — 여러 줄에 걸친 `exception(` 을 못 봤다(거짓 초록)
`named_classes()` 가 **한 줄씩** 훑어, 정규식의 `\s*` 가 개행에 닿지 못했다(rustfmt 가 긴 호출을 쪼갠다).
```
개악: 다중 줄 exception( 에 미등재 이름을 넣는다
  전  ✓ 41 named exception class(es) across 812 call site(s); all 263 loadable   rc=0   ← ★안 보인다
  후  ✗ java/lang/NoSuchClassHereError — named at …:2                            rc=1
```
★**처방**: 파일 전체를 훑고 줄 번호는 매치 오프셋에서 역산한다(보고 위치는 그대로 유지).

### F3 — `list_proto()` 등재가 안 보였다(거짓 빨강 + 사실오류)
`REGISTERED` 가 `::as_proto\(\)` 만 잡아 **3건**(`AbstractListItr`·`ArrayListItr`·`VectorItr`)을 놓쳤다.
★**그래서 초판이 「정의됐지만 미등재 5건」이라 «기록»한 것이 틀렸다** — 위 정정 절 참조(실제 미등재 **0**).
※근인이 하나 더 있다: 한 `impl` 블록에 proto 생성자가 **둘**이면(`AbstractListItr::as_proto` → `…$Itr` ·
`::list_proto` → `…$ListItr`) 첫 `name:` 만 집어 **엉뚱한 클래스를 귀속**시켰다 ⇒ (모듈, 타입, **함수**) 키가 그것도 함께 고친다.

## ★ⓑ 세 수 — «술어를 밝혀» 내가 각각 셌다
| 수 | 값 | 술어 |
|---|---|---|
| `protos[]` 항목 | **268** | `let protos = [` ~ `];` 사이에서 `crate::` 로 시작하는 줄 |
| 그중 비 | `as_proto` **265** · `list_proto` **3** · 그 밖 **0** | 같은 구간에서 함수명으로 분류 |
| `.exception(` 총계 | **846** | 리터럴 `.exception(` 출현 · 인자 무관 |
| 〃 중 첫 인자가 java/javax 리터럴 | **846** | 검사기의 `NAMED` 정규식을 **파일 전체**에 적용 |
| 〃 중 개행을 사이에 둔 것 | **34** | `exception\(\s*\n\s*"…"` |
★**846 − 34 = 812** ⇒ ★**검수자의 846 과 초판의 812 는 «다른 술어»가 아니라 «F2 그 자체»였다.** 그것이 그 차이의 전부다.

## ★ⓒ F2 를 고치면 «새 red» 가 나는가 — **편집 «전»에 쟀다**
늘어난 이름은 **2개**(`java/lang/InstantiationError` · `java/lang/UnsupportedClassVersionError`)이고
★**둘 다 등재돼 있다** ⇒ ★**새 red 0 · rc=0 유지.** ⇒ 멈출 사유가 없어 진행했다(티켓 ⓒⅰ 갈래).

## 수의 전/후 — ★왜 달라졌는지
| 축 | 전 | 후 | 왜 |
|---|---|---|---|
| 이름(고유) | 41 | **43** | F2 — 다중 줄 호출 34건이 보이게 됐다 |
| 호출부 | 812 | **846** | 〃 |
| loadable | 263 | **268** | F3(+3 `list_proto`) · F1((모듈,타입,함수) 키로 충돌쌍 양쪽이 각각 해석 +2) |
★**loadable 이 «늘었다»는 것은 검사가 느슨해진 것이 아니다** — 종전에는 **265 등재를 263 으로밖에 해석 못 해**
그 차이가 «없는 이름을 있다고» 하거나 «있는 이름을 없다고» 하는 두 오류로 나타났다.

## 시간 — ★**유의한 증가 없음**(단정하지 않는다)
`real` ×3: 전 **1.42 / 1.67 / 1.46** ↔ 후 **2.01 / 1.69 / 1.19**. ★**구간이 겹친다** — 이 머신에서는
부하가 판본 차이를 덮는다(이 저장소가 같은 축에서 이미 배운 것). ⇒ 「늘지 않았다」가 아니라 **「유의하게 늘지 않았다」**로 적는다.

## 이 승계 회차에서 잃는 것
- ★**검사 대상이 늘었다**(812 → 846) — 앞으로 다중 줄 호출이 새로 생기면 **더 자주 red 가 날 수 있다**.
  ★**그것은 실패가 아니라 «검사기가 드디어 보이게 된 것»**이다. 다만 그 red 를 처음 보는 사람에게는 새 비용이다.
- ★**해석 로직이 복잡해졌다** — 키가 1튜플에서 3튜플이 되고 `impl` 블록을 함수 단위로 쪼갠다.
  ⇒ ★**파싱으로 재유도하는 방식 자체의 취약함**이 드러났다(결함 3건이 전부 그 자리였다) — 후속 카드로 냈다.
- ★**여전히 «바닥»이다** — 런타임 조립 이름·`new_class(` 경로는 그대로 안 보인다(초판의 한계 그대로).
