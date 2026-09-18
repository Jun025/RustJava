# 「조용한 실패」를 잡는 검사기에 «조용히 통과하는 길»이 있었다

## ⓐ 재현 — ★**합성이 아니라 «진짜 얕은 클론»에서 났다**

`scripts/check-merge-dropped-symbols.py` 의 `run()` 이 git 실패를 `None` 으로 삼키고,
호출부가 전부 `(run(...) or "")` 로 받아 ★**「git 이 못 답했다」가 「git 이 «없다»고 답했다」로 접혔다.**

이 저장소를 **깊이 10** 으로 얕게 클론해 그대로 돌렸다(`git clone --depth 10`):
```
--- 전(착지 전 판본)                                    rc=0
  ✓ 97660921 (0 file(s) examined)
  ✓ 56bb54fa (0 file(s) examined)
11 merge(s) in …: 0 definition(s) dropped without a trailer
--- 후(이 회차)                                          rc=2
cannot measure: shallow clone: a merge's second parent is not here, so every read of it would
look empty. Fetch the full history (git fetch --unshallow) and run again.
```
★**대조 — 같은 머지를 «완전 클론»에서 재면**: `56bb54fa` 는 **examined 20**, `430fef8a` 는 **11** 이다.
⇒ ★**20개를 보던 머지가 0개로 접히고 그 run 전체가 green 이었다.** 검사기가 막으려는 실패 양식 그 자체다.

★**preflight 만이 아니라 «raise 경로»도 확인했다**(셋 다 rc=2 · 문면에 git stderr 를 그대로 싣는다):
```
잘못된 범위      → cannot measure: git rev-list --merges no-such-ref..HEAD exited 128: fatal: ambiguous argument…
루프 «안» diff 실패 → cannot measure: git diff exited 128: …            ← 머지별 검사 도중
git 아닌 디렉터리 → cannot measure: git rev-parse --is-shallow-repository exited 128: fatal: not a git repository
```

## ⓑ 호출부 구분표 — ★**급소는 «실패»와 «빈 결과»를 가르는 것이다**

`run()` 호출부는 **8곳**이고, ★**「실패 시 옳은 동작」이 한 곳만 다르다**:

| 자리 | 호출 | 실패의 뜻 | 옳은 동작 | 처분 |
|---|---|---|---|---|
| `symbols()` | `show <rev>:<path>` | ★**둘이 섞인다** — ⑴그 트리에 그 경로가 없다(**정상**) ⑵객체가 없다(**실패**) | ⑴은 `set()`, ⑵는 실패 | ★**`absence_is_an_answer=True`** + **preflight 로 ⑵를 배제** |
| `excused()` | `log -1 --format=%B` | trailer 를 못 읽는다 ⇒ 면제가 «덜» 적용돼 false red | 실패 | raise |
| `check()` | `rev-list --parents -n 1` | 부모를 못 읽는다 ⇒ **「머지가 아니다」로 오독** → 조용한 통과 | 실패 | raise |
| `check()` | `merge-base` | base 를 못 얻는다 ⇒ `[],0` → 조용한 통과 | 실패 | raise |
| `check()` ×2 | `diff --name-only` | 변경 파일 목록이 빈다 ⇒ **examined 0** → 조용한 통과 | 실패 | raise |
| `main()` | `rev-list --merges` | 범위를 못 읽는다 | 실패 | raise |
| `main()` | `log -1 --format=%s` | 제목을 못 읽는다(표시용) | 실패 | raise |

★★**`show` 만 «실패가 답»인 이유**: `git show <rev>:<path>` 는 **경로가 그 트리에 없을 때도** 실패한다.
그리고 그 경우는 **흔하고 정상**이다(머지가 «새로 추가한» 파일은 `theirs` 에 없다).
⇒ rc 만으로는 못 가른다 ⇒ ★**「객체가 없을 수 있는 환경」 자체를 preflight 로 먼저 배제**하고,
그 뒤의 `show` 실패는 **부재로만** 읽는다. ★그 전제를 코드 주석에 적었다(다음 사람이 preflight 를 지우면 이 가정이 깨진다).

## ⓒ CI — ★**이미 `fetch-depth: 0` 이다. 그래서 이 변경이 PR 을 막지 않는다**

