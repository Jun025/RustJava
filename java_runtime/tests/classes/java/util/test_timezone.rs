use java_runtime::classes::java::lang::{Object, String};
use java_runtime::classes::java::util::TimeZone;
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_timezone() -> Result<()> {
    let jvm = test_jvm().await?;

    let id = JavaLangString::from_rust_string(&jvm, "UTC").await?;
    let timezone: ClassInstanceRef<TimeZone> = jvm
        .invoke_static("java/util/TimeZone", "getTimeZone", "(Ljava/lang/String;)Ljava/util/TimeZone;", (id,))
        .await?;

    assert!(!timezone.is_null());

    Ok(())
}

#[tokio::test]
async fn test_timezone_available_ids() -> Result<()> {
    let jvm = test_jvm().await?;

    let ids: ClassInstanceRef<Object> = jvm
        .invoke_static("java/util/TimeZone", "getAvailableIDs", "()[Ljava/lang/String;", ())
        .await?;
    assert!(!ids.is_null());

    let len = jvm.array_length(&ids).await?;
    assert!(len >= 1, "getAvailableIDs must return a non-empty set");

    // every returned id must be resolvable by getTimeZone (consistency)
    let elems: std::vec::Vec<ClassInstanceRef<String>> = jvm.load_array(&ids, 0, len).await?;
    for id in elems {
        let tz: ClassInstanceRef<TimeZone> = jvm
            .invoke_static("java/util/TimeZone", "getTimeZone", "(Ljava/lang/String;)Ljava/util/TimeZone;", (id,))
            .await?;
        assert!(!tz.is_null());
    }

    Ok(())
}
