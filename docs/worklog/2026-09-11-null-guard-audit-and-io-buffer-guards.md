# 2026-09-11 — null 가드 감사(세기) + `java/io` 버퍼 18곳 가드

채택 제안 `2026-09-11-null-guard-string-init-and-arraycopy#p0`.

선행 회차는 **손으로 열거한** 9곳을 닫았고, ★그 열거 자신이 `arraycopy` 의 `dest` 를 빠뜨렸다.
그래서 이번엔 **술어로 셌다**.

## 감사 — 수와 술어 (★**재측 2026-09-12** · 트리별 병기)

★★**[게이트② F1 정정] 수는 «트리와 함께» 적는다 — 초판이 두 트리를 섞었다.**
정본 = **`scripts/audit-null-guards.py`**(이 회차가 커밋했다). 재현: `python3 scripts/audit-null-guards.py [<runtime-src>]`.

| 트리 | N | M | K |
|---|---|---|---|
| `eaad8e9c`(두 회차 전 · ★초판이 잘못 인용한 트리) | 1,175 | 60 | 46 |
| **base `6da7d66f`**(= 이 PR 의 `merge-base`) | **1,175** | **52** | ★**38** |
| **head** | **1,175** | **34** | **20** |
| **Δ**(base→head) | 0 | **18** | ★**18** = 이 회차의 가드 수(자기정합) |

★★**[게이트² R2] 이 표의 M 은 «한 번 더» 틀렸었다 — 커밋한 스크립트가 2 낮게 셌다**(50/32 로 신고).
근인은 `fn` 정규식 한 줄이다: 이름 뒤 `(` 를 **즉시** 요구하고 접두를 `pub `/`async ` 로만 받아
★**제네릭 fn(`fn sort_primitive<T, F>(`)과 `pub(crate)`/`pub(super)` 선언 34개가 통째로 안 보였다**
(놓친 2건 = `java/util/arrays.rs sort_primitive(array)` · `java/util/timer_task_queue.rs add(task)`).
★고친 뒤 **세 트리 × 세 독립 구현(커밋본·검수 재구현·전 검수)이 전건 일치**한다.
★★**그리고 K 가 흔들리지 않은 것은 «설계가 아니라 운»이다** — 그 2건이 **우연히** `as_proto` 미등재였을 뿐이고,
제네릭·`pub(crate)` 진입점이 **하나라도** 등재됐으면 ★**K 가 조용히 낮아져 이 감사가 「닫혔다」고 답했을 것**이다.
⇒ 그 «눈먼 구간»을 스크립트 docstring 에 박았다(무엇을 못 보는지).

술어: **N** = `rustjava-runtime/src/**` 의 fn 파라미터 중 타입이 `ClassInstanceRef<…>`(★`this`·`_` 접두 제외) ·
**M** = N 중 그 파라미터가 **deref 강제 sink** 에 `&p`/`&mut p`(또는 `p.as_class_instance()`)로 도달하고 ★**그 «앞»에 `p.is_null()` 가 없는** 것 ·
**K** = M 중 그 fn 이 같은 파일 `JavaMethodProto::new(… Self::fn …)` 에 등재 = **게스트가 부를 수 있다**.

★★**초판의 「M 58 · K 46」은 «다른 트리»의 수다** — **`eaad8e9c`**(선행 9곳 회차 «이전» main)에서 재고 라벨만 `6da7d66f` 로 달았다
(★그 트리의 «옳은» 수는 **60/46** 이다 — 58 은 위 R2 의 undercount 가 겹친 값이다).
⇒ ★**`46 − 18 = 28 ≠ 20`** 이라는 산술 모순이 그 혼입의 «증상»이었고, 검수자가 술어를 재구현해 그것으로 잡았다.
★**이 저장소가 반복해 규탄한 「base 를 병기하지 않으면 수가 섞인다」의 교과서적 재현이다.**
★★**[철회] 전 회차가 「N·M 의 차는 구현 세부이고 내 쪽이 정본」이라는 뜻으로 적은 문안을 «철회»한다 — 틀린 것은 내 쪽이었다.**
검수 재구현(N 1,175 · M 60/52/34)이 옳았고, 커밋본을 고치니 **그 값에 정확히 수렴**했다.
※그보다 앞선 검수의 N **1,130** 은 또 다른 구현이고, M/K 는 **52·34 / 38·20 으로 이미 일치**했다.

**deref 강제 sink = 17개**(`jvm/src/jvm.rs` 의 시그니처로 확정 — `&Box<dyn ClassInstance>` · `&mut Box<…>` · `impl AsClassInstance`):
`array_element_type` `array_length` `array_raw_buffer` `array_raw_buffer_mut` `get_field` `interrupt_java_thread`
`invoke_special` `invoke_virtual` `is_java_thread_interrupted` `load_array` `monitor_enter` `monitor_exit`
`object_notify` `object_wait_prepare` `put_field` `shallow_clone` `store_array`

