#!/usr/bin/env python3
"""Emit the hand-assembled `invokedynamic` fixtures under `test-data/indy/`.

The javac-compiled fixtures next to these (StringConcat, Lambda, ConstantKinds) cover the
shapes a compiler actually emits. This file exists for the one shape a compiler never emits
and that nothing else can express: a call site whose bootstrap method is a **near miss** for
`StringConcatFactory.makeConcatWithConstants`.

Why that is worth a fixture: the linker identifies the factory by four axes — reference kind,
owning class, method name, descriptor. Without such a fixture the identity check is not
observable, because every other bootstrap we have on hand is *also* rejected for an unrelated
reason (Lambda and ConstantKinds carry MethodType/MethodHandle static arguments rather than the
String recipe the linker requires, so they never reach the identity check). A mutation that
deletes the identity check would then leave every test green while silently linking anything —
which is precisely the "linked everything" change this must not be.

`NotStringConcatFactory` differs from the real factory in exactly one axis: the owning class.
Kind, name, descriptor and the String static argument all match, so it reaches the identity
check and nothing else can reject it.

Regenerate with:  python3 test-data/src/indy/make_indy_fixtures.py
"""

import struct
from pathlib import Path

OUT = Path(__file__).resolve().parents[2] / "indy"

u1 = lambda x: struct.pack(">B", x)
u2 = lambda x: struct.pack(">H", x)
u4 = lambda x: struct.pack(">I", x)

FACTORY_CLASS = "java/lang/invoke/StringConcatFactory"
FACTORY_DESCRIPTOR = (
    "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;"
    "Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite;"
)


class Pool:
    def __init__(self):
        self.entries = []  # 1-based, no long/double so no double slots

    def add(self, blob):
        self.entries.append(blob)
        return len(self.entries)

    def utf8(self, s):
        b = s.encode()
        return self.add(u1(1) + u2(len(b)) + b)

    def string(self, s):
        return self.add(u1(8) + u2(self.utf8(s)))

    def klass(self, name):
        return self.add(u1(7) + u2(self.utf8(name)))

    def name_and_type(self, name, descriptor):
        return self.add(u1(12) + u2(self.utf8(name)) + u2(self.utf8(descriptor)))

    def methodref(self, class_index, nat_index):
        return self.add(u1(10) + u2(class_index) + u2(nat_index))

    def fieldref(self, class_index, nat_index):
        return self.add(u1(9) + u2(class_index) + u2(nat_index))

    def bytes(self):
        return u2(len(self.entries) + 1) + b"".join(self.entries)


def near_miss_call_site(name, bootstrap_class, bootstrap_name, bootstrap_descriptor, bootstrap_kind=6):
    """A class whose single `invokedynamic` names a bootstrap that is a near miss for the
    string-concat factory: same shape, one axis different.

    `bootstrap_kind` is the `reference_kind` of the CONSTANT_MethodHandle. It defaults to 6
    (REF_invokeStatic), what the real factory uses; 7 (REF_invokeSpecial) is the near miss for
    that axis. Both pair with a Methodref under JVMS 4.4.8, so the file stays valid and the
    identity check is the only thing that can refuse it — which is the point of these fixtures."""
    cp = Pool()
    this_class = cp.klass(name)
    super_class = cp.klass("java/lang/Object")
    main_name, main_desc, code_name = cp.utf8("main"), cp.utf8("([Ljava/lang/String;)V"), cp.utf8("Code")

    bootstrap = cp.add(
        u1(15)
        + u1(bootstrap_kind)
        + u2(cp.methodref(cp.klass(bootstrap_class), cp.name_and_type(bootstrap_name, bootstrap_descriptor)))
    )
    recipe = cp.string("linked-by-mistake")
    # The call site takes nothing and yields a String, like a constant-only concat would.
    call_site = cp.add(u1(18) + u2(0) + u2(cp.name_and_type("makeConcat", "()Ljava/lang/String;")))

    body = u1(0xBA) + u2(call_site) + u2(0) + b"\x57" + b"\xb1"  # invokedynamic; pop; return
    code_attr = u2(1) + u2(1) + u4(len(body)) + body + u2(0) + u2(0)
    method = u2(0x0009) + u2(main_name) + u2(main_desc) + u2(1) + u2(code_name) + u4(len(code_attr)) + code_attr

    bootstrap_body = u2(1) + u2(bootstrap) + u2(1) + u2(recipe)  # one method, one static argument
    class_attributes = [u2(cp.utf8("BootstrapMethods")) + u4(len(bootstrap_body)) + bootstrap_body]

    return (
        b"\xca\xfe\xba\xbe" + u2(0) + u2(52) + cp.bytes()
        + u2(0x0021) + u2(this_class) + u2(super_class)
        + u2(0) + u2(0) + u2(1) + method
        + u2(len(class_attributes)) + b"".join(class_attributes)
    )


