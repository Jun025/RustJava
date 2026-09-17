#!/bin/sh
# Rebuild the javac-compiled fixtures and check they come out byte-for-byte identical.
#
# What this answers: *which compiler* built a fixture. The class file version pin
# (tests/test_fixture_pins.rs, test-data/class-file-versions.txt) proves what a fixture targets —
# javac 21 and javac 26 both stamp 65.0 at --release 21 — but the bytes differ if the compiler or
# its flags differ, so reproducing them is the check the version alone cannot be.
#
# javac is deterministic for the same source, flags and compiler, which is what makes this work:
# measured 2026-09-17, all six test-data/indy class files reproduce exactly under
# `javac 26.0.1 --release 21`, and five sampled root fixtures reproduce under the --release their
# recorded major version implies (65->21, 66->22, 70->26).
#
# It is *not* wired into CI, and that is deliberate rather than an oversight: there is no JDK in
# .github/workflows/rust.yml and none on PATH here, so a test that needed one would either fail
# everywhere or skip everywhere. This is the manual check you run when you regenerate a fixture.
#
# Usage:  sh test-data/src/verify-javac-fixtures.sh [directory]   # default: test-data/indy
#         JAVAC=/path/to/javac sh test-data/src/verify-javac-fixtures.sh
#
# The --release to rebuild with is read from the fixture itself: a class file's major version is
# its target (major - 44), so nothing external has to be consulted or kept in sync.
#
# Exit: 0 all reproduced · 1 something differed or could not be rebuilt · 2 no javac (nothing was
#       checked — not a pass)

set -eu

DIR="${1:-test-data/indy}"

# `command -v javac` finds macOS's stub, which is executable and then reports there is no JDK — so
# candidates are probed by running them, not by testing the path.
if [ -n "${JAVAC:-}" ]; then
    candidates="$JAVAC"                             # explicit wins outright: falling back would
else                                                # hide which compiler actually did the check
    candidates="javac /opt/homebrew/opt/openjdk/bin/javac"
fi
found=""
for candidate in $candidates; do
    if "$candidate" -version >/dev/null 2>&1; then found="$candidate"; break; fi
done
if [ -z "$found" ]; then
    echo "no working javac in: $candidates (set JAVAC=...); nothing was verified" >&2
    exit 2
fi
JAVAC="$found"
echo "javac: $("$JAVAC" -version 2>&1)"

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
checked=0
differed=0
unbuildable=0

for class in "$DIR"/*.class; do
    base=$(basename "$class" .class)
    outer=${base%%\$*}                              # Foo$Bar.class comes from Foo.java
    sub=${DIR#test-data}                            # "test-data" -> "", "test-data/indy" -> "/indy"
    source="test-data/src${sub}/$outer.java"
    [ -f "$source" ] || continue                    # generator output has no .java; not this check's job

    # The fixture's own major version is the target it was built for (JVMS 4.1: 65 = Java 21).
    major=$(od -An -tu1 -j6 -N2 "$class" | tr -s ' \n' ' ' | awk '{ print $1 * 256 + $2 }')
    release=$((major - 44))

    # Deliberately no -sourcepath: test-data/src holds Exception.java, Array.java, Method.java and
    # friends, so putting it on the source path makes javac resolve `Exception` to the fixture
    # rather than java.lang.Exception. Measured — it turns clean rebuilds into type errors. The
    # cost is that a source needing a sibling cannot be rebuilt alone, which is reported as
    # "could not be rebuilt" rather than as drift.
    rm -rf "$work/out" && mkdir -p "$work/out"
    if ! "$JAVAC" --release "$release" -d "$work/out" "$source" 2>"$work/err"; then
        echo "  ? $base: could not rebuild at --release $release: $(grep -m1 error "$work/err" || head -1 "$work/err")" >&2
        unbuildable=$((unbuildable + 1))
        continue
    fi

    checked=$((checked + 1))
    if cmp -s "$work/out/$base.class" "$class"; then
        echo "  ✓ $base (--release $release)"
    else
        echo "  ✗ $base (--release $release): rebuilt bytes differ from the committed fixture" >&2
        differed=$((differed + 1))
    fi
done

if [ "$checked" -eq 0 ]; then
    echo "no javac-compiled fixtures found in $DIR; nothing was verified" >&2
    exit 2
fi
# "could not rebuild" is reported apart from "rebuilt and differs": one says the fixture drifted,
# the other says this script could not ask the question. Conflating them overstates the finding.
echo "$checked rebuilt: $((checked - differed)) reproduced, $differed differed; $unbuildable could not be rebuilt"
[ "$differed" -eq 0 ] && [ "$unbuildable" -eq 0 ]
