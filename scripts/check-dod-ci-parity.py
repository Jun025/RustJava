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
reproduced locally at all, so CI is its only net.

Axis A membership is decided by WHICH TOOL a step invokes — not by whether it carries an
`if:`. That was changed on 2026-09-08 and the old rule ("`if:`-guarded steps are excluded")
is gone, because `if:` says nothing about what a step *does*, which made it an escape
hatch. Measured on `4959d0f` before changing anything:

  - put `cargo test --all` under an `if:` and delete its DoD line  -> rc=0, GREEN.
    The real gate left the compared set and nothing complained. That is the hole.
  - put `cargo test --all` under an `if:` and keep the DoD line    -> rc=1, RED.
    A false positive: a check CI runs on some OS cells is still correct to run locally.

The tool-name rule turns both around: the first case is red (CI has a cargo step the DoD
does not), the second is green. Setup steps (`git config …`) are excluded by their tool
name, not by their `if:` — and they are still PRINTED, never silently dropped.

Classifying by tool name left one hole, closed on 2026-09-08: a tool nobody had listed
(`npm`, `make`) simply got printed and the lock stayed green — so a whole new CI check
could arrive with the DoD block never learning about it. Now a step whose tool is in
NEITHER CHECK_TOOLS nor SETUP_TOOLS is a failure. Registering is one line; see the
SETUP_TOOLS comment for which tuple and why. That escape hatch is the point, not a
weakness: without it the rule would also red on the legitimate `git config` setup step,
which was measured to be the only out-of-list tool on the day the rule was written.
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

# ★ The axis-A classifier. A step belongs to axis A iff the tool it invokes is in here.
#   `cargo` is the tool the adopted proposal named; `python3` is here because the two
#   doc locks (`check-worklog-json.py`, this file) are also in the DoD block — dropping
#   them would lose 2 of today's 6 compared commands, and a replacement that loses
#   coverage is not a fix. Deliberately hardcoded: deriving the set from the DoD block
#   would rebuild the same escape hatch one level up (delete every `python3` line and
#   `python3` leaves the axis, taking the CI steps with it).
CHECK_TOOLS = ("cargo", "python3")

# ★ Tools that are legitimately NOT checks — setup a local DoD has no business reproducing.
#   This tuple exists because "not a check tool" was measured to be TWO different things:
#   on 2026-09-08 `rust.yml` had exactly one step outside CHECK_TOOLS and it was
#   `git config --global core.autocrlf false` (windows line-ending setup). So a blanket
#   "outside the list => fail" would have gone red on a correct workflow, on day one.
#
#   ★A step whose tool is in NEITHER tuple is a failure: it is a check nobody taught the
#   DoD about, and until 2026-09-08 it merely got printed while the lock stayed green.
#   ★Registering is one line — that is the whole escape hatch, and it is deliberate:
#     · a real check the DoD must also run  -> add it to CHECK_TOOLS (and to the DoD block)
#     · setup that stays out of the compare -> add it here, with why
SETUP_TOOLS = ("git",)


def norm(cmd):
    return " ".join(cmd.split())


def tool_of(cmd):
    """The tool a command line invokes = its first shell word."""
    words = cmd.split()
    return words[0] if words else ""


def split_toolchain(cmd):
    """`cargo +beta clippy …` -> ("beta", "cargo clippy …"); anything else -> (None, cmd)."""
    m = re.match(r"^cargo\s+\+(\S+)\s+(.*)$", cmd)
    if m:
        return m.group(1), norm("cargo " + m.group(2))
    if cmd.startswith("cargo "):
        return DEFAULT_TOOLCHAIN, cmd
    return None, cmd


