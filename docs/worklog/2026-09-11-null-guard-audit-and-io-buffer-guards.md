# 2026-09-11 — null 가드 감사(세기) + `java/io` 버퍼 18곳 가드

채택 제안 `2026-09-11-null-guard-string-init-and-arraycopy#p0`.

선행 회차는 **손으로 열거한** 9곳을 닫았고, ★그 열거 자신이 `arraycopy` 의 `dest` 를 빠뜨렸다.
그래서 이번엔 **술어로 셌다**.

## 감사 — 수와 술어 (측정 2026-09-11 · 트리 `origin/main` `6da7d66f`)

| 축 | 값 | 술어 |
|---|---|---|
| **N** | **1,163** | `rustjava-runtime/src/**` 의 fn 파라미터 중 타입이 `ClassInstanceRef<…>` 인 것(★`this` 와 `_` 접두는 제외) |
| **M** | **58** | N 중, 그 파라미터가 **deref 강제 sink** 에 `&p`/`&mut p`(또는 `p.as_class_instance()`)로 도달하고 ★**그 «앞»에 `p.is_null()` 가드가 없는** 것 |
| **K** | **46** | M 중, 그 fn 이 같은 파일의 `JavaMethodProto::new(… Self::fn …)` 에 등재 = **게스트 바이트코드가 부를 수 있다** ⇒ null 이 «도달 가능» |

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

## 픽스처 커버리지 — ★**18 중 12만 덮는다**(숨기지 않는다)

`NullBufferGuards` 10케이스가 무는 것은 `BufferedOutputStream`·`DataInputStream`·`DataOutputStream`×3·
`InputStreamReader`·`OutputStreamWriter`·`Writer`×2·`String.getChars` 다.
★**덮지 «못한» 6곳** = `FileInputStream.read` · `FileOutputStream.write` · `RandomAccessFile` 4종 —
**실파일 핸들이 있어야 인스턴스가 생기고**, 테스트가 작업 디렉터리에 파일을 만드는 것을 피했다.
⇒ ★**그 6곳은 «가드는 들어갔으나 픽스처가 잠그지 않았다»** — 다음 회차가 임시 파일 픽스처를 세우면 닫힌다.

## 재현 — 감사를 다시 돌리는 법

이 회차는 **감사 스크립트를 커밋하지 않았다**(CI 에 배선되지 않은 검사기는 낡는다 — 이 저장소가 이미 그렇게 판정했다).
술어는 위 표가 정본이고, 재현은 **sink 17개 + 「첫 sink 도달 «앞»에 `is_null()` 이 있는가」** 를 그대로 구현하면 된다.
★**다시 돌릴 때는 반드시 «정답지 검증»을 먼저 하라** — 이 회차가 그것으로 자기 버그를 잡았다.

## 제안
- p0: 잔여 20건의 **규격 삼분**(NPE 의무 / null 합법 / 도달 불가) 후 첫 갈래만 닫기.
