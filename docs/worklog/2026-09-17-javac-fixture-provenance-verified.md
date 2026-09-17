# 2026-09-17 — 「어느 javac 이 만들었나」를 «기록»이 아니라 «검증»으로 바꿨다

티켓 `rustjava-adopt-indy-fixture-jdk-pin-and-slot-accounting-p1` — 채택 제안
`2026-09-16-indy-fixture-jdk-pin-and-slot-accounting#p1`.

★**제안의 결론 두 개가 실측으로 반증됐다.** 그래서 제안이 «불가능»하다고 적은 쪽을 만들었다.

## ⓒ 반증 1 — 「검증할 수 없다」는 거짓이다

제안 tradeoff: 「**Nothing offline can verify a recorded compiler version** — a record is only as good
as the person writing it, so this buys **provenance, not enforcement**.」

★**실측**: `javac` 은 같은 소스·같은 플래그·같은 컴파일러에 대해 **결정적**이다. 기록된 도구
(`javac 26.0.1 --release 21`)로 다시 빌드하니 ★**`test-data/indy` 의 6개 클래스 파일이 «바이트 단위로 동일»**했다.

```
$ sh test-data/src/verify-javac-fixtures.sh
javac: javac 26.0.1
  ✓ ConstantKinds$Op · ConstantKinds$Suit · ConstantKinds · Lambda$Op · Lambda · StringConcat  (--release 21)
6 rebuilt: 6 reproduced, 0 differed; 0 could not be rebuilt
```

⇒ ★**기록이 «검증»된다.** 「사람이 적은 만큼만 믿을 수 있다」가 아니라 **재현으로 확인된다.**

## ⓐ·ⓒ 반증 2 — 「강제 가능한 축은 한 픽스처만 덮는다」도 거짓이다

제안 why: 「The only assertions holding that axis today are the constant counts in
`classfile/src/constant_pool.rs`, and they **cover one fixture**.」

★**실측 — javac 산출 indy 픽스처 «셋 모두» 형상 단언을 갖고 있다**:

| 픽스처 | 형상 단언 | 어디 |
|---|---|---|
| `ConstantKinds` | 태그별 **정확한 개수**(MethodType 1 · Dynamic 3 · MethodHandle 7 · InvokeDynamic 3) | `classfile/src/constant_pool.rs` |
| `StringConcat` | 부트스트랩 **kind·class·name·descriptor** 4축 + 인자 수 + ★**인자가 «가리키는 값»**(`"a\u{1}"` 레시피) · 게다가 ★**바이트 창** `[15, 6, 0, 35]` 를 찾아 「layout changed」로 실패한다 | `classfile/tests/test.rs` |
| `Lambda` | 부트스트랩 `LambdaMetafactory.metafactory` · 인자 **3개** · ★**`args[0] == args[2] != args[1]`**(samMethodType/instantiatedMethodType 동일성) | `classfile/tests/test.rs` |

⇒ ★**제안이 「더 만들어야 한다」고 본 「enforceable half」는 이미 3/3 있다.**
★그리고 제안이 예로 든 위험(「enum switch 가 현대 javac 에서 condy 가 된다」)은 ★**`ConstantKinds` 의 Dynamic 개수 3 이 정확히 그것을 잠그고 있다.**

## 만든 것 — `test-data/src/verify-javac-fixtures.sh`

기록을 **검사**로 바꾸는 최소 도구. ★**재빌드해서 바이트를 비교한다.**
- ★**`--release` 를 «픽스처 자신»에서 읽는다**(major − 44) — 외부 표에 의존하지 않아 **동기화할 것이 없다**.
- ★**명시한 `JAVAC` 가 안 되면 «다른 컴파일러로 조용히 대체하지 않는다»** — rc=2 로 멈춘다.
  (그러지 않으면 「무엇이 검증했나」가 흐려진다.)
