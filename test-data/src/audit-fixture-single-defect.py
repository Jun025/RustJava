#!/usr/bin/env python3
"""Audit the hand-assembled fixtures for **single-defectness**.

The mutation audits so far asked: does each test fail when the thing it names is broken? This asks
the other half — is each *fixture* broken in exactly one way? A fixture that is wrong twice keeps
its test passing after one of the two is fixed, and nobody notices that the other stopped being
covered. That is silent, and it is the failure this script exists to make loud.

## The test it applies

For a generated fixture the question has an exact answer, because the generator takes the defect as
a parameter: **repair the named defect and the bytes must equal the canonical build.**

    generator(**given)     ==  the committed fixture       (the generator still describes it)
    generator(**repaired)  ==  generator(**canonical)      (nothing else differs)

Byte equality is stronger than "the repaired file loads": a second defect that happens not to
matter to our loader would pass the loading test and fail this one. It is also cheaper — no JVM.

### ★ Where `repaired` comes from is the whole test

`repaired` is `given` — the committed fixture's own arguments — with **only the named axis**
overwritten. It is not rebuilt from the canonical arguments. The difference is not stylistic:

    repaired = dict(given, **{field: CANON[field]})   # a second defect survives the repair
    repaired = dict(CANON, name=...)                  # a second defect is thrown away → always green

The second form asks "is a fixture built from canonical arguments single-defect", which is
tautological. Gate 2 measured that: with the second form, 14 of 18 verdicts could not fail, and
second defects planted in the `ldc`, `cp` and `indy LINKED` families were all reported as
`single-defect`. `given` is bound to the generator's *named* parameters with
`inspect.signature(...).bind(*spec)`, so a parameter added to a generator tomorrow is carried into
`repaired` automatically rather than silently dropped.

The asymmetry is deliberate: `repaired` derives from `given`, `canonical` does not. A `canonical`
that also derived from `given` would carry the second defect too, and the two sides would match
again. `AUDIT SPEC STALE` below is the guard on that asymmetry.

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
* ★ **The canonical arguments and the named axis are written down here, not derived.** The script
  can prove that repairing the axis it was *told* about leaves nothing behind; it cannot know that
  the axis named is the one the fixture is for, or that a `CANON` table below is really canonical.
  Both are human annotations, and a wrong one produces a confident wrong verdict. The guard that
  exists is narrow and mechanical: if a spec binds a parameter no `CANON` table names, the fixture
  is reported `AUDIT SPEC STALE` (rc=2) instead of judged — so *growing* a generator cannot quietly
  re-open the hole, though *mis-labelling* an existing axis still can.

Run:  python3 test-data/src/audit-fixture-single-defect.py     (rc=0 clean, rc=1 a fixture is
      wrong in more than one way, rc=2 the audit cannot judge — the generator no longer reproduces
      a committed fixture, or a spec sets a parameter this script does not know the canonical of)
"""

import importlib.util
import inspect
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


def bound(generator, spec):
    """A fixture's positional spec as the generator's *named* parameters, defaults filled in.

    Naming them is what lets `repaired` be "given, with one field changed" rather than "canonical,
    rebuilt" — see the docstring. Going through the signature rather than indexing the tuple means
    a new parameter lands in `given` on its own.
    """
    arguments = inspect.signature(generator).bind(*spec)
    arguments.apply_defaults()
    return dict(arguments.arguments)


# (directory, filename, axis, generator, given, repaired, canonical)
# A `None` axis means the fixture carries no defect — it is a valid file, listed to keep the census
# complete rather than to be checked; `repaired`/`canonical` are `None` for those rows.
CASES = []


def add(directory, filename, generator, spec, axis_and_fields, canon):
    """Build the three argument sets for one fixture, and refuse to judge one we cannot."""
    given = bound(generator, spec)
    if axis_and_fields is None:
        CASES.append((directory, filename, None, generator, given, None, None))
        return

    axis, fields = axis_and_fields
    # `canonical` is `canon` — not derived from `given` — so it cannot inherit a second defect.
    # That only holds while `canon` names every axis the spec can set; otherwise the unnamed one
    # would fall through to the generator's default on both sides and cancel out.
    unknown = sorted(set(given) - {"name"} - set(canon))
    if unknown:
        CASES.append((directory, filename, f"★{','.join(unknown)}", generator, given, None, None))
        return
    CASES.append(
        (
            directory,
            filename,
            axis,
            generator,
            given,
            dict(given, **{field: canon[field] for field in fields}),
            dict(canon, name=given["name"]),
        )
    )


