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

#[tokio::test]
async fn test_unsupported_constant_pool_tag_raises_class_format_error() {
    // 13 and 14 are unassigned by JVMS 4.4; 19 (Module) is assigned but only legal inside a
    // module-info. Widening the parser to the method-handle tags (15..=18) must not have
    // widened it to "anything goes" — a file carrying a tag that cannot appear here is still
    // a corrupt file, not a file using a feature we have not implemented.
    for tag in [13u8, 14, 19] {
        let mut bytes = hello_class();
        // offset 10 is the first constant pool tag; 10 (Methodref) in the committed fixture
        assert_eq!(bytes[10], 10, "test-data/Hello.class layout changed; adjust the mutation offset");
        bytes[10] = tag;
        let (dir, path) = fixture(&format!("BadTag{tag}Hello.class"), &bytes);

        let err = run_class(&path, &[dir.as_path()], &[]).await.unwrap_err().to_string();
        assert!(
            err.contains("java.lang.ClassFormatError"),
            "tag {tag}: expected ClassFormatError, got: {err}"
        );
    }
}

// javac 9+ emits `invokedynamic` for something as ordinary as string `+`, so the constant
// pool tags it needs (15 MethodHandle, 18 InvokeDynamic) decide which of two very different
// sentences the user reads: "your file is broken" or "this runtime cannot do that yet".
// Before the tags were parsed this fixture died as `ClassFormatError: Invalid class file`.
#[tokio::test]
async fn test_invokedynamic_class_reports_unsupported_feature_not_malformed() {
    let path = Path::new("test-data/indy/StringConcat.class");

    let err = run_class(path, &[Path::new("./test-data/indy/")], &[]).await.unwrap_err().to_string();

    assert!(
        err.contains("java.lang.UnsupportedOperationException") && err.contains("invokedynamic"),
        "expected the unsupported-feature diagnosis, got: {err}"
    );
    assert!(
        !err.contains("ClassFormatError"),
        "a class javac emits for `a` + int is not malformed, got: {err}"
    );
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