def recipe_arity_call_site(name, recipe, arguments):
    """A call site that *is* the factory, but whose recipe and descriptor disagree.

    The recipe and the call site descriptor are two accounts of the same concatenation, written
    by the same compiler, so javac cannot emit this — only a corrupt or hand-built file can, which
    is why it is assembled here. `recipe` is the bootstrap's static argument; `arguments` are the
    strings the call site actually pushes, and its descriptor is derived from them.

    The class prints the result, so a disagreement the runtime fails to notice shows up as the
    wrong text rather than as nothing at all."""
    cp = Pool()
    this_class = cp.klass(name)
    super_class = cp.klass("java/lang/Object")
    main_name, main_desc, code_name = cp.utf8("main"), cp.utf8("([Ljava/lang/String;)V"), cp.utf8("Code")

    bootstrap = cp.add(
        u1(15)  # kind 6 = REF_invokeStatic, as the real factory is
        + u1(6)
        + u2(cp.methodref(cp.klass(FACTORY_CLASS), cp.name_and_type("makeConcatWithConstants", FACTORY_DESCRIPTOR)))
    )
    descriptor = "(" + "Ljava/lang/String;" * len(arguments) + ")Ljava/lang/String;"
    call_site = cp.add(u1(18) + u2(0) + u2(cp.name_and_type("concat", descriptor)))
    out = cp.fieldref(cp.klass("java/lang/System"), cp.name_and_type("out", "Ljava/io/PrintStream;"))
    println = cp.methodref(cp.klass("java/io/PrintStream"), cp.name_and_type("println", "(Ljava/lang/String;)V"))
    recipe_index = cp.string(recipe)

    body = u1(0xB2) + u2(out)  # getstatic System.out
    for argument in arguments:
        body += u1(0x12) + u1(cp.string(argument))  # ldc argument
    body += (
        u1(0xBA) + u2(call_site) + u2(0)  # invokedynamic concat(String...)String
        + u1(0xB6) + u2(println)          # invokevirtual println(String)V
        + b"\xb1"                         # return
    )
    code_attr = u2(1 + len(arguments)) + u2(1) + u4(len(body)) + body + u2(0) + u2(0)
    method = u2(0x0009) + u2(main_name) + u2(main_desc) + u2(1) + u2(code_name) + u4(len(code_attr)) + code_attr

    bootstrap_body = u2(1) + u2(bootstrap) + u2(1) + u2(recipe_index)  # one method, the recipe
    class_attributes = [u2(cp.utf8("BootstrapMethods")) + u4(len(bootstrap_body)) + bootstrap_body]

    return (
        b"\xca\xfe\xba\xbe" + u2(0) + u2(52) + cp.bytes()
        + u2(0x0021) + u2(this_class) + u2(super_class)
        + u2(0) + u2(0) + u2(1) + method
        + u2(len(class_attributes)) + b"".join(class_attributes)
    )


