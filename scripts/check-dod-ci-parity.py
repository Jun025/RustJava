#!/usr/bin/env python3
"""Lock: the local DoD command block must reproduce what CI actually runs.

Why this lock exists — measured, not assumed. The `…-going-stale` lineage ran five
rounds (initial → fix3) and every single one claimed "fully synced"; every single one
left one more place where the *number of CI checks the local DoD reproduces* disagreed
with `.github/workflows/rust.yml`. Hand comparison failed five times in a row.

So this does NOT scan prose for stale numbers — that was tried and rejected: the sixth
miss was a *word* ("CI 검사 «한 줄»"), not a number, and no string scanner catches that.
Instead it parses the two things themselves and prints the symmetric difference. Prose
goes stale because the two sets drift; this locks the sets.

Two axes, mirroring the reopen condition in docs/upstream-sync-approach.md §4:
  axis A — the command set   (a `- run:` step in CI ↔ a line in the DoD block)
  axis B — the toolchain set (`strategy.matrix.rust` ↔ `cargo +<tc>` prefixes in the DoD)

The two axes are compared INDEPENDENTLY, not as a cross product. That is a deliberate
ceiling, stated so it cannot hide: CI runs all 4 cargo checks under both stable and beta,
while the DoD only doubles up `clippy` (`cargo +beta clippy`). Running `cargo +beta test`
locally means a full second-toolchain rebuild every round, and the 2026-09-04 decision
round weighed that and declined it — clippy is the only lint-bearing axis, and that is
where the measured gap was (9 `#[allow(clippy::double_must_use)]` deleted → stable 0,
beta rc=101 / 6 diagnostics). Widening to the cross product is a decision, not a bug fix.

★That decision was made on 2026-09-04 and the answer is NO — do not widen. Cost and benefit
were both measured; see `docs/upstream-sync-approach.md` §4 "[2026-09-04 판정] 파리티 락을
«교차곱»까지 넓히는가" for the numbers and, importantly, the reopen condition (a counting
command over `rust.yml` failure history, today's value 0). Do not re-argue it from taste.

Also out of scope by the same decision: the OS axis (macos/ubuntu/windows). It cannot be
reproduced locally at all, so CI is its only net. Conditional (`if:`-guarded) steps are
therefore excluded from axis A — but they are PRINTED, never silently dropped.
"""

import re
import sys
from pathlib import Path

# ★ The canonical locations live here, in code — not in a doc. Docs in this lineage went
#   stale five times; this file is what the CI job actually executes.
ROOT = Path(__file__).resolve().parents[1]
CI_FILE = ROOT / ".github" / "workflows" / "rust.yml"
DOD_FILE = ROOT / "CLAUDE.md"
DOD_SECTION = "## Definition of Done"

# Bare `cargo` resolves to the default toolchain. That equals CI's `stable` cell only
# because this repo pins nothing; a rust-toolchain file would silently break the mapping.
TOOLCHAIN_PIN_FILES = ("rust-toolchain.toml", "rust-toolchain")
DEFAULT_TOOLCHAIN = "stable"


def norm(cmd):
    return " ".join(cmd.split())


def split_toolchain(cmd):
    """`cargo +beta clippy …` -> ("beta", "cargo clippy …"); anything else -> (None, cmd)."""
    m = re.match(r"^cargo\s+\+(\S+)\s+(.*)$", cmd)
    if m:
        return m.group(1), norm("cargo " + m.group(2))
    if cmd.startswith("cargo "):
        return DEFAULT_TOOLCHAIN, cmd
    return None, cmd


def parse_ci(text):
    """-> (checks, conditional, toolchains) reading `- run:` steps out of the workflow.

    Hand-rolled rather than PyYAML: the sibling lock is stdlib-only and CI installs
    nothing for it. The file is small and its shape is asserted below.
    """
    lines = text.splitlines()
    steps, cur, step_indent = [], None, None
    toolchains = set()

    for raw in lines:
        m = re.match(r"^(\s*)rust:\s*\[(.*)\]\s*$", raw)
        if m:
            toolchains |= {t.strip() for t in m.group(2).split(",") if t.strip()}
            continue
        m = re.match(r"^(\s*)-\s+(.*)$", raw)
        if m and (step_indent is None or len(m.group(1)) == step_indent):
            # a list item at steps depth starts a new step — but only inside `steps:`
            if re.match(r"^(name|uses|run|if):", m.group(2)) or (cur is not None):
                if re.match(r"^(name|uses|run|if):", m.group(2)):
                    step_indent = len(m.group(1))
                    cur = [m.group(2)]
                    steps.append(cur)
                    continue
        if cur is not None:
            stripped = raw.strip()
            if raw.strip() == "":
                continue
            indent = len(raw) - len(raw.lstrip())
            if step_indent is not None and indent <= step_indent and not raw.lstrip().startswith("-"):
                # dedented out of the step list (next job/key)
                cur, step_indent = None, None
                continue
            cur.append(stripped)

    checks, conditional = [], []
    for step in steps:
        body = "\n".join(step)
        if not re.search(r"^run:", body, re.M):
            continue
        run = re.search(r"^run:\s*(.*)$", body, re.M).group(1)
        if run.startswith("|") or run.startswith(">"):
            # block scalar: a multi-line shell snippet. Keep it identifiable but do not
            # pretend it is a single command — every one of these so far is setup.
            tail = [s for s in step if not re.match(r"^(name|uses|run|if|with):", s)]
            run = "<셸 블록> " + (tail[0] if tail else "")
        cond = re.search(r"^if:\s*(.*)$", body, re.M)
        if cond:
            conditional.append((norm(run), norm(cond.group(1))))
        else:
            checks.append(norm(run))
    return checks, conditional, toolchains


