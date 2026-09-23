# 2026-09-23 — `## 다음` 전수 재측, 그리고 그 밑에서 발견된 EUC-KR 경계 결함

티켓 `rustjava-next-slice-and-stale-next-pointer`. 계기는 `LANE_IDLE rustjava`(1088분 조용 · 큐 0 · running 0).
오라클 판정은 「워커는 멀쩡하고 일이 안 들어온다 ⇒ 처방은 워커 증설이 아니라 발권」이었고, 발권이 멈춘
이유는 `STATE.md` `## 다음` 의 최우선 항목이 **이미 끝난 일**을 가리키고 있었기 때문이다.

## 1. 고른 조각과 왜

티켓이 조각을 지정하지 않았다(그것이 설계였다). `## 다음` 전 절을 읽고 **모든 후보를 직접 재서** 고르는 것이
첫 일이었고, 재고 나니 고를 것이 하나로 좁혀졌다.

| ①~⑤ 가 「다음」이라 부르던 것 | 닫은 커밋 | `--is-ancestor` | 판정 |
|---|---|---|---|
| ① 꼬리 「다음 실작업 = ③의 null-guard」 | `6da7d66f` | ANCESTOR | 닫힘 |
| ② `wie-ktf-hardening` 잔존 2건 | `6da7d66f` | ANCESTOR | 닫힘 |
| ③-1 `upstream-sync-s5…s8` | (S5~S8 전건) | — | 닫힘 |
| ③-2 `null-guard-string-init-and-arraycopy` | `6da7d66f` | ANCESTOR | 닫힘 |
| ③-3 / ④-1 ⒝ 「`makeConcat` 남음」 | `e94cfe91` | ANCESTOR | 닫힘 |
| ④-1 ⒝ 「`LambdaMetafactory` 런타임 남음」 | `8c7b473f` | ANCESTOR | 닫힘 |
| ④-2 「남는 대역 = 1 · `LdcDynamicNoBSM`」 | `d9f45ebf` | ANCESTOR | 닫힘 |
| ④-3 `InputStreamReader` 디코더 경계 | — | — | ★**열림** |

측정 명령(전건 동일 형태):

```sh
sha=$(git log origin/main --format='%H %s' | /usr/bin/grep -F "[<ticket-id>]" | head -1 | cut -d' ' -f1)
git merge-base --is-ancestor "$sha" origin/main && echo ANCESTOR
```

★**워킹트리 `grep` 으로 판정하지 않았다** — 이 저장소의 원장이 반복해 규탄한 형태다.
★**`gh` 는 전건 `-R Jun025/RustJava`** 를 붙였다(fork 부모 오조회 방지).

부재 판정 두 건은 파일로 직접 물었다:

```
$ ls rustjava-runtime/src/classes/java/lang/ | grep -i invoke
(출력 없음)          ← java.lang.invoke 패키지 여전히 0
$ grep -rn "EUC-KR" rustjava-runtime/tests/
test_print_stream.rs:506:  ...   ← InputStreamReader 의 EUC-KR 테스트는 0건
```

⇒ 고른 조각 = **④-3**. 이유는 셋이다. ⑴`## 다음` 에서 **유일하게 열려 있는** 항목이다.
⑵사료가 「①의 머지로 완화책이 들어오니 **다시 재라**」고 명시적으로 남긴 자리라, 재는 것이 곧 계약 이행이다.
⑶남은 다른 후보(`java.lang.invoke` 패키지)는 이 `timeout_min` 에 **들어가지 않는다**(아래 2절).

## 2. 크기 판정과 분할안

- ④-3 재측 + 고침 + 잠금 = **이 회차 안에 끝났다**(실제로 끝냈다).
- ★**`java.lang.invoke` 패키지 신설은 «한 조각이 아니다»** — 이 회차에 넣지 않았다. 분할안:
  1. **`2026-09-17-link-lambdametafactory#p1`(S · 결정)** — 람다 클래스를 리플렉션에 보일지. ★**먼저다**:
     이 결정이 패키지가 무엇을 내놓아야 하는지를 정한다.
  2. **`2026-09-17-link-lambdametafactory#p0`(M)** — 어댑터를 박싱부터 넓힌다. 현재는 어댑터가 필요하면
     **링크하지 않는다**(판정이 lowering 시점이라 「로드되거나 안 되거나」). 여기까지는 패키지 없이 된다.
  3. **`java.lang.invoke` 패키지 신설(L)** — `MethodHandle`/`MethodType`/`CallSite` **객체**. 선행 =
     `2026-09-17-string-concat-recipe-arity#p1` 이 명시적으로 이것을 기다린다.
  ★**1·2 를 패키지보다 먼저 두는 이유**: 지금 링크는 콜사이트를 opcode 로 내려써서 돌고 있고(`string_concat.rs`·
  `lambda.rs`) 객체가 **필요 없다**. 패키지를 먼저 지으면 무엇을 지을지 모르는 채 짓게 된다.

