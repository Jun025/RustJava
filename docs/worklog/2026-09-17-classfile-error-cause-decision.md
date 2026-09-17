# 2026-09-17 — `ClassFileError` 가 «원인»을 실어야 하는가 — 판정

티켓 `rustjava-adopt-cp-tag-passthrough-detectable-p1` — 채택 제안 `2026-09-16-cp-tag-passthrough-detectable#p1`.
낱말이 **`Decide`** 다. ★**코드 변경은 «틀린 주석 한 곳» 정정뿐**이고 산출물은 **판정과 그 근거**다.

## 판정

> ★**할 값이 있다. 단 제안이 적은 이름·이유·범위가 «셋 다» 틀렸으므로 그대로 집행하면 안 된다.**
> ★**이 회차에서 구현하지 «않는다»** — 제대로 하려면 `validate_class` 의 8항 `||` 사슬을 쪼개야 하고,
> 그것은 이 티켓이 **명시적으로 금지한 리팩터**다(계약 3). ⇒ **범위를 바로잡아 후속으로 넘긴다.**

## ⓐ 제안이 «지금도» 참인가 — 현상은 참이다

`classfile/src/error.rs` 는 지금도 **2변형**뿐이다(`InvalidFormat` · `UnsupportedVersion(u16)`),
그래서 모든 파스·검증 실패가 **같은 문장**으로 나온다. 실측: `tests/test_class_format.rs` 에서
★**`ClassFormatError` «종류만» 단언하는 자리가 8곳**이다. ⇒ 「왜 거부됐는지 단언할 수 없다」는 **참**.

## ⓒ ★그러나 제안의 «역사»는 거짓이다 — 두 겹으로

제안 제목: 「carry a cause **again**」 · why: 「(cut 822504b)」 · tradeoff: 「**needs upstream changes** …
this repo is a fork whose upstream contact is deliberately read-only」.

실측:
1. ★**822504b 는 «자르지» 않았다 — 그 파일을 «만들었다».** `git show 822504b -- classfile/src/error.rs` 는
   **`new file`** 이고 내용이 지금과 **동일한 2변형**이다.
2. ★**그 이전에는 원인이 아니라 «아무것도» 없었다** — `git show 822504b^:classfile/src/class.rs` 기준
   `ClassInfo::parse` 는 **`Option<Self>`** 를 돌려줬다(실패에 정보 0 · 게다가 `.unwrap()` 투성이).
   ⇒ ★**822504b 는 «후퇴»가 아니라 «개선»이었다**(`Option` → `Result<_, ClassFileError>`).
3. ⇒ ★**「again」도 「restore」도 성립하지 않는다.** 이 리니지에 원인이 실려 있던 시기는 **없다.**

그리고 「upstream 이 해야 한다」는 ★**우리 fork 의 실측과 어긋난다**:
- `upstream/main` 기준 우리는 **5커밋 뒤**이고, 그 5개 중 이 파일들을 만지는 것은 **0건**이다.
- `classfile/src/error.rs` 를 upstream 이 만진 커밋은 **1건**(=생성)뿐 — ★**이 crate 에서 가장 안정된 파일**이다.
- ★**우리는 이미 이 crate 에서 크게 갈라져 있다**: `constant_pool.rs` **+211/−6** · `validation.rs` **+137/−0** ·
  `attribute.rs` **+129/−3** · `opcode.rs` **+83/−7**.
⇒ ★**「순수 로컬이 아니다」는 거짓**이다. `AGENTS.md` 의 read-only 규율은 **upstream 으로 «보내는 것»**을 금할 뿐,
로컬 변경을 금하지 않는다(우리는 매 회차 이 crate 를 고친다).

## ⓑ ★이미 같은 축이 «한 enum 건너» 있다 — 이것이 판정을 「할 값 있다」로 민다

`jvm-bytecode/src/error.rs` 의 형제 타입은 **이미 원인을 싣는다**:
`UnsupportedFeature(&'static str)`(**5곳**에서 사용 · `"ldc of a method handle"` 같은 문장이 그 결과다) ·
`UnsupportedClassVersion(u16)`. ⇒ ★**설계는 이 저장소에서 이미 «증명»돼 있고, `InvalidClassFile` 만 예외다.**
★그러니 이것은 «새 발명»이 아니라 **일관성 회복**이고, 모양도 자명하다(`InvalidFormat(&'static str)`).

## 이득 — 실측하되 «과장하지 않는다»

- ★**8곳**의 kind-only 단언이 원인을 직접 이름 부를 수 있게 된다.
- ★**참조 JVM 격차가 줄어든다**: OpenJDK 26 은 같은 파일들에 `Multiple BootstrapMethods attributes in class file X` ·
  `argument_index 65535 has bad constant type` 를 낸다(이 리니지가 직접 관측한 문장들). 우리는 전부 `Invalid class file` 이다.
  ★**이 리니지의 주제가 «정직한 진단»인데 진단 문면 자체가 평탄하다** — 그 모순이 이 제안의 진짜 값이다.