def parse_dod(text):
    """-> command lines of the first fenced block inside the DoD section."""
    start = text.index(DOD_SECTION)
    block = re.search(r"```[a-z]*\n(.*?)\n\s*```", text[start:], re.S)
    if not block:
        sys.exit("FAIL: CLAUDE.md §Definition of Done 에 코드블록이 없다 — 정본이 사라졌다")
    return [norm(l) for l in block.group(1).splitlines() if l.strip()]


def main():
    ci_cmds, ci_cond, ci_tcs = parse_ci(CI_FILE.read_text())
    dod_lines = parse_dod(DOD_FILE.read_text())

    dod_cmds, dod_tcs = set(), set()
    for line in dod_lines:
        tc, cmd = split_toolchain(line)
        dod_cmds.add(cmd)
        if tc:
            dod_tcs.add(tc)

    ci_set = set(ci_cmds)
    problems = []

    print("DOD-CI-PARITY  로컬 DoD ↔ .github/workflows/rust.yml")
    print(f"  정본: {DOD_FILE.name} §Definition of Done 의 첫 코드블록  ↔  {CI_FILE.relative_to(ROOT)}")

    print(f"\n  [축 A · 명령]  CI {len(ci_set)}개 · DoD {len(dod_cmds)}개")
    for c in sorted(ci_set | dod_cmds):
        mark = "  " if (c in ci_set and c in dod_cmds) else "★!"
        print(f"    {mark} {c}   (CI={'y' if c in ci_set else 'n'} DoD={'y' if c in dod_cmds else 'n'})")
    only_ci, only_dod = ci_set - dod_cmds, dod_cmds - ci_set
    if only_ci or only_dod:
        problems.append("축 A")
        for c in sorted(only_ci):
            print(f"    ★ CI 에만 있다 — DoD 에 이 줄을 넣어라: {c}")
        for c in sorted(only_dod):
            print(f"    ★ DoD 에만 있다 — CI 가 안 치는 것을 DoD 가 시킨다: {c}")

    print(f"\n  [축 B · toolchain]  CI {sorted(ci_tcs)} · DoD {sorted(dod_tcs)}")
    only_ci_tc, only_dod_tc = ci_tcs - dod_tcs, dod_tcs - ci_tcs
    if only_ci_tc or only_dod_tc:
        problems.append("축 B")
        for t in sorted(only_ci_tc):
            print(f"    ★ CI 에만 있다 — DoD 에 `cargo +{t} …` 줄이 없다: {t}")
        for t in sorted(only_dod_tc):
            print(f"    ★ DoD 에만 있다 — CI 가 안 도는 toolchain 이다: {t}")

    # Never silent: conditional steps are excluded from axis A, so they get printed.
    print(f"\n  [제외] 조건부 step {len(ci_cond)}건 — OS 축은 로컬 재현 불가(설계상 CI 가 유일한 그물)")
    for run, cond in ci_cond:
        print(f"    - if: {cond}   run: {run[:70]}")

    pinned = [f for f in TOOLCHAIN_PIN_FILES if (ROOT / f).exists()]
    if pinned:
        problems.append("toolchain 고정")
        print(f"\n  ★ {pinned} 이 생겼다 — 맨 `cargo` 가 더는 «{DEFAULT_TOOLCHAIN}» 이 아니다.")
        print("    이 검사기의 축 B 매핑이 깨졌으니 split_toolchain() 을 함께 고쳐라.")

    if problems:
        print(f"\nFAIL 대칭차 있음: {' · '.join(problems)}")
        return 1
    # ★ Not silent on success either — the sibling lineage made "조용한 통과" a violation.
    print(f"\nOK 두 축 모두 대칭차 0 — 명령 {len(ci_set)}개 · toolchain {len(ci_tcs)}개로 «둘 다 일치»")
    return 0


if __name__ == "__main__":
    sys.exit(main())
