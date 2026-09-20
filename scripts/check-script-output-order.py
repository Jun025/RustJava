#!/usr/bin/env python3
"""Refuse a set read in a position whose order reaches a checker's output.

Why: on 2026-09-19 `check-merge-dropped-symbols.py` iterated a set of paths, so two runs of the
same command printed the same six findings in two different orders — Python's per-process string
hash seed decides. A before/after diff read as a regression until the unchanged version was shown
to disagree with itself. The fix was one word, `sorted(...)`, and nothing locks it: measured that
round, removing it again leaves `cargo fmt`, `cargo clippy`, `cargo test` and all four python
checkers green. The guard was zero, which is why this file exists.

What it asserts, in one line a reader can check: **in `scripts/*.py`, no set is read in one of the
three positions whose order reaches output — the iterable of a `for` or a comprehension, the first
argument of `str.join`, or a `*`-unpacking — unless it is inside `sorted(...)`.**

The last two were added after a review wrote `", ".join(myset)` and `print(*myset)` and watched both
pass. That is the 2026-09-19 incident exactly, minus the loop: a set of paths printed in hash order.
Three positions is not "every position", and the sentence above says so rather than promising a
coverage this does not have — the list below is the rest.

Why static rather than re-running under two `PYTHONHASHSEED`s: an unordered set is only *visibly*
unordered when the hash order happens to differ from the sorted one, so a two-run comparison is a
coin flip per run — with the two paths of the real incident it agrees with itself about half the
time. This axis has no such gap: it does not need the bug to be observable to see it, it costs no
re-run, and it fires on a *new* set appearing anywhere in these files, not only on the one line the
2026-09-19 round fixed.

Blind spots — listed rather than implied, and none of them is a claim of safety:
  * **Order laundered through a container is not followed.** `d = dict.fromkeys(myset)` and then
    `for k in d` keeps the set's order and passes; so do `y = list(myset)` then `for x in y`, and
    `bag["k"] = myset` then `for x in bag["k"]`. An earlier version of this file said "a set feeding
    a dict is caught at the set" — that was **false**, shown by a review that ran it, and a wrong
    blind-spot entry is worse than a missing one because a reader takes it as a guarantee. Measured
    on this tree: `dict.fromkeys` appears **0 times**, so this is a hole in the promise and not a
    live miss. Closing it properly means following order taint through containers, which is a
    different program from this one; doing `fromkeys` alone would buy the *look* of that program for
    two lines, which is the failure this paragraph is about.
  * **Set-ness is inferred syntactically** — a `set()`/`{…}`/set comprehension, a set *operator*, or
    a name or local function carrying one. So a set that arrives some other way is missed: a
    tuple-unpacked call return, an import, a parameter. One such name exists today,
    `ci_runs, ci_tcs = parse_ci(...)` in `check-dod-ci-parity.py` — read only through `sorted()` or a
    set operator, so nothing is wrong there now, but an unsorted read of it would pass. Method-
    spelled set operations (`a.difference(b)`) are not read either, only the operators (`a - b`).
  * **Names have no scope.** `found = set()` in one function and `found = [...]` in another make the
    *list* read go red. No such collision exists today, but `found` is one of the names from the
    original incident, so the false red is reachable — and a false red invites a wrong `sorted()`,
    which is a worse outcome than a miss.
  * **Draining is not reading.** `while s: s.pop()` takes a set in hash order and passes (0 today).
  * Every `scripts/*.py`, not just the CI checkers: the surveys get diffed across rounds too, and
    they cost nothing to include — all of them pass today.

Exit: 0 nothing found, 1 at least one unordered read, 2 the files it checks are not there.
"""

import ast
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SET_OPS = (ast.BitOr, ast.BitAnd, ast.Sub, ast.BitXor)


