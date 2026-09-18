#!/usr/bin/env python3
"""Emit the duplicate-class-attribute fixtures under `test-data/attr/`.

JVMS 4.7 marks several ClassFile attributes as at-most-one. `classfile/src/validation.rs` rejects a
duplicate of the ones a real JVM rejects — and *only* those, because the spec and HotSpot disagree
in two places that these fixtures pin:

* `DuplicateNestHostOldMajor` carries two `NestHost` attributes at major 52, where the attribute is
  not defined yet. Unrecognised attributes are ignored (JVMS 4.7.1), so the file is legal and must
  keep loading. Counting without a version gate would reject it.
* `DuplicateSynthetic` carries two `Synthetic` attributes. JVMS 4.7.8 says at most one; OpenJDK
  26.0.1 loads it anyway. We follow the JVM, so this one must keep loading too.

Measured 2026-09-17 with OpenJDK 26.0.1, one attribute at a time, via `Class.forName`:

    Multiple SourceFile / InnerClasses / SourceDebugExtension / BootstrapMethods (52)  -> ClassFormatError
    Multiple NestHost / NestMembers (55)                                               -> ClassFormatError
    Multiple NestHost (52)                                                             -> loads
    Multiple Synthetic (52)                                                            -> loads

These are hand-assembled because no compiler emits a duplicate attribute; that is the whole point.

Regenerate with:  python3 test-data/src/attr/make_attr_fixtures.py
"""

import struct
from pathlib import Path

OUT = Path(__file__).resolve().parents[2] / "attr"

u1 = lambda x: struct.pack(">B", x)
u2 = lambda x: struct.pack(">H", x)
u4 = lambda x: struct.pack(">I", x)


class Pool:
    def __init__(self):
        self.entries = []

    def add(self, entry):
        self.entries.append(entry)
        return len(self.entries)

    def utf8(self, text):
        raw = text.encode()
        return self.add(u1(1) + u2(len(raw)) + raw)

    def klass(self, name):
        return self.add(u1(7) + u2(self.utf8(name)))

    def bytes(self):
        return u2(len(self.entries) + 1) + b"".join(self.entries)


def payload(attribute, pool, this_class):
    """The smallest well-formed body for each attribute — the duplication is what is under test."""
    if attribute == "Synthetic":
        return b""
    if attribute == "SourceDebugExtension":
        return b"\x00"  # one arbitrary byte; the attribute is an unstructured UTF-8 blob
    if attribute in ("InnerClasses", "NestMembers"):
        return u2(0)  # an empty table
    if attribute == "NestHost":
        return u2(this_class)
    if attribute == "SourceFile":
        return u2(pool.utf8("Attr.java"))
    raise SystemExit(f"no payload for {attribute}")


def duplicated_attribute(name, attribute, major, times=2):
    """A class that is valid in every respect except for carrying `attribute` `times` over."""
    pool = Pool()
    this_class = pool.klass(name)
    super_class = pool.klass("java/lang/Object")
    main_name, main_desc, code_name = pool.utf8("main"), pool.utf8("([Ljava/lang/String;)V"), pool.utf8("Code")

    body = b"\xb1"  # return
    code_attr = u2(0) + u2(1) + u4(len(body)) + body + u2(0) + u2(0)
    method = u2(0x0009) + u2(main_name) + u2(main_desc) + u2(1) + u2(code_name) + u4(len(code_attr)) + code_attr

    attribute_name = pool.utf8(attribute)
    info = payload(attribute, pool, this_class)
    attributes = (u2(attribute_name) + u4(len(info)) + info) * times

    return (
        b"\xca\xfe\xba\xbe" + u2(0) + u2(major) + pool.bytes()
        + u2(0x0021) + u2(this_class) + u2(super_class)
        + u2(0) + u2(0) + u2(1) + method
        + u2(times) + attributes
    )


# (filename, attribute, class file version) — the last two must keep loading, see the module docstring.
FIXTURES = {
    "DuplicateSourceFile.class": ("DuplicateSourceFile", "SourceFile", 52),
    "DuplicateInnerClasses.class": ("DuplicateInnerClasses", "InnerClasses", 52),
    "DuplicateSourceDebugExtension.class": ("DuplicateSourceDebugExtension", "SourceDebugExtension", 52),
    "DuplicateNestHost.class": ("DuplicateNestHost", "NestHost", 55),
    "DuplicateNestMembers.class": ("DuplicateNestMembers", "NestMembers", 55),
    "DuplicateNestHostOldMajor.class": ("DuplicateNestHostOldMajor", "NestHost", 52),
    "DuplicateSynthetic.class": ("DuplicateSynthetic", "Synthetic", 52),
}

if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for filename, args in FIXTURES.items():
        (OUT / filename).write_bytes(duplicated_attribute(*args))
        print(f"wrote {OUT / filename}")
