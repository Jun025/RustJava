# 2026-09-23 — charset 보류 술어를 `Charset` 쪽으로 (rustjava-2026-09-23-stale-next-pointer-and-euc-kr-boundary-adopt-p0)

채택 제안 `2026-09-23-stale-next-pointer-and-euc-kr-boundary#p0`.

- **무엇을**: `read()` 의 `charset == "UTF-8"` / `charset == "EUC-KR"` 분기를 `Charset::bytes_to_hold_back(&[u8]) -> usize` 로 옮겼다.
  match 에 `_ =>` 가 없다 — 단일바이트 charset 은 `Iso8859_1 | UsAscii => 0` 으로 명시.
- **반증 ⒝**: `new_stream_decoder` 는 exhaustive 지만 UTF-8·EUC-KR 이 같은 `CharsetStreamDecoder::EncodingRs` 로 접혀
  디코더만으로는 보류 규칙을 가를 수 없다 ⇒ 합칠 곳이 없어 `Charset` 메서드로 뒀다.
- **동작 불변**: 두 술어 본문은 글자 그대로 옮겼다(UTF-8 의 `decode_length > 0` 가드는 `is_empty()` 조기 반환으로).
- **양방향 변이**: 임시 변종 → `bytes_to_hold_back` 에서 non-exhaustive 컴파일 오류 · EUC-KR 보류 0 → #90 경계 테스트 red · 원상 green.
- **한계(제안 자신의 tradeoff 그대로)**: match 가 막는 것은 «잊음»이지 «틀린 술어»가 아니다.
- 후속 추천 0건.