def parse_ci(text):
    """-> (runs, toolchains); runs = [(tool, normalized command, `if:` condition or "")].

    Classification is the caller's job — this only reports what each `- run:` step is.

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

    runs = []
    for step in steps:
        body = "\n".join(step)
        if not re.search(r"^run:", body, re.M):
            continue
        run = re.search(r"^run:\s*(.*)$", body, re.M).group(1)
        if run.startswith("|") or run.startswith(">"):
            # block scalar: a multi-line shell snippet. Its tool is the first line's tool,
            # but the display keeps it identifiable rather than pretending it is one
            # command — so a block scalar that ever does invoke a check tool goes red and
            # asks for a human, instead of silently half-matching a DoD line.
            tail = [s for s in step if not re.match(r"^(name|uses|run|if|with):", s)]
            first = tail[0] if tail else ""
            tool, run = tool_of(first), "<셸 블록> " + first
        else:
            tool = tool_of(run)
        cond = re.search(r"^if:\s*(.*)$", body, re.M)
        runs.append((tool, norm(run), norm(cond.group(1)) if cond else ""))
    return runs, toolchains


def parse_dod(text):
    """-> command lines of the first fenced block inside the DoD section."""
    start = text.index(DOD_SECTION)
    block = re.search(r"```[a-z]*\n(.*?)\n\s*```", text[start:], re.S)
    if not block:
        sys.exit("FAIL: CLAUDE.md §Definition of Done 에 코드블록이 없다 — 정본이 사라졌다")
    return [norm(l) for l in block.group(1).splitlines() if l.strip()]


def main():
    ci_runs, ci_tcs = parse_ci(CI_FILE.read_text())
    dod_lines = parse_dod(DOD_FILE.read_text())

    ci_checks = [r for r in ci_runs if r[0] in CHECK_TOOLS]
    ci_setup = [r for r in ci_runs if r[0] not in CHECK_TOOLS]

    dod_cmds, dod_tcs, dod_setup = set(), set(), []
    for line in dod_lines:
        tc, cmd = split_toolchain(line)
        if tool_of(cmd) not in CHECK_TOOLS:
            dod_setup.append(cmd)
            continue
        dod_cmds.add(cmd)
        if tc:
            dod_tcs.add(tc)

    ci_set = {cmd for _, cmd, _ in ci_checks}
    problems = []

    print("DOD-CI-PARITY  로컬 DoD ↔ .github/workflows/rust.yml")
    print(f"  정본: {DOD_FILE.name} §Definition of Done 의 첫 코드블록  ↔  {CI_FILE.relative_to(ROOT)}")
    print(f"  분류축: «어느 도구를 부르는가» = {list(CHECK_TOOLS)} — ★`if:` 는 보지 않는다")
    print(f"  셋업 등록분: {list(SETUP_TOOLS)} — ★이 둘 «밖»의 도구를 부르는 step 은 FAIL 이다(등록은 한 줄)")

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

    # Never silent — three things the axis does not compare get printed anyway.
    on_cond = [(cmd, cond) for _, cmd, cond in ci_checks if cond]
    print(f"\n  [주의] 축 A 에 있으나 조건부인 step {len(on_cond)}건 — 일부 OS 셀에서만 돈다(로컬은 항상 친다)")
    for cmd, cond in on_cond:
        print(f"    - if: {cond}   run: {cmd[:70]}")

    unknown = [r for r in ci_setup if r[0] not in SETUP_TOOLS]
    unknown_dod = [c for c in dod_setup if tool_of(c) not in SETUP_TOOLS]
    if unknown or unknown_dod:
        problems.append("미등록 도구")
        for tool, cmd, _ in unknown:
            print(f"\n    ★ CI 가 «모르는 도구»를 부른다 — 도구={tool or '?'}: {cmd[:70]}")
        for cmd in unknown_dod:
            print(f"\n    ★ DoD 가 «모르는 도구»를 부른다 — 도구={tool_of(cmd) or '?'}: {cmd[:70]}")
        print("      ⇒ 한 줄로 등록하라: 진짜 검사면 CHECK_TOOLS 에(그리고 DoD 블록에도) ·")
        print(f"         셋업이면 SETUP_TOOLS 에 «왜»와 함께. 현재 등록분: CHECK={list(CHECK_TOOLS)} SETUP={list(SETUP_TOOLS)}")

    print(f"\n  [제외] 검사 도구를 안 부르는 run: step {len(ci_setup)}건 — 셋업(도구 이름으로 갈랐다)")
    for tool, cmd, cond in ci_setup:
        pre = f"if: {cond}   " if cond else ""
        print(f"    - 도구={tool or '?'}   {pre}run: {cmd[:70]}")
    if dod_setup:
        print(f"  [제외] DoD 쪽 {len(dod_setup)}줄 — 같은 술어로 뺐다: {' · '.join(dod_setup)}")

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
