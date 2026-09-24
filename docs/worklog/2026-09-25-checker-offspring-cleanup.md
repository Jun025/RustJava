## [2026-09-25] 09-18 채택분이 남긴 호출자 없는 산출물 정리 (rustjava-checker-offspring-and-orphan-artefacts-cleanup)
- 무엇을: `scripts/ldc-tag-survey-targets/`(Kotlin·Scala 85줄)와 그 조사 블록을 지웠다. `docs/test-data-target-policy.md` 의 결정 논거는 `test-data/class-file-versions.txt` 머리로 접고 문서를 지웠다. `audit-fixture-single-defect.py` 를 `AGENTS.md` 의 픽스처 재생성 절차에 1줄로 올렸다. `check-merge-dropped-symbols.py` docstring 을 검사 내용·trailer 규칙·종료 코드만 남겨 84줄에서 38줄로 줄였고, 측정 이력은 아래로 옮겼다.
- 왜: 셋 다 CI·테스트·DoD 어디에서도 부르지 않았다. 정책 문서는 들어오는 참조가 0이었고 같은 논거가 두 곳에 있었다. 검사기 docstring 은 측정 서술이라 09-18 반려 4회 중 3회가 그 수치 정정이었다.
- 사용자 영향: 없음. 검사 로직·CI 잡은 건드리지 않았다. 감사 스크립트는 JDK 없이 0.8초에 rc=0(검사 18 · 결함 없음 5)이라 지우지 않고 수동 경로에 올렸다.

열린 카드 2장(`2026-09-18-root-fixture-target-decision#p0`·`#p1`)의 `target` 이 지운 `docs/test-data-target-policy.md` 를 가리킨다. 그 결정·재개 조건은 이제 `test-data/class-file-versions.txt` 머리에 있다. 카드 처분은 이 회차 범위가 아니라 그대로 둔다.

### `check-merge-dropped-symbols.py` 에서 옮긴 측정 이력 (원문 그대로)

```text
Report definitions that were in the branch a merge pulled from, and are not in its result.

What this answers: *did a conflict resolution silently drop one side's work*. That failure is
invisible to everything else we have, which is why it exists. Measured 2026-09-18 by reconstructing
the loss on `origin/main` and running every axis against it: the generator ran (rc=0), the
single-defect auditor passed (23 checks, rc=0), `test_class_format` passed (21), `test_fixture_pins`
passed (3), `check-dod-ci-parity` passed, `git status` was empty and there was no conflict marker.
Nothing went red. The only signal was counting names by hand, which is what this does instead.

The real incident: two merges on one PR branch (e53b2142, 514d5b08) each dropped four definitions
that came from the branch being merged in -- `Pool.fieldref`, `MAKECONCAT_DESCRIPTOR`,
`make_concat_call_site` and the `LINKED` table. The suite stayed green because the fixtures those
build are committed bytes, so when the generator lost the code the expected output did not move.

Why it is silent, precisely: the loss is invisible exactly when the dropped names are *consistent
with each other*. Dropping a slice that something else still calls raises AttributeError and gets
noticed; dropping a whole feature -- which is what taking one side of a conflict does -- leaves a
file that parses, runs and agrees with every committed artefact. Both were measured.

Scope: a definition is flagged when it exists in the merge's second parent and not in the merge
result, looked for in every file that either the merged-in branch or the merge itself changed.
That second half costs something. Measured by running both filters over one window of 83 merges
(38b0df38..8c7b473f, the last 200 commits of origin/main at the time): the narrow filter reports
8 merges / 19 definitions, this one reports 10 / 26. The seven extra definitions come from two
merges -- 514d5b08 (six) and 37ea5a13 (one) -- and they are not all one kind. Four are the second
of the two real incidents, restored later and present in the tree today. The other three are the
class described next. On time: the measurement noise is larger than the difference between the two
versions -- three independent runs of the narrow filter over this window span 229s to 585s, a factor
of 2.6, while the gap between the versions in one back-to-back run was 70s over 83 merges, about
0.8s per merge. So there is no significant increase to report, which is a weaker claim than "no
slower" and the one the numbers actually support. A pull request carries 0-2 internal merges. The
price of widening is reading, not waiting.

A branch that deletes or renames a definition main still has goes red on its next base pull, and an
ordinary refactor then has to carry a trailer to say so. 37ea5a13 is exactly that: it pulled
origin/main into a branch that had already generalised `at_most_one_bootstrap_methods_attribute`
into `at_most_one_of_each_single_class_attribute`, and the pull reported the old name as dropped.
Three of the seven extra definitions are this class, so it is a class and not a corner. "CI only
looks at origin/main..HEAD" is not a reason to discount it -- that range is precisely where this
lands, on the next base pull of an open branch.

That deliberately also flags *our* intentional deletions, because from the outside the two
look identical -- which is the whole difficulty. Saying which is which is a judgement, so it is
recorded as one, on the merge commit:

    Dropped-from-theirs: method fieldref -- superseded by the new pool builder, see <round>

One trailer per name, a reason after `--`, and both are required. THE NAME MUST BE COPIED FROM THIS
CHECK'S OWN OUTPUT, character for character -- it is matched literally, so `method fieldref` works
and `Pool.fieldref` does not, even though the second reads better. Run the check, copy the name it
prints after the colon, paste it. (An earlier version of this docstring used the prettier form as its
only worked example, which meant anyone who followed it got no exemption and lost a round.)

The list lives in the merge commit message rather than in a file on purpose: a file accumulates
entries that outlive the merge they excused and quietly turns the check off, while a trailer can
only ever excuse the one commit it is written on.

Usage:
    check-merge-dropped-symbols.py [<range>]     # default: origin/main..HEAD

Exit: 0 nothing dropped, or everything dropped is accounted for by a trailer
      1 something was dropped and not accounted for
      2 could not measure -- a git call failed, or the clone is shallow

★ 2 is not a softer 0. Every git call here raises rather than returning nothing, because the one
failure this check must never produce is a green one: in a shallow clone the second parent's objects
are absent, every read of it comes back empty, and the old code printed `✓ (0 file(s) examined)` and
exited 0. Measured on this repository at depth 10: merges that examine 20 files in a full clone
examined 0 and the whole run was green. A check for a silent loss had a silent pass in it.

`-s ours` merges -- "record these upstream cuts as ancestors" -- drop the whole other side on
purpose, and one trailer per name would mean hundreds. They use the wildcard form instead:

    Dropped-from-theirs: * -- recorded as a merged ancestor only, our tree is authoritative

There is deliberately no structural exemption for them. The obvious one, "skip a merge whose tree
equals its first parent's", was written and then removed: it is true of `-s ours` *and* of a
resolution that took our side wholesale, and the second of those is the exact failure this exists
to catch. Measured -- with that exemption in place, a synthetic merge that discarded the other
side's new function passed. So the wholesale case has to be declared rather than inferred.

A squash landing has one parent, so there is nothing for this to compare and it reports nothing.
That is not a gap: the merges this catches are the base-pulls *inside* a branch, and those are
checked while the pull request is open, whichever way the branch is eventually landed.
```
