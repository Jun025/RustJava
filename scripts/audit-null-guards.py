#!/usr/bin/env python3
"""Count runtime entry points where a null `ClassInstanceRef` argument would abort the host.

★This is an AUDIT, not a lock. It is deliberately NOT wired into CI: it reports numbers,
 it does not gate. Nothing fails because this script prints a large K.

Why it is committed at all (2026-09-11, gate2 F5): the round that first produced these
numbers could not cross-check its own arithmetic, and shipped a K measured on the WRONG
TREE. One executable definition of the predicate is cheaper than re-deriving it from prose.

    python3 scripts/audit-null-guards.py [<runtime-src-dir>] [--json <out>]

To measure another commit, materialise it and point at that tree — the script never
consults git, so the tree you pass is the tree you measure:

    git worktree add /tmp/t <sha>
    python3 scripts/audit-null-guards.py /tmp/t/rustjava-runtime/src

Predicate
---------
N  fn parameters typed `ClassInstanceRef<…>`, excluding `this` and `_`-prefixed.
M  of N, the parameter reaches a DEREF-FORCING SINK as `&p` / `&mut p`
   (or `p.as_class_instance()`) with no `p.is_null()` earlier in the body.
K  of M, the fn is registered via `JavaMethodProto::new(…, Self::fn, …)` in the same
   file, i.e. guest bytecode can call it, i.e. null can actually arrive.

SINKS are the `Jvm` methods that take `&Box<dyn ClassInstance>`, `&mut Box<…>` or
`impl AsClassInstance` — passing a `ClassInstanceRef` there forces Deref/DerefMut/
AsClassInstance, all three of which are `.unwrap()` on an Option (see
`jvm/src/class_instance.rs`). That unwrap is the host abort.

Known limits — K is a LOWER bound, and not a clean one:
  * intraprocedural only. A fn that derefs inside a helper is invisible (this is why
    `String::init_with_string` was missed: it derefs inside `Self::value_range`).
  * name-based, so shadowing yields false positives (`java/net/url.rs` flags a `handler`
    parameter that the body shadows and ignores).
  * `null can arrive` is approximated by `registered in as_proto`; it does not ask
    whether the JDK spec actually permits null there (for `URL(context, spec, handler)`
    a null handler is LEGAL, so a guard would be wrong).

★When you re-run this, validate it against a known answer key BEFORE trusting the output:
 measure the tree before and after a round that added guards, and check the set difference
 equals exactly that round's guards. That check is what caught an off-by-one here once.
"""

import argparse
import collections
import json
import pathlib
import re
import sys

SINKS = (
    "array_element_type", "array_length", "array_raw_buffer", "array_raw_buffer_mut",
    "get_field", "interrupt_java_thread", "invoke_special", "invoke_virtual",
    "is_java_thread_interrupted", "load_array", "monitor_enter", "monitor_exit",
    "object_notify", "object_wait_prepare", "put_field", "shallow_clone", "store_array",
)

FN = re.compile(r"(?:^|\n)\s*(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z0-9_]+)\s*\(")
PARAM = re.compile(r"([A-Za-z0-9_]+)\s*:\s*(?:&\s*)?(?:mut\s+)?ClassInstanceRef\s*<")
PROTO = re.compile(r'JavaMethodProto::new\(\s*"[^"]*"\s*,\s*"[^"]*"\s*,\s*Self::([A-Za-z0-9_]+)')
SINK_CALL = r"\.\s*(?:" + "|".join(SINKS) + r")\s*\(\s*&\s*(?:mut\s+)?\**\s*"


def _balanced(text, start, open_ch, close_ch):
    """Index of the closer matching the opener at `start`, or -1."""
    depth = 0
    for i in range(start, len(text)):
        if text[i] == open_ch:
            depth += 1
        elif text[i] == close_ch:
            depth -= 1
            if depth == 0:
                return i
    return -1


def split_fns(text):
    """Yield (name, params_src, body_src). All indices address `text` itself."""
    for m in FN.finditer(text):
        paren = m.end() - 1
        close = _balanced(text, paren, "(", ")")
        if close < 0:
            continue
        brace = text.find("{", close)
        if brace < 0:
            continue
        end = _balanced(text, brace, "{", "}")
        if end < 0:
            continue
        yield m.group(1), text[paren + 1:close], text[brace:end]


def audit(root):
    rows, n = [], 0
    for path in sorted(pathlib.Path(root).rglob("*.rs")):
        text = path.read_text(encoding="utf-8", errors="replace")
        protos = set(PROTO.findall(text))
        for name, params, body in split_fns(text):
            for pm in PARAM.finditer(params):
                p = pm.group(1)
                if p == "this" or p.startswith("_"):
                    continue
                n += 1
                hits = [m.start() for m in re.finditer(SINK_CALL + re.escape(p) + r"\b", body)]
                hits += [m.start() for m in
                         re.finditer(re.escape(p) + r"\s*\.\s*as_class_instance\s*\(", body)]
                if not hits:
                    continue
                guard = re.search(r"\b" + re.escape(p) + r"\s*\.\s*is_null\s*\(\s*\)", body)
                if guard and guard.start() < min(hits):
                    continue
                rows.append({"file": str(path), "fn": name, "param": p,
                             "guest_reachable": name in protos})
    return n, rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("root", nargs="?", default="rustjava-runtime/src")
    ap.add_argument("--json", dest="out")
    args = ap.parse_args()

    n, rows = audit(args.root)
    k_rows = [r for r in rows if r["guest_reachable"]]
    print(f"tree: {args.root}")
    print(f"N (ClassInstanceRef params, excl. this/_) = {n}")
    print(f"M (unguarded, reaches a deref-forcing sink) = {len(rows)}")
    print(f"K (of M, registered in as_proto)            = {len(k_rows)}")
    print()
    by = collections.defaultdict(list)
    for r in k_rows:
        by[r["file"].split("/classes/")[-1]].append(f'{r["fn"]}({r["param"]})')
    for f in sorted(by):
        print(f"  {f}: " + ", ".join(sorted(by[f])))
    if args.out:
        json.dump(rows, open(args.out, "w"), indent=1)
    return 0


if __name__ == "__main__":
    sys.exit(main())