★**왜 sink 를 한정했는가**: 초판은 「`&p` 가 나오면 deref」로 셌는데 그것은 **과대계상**이다 —
`Self::value_range(jvm, &value)` 처럼 **`&ClassInstanceRef<T>` 를 받는** 헬퍼는 deref 하지 «않는다».
한정 전 220 → 한정 후 152 → (off-by-one 수정 후) 58 → ★(**R2 의 `fn` 눈먼 구간까지 수정 후**) **60**. ※전부 `eaad8e9c` 기준.

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
java/io          4   read_utf_from_input(input) · init(file) · init_with_append(file) · init_with_file(file)
java/net         3   init(url) · init_with_context_spec_handler(handler) · set_url(url)
java/util        6   init(list)×4 [abstract_list · array_list_itr · linked_list_itr · vector_itr]
                     · on_access(map) · init(map)
java/util/regex  3   init(pattern) · matches(pattern) · split_with_limit(input)
java/util/zip    3   init_with_zip_entry(zip_entry) · get_input_stream(entry) · init(file)
org/rustjava/net 1   open_connection(url)
                ──   합계 20
```
★★**[게이트² R3 정정] 전 회차의 분해는 오기였다** — `open_connection` 을 `java/net` 으로 세고 `java/util/regex` 3건을 `java/util` 에 합쳐
「4·8·3·1」로 적었다. 위가 도구 출력 그대로다.
★★**일괄 가드는 «틀린다»** — `URL(URL context, String spec, URLStreamHandler handler)` 는 JDK 규격상
**`handler == null` 이 합법**이다(기본 핸들러를 쓰라는 뜻). 실제로 이 repo 의 구현도 그 파라미터를 **섀도잉해 무시**한다
(⇒ 그 1건은 이 계측의 **오탐**이기도 하다 — 이름 기반 절차내 스캔은 섀도잉을 못 본다).
⇒ **후속의 본체는 「규격으로 삼분하는 것」**(NPE 의무 / null 합법 / 도달 불가)이고, 첫 갈래만 닫는다.

## 검증 — 양방향

- **정상 → green**: `cargo test --test test_class` **ok** · DoD 7종 rc=0 · `cargo test --all` **554/0/1**
  (★계수 불변이 맞다 — `test_class` 는 `test-data/` 를 **한 테스트 함수**가 순회한다).
- **개악 → red**: ★★**커버리지를 «추론»하지 않고 «측정»했다 — 18곳을 «하나씩» 빼고 각각 빌드·실행했다**(2회전 = 36빌드).
  ⇒ **12곳 red(덮인다) · 6곳 green(안 덮인다)**. 그 밖에 `getChars` **가드 위치** 원복 개악도 red.
  ★**이 측정이 R1 을 잡는 «지문»이다** — 이미 덮인 가드를 또 덮으면 red 가 «안» 나므로, 중복 케이스가 즉시 드러난다.

## 픽스처 커버리지 — ★**측정값 12/18 · 미커버 6** (게이트² R1 정정)

★★**[R1] 전 회차의 근인 서술이 «뒤집혀» 있었다 — 그 문장이 다음 사람을 오도한다.**
종전 문안: 「`InputStreamReader` 가 **오버라이드하므로** `Reader` base 에 닿지 못한다」 ⇒ ★**정반대다.**
**등재 서술자 실측**: `InputStreamReader` 는 `read` 를 **`([CII)I` 로만** 등재하고 `Reader` 가 **`([C)I`** 를 등재한다
(`OutputStreamWriter` `write([CII)V` ↔ `Writer` `write([C)V` 도 같은 형상).
⇒ ★**오버라이드가 «없어서» 1인자 호출이 `Reader::read([C)` 로 해소된다.**

⇒ 그래서 전 회차가 더한 **중첩 클래스 2케이스는 «이미 덮인 가드»를 또 덮었고** 커버리지는 **10/18 그대로**였다(검수 지적이 옳다).
★**그 2케이스와 `$BareReader`/`$BareWriter` 를 지웠다.**
★**대신 «그 가드가 실제로 사는 서술자»를 겨눴다** — `read(char[],int,int)` · `write(char[],int,int)` **3인자 호출**.
★★**판정 축은 «클래스 이름»이 아니라 «서술자 + 등재 위치»다**(검수가 준 축 그대로).

★★**그리고 이번엔 «추론»하지 않고 «쟀다» — 가드 18곳을 하나씩 빼고 각각 빌드·실행**:

| | 가드 | 판정 |
|---|---|---|
| **덮는다(12)** | `buffered_output_stream.write_bytes` · `data_input_stream.read_fully` · `data_output_stream`×3 · ★`input_stream_reader.read` · ★`output_stream_writer.write` · `reader.read` · `writer.write_chars` · `writer.write_string` · `writer.write_string_offset` · `string.get_chars` | 단일 제거 → **red** |
| ★**미커버(6)** | `file_input_stream.read_array` · `file_output_stream.write_bytes_offset` · `random_access_file`×4 | 단일 제거 → **green** |

★**미커버 6 은 전부 «실파일 핸들이 있어야 인스턴스가 생기는» 것**이고, 테스트가 작업 디렉터리에 파일을 만드는 것을 피했다.
⇒ ★**「가드는 들어갔으나 픽스처가 잠그지 않는다」** — `proposals[1]` 이 진다.
※★전 회차가 「reader/writer 단일 개악이 red 였으니 새 케이스가 문다」고 적은 것은 **논증이 안 된다** —
그 red 는 **기존 `InputStreamReader` 케이스만으로도** 났다. ★**한 가드를 두 케이스가 덮으면 단일 개악은 그 둘을 구별하지 못한다.**

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
