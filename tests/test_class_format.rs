#![allow(dead_code)] // test_helper is shared with test_class.rs; not all helpers are used here

mod test_helper;

use std::{
    fs,
    path::{Path, PathBuf},
};

use test_helper::run_class;

// Fixtures are derived deterministically from the committed test-data/Hello.class
// by byte manipulation, so corruption scenarios stay reproducible without
// committing corrupted binaries.
fn fixture(name: &str, bytes: &[u8]) -> (PathBuf, PathBuf) {
    // relative path with trailing slash, like "./test-data/": classpath entries are
    // turned into URLs and joined with the class file name
    let dir = PathBuf::from(format!("./target/class_format_fixtures_{}/", std::process::id()));
    fs::create_dir_all(&dir).unwrap();

    let path = dir.join(name);
    fs::write(&path, bytes).unwrap();

    (dir, path)
}

fn hello_class() -> Vec<u8> {
    fs::read("test-data/Hello.class").unwrap()
}

// This note used to say only the exception *kind* could be asserted, because `InvalidFormat` carried
// no cause and both places that turn it into a Java exception hardcoded "Invalid class file". That is
// done: `InvalidFormat(&'static str)` threads the cause to the boundary, and `validate_class` is one
// `if` per rule so the cause can differ. `test_a_rejected_class_says_why` below is what makes that
// visible; the kind-only assertions elsewhere in this file are left as they are, because the kind is
// what those tests are about.
//
// (The note before *that* one said the variants were "cut" upstream at 822504b and that restoring them
// "needs upstream variants". Both halves were wrong, measured: that commit *created*
// classfile/src/error.rs — before it, `ClassInfo::parse` returned `Option`, so failure carried nothing
// at all — and this fork already diverges by hundreds of lines in this crate.)
#[tokio::test]
async fn test_truncated_class_raises_class_format_error() {
    let (dir, path) = fixture("TruncatedHello.class", &hello_class()[..60]);

    let err = run_class(&path, &[dir.as_path()], &[]).await.unwrap_err().to_string();
    assert!(err.contains("java.lang.ClassFormatError"), "expected ClassFormatError, got: {err}");
}

// 13 and 14 are unassigned by JVMS 4.4; 19 (Module) is assigned but only legal inside a
// module-info. Widening the parser to the method-handle tags (15..=18) must not have widened it
// to "anything goes" — a file carrying a tag that cannot appear here is still a corrupt file, not
// a file using a feature we have not implemented.
//
// The fixtures carry that tag on a **trailing, unreferenced, payload-free** pool entry, so the
// unknown tag is the only thing wrong with the file. That is what makes this test able to fail:
// the previous version overwrote the tag of `Hello.class`'s first entry, which is a Methodref the
// code invokes, so the file broke along several paths at once. `ClassFileError` flattened every
// parse failure into "Invalid class file" back then, so that assertion could not tell "rejected because the
// tag is unknown" from "rejected because the class fell apart" — and measurably did not: with the
// tag switch's pass-through branch mutated from reject to accept, it still passed.
// See `test-data/src/cp/make_cp_fixtures.py`.
#[tokio::test]
async fn test_unsupported_constant_pool_tag_raises_class_format_error() {
    for tag in [13u8, 14, 19] {
        let path = PathBuf::from(format!("test-data/cp/UnreferencedTag{tag}.class"));

        let err = run_class(&path, &[Path::new("./test-data/cp/")], &[])
            .await
            .expect_err("a tag that cannot appear in a class file must be rejected")
            .to_string();
        assert!(
            err.contains("java.lang.ClassFormatError"),
            "tag {tag}: expected ClassFormatError, got: {err}"
        );
    }
}

