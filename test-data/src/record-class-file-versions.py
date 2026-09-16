#!/usr/bin/env python3
"""Rewrite `test-data/class-file-versions.txt` from what is on disk.

Run this *after* deliberately recompiling a fixture, so the recorded version moves in the same
commit as the bytes it describes. Running it to silence a failing pin without looking at why the
version moved defeats the point — the check exists because a changed version is easy to miss
inside a binary diff.

Usage:  python3 test-data/src/record-class-file-versions.py
"""

import struct
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TABLE = ROOT / "class-file-versions.txt"


def main():
    old = TABLE.read_text().splitlines(keepends=True) if TABLE.exists() else []
    header = [line for line in old if line.startswith("#")]
    if not header:
        print(f"{TABLE} has no header comment to preserve; refusing to write", file=sys.stderr)
        return 1

    rows = []
    for path in sorted(ROOT.rglob("*.class")):
        data = path.read_bytes()
        minor, major = struct.unpack(">H", data[4:6])[0], struct.unpack(">H", data[6:8])[0]
        rows.append(f"{major}.{minor} {path.relative_to(ROOT)}\n")

    TABLE.write_text("".join(header) + "".join(rows))
    print(f"recorded {len(rows)} fixtures in {TABLE}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
