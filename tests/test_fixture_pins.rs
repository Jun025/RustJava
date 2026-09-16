//! Toolchain pin for the committed javac fixtures under `test-data/indy`.
//!
//! Nothing in this repository recompiles them: there is no `setup-java` step in
//! `.github/workflows/rust.yml` and `javac` is not on PATH on the machine the tests run on, so a
//! check cannot ask the compiler what it would produce. The only thing that can notice a
//! regeneration on a different toolchain is an assertion on the bytes that are actually committed.

use std::{collections::BTreeMap, ffi::OsStr, fs, path::Path};

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
/// (`classfile/src/constant_pool.rs`, `tests/test_class_format.rs`), which count the exact
/// constants javac put in them.
const PINNED_MAJOR: u16 = 65;
const PINNED_MINOR: u16 = 0;

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

        assert_eq!(
            (major, minor),
            (PINNED_MAJOR, PINNED_MINOR),
            "{} is class file {major}.{minor}, not the pinned {PINNED_MAJOR}.{PINNED_MINOR}. \
             Recompile it with: javac --release 21 -d test-data/indy test-data/src/indy/*.java",
            path.display()
        );

        checked.push(path);
    }

    // Without this the test is a green assertion about nothing the moment the directory moves or
    // the source-pairing rule stops matching anything.
    assert!(!checked.is_empty(), "no javac-compiled fixture found under test-data/indy");
}

/// Every committed fixture's class file version, frozen in `test-data/class-file-versions.txt`.
///
/// The pin above says what `test-data/indy` *should* be and why; this one says only that nothing
/// moves without someone writing it down. That is the weaker claim on purpose: the rest of
/// `test-data` was never compiled to one target — 52, 65, 66, 68 and 70 all appear — and unifying
/// it would mean recompiling, which changes bytes and can change what a fixture exercises.
///
/// All three directions are checked, because a pin that only compares the fixtures it already
/// knows about is opt-in, and opt-in pins rot: a fixture added tomorrow would simply not be
/// covered, silently.
#[test]
fn committed_fixtures_keep_their_recorded_class_file_version() {
    let table = fs::read_to_string("test-data/class-file-versions.txt").unwrap();
    let mut recorded = table
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            let (version, path) = line.split_once(' ').expect("each row is `<major>.<minor> <path>`");
            (path.to_owned(), version.to_owned())
        })
        .collect::<BTreeMap<_, _>>();
    assert!(!recorded.is_empty(), "the version table is empty; it should list every fixture");

    let mut unrecorded = Vec::new();
    let mut moved = Vec::new();
    for path in class_files(Path::new("test-data")) {
        let relative = path.strip_prefix("test-data").unwrap().to_string_lossy().into_owned();
        let bytes = fs::read(&path).unwrap();
        let actual = format!(
            "{}.{}",
            u16::from_be_bytes([bytes[6], bytes[7]]),
            u16::from_be_bytes([bytes[4], bytes[5]])
        );

        match recorded.remove(&relative) {
            None => unrecorded.push(relative),
            Some(expected) if expected != actual => moved.push(format!("{relative}: recorded {expected}, found {actual}")),
            Some(_) => {}
        }
    }

    assert!(
        moved.is_empty(),
        "class file version changed without the table changing with it: {moved:#?}\n\
         If the recompile was deliberate, run test-data/src/record-class-file-versions.py and \
         commit the table next to the fixture."
    );
    assert!(
        unrecorded.is_empty(),
        "fixtures with no recorded version: {unrecorded:#?}\n\
         Run test-data/src/record-class-file-versions.py to add them."
    );
    assert!(
        recorded.is_empty(),
        "the table records fixtures that are no longer here: {:#?}\n\
         Run test-data/src/record-class-file-versions.py to drop them.",
        recorded.keys().collect::<Vec<_>>()
    );
}

fn class_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            out.extend(class_files(&path));
        } else if path.extension() == Some(OsStr::new("class")) {
            out.push(path);
        }
    }
    out.sort();
    out
}
