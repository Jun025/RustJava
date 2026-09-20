#!/usr/bin/env python3
"""Refuse an unordered iteration that can reach a checker's output.

Why: on 2026-09-19 `check-merge-dropped-symbols.py` iterated a set of paths, so two runs of the
same command printed the same six findings in two different orders — Python's per-process string
hash seed decides. A before/after diff read as a regression until the unchanged version was shown
to disagree with itself. The fix was one word, `sorted(...)`, and nothing locks it: measured that
round, removing it again leaves `cargo fmt`, `cargo clippy`, `cargo test` and all four python
checkers green. The guard was zero, which is why this file exists.

What it asserts, in one line a reader can check: **no `for` loop or comprehension in
`scripts/*.py` iterates a set unless it is inside `sorted(...)`.**

Why static rather than re-running under two `PYTHONHASHSEED`s: an unordered set is only *visibly*
unordered when the hash order happens to differ from the sorted one, so a two-run comparison is a
coin flip per run — with the two paths of the real incident it agrees with itself about half the
time. This axis has no such gap: it does not need the bug to be observable to see it, it costs no
re-run, and it fires on a *new* set appearing anywhere in these files, not only on the one line the
2026-09-19 round fixed.

Blind spots, stated rather than implied:
  * Dicts are not flagged. They iterate in insertion order, so their output order is as
    deterministic as whatever built them — a set feeding a dict is caught at the set.
  * The set-ness of a value is inferred syntactically (a `set()`/`{…}`/set comprehension, a set
    operator, a name or a local function that carries one). A set arriving from something this
    cannot see — a tuple-unpacked call return, an import, a parameter — is missed, and one such
    name exists today: `ci_runs, ci_tcs = parse_ci(...)` in `check-dod-ci-parity.py` binds a set
    this pass does not know about (it is only ever read through `sorted()` or a set operator, so
    nothing is wrong there now, but an unsorted iteration of it would pass). It is a check for the
    shape that actually bit us, not a type system.
  * Every `scripts/*.py`, not just the CI checkers: the surveys get diffed across rounds too, and
    they cost nothing to include — all of them pass today.

Exit: 0 every iteration is ordered, 1 at least one is not, 2 the files it checks are not there.
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
        growing = False
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


def unordered_iterations(tree, names, funcs):
    """[(line, source)] for every iteration over a set that is not wrapped in sorted()."""
    iterables = [node.iter for node in ast.walk(tree) if isinstance(node, (ast.For, ast.AsyncFor))]
    iterables += [gen.iter for node in ast.walk(tree) for gen in getattr(node, "generators", [])]
    found = []
    for iterable in iterables:
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
        bad = unordered_iterations(tree, names, funcs)
        for line, source in bad:
            print(f"✗ {path.relative_to(ROOT)}:{line}: iterates a set — two runs can print this in two orders")
            print(f"    {source}")
            print("    wrap the iterable in sorted(), as check-merge-dropped-symbols.py does")
        total += len(bad)
        if not bad:
            print(f"  ✓ {path.relative_to(ROOT)}")

    print(f"{len(scripts)} script(s): {total} unordered iteration(s) that could reach output")
    return 1 if total else 0


if __name__ == "__main__":
    sys.exit(main())
