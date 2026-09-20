# 2026-09-20 — 검사기 출력 순서를 «잠갔다»

채택 제안 `2026-09-19-merge-drops-deterministic-order#p0`.
티켓 `rustjava-checker-output-determinism-has-no-guard`.

## 먼저 잰 것 — 아직 안 고쳐졌나

- `scripts/`·`rust.yml` 을 만진 마지막 커밋은 **`35f34797`**, 그 한 줄 수정 자신이다. 잠금은 없다.
- 추적 파일에서 `PYTHONHASHSEED` 는 **산문(REPORT·STATE·worklog)과 고쳐진 스크립트의 주석에만** 있다 —
  어떤 검사에도 없다.
- 그물이 «0» 이라는 제안의 주장을 **이 트리에서 재현**: 제품 호출부에 비결정성을 되돌려 놓고 재니
  파이썬 검사기 **4종 전건 rc 0** · `cargo fmt --all -- --check` **rc 0**.

## 무엇을 했나

`scripts/check-script-output-order.py` — **`scripts/*.py` 의 어떤 `for`·컴프리헨션도
`sorted(...)` 밖에서 set 을 순회하지 않는다**를 AST 로 단언한다. 새 CI 잡 `script_output_order`
하나 + DoD 블록 10번째 줄(대칭 잠금이 둘을 함께 움직이게 한다).

## 왜 «정적»인가 — 제안이 제시한 두 대안을 다 쓰지 않았다

- **두 `PYTHONHASHSEED` 로 재실행**: set 은 해시 순서가 정렬 순서와 «다를 때만» 눈에 보인다.
  2026-09-19 사고의 경로 2개에서는 그 확률이 약 절반이라, 2회 비교는 회차당 동전 던지기다.
  느린 것이 문제가 아니라 **절반은 눈을 감는다**는 것이 문제다.
- **`assert sorted` 로 직접 단언**: 제안 자신이 적은 약점 그대로 — 「다른 곳에 생긴 두 번째 출처를
  못 잡는다」. 정적 축은 그 두 약점을 **둘 다** 갖지 않는다(재실행 0 · 새 출처 포착 · M2 로 실증).

## 양방향 — 두 개악, 두 복원 (전부 제품 호출부 · 사본 아님)

| | 개악 | 결과 |
|---|---|---|
| M1 | `check-merge-dropped-symbols.py:254` 의 `sorted()` 제거 | **rc 1** · 그 파일·줄을 지목 → 복원 **rc 0** |
| M2 | `check-dod-ci-parity.py:215` `for c in sorted(only_ci)` → `for c in only_ci` (★**다른 파일의 새 출처**) | **rc 1** · 그 파일·줄을 지목 → 복원 **rc 0** |

정상 = `7 script(s): 0 unordered iteration(s) that could reach output` · rc 0 · 0.06초.

## 대가 — 숨기지 않는다

- **새 CI 잡 하나**. 제안이 「real weight」라고 부른 그것이고, 값을 깎지 않았다(툴체인 없는
  checkout + `python3` 두 줄 = 기존 doc 잡 4개와 같은 형상).
- ★**측정된 사각 1건**: 튜플 언패킹으로 함수 반환을 받은 set 은 이 패스가 못 본다.
  오늘 실제로 하나 있다 — `ci_runs, ci_tcs = parse_ci(...)`. 거기에 정렬 없는 `for t in ci_tcs:` 를
  넣어 보니 잠금은 **rc 0** 으로 통과했다. 지금 그 이름은 전부 `sorted()`/집합 연산으로만 읽히므로
  틀린 곳은 없지만, **구멍은 진짜다.** 후속 제안 `#p0` 으로 남겼다(문서에만 두면 썩는 계급이다).
- **dict 는 안 본다**: 삽입 순서를 지키므로 결정성은 그것을 만든 쪽에 달렸고, set 이 dict 를 먹이면
  set 에서 잡힌다.

## 제안 문면보다 넓힌 곳 한 군데

제안의 `target` 은 「scripts/ (all four checkers)」였는데 glob 을 `scripts/*.py` 로 썼다 —
한 단어 차이이고, 포함된 7개(감사·조사 스크립트 2개 + 이 파일 포함) **전건이 오늘 통과**한다.
회차 간 diff 를 하는 것은 검사기만이 아니다.
