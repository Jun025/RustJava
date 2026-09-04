# [2026-09-05] `behind` 를 «재는» 자리 신설 — `.github/workflows/upstream-behind.yml`

**티켓**: `rustjava-upstream-behind-measure-scheduled-workflow`
**채택 제안**: `2026-09-04-upstream-sync-cadence-decision#p0`
**성격**: 워크플로 **1개 신설** · 코드(`.rs`) **0줄** · `bin/healthcheck`·orchestrator **무접촉** · S9 **미개시**

---

## 1. 왜 — 임계는 정해졌는데 «재는 주체»가 없었다

2026-09-04 판정(PR #29 · 머지 `4b91e8f8`)이 트리거를 **`behind ≥ 20`** 으로 못박았으나,
★**그 수는 «사람이 생각날 때만» 재졌다.** 총괄 재실측이 그 전제를 확인했다 —
`git grep -lI 'rev-list --count' origin/main -- .github/ scripts/` → ★**0건**.

## 2. ★★알림 방식 — ⒝(Job Summary + annotation · 항상 green)

★**이 회차 결정의 «대부분»이 여기다**(제안이 그렇게 적었다). ★**취향이 아니라 이 저장소 실측으로 골랐다.**

| 안 | 실측 | 판정 |
|---|---|---|
| ⒜ **red** | ★선례 `rust-audit.yaml` 이 최근 **20 run 중 19 failure** 인데 ★**대응 티켓 «0건»**(제목 기준 전수) · `coverage` 리니지도 만성 red 를 «행동»이 아니라 «green 으로» 끝냈다 | ★**여기서 «안 듣는다»** ⇒ 기각 |
| 〃 부작용 | ★예약 red 는 **main tip 의 check-run 에 붙는다**(실측 `8c1238b5` 에 `audit` failure **7건**) ⇒ 게이트③의 「착지본 green」 읽기를 흐린다 | 〃 |
| 〃 파급(티켓이 확인하라 한 것) | ★**PR 은 «막지 않는다»** — 예약 run 은 **main tip** sha 에 붙고 PR head 와 다르다(내 회차 전건에서 PR head check-run 에 `audit` **0건**) | ★**막지는 않는다**(그래도 위 두 이유로 기각) |
| ⒞ **이슈** | ★이 fork 는 ★**issues 가 비활성**(`hasIssuesEnabled: false`) | ★**구조적으로 불가** |
| ★**⒝ Job Summary + annotation** | 부작용 0 · `gh` 로 조회 가능 · main tip 오염 0 | ★**채택** |

★★**⒝ 의 «수동성»을 알고 고른다** — 그것이 이 선택의 대가이고, ★**재개 조건이 그 대가를 «잰다»**(§5).
★**정본 «읽기»는 여전히 로컬 한 줄**(`git rev-list --count`)이다 — 워크플로는 그것을 대체하지 않고
★**«아무도 안 물어도 재는»** 역할이다.

## 3. ★주기 — 주 1회(`cron: "17 6 * * 1"`)

★**선례를 «베끼지» 않았다** — `rust-audit.yaml` 은 **일간**(`0 0 * * *`)이지만 그 주기는 보안 권고 축의 것이다.
임계 도달 속도로 정당화한다(§⒞ 실측 · 12개월 144커밋/51주):

| 축 | 값 |
|---|---|
| `behind 20` 도달 | 중앙 **44일** · ★**최소 9일** · 최대 139일 |
| 고른 주기 | ★**7일** |
| 근거 | ★**7 < 9** — «가장 빠른 관측 상승»보다 짧아 **한 주기 넘게 놓칠 수 없다** |
| 일간을 안 고른 이유 | **7배 비용**에 판단 가치 0 — ★**회차보다 빨리 움직일 수 없다** |
| 오프셋 | 정시(큐 혼잡)와 `rust-audit` 의 **00:00 슬롯**을 피했다 |

## 4. ★한 번 «돌려서» 낸 오늘의 값

워크플로 본문의 «그 명령»을 그대로 태웠다(신선한 클론 · upstream remote 추가 → `fetch` → `rev-list --count`):

```
behind        = 0
threshold     = 20
merge-base    = bd42427
upstream HEAD = bd42427
판정          = 임계 미만 — 할 일 없음
```

## 5. ★재개 조건 — «수동성»이 대가를 치렀는지 잰다

> ★**`behind ≥ 20` 이 «7일 이상» 지속됐는데 그 사이 동기 회차가 «열리지 않았으면» 알림 방식을 다시 연다.**

```sh
git fetch -q upstream && git rev-list --count origin/main..upstream/main
git log -1 --format=%ci --grep='upstream 동기' --first-parent origin/main
```
★**오늘의 값 = behind `0` · 마지막 동기 착지 `a76b305`(S8 · 2026-09-05 01:46:33 +0900)**
⇒ ★**아직 대가를 치르지 않았다**(관측 시작점). ★**그때 ⒞(issues 를 켠다) ↔ ⒜(위 부작용 감수)를 «다시» 저울질하라.**

## 6. 경계

★**upstream 발신 0** — remote 추가 + `fetch` 뿐(push·PR·issue·comment **0**) · `permissions: contents: read`.
★**자동 발권 0** — 임계를 넘어도 워크플로가 **회차를 열지 않는다**(발권은 사람 · 판정이 그렇게 정했다).
★**임계 20 무접촉**(착지 판정 PR #29) · ★**S9 미개시**(behind **0**) · ★**`bin/healthcheck`·orchestrator 무접촉** ·
★**`scripts/` 무접촉**(파리티 검사기 `rc=0` — 그 검사기는 `CI_FILE` 을 `rust.yml` 로 **코드에 박아** 둬서 새 워크플로를 보지 않는다) ·
★`.rs` **0줄** · 머지 **0** · force-push **0** · `main` 직접 push **0** ·
★게이트③은 ★**`--squash` 금지 저장소**(`contracts/upstream-sync-repos.conf` 등재) ⇒ `merge_strategy: merge`.
