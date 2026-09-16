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

// Only the exception *kind* is asserted, not the message: upstream `ClassFileError`
// (cut 822504b) collapses every parse failure into a flat "Invalid class file",
// so per-cause wording is no longer available. Restoring it needs upstream variants.
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
// code invokes, so the file broke along several paths at once. `ClassFileError` flattens every
// parse failure into "Invalid class file", so that assertion could not tell "rejected because the
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