MAKECONCAT_DESCRIPTOR = "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite;"


def make_concat_call_site(name, left, right, bootstrap_descriptor=None, static_arguments=0):
    """A call site bound to `StringConcatFactory.makeConcat` — the recipe-free sibling.

    It takes no static arguments: the call site descriptor alone says what to concatenate. The
    class prints the result, so a test can assert *what* was concatenated rather than only that
    the call site linked — a synthesised recipe of the wrong length would still link and still
    run, and would go unnoticed if the result were discarded.

    Hand-assembled because javac does not emit this shape: targets 9 through 26 all emit
    `makeConcatWithConstants`, and only the non-default `-XDstringConcat=indy` produces
    `makeConcat`. Depending on an internal compiler flag to regenerate a fixture would be worse
    than assembling it here.

    `bootstrap_descriptor` defaults to the real one. Passing another is the near miss for this
    entry point's descriptor axis: with no static arguments, the argument-count guard cannot
    refuse it, so only the descriptor comparison can — which is what makes that comparison
    observable. Measured: without this fixture, loosening the check to match the name alone left
    every test green.

    `static_arguments` is the near miss for the other half of the same rule: this entry point is
    defined as taking none, so a bootstrap that names it *and* carries one is a contradiction. With
    the descriptor correct, only the argument-count guard can refuse it."""
    cp = Pool()
    this_class = cp.klass(name)
    super_class = cp.klass("java/lang/Object")
    main_name, main_desc, code_name = cp.utf8("main"), cp.utf8("([Ljava/lang/String;)V"), cp.utf8("Code")

    bootstrap = cp.add(
        u1(15)  # kind 6 = REF_invokeStatic, as the real factory is
        + u1(6)
        + u2(cp.methodref(cp.klass(FACTORY_CLASS), cp.name_and_type("makeConcat", bootstrap_descriptor or MAKECONCAT_DESCRIPTOR)))
    )
    call_site = cp.add(u1(18) + u2(0) + u2(cp.name_and_type("concat", "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")))
    out = cp.fieldref(cp.klass("java/lang/System"), cp.name_and_type("out", "Ljava/io/PrintStream;"))
    println = cp.methodref(cp.klass("java/io/PrintStream"), cp.name_and_type("println", "(Ljava/lang/String;)V"))
    left_index, right_index = cp.string(left), cp.string(right)

    body = (
        u1(0xB2) + u2(out)                      # getstatic System.out
        + u1(0x12) + u1(left_index)             # ldc left
        + u1(0x12) + u1(right_index)            # ldc right
        + u1(0xBA) + u2(call_site) + u2(0)      # invokedynamic concat(String, String)String
        + u1(0xB6) + u2(println)                # invokevirtual println(String)V
        + b"\xb1"                               # return
    )
    code_attr = u2(3) + u2(1) + u4(len(body)) + body + u2(0) + u2(0)
    method = u2(0x0009) + u2(main_name) + u2(main_desc) + u2(1) + u2(code_name) + u4(len(code_attr)) + code_attr

    arguments = b"".join(u2(left_index) for _ in range(static_arguments))
    bootstrap_body = u2(1) + u2(bootstrap) + u2(static_arguments) + arguments  # one bootstrap method
    class_attributes = [u2(cp.utf8("BootstrapMethods")) + u4(len(bootstrap_body)) + bootstrap_body]

    return (
        b"\xca\xfe\xba\xbe" + u2(0) + u2(52) + cp.bytes()
        + u2(0x0021) + u2(this_class) + u2(super_class)
        + u2(0) + u2(0) + u2(1) + method
        + u2(len(class_attributes)) + b"".join(class_attributes)
    )


