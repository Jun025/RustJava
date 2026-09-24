#!/usr/bin/env python3
"""Lock for the `docs/worklog/` machine-readable `.json` convention (AGENTS.md).

Why this lock exists: a malformed `.json` silently produces zero cards in the cockpit
"후속 작업 추천" panel — no error, no warning, just nothing. This turns that silence red.
It checks only the keys the consumer (`/api/proposals`, `scanRepoSimple`) actually reads;
every other key is free.

It never demands a `.json` for an existing `.md` — the convention is not retroactive.

qts locks the same six axes in `tests/guardrail/test_worklog_json_schema.py`; this repo has no
pytest (and `cargo test` would need a JSON dependency to read docs), so it is a plain script.
"""

import json
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WORKLOG = ROOT / "docs" / "worklog"
AGENTS = ROOT / "AGENTS.md"

# keys scanRepoSimple reads out of a proposals[] element — all strings
PROPOSAL_KEYS = ("title", "plainSummary", "userBenefit", "why", "tradeoff", "effort", "target")
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}")

KINDS = ("product", "meta")
MAX_PROPOSALS = 2  # AGENTS.md "Proposal threshold"

# The cap binds only new or changed files: a past worklog identical to the base passes (no retroactive edits).
# CI sets WORKLOG_BASE=HEAD^1 (the PR merge commit's base side); locally the default is the fork point from origin/main.
BASE = os.environ.get("WORKLOG_BASE") or subprocess.run(
    ["git", "merge-base", "HEAD", "origin/main"], cwd=ROOT, capture_output=True, text=True
).stdout.strip()


def at_base(p):
    if not BASE:
        return None
    r = subprocess.run(["git", "show", f"{BASE}:{p.relative_to(ROOT).as_posix()}"], cwd=ROOT, capture_output=True)
    return r.stdout if r.returncode == 0 else None


fail = []


def check(cond, msg):
    if not cond:
        fail.append(msg)


for p in sorted(WORKLOG.glob("*.json")):
    try:
        obj = json.loads(p.read_text(encoding="utf-8"))
    except json.JSONDecodeError as e:
        fail.append(f"{p.name}: unparseable — the whole file is ignored by the consumer ({e})")
        continue
    if not isinstance(obj, dict):
        fail.append(f"{p.name}: top level is not an object")
        continue

    check(p.with_suffix(".md").exists(), f"{p.name}: no `.md` sibling — the `.json` is an addition, not a replacement")
    check(DATE_RE.match(p.stem), f"{p.name}: filename does not start with YYYY-MM-DD")
    check(obj.get("date") == p.stem[:10], f"{p.name}: date({obj.get('date')}) != filename date({p.stem[:10]}) — sorting would lie")

    proposals = obj.get("proposals", [])
    if not isinstance(proposals, list):
        fail.append(f"{p.name}: proposals is not an array")
        proposals = []
    for i, prop in enumerate(proposals):
        if not isinstance(prop, dict):
            fail.append(f"{p.name}#p{i}: element is not an object (consumer skips it)")
            continue
        for k in PROPOSAL_KEYS:
            v = prop.get(k)
            check(isinstance(v, str) and v.strip(), f"{p.name}#p{i}: '{k}' missing — that field renders empty on the card")
        check("kind" not in prop or prop["kind"] in KINDS, f"{p.name}#p{i}: kind {prop.get('kind')!r} — must be one of {KINDS} or absent")
    if len(proposals) > MAX_PROPOSALS and at_base(p) != p.read_bytes():
        fail.append(f"{p.name}: {len(proposals)} proposals in a new/changed worklog — the cap is {MAX_PROPOSALS} (AGENTS.md \"Proposal threshold\")")

    for key in ("adoptedProposals", "declinedProposals"):
        refs = obj.get(key, [])
        if not isinstance(refs, list):
            fail.append(f"{p.name}: {key} is not an array")
            continue
        for r in refs:
            check(isinstance(r, str) and "#p" in r, f"{p.name}: {key} element {r!r} — must be `<basename>#p<index>` for the disposition to apply")

agents = AGENTS.read_text(encoding="utf-8")
check("## Round Worklog" in agents, "AGENTS.md: the Round Worklog convention section is gone")
for k in ("adoptedProposals", "declinedProposals", "proposals[]"):
    check(k in agents, f"AGENTS.md: the consumed key '{k}' is no longer documented")

for m in fail:
    print(f"WORKLOG-JSON: {m}", file=sys.stderr)
if not BASE:
    print("WORKLOG-JSON: no git base resolved — every file counts as changed for the proposal cap", file=sys.stderr)
print(f"checked {len(list(WORKLOG.glob('*.json')))} worklog .json file(s) against base {BASE[:12] or '-'}, {len(fail)} problem(s)")
sys.exit(1 if fail else 0)