def produces_set(node, names, funcs):
    """Is this expression a set, as far as syntax can tell."""
    if isinstance(node, (ast.Set, ast.SetComp)):
        return True
    if isinstance(node, ast.Call) and isinstance(node.func, ast.Name):
        return node.func.id in ("set", "frozenset") or node.func.id in funcs
    if isinstance(node, ast.BinOp) and isinstance(node.op, SET_OPS):
        return produces_set(node.left, names, funcs) or produces_set(node.right, names, funcs)
    if isinstance(node, ast.Name):
        return node.id in names
    return False


def set_values(tree):
    """(names, functions) that carry a set. Iterated to a fixpoint because one feeds the other:
    `found = set()` makes `symbols()` a set-returning function, which makes `theirs_symbols` a set.
    """
    names, funcs = set(), set()
    growing = True
    while growing:
        before = len(names) + len(funcs)
        for node in ast.walk(tree):
            if isinstance(node, ast.Assign):
                pairs = []
                if isinstance(node.value, ast.Tuple) and isinstance(node.targets[0], ast.Tuple):
                    pairs = list(zip(node.targets[0].elts, node.value.elts))  # a, b = set(), []
                else:
                    pairs = [(t, node.value) for t in node.targets]
                for target, value in pairs:
                    if isinstance(target, ast.Name) and produces_set(value, names, funcs):
                        names.add(target.id)
            elif isinstance(node, ast.AugAssign) and isinstance(node.target, ast.Name):
                if produces_set(node.value, names, funcs):
                    names.add(node.target.id)
            elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                for inner in ast.walk(node):
                    if isinstance(inner, ast.Return) and inner.value is not None and produces_set(inner.value, names, funcs):
                        funcs.add(node.name)
        growing = len(names) + len(funcs) > before
    return names, funcs



def order_reaching_output(tree):
    """Every expression whose element order can end up in a printed line.

    Three shapes, and the docstring's promise is exactly these three: what a `for` or comprehension
    walks, what `str.join` is handed, and what a `*` spreads. A `Starred` in a target
    (`a, *rest = ...`) is a Store and is not one of them.
    """
    for node in ast.walk(tree):
        if isinstance(node, (ast.For, ast.AsyncFor)):
            yield node.iter
        for generator in getattr(node, "generators", []):
            yield generator.iter
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.func.attr == "join" and node.args:
            yield node.args[0]
        if isinstance(node, ast.Starred) and isinstance(node.ctx, ast.Load):
            yield node.value


def unordered_reads(tree, names, funcs):
    """[(line, source)] for every set read in one of those positions without a sorted() over it."""
    found = []
    for iterable in order_reaching_output(tree):
        pending = [iterable]
        while pending:
            node = pending.pop()
            if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id == "sorted":
                continue  # anything under a sorted() is ordered, however it was built
            if produces_set(node, names, funcs):
                found.append((iterable.lineno, ast.unparse(iterable)))
                break
            pending.extend(ast.iter_child_nodes(node))
    return sorted(found)


def main():
    scripts = sorted((ROOT / "scripts").glob("*.py"))
    if not scripts:
        # A move must not turn this into a green run over nothing — the sibling lineage's
        # whole subject is checks that pass by looking at zero things.
        print("cannot measure: no scripts/*.py to read", file=sys.stderr)
        return 2

    total = 0
    for path in scripts:
        tree = ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
        names, funcs = set_values(tree)
        bad = unordered_reads(tree, names, funcs)
        for line, source in bad:
            print(f"✗ {path.relative_to(ROOT)}:{line}: reads a set — two runs can print this in two orders")
            print(f"    {source}")
            print("    wrap it in sorted(), as check-merge-dropped-symbols.py does")
        total += len(bad)
        if not bad:
            print(f"  ✓ {path.relative_to(ROOT)}")

    # The count names the three positions it looked at rather than claiming "could reach output":
    # those are not the same set, and the docstring's blind spots are the difference.
    print(f"{len(scripts)} script(s): {total} set(s) read unordered in a for/comprehension, str.join or *unpacking")
    return 1 if total else 0


if __name__ == "__main__":
    sys.exit(main())
