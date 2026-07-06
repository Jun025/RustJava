use alloc::{format, string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.Byte
pub struct Byte;

impl Byte {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Byte",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(B)V", Self::init, Default::default()),
                JavaMethodProto::new("parseByte", "(Ljava/lang/String;)B", Self::parse_byte, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(B)Ljava/lang/Byte;", Self::value_of, MethodAccessFlags::STATIC),
                JavaMethodProto::new("byteValue", "()B", Self::byte_value, Default::default()),
                JavaMethodProto::new("shortValue", "()S", Self::short_value, Default::default()),
                JavaMethodProto::new("intValue", "()I", Self::int_value, Default::default()),
                JavaMethodProto::new("longValue", "()J", Self::long_value, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("toString", "(B)Ljava/lang/String;", Self::to_string_static, MethodAccessFlags::STATIC),
            ],
            fields: vec![JavaFieldProto::new("value", "B", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i8) -> Result<()> {
        tracing::debug!("java.lang.Byte::<init>({:?}, {:?})", &this, value);

        jvm.put_field(&mut this, "value", "B", value).await?;

        Ok(())
    }

    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        tracing::debug!("java.lang.Byte::byteValue({:?})", &this);

        jvm.get_field(&this, "value", "B").await
    }

    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        tracing::debug!("java.lang.Byte::shortValue({:?})", &this);

        let value: i8 = jvm.get_field(&this, "value", "B").await?;

        Ok(value as i16)
    }

    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.Byte::intValue({:?})", &this);

        let value: i8 = jvm.get_field(&this, "value", "B").await?;

        Ok(value as i32)
    }

    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.lang.Byte::longValue({:?})", &this);

        let value: i8 = jvm.get_field(&this, "value", "B").await?;

        Ok(value as i64)
    }

    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: i8) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Byte::valueOf({:?})", value);

        let instance = jvm.new_class("java/lang/Byte", "(B)V", (value,)).await?;

        Ok(instance.into())
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Byte::toString({:?})", &this);

        let value: i8 = jvm.get_field(&this, "value", "B").await?;

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;

        Ok(string.into())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i8) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Byte::toString({:?})", value);

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;

        Ok(string.into())
    }

    async fn parse_byte(jvm: &Jvm, _: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<i8> {
        tracing::debug!("java.lang.Byte::parseByte({:?})", &s);

        let s = JavaLangString::to_rust_string(jvm, &s).await?;

        // Java parses the decimal string as a signed byte (-128..127); unparseable or
        // out-of-range input throws NumberFormatException, not a VM abort.
        match s.parse::<i8>() {
            Ok(value) => Ok(value),
            Err(_) => Err(jvm
                .exception("java/lang/NumberFormatException", &format!("For input string: \"{s}\""))
                .await),
        }
    }
}
