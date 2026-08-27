# 2026-08-27 — S1~S4 가 착지하고도 fork 가 upstream 에 가까워지지 않은 근인: 게이트③ `--squash`

## 증상

S1~S4(PR #11·#13·#16·#17)가 **전건 `merged=true`** 인데도:

| 축 | 값 (2026-08-27 재실측) |
|---|---|
| `git merge-base origin/main upstream/main` | **`62cf0c6a`** (2026-06-28 `Make ensure_initialized public`) = ★fork 시점 그대로 |
| S1~S4 컷(`1f356ae`·`af4f6f8`·`822504b`·`3296139`)이 `origin/main` 조상인가 | ★**4건 전부 «아니다»** (전건 `upstream/main` 조상임은 확인) |
| `git rev-list --count origin/main..upstream/main` | **33** |
| 착지 PR | #11(7커밋)·#13(10)·#16(15)·#17(21) — 전부 `merged=true` |
| 회차별 충돌 수 | ★**기준을 달지 않으면 비교 불가** — 아래 「회차별 충돌」 절 참조 |

내용은 들어와 있다(`java_runtime/src/charset.rs`·`test_data/UnsupportedCharset.*` 실재).
**들어오지 않은 것은 계보다.**

### 회차별 충돌 — ★**기준 명시**

| 회차 | 그냥 재면(복원 전) | `-s ours` 복원 후 | 계보 상태 | ★재생분(앞 회차가 이미 닫은 자리) |
|---|---|---|---|---|
| S1 | **2** | — (**복원 불요** · fork 시점 base) | 해당 없음 | — |
| S2 | **15** | **5** | 끊김 → 복원함 | ★**15 중 10** |
| S3 | **11** | — (**복원 불요**) | ★**온전**(`merge-base` = `af4f6f8`) | ★**0** |
| S4 | **20** | **2** | 끊김 → 복원함 | ★**20 중 18** |

★★**S3 는 «사례»가 아니라 «반례»다 — 계열에서 빼기만 하지 말고 이 문장을 남겨라.**
S3 는 S2 브랜치 «위에» 쌓아 `merge-base` 가 `af4f6f8` 로 **정상**이었고 `-s ours` 복원을
**하지 않았다**(`s3.done.md` §0-⑴). 그 **11** 은 **계보가 온전한 상태의 순수 신규 충돌**이고
**재생분 0** 이다. ⇒ 「계보 미수렴의 비용」 계열에 넣으면 **근거가 아니라 반증**이 된다.

★**그래서 이 병의 근거는 «충돌 총수»가 아니라 «재생분»이다** — 기준이 하나이고 S3 도 자연히 설명된다:
★**S2 10 → S4 18**(둘 다 계보가 끊긴 회차 · 출처 `s2.done.md` 열거 · `s4.done.md` §2) · **S3 0**(계보 온전).

★**초판은 「2 → 5 → 11 → 20」을 «우상향 계열»로 적고 «원장 기재»라고 표기했는데, 그 계열은 원장에 없다** —
네 수가 서로 다른 기준(S2 는 복원 «후», S4 는 복원 «전»)에서 하나씩 뽑혔고,
기준을 통일하면 복원후 **2·5·11·2** · 복원전 **2·15·11·20** 으로 ★**어느 쪽도 우상향이 아니다**.

## 근인 확정 — 부모 수로 증명

```
PR #11 merged=true pr_commits=7  merge_commit=6bfe97c4 parents=1
PR #13 merged=true pr_commits=10 merge_commit=11ef5010 parents=1
PR #16 merged=true pr_commits=15 merge_commit=4bb796de parents=1
PR #17 merged=true pr_commits=21 merge_commit=3a597768 parents=1
```

**4건 전부 부모 1개 = squash.** 7·10·15·21 커밋이 각각 1커밋으로 접혔다.

### ★가설 반증 시도 — 실패했다(= 가설이 맞다)

「브랜치를 cherry-pick 으로 만들어서 애초에 계보가 없었던 것 아닌가」를 직접 확인했다:

```
$ git cat-file -p 34a4235 | grep '^parent'
parent c80638a…    # 우리 쪽
parent 3296139…    # ★upstream 컷
```

`feat/rustjava-upstream-sync-s4` 브랜치의 `34a4235` 는 **부모 2개인 진짜 머지**이고
`git merge-base --is-ancestor 3296139 feat/rustjava-upstream-sync-s4` → **YES**.
⇒ ★**브랜치는 계보를 갖고 있었고, 게이트③ `--squash` 가 그것을 버렸다.** 다른 설명은 없다.

같은 브랜치의 `c80638a`(「chore: record upstream cut 822504b as merged (tree unchanged)」)는
**S4 회차가 이미 손으로 `-s ours` 계보 기록을 시도한 흔적**인데, 그것 역시 같은 스쿼시에 함께 지워졌다.
⇒ ★**처방을 브랜치에 넣는 것만으로는 부족하다 — 게이트③이 스쿼시하는 한 매 회차 무효화된다.**

## 규율 위치 (문구로 지목 · 줄번호 인용 금지)

- `~/orchestrator/ORCHESTRATOR.md` — 「**제품 repo = `--squash`** / ★**orchestrator-ops = `--merge`(merge commit) · `--squash` 금지**」 (2곳)
- `~/orchestrator/templates/merge-ticket.tpl` — 「`gh pr merge <PR> -R <owner/repo> --squash --subject "$SUBJ"` # 제품 repo(otterpebble·qts·wie·RustJava·dodu)」

★같은 템플릿이 이 병의 **형제 증상을 이미 알고 있다**: 「부모 PR 이 `--squash` 로 착지하면
자식 PR 은 «공통 조상이 사라져» `add/add` 로 전건 충돌한다」. 우리 것은 그 upstream 판이다.

## 처분 — **갈래 ⒞ (⒜+⒝)**

### ⒜ 계보 기록 — 이 PR 이 집행한 것

`origin/main`(`3a59776`) 위에서 `git merge -s ours 3296139`.
`1f356ae`·`af4f6f8`·`822504b` 는 전부 `3296139` 의 조상이므로 **한 번의 머지가 네 컷을 모두 덮는다**.

**트리 변경 0 증명 — 트리 SHA 동일**:

```
tree BEFORE (3a59776): c4f57d10bce2087cebe2e1156f716f6ba8f75335
tree AFTER  (c118d21): c4f57d10bce2087cebe2e1156f716f6ba8f75335
git diff 3a59776..c118d21 → 0 lines
```

★`git diff --stat` 빈 출력은 `-s ours` 에서 정의상 항상 참이라 근거로 약하다(S4 워크로그의 교훈).
그래서 **트리 오브젝트 SHA 자체가 같음**을 근거로 쓴다 — 이쪽은 정의가 아니라 실물이다.

**효과**:

| 축 | 전 | 후 |
|---|---|---|
| `merge-base` vs `upstream/main` | `62cf0c6a` (2026-06-28) | **`3296139c`** (2026-07-19 `Add CLI classpath options (#184)`) |
| behind count | **33** | **18** |
| S1~S4 컷 조상 여부 | 0/4 | **4/4** |

### ⒝ 앞으로의 upstream 동기 PR 은 `--merge` — ★**총괄 소관. 여기서 고치지 않았다**

★**이 PR 자체가 `--squash` 로 착지하면 위 ⒜는 무의미하다** — 부모 2개가 1개로 접히면서
`3296139` 조상 관계가 다시 사라지고, `merge-base` 는 `62cf0c6a` 로 되돌아간다.

⇒ `ORCHESTRATOR.md`·`templates/merge-ticket.tpl` 의 제품 repo `--squash` 규율에
**upstream 동기 PR 예외**를 넣을지는 **총괄이 판정한다**(REPORT.md 후속 추천 (1)).
★**wie 도 같은 함정 위에 있다 — behind 1067.**

### ⒟(기각)를 택하지 않은 이유

기각하려면 다른 근인이 필요한데, 부모 수 4/4 = 1 과 브랜치 쪽 부모 2개가 **동시에** 성립하는
설명은 스쿼시뿐이다. 브랜치 재작성·cherry-pick 가설은 위 반증 시도에서 실측으로 깨졌다.

## 파급 — 잔여 회차

`docs/upstream-sync-approach.md` §5 표의 「새 충돌」은 **base 가 전진한다는 전제 위의 수**였다.
예측 대 실측 — ★**«복원 전» 으로 기준 통일**(계획서가 쓴 `merge-tree origin/main <컷>` 과 같은 기준):
S1 2↔**2** · S2 +5↔**15** · S3 +9↔**11** · S4 **0↔20**.
★**초판은 예측이 맞아 보이는 칸엔 «복원후»(S2 5), 무너져 보이는 칸엔 «복원전»(S4 20)을 짝지었다** — 그 짝을 폐기한다.
S4 는 **복원 전 20 · 복원 후 2** 로 ★**어느 쪽으로 읽어도 「0」 예측을 빗나갔다**.
§5 에 그 전제와 정정을 명시했다(S3 워크로그 `#p0` 제안 채택).

남은 18커밋(S5 `c4665b0` · S6 `95ebc5c` · S7 `ba5797b`)은 **이 PR 이 `--merge` 로 착지한 뒤**
`merge-base` 가 `3296139` 인 상태에서 재측정해야 의미 있는 수가 나온다.

## 하지 않은 것

- **머지 0** — PR 을 만들고 멈춘다(게이트②·③은 별 세션).
- **upstream 발신 0** · **force-push 0** · **history rewrite 0** · **`reset --hard` 0**.
- **제품 코드(`.rs`) 변경 0** — 이 회차는 계보와 규율만 다룬다.
- `~/orchestrator` 무접촉 — 규율 파일은 총괄 소관이라 손대지 않았다.
- `upstream/main` 헤드까지 `-s ours` 하지 **않았다**. S5~S7 내용은 실제로 없으므로
  그것까지 계보를 얹으면 **거짓 주장**이 되고 남은 물량이 조용히 사라진다. `3296139` 까지만이 정확하다.
