# 2026-09-11 — null 가드 감사(세기) + `java/io` 버퍼 18곳 가드

채택 제안 `2026-09-11-null-guard-string-init-and-arraycopy#p0`.

선행 회차는 **손으로 열거한** 9곳을 닫았고, ★그 열거 자신이 `arraycopy` 의 `dest` 를 빠뜨렸다.
그래서 이번엔 **술어로 셌다**.

## 감사 — 수와 술어 (측정 2026-09-11 · 트리 `origin/main` `6da7d66f`)

★★**[게이트② F1 정정] 수는 «트리와 함께» 적는다 — 초판이 두 트리를 섞었다.**
정본 = **`scripts/audit-null-guards.py`**(이 회차가 커밋했다). 재현: `python3 scripts/audit-null-guards.py [<runtime-src>]`.

| 트리 | N | M | K |
|---|---|---|---|
| **base `6da7d66f`**(= 이 PR 의 `merge-base`) | 1,163 | **50** | ★**38** |
| **head `068fdb95`** | 1,163 | **32** | **20** |
| **Δ** | 0 | **18** | ★**18** = 이 회차의 가드 수(자기정합) |

술어: **N** = `rustjava-runtime/src/**` 의 fn 파라미터 중 타입이 `ClassInstanceRef<…>`(★`this`·`_` 접두 제외) ·
**M** = N 중 그 파라미터가 **deref 강제 sink** 에 `&p`/`&mut p`(또는 `p.as_class_instance()`)로 도달하고 ★**그 «앞»에 `p.is_null()` 가 없는** 것 ·
**K** = M 중 그 fn 이 같은 파일 `JavaMethodProto::new(… Self::fn …)` 에 등재 = **게스트가 부를 수 있다**.

★★**초판의 「M 58 · K 46」은 «다른 트리»의 수다** — **`eaad8e9c`**(선행 9곳 회차 «이전» main)에서 재고 라벨만 `6da7d66f` 로 달았다.
⇒ ★**`46 − 18 = 28 ≠ 20`** 이라는 산술 모순이 그 혼입의 «증상»이었고, 검수자가 술어를 재구현해 그것으로 잡았다.
★**이 저장소가 반복해 규탄한 「base 를 병기하지 않으면 수가 섞인다」의 교과서적 재현이다.**
※검수 재구현치는 N **1,130** · M **52/34**(K 는 **38/20 으로 정확 일치**) — ★**판단을 이끄는 K 가 두 구현에서 같다.**
N·M 의 차는 구현 세부(파라미터 계수 방식)이고, ★**이제 스크립트가 커밋돼 있으므로 «어느 쪽이 맞나»는 돌려서 답한다.**

**deref 강제 sink = 17개**(`jvm/src/jvm.rs` 의 시그니처로 확정 — `&Box<dyn ClassInstance>` · `&mut Box<…>` · `impl AsClassInstance`):
`array_element_type` `array_length` `array_raw_buffer` `array_raw_buffer_mut` `get_field` `interrupt_java_thread`
`invoke_special` `invoke_virtual` `is_java_thread_interrupted` `load_array` `monitor_enter` `monitor_exit`
`object_notify` `object_wait_prepare` `put_field` `shallow_clone` `store_array`

★**왜 sink 를 한정했는가**: 초판은 「`&p` 가 나오면 deref」로 셌는데 그것은 **과대계상**이다 —
`Self::value_range(jvm, &value)` 처럼 **`&ClassInstanceRef<T>` 를 받는** 헬퍼는 deref 하지 «않는다».
한정 전 220 → 한정 후 152 → (계측기 버그 수정 후) **58**.

## ★계측기를 «먼저» 검증했다 — 그리고 초판이 틀렸다

정답지 = 선행 회차가 닫은 **9곳**. 가드 «전»(`eaad8e9c`)·«후» 트리에 같은 자를 대고 차집합을 봤다.

- **초판: 3/9** — 근인은 **off-by-one** 이다. 정규식을 `"\n" + src` 에 돌려 놓고 인덱스는 `src` 에 적용해
  괄호 스캔이 **한 칸 밀렸고**, 시그니처의 `)` 에서 멈추지 못해 파라미터 문자열이 본문까지 삼켰다.
