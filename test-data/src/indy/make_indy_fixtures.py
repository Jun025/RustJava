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

    def method_type(self, descriptor):
        return self.add(u1(16) + u2(self.utf8(descriptor)))

    def method_handle(self, kind, reference_index):
        return self.add(u1(15) + u1(kind) + u2(reference_index))

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


METAFACTORY_CLASS = "java/lang/invoke/LambdaMetafactory"
METAFACTORY_DESCRIPTOR = (
    "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;"
    "Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;)"
    "Ljava/lang/invoke/CallSite;"
)


def lambda_near_miss(name, bootstrap_class, bootstrap_name, bootstrap_descriptor, bootstrap_kind=6):
    """The same near-miss idea as `near_miss_call_site`, for the *other* linked factory:
    `LambdaMetafactory.metafactory`.

    A separate builder rather than a parameter on that one, because the shapes differ in what the
    identity check has to see past. A metafactory bootstrap carries three static arguments —
    MethodType, MethodHandle, MethodType — and the call site returns the functional interface
    rather than a String. A near miss has to carry all of that correctly, or it is refused for
    the wrong reason and the identity check stays unobserved.

    `java/lang/Runnable` is the interface, and `()V` the method type, so that a fixture which
    *does* get linked (because an identity comparison was deleted) links to something real and
    runs to completion. Then the only difference a test can see is refusal versus no refusal,
    which is the difference being measured."""
    cp = Pool()
    this_class = cp.klass(name)
    super_class = cp.klass("java/lang/Object")
    main_name, main_desc, code_name = cp.utf8("main"), cp.utf8("([Ljava/lang/String;)V"), cp.utf8("Code")

    bootstrap = cp.method_handle(
        bootstrap_kind,
        cp.methodref(cp.klass(bootstrap_class), cp.name_and_type(bootstrap_name, bootstrap_descriptor)),
    )
    # samMethodType, implMethod, instantiatedMethodType. The implementation is this class's own
    # no-op static method, so a linked call site has something real to delegate to.
    sam_type = cp.method_type("()V")
    implementation = cp.method_handle(6, cp.methodref(this_class, cp.name_and_type("impl", "()V")))
    call_site = cp.add(u1(18) + u2(0) + u2(cp.name_and_type("run", "()Ljava/lang/Runnable;")))

    body = u1(0xBA) + u2(call_site) + u2(0) + b"\x57" + b"\xb1"  # invokedynamic; pop; return
    code_attr = u2(1) + u2(1) + u4(len(body)) + body + u2(0) + u2(0)
    main = u2(0x0009) + u2(main_name) + u2(main_desc) + u2(1) + u2(code_name) + u4(len(code_attr)) + code_attr

    impl_body = b"\xb1"  # return
    impl_code = u2(0) + u2(0) + u4(len(impl_body)) + impl_body + u2(0) + u2(0)
    impl = (
        u2(0x0008) + u2(cp.utf8("impl")) + u2(cp.utf8("()V")) + u2(1) + u2(code_name) + u4(len(impl_code)) + impl_code
    )

    bootstrap_body = u2(1) + u2(bootstrap) + u2(3) + u2(sam_type) + u2(implementation) + u2(sam_type)
    class_attributes = [u2(cp.utf8("BootstrapMethods")) + u4(len(bootstrap_body)) + bootstrap_body]

    return (
        b"\xca\xfe\xba\xbe" + u2(0) + u2(52) + cp.bytes()
        + u2(0x0021) + u2(this_class) + u2(super_class)
        + u2(0) + u2(0) + u2(2) + main + impl
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

# One per axis of the metafactory identity, for the same reason the four above exist: with only
# some of them, deleting a single comparison leaves every test green. The linked counterpart is
# javac's own output (Lambda.class, LambdaKinds.class) — these are what it must *not* be confused
# with.
METAFACTORY_FIXTURES = {
    # Differs in the owning class.
    "NotLambdaMetafactory.class": (
        "NotLambdaMetafactory",
        "java/lang/invoke/NotLambdaMetafactory",
        "metafactory",
        METAFACTORY_DESCRIPTOR,
    ),
    # Differs in the method name. `altMetafactory` is a real LambdaMetafactory bootstrap — the one
    # javac emits for serializable and multi-interface lambdas — so this is the near miss a
    # compiler could actually hand us.
    "NotMetafactory.class": (
        "NotMetafactory",
        METAFACTORY_CLASS,
        "altMetafactory",
        METAFACTORY_DESCRIPTOR,
    ),
    # Differs in the descriptor: `altMetafactory`'s, which is varargs where `metafactory`'s is
    # three explicit types. Still a well-formed method descriptor.
    "NotMetafactoryDescriptor.class": (
        "NotMetafactoryDescriptor",
        METAFACTORY_CLASS,
        "metafactory",
        "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;"
        "[Ljava/lang/Object;)Ljava/lang/invoke/CallSite;",
    ),
    # Differs in the reference kind: 7 (REF_invokeSpecial) instead of 6 (REF_invokeStatic).
    "NotInvokeStaticMetafactory.class": (
        "NotInvokeStaticMetafactory",
        METAFACTORY_CLASS,
        "metafactory",
        METAFACTORY_DESCRIPTOR,
        7,
    ),
}

if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for filename, args in FIXTURES.items():
        (OUT / filename).write_bytes(near_miss_call_site(*args))
        print(f"wrote {OUT / filename}")
    for filename, args in METAFACTORY_FIXTURES.items():
        (OUT / filename).write_bytes(lambda_near_miss(*args))
        print(f"wrote {OUT / filename}")
