use alloc::{boxed::Box, collections::BTreeMap, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_runtime::{RuntimeClassProto, RuntimeContext};
use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm};

struct TestClass;
impl TestClass {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TestClass",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("ran", "Z", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "ran", "Z", false).await?;

        Ok(())
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "ran", "Z", true).await?;

        Ok(())
    }
}

#[tokio::test]
async fn test_thread() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let class = Box::new(ClassDefinitionImpl::from_class_proto(
        TestClass::as_proto(),
        Box::new(runtime.clone()) as Box<_>,
    ));
    jvm.register_class(class, None).await?;

    let test_class = jvm.new_class("TestClass", "()V", ()).await?;

    let thread = jvm
        .new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (test_class.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&thread, "start", "()V", []).await?;

    let _: () = jvm.invoke_virtual(&thread, "join", "()V", []).await?;

    let ran: bool = jvm.get_field(&test_class, "ran", "Z").await?;
    assert!(ran);

    Ok(())
}

struct IdentityTestClass;
impl IdentityTestClass {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "IdentityTestClass",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("t1", "Ljava/lang/Thread;", Default::default()),
                JavaFieldProto::new("t2", "Ljava/lang/Thread;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let t1: ClassInstanceRef<Self> = jvm.invoke_static("java/lang/Thread", "currentThread", "()Ljava/lang/Thread;", ()).await?;
        let t2: ClassInstanceRef<Self> = jvm.invoke_static("java/lang/Thread", "currentThread", "()Ljava/lang/Thread;", ()).await?;

        jvm.put_field(&mut this, "t1", "Ljava/lang/Thread;", t1).await?;
        jvm.put_field(&mut this, "t2", "Ljava/lang/Thread;", t2).await?;

        Ok(())
    }
}

#[tokio::test]
async fn test_current_thread_identity() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let class = Box::new(ClassDefinitionImpl::from_class_proto(
        IdentityTestClass::as_proto(),
        Box::new(runtime.clone()) as Box<_>,
    ));
    jvm.register_class(class, None).await?;

    let test_class = jvm.new_class("IdentityTestClass", "()V", ()).await?;

    let thread = jvm
        .new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (test_class.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&thread, "start", "()V", []).await?;
    let _: () = jvm.invoke_virtual(&thread, "join", "()V", []).await?;

    // inside the spawned thread, currentThread() must be the started Thread itself,
    // and repeated calls must return the same reference
    let t1: ClassInstanceRef<IdentityTestClass> = jvm.get_field(&test_class, "t1", "Ljava/lang/Thread;").await?;
    let t2: ClassInstanceRef<IdentityTestClass> = jvm.get_field(&test_class, "t2", "Ljava/lang/Thread;").await?;
    let t1 = t1.instance.unwrap();
    let t2 = t2.instance.unwrap();
    assert!(t1.equals(&*t2)?);
    assert!(t1.equals(&*thread)?);

    // on the main task, currentThread() is stable across calls but is a different
    // thread than the spawned one
    let m1: ClassInstanceRef<IdentityTestClass> = jvm.invoke_static("java/lang/Thread", "currentThread", "()Ljava/lang/Thread;", ()).await?;
    let m2: ClassInstanceRef<IdentityTestClass> = jvm.invoke_static("java/lang/Thread", "currentThread", "()Ljava/lang/Thread;", ()).await?;
    let m1 = m1.instance.unwrap();
    let m2 = m2.instance.unwrap();
    assert!(m1.equals(&*m2)?);
    assert!(!m1.equals(&*thread)?);

    Ok(())
}
