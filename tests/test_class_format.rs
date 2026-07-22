#![allow(dead_code)] // test_helper is shared with test_class.rs; not all helpers are used here

mod test_helper;

use std::{
    fs,
    path::{Path, PathBuf},
};

use test_helper::run_class;

// Fixtures are derived deterministically from the committed test_data/Hello.class
// by byte manipulation, so corruption scenarios stay reproducible without
// committing corrupted binaries.
fn fixture(name: &str, bytes: &[u8]) -> (PathBuf, PathBuf) {
    // relative path with trailing slash, like "./test_data/": classpath entries are
    // turned into URLs and joined with the class file name
    let dir = PathBuf::from(format!("./target/class_format_fixtures_{}/", std::process::id()));
    fs::create_dir_all(&dir).unwrap();

    let path = dir.join(name);
    fs::write(&path, bytes).unwrap();

    (dir, path)
}

fn hello_class() -> Vec<u8> {
    fs::read("test_data/Hello.class").unwrap()
}

#[tokio::test]
async fn test_truncated_class_raises_class_format_error() {
    let (dir, path) = fixture("TruncatedHello.class", &hello_class()[..60]);

    let err = run_class(&path, &[dir.as_path()], &[]).await.unwrap_err().to_string();
    assert!(err.contains("java.lang.ClassFormatError"), "expected ClassFormatError, got: {err}");
    assert!(err.contains("Truncated"), "expected truncation cause in message, got: {err}");
}

#[tokio::test]
async fn test_unsupported_constant_pool_tag_raises_class_format_error() {
    let mut bytes = hello_class();
    // offset 10 is the first constant pool tag; 10 (Methodref) in the committed fixture
    assert_eq!(bytes[10], 10, "test_data/Hello.class layout changed; adjust the mutation offset");
    bytes[10] = 18; // CONSTANT_InvokeDynamic, unsupported
    let (dir, path) = fixture("BadTagHello.class", &bytes);

    let err = run_class(&path, &[dir.as_path()], &[]).await.unwrap_err().to_string();
    assert!(err.contains("java.lang.ClassFormatError"), "expected ClassFormatError, got: {err}");
    assert!(err.contains("tag 18"), "expected offending tag in message, got: {err}");
}

#[tokio::test]
async fn test_bad_magic_raises_class_format_error() {
    let mut bytes = hello_class();
    bytes[0] = 0x00; // magic becomes 0x00FEBABE
    let (dir, path) = fixture("BadMagicHello.class", &bytes);

    let err = run_class(&path, &[dir.as_path()], &[]).await.unwrap_err().to_string();
    assert!(err.contains("java.lang.ClassFormatError"), "expected ClassFormatError, got: {err}");
    assert!(err.contains("magic"), "expected magic mismatch cause in message, got: {err}");
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
