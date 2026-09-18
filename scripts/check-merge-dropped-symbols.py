#!/usr/bin/env python3
"""Report definitions that were in the branch a merge pulled from, and are not in its result.

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
      2 could not run (bad range, no git)

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
"""

import re
import subprocess
import sys

# Both languages the repo builds fixtures and code in. The method patterns matter as much as the
# top-level ones: of the four definitions the real incident dropped, one (`fieldref`) was a method
# inside `class Pool`, so a check that only looked at column zero would have found three of four.
PATTERNS = {
    ".py": [
        (re.compile(r"^def ([A-Za-z_]\w*)"), "def {}"),
        (re.compile(r"^class ([A-Za-z_]\w*)"), "class {}"),
        (re.compile(r"^([A-Z][A-Z0-9_]*)\s*="), "{}"),
        (re.compile(r"^    def ([A-Za-z_]\w*)"), "method {}"),
    ],
    ".rs": [
        (re.compile(r"^(?:pub(?:\([\w:]+\))?\s+)?(?:async\s+)?fn ([A-Za-z_]\w*)"), "fn {}"),
        (re.compile(r"^(?:pub(?:\([\w:]+\))?\s+)?(?:struct|enum|trait|union) ([A-Za-z_]\w*)"), "type {}"),
        (re.compile(r"^(?:pub(?:\([\w:]+\))?\s+)?(?:const|static) ([A-Za-z_]\w*)"), "const {}"),
        (re.compile(r"^    (?:pub(?:\([\w:]+\))?\s+)?(?:async\s+)?fn ([A-Za-z_]\w*)"), "method {}"),
    ],
}


def run(*args):
    result = subprocess.run(["git", *args], capture_output=True, text=True)
    return result.stdout if result.returncode == 0 else None


def symbols(rev, path):
    """The definitions `path` holds at `rev`, or None if it is not a file we read."""
    for suffix, patterns in PATTERNS.items():
        if path.endswith(suffix):
            break
    else:
        return None
    blob = run("show", f"{rev}:{path}")
    if blob is None:
        return set()
    found = set()
    for line in blob.split("\n"):
        for pattern, shape in patterns:
            match = pattern.match(line)
            if match:
                found.add(shape.format(match.group(1)))
                break
    return found


def excused(merge):
    """Names the merge commit itself says were dropped on purpose, with a reason."""
    message = run("log", "-1", "--format=%B", merge) or ""
    names = set()
    for line in message.split("\n"):
        match = re.match(r"^Dropped-from-theirs:\s*(.+?)\s+--\s+(\S.*)$", line.strip())
        if match:
            names.add(match.group(1).strip())
    return names


def excuses_everything(accounted):
    """`Dropped-from-theirs: * -- <why>` covers a merge that keeps our tree on purpose."""
    return "*" in accounted


def check(merge):
    """(findings, files_examined) for one merge commit. findings is a list of (path, name)."""
    parents = (run("rev-list", "--parents", "-n", "1", merge) or "").split()
    if len(parents) < 3:
        return [], 0  # not a merge: a squash or an ordinary commit has nothing to compare
    ours, theirs = parents[1], parents[2]
    base = (run("merge-base", ours, theirs) or "").strip()
    if not base:
        return [], 0
    # Both what the merged-in branch touched and what the merge itself touched. Restricting this to
    # the first set was the original shape and it was wrong: a resolution can revert a file the other
    # branch never touched -- "fixed the conflict in A and put B back" -- and that is this check's
    # whole reason for existing. Measured on the second of the two incidents, 514d5b08, by calling
    # check() on each version: the narrow filter examines 4 files, finds 0 and passes; this one
    # examines 13 and names the same four definitions the first incident dropped. The narrow filter
    # was not blind -- it read four files and still missed it, because none of the four was where
    # the loss landed. The cost is real and is recorded in Scope above.
    changed = set((run("diff", "--name-only", base, theirs) or "").split("\n")) | set(
        (run("diff", "--name-only", base, merge) or "").split("\n")
    )
    findings = []
    examined = 0
    accounted = excused(merge)
    if excuses_everything(accounted):
        return [], 0
    for path in filter(None, changed):
        theirs_symbols = symbols(theirs, path)
        if theirs_symbols is None:
            continue
        examined += 1
        for name in sorted(theirs_symbols - symbols(merge, path)):
            if name not in accounted:
                findings.append((path, name))
    return findings, examined


def main():
    rev_range = sys.argv[1] if len(sys.argv) > 1 else "origin/main..HEAD"
    merges = run("rev-list", "--merges", rev_range)
    if merges is None:
        print(f"cannot read {rev_range}", file=sys.stderr)
        return 2

    merges = [m for m in merges.split("\n") if m]
    total = 0
    for merge in merges:
        findings, examined = check(merge)
        subject = (run("log", "-1", "--format=%s", merge) or "").strip()
        if findings:
            print(f"✗ {merge[:8]} {subject[:60]}")
            for path, name in findings:
                print(f"    {path}: {name} — in the merged-in branch, not in the result")
            total += len(findings)
        else:
            print(f"  ✓ {merge[:8]} ({examined} file(s) examined)")

    print(f"{len(merges)} merge(s) in {rev_range}: {total} definition(s) dropped without a trailer")
    if total:
        print(
            "If a drop was deliberate, say so on the merge commit:\n"
            "    Dropped-from-theirs: <name> -- <why>",
            file=sys.stderr,
        )
    return 1 if total else 0


if __name__ == "__main__":
    sys.exit(main())
