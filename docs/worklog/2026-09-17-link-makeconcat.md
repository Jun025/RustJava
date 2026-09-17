# 2026-09-17 — `StringConcatFactory.makeConcat` 도 링크한다 — 단 «이유는 제안이 적은 것이 아니다»

티켓 `rustjava-adopt-link-stringconcatfactory-p0` — 채택 제안 `2026-09-16-link-stringconcatfactory#p0`.

★**제품 동작이 바뀐다**: `makeConcat` 부트스트랩에 묶인 `invokedynamic` 콜사이트가 이제 **실행된다**
(종전 `UnsupportedOperationException: invokedynamic`).

## ⓒ ★제안의 전제는 «틀렸다» — 그리고 그 사실이 이 회차의 절반이다

제안 `plainSummary`: 「the sibling factory with no recipe — **used when javac concatenates without constant text**」
제안 `userBenefit`: 「**More javac-emitted** string concatenation runs instead of being refused」

★**실측(javac 26.0.1 · 같은 소스 `a + b`)**:

| 대상 | 내는 부트스트랩 |
|---|---|
| `--release 9` · `11` · `17` · `21` · `26` | ★**전부 `makeConcatWithConstants`** |
| `-XDstringConcat=indy`(비기본 내부 플래그) | `makeConcat` |
| `-XDstringConcat=inline` | `StringBuilder` |

★**상수 텍스트가 «없는» `a + b` 도 `makeConcatWithConstants` 다** — 레시피가 자리표시자 둘(``)일 뿐 리터럴이 0인 것이다.
⇒ ★**「javac 이 상수 텍스트 없이 연결할 때 쓴다」는 거짓**이고, ★**「javac 산출물이 더 많이 돈다」도 거짓**이다
(기본 javac 산출물은 **이미 전부 링크된다**).

## 그래도 «왜» 했나 — 근거를 바꿔서 적는다

⑴★**만들 수 있는 형상이다** — `-XDstringConcat=indy` 로 **실제로 만들었다**.
  이 리니지가 `ldc` 태그 지원을 정당화한 기준(「ASM 이 실제로 낸다」)과 **같은 기준**이다.
⑵★**`makeConcat` 은 `StringConcatFactory` 의 «문서화된 공개 진입점»** 이고, 바이트코드 생성기는 그것을 택할 수 있다.
⑶★★**실행기에 «새 경로가 필요 없다»** — `makeConcat(n개 인자)` ≡ 레시피가 `\u{1}` **n개**인 `makeConcatWithConstants` 다.
  ⇒ 콜사이트 서술자의 인자 수로 **레시피를 합성**하면 끝이고, `concat_with_constants` 는 **한 줄도 안 바뀐다**.
  ★제안이 「a recipe-free path through the same executor」라고 본 것보다 **더 싸다**(경로가 아예 안 생긴다).

## 설계 — 제안의 「짧은 명시 목록을 유지하라」를 지켰다

- 상수 튜플 **둘**(이름+서술자 쌍)뿐이고 **레지스트리로 만들지 않았다**(제안 tradeoff 준수).
- ★**이름과 서술자를 «쌍»으로 맞춘다** — `(name, descriptor)` 를 함께 매칭하므로
  ★**한쪽 진입점의 이름에 다른 쪽의 서술자를 붙인 것은 링크되지 않는다.**
  ⇒ 이 결정이 ★**#55 가 넣은 name 축 근접 실패 픽스처(`NotMakeConcatWithConstants`)를 «살렸다»** — 그 파일은
  이름이 `makeConcat` 이지만 서술자가 `makeConcatWithConstants` 의 것이라 **여전히 거부된다**.
- ★**레시피 합성은 «콜사이트마다»** 한다 — 한 부트스트랩 항목을 서술자가 다른 여러 콜사이트가 공유할 수 있어
  부트스트랩 해석 단계에서는 **고정할 수 없다**. 그래서 해석은 `LinkedFactory::{WithConstants, NoRecipe}` 로 «인식»만 하고,
  합성은 하강 루프에서 한다.

## 픽스처 — ★**결과를 «출력»하게 만들었다**(이유가 있다)

`test-data/indy/MakeConcat.class`(생성기 · 합성): `ldc "a"; ldc "b"; invokedynamic concat(String,String)String; println`.
★**「링크됐다」가 아니라 「무엇이 연결됐나」를 단언**한다 — ★**레시피를 «틀린 길이»로 합성해도 링크는 되고 실행도 된다.**
값을 버리면 그 오류가 **안 보인다**(실측: 아래 M2).
★**javac 이 안 내는 형상이라 손으로 조립했다** — 비기본 내부 플래그에 픽스처 재생성을 의존시키는 것이 더 나쁘다.

## ★★개악 대조 — 4종 중 «둘»이 처음엔 살아남았고, 그래서 픽스처를 더 만들었다

| 개악 | 처음 | 픽스처 보강 후 |
|---|---|---|
| **M1** `makeConcat` 튜플 제거(= 제안 이전) | **KILLED** | KILLED |
| **M2** 레시피 길이를 `repeat(1)` 로 | **KILLED**(출력 대조가 잡는다) | KILLED |
| **M3** 쌍 검사를 느슨하게(`(name, _)`) | ★**SURVIVED** | ★**KILLED** |
| **M4** 「정적 인자 없음」 가드 제거 | ★**SURVIVED** | ★**KILLED** |

★**M3·M4 가 살아남은 이유는 «축마다 픽스처가 없어서»다** — 직전 `…-p0-fix` 회차가 세운
「**죽었다 ≠ 그 가지가 «전부» 덮였다**」가 ★**내 새 코드에 그대로 적용됐다.**
⇒ 근접 실패 둘을 더했다: `MakeConcatWrongDescriptor`(이름은 맞고 **서술자만** 다르며 **정적 인자 0** ⇒ 서술자 비교만이 거부할 수 있다) ·
`MakeConcatWithArgument`(이름·서술자 맞고 **정적 인자 1** ⇒ 인자 수 가드만이 거부할 수 있다).
★**둘 다 «그 축 하나»만 다르다** — 그래야 그 축이 관측된다.

## 잃는 것 / 안 하면 무엇이 나쁜가

⒜**잃는 것**: ⑴★**링크 수용 범위가 넓어진다** — 제안이 경고한 그대로이고, 「이 런타임은 콜사이트 «하나»를 링크한다」가
  더 이상 참이 아니다(이제 **둘**이다). ⇒ 상수 튜플을 **명시 목록**으로 유지하고 doc 에 그 이유를 적었다.
  ⑵픽스처 **3개** · 테스트 1개 · `cargo test --all` **573 → 574**.
  ⑶`resolve_bootstrap_methods` 의 반환형이 튜플 → 작은 enum 으로 바뀌었다(호출부 1곳).
⒝**안 하면**: `makeConcat` 에 묶인 콜사이트는 계속 **「이 런타임이 아직 못 한다」**로 거부된다 —
  ★**실행기는 그것을 «이미 할 수 있는데»** 그렇다(레시피 합성 한 줄이면 되는 일이었다).
