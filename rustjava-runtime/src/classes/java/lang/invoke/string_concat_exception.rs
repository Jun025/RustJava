use alloc::vec;

use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::JavaMethodProto;
use jvm_types::{ClassAccessFlags, MethodAccessFlags};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.invoke.StringConcatException
// Only the constructor `Jvm::exception` calls: this class exists to be the cause of the
// BootstrapMethodError a mismatched string concat recipe raises (jvm-bytecode/src/interpreter.rs).
pub struct StringConcatException;

impl StringConcatException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/invoke/StringConcatException",
            parent_class: Some("java/lang/Exception"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "<init>",
                "(Ljava/lang/String;)V",
                Self::init_with_message,
                MethodAccessFlags::PUBLIC,
            )],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.invoke.StringConcatException::<init>({this:?}, {message:?})");

        jvm.invoke_special(&this, "java/lang/Exception", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
}
