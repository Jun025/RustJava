# 2026-09-12 — 파일 기반 IO 가드 6곳을 «회귀로 묶었다»

채택 제안 `2026-09-11-null-guard-audit-and-io-buffer-guards#p1`.

★**「가드가 있다」와 「가드가 잠겨 있다」는 다른 문장이다.** 이 6곳은 가드가 들어간 뒤에도
**지워도 아무도 모르는** 상태였다 — 픽스처가 인스턴스를 만들 수 없었기 때문이다(살아 있는 파일 핸들이 필요하다).
⇒ ★**이 회차의 산출물은 «가드»가 아니라 «가드를 지키는 시험»이다. `.rs` 변경 0.**

## 1. 전수 — ★내가 셌다 (측정 2026-09-12 · 트리 `b50d7d0c` = 이 PR 의 base)

| # | 자리 | 등재 서술자 | 가드 | 시험(착수 시) |
|---|---|---|---|---|
| 1 | `FileInputStream.read_array(buf)` | `read([BII)I` | 있음 | ★**없음** |
| 2 | `FileOutputStream.write_bytes_offset(buffer)` | `write([BII)V` | 있음 | ★**없음** |
| 3 | `RandomAccessFile.read(buf)` | `read([B)I` | 있음 | ★**없음** |
| 4 | `RandomAccessFile.read_offset_length(buf)` | `read([BII)I` | 있음 | ★**없음** |
| 5 | `RandomAccessFile.write(buf)` | `write([B)V` | 있음 | ★**없음** |
| 6 | `RandomAccessFile.write_offset_length(buf)` | `write([BII)V` | 있음 | ★**없음** |

⇒ ★**6 이 맞다**(가드 유무는 소스에서 · 시험 유무는 아래 §3 의 개악 대조로 «측정»했다).

## 2. 픽스처 — `test-data/NullFileIoGuards`

**7케이스**: 위 6곳 + ★**정리 확인 1건**.
- 스크래치 파일 `null-file-io-guards.tmp` 를 **작업 디렉터리 상대 경로**로 만든다(★절대경로 **0** · 권한 가정 **0**).
- `FileOutputStream` 으로 4바이트를 써서 **살아 있는 핸들**을 만든 뒤, 각 진입점에 **null 버퍼**를 넘긴다.
- ★★**정리를 «시험의 일부»로 만들었다** — 끝에서 `delete()` 하고 **`exists()` 를 출력**한다.
  기대 출력이 `gone` 이므로 ★**파일이 남으면 그 자체로 시험이 진다**(사람이 따로 확인할 필요가 없다).

★**흔들림 0**(계약 5): `SKIP` **0** · `command -v` **0** · OS 분기·`System.getProperty` **0** · 절대경로 **0**.
★**3회 재실행 전건 `ok`**(23.4s · 20.8s · 14.4s — 시간만 다르고 결과 동일).

## 3. ★★개악 대조 — «자리마다» 쟀다 (한 자리로 일반화하지 않았다)

가드를 **하나씩** 지우고 각각 빌드·실행. ★**6/6 전건 red**:

```
RED(잠근다)  java/io/file_input_stream.rs   :: read_array(buf)
RED(잠근다)  java/io/file_output_stream.rs  :: write_bytes_offset(buffer)
RED(잠근다)  java/io/random_access_file.rs  :: read(buf)
RED(잠근다)  java/io/random_access_file.rs  :: read_offset_length(buf)
RED(잠근다)  java/io/random_access_file.rs  :: write(buf)
RED(잠근다)  java/io/random_access_file.rs  :: write_offset_length(buf)
```
⇒ ★**착수 시 「시험 없음」이었다는 것도 이 대조가 «증명»한다** — 같은 자리가 이 픽스처 «전»에는 green 이었다(선행 회차 실측).
★매 회차 뒤 트리 복원 확인(`git status` 잔여 **0**) · ★**패닉이 남긴 스크래치 파일도 회차마다 지웠다**(개악 중에는 정리 코드에 도달하지 못한다).

## 4. 정리 확인 (DoD ⒟)

시험 후 `null-file-io-guards.tmp` **부재** · `git status` 에 그 이름 **0건** · ★픽스처 자신의 마지막 케이스가 `gone` 을 단언한다.
※`.gitignore` 에는 넣지 «않았다» — 넣으면 「남아도 안 보인다」가 되어 위 단언이 무력해진다.

## 5. 범위

★**`.rs` 변경 0**(`git status rustjava-runtime/` = 0) — 계약 4 「가드 자체를 바꾸지 마라」.
기존 픽스처·시험 **감소 0**(`test-data/*.class` 108 · `*.txt` 61 — 이번에 각각 +1).
`cargo test --all` **554 / 0 / 1**(★전체 실행 · `test_class` 는 한 함수가 순회하므로 계수 불변이 맞다).

## 제안
- p0: 이 픽스처가 «파일을 쓰는 첫 시험»이다 — 병렬 실행이 도입되면 고정 이름이 충돌한다. 그때 이름에 난수를 넣거나 harness 가 작업 디렉터리를 격리해야 한다(오늘은 `test_class` 가 단일 함수라 무해).
