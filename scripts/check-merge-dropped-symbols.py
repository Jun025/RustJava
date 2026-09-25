#!/usr/bin/env python3
"""Report definitions that were in the branch a merge pulled from, and are not in its result.

What it checks: whether a conflict resolution silently dropped one side's work. A definition is
flagged when it exists in the merge's second parent and not in the merge result, looked for in every
file that either the merged-in branch or the merge itself changed. Nothing else catches this: a
dropped *whole feature* still parses, runs and matches every committed artefact.

The wider scope means a branch that deletes or renames a definition main still has goes red on its
next base pull, and an intentional deletion looks exactly like a lost one from outside. Which is
which is a judgement, recorded as a trailer on the merge commit:

    Dropped-from-theirs: method fieldref -- superseded by the new pool builder, see <round>

One trailer per name, and a reason after `--` is required. THE NAME IS MATCHED LITERALLY -- copy it
from this check's own output, after the colon (`method fieldref` works, `Pool.fieldref` does not).
The list lives on the merge commit, not in a file, so it can only ever excuse that one commit.

`-s ours` merges drop the whole other side on purpose and use the wildcard instead:

    Dropped-from-theirs: * -- recorded as a merged ancestor only, our tree is authoritative

There is no structural exemption for them: "tree equals first parent" is also true of a resolution
that took our side wholesale, which is the exact failure this catches, so it must be declared.

A squash landing has one parent and reports nothing; the merges this catches are the base pulls
inside a branch, checked while its pull request is open.

Usage:
    check-merge-dropped-symbols.py [<range>]     # default: origin/main..HEAD

Exit: 0 nothing dropped, or everything dropped is accounted for by a trailer
      1 something was dropped and not accounted for
      2 could not measure -- a git call failed, or the clone is shallow

2 is not a softer 0: every git call raises rather than returning nothing, because in a shallow clone
the second parent's objects are absent and an empty read would otherwise pass green.

Measurement history (the incident, scope and timing numbers): docs/worklog/2026-09-25-checker-offspring-cleanup.md.
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


class CannotMeasure(RuntimeError):
    """git could not answer, so this run knows nothing -- as opposed to knowing there is nothing."""


def run(*args, absence_is_an_answer=False):
    """git's stdout.

    ★ A failing git raises. It used to return None, and every caller wrote `run(...) or ""`, which
    turned "git could not tell me" into "git told me nothing": an empty diff, no parents, no
    symbols -- and then `✓ (0 file(s) examined)` with rc 0. A check that exists to catch a silent
    loss had a silent pass in it, in the one environment (an incomplete clone) where it matters.

    `absence_is_an_answer=True` is for the one call where a non-zero exit is a real answer:
    `git show <rev>:<path>` fails when the path is simply not in that tree, which is ordinary. The
    preflight in main() rules out the other reason that call can fail, so once it has run, a
    failure there means absence and nothing else. Every other call site raises.
    """
    result = subprocess.run(["git", *args], capture_output=True, text=True)
    if result.returncode == 0:
        return result.stdout
    if absence_is_an_answer:
        return None
    raise CannotMeasure(f"git {' '.join(args)} exited {result.returncode}: {result.stderr.strip() or '(no stderr)'}")


def preflight():
    """What has to be true before a failing `git show` can be read as "the path is not there".

    One environment is refused here, and it is the one this check is blind in: a shallow clone does
    not hold the objects a merge's second parent needs, so every read of it fails and every failure
    used to read as "empty". Measured on this repository at depth 10: merges that examine 20 files in
    a full clone examined 0 and the run was green.

    PARTIAL CLONES ARE NOT REFUSED -- decided 2026-09-19, `rustjava-partial-clone-refusal-decision`,
    adopting `2026-09-18-merge-drops-no-silent-git-failure#p0`. Do not "fix" this by adding a
    `remote.origin.partialclonefilter` check here; the question was asked and answered with numbers:

      * A blobless clone (`--filter=blob:none`) with its promisor reachable gives the **identical**
        answer to a full clone -- measured on `e53b2142^..e53b2142`: both rc 1, both 6 dropped
        definitions -- it is only slower (15.7 s vs 2.5 s). Refusing it would refuse a setup that
        works, and unlike a shallow clone it can fetch what it is missing.
      * The failure is narrower than "partial clone": it needs the promisor to be **unreachable**.
        Measured on a fresh blobless clone with the remote pointed at an invalid host, the same range
        printed `0 definition(s) dropped` and exited **0** -- a green run where a full clone reports
        six. That is the silent pass, and it is real.
      * So the fix belongs where the ambiguity is, not in the environment: `symbols()` now asks
        `ls-tree` whether the path is in that tree before reading a failed `show` as absence. Trees
        are present in a partial clone even when blobs are not, so it separates the two without
        matching git's error text (rejected as version-fragile by the round that added `preflight`).

    Detection axis, if it is ever needed: `git config --get remote.origin.partialclonefilter`
    returns `blob:none`, while `rev-parse --is-shallow-repository` returns false -- which is why the
    shallow check above does not catch this case.
    """
    if (run("rev-parse", "--is-shallow-repository") or "").strip() == "true":
        raise CannotMeasure(
            "shallow clone: a merge's second parent is not here, so every read of it would look empty. "
            "Fetch the full history (git fetch --unshallow) and run again."
        )


def symbols(rev, path):
    """The definitions `path` holds at `rev`, or None if it is not a file we read."""
    for suffix, patterns in PATTERNS.items():
        if path.endswith(suffix):
            break
    else:
        return None
    # The only tolerated failure in the file: a path that is not in this tree is a real answer,
    # and it means "no definitions here". Everything else has to be ruled out before that reading
    # is safe -- preflight() rules out a shallow clone, and the `ls-tree` below rules out the other
    # environment that makes `show` fail on a path that *is* there (see partial clones, above).
    blob = run("show", f"{rev}:{path}", absence_is_an_answer=True)
    if blob is None:
        if (run("ls-tree", rev, "--", path) or "").strip():
            raise CannotMeasure(
                f"{rev}:{path} is in that tree but its content could not be read. In a partial clone this is "
                "a blob the promisor remote did not hand over; reading it as 'no definitions here' is the "
                "silent green this check exists to prevent. Restore access to the remote (or run "
                "`git fetch origin` to materialise the missing blobs) and run again."
            )
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
    message = run("log", "-1", "--format=%B", merge)
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
    parents = run("rev-list", "--parents", "-n", "1", merge).split()
    if len(parents) < 3:
        return [], 0  # not a merge: a squash or an ordinary commit has nothing to compare
    ours, theirs = parents[1], parents[2]
    base = run("merge-base", ours, theirs).strip()
    if not base:
        # git succeeded and still named no base: unrelated histories. That is a fact, not a failure,
        # but there is nothing to compare against, so say so rather than pass.
        raise CannotMeasure(f"{merge[:8]}: its parents share no merge base, so there is nothing to diff")
    # Both what the merged-in branch touched and what the merge itself touched. Restricting this to
    # the first set was the original shape and it was wrong: a resolution can revert a file the other
    # branch never touched -- "fixed the conflict in A and put B back" -- and that is this check's
    # whole reason for existing. Measured on the second of the two incidents, 514d5b08, by calling
    # check() on each version: the narrow filter examines 4 files, finds 0 and passes; this one
    # examines 13 and names the same four definitions the first incident dropped. The narrow filter
    # was not blind -- it read four files and still missed it, because none of the four was where
    # the loss landed. The cost is real and is recorded in Scope above.
    changed = set(run("diff", "--name-only", base, theirs).split("\n")) | set(
        run("diff", "--name-only", base, merge).split("\n")
    )
    findings = []
    examined = 0
    accounted = excused(merge)
    if excuses_everything(accounted):
        return [], 0
    # `sorted` so that two runs can be diffed. `changed` is a set of paths, and iterating it takes
    # Python's per-process randomised string hash order: measured on origin/main's version over
    # `e53b2142^..e53b2142`, ten runs under PYTHONHASHSEED=random printed the same six findings in
    # two different orders, differing only in which path's block came first. That cost the round
    # that found it real time -- a before/after diff read as a regression until the unchanged
    # version was shown to disagree with itself. Names *within* a path were already sorted below,
    # so the path order was the only axis left. Sorting here rather than at the print site fixes
    # the order of what `check()` returns, not just what main() happens to print.
    for path in sorted(filter(None, changed)):
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
    try:
        preflight()
        merges = run("rev-list", "--merges", rev_range)
    except CannotMeasure as failure:
        # ★ Not `✓`, and not rc 0. "I could not look" is its own outcome, and the whole point of
        # this file is that it must not be spelled the same way as "I looked and it was clean".
        print(f"cannot measure: {failure}", file=sys.stderr)
        return 2

    merges = [m for m in merges.split("\n") if m]
    total = 0
    for merge in merges:
        try:
            findings, examined = check(merge)
            subject = run("log", "-1", "--format=%s", merge).strip()
        except CannotMeasure as failure:
            print(f"cannot measure: {failure}", file=sys.stderr)
            return 2
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