## 3. `## 다음` 전/후

**전** — 「다음」이라 적힌 8항목 중 **7이 닫혀 있었고**, ①의 꼬리가 최우선으로 **닫힌 일**을 가리켰다.
**후** — 새 `⓪살아 있는 후보` 블록을 맨 위에 두고 ①~⑤ 를 통째로 사료로 표시했다. 각 절에는 «닫혔다»는
표시를 그 자리에 붙였다(지우지 않고 접었다 — 이 집의 정정 표기 규율).

닫힌 지목 6자리를 그 자리에서 표시했다: ①의 꼬리 · ②의 절 제목(★본문 표에는 「닫힘」이 이미 적혀 있었는데
**제목만 「2건」으로 남아** 목차만 보는 사람이 속게 돼 있었다) · ③의 절 제목 · ④-1 ⒝ · ④-2 「남는 대역 1」 ·
④-3 · ⑤의 열린 PR 표(★「열린 PR = 1건」이라는 **수만 우연히 맞아서** 27일간 안 고쳐졌다 — #16 은 머지됐고
지금 열린 것은 #81 이다).

★**세 번째 재발을 한 줄로 적었다**(⓪ 블록 머리): ⑴③-0 `…claude-md-prune-disposition`(2026-08-27 해소)이 최우선에 남음
⑵①의 「다음은 S5」가 8일 낡아 2026-09-11 **중복 발권** ⑶그 자리를 고치며 쓴 「③의 null-guard」가 **쓰인 날 이미 닫혀 있었음**.

## 4. 한 일의 전/후 — EUC-KR 경계

`rustjava-runtime/src/classes/java/io/input_stream_reader.rs`.

```rust
// 전
} else if !end_of_input && charset == "EUC-KR" && read_buf_data.last().is_some_and(|value| *value >= 0x81) {
    decode_length -= 1;
}

// 후
} else if !end_of_input && charset == "EUC-KR" {
    let mut index = 0;
    while index < decode_length {
        index += if read_buf_data[index] >= 0x81 { 2 } else { 1 };
    }
    if index > decode_length {
        decode_length -= 1;
    }
}
```

★**왜 전이 틀렸나.** UTF-8 의 역주사는 **연속 바이트(`0x80..=0xbf`)가 선두 바이트와 서로소**라서 성립한다 —
끝에서부터 되짚으면 마지막 시퀀스의 선두를 **모호함 없이** 찾는다. EUC-KR 은 그렇지 않다:
후행 바이트가 선두 범위(`0x81..=0xfe`)와 **겹친다**. 그래서 「마지막 바이트가 선두처럼 보인다」는
**완성된 쌍에 대해서도 참**이고, 그 경우 보류는 그 쌍의 **선두 바이트를 홀로 남긴다**.

그 홀로 남은 선두가 왜 사라지나 — `read()` 는 매 호출 `Charset::resolve(...).new_stream_decoder()` 로
**디코더를 새로 만든다**. `encoding_rs` 는 미완성 시퀀스를 **자기 내부 상태에 버퍼링하고 «소비했다»고 보고**하므로,
코드는 그 바이트만큼 `readBuf` 를 전진시키고 → 디코더는 그 자리에서 버려지고 → ★**그 바이트는 영영 없다.**

★**구조 자체는 고치지 않았다**: `encoding_rs::Decoder` 는 자바 필드에 담을 수 없으니 «보류 휴리스틱»이 설계다.
⇒ ★**다섯째 charset(멀티바이트)을 더하면 이 함정이 그대로 되살아난다.** 후속 카드로 남겼다.

## 5. 개악 대조쌍 — 출력 원문

정상(고침 후):

```
running 8 tests
test classes::java::io::test_input_stream_reader::test_input_stream_reader_does_not_return_zero_for_split_multibyte_input ... ok
test classes::java::io::test_input_stream_reader::test_isr ... ok
test classes::java::io::test_input_stream_reader::test_isr_unsupported_charset_throws ... ok
test classes::java::io::test_input_stream_reader::test_isr_iso_8859_1 ... ok
test classes::java::io::test_input_stream_reader::test_input_stream_reader_rejects_unknown_encoding ... ok
test classes::java::io::test_input_stream_reader::test_reader_default_contract_and_lifecycle ... ok
test classes::java::io::test_input_stream_reader::test_input_stream_reader_preserves_split_multibyte_and_buffered_eof ... ok
test classes::java::io::test_input_stream_reader::test_input_stream_reader_keeps_euc_kr_pairs_across_the_read_buffer_edge ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 480 filtered out; finished in 0.37s
```

★**착수 시점(고치기 «전») 같은 테스트** — 이것이 결함의 실재 증거다:

```
test classes::java::io::test_input_stream_reader::test_input_stream_reader_keeps_euc_kr_pairs_across_the_read_buffer_edge ... FAILED
assertion `left == right` failed
  left: "12345678\u{FFFD}"
 right: "12345678한"
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 487 filtered out; finished in 0.37s
```

**M1 — 종전 술어(`last() >= 0x81`)로 되돌림** → red:

```
test ...::test_input_stream_reader_keeps_euc_kr_pairs_across_the_read_buffer_edge ... FAILED
  left: "12345678\u{FFFD}"
 right: "12345678한"
test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 480 filtered out; finished in 0.08s
```

**M2 — EUC-KR 보류 가지 통째 제거** → red. ★**실패 «모양»이 다르다**(대체문자가 아니라 **소실**):

```
test ...::test_input_stream_reader_keeps_euc_kr_pairs_across_the_read_buffer_edge ... FAILED
  left: "123456789"
 right: "123456789한"
test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 480 filtered out; finished in 0.11s
```

**M3 — UTF-8 역주사 무력화**(`charset == "UTF-8"` → `"NEVER-MATCHES"`) → ★**형제 2건이 red 이고 내 테스트는 green**:

```
test ...::test_input_stream_reader_does_not_return_zero_for_split_multibyte_input ... FAILED
test ...::test_input_stream_reader_preserves_split_multibyte_and_buffered_eof ... FAILED
test ...::test_input_stream_reader_keeps_euc_kr_pairs_across_the_read_buffer_edge ... ok
  left: [65533]
 right: [54620]
  left: 10
 right: 9
test result: FAILED. 6 passed; 2 failed; 0 ignored; 0 measured; 480 filtered out; finished in 0.16s
```

★**M3 이 이 회차에서 가장 값비싼 한 줄이다.** M1·M2 는 「내 고침이 값을 한다」만 말하지만, M3 은
⒜UTF-8 축이 **형제 테스트로 이미 잠겨 있었다**(그래서 사료의 「완화책은 들어온다」가 절반은 참이었다)는 것과
⒝내 새 테스트가 그 형제의 **중복이 아니라 독립 프로브**라는 것을 **동시에** 말한다.
★**green 을 보존의 증거로 읽지 않았다는 근거가 이것이다.**

★**픽스처 3형상은 배선과 같은 커밋**이다(`.class` 픽스처 신설 0 — 이 축은 바이트 배열이라 배선이 곧 테스트다):
⑴`BUF_SIZE` 딱 10바이트(쌍이 readBuf 안에 온전하고 후행이 마지막) ⑵11바이트(쌍이 **진짜로 갈린다** — 선두는 안, 후행은 다음 채움)
⑶두 쌍이 경계를 걸침(한 바이트 보류가 **우연히 맞을 수 없게**).

## 6. 남긴 것

- 「`## 다음` 이 `## 완료` 에 있는 id 를 가리키면 red」라는 기계 강제를 **계산해 보고 기각했다**:
  현재 13개 id 중 **9개가 걸리는데 그중 대부분이 「해소됨 · 발권하지 마라」라고 «옳게» 적힌 주석**이다.
  술어가 「이름이 나온다」이지 「다음이라고 가리킨다」가 아니라서, 오탐이 지배적이다.
  ⇒ ★**못 재는 것을 재는 척하지 않는다.** `AGENTS.md` 의 워크로그 부재 판정과 같은 결론이다.
- `## 다음` 의 정본이 사실은 `docs/worklog/*.json` 의 열린 카드라는 점을 ⓪ 블록에 적었다
  (2026-09-23 실측 **열린 30건** · 그중 upstream 캠페인·형제 repo(wie·qts) 것이 **11건**).
- PR **#81** 이 `CONFLICTING`·`DIRTY` 로 열려 있다(워밍 후 재조회). 게이트③ 미착지 — 이 회차 범위 밖.
