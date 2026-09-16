#!/usr/bin/env python3
"""Emit the constant-pool tag fixtures under `test-data/cp/`.

These exist for one reason: to make the tag switch's pass-through branch observable end to end.

`tests/test_class_format.rs` already had a test for "we still reject unknown constant pool tags",
built by overwriting the tag byte of `test-data/Hello.class`'s first pool entry. That entry is a
Methodref the code invokes, so overwriting it breaks the class along several independent paths at
once — the operand of `invokespecial` stops being a method reference, and so on. `ClassFileError`
collapses every parse failure into a flat "Invalid class file", so the assertion cannot tell
"rejected because the tag is unknown" from "rejected because the class fell apart". Measured: with
the pass-through branch mutated from reject to accept, that test still passed.

The fixtures here make the unknown tag the **only** thing wrong:

  * the entry is unreferenced — nothing in the code or the class structure points at it, so no
    downstream consumer can reject it on the entry's behalf;
  * it is the **last** pool entry and carries **no payload**, which is what an unassigned tag
    looks like — there is no defined size for it.

Both properties are load-bearing. Being last and payload-free means a pass-through that consumes
nothing leaves the reader correctly positioned at `access_flags`, so a mutated parser produces a
*working* class rather than a differently-broken one. That is what turns the mutation into a
red test instead of a green one for the wrong reason.

Tags 13 and 14 are unassigned by JVMS 4.4; 19 (Module) is assigned but legal only inside a
module-info, so it cannot appear here either.

Regenerate with:  python3 test-data/src/cp/make_cp_fixtures.py
"""

import struct
from pathlib import Path

OUT = Path(__file__).resolve().parents[2] / "cp"

u1 = lambda x: struct.pack(">B", x)
u2 = lambda x: struct.pack(">H", x)
u4 = lambda x: struct.pack(">I", x)


class Pool:
    def __init__(self):
        self.entries = []  # 1-based, no long/double so no double slots

    def add(self, blob):
        self.entries.append(blob)
        return len(self.entries)

    def utf8(self, s):
        b = s.encode()
        return self.add(u1(1) + u2(len(b)) + b)

    def klass(self, name):
        return self.add(u1(7) + u2(self.utf8(name)))

    def bytes(self):
        return u2(len(self.entries) + 1) + b"".join(self.entries)


def unreferenced_unknown_tag(name, tag):
    """A class that is valid in every respect except for one trailing, unreferenced pool entry
    whose tag cannot appear in a class file."""
    cp = Pool()
    this_class = cp.klass(name)
    super_class = cp.klass("java/lang/Object")
    main_name, main_desc, code_name = cp.utf8("main"), cp.utf8("([Ljava/lang/String;)V"), cp.utf8("Code")

    # Last, and payload-free — see the module docstring; both properties are load-bearing.
    cp.add(u1(tag))

    body = b"\xb1"  # return
    code_attr = u2(0) + u2(1) + u4(len(body)) + body + u2(0) + u2(0)
    method = u2(0x0009) + u2(main_name) + u2(main_desc) + u2(1) + u2(code_name) + u4(len(code_attr)) + code_attr

    return (
        b"\xca\xfe\xba\xbe" + u2(0) + u2(52) + cp.bytes()
        + u2(0x0021) + u2(this_class) + u2(super_class)
        + u2(0) + u2(0) + u2(1) + method
        + u2(0)  # no class attributes
    )


FIXTURES = {f"UnreferencedTag{tag}.class": (f"UnreferencedTag{tag}", tag) for tag in (13, 14, 19)}

if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for filename, args in FIXTURES.items():
        (OUT / filename).write_bytes(unreferenced_unknown_tag(*args))
        print(f"wrote {OUT / filename}")
