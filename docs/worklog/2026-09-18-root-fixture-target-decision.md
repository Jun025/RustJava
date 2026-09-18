# 2026-09-18 — 루트 fixture 를 한 target 으로 모을 것인가 — ★**모으지 않는다** (rustjava-adopt-test-data-version-freeze-uniform-target-p0)

채택 제안 `2026-09-17-test-data-version-freeze#p0` 의 처분. ★**결정 회차 — 코드 0행 · 픽스처 재컴파일 0 · `.class` 바이트 0 변경.**
산출물은 결정 문서 `docs/test-data-target-policy.md` 와 그 근거 실측이다.

## 쓴 명령(재측용 — 다시 «논하지» 말고 다시 «재라»)

```sh
# ⑴ 버전 분포 — 동결 파일이 아니라 fixture 자신에서 읽는다
for c in test-data/*.class; do od -An -tu1 -j6 -N2 "$c" | tr -s ' \n' ' ' | awk '{print $1*256+$2}'; done | sort -n | uniq -c

# ⑵ 버전 민감 축 — 커밋본과 «--release 21 재컴파일본» 을 대조
javac -g --release 21 -d <tmp> test-data/src/<name>.java
LC_ALL=C grep -ac 'BootstrapMethods\|StringBuilder\|access\$\|NestMembers' <class>
javap -c -p <class> | grep -oE '^\s+[0-9]+: [a-z0-9_]+' | awk '{print $2}'   # 전체 명령 시퀀스
```

## ⑴ 분포 — 114개 · 다섯 버전

| major | Java | 개수 |
|---|---|---|
| 52 | 8 | **40** |
| 65 | 21 | **62** |
| 66 | 22 | 8 |
| 68 | 24 | 1 |
| 70 | 26 | 3 |

★동결 파일 머리주석의 수(52×40·65×62·66×8·68×1·70×3)와 **일치**한다 — 베끼지 않고 따로 셌다.

## ⑵ 양방향 — 「버전이 곧 시험 대상」 ↔ 「아무 버전이나 되는 것」

### 버전이 곧 시험 대상 (★재컴파일하면 «다른 경로»를 타게 된다)

**축 1 · 문자열 연결.** javac 은 `+` 를 Java 8 까지 `StringBuilder`, 9 부터 `invokedynamic` 으로 낮춘다.

| fixture | 커밋본(52) | `--release 21` 재컴파일 |
|---|---|---|
| `FormatterIntegration` · `NullSpecGuards` | `StringBuilder` | ★`BootstrapMethods` 생기고 `StringBuilder` **사라짐** |
| `FormatterIntegration$FailingAppendable` | `StringBuilder` | ★`BootstrapMethods` 는 생기나 **`StringBuilder` 는 남는다** — `FormatterIntegration.java:36` 의 `private final StringBuilder output` 은 ★**«낮춤 산물»이 아니라 «명시적 API 사용»**이라 target 과 무관하다 |

★**`StringConcat.class`(major 65)는 루트에서 `invokedynamic` 을 가진 «유일한» fixture 다**
⇒ 루트 트리는 **낮춤 전략마다 정확히 하나씩** 갖고 있고, 52 를 재타깃하면 **전-indy 쪽이 지워진다**.

**축 2 · nestmate 접근.** JEP 181(Java 11) 전에는 중첩 클래스의 private 접근이 `access$NNN` **합성 브리지**를 거쳤고,
11 부터는 `NestHost`/`NestMembers` 로 **직접** 한다.

| fixture | 커밋본(52) | `--release 21` |
|---|---|---|
| `ThreadInterruption` | `access$` **×10** · Nest 속성 **0** | `access$` **0** · `NestMembers` **생김** |
| `MonitorSemantics` | `access$` **×4** · Nest 속성 **0** | `access$` **0** · `NestMembers` **생김** |

★이 repo 는 `NestHost`/`NestMembers` 처리를 **최근에** 넣었다 ⇒ 양쪽 다 **살아 있는 시험면**이다.

★★**`NativeMethod` 는 «제3 축»이 아니라 축 2 의 «다른 얼굴»이다 — 게이트② 검수자가 규명했다**(★재조사하지 않고 인용한다).
`javap -c -p` 전문 diff 가 **단 한 줄**이다: `private native void missing()` 를 **같은 클래스의 `static main`** 에서 부르는 자리가
`invokespecial` → **`invokevirtual`** 로 바뀐다. JEP 181 이후 **같은 nest 안의 private 인스턴스 메서드**는 직접 호출되고,
여기서는 **자기 클래스가 자기 nest host** 라 지울 `access$` 브리지가 없어 **opcode 만** 움직인다.
⇒ ★**상이 3건이 두 축으로 «전부» 설명된다 — 설명 안 되는 잔여 0.**

### 아무 버전이나 되는 것 (★재타깃해도 얻는 것이 0)

major 52 중 **소스가 있고 단독 재빌드 가능**하며 `StringBuilder`·`BootstrapMethods` **둘 다 없는** 20건을
`--release 21` 로 재컴파일해 **명령 시퀀스 전체**를 대조:

- ★**16건 = 완전 동일**(`ArrayEdgeCases`·`BooleanTest`·`CheckCast` …) ⇒ 이들에게 target 은 **임의값**이다.
- 3건 다름 = 위 nestmate 둘 + `NativeMethod`.
- 1건 재빌드 불가(형제 참조 — `verify-javac-fixtures.sh` 의 알려진 한계).