FIXTURES = {
    # Differs from the real factory in the owning class alone. Everything else — kind 6, the
    # name, the descriptor, a String first static argument — matches, so only the identity
    # check can refuse it.
    "NotStringConcatFactory.class": (
        "NotStringConcatFactory",
        "java/lang/invoke/NotStringConcatFactory",
        "makeConcatWithConstants",
        FACTORY_DESCRIPTOR,
    ),
    # One fixture per remaining axis. Without all four, deleting a single axis from the identity
    # check leaves every test green — measured: with only the owning-class fixture present, the
    # kind, name and descriptor axes could each be removed and `cargo test --all` stayed at
    # 570 passed / 0 failed. A mutation that deletes all four at once dies on the class fixture
    # alone, which is why the audit that ran it read "this branch is covered".
    #
    # Differs in the method name. `makeConcat` is a real StringConcatFactory bootstrap, so this is
    # the near miss a compiler could actually hand us.
    "NotMakeConcatWithConstants.class": (
        "NotMakeConcatWithConstants",
        FACTORY_CLASS,
        "makeConcat",
        FACTORY_DESCRIPTOR,
    ),
    # Differs in the descriptor: the trailing `[Ljava/lang/Object;` (the constants varargs) is
    # gone. Still a well-formed method descriptor, so nothing upstream rejects it.
    "NotFactoryDescriptor.class": (
        "NotFactoryDescriptor",
        FACTORY_CLASS,
        "makeConcatWithConstants",
        "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;"
        "Ljava/lang/String;)Ljava/lang/invoke/CallSite;",
    ),
    # Differs in the reference kind: 7 (REF_invokeSpecial) instead of 6 (REF_invokeStatic).
    "NotInvokeStaticFactory.class": (
        "NotInvokeStaticFactory",
        FACTORY_CLASS,
        "makeConcatWithConstants",
        FACTORY_DESCRIPTOR,
        7,
    ),
}

# The linkable counterpart of the near misses above: this one *is* the factory, so it must run.
LINKED = {
    "MakeConcat.class": ("MakeConcat", "a", "b"),
    # Same shape, but the bootstrap claims the *other* entry point's descriptor. It takes no static
    # arguments, so nothing but the descriptor comparison stands between it and being linked.
    "MakeConcatWrongDescriptor.class": ("MakeConcatWrongDescriptor", "a", "b", FACTORY_DESCRIPTOR),
    # Correct name and descriptor, but carrying a static argument this entry point does not take.
    "MakeConcatWithArgument.class": ("MakeConcatWithArgument", "a", "b", None, 1),
}

# Linked bootstraps whose recipe contradicts the call site, one fixture per direction. OpenJDK 26
# refuses both at linkage with BootstrapMethodError (StringConcatException: "Recipe and method type
# do not match"), before any argument is converted — measured, not assumed.
RECIPE_ARITY = {
    # Recipe wants two arguments, the call site pushes one.
    "RecipeWantsMoreArguments.class": ("RecipeWantsMoreArguments", "\u0001\u0001", ["a"]),
    # The other direction, which no mid-recipe guard can see: the recipe runs out first, so the
    # second argument is silently dropped and the concatenation quietly returns the wrong string.
    "RecipeWantsFewerArguments.class": ("RecipeWantsFewerArguments", "\u0001", ["a", "b"]),
    # The same disagreement on the constants axis: \u0002 consumes a constant the bootstrap did not
    # carry. Held by the same comparison, so it is a fixture rather than a second mechanism.
    "RecipeWantsAConstant.class": ("RecipeWantsAConstant", "\u0001\u0002", ["a"]),
}

if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for filename, args in FIXTURES.items():
        (OUT / filename).write_bytes(near_miss_call_site(*args))
        print(f"wrote {OUT / filename}")
    for filename, args in LINKED.items():
        (OUT / filename).write_bytes(make_concat_call_site(*args))
        print(f"wrote {OUT / filename}")
    for filename, args in RECIPE_ARITY.items():
        (OUT / filename).write_bytes(recipe_arity_call_site(*args))
        print(f"wrote {OUT / filename}")
