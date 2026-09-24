#!/usr/bin/env python3
"""Refuse any change to the frozen ledgers `REPORT.md` and `STATE.md`.

Why: until 2026-09-25 every PR prepended to these two files. Each landing then conflicted with
every open sibling PR: 42 union merges out of 61 PRs between 2026-09-10 and 2026-09-25. Round
records now go to per-round files in `docs/worklog/` (AGENTS.md §Round Worklog), and the two files
are kept as history. A round that goes back to the old habit would bring the conflicts back, so
the files are pinned by hash here.

A merge conflict in either file means your side added an entry. Take `main`'s side and move the
entry into your round's `docs/worklog/*.md`.
"""

import hashlib
import sys
from pathlib import Path

FROZEN = {
    "REPORT.md": "dd2f16f4e0da590919fcfcdb71d4f9ad6105d1c58b8284472fdaff1b0dab9079",
    "STATE.md": "38a53e3c415a609b265334e69a59cfd4b31cc4ad4a284b567d9377c47f5b6e02",
}

root = Path(__file__).resolve().parent.parent
bad = []
for name, want in FROZEN.items():
    path = root / name
    got = hashlib.sha256(path.read_bytes()).hexdigest() if path.exists() else "missing"
    if got != want:
        bad.append(f"{name}: sha256 {got} != frozen {want}")

if bad:
    print("check-ledgers-frozen: FAIL — these files are frozen since 2026-09-25", file=sys.stderr)
    for line in bad:
        print(f"  {line}", file=sys.stderr)
    print("  write the round entry to docs/worklog/YYYY-MM-DD-<slug>.md instead (AGENTS.md §Round Worklog)", file=sys.stderr)
    sys.exit(1)
print(f"check-ledgers-frozen: ok — {len(FROZEN)} frozen ledgers unchanged")
