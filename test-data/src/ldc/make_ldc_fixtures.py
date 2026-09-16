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


def unknown_tag(tag):
    """A tag `parse_tagged` rejects, in the same 3-byte shape as a MethodType so the entry
    `ldc` names is the mutated one and nothing after it shifts."""

    def build(cp, _attributes):
        return cp.add(u1(tag) + u2(cp.utf8("()V")))

    return build


def dynamic_without_bootstrap_methods(cp, _attributes):
    """A Dynamic entry naming bootstrap method 0 of an attribute that is not there. JVMS 4.7.23
    requires the attribute whenever the pool holds a Dynamic/InvokeDynamic entry, so this is a
    corrupt file — OpenJDK 26 says `ClassFormatError: Missing BootstrapMethods attribute`."""
    return cp.add(u1(17) + u2(0) + u2(cp.name_and_type("x", "Ljava/lang/Object;")))


def dynamic(name, descriptor, bootstrap_method, bootstrap_descriptor, attr_index=0, static_arguments=()):
    """A real condy, so the file is structurally complete: JVMS 4.7.23 requires the
    BootstrapMethods attribute that a Dynamic entry indexes into.

    `attr_index` is the `bootstrap_method_attr_index` written into the entry. It defaults to 0,
    the one entry this builder emits; passing anything else produces the out-of-range case, which
    is the other half of the same rule and cannot be built any other way — the table is written
    here, so only here can the index be made to overshoot it.

    `static_arguments` are the `bootstrap_arguments` pool indices. The default is empty, which is
    what `ConstantBootstraps.nullConstant` takes; passing an index that is not in the pool is the
    only way to build the unbounded-argument case, for the same reason — the table is written
    here."""

    def build(cp, attributes):
        bootstrap = cp.add(
            u1(15)  # kind 6 = REF_invokeStatic
            + u1(6)
            + u2(cp.methodref(cp.klass("java/lang/invoke/ConstantBootstraps"), cp.name_and_type(bootstrap_method, bootstrap_descriptor)))
        )
        entry = cp.add(u1(17) + u2(attr_index) + u2(cp.name_and_type(name, descriptor)))

        arguments = b"".join(u2(x) for x in static_arguments)
        body = u2(1) + u2(bootstrap) + u2(len(static_arguments)) + arguments  # one bootstrap method
        attributes.append(u2(cp.utf8("BootstrapMethods")) + u4(len(body)) + body)
        return entry

    return build


LOOKUP = "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Object;"
null_constant = dynamic("x", "Ljava/lang/Object;", "nullConstant", LOOKUP)
# Same file, but the bootstrap method's one static argument names a pool index that is not there.
# 0xFFFF is past the end of any pool this generator builds, and index 0 is never a valid entry.
bad_argument_constant = dynamic("x", "Ljava/lang/Object;", "nullConstant", LOOKUP, static_arguments=(0xFFFF,))
# Same file, but the entry names bootstrap method 1 of a table holding only method 0.
past_end_constant = dynamic("x", "Ljava/lang/Object;", "nullConstant", LOOKUP, attr_index=1)
# Long.MAX_VALUE, i.e. `J`-typed, which JVMS 6.5 puts on the `ldc2_w` side of the split.
long_constant = dynamic("MAX_VALUE", "J", "getStaticFinal", LOOKUP)


# JVMS 4.4 ties each constant kind to a minimum class file version: tags 15/16/18 need
# major >= 51, tag 17 needs major >= 55. That is why LdcDynamic is major 55 while the rest are
# 52 — emitting tag 17 at 52 makes a real JVM answer "Class file version does not support
# constant tag 17", a *version* complaint that would hide the axis those fixtures test.
# LdcDynamicOldMajor below is that complaint on purpose.


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
    # Negative control 2: same shape, but the entry `ldc` names carries a tag that cannot
    # appear in a class file at all — 13 and 14 are unassigned by JVMS 4.4, and 19 (Module) is
    # assigned but legal only inside a module-info. A tag that cannot appear here is still a
    # corrupt file, whatever `ldc` was widened to accept.
    "LdcTag13.class": ("LdcTag13", unknown_tag(13), ldc, 1),
    "LdcTag14.class": ("LdcTag14", unknown_tag(14), ldc, 1),
    "LdcUnknownTag.class": ("LdcUnknownTag", unknown_tag(19), ldc, 1),
    # Negative control 3: a *valid* condy shape at a class file version that predates it
    # (JVMS 4.4 ties tag 17 to major >= 55). OpenJDK 26: "Class file version does not support
    # constant tag 17". Widening `ldc` removed the accidental backstop that used to catch this.
    "LdcDynamicOldMajor.class": ("LdcDynamicOldMajor", null_constant, ldc, 1, 52),
    # Negative control 4, two halves of one rule (JVMS 4.4.10 / 4.7.23): a Dynamic entry has to
    # name a real bootstrap method. It can fail by the attribute being absent, or by the index
    # overshooting a table that is present — OpenJDK 26 rejects both.
    "LdcDynamicNoBSM.class": ("LdcDynamicNoBSM", dynamic_without_bootstrap_methods, ldc, 1, 55),
    # Negative control 5 (JVMS 4.7.23): a bootstrap method's static arguments are pool indices too,
    # and an index naming nothing is a broken file rather than a feature we have not implemented.
    "LdcDynamicBSMArgPastEnd.class": ("LdcDynamicBSMArgPastEnd", bad_argument_constant, ldc, 1, 55),
    "LdcDynamicBSMIndexPastEnd.class": ("LdcDynamicBSMIndexPastEnd", past_end_constant, ldc, 1, 55),
}

if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for filename, (name, constant, code, max_stack, *major) in FIXTURES.items():
        (OUT / filename).write_bytes(class_file(name, constant, code, max_stack, *major))
        print(f"wrote {OUT / filename}")
