#!/usr/bin/env python3
"""Every exception class the Rust code names by string must be one the runtime can load.

What this answers: *will an error path throw, or panic*. `Jvm::exception` (jvm/src/jvm.rs) ends in

    let instance = self.new_class(r#type, "(Ljava/lang/String;)V", (message_str,)).await.unwrap();

Since 2026-09-19 (`rustjava-jvm-exception-throws-instead-of-unwrap`) that `.unwrap()` is gone: a name
the loader cannot resolve now returns the NoClassDefFoundError the loader raised, rather than aborting
the process. The check did not lose its job, it changed: an unresolvable name no longer kills the
runtime, it silently raises *the wrong exception* -- the caller asked for IOException and gets
NoClassDefFoundError, so the `catch` that was supposed to handle it does not match. That is quieter
than a crash and therefore worth locking, and it is invisible until something walks that path. It is also self-referential: jvm.rs:842 reports a missing class by
calling `exception("java/lang/NoClassDefFoundError", ...)`, so the error path's own class has to be
loadable or the report itself panics.

Measured after gate 2 corrected three defects in the first draft: 43 distinct `java/`-prefixed
names across 846 `exception(...)` call sites, and 268 distinct class names registered in the loader.
Nothing is missing -- the baseline is 0, which is what makes the lock cheap. The first draft read
41 / 812 / 263 and every one of those was an undercount: it scanned line by line (losing 34 calls
rustfmt had broken across a newline), matched only `as_proto()` (losing three `list_proto()`
registrations), and keyed types by bare name (letting one of a colliding pair answer for the
other). The one that was missing before, `java/lang/BootstrapMethodError`,
is the reason this exists: removing its registration is the round-trip test (see below).

HOW THE TWO SETS ARE BUILT
  named     every string literal in the first argument of an `exception(` call, in any *.rs in the
            workspace, whose value starts `java/` or `javax/`.
  loadable  `rustjava-runtime/src/loader.rs` lists `crate::classes::<path>::<Type>::as_proto()` and
            `::list_proto()`; the loader returns a proto only if `proto.name == name`, so the
            loadable set is exactly the `name:` literal each *registered* constructor writes. The
            resolution key is (module path, type, function) -- see `loadable_classes()` for why all
            three are needed.

WHAT THIS DOES NOT SEE -- it is a floor, not a proof:
  * A name built at run time (`format!`, a `const`, a variable, a match arm returning &str) is not a
    literal at the call site, so it is invisible here. Only the literal spelling is checked.
  * Only `exception(` is scanned. A class named through `new_class(` or `find_class(` directly is
    not covered; those paths return Result to their caller rather than unwrapping, which is why the
    panic axis is this one.
  * A registered proto whose class fails to *initialise* at run time still loads here. This checks
    resolvability of the name, not the health of the class.
  * Names are compared as written. A typo that happens to match another real class passes.

DONE SEPARATELY, as its own axis: `Jvm::exception` reporting instead of unwrapping
(`jvm/tests/test_exception_construction.rs`). This check is what keeps that report from being needed.

Exit: 0 every named class is loadable
      1 at least one named class has no registered proto
      2 could not measure (missing file, or a registered entry whose name cannot be resolved)

It fails closed: if a registered entry cannot be traced back to a `name:` literal the check reports
2 rather than silently shrinking the loadable set, which would turn into a false red.
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LOADER = ROOT / "rustjava-runtime" / "src" / "loader.rs"
CLASSES = ROOT / "rustjava-runtime" / "src" / "classes"

# First argument of `exception(`, when it is a java/ or javax/ string literal. `\s*` has to be able
# to cross a newline: rustfmt breaks the call when the line is long, and 34 of the 846 call sites in
# this tree are written that way. Matching is done against the whole file for that reason -- reading
# it line by line made the `\s*` unreachable and lost those 34 silently.
NAMED = re.compile(r'exception\(\s*"((?:java|javax)/[A-Za-z0-9_$/]+)"')
# `crate::classes::java::lang::BootstrapMethodError::as_proto(),` -- and `list_proto()`, which three
# entries use (AbstractListItr, ArrayListItr, VectorItr). Matching only `as_proto` made the check
# report those three classes as unregistered when they are registered.
REGISTERED = re.compile(r"crate::classes::([A-Za-z0-9_:]+)::((?:as|list)_proto)\(\)")
# `impl <Type> {` -- the start of a block, not the whole block: a type can hold more than one proto
# constructor and they name different classes.
IMPL_START = re.compile(r"^impl\s+([A-Za-z0-9_]+)\s*\{", re.M)
# `pub fn as_proto() -> RuntimeClassProto { … }` inside such a block.
PROTO_FN = re.compile(r"pub fn ([a-z_]+)\(\)\s*->\s*RuntimeClassProto\s*\{(.*?)\n    \}", re.S)
NAME_FIELD = re.compile(r'name:\s*"([^"]+)"')


# The independent witness for "did the parse come up short". `REGISTERED` is the pattern under
# suspicion, so counting its own matches proves nothing; this counts the same calls a second way,
# by the one token a registration cannot be written without. Occurrences rather than lines, and no
# trailing comma, so it does not care how the list is formatted -- measured on this file: `_proto()`
# 268, `_proto(),` lines 268, `crate::classes::` 268, registrations parsed 268, all agreeing.
PROTO_CALL = re.compile(r"_proto\(\)")

def die(message):
    print(f"cannot measure: {message}", file=sys.stderr)
    raise SystemExit(2)


def read(path):
    return path.read_text(encoding="utf-8", errors="replace")


# Build artefacts dwarf the source tree, and `rglob` from the root walks them even when the results
# are filtered out afterwards: pruning here took the check from ~33s to well under a second.
SKIP_DIRS = {"target", ".git"}


def rust_files():
    for entry in sorted(ROOT.iterdir()):
        if entry.name in SKIP_DIRS:
            continue
        if entry.is_dir():
            yield from sorted(entry.rglob("*.rs"))
        elif entry.suffix == ".rs":
            yield entry


def named_classes():
    """{class name: [file:line, ...]} for every literal exception(...) name.

    Matched against the whole file rather than line by line, because rustfmt breaks a long call
    after `exception(` and the pattern's `\\s*` has to cross that newline. Line numbers are
    recovered from the match offset so the report still points at a place.
    """
    found = {}
    for path in rust_files():
        text = read(path)
        for match in NAMED.finditer(text):
            line = text.count("\n", 0, match.start()) + 1
            found.setdefault(match.group(1), []).append(f"{path.relative_to(ROOT)}:{line}")
    return found


def loadable_classes():
    """Names the bootstrap loader can return, i.e. the registered protos.

    Keyed by (module path, type, function), not by type alone. Two of each are needed:

      * Bare type names collide. This tree holds two `Formatter`s (`java/util` and
        `java/util/logging`) and two `JarURLConnection`s (`java/net` and `org/rustjava/net`), each
        registered separately. Keying by type alone let one of a pair answer for the other, so
        deleting a registration left the check green -- the exact failure it exists to catch.
      * One type can hold more than one proto constructor. `AbstractListItr::as_proto()` names
        `java/util/AbstractList$Itr` while its `list_proto()` names `java/util/AbstractList$ListItr`;
        taking the first `name:` in the block attributed the wrong class to the registration and
        reported the other as absent.

    The module path comes from the file's own location under `classes/`, which is what the
    `crate::classes::…` path in the loader spells.
    """
    if not LOADER.is_file():
        die(f"{LOADER.relative_to(ROOT)} is missing")
    if not CLASSES.is_dir():
        die(f"{CLASSES.relative_to(ROOT)} is missing")

    name_of = {}
    for path in sorted(CLASSES.rglob("*.rs")):
        module = "::".join(path.relative_to(CLASSES).parts[:-1])
        text = read(path)
        starts = [(m.start(), m.group(1)) for m in IMPL_START.finditer(text)] + [(len(text), None)]
        for (start, type_name), (end, _) in zip(starts, starts[1:]):
            for function in PROTO_FN.finditer(text[start:end]):
                field = NAME_FIELD.search(function.group(2))
                if field:
                    name_of[(module, type_name, function.group(1))] = field.group(1)

    loader_text = read(LOADER)
    registered = REGISTERED.findall(loader_text)
    if not registered:
        die(f"no proto registrations found in {LOADER.relative_to(ROOT)}")

    # Did this parse come up short? The whole check rests on `registered` being every registration,
    # and the failure mode is silent: a pattern that matches fewer entries yields a smaller loadable
    # set, and a smaller loadable set makes missing classes look present. Measured on the first draft
    # of this file: it matched only `as_proto`, parsed 265 of 268 and resolved 263 names, and said
    # nothing. Counting the calls a second, independent way turns that into an exit 2.
    witness = len(PROTO_CALL.findall(loader_text))
    if len(registered) != witness:
        die(
            f"{len(registered)} registrations parsed but {witness} proto calls are in "
            f"{LOADER.relative_to(ROOT)}. The pattern that reads them is missing some, so the loadable "
            "set is short and every class it lost would read as 'not loadable'. Fix REGISTERED rather "
            "than trusting this run."
        )

    names, unresolved, name_of_entry = set(), [], {}
    for path, function in registered:
        parts = path.split("::")
        key = ("::".join(parts[:-1]), parts[-1], function)
        if key in name_of:
            names.add(name_of[key])
            name_of_entry.setdefault(name_of[key], []).append(f"{path}::{function}()")
        else:
            unresolved.append(f"{path}::{function}()")
    if unresolved:
        die("registered entries with no resolvable name: " + ", ".join(sorted(set(unresolved))))

    # Two registrations that resolve to one name mean the resolution is wrong, not that the runtime
    # has a duplicate: it is what the first draft did when it keyed types by bare name and let one of
    # a colliding pair answer for its twin (265 parsed, 263 names). Named rather than counted, so the
    # reader can tell a genuine duplicate registration from a mis-attribution at a glance.
    collapsed = {name: entries for name, entries in name_of_entry.items() if len(entries) > 1}
    if collapsed:
        detail = "; ".join(f"{name} <- {', '.join(sorted(entries))}" for name, entries in sorted(collapsed.items()))
        die(
            f"{len(registered)} registrations resolved to only {len(names)} names. Two registrations "
            f"naming one class means this file read them wrong, and a short loadable set reads as "
            f"'not loadable': {detail}"
        )
    return names


def main():
    named = named_classes()
    loadable = loadable_classes()
    missing = sorted(name for name in named if name not in loadable)

    if missing:
        print(f"{len(missing)} named exception class(es) the runtime cannot load:")
        for name in missing:
            sites = named[name]
            print(f"  ✗ {name} — named at {sites[0]}" + (f" and {len(sites) - 1} more" if len(sites) > 1 else ""))
        print()
        print("Jvm::exception returns whatever new_class() failed with, so each of these raises")
        print("NoClassDefFoundError instead of the exception the code asked for -- a catch on the")
        print("intended class will not match.")
        print("Add the class under rustjava-runtime/src/classes/ and register its as_proto() in")
        print("rustjava-runtime/src/loader.rs, or stop naming it.")
        return 1

    print(
        f"✓ {len(named)} named exception class(es) across "
        f"{sum(len(v) for v in named.values())} call site(s); all {len(loadable)} loadable"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
