//! Toolchain pin for the committed javac fixtures under `test-data/indy`.
//!
//! Nothing in this repository recompiles them: there is no `setup-java` step in
//! `.github/workflows/rust.yml` and `javac` is not on PATH on the machine the tests run on, so a
//! check cannot ask the compiler what it would produce. The only thing that can notice a
//! regeneration on a different toolchain is an assertion on the bytes that are actually committed.

use std::{ffi::OsStr, fs, path::Path};

/// Fixtures under `test-data/indy` that have a `.java` source next to them were produced with:
///
/// ```text
/// javac --release 21 -d test-data/indy test-data/src/indy/<Name>.java     # OpenJDK 26.0.1 javac
/// ```
///
/// `--release 21` is the part the bytes can prove: it fixes the class file version at 65.0. A bare
/// `javac` stamps the running JDK's version instead (26 emits 70.0), which is the accident this
/// catches — "whatever javac happened to be installed" is how these files got here in the first
/// place. It does *not* pin the compiler binary; two javac versions at `--release 21` both emit
/// 65.0. That second axis is held by the constant-shape assertions on the same fixtures
/// (`classfile/src/constant_pool.rs`, `classfile/tests/test.rs`, `tests/test_class_format.rs`),
/// which pin the exact constants javac put in them.
///
/// The line above used to be the whole record, and a record is only as good as whoever wrote it.
/// It is not just a record any more: javac is deterministic for the same source, flags and
/// compiler, so rebuilding is a check.
///
/// ```text
/// sh test-data/src/verify-javac-fixtures.sh            # needs a JDK; CI has none, so it is manual
/// ```
///
/// Measured 2026-09-17 with `javac 26.0.1`: all six class files here rebuild **byte for byte**.
/// So the compiler recorded above is the compiler that produced them, verified rather than
/// asserted. Re-run it after regenerating a fixture; a mismatch means the toolchain moved.
const PINNED_MAJOR: u16 = 65;
const PINNED_MINOR: u16 = 0;

/// One fixture is compiled at a different release, and it is pinned rather than exempted.
///
/// `LambdaCapturingThis` exists to carry a `REF_invokeSpecial` bootstrap argument, which javac
/// stopped emitting at Java 11 (nestmates, JEP 181) — `--release 8` produces kind 7 for that
/// source and `--release 21` produces kind 5, so the *release* is the fixture's whole point. Its
/// source says so too. Recording the pin here keeps the promise identical for it: regenerating it
/// on the wrong release is still caught.
fn pinned_version_of(stem: &str) -> (u16, u16, &'static str) {
    match stem {
        "LambdaCapturingThis" => (52, 0, "8"),
        _ => (PINNED_MAJOR, PINNED_MINOR, "21"),
    }
}

/// Synthetic fixtures — the ones a generator script writes byte by byte — are deliberately not
/// pinned here: they are not javac output and pick their own version (`NotStringConcatFactory`
/// targets 52.0). "Has a `.java` source" is the structural way to tell the two apart, and it
/// extends by itself: drop `Foo.java` beside `Foo.class` and the pin covers `Foo` too.
fn javac_source_of(class_file: &Path) -> Option<std::path::PathBuf> {
    let stem = class_file.file_stem()?.to_str()?;
    // `ConstantKinds$Op.class` is compiled from `ConstantKinds.java`.
    let outer = stem.split('$').next()?;
    let source = Path::new("test-data/src/indy").join(format!("{outer}.java"));

    source.exists().then_some(source)
}

#[test]
fn indy_javac_fixtures_keep_the_pinned_class_file_version() {
    let mut checked = Vec::new();

    for entry in fs::read_dir("test-data/indy").unwrap() {
        let path = entry.unwrap().path();
        if path.extension() != Some(OsStr::new("class")) || javac_source_of(&path).is_none() {
            continue;
        }

        let bytes = fs::read(&path).unwrap();
        let major = u16::from_be_bytes([bytes[6], bytes[7]]);
        let minor = u16::from_be_bytes([bytes[4], bytes[5]]);

        // `Outer$Inner.class` is compiled from `Outer.java`, so it inherits the outer name's pin.
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let (pinned_major, pinned_minor, release) = pinned_version_of(stem.split('$').next().unwrap());

        assert_eq!(
            (major, minor),
            (pinned_major, pinned_minor),
            "{} is class file {major}.{minor}, not the pinned {pinned_major}.{pinned_minor}. \
             Recompile it with: javac --release {release} -d test-data/indy test-data/src/indy/{}.java",
            path.display(),
            stem.split('$').next().unwrap()
        );

        checked.push(path);
    }

    // Without this the test is a green assertion about nothing the moment the directory moves or
    // the source-pairing rule stops matching anything.
    assert!(!checked.is_empty(), "no javac-compiled fixture found under test-data/indy");
}