// Exactly one `invokedynamic` bootstrap is linked: `StringConcatFactory.makeConcatWithConstants`,
// which is what javac 9+ lowers string `+` to. Every other bootstrap is still refused, and this
// asserts both halves — because a change that linked *everything* would satisfy the first half
// alone and read identically from the outside.
//
// The linked half is asserted here as "it loads and runs"; what it actually prints is compared
// against `test-data/StringConcat.txt` by `tests/test_class.rs`, which is where output belongs.
#[tokio::test]
async fn test_only_the_string_concat_bootstrap_is_linked() {
    let indy = Path::new("./test-data/indy/");

    run_class(Path::new("test-data/indy/StringConcat.class"), &[indy], &[])
        .await
        .expect("javac's string `+` call site should link and run");

    // LambdaMetafactory (Lambda) and ConstantBootstraps/condy (ConstantKinds) are not linked.
    for name in ["Lambda", "ConstantKinds", "NotStringConcatFactory"] {
        let path = PathBuf::from(format!("test-data/indy/{name}.class"));

        let err = run_class(&path, &[indy], &[]).await.unwrap_err().to_string();

        assert!(
            err.contains("java.lang.UnsupportedOperationException") && err.contains("invokedynamic"),
            "{name}: a bootstrap we do not link must stay refused, got: {err}"
        );
        assert!(
            !err.contains("ClassFormatError"),
            "{name}: refusing to link is not the same as calling the file broken, got: {err}"
        );
    }
}

// The factory's other entry point. `makeConcat` takes no static arguments — the call site
// descriptor alone says what to concatenate — so the recipe is synthesised from its arity rather
// than read from the bootstrap.
//
// The fixture prints its result, and this asserts *that* rather than only that the class ran: a
// recipe synthesised at the wrong length still links and still runs, and would concatenate the
// wrong number of arguments unnoticed if the value were discarded.
//
// javac does not emit this shape — targets 9 through 26 all emit `makeConcatWithConstants`, even
// for `a + b` with no literal text — so the fixture is hand-assembled
// (test-data/src/indy/make_indy_fixtures.py).
#[tokio::test]
async fn test_the_recipe_free_factory_concatenates_every_argument() {
    let path = Path::new("test-data/indy/MakeConcat.class");

    let output = run_class(path, &[Path::new("./test-data/indy/")], &[])
        .await
        .expect("a makeConcat call site should link and run");

    assert_eq!(output.trim_end(), "ab", "both arguments must be concatenated, in order");
}

// The identity check above is four comparisons, and the test above can only observe one of them.
// `NotStringConcatFactory` differs in the owning class, so deleting *that* comparison links it and
// the test fails — but deleting any of the other three changes nothing any fixture can see. Measured
// before these fixtures existed: with the kind, name or descriptor comparison removed one at a
// time, `cargo test --all` stayed at 570 passed / 0 failed.
//
// So each axis gets a fixture that differs in that axis alone. Every one of them is a valid class
// file that reaches the identity check — nothing upstream can reject them — which is what makes
// each comparison observable rather than merely present.
#[tokio::test]
async fn test_each_axis_of_the_factory_identity_is_observable() {
    for (name, axis) in [
        ("NotStringConcatFactory", "owning class"),
        ("NotMakeConcatWithConstants", "method name"),
        ("NotFactoryDescriptor", "descriptor"),
        ("NotInvokeStaticFactory", "reference kind"),
        // The same axis for the recipe-free entry point. It carries no static arguments, so the
        // argument-count guard cannot refuse it and the descriptor comparison is all that is left
        // — which is exactly what makes that comparison observable.
        ("MakeConcatWrongDescriptor", "descriptor, on the recipe-free entry point"),
        // And the other half of that entry point's rule: it takes no static arguments, so one that
        // carries a static argument is not the shape it claims to be.
        ("MakeConcatWithArgument", "a static argument the recipe-free entry point does not take"),
    ] {
        let path = PathBuf::from(format!("test-data/indy/{name}.class"));

        let err = run_class(&path, &[Path::new("./test-data/indy/")], &[]).await.unwrap_err().to_string();

        assert!(
            err.contains("java.lang.UnsupportedOperationException") && err.contains("invokedynamic"),
            "{name}: a bootstrap differing in {axis} must not be linked, got: {err}"
        );
        assert!(
            !err.contains("ClassFormatError"),
            "{name}: it has to reach the identity check, so it must be a readable file, got: {err}"
        );
    }
}

