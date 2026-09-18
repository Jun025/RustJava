#!/usr/bin/env python3
"""Every exception class the Rust code names by string must be one the runtime can load.

What this answers: *will an error path throw, or panic*. `Jvm::exception` (jvm/src/jvm.rs) ends in

    let instance = self.new_class(r#type, "(Ljava/lang/String;)V", (message_str,)).await.unwrap();

so a name the bootstrap loader cannot resolve does not become a Java exception -- it unwraps an
`Err` and aborts the process. That is the one failure mode a JVM must not have, and it is invisible
until something walks that path. It is also self-referential: jvm.rs:842 reports a missing class by
calling `exception("java/lang/NoClassDefFoundError", ...)`, so the error path's own class has to be
loadable or the report itself panics.

Measured when this was written: 41 distinct `java/`-prefixed names across 812 `exception(...)` call
sites, and 263 distinct class names registered in the loader. Nothing was missing -- the baseline is
0, which is what makes the lock cheap. The one that was missing before, `java/lang/BootstrapMethodError`,
is the reason this exists: removing its registration is the round-trip test (see below).

HOW THE TWO SETS ARE BUILT
  named     every string literal in the first argument of an `exception(` call, in any *.rs in the
            workspace, whose value starts `java/` or `javax/`.
  loadable  `rustjava-runtime/src/loader.rs` lists `crate::classes::<path>::<Type>::as_proto()`; the
            loader returns a proto only if `proto.name == name`, so the loadable set is exactly the
            `name:` literal of each *registered* type. Each type is resolved back to its `name:` by
            reading its `impl <Type>` block under `rustjava-runtime/src/classes/`.

WHAT THIS DOES NOT SEE -- it is a floor, not a proof:
  * A name built at run time (`format!`, a `const`, a variable, a match arm returning &str) is not a
    literal at the call site, so it is invisible here. Only the literal spelling is checked.
  * Only `exception(` is scanned. A class named through `new_class(` or `find_class(` directly is
    not covered; those paths return Result to their caller rather than unwrapping, which is why the
    panic axis is this one.
  * A registered proto whose class fails to *initialise* at run time still loads here. This checks
    resolvability of the name, not the health of the class.
  * Names are compared as written. A typo that happens to match another real class passes.

DELIBERATELY NOT DONE HERE: turning the `.unwrap()` into a thrown exception. That is a separate
axis -- this one makes sure there is nothing left to unwrap on.

Exit: 0 every named class is loadable
      1 at least one named class has no registered proto
      2 could not measure (missing file, or a registered entry whose name cannot be resolved)

It fails closed: if a registered `as_proto()` entry cannot be traced back to a `name:` literal the
check reports 2 rather than silently shrinking the loadable set, which would turn into a false red.
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LOADER = ROOT / "rustjava-runtime" / "src" / "loader.rs"
CLASSES = ROOT / "rustjava-runtime" / "src" / "classes"

# First argument of `exception(`, when it is a java/ or javax/ string literal.
NAMED = re.compile(r'exception\(\s*"((?:java|javax)/[A-Za-z0-9_$/]+)"')
# `crate::classes::java::lang::BootstrapMethodError::as_proto(),`
REGISTERED = re.compile(r"crate::classes::([A-Za-z0-9_:]+)::as_proto\(\)")
# The `name:` field inside an `impl <Type> { ... }` block.
IMPL_BLOCK = re.compile(r"impl\s+([A-Za-z0-9_]+)\s*\{(.*?)\n\}", re.S)
NAME_FIELD = re.compile(r'name:\s*"([^"]+)"')


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
    """{class name: [file:line, ...]} for every literal exception(...) name."""
    found = {}
    for path in rust_files():
        for number, line in enumerate(read(path).splitlines(), 1):
            for match in NAMED.finditer(line):
                found.setdefault(match.group(1), []).append(f"{path.relative_to(ROOT)}:{number}")
    return found


def loadable_classes():
    """Names the bootstrap loader can return, i.e. the registered protos."""
    if not LOADER.is_file():
        die(f"{LOADER.relative_to(ROOT)} is missing")
    if not CLASSES.is_dir():
        die(f"{CLASSES.relative_to(ROOT)} is missing")

    name_of = {}
    for path in sorted(CLASSES.rglob("*.rs")):
        text = read(path)
        for match in IMPL_BLOCK.finditer(text):
            field = NAME_FIELD.search(match.group(2))
            if field:
                name_of.setdefault(match.group(1), field.group(1))

    registered = REGISTERED.findall(read(LOADER))
    if not registered:
        die(f"no ::as_proto() entries found in {LOADER.relative_to(ROOT)}")

    names, unresolved = set(), []
    for path in registered:
        type_name = path.split("::")[-1]
        if type_name in name_of:
            names.add(name_of[type_name])
        else:
            unresolved.append(path)
    if unresolved:
        die("registered entries with no resolvable name: " + ", ".join(sorted(set(unresolved))))
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
        print("Jvm::exception unwraps new_class(), so each of these panics instead of throwing.")
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
