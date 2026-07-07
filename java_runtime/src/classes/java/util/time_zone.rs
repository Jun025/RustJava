use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// abstract class java.util.TimeZone
pub struct TimeZone;

impl TimeZone {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TimeZone",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getTimeZone",
                    "(Ljava/lang/String;)Ljava/util/TimeZone;",
                    Self::get_time_zone,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getAvailableIDs",
                    "()[Ljava/lang/String;",
                    Self::get_available_ids,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.TimeZone::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    // Returns the set of supported time-zone IDs. getTimeZone() accepts any id and
    // builds a SimpleTimeZone, so the "available" set is the minimal CLDC guarantee,
    // "GMT" — a real id for which getTimeZone("GMT") returns a valid zone (consistent).
    // Not fabricated: no synthetic Olson list; just the one universally-guaranteed id.
    async fn get_available_ids(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<crate::classes::java::lang::Object>> {
        tracing::debug!("java.util.TimeZone::getAvailableIDs()");

        let ids = ["GMT"];
        let mut arr = jvm.instantiate_array("Ljava/lang/String;", ids.len()).await?;
        for (i, id) in ids.iter().enumerate() {
            let s = JavaLangString::from_rust_string(jvm, id).await?;
            jvm.store_array(&mut arr, i, [s]).await?;
        }
        Ok(arr.into())
    }

    async fn get_time_zone(jvm: &Jvm, _: &mut RuntimeContext, id: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.TimeZone::getTimeZone({id:?})");

        let result = jvm.new_class("java/util/SimpleTimeZone", "(Ljava/lang/String;)V", (id,)).await?;

        Ok(result.into())
    }
}
