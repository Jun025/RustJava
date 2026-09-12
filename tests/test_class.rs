mod test_helper;

use std::{fs, path::Path};

use jvm::Result;

use test_helper::{run_class, run_jar};

// TODO parameterized tests..
//
// Some fixtures write scratch files into the working directory under a fixed name
// (NullFileIoGuards creates and deletes `null-file-io-guards.tmp`). That is only safe
// because exactly one *active* test function iterates `test-data/` at a time:
//
//   * this function is the only `#[test]` in this binary, so its fixtures run in sequence;
//   * `tests/test_real_jvm.rs` walks the same directory but is `#[ignore]`d;
//   * `cargo test` runs test binaries one after another, not concurrently.
//
// Break any one of those and two iterations can race on the same scratch path: the loser
// sees a half-written or already-deleted file and the failure looks like a runtime bug in
// whichever fixture happens to lose. Parallelising this (adding a second test function
// here, un-ignoring test_real_jvm, or moving to a per-test-process runner) therefore has a
// prerequisite — give each fixture its own working directory first.
#[tokio::test]
async fn test_class() -> Result<()> {
    let base_path = Path::new("test-data");

    let paths = fs::read_dir(base_path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let extension = path.extension();
        if let Some(x) = extension {
            if x != "class" && x != "jar" {
                continue;
            }
        } else {
            continue;
        }

        let name = path.file_stem().unwrap().to_str().unwrap();
        if name.contains('$') {
            continue;
        }

        let expected_path = base_path.join(format!("{name}.txt"));
        let expected = fs::read_to_string(expected_path).unwrap();

        let result = if extension.unwrap().to_str().unwrap() == "jar" {
            run_jar(&path, &[]).await
        } else {
            run_class(&path, &[Path::new("./test-data/")], &[]).await
        };

        if let Err(err) = result {
            panic!("Test {name} failed with error: {err}");
        } else {
            assert_eq!(result.as_ref().unwrap().clone(), expected, "Test {} failed: {}", name, result.unwrap());
        }
    }

    Ok(())
}
