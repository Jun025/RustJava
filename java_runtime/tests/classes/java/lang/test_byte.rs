use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use java_runtime::classes::java::lang::Byte;

use test_utils::test_jvm;

#[tokio::test]
async fn test_parse_byte() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "42").await?;
    assert_eq!(
        42i8,
        jvm.invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;)B", (string,))
            .await?
    );

    // negative, in-range
    let string = JavaLangString::from_rust_string(&jvm, "-128").await?;
    assert_eq!(
        -128i8,
        jvm.invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;)B", (string,))
            .await?
    );

    // out of byte range -> NumberFormatException
    let string = JavaLangString::from_rust_string(&jvm, "200").await?;
    let r: Result<i8> = jvm.invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;)B", (string,)).await;
    assert!(r.is_err());

    // non-numeric -> NumberFormatException
    let string = JavaLangString::from_rust_string(&jvm, "x").await?;
    let r: Result<i8> = jvm.invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;)B", (string,)).await;
    assert!(r.is_err());

    Ok(())
}

#[tokio::test]
async fn test_byte_box_unbox() -> Result<()> {
    let jvm = test_jvm().await?;

    let b: ClassInstanceRef<Byte> = jvm.invoke_static("java/lang/Byte", "valueOf", "(B)Ljava/lang/Byte;", (7i8,)).await?;

    assert_eq!(7i8, jvm.invoke_virtual(&b, "byteValue", "()B", ()).await?);
    assert_eq!(7i32, jvm.invoke_virtual(&b, "intValue", "()I", ()).await?);
    assert_eq!(7i64, jvm.invoke_virtual(&b, "longValue", "()J", ()).await?);

    let s: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm.invoke_virtual(&b, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!("7", JavaLangString::to_rust_string(&jvm, &s).await?);

    Ok(())
}