- **수정 후: 8/9** · 새로 생긴 것 **0**.
- ★**남은 1건 = `init_with_string(value)`** — 그 fn 은 `Self::value_range(jvm, &value)` **안에서** deref 한다.
  ⇒ ★★**이 계측은 절차간(interprocedural)을 못 본다. 따라서 N·M·K 는 «하한»이다.**
  ※검수자가 선행 회차에 적은 「899 중 560 · **상한값**」과 **방향이 반대**다 — 그쪽은 fn 단위 크루드 스캔이고 이쪽은 sink 한정 + 절차내다. **두 수를 같은 축으로 비교하지 마라.**

## 부수 발견 — 패닉 impl 은 «둘»이 아니라 «셋»이다

`jvm/src/class_instance.rs` 에 `Deref`(`as_ref().unwrap()`) · `DerefMut`(`as_mut().unwrap()`) 말고
★**`AsClassInstance::as_class_instance`(`as_deref().unwrap()`)** 가 있다 — `monitor_enter`/`monitor_exit`/`object_wait_prepare`/`object_notify` 가 그 경로다.

## 이 회차가 «닫은» 선 — `java/io` 데이터 전송 + `String.getChars` (18곳)

`BufferedOutputStream.write(byte[],int,int)` · `DataInputStream.readFully(byte[])` ·
`DataOutputStream.{writeBytes,writeChars,writeUTF}(String)` · `FileInputStream.read(byte[])` ·
`FileOutputStream.write(byte[],int,int)` · `InputStreamReader.read(char[])` · `OutputStreamWriter.write(char[])` ·
`RandomAccessFile.{read(byte[]),read(byte[],int,int),write(byte[]),write(byte[],int,int)}` ·
`Reader.read(char[])` · `Writer.{write(char[]),write(String),write(String,int,int)}` · `String.getChars(int,int,char[],int)`

★**선의 근거**: 전부 **JDK 규격이 NPE 를 명시**하는 «데이터를 옮기는 버퍼»이고, 게스트가 가장 흔히 null 을 넘기는 자리다.
★**가드 형태는 선행 회차 그대로**(진입부 `is_null()` → `jvm.exception("java/lang/NullPointerException", "<param> is null")`) — 새 형태를 만들지 않았다.

## ★남긴 20건 — 「가드를 넣어라」가 «아니다»

```
java/io      read_utf_from_input(input) · init(file)×2 · init_with_file(file)
java/net     init(url) · init_with_context_spec_handler(handler) · set_url(url) · open_connection(url)
java/util    init(list)×4 · on_access(map) · init(map) · init(pattern) · matches(pattern) · split_with_limit(input)
java/util/zip init_with_zip_entry(zip_entry) · get_input_stream(entry) · init(file)
```
★★**일괄 가드는 «틀린다»** — `URL(URL context, String spec, URLStreamHandler handler)` 는 JDK 규격상
**`handler == null` 이 합법**이다(기본 핸들러를 쓰라는 뜻). 실제로 이 repo 의 구현도 그 파라미터를 **섀도잉해 무시**한다
(⇒ 그 1건은 이 계측의 **오탐**이기도 하다 — 이름 기반 절차내 스캔은 섀도잉을 못 본다).
⇒ **후속의 본체는 「규격으로 삼분하는 것」**(NPE 의무 / null 합법 / 도달 불가)이고, 첫 갈래만 닫는다.

## 검증 — 양방향

- **정상 → green**: `cargo test --test test_class` **ok** · DoD 7종 rc=0 · `cargo test --all` **554/0/1**
  (★계수 불변이 맞다 — `test_class` 는 `test-data/` 를 **한 테스트 함수**가 순회한다).
- **개악 → red**: ⑴18곳 **전건** 되돌림 → **FAILED** `class_instance.rs:108`(`Deref`)
  ⑵★**단일 가드**(`String.getChars` 의 `dst`)만 제거 → **FAILED** `:114`(`DerefMut`) ⇒ 픽스처가 **케이스별로** 문다.

## 픽스처 커버리지 — ★**13케이스가 가드 12/18 을 문다** (F2 정정 + 권고 채택)