def indy_cases():
    canon = dict(
        bootstrap_class=indy.FACTORY_CLASS,
        bootstrap_name="makeConcatWithConstants",
        bootstrap_descriptor=indy.FACTORY_DESCRIPTOR,
        bootstrap_kind=6,
    )
    axes = {
        "NotStringConcatFactory.class": ("owning class", ("bootstrap_class",)),
        "NotMakeConcatWithConstants.class": ("method name", ("bootstrap_name",)),
        "NotFactoryDescriptor.class": ("descriptor", ("bootstrap_descriptor",)),
        "NotInvokeStaticFactory.class": ("reference kind", ("bootstrap_kind",)),
    }
    for filename, axis in axes.items():
        add("indy", filename, indy.near_miss_call_site, indy.FIXTURES[filename], axis, canon)

    # The recipe-free entry point. `MakeConcat` itself is the canonical — it is *linked*, not a near
    # miss — so it is the `no defect` row that the other two are measured against, and its arguments
    # are the canonical ones the other two are repaired towards.
    canon = dict(left="a", right="b", bootstrap_descriptor=None, static_arguments=0)
    axes = {
        "MakeConcat.class": None,
        "MakeConcatWrongDescriptor.class": ("descriptor", ("bootstrap_descriptor",)),
        "MakeConcatWithArgument.class": ("static argument", ("static_arguments",)),
    }
    for filename, axis in axes.items():
        add("indy", filename, indy.make_concat_call_site, indy.LINKED[filename], axis, canon)


def ldc_cases():
    # Two canonical shapes: the `ldc`-of-a-legal-constant one, and the condy one that needs major 55
    # for the tag to be legal at all. `None` marks a valid file (the feature is unimplemented, which
    # is not a defect in the fixture).
    tag_shape = dict(build_constant=ldc.method_type, code=ldc.ldc, max_stack=1, major=52)
    condy_shape = dict(build_constant=ldc.null_constant, code=ldc.ldc, max_stack=1, major=55)
    repairs = {
        "LdcMethodHandle.class": (None, tag_shape),
        "LdcMethodType.class": (None, tag_shape),
        "LdcDynamic.class": (None, condy_shape),
        "Ldc2WDynamic.class": (None, condy_shape),
        # Repairing the width means repairing the stack it reserves with it: `ldc2_w` pushes a
        # two-slot value. One axis, two parameters.
        "Ldc2WMethodType.class": (("ldc2_w width", ("code", "max_stack")), tag_shape),
        "LdcTag13.class": (("illegal tag 13", ("build_constant",)), tag_shape),
        "LdcTag14.class": (("illegal tag 14", ("build_constant",)), tag_shape),
        "LdcUnknownTag.class": (("illegal tag 19", ("build_constant",)), tag_shape),
        "LdcDynamicOldMajor.class": (("class file version", ("major",)), condy_shape),
        "LdcDynamicNoBSM.class": (("missing BootstrapMethods", ("build_constant",)), condy_shape),
        "LdcDynamicBSMArgPastEnd.class": (("static argument index", ("build_constant",)), condy_shape),
        "LdcDynamicBSMIndexPastEnd.class": (("bootstrap method index", ("build_constant",)), condy_shape),
        "LdcDynamicDuplicateBSM.class": (("duplicate BootstrapMethods", ("build_constant",)), condy_shape),
    }
    for filename, (axis, canon) in repairs.items():
        add("ldc", filename, ldc.class_file, ldc.FIXTURES[filename], axis, canon)


def cp_cases():
    # The defect is the tag byte of a trailing, unreferenced, payload-free entry. Tag 1 (Utf8) is
    # the legal shape in the same three bytes, so the repaired build is the canonical one.
    canon = dict(tag=1)
    for filename, spec in cp.FIXTURES.items():
        add("cp", filename, cp.unreferenced_unknown_tag, spec, (f"tag {spec[1]}", ("tag",)), canon)


indy_cases()
ldc_cases()
cp_cases()

rc = 0
checked = 0
no_defect = 0
print(f"{'fixture':38s} {'axis':28s} verdict")
print("-" * 82)
for directory, filename, axis, generator, given, repaired, canonical in CASES:
    committed = (ROOT / "test-data" / directory / filename).read_bytes()
    if generator(**given) != committed:
        print(f"{filename:38s} {'-':28s} ★GENERATOR DRIFT — 커밋본을 재현하지 못한다")
        rc = max(rc, 2)
        continue
    if axis is None:
        print(f"{filename:38s} {'(no defect — valid file)':28s} n/a")
        no_defect += 1
        continue
    if repaired is None:
        print(f"{filename:38s} {axis:28s} ★AUDIT SPEC STALE — 이 인자의 정본을 모른다")
        rc = max(rc, 2)
        continue
    checked += 1
    got, want = generator(**repaired), generator(**canonical)
    if got == want:
        print(f"{filename:38s} {axis:28s} single-defect")
    else:
        # A byte count, not an edit distance: the two builds can differ in length, and then `zip`
        # walks them out of step and the tail counts as differing even where it is the same bytes
        # shifted. It is a "how loud", not a measurement — the verdict rests on `got != want`.
        differing = sum(1 for a, b in zip(got, want) if a != b) + abs(len(got) - len(want))
        print(f"{filename:38s} {axis:28s} ★MULTI-DEFECT — 수리 후에도 {differing} 바이트 남는다")
        rc = max(rc, 1)

print("-" * 82)
print(f"검사 {checked}건 · 결함 없음(유효 파일) {no_defect}건 · 합계 {len(CASES)}건 · rc={rc}")
sys.exit(rc)
