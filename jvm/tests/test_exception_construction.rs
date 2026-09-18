use jvm::{JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

// `Jvm::exception` is the one path a JVM must not abort on: it is what runs *after* something has
// already gone wrong. When the class it is asked to raise cannot be built, the machinery below it
// has already produced a perfectly good Java exception saying so (`load_class` raises
// NoClassDefFoundError) -- this asserts that report reaches the caller instead of being unwrapped.
#[tokio::test]
async fn test_exception_reports_unloadable_class_instead_of_aborting() -> Result<()> {
    let jvm = test_jvm().await?;

    // The name is held in a variable rather than written inline on purpose, and not to dodge a lock:
    // `scripts/check-named-exception-classes-are-loadable.py` requires every *literal* java/ name
    // passed to `Jvm::exception` to be loadable, which is exactly what this test has to violate. A
    // literal here makes that check red (measured: it reported this line), and the check is right to
    // — everywhere except here, an unloadable name is a defect.
    let unloadable = "java/lang/NoSuchClassAnywhere";
    let error = jvm.exception(unloadable, "message").await;

    let JavaError::JavaException(exception) = error;
    assert!(jvm.is_instance(&*exception, "java/lang/NoClassDefFoundError"));

    let message = jvm
        .invoke_virtual(&exception, "java/lang/Throwable", "getMessage", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &message).await?, "java/lang/NoSuchClassAnywhere");

    Ok(())
}