// A bootstrap that *is* the factory, but whose recipe contradicts the call site it was linked to.
// javac cannot produce this — the recipe and the descriptor are two accounts of the same
// concatenation, written by the same compiler — so the fixtures are hand-assembled
// (test-data/src/indy/make_indy_fixtures.py).
//
// What the diagnosis should be was measured rather than chosen: OpenJDK 26 refuses all three with
// `BootstrapMethodError` caused by `StringConcatException`, at linkage. So it is neither a
// `ClassFormatError` (the class file format has nothing to say about bootstrap argument semantics,
// and the file parses) nor `UnsupportedOperationException` (the bootstrap *is* linked; the file is
// what is wrong).
//
// `RecipeWantsFewerArguments` is the direction a guard that fires when the recipe runs out of
// arguments cannot see: before this check, it concatenated the arguments the recipe did ask for
// and printed a quietly wrong "a" instead of refusing.
#[tokio::test]
async fn test_a_recipe_that_contradicts_its_call_site_is_a_bootstrap_method_error() {
    for (name, disagreement) in [
        ("RecipeWantsMoreArguments", "recipe wants two arguments, the call site provides one"),
        ("RecipeWantsFewerArguments", "recipe wants one argument, the call site provides two"),
        ("RecipeWantsAConstant", "recipe wants a constant the bootstrap did not carry"),
    ] {
        let path = PathBuf::from(format!("test-data/indy/{name}.class"));

        let err = run_class(&path, &[Path::new("./test-data/indy/")], &[]).await.unwrap_err().to_string();

        assert!(
            err.contains("java.lang.BootstrapMethodError"),
            "{name} ({disagreement}): expected the linkage diagnosis, got: {err}"
        );
        assert!(
            !err.contains("ClassFormatError") && !err.contains("UnsupportedOperationException"),
            "{name}: the file parses and the bootstrap is one we link, got: {err}"
        );
    }
}

#[tokio::test]
async fn test_bad_magic_raises_class_format_error() {
    let mut bytes = hello_class();
    bytes[0] = 0x00; // magic becomes 0x00FEBABE
    let (dir, path) = fixture("BadMagicHello.class", &bytes);

    let err = run_class(&path, &[dir.as_path()], &[]).await.unwrap_err().to_string();
    assert!(err.contains("java.lang.ClassFormatError"), "expected ClassFormatError, got: {err}");
}

#[tokio::test]
async fn test_missing_class_still_raises_no_class_def_found_error() {
    let (dir, _) = fixture("Unrelated.class", &hello_class());

    let err = run_class(Path::new("NoSuchClass.class"), &[dir.as_path()], &[])
        .await
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("java.lang.NoClassDefFoundError"),
        "expected NoClassDefFoundError, got: {err}"
    );
    assert!(
        !err.contains("ClassFormatError"),
        "not-found must stay distinct from unreadable, got: {err}"
    );
}

// `StringConcat.class` above only exercises tags 15 and 18. This fixture is the other two read
// from real javac output rather than hand-assembled bytes: tag 16 (MethodType) from a lambda's
// bootstrap arguments, and tag 17 (Dynamic) from a switch with qualified enum constant labels,
// which is rare enough that exactly one class in OpenJDK 26's own 27,902 carries it.
// `constant_pool.rs` asserts the fixture really does carry both; this asserts what it does.
#[tokio::test]
async fn test_class_carrying_every_method_handle_family_tag_is_unsupported_not_malformed() {
    let path = Path::new("test-data/indy/ConstantKinds.class");

    let err = run_class(path, &[Path::new("./test-data/indy/")], &[]).await.unwrap_err().to_string();

    assert!(
        err.contains("java.lang.UnsupportedOperationException") && err.contains("invokedynamic"),
        "expected the unsupported-feature diagnosis, got: {err}"
    );
    assert!(
        !err.contains("ClassFormatError"),
        "a class OpenJDK 26 runs to completion is not malformed, got: {err}"
    );
}

// Synthetic, by necessity: javac has no source construct that makes `ldc` name a
// CONSTANT_MethodHandle/MethodType/Dynamic entry — measured over the 27,902 javac-compiled
// classes of the JDK's own jmods (1.27M ldc sites, zero hits) and over targeted sources at
// --release 21/25/26; see docs/worklog/2026-09-16-ldc-tags-15-16-17.md. So these fixtures are
// assembled byte by byte (test-data/src/ldc/make_ldc_fixtures.py). OpenJDK 26 loads and runs
// all four without complaint, which is the whole point: files this runtime cannot *do*, not
// files it cannot *read*.
#[tokio::test]
async fn test_ldc_of_method_handle_family_reports_unsupported_feature_not_malformed() {
    for (name, feature) in [
        ("LdcMethodHandle", "ldc of a method handle"),
        ("LdcMethodType", "ldc of a method type"),
        ("LdcDynamic", "ldc of a dynamically-computed constant"),
        ("Ldc2WDynamic", "ldc of a dynamically-computed constant"),
    ] {
        let path = PathBuf::from(format!("test-data/ldc/{name}.class"));

        let err = run_class(&path, &[Path::new("./test-data/ldc/")], &[]).await.unwrap_err().to_string();

        assert!(
            err.contains("java.lang.UnsupportedOperationException") && err.contains(feature),
            "{name}: expected the unsupported-feature diagnosis, got: {err}"
        );
        assert!(
            !err.contains("ClassFormatError"),
            "{name}: a file OpenJDK loads is not malformed, got: {err}"
        );
    }
}