- ★★**그러나 제안의 「a stronger lock than a carefully shaped fixture」는 «절반만» 참이다.**
  원인은 **「어느 검사가 울렸나」**를 잠그고, 픽스처는 **「그 검사가 관측 가능한가」**를 잠근다 — **다른 자물쇠다.**
  ★**증거**: 바로 직전 `-fix` 회차가 찾은 구멍(신원 4축 중 3축 미관측)은 ★**파스 오류가 아니라 «링크» 축**이라
  원인을 실었어도 **잡히지 않았다.** ⇒ ★**원인은 픽스처 규율을 «대체»하지 못한다. 더한다.**

## 비용 — 제안이 적은 `target: classfile/src/error.rs` 는 «틀렸다»(1파일이 아니라 3층)

평탄화는 ★**아래로 두 번 더** 일어난다(실측):
```
ClassFileError::InvalidFormat                      (classfile/src/error.rs · 생산 9곳: validation 6 · class 3)
  → ClassDefinitionError::InvalidClassFile         (jvm-bytecode/src/error.rs · From impl 이 원인을 버린다)
    → jvm.exception("java/lang/ClassFormatError", "Invalid class file")   ← ★문자열 «하드코딩» 2곳
       (src/runtime.rs:189 · test-utils/src/lib.rs:334)
```
⇒ ★**`error.rs` 만 고치면 관측 가능한 변화가 «0» 이다** — 원인을 아무도 넣지 않고 아무도 읽지 않는,
이 저장소가 반복해 규탄한 「통과하지만 아무것도 재지 않는」 형태가 된다.

★★**그리고 진짜 비용은 따로 있다**: `validate_class` 는 **8항 `||` 사슬**을 한 번에 평가하고 **하나의** `InvalidFormat` 을 낸다.
원인을 «검사마다» 다르게 하려면 ★**그 사슬을 쪼개야** 한다 — ★그것은 이 티켓이 금지한 **리팩터**다(계약 3).
⇒ ★**그래서 여기서 구현하지 않는다.** 범위를 틀린 채로 집행하는 것보다 **바로잡아 넘기는 것**이 싸다.

## 후속에 넘기는 «정확한» 범위

⑴`ClassFileError::InvalidFormat(&'static str)` — 형제 타입의 `UnsupportedFeature(&'static str)` 와 **같은 모양**(no_std 안전).
⑵`ClassDefinitionError::InvalidClassFile(&'static str)` + `From` 이 원인을 **버리지 않게**.
⑶예외 문면 **2곳**이 그 문자열을 쓰게(`src/runtime.rs` · `test-utils/src/lib.rs`) — ★**두 곳을 같이 고쳐야** 테스트가 본다.
⑷`validate_class` 의 8항 사슬을 **검사마다 반환**으로 쪼갠다(원인이 갈리는 유일한 자리).
⑸그 뒤에 **단언을 조인다** — kind-only 8곳 중 «원인이 갈리는» 것부터.
★**⑴만 하고 멈추면 안 된다**(관측 변화 0). ★**⑷ 없이 ⑴~⑶만 하면** 모든 검증 실패가 같은 원인 문자열을 이고 **평탄함이 이사할 뿐**이다.

## 이 회차가 «지금» 고친 것 — 틀린 주석 한 곳

`tests/test_class_format.rs` 머리 주석이 **제안의 근거로 인용된 바로 그 문장**이고, 그것이 **거짓**이었다
(「cut 822504b」·「Restoring it needs upstream variants」). ⇒ ★**그 자리에서 고쳤다** — 착지한 트리가 거짓을 나르면
다음 사람이 같은 잘못된 전제로 같은 제안을 다시 만든다. ★**두 번째 언급(`ClassFileError` 가 평탄화한다)은 «참»이라 건드리지 않았다.**

## 잃는 것 / 안 하면 무엇이 나쁜가

⒜**잃는 것(이 회차)**: ★**주석 한 곳의 변경뿐** — 동작·테스트 수 불변(`cargo test --all` **572 / 0 failed / 1 ignored**).
★**판정을 «지금» 집행하지 않는 대가**는 있다: 8곳의 단언이 당분간 kind-only 로 남고, 그 사이 새로 들어오는
검증 규칙도 같은 평탄한 문장을 쓴다(이 회차 직전에 착지한 `#p0`·`#p1` 이 정확히 그 예다 — 둘 다 `InvalidFormat` 을 낸다).
⒝**안 하면**: 사용자는 «무엇이 잘못됐는지» 못 듣고, 테스트는 ★**「깨졌다」와 「다른 이유로 깨졌다」를 구별하지 못한다** —
그 구별 불가가 #49 의 테스트를 단언 조이기로 못 고치게 만든 **바로 그 원인**이다(그 회차는 픽스처를 다시 지어 우회했다).