- ★**CI 에 배선하지 «않았다»** — `.github/workflows/rust.yml` 에 JDK 가 없고 이 맥의 PATH 에도 없다.
  JDK 를 요구하는 테스트는 **어디서나 실패하거나 어디서나 건너뛴다**. 이건 «재생성했을 때 사람이 돌리는» 검사다.
- ★**「못 만들었다」와 「만들었는데 다르다」를 «가른다»** — 합치면 발견을 과장하게 된다(아래 실패담 참조).

## ★실패담 둘 — 이 회차가 실제로 밟은 것

⑴**`command -v javac` 이 macOS 스텁을 찾는다**(실행 파일인데 「JDK 없음」을 출력한다) ⇒ 경로가 아니라 **실행해서** 고른다.
⑵★**`-sourcepath` 를 넣었다가 «더 나빠졌다»** — `test-data/src` 에는 **`Exception.java`·`Array.java`·`Method.java`** 가 있어
소스 경로에 올리면 javac 이 `Exception` 을 ★**`java.lang.Exception` 이 아니라 그 픽스처로** 해석한다
(실측: 멀쩡히 재현되던 파일들이 `incompatible types: Exception cannot be converted to Throwable` 로 무너졌다).
⇒ **되돌렸고, 그 대가**(형제 클래스를 참조하는 소스는 홀로 재빌드 불가)를 ★**「could not be rebuilt」로 «따로» 보고**한다.

## 개악 대조(양방향)

| 조작 | 결과 |
|---|---|
| 커밋된 `StringConcat.class` 의 **마지막 1바이트** 반전 | ★**✗ 감지** — `rebuilt bytes differ from the committed fixture` |
| 복원 | **6 reproduced, 0 differed** |
| 명시한 `JAVAC` 가 없는 경우 | ★**rc=2 「nothing was verified」** — ★**«통과»가 아니다** |

## ★일반화 — 시켜 보고 «나온 값»을 적는다(고치지는 않았다)

같은 도구를 루트에 돌렸다(`sh … test-data`):
```
109 rebuilt: 104 reproduced, 5 differed; 3 could not be rebuilt
```
★**다른 5개**: `MonitorSemantics`(+내부 클래스 2) · `NativeMethod` — 셋 다 `--release 8` · `OddEven` — `--release 21`.
★**원인은 여기서 «단정하지 않는다»** — 「다른 컴파일러로 빌드됐다」와 「빌드 뒤 소스가 수정됐다」가 **둘 다 이 관측과 맞는다.**
★**3개는 홀로 재빌드 불가**(형제 클래스 참조) — 위 `-sourcepath` 제약의 결과다.
⇒ ★**이 회차의 범위(제안 대상 = `test-data/src/indy/`)를 넘으므로 고치지 않았고, 후속으로 넘겼다.**
★**그래도 적는 이유**: 이 수가 ★**제안이 걱정한 드리프트가 «실재»한다는 첫 직접 증거**다 — 가정이 아니다.

## 잃는 것 / 안 하면 무엇이 나쁜가

⒜**잃는 것**: ⑴**스크립트 1개**가 는다(≈70줄 · 읽기 전용 · 트리를 고치지 않는다) ⑵★**CI 가 돌리지 않는다** —
JDK 가 없으니 **자동으로 지켜지지 않는다**. 이것이 이 축의 진짜 한계이고, 숨기지 않는다.
⑶픽스처를 의도적으로 재생성하면 **한 번 돌려 확인하는 습관**이 필요하다(문서·doc 주석에 적었다).
★**왜 그래도 남기나**: 직전 회차가 «커밋하지 않기로» 한 개악 하네스와 **다른 종류**다 —
그건 **제품 소스를 치환**해서 죽으면 트리를 오염시켰고, 이건 **읽고 비교만** 한다(실패해도 트리 무변).
⒝**안 하면**: 「javac 26.0.1 로 만들었다」가 ★**영원히 «주장»으로 남는다.** 그리고 루트의 5건이 보여 주듯
★**드리프트는 이미 일어나 있다** — 확인할 수단이 없으면 다음 회차도 그것을 못 본다.