★★**[게이트② F2] 초판의 「18 중 12」는 «그 시점엔 거짓»이었다 — 실측 10.**
근인: `new InputStreamReader(...).read(char[])` · `new OutputStreamWriter(...).write(char[])` 는
그 **서브클래스가 오버라이드**하므로 `Reader`/`Writer` 의 **base 구현에 닿지 못한다** ⇒ 그 2곳은 «덮은 줄 알았지만» 안 덮였다.

⇒ ★**검수 권고를 채택해 2케이스를 «추가»했다**: `Reader`/`Writer` 를 상속하되 **추상 `(char[],int,int)` 만** 구현하는
**중첩 클래스**(`NullBufferGuards$BareReader`·`$BareWriter` — ★하버스가 `$` 든 이름을 건너뛰므로 별 `.txt` 가 필요 없다)로 base 구현에 도달시켰다.
★**도달을 «실행»으로 증명했다** — `reader.rs read(buf)` 만, `writer.rs write_chars(chars)` 만 각각 제거하면 **각각 red** 다.

**덮는 12** = `BufferedOutputStream.write` · `DataInputStream.readFully` · `DataOutputStream`×3 ·
`InputStreamReader.read` · `OutputStreamWriter.write` · `Writer.write(String)` · `Writer.write(String,int,int)` ·
★`Reader.read(char[])` · ★`Writer.write(char[])` · `String.getChars`
(+13번째 케이스는 `getChars` 의 **예외 선후 잠금** — 같은 가드의 «위치»를 문다).

★**덮지 «못한» 6곳** = `FileInputStream.read` · `FileOutputStream.write` · `RandomAccessFile` 4종 —
**실파일 핸들이 있어야 인스턴스가 생기고**, 테스트가 작업 디렉터리에 파일을 만드는 것을 피했다.
⇒ ★**그 6곳은 «가드는 들어갔으나 픽스처가 잠그지 않는다»** — 다음 회차가 임시 파일 픽스처를 세우면 닫힌다(`proposals[1]`).

## ★F4 — `String.getChars` 가드의 «위치»를 고쳤다 (선택했다)

JDK `getChars` 는 **범위를 먼저** 검사하므로 «잘못된 범위 + null dst» 면 **`StringIndexOutOfBoundsException` 이 이긴다**.
초판은 가드를 범위 검사 «앞»에 둬 그 선후를 **NPE 로 뒤집었다** — 티켓 계약 4(「동작 의미를 바꾸지 마라」)의 그 축이다.
⇒ ★**옮겼다**(`issues` 에 적는 쪽이 아니라). 그리고 ★**선후 잠금 케이스를 픽스처에 넣었다** — 옛 위치로 되돌리면 그 케이스가 **red** 다.

## 재현 — ★**스크립트를 커밋했다**(F5 · 초판 판단을 뒤집는다)

```sh
python3 scripts/audit-null-guards.py                       # 현재 트리
git worktree add /tmp/t <sha> && python3 scripts/audit-null-guards.py /tmp/t/rustjava-runtime/src
```
★★**초판은 「CI 에 배선되지 않은 검사기는 낡는다」며 커밋하지 «않았다». 그 판단을 뒤집는다 — 근거는 이 회차 자신이다**:
★**작성자가 자기 수를 교차검증할 수단이 없어 F1(두 트리 혼입)이 났고**, 검수자는 **술어를 prose 에서 재구현**해야 했다(그래서 N·M 이 갈렸다).
⇒ **실행 가능한 정의 하나가 산문보다 싸다.**
★**단 «잠금이 아니라 감사»다** — CI 에 배선하지 «않았다». K 가 커도 아무것도 red 가 되지 않는다(이 스크립트는 수를 보고할 뿐이다).
★**다시 돌릴 때는 반드시 «정답지 검증»을 먼저 하라**(스크립트 docstring 에도 박았다) — 이 회차가 그것으로 자기 off-by-one 을 잡았다.

## 제안
- p0: 잔여 20건의 **규격 삼분**(NPE 의무 / null 합법 / 도달 불가) 후 첫 갈래만 닫기.
- p1: **파일 기반 IO 픽스처** — 가드는 있는데 잠그지 않는 6곳.