`.github/workflows/rust.yml` 의 `merge_drops` job:
```yaml
  # … fetch-depth 0 because the check needs the merge commits and their parents,
  # which a shallow checkout does not have. One runner, not the matrix.
  merge_drops:
    steps:
      - uses: actions/checkout@v7
        with:
          fetch-depth: 0
      - run: python3 scripts/check-merge-dropped-symbols.py
```
★**rc≠0 은 그 step 을 실패시키고 job 이 빨개진다**(마지막 step 이라 job rc 가 곧 스크립트 rc).
★**그리고 얕은 클론은 그 job 에서 «구조적으로 일어나지 않는다»** — 다른 job 들은 기본(얕음)이지만 이 검사기를 돌리지 않는다.
⇒ ★**「즉시 전 PR 을 막는다」는 일어나지 않는다.** 계약 2 가 「재라」고 한 그 수를 재서 적는다: **해당 job 의 얕은 클론 빈도 = 0**.
★이 사실 때문에 처방이 «객체를 먼저 받는 것»이 아니라 **«rc 를 올리는 것»** 으로 정해졌다 — CI 는 이미 받고 있고,
남은 위험은 **사람이 손으로 돌리는 얕은 트리**뿐이며 거기서는 «빨강 + 처방 문면»이 옳다.

## 무엇을 바꿨나 — 로직은 «실패 처리»만

- `CannotMeasure` 예외 신설 · `run()` 이 **raise**(단 `absence_is_an_answer=True` 한 자리만 `None`).
- `preflight()` — 얕은 클론이면 즉시 `CannotMeasure`.
- `main()` 이 두 자리에서 잡아 ★**`cannot measure: …` + rc=2**.
- 종료코드 계약은 ★**이미 있던 `2`** 를 쓴다(「could not run」 → 「could not measure」로 문면만 정확히).
★**판정 술어(무엇을 dropped symbol 로 보는가)·필터 폭(narrow/wide)·PATTERNS 무접촉.**

## 양방향

| 축 | 결과 |
|---|---|
| ⒜ **git 실패** | 얕은 클론 **rc=0 `✓ (0 file(s) examined)` → rc=2 「cannot measure」** · 범위 오류/루프 내 실패/비-git **전부 rc=2** |
| ⒝ **정상인데 examined=0** | `97660921`(`.md`/`.json` 만 바뀐 머지 · ★완전 클론에서도 진짜 0) → ★**여전히 `✓` rc=0** |
| ⒞ **정상 탐지 회귀 0** | `e53b2142` **findings 6** · `514d5b08` **6** · `56bb54fa` examined **20** · `430fef8a` **11** — 전부 변경 전과 동일 |
★⒝ 가 없으면 이 변경은 「검사기를 더 시끄럽게 한 것」과 구별되지 않는다. ★그 사례를 **합성하지 않고 생산 데이터에서** 골랐다.

## 잃는 것 — 「없다」로 적지 않는다

- ★**손으로 얕은 트리에서 돌리던 사람은 이제 빨강을 본다.** 그 전에는 초록이었다 — ★**그 초록이 거짓이었다**는 것이 이 회차의 요지이고,
  처방 문면(`git fetch --unshallow`)을 메시지에 넣었다. ★그래도 **처음 겪는 사람에게는 «새로 깨진 것»으로 보인다.**
- ★**`excused()`·`log --format=%s` 처럼 «표시용·면제용» 호출까지 raise 로 올렸다** — 그쪽 실패는 원래 false red 나 빈 제목으로 끝났다.
  ⇒ ★**더 자주 멈출 수 있다.** 일괄로 올린 이유는 「어느 실패가 안전한가」를 자리마다 판단하면 그 판단이 낡기 때문이고,
  안전한 예외는 **`show` 한 곳뿐**임을 표로 못박았다. ★그 선택의 대가는 «과민»이다.
- ★★**preflight 는 «얕음»만 본다** — 객체가 빠지는 다른 경로(부분 클론 `--filter=blob:none` · 손상된 객체)는
  **여전히 `show` 의 «부재»로 읽힐 수 있다.** ⇒ ★**이 회차가 닫은 것은 «가장 흔한 한 갈래»이지 전부가 아니다.**
  그 경우 `diff`·`rev-list` 쪽은 raise 로 잡히지만, **`show` 만 실패하는 형상은 남는다.**
- ★**F8 의 「0」이 «정말 이 경로로» 났는지는 증명하지 못한다** — 그 회차의 실행 기록이 없다. ★가설과 정합할 뿐이다.

## 후속 추천

⑴★**부분 클론·손상 객체까지 preflight 로 덮을 것인가**(S) — 위 「잃는 것」 셋째. `--filter` 로 받은 트리에서
   `show` 가 부재와 구별되지 않는다. ★비용은 preflight 가 무거워지는 것이고, 이득은 **남은 한 갈래**다.
⑵★**`show` 의 두 실패를 stderr 로 가를 것인가**(S) — `fatal: path '…' does not exist` ↔ 객체 부재는 문면이 다르다.
   ★문면 의존이라 git 판올림에 약하다 — 그래서 이번에는 **환경을 배제하는 쪽**을 골랐다. 그 판단을 다시 볼 자리다.