⇒ ★**정직한 분할: 본 20건 중 «3건이 버전이 답»(nestmate 2 + `NativeMethod`) · «1건 단독 재빌드 불가» · «16건은 아무래도 좋다».**
★**초판의 「5건」은 자기 산술과 어긋났다**(5+16=21≠20) — 선별 기준(`StringBuilder` 없음)에서 **정의상 배제된** 2건을 얹은 수다.
★**오차의 방향이 급소다**: 「버전이 답」쪽 **과대**라 ★**결론을 더 세게 보이게 한다.** 결론 자체는 이 비가 아니라 아래 「0·0」에 선다.

### ★그 커버리지는 다른 데 «없다»

`test-data/{cp,indy,ldc,attr}` 의 생성기 산출 **64건** 전수에서
`StringBuilder` **0** · `access$` 브리지 **0**. ⇒ ★**루트 52 무리가 그 두 경로의 «유일한» 시험면이다.**

## 결정과 그 이유

> ★**모으지 않는다.**

제안이 든 이득은 「bytecode 모양을 **숫자 하나로** 예측 가능하게 한다」인데, 실측이 그 논거를 **뒤집는다**:
★지금 major 52 는 **「전-indy · 전-nestmate 모양」이라는 뜻을 실제로 갖는다**. 전부 65 로 펴면
숫자가 더 많은 정보를 주는 게 아니라 ★**지금 담고 있는 구분이 사라지고**, 그 대가로 **런타임이 아직 구현해야 하는 두 경로의 유일한 커버리지가 지워진다.**

★**그리고 대가는 실재한다**: 재컴파일은 바이트를 바꾸고, 루트 fixture 의 버전은
`test-data/class-file-versions.txt` 에 핀돼 `tests/test_fixture_pins.rs` 가 단언하며,
루트 **66건**은 stdout 을 커밋된 `.txt` 와 `tests/test_class.rs` 가 대조한다.
⇒ 일괄 작업이 아니라 **fixture 마다의 판단**이고, 본 20건 중 **16건은 그 판단이 「그대로 둔다」**로 끝난다.

★**제안이 스스로 적은 「대부분은 'leave it' 이 정직한 답일 것」이 맞았다** — 다만 그 이유는 「귀찮아서」가 아니라
★**그 spread 의 일부가 «우연»이 아니라 «커버리지»이기 때문**이고, 그것을 가른 것이 이 회차의 실질이다.

## ★되돌릴 조건

- **런타임이 옛 모양을 버리면** — major ≤ 52 를 안 받거나 `StringBuilder`·`access$` 경로가 불요가 되면 52 무리는 커버리지가 아니다.
- **그 커버리지가 다른 데서 생기면** — 위 「0 · 0」은 **날짜 있는 측정**이지 항구 속성이 아니다. 뒤 회차가 그 경로를 덮는 fixture 를 넣으면 재실행하라.
- **동결이 사라지면** — 이 결정은 `class-file-versions.txt` + `test_fixture_pins.rs` 가 버전을 붙잡아 준다는 전제 위에 선다.
- **제3의 버전 축이 나오면** — 「버전이 답인」 집합이 넓어져 이 결정을 **강화**한다. ★단 초판이 그 후보로 적은 `NativeMethod` 는 ★**축 2 로 닫혔다**(위) ⇒ 현재 축은 **둘**이고, 이 조건은 «새 후보»를 기다린다.

## 잃는 것 — 「없다」로 적지 않는다

- ★**비균일은 그대로 남는다.** 새로 fixture 를 넣는 사람은 「어느 target 으로?」에 여전히 답이 없다 —
  이 결정은 **기존 것을 건드리지 않기로** 한 것이지 **신규 규칙을 세운 것이 아니다**(그건 별 축이다).
- ★**16건은 «아무래도 좋은 채»로 남는다** — 정리하면 깔끔해질 자리를 정리하지 않기로 했고, 그 이유는
  이득(0)보다 대가(핀 갱신 · `.txt` 대조 위험)가 크기 때문이지 **그 자리가 옳아서가 아니다.**
- ★**본 것은 40 중 20 이다.** 나머지 20 은 **내부 클래스 18 + `StringBuilder` 보유 최상위 2** 다 — ★**«소스 없음»은 공집합이다**(major-52 **40건 전건**이 `test-data/src/<outer>.java` 를 갖는다 · NOSRC **0**). ★초판이 적은 배제 사유가 틀렸다(수 20 은 맞다). 16/20 을 40 전체의 비로 읽지 마라.
- ★**선별 기준은 「소스가 있는」이 아니라 「최상위 + `StringBuilder` 없음」이다** — 초판은 「단독 재빌드 가능한 20건」이라 적었는데 ★그중 1건(`VirtualDispatchSemantics`)은 **재빌드가 안 됐다** ⇒ 재빌드 가능 여부는 **기준이 아니라 결과**다.

## 후속 — ★**카드로 냈다**(초판은 산문에만 적어 cockpit 도달이 «0장»이었다)

worklog `.json` 의 `proposals[]` **2장**(7키 전부):
⑴**신규 fixture 의 target 규칙을 세울 것인가**(M — 위 「잃는 것」 첫 줄이 연 자리) ·
⑵**되돌릴 조건에 «관측자» 붙이기**(S — 게이트② 실측: 넷 중 «동결이 사라지면» 하나만 자기집행이고
나머지 셋은 **누군가 이 문서를 열어야만** 발화한다).

★**초판이 후속으로 적은 「`NativeMethod` 제3 축 여부」는 카드가 아니다** — 게이트② 검수자가 그 자리에서
규명해 **닫혔다**(축 2 의 다른 얼굴). 닫힌 질문을 카드로 내면 운영자가 **이미 답이 있는 것**을 채택하게 된다.

