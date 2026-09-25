#!/usr/bin/env python3
"""Count which constant pool tags the `ldc` family actually names, over jars/directories of class files.

Answers one question: does anything in this corpus execute `ldc`/`ldc_w`/`ldc2_w` against a
CONSTANT_MethodHandle (15), MethodType (16) or Dynamic (17)? Those three are what
`classfile/src/opcode.rs` accepts and `jvm-bytecode` reports as unsupported rather than corrupt,
and the round that added them could only prove javac never emits the shape — third-party
generators were unmeasured (docs/worklog/2026-09-16-ldc-tags-15-16-17.md).

Usage:  python3 scripts/survey-ldc-constant-tags.py <jar-or-dir> [...]

Prints, per input: classes, ldc sites, the tag histogram, and the count of operands no `ldc` can
legally take. That last number is the scanner's own error bar — an instruction walk that loses
alignment lands on nonsense, so a non-zero share there is a reason to distrust the rest. It is
*detectable* misdecodes only: a misdecode landing on a plausible tag stays invisible.

Positive control, so a 0 is not "the scanner cannot see it":

    python3 scripts/survey-ldc-constant-tags.py test-data/ldc
    -> tags 15/16/17 as an ldc operand: {MethodHandle 1, MethodType 2, Dynamic 7}

Targeted Kotlin/Scala shapes were measured once (tags 15/16/17 as an ldc operand: 0) and their
sources dropped; commands and numbers are in docs/worklog/2026-09-17-ldc-targeted-shapes-kotlin-scala.md.
CI runs none of this — like `test-data/src/verify-javac-fixtures.sh`, it is a check a person runs.
"""

import sys
import zipfile
from collections import Counter
from pathlib import Path

TAG_NAMES = {
    1: "Utf8", 3: "Integer", 4: "Float", 5: "Long", 6: "Double", 7: "Class", 8: "String",
    9: "Fieldref", 10: "Methodref", 11: "InterfaceMethodref", 12: "NameAndType",
    15: "MethodHandle", 16: "MethodType", 17: "Dynamic", 18: "InvokeDynamic", 19: "Module", 20: "Package",
}
# JVMS 6.5 ldc/ldc_w/ldc2_w: int, float, long, double, String, Class, MethodType, MethodHandle, Dynamic.
LDC_LEGAL = {3, 4, 5, 6, 7, 8, 15, 16, 17}
TARGET = (15, 16, 17)

# Fixed operand widths, opcode -> bytes following the opcode. Absent = variable, handled below.
WIDTHS = {}
for op in range(0x00, 0x0F + 1): WIDTHS[op] = 0          # nop..dconst_1
for op in list(range(0x1A, 0x35 + 1)) + list(range(0x3B, 0x83 + 1)) + list(range(0x85, 0x98 + 1)):
    WIDTHS[op] = 0                                        # *load_<n>, *store_<n>, stack/math/convert/compare
for op in (0x10, 0x12, 0x15, 0x16, 0x17, 0x18, 0x19, 0x36, 0x37, 0x38, 0x39, 0x3A, 0xA9, 0xBC):
    WIDTHS[op] = 1                                        # bipush, ldc, *load, *store, ret, newarray
for op in (0x11, 0x13, 0x14, 0x84, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xBB, 0xBD, 0xC0, 0xC1):
    WIDTHS[op] = 2                                        # sipush, ldc_w, ldc2_w, iinc, field/method, new, checkcast
for op in range(0x99, 0xA8 + 1): WIDTHS[op] = 2           # if*, goto, jsr
for op in (0xAC, 0xAD, 0xAE, 0xAF, 0xB0, 0xB1, 0xBE, 0xBF, 0xC2, 0xC3): WIDTHS[op] = 0  # returns, arraylength, throw, monitor*
for op in (0xB9, 0xBA): WIDTHS[op] = 4                    # invokeinterface, invokedynamic
WIDTHS[0xC5] = 3                                          # multianewarray
for op in (0xC6, 0xC7): WIDTHS[op] = 2                    # ifnull, ifnonnull
for op in (0xC8, 0xC9): WIDTHS[op] = 4                    # goto_w, jsr_w


def parse_constant_pool(b, off):
    """Returns (tags_by_index, utf8_by_index, offset_after_pool). Long/Double take two slots (JVMS 4.4.5)."""
    count = int.from_bytes(b[off:off + 2], "big")
    off += 2
    tags, utf8s, i = {}, {}, 1
    while i < count:
        tag = b[off]
        tags[i] = tag
        off += 1
        if tag == 1:
            length = int.from_bytes(b[off:off + 2], "big")
            utf8s[i] = b[off + 2:off + 2 + length].decode("utf-8", "replace")
            off += 2 + length
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
            off += 4
        elif tag in (5, 6):
            off += 8
            i += 1
        elif tag in (7, 8, 16, 19, 20):
            off += 2
        elif tag == 15:
            off += 3
        else:
            raise ValueError(f"unknown constant pool tag {tag}")
        i += 1
    return tags, utf8s, off


