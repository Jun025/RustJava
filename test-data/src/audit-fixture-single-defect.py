#!/usr/bin/env python3
"""Audit the hand-assembled fixtures for **single-defectness**.

The mutation audits so far asked: does each test fail when the thing it names is broken? This asks
the other half — is each *fixture* broken in exactly one way? A fixture that is wrong twice keeps
its test passing after one of the two is fixed, and nobody notices that the other stopped being
covered. That is silent, and it is the failure this script exists to make loud.

## The test it applies

For a generated fixture the question has an exact answer, because the generator takes the defect as
a parameter: **repair the named defect and the bytes must equal the canonical build.**

    generator(name, …defect…)  ==  the committed fixture       (the generator still describes it)
    generator(name, …repaired…) ==  generator(name, …canonical…)   (nothing else differs)

Byte equality is stronger than "the repaired file loads": a second defect that happens not to
matter to our loader would pass the loading test and fail this one. It is also cheaper — no JVM.

## What it does not cover, said plainly

* **javac-compiled fixtures** (`Lambda`, `StringConcat`, `ConstantKinds`, `LambdaKinds` …) are not
  defective at all; they are the positive controls. "Exactly one defect" does not apply to zero.
* **Legal-but-unimplemented fixtures** (`LdcMethodHandle`, `LdcMethodType`, `LdcDynamic`,
  `Ldc2WDynamic`, and the `indy` near misses' *linked* counterpart `MakeConcat`) carry **no defect**
  either — the file is valid and the feature is missing. They are listed as `no defect` below rather
  than skipped, so the census stays complete.
* **Fixtures mutated inside a test** (`tests/test_class_format.rs` builds several by patching bytes
  at run time) are not on disk for this script to read. Their single-defectness is argued in the
  comment next to each, which is weaker — see the report for that round.

Run:  python3 test-data/src/audit-fixture-single-defect.py     (rc=0 clean, rc=1 a fixture is
      wrong in more than one way, rc=2 the generator no longer reproduces a committed fixture)
"""

import importlib.util
import pathlib
import sys

# 생성기를 import 로 불러오므로 그대로 두면 `test-data/src/*/__pycache__` 가 남는다. 이 트리는 여러
# 세션이 공유하므로 «내가 만든 쓰레기»를 남기지 않는다 — 읽기만 하는 감사에 캐시는 필요도 없다.
sys.dont_write_bytecode = True

ROOT = pathlib.Path(__file__).resolve().parents[2]


