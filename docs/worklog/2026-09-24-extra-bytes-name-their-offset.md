# 2026-09-24 — «extra bytes» 거부가 클래스가 끝난 바이트 위치를 말한다

티켓 `rustjava-2026-09-23-validation-rules-name-their-position-adopt-p0` · 채택 `2026-09-23-validation-rules-name-their-position#p0`(원 worklog 무접촉).

## 판정 — `ClassInfo::parse` 의 parse-level 거부 3종

| 거부 | 위치를 쥐나 | 근거(실측) | 처분 |
|---|---|---|---|
| truncated or unparsable | 쥐지만 **믿을 수 없다** | nom 오류의 `input` 은 파일의 부분 슬라이스라 오프셋 복원은 된다. 그러나 1바이트 변이(값 0/0xff/0x01 × 전 위치)에서 오프셋이 손상 바이트 ±8B 안에 든 것은 Hello **358/639**, StringConcat **535/1151** — 절반 가까이가 엉뚱한 곳을 가리킨다(예: 풀 개수 바이트 8 손상 → 302 보고). 오류 지점마다 뜻도 다르다: `MapRes` = 감싼 구조의 시작(속성 본문 오류는 속성 시작으로 접힌다) · `Verify`/`Switch` = 필드를 읽은 «뒤». 절단은 Eof 로 전건(417/417 · 848/848) 오프셋 ≤ 절단점이지만 절단점과 8B 넘게 떨어진 경우가 121/417 · 388/848(최대 151B — 속성 본문 `take`). | **위치 없음 유지** |
| extra bytes after the end | ★**정확히 쥔다** — 제안의 「가리킬 자리 없음」은 반증 | `file.len() - remaining.len()` = 파서가 클래스를 끝낸 바이트. 뜻이 하나다. | **`Location::ByteOffset(u32)` 로 싣는다** |
| version predates 45.0 | 없다 | 버전은 늘 바이트 6..8 고정 — 위치는 정보가 아니다(값이 정보이고 그건 이미 문장이 말한다). | 유지 |

## 변경
- `classfile/src/error.rs`: `Location::ByteOffset(u32)` — 여섯째 종류이자 «표 인덱스가 아닌» 유일한 종류. Display `at byte offset N`.
- `classfile/src/class.rs`: extra-bytes 거부 → `InvalidFormatAt { cause(불변), ByteOffset }`. 4 GiB 초과 파일은 `u32` 에 못 담아 종전 `InvalidFormat` 로 떨어진다(틀린 수 대신 수 없음).
- 문면: 전 `extra bytes after the end of the class file` → 후 `extra bytes after the end of the class file (at byte offset 417)`(Hello.class + 3바이트 · `located_message` 경유).
- ★대가: `ClassFileError` 24 → **32 바이트**(64-bit). `u32` 는 `&'static str` 옆에 두 `u16` 처럼 들어가지 않는다. 크기 잠금 시험을 새 값으로 갱신하고 이유를 적었다. `[u16; 2]` 로 쪼개면 24 에 맞지만 공개 API 에 어색한 타입을 둔다 — 기각.
- 공개 API: `Location` 에 변종 추가. `Location` 은 2026-09-23(`c40db08e`)에 생겼고 태그·릴리스 전이라 외부 파괴 0.

## 검증
- 새 단언: Hello.class + `[0,0,0]` → `ByteOffset(417)` · Display 문자열.
- 변이: M1 오프셋=`file.len()` · M2 오프셋=0 · M3 종전 `class.rs` — **전건 red** ↔ 변경 green.