def skip_attributes(b, off):
    n = int.from_bytes(b[off:off + 2], "big")
    off += 2
    out = []
    for _ in range(n):
        name_index = int.from_bytes(b[off:off + 2], "big")
        length = int.from_bytes(b[off + 2:off + 6], "big")
        out.append((name_index, off + 6, length))
        off += 6 + length
    return out, off


def walk_code(code, tags, hist, impossible):
    """Walk one Code attribute's bytecode, resolving every ldc-family operand to a pool tag."""
    pc, n = 0, len(code)
    while pc < n:
        op = code[pc]
        if op in (0x12, 0x13, 0x14):                       # ldc, ldc_w, ldc2_w
            width = 1 if op == 0x12 else 2
            index = int.from_bytes(code[pc + 1:pc + 1 + width], "big")
            tag = tags.get(index)
            hist[tag] += 1
            if tag not in LDC_LEGAL:
                impossible.append((index, tag))
            pc += 1 + width
        elif op == 0xC4:                                   # wide
            pc += 6 if code[pc + 1] == 0x84 else 4
        elif op == 0xAA:                                   # tableswitch
            p = pc + 1 + ((4 - (pc + 1) % 4) % 4)
            low = int.from_bytes(code[p + 4:p + 8], "big", signed=True)
            high = int.from_bytes(code[p + 8:p + 12], "big", signed=True)
            pc = p + 12 + 4 * (high - low + 1)
        elif op == 0xAB:                                   # lookupswitch
            p = pc + 1 + ((4 - (pc + 1) % 4) % 4)
            npairs = int.from_bytes(code[p + 4:p + 8], "big", signed=True)
            pc = p + 8 + 8 * npairs
        else:
            width = WIDTHS.get(op)
            if width is None:
                raise ValueError(f"unknown opcode 0x{op:02x} at {pc}")
            pc += 1 + width


def scan_class(b, hist, impossible, pool_census=None):
    if b[:4] != b"\xca\xfe\xba\xbe":
        raise ValueError("not a class file")
    tags, utf8s, off = parse_constant_pool(b, 8)
    if pool_census is not None:
        # Presence in the pool is a different question from being an `ldc` operand, and the answers
        # differ in practice: javac puts MethodHandle/MethodType in the pool constantly, as
        # bootstrap arguments of an invokedynamic, while never naming one with `ldc`. Counting only
        # the second would report "zero" for both "never uses these constants" and "uses them, but
        # not this way", which are not the same finding.
        for tag in tags.values():
            if tag in TARGET:
                pool_census[tag] += 1
    # access_flags, this_class, super_class, interfaces_count (2 bytes each), then the interfaces.
    off += 8 + 2 * int.from_bytes(b[off + 6:off + 8], "big")
    for _ in range(2):                                                # fields, then methods
        count = int.from_bytes(b[off:off + 2], "big")
        off += 2
        for _ in range(count):
            off += 6                                                  # access, name, descriptor
            attrs, off = skip_attributes(b, off)
            for name_index, start, length in attrs:
                if utf8s.get(name_index) == "Code":
                    code_len = int.from_bytes(b[start + 4:start + 8], "big")
                    walk_code(b[start + 8:start + 8 + code_len], tags, hist, impossible)
    return tags



def iter_classes(path):
    p = Path(path)
    if p.is_dir():
        for f in sorted(p.rglob("*.class")):
            yield str(f), f.read_bytes()
    elif p.suffix in (".jar", ".zip"):
        with zipfile.ZipFile(p) as z:
            for name in sorted(z.namelist()):
                if name.endswith(".class"):
                    yield f"{p.name}!{name}", z.read(name)
    else:
        yield str(p), p.read_bytes()


def main(argv):
    if len(argv) < 2:
        print(__doc__)
        return 64
    for path in argv[1:]:
        hist, impossible, classes, failed = Counter(), [], 0, []
        pool_census = Counter()
        for name, data in iter_classes(path):
            classes += 1
            try:
                scan_class(data, hist, impossible, pool_census)
            except Exception as exc:                        # a corrupt or unparsable entry is data, not a crash
                failed.append((name, str(exc)))
        sites = sum(hist.values())
        found = {TAG_NAMES.get(t, t): hist[t] for t in TARGET if hist[t]}
        print(f"\n{path}")
        print(f"  classes {classes} · ldc sites {sites} · unparsable classes {len(failed)}")
        print(f"  ★ tags 15/16/17 as an ldc operand: {found if found else 0}")
        in_pool = {TAG_NAMES[t]: pool_census[t] for t in TARGET if pool_census[t]}
        print(f"  same tags merely present in a constant pool: {in_pool if in_pool else 0}")
        print("  operand tags: " + ", ".join(
            f"{TAG_NAMES.get(t, t)} {c}" for t, c in sorted(hist.items(), key=lambda x: -x[1])))
        print(f"  impossible operands (scanner error bar): {len(impossible)}"
              f" ({100 * len(impossible) / sites:.2f}%)" if sites else "  impossible operands: n/a")
        for name, exc in failed[:3]:
            print(f"    unparsable: {name}: {exc}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