// The other direction. Widening `ldc` must not have widened it to "anything the constant pool
// holds": a wide `ldc2_w` still takes only wide constants, and a constant pool tag that cannot
// appear in a class file at all is still a corrupt file. Without this, replacing the ldc arms
// with a constant `Ok` would pass the test above.
#[tokio::test]
async fn test_ldc_of_an_illegal_constant_is_still_malformed() {
    for name in ["Ldc2WMethodType", "LdcTag13", "LdcTag14", "LdcUnknownTag", "LdcDynamicOldMajor"] {
        let path = PathBuf::from(format!("test-data/ldc/{name}.class"));

        let err = run_class(&path, &[Path::new("./test-data/ldc/")], &[]).await.unwrap_err().to_string();

        assert!(
            err.contains("java.lang.ClassFormatError"),
            "{name}: expected ClassFormatError, got: {err}"
        );
    }
}

// The version rule is a four-row table (JVMS 4.4), and the reviewer's case was one row. The
// others are reached by lowering the version of a class that already carries the tag, which is
// this file's existing idiom and costs no new binary — including one real javac class, so the
// rule is not only exercised against fixtures we assembled ourselves.
#[tokio::test]
async fn test_a_constant_tag_below_its_minimum_class_file_version_is_malformed() {
    // (fixture, tags it carries, a major version that predates them)
    for (source, tags, major) in [
        ("test-data/ldc/LdcMethodHandle.class", "15", 50u16),
        ("test-data/ldc/LdcMethodType.class", "16", 50),
        ("test-data/indy/StringConcat.class", "15 and 18", 50),
        ("test-data/ldc/LdcDynamic.class", "17", 54),
    ] {
        let mut bytes = fs::read(source).unwrap();
        bytes[6..8].copy_from_slice(&major.to_be_bytes());
        // a distinct name per case: one shared name would let a stale file pass for the next
        let stem = source.rsplit('/').next().unwrap().trim_end_matches(".class");
        let (dir, path) = fixture(&format!("OldMajor{stem}.class"), &bytes);

        let err = run_class(&path, &[dir.as_path()], &[]).await.unwrap_err().to_string();

        assert!(
            err.contains("java.lang.ClassFormatError"),
            "tag {tags} at major {major}: expected ClassFormatError, got: {err}"
        );
    }
}

// A bootstrap method's static arguments are pool indices too (JVMS 4.7.23), and nothing used to
// check they name anything. The one place that reads them — the StringConcatFactory linker — treats
// a dangling index as "not a shape I can link" and declines, so the file went on to be reported as
// an unsupported feature. It is not: no JVM can read it.
//
// This is a bounds check and not resolution. The fixture's argument is 0xFFFF, past the end of a
// 19-entry pool, so it fails on being absent rather than on being the wrong kind of constant.
#[tokio::test]
async fn test_a_bootstrap_method_argument_naming_nothing_is_malformed() {
    let path = Path::new("test-data/ldc/LdcDynamicBSMArgPastEnd.class");

    let err = run_class(path, &[Path::new("./test-data/ldc/")], &[]).await.unwrap_err().to_string();

    assert!(err.contains("java.lang.ClassFormatError"), "expected ClassFormatError, got: {err}");
    assert!(
        !err.contains("UnsupportedOperationException"),
        "an argument index naming nothing is a broken file, not an unsupported feature, got: {err}"
    );
}