def load(relative):
    path = ROOT / relative
    spec = importlib.util.spec_from_file_location(path.stem.replace("-", "_"), path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


indy = load("test-data/src/indy/make_indy_fixtures.py")
ldc = load("test-data/src/ldc/make_ldc_fixtures.py")
cp = load("test-data/src/cp/make_cp_fixtures.py")

# (directory, filename, defect axis, committed-bytes builder, repaired builder, canonical builder)
# A `None` axis means the fixture carries no defect — it is a valid file, listed to keep the census
# complete rather than to be checked.
CASES = []


def indy_cases():
    factory = dict(
        bootstrap_class=indy.FACTORY_CLASS,
        bootstrap_name="makeConcatWithConstants",
        bootstrap_descriptor=indy.FACTORY_DESCRIPTOR,
        bootstrap_kind=6,
    )
    axes = {
        "NotStringConcatFactory.class": ("owning class", "bootstrap_class"),
        "NotMakeConcatWithConstants.class": ("method name", "bootstrap_name"),
        "NotFactoryDescriptor.class": ("descriptor", "bootstrap_descriptor"),
        "NotInvokeStaticFactory.class": ("reference kind", "bootstrap_kind"),
    }
    for filename, (axis, field) in axes.items():
        spec = indy.FIXTURES[filename]
        given = dict(
            name=spec[0], bootstrap_class=spec[1], bootstrap_name=spec[2], bootstrap_descriptor=spec[3]
        )
        if len(spec) > 4:
            given["bootstrap_kind"] = spec[4]
        repaired = dict(given, **{field: factory[field]})
        canonical = dict(factory, name=spec[0])
        CASES.append(
            (
                "indy",
                filename,
                axis,
                lambda g=given: indy.near_miss_call_site(**g),
                lambda r=repaired: indy.near_miss_call_site(**r),
                lambda c=canonical: indy.near_miss_call_site(**c),
            )
        )

    # The recipe-free entry point. `MakeConcat` itself is the canonical — it is *linked*, not a near
    # miss — so it is the `no defect` row that the other two are measured against.
    axes = {
        "MakeConcat.class": None,
        "MakeConcatWrongDescriptor.class": "descriptor",
        "MakeConcatWithArgument.class": "static argument",
    }
    for filename, axis in axes.items():
        spec = indy.LINKED[filename]
        CASES.append(
            (
                "indy",
                filename,
                axis,
                lambda s=spec: indy.make_concat_call_site(*s),
                lambda s=spec: indy.make_concat_call_site(s[0], s[1], s[2]),
                lambda s=spec: indy.make_concat_call_site(s[0], s[1], s[2]),
            )
        )


def ldc_cases():
    # Repair -> the parameters the canonical build uses. `None` marks a valid file (the feature is
    # unimplemented, which is not a defect in the fixture).
    repairs = {
        "LdcMethodHandle.class": None,
        "LdcMethodType.class": None,
        "LdcDynamic.class": None,
        "Ldc2WDynamic.class": None,
        "Ldc2WMethodType.class": ("ldc2_w width", (ldc.method_type, ldc.ldc, 1)),
        "LdcTag13.class": ("illegal tag 13", (ldc.method_type, ldc.ldc, 1)),
        "LdcTag14.class": ("illegal tag 14", (ldc.method_type, ldc.ldc, 1)),
        "LdcUnknownTag.class": ("illegal tag 19", (ldc.method_type, ldc.ldc, 1)),
        "LdcDynamicOldMajor.class": ("class file version", (ldc.null_constant, ldc.ldc, 1, 55)),
        "LdcDynamicNoBSM.class": ("missing BootstrapMethods", (ldc.null_constant, ldc.ldc, 1, 55)),
        "LdcDynamicBSMArgPastEnd.class": ("static argument index", (ldc.null_constant, ldc.ldc, 1, 55)),
        "LdcDynamicBSMIndexPastEnd.class": ("bootstrap method index", (ldc.null_constant, ldc.ldc, 1, 55)),
        "LdcDynamicDuplicateBSM.class": ("duplicate BootstrapMethods", (ldc.null_constant, ldc.ldc, 1, 55)),
    }
    for filename, repair in repairs.items():
        name, *given = ldc.FIXTURES[filename]
        axis, canonical = (None, None) if repair is None else repair
        CASES.append(
            (
                "ldc",
                filename,
                axis,
                lambda n=name, g=given: ldc.class_file(n, *g),
                lambda n=name, c=canonical, g=given: ldc.class_file(n, *(c if c else g)),
                lambda n=name, c=canonical, g=given: ldc.class_file(n, *(c if c else g)),
            )
        )


def cp_cases():
    # The defect is the tag byte of a trailing, unreferenced, payload-free entry. Tag 1 (Utf8) is
    # the legal shape in the same three bytes, so the repaired build is the canonical one.
    for filename, spec in cp.FIXTURES.items():
        CASES.append(
            (
                "cp",
                filename,
                f"tag {spec[1]}",
                lambda s=spec: cp.unreferenced_unknown_tag(*s),
                lambda s=spec: cp.unreferenced_unknown_tag(s[0], 1),
                lambda s=spec: cp.unreferenced_unknown_tag(s[0], 1),
            )
        )


indy_cases()
ldc_cases()
cp_cases()

rc = 0
checked = 0
no_defect = 0
print(f"{'fixture':38s} {'axis':28s} verdict")
print("-" * 82)
for directory, filename, axis, built, repaired, canonical in CASES:
    committed = (ROOT / "test-data" / directory / filename).read_bytes()
    if built() != committed:
        print(f"{filename:38s} {'-':28s} ★GENERATOR DRIFT — 커밋본을 재현하지 못한다")
        rc = max(rc, 2)
        continue
    if axis is None:
        print(f"{filename:38s} {'(no defect — valid file)':28s} n/a")
        no_defect += 1
        continue
    checked += 1
    if repaired() == canonical():
        print(f"{filename:38s} {axis:28s} single-defect")
    else:
        got, want = repaired(), canonical()
        differing = sum(1 for a, b in zip(got, want) if a != b) + abs(len(got) - len(want))
        print(f"{filename:38s} {axis:28s} ★MULTI-DEFECT — 수리 후에도 {differing} 바이트 남는다")
        rc = 1

print("-" * 82)
print(f"검사 {checked}건 · 결함 없음(유효 파일) {no_defect}건 · 합계 {len(CASES)}건 · rc={rc}")
sys.exit(rc)
