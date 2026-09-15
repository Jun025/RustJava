#!/usr/bin/env python3
"""Emit the `ldc`-of-method-handle-family fixtures under `test-data/ldc/`.

These are **synthetic**: javac has no source construct that makes `ldc` name a
CONSTANT_MethodHandle (15), MethodType (16) or Dynamic (17) entry — measured, see
`docs/worklog/2026-09-16-ldc-tags-15-16-17.md`. Only a bytecode generator (ASM,
`java.lang.classfile`, jasm) produces them, so the fixtures are assembled here
byte by byte. That also buys the two negative controls, which no class-file
builder API would let us construct: they are deliberately malformed.

Regenerate with:  python3 test-data/src/ldc/make_ldc_fixtures.py
"""

import struct
from pathlib import Path

OUT = Path(__file__).resolve().parents[2] / "ldc"

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

    def name_and_type(self, name, descriptor):
        return self.add(u1(12) + u2(self.utf8(name)) + u2(self.utf8(descriptor)))

    def methodref(self, class_index, nat_index):
        return self.add(u1(10) + u2(class_index) + u2(nat_index))

    def bytes(self):
        return u2(len(self.entries) + 1) + b"".join(self.entries)


def class_file(name, build_constant, code, max_stack, major=52):
    """A class with a single `static void main(String[])` holding `code` + return."""
    cp = Pool()
    this_class = cp.klass(name)
    super_class = cp.klass("java/lang/Object")
    main_name, main_desc, code_name = cp.utf8("main"), cp.utf8("([Ljava/lang/String;)V"), cp.utf8("Code")
    class_attributes = []
    constant_index = build_constant(cp, class_attributes)

    body = code(constant_index) + b"\xb1"  # ... return
    code_attr = u2(max_stack) + u2(1) + u4(len(body)) + body + u2(0) + u2(0)
    method = u2(0x0009) + u2(main_name) + u2(main_desc) + u2(1) + u2(code_name) + u4(len(code_attr)) + code_attr

    return (
        b"\xca\xfe\xba\xbe" + u2(0) + u2(major) + cp.bytes()
        + u2(0x0021) + u2(this_class) + u2(super_class)
        + u2(0) + u2(0) + u2(1) + method
        + u2(len(class_attributes)) + b"".join(class_attributes)
    )


def method_handle(cp, _attributes):
    # kind 5 = REF_invokeVirtual, which JVMS 4.4.8 pairs with a Methodref.
    return cp.add(u1(15) + u1(5) + u2(cp.methodref(cp.klass("java/lang/Object"), cp.name_and_type("hashCode", "()I"))))


def method_type(cp, _attributes):
    return cp.add(u1(16) + u2(cp.utf8("()V")))


def unknown_tag(cp, _attributes):
    return cp.add(u1(19) + u2(cp.utf8("()V")))


def dynamic(name, descriptor, bootstrap_method, bootstrap_descriptor):
    """A real condy, so the file is structurally complete: JVMS 4.7.23 requires the
    BootstrapMethods attribute that a Dynamic entry indexes into."""

    def build(cp, attributes):
        bootstrap = cp.add(
            u1(15)  # kind 6 = REF_invokeStatic
            + u1(6)
            + u2(cp.methodref(cp.klass("java/lang/invoke/ConstantBootstraps"), cp.name_and_type(bootstrap_method, bootstrap_descriptor)))
        )
        entry = cp.add(u1(17) + u2(0) + u2(cp.name_and_type(name, descriptor)))

        body = u2(1) + u2(bootstrap) + u2(0)  # one bootstrap method, no static arguments
        attributes.append(u2(cp.utf8("BootstrapMethods")) + u4(len(body)) + body)
        return entry

    return build


LOOKUP = "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Object;"
null_constant = dynamic("x", "Ljava/lang/Object;", "nullConstant", LOOKUP)
# Long.MAX_VALUE, i.e. `J`-typed, which JVMS 6.5 puts on the `ldc2_w` side of the split.
long_constant = dynamic("MAX_VALUE", "J", "getStaticFinal", LOOKUP)


# JVMS 4.4: tags 15/16 need major >= 51, tag 17 needs major >= 55. Emitting tag 17 at 52
# makes a real JVM answer "ClassFormatError: Class file version does not support constant
# tag 17" — a version complaint, which would hide the axis this fixture is here to test.


ldc = lambda i: u1(0x12) + u1(i) + b"\x57"  # ldc <i>; pop
ldc2_w = lambda i: u1(0x14) + u2(i) + b"\x58"  # ldc2_w <i>; pop2

# filename -> (class name, constant builder, code builder, max_stack, major version)
FIXTURES = {
    # `ldc` of a constant kind this runtime cannot resolve yet — unsupported, not corrupt.
    "LdcMethodHandle.class": ("LdcMethodHandle", method_handle, ldc, 1),
    "LdcMethodType.class": ("LdcMethodType", method_type, ldc, 1),
    "LdcDynamic.class": ("LdcDynamic", null_constant, ldc, 1, 55),
    "Ldc2WDynamic.class": ("Ldc2WDynamic", long_constant, ldc2_w, 2, 55),
    # Negative control 1: JVMS 6.5 lets `ldc2_w` load only long/double(-typed) constants,
    # so a MethodType there is a corrupt file and must stay one.
    "Ldc2WMethodType.class": ("Ldc2WMethodType", method_type, ldc2_w, 2),
    # Negative control 2: same shape, but the entry `ldc` names carries tag 19 (Module —
    # same 3-byte width as MethodType, legal only in a module-info). A tag that cannot
    # appear here is still a corrupt file.
    "LdcUnknownTag.class": ("LdcUnknownTag", unknown_tag, ldc, 1),
}

if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for filename, (name, constant, code, max_stack, *major) in FIXTURES.items():
        (OUT / filename).write_bytes(class_file(name, constant, code, max_stack, *major))
        print(f"wrote {OUT / filename}")