// The band that used to be left open, now closed — this is the flipped assertion the previous
// round asked for by name. A Dynamic entry has to name a real bootstrap method (JVMS 4.4.10,
// 4.7.23), and it can fail either way: the attribute absent, or the index past the end of a table
// that is present. Both were answered "this runtime does not support that yet" about files
// OpenJDK 26 rejects outright, which is the one sentence this lineage exists to keep honest.
#[tokio::test]
async fn test_a_dynamic_constant_naming_a_missing_bootstrap_method_is_malformed() {
    for (name, how) in [
        ("LdcDynamicNoBSM", "no BootstrapMethods attribute at all"),
        ("LdcDynamicBSMIndexPastEnd", "index 1 into a one-entry table"),
    ] {
        let path = PathBuf::from(format!("test-data/ldc/{name}.class"));

        let err = run_class(&path, &[Path::new("./test-data/ldc/")], &[]).await.unwrap_err().to_string();

        assert!(
            err.contains("java.lang.ClassFormatError"),
            "{name} ({how}): expected ClassFormatError, got: {err}"
        );
        assert!(
            !err.contains("UnsupportedOperationException"),
            "{name} ({how}): a file no JVM can read is not merely unsupported, got: {err}"
        );
    }
}

// The third way that rule can break, and the one that has no right answer to fall back on: JVMS
// 4.7.23 allows at most one BootstrapMethods attribute, and the resolver takes the first it finds.
// With two tables that choice is arbitrary — the index gets bounded against whichever came first
// while the other is ignored — so the file has to be refused rather than read one way or the other.
//
// The fixture carries the same valid table twice, so being rejected is *only* attributable to there
// being two of them: either table alone makes a file that loads.
#[tokio::test]
async fn test_a_class_declaring_bootstrap_methods_twice_is_malformed() {
    let path = Path::new("test-data/ldc/LdcDynamicDuplicateBSM.class");

    let err = run_class(path, &[Path::new("./test-data/ldc/")], &[]).await.unwrap_err().to_string();

    assert!(err.contains("java.lang.ClassFormatError"), "expected ClassFormatError, got: {err}");
    assert!(
        !err.contains("UnsupportedOperationException"),
        "two bootstrap tables is a broken file, not an unsupported feature, got: {err}"
    );
}

// The same sentence, for the harder shape. A lambda's `BootstrapMethods` entry carries
// MethodType and MethodHandle constants as static arguments, which nothing here can resolve —
// so parsing the attribute is exactly where a lambda class could start being called corrupt
// again. The classfile-level test asserts the indices survive; this asserts what the user reads.
#[tokio::test]
async fn test_lambda_class_reports_unsupported_feature_not_malformed() {
    let path = Path::new("test-data/indy/Lambda.class");

    let err = run_class(path, &[Path::new("./test-data/indy/")], &[]).await.unwrap_err().to_string();

    assert!(
        err.contains("java.lang.UnsupportedOperationException") && err.contains("invokedynamic"),
        "expected the unsupported-feature diagnosis, got: {err}"
    );
    assert!(
        !err.contains("ClassFormatError"),
        "a class javac emits for `x -> x + 1` is not malformed, got: {err}"
    );
}

// The point of threading a cause: two broken files get two different sentences, and each one names
// the rule that fired. Before this, every one of them read "Invalid class file".
//
// Asserted at the boundary a user actually sees — the `ClassFormatError` message — rather than on
// `ClassFileError`, because the value of the change is that the string survives three layers
// (classfile -> jvm-bytecode -> runtime) instead of being dropped by the `From` impl.
#[tokio::test]
async fn test_a_rejected_class_says_why() {
    for (fixture, directory, cause) in [
        (
            "test-data/ldc/LdcDynamicDuplicateBSM.class",
            "./test-data/ldc/",
            "multiple BootstrapMethods attributes",
        ),
        (
            "test-data/ldc/LdcDynamicOldMajor.class",
            "./test-data/ldc/",
            "class file version does not support a constant tag it carries",
        ),
        (
            "test-data/ldc/LdcDynamicBSMArgPastEnd.class",
            "./test-data/ldc/",
            "a bootstrap method argument names nothing or is not a loadable constant",
        ),
    ] {
        let err = run_class(Path::new(fixture), &[Path::new(directory)], &[]).await.unwrap_err().to_string();

        assert!(
            err.contains("java.lang.ClassFormatError"),
            "{fixture}: expected ClassFormatError, got: {err}"
        );
        assert!(err.contains(cause), "{fixture}: expected the message to say {cause:?}, got: {err}");
    }
}
