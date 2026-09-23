use core::hash::{Hash, Hasher};

use jvm::{ClassDefinition, ClassInstance, ClassInstanceRef, Field, JavaError, JavaValue, Result};

use test_utils::test_jvm;

/// A host object whose fields cannot be read -- what wie's `ClassInstance` impl reports when the guest
/// hands it a word that does not decode to a live object.
#[derive(Clone, Debug)]
struct Unreadable(Box<dyn ClassInstance>);

impl ClassInstance for Unreadable {
    fn destroy(self: Box<Self>) {}

    fn identity(&self) -> usize {
        self.0.identity()
    }

    fn shallow_clone(&self) -> Result<Box<dyn ClassInstance>> {
        Ok(Box::new(self.clone()))
    }

    fn class_definition(&self) -> Box<dyn ClassDefinition> {
        self.0.class_definition()
    }

    fn equals(&self, other: &dyn ClassInstance) -> Result<bool> {
        Ok(other
            .as_any()
            .downcast_ref::<Self>()
            .is_some_and(|other| other.identity() == self.identity()))
    }

    fn get_field(&self, field: &dyn Field) -> Result<JavaValue> {
        Err(JavaError::Unraisable(format!("cannot read {} of a dead guest object", field.name())))
    }

    fn put_field(&mut self, _: &dyn Field, _: JavaValue) -> Result<()> {
        unreachable!()
    }
}

impl Hash for Unreadable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // distinct from the wrapped instance, which is reachable on its own
        (self.identity(), "Unreadable").hash(state);
    }
}

// `find_reachable_objects` used to `.unwrap()` every field read, so a host `ClassInstance` that could
// not read a field took the process down from inside `Jvm::collect_garbage` -- which already returns
// `Result`. The error now reaches the caller unchanged, and nothing is freed on the way out.
#[tokio::test]
async fn a_field_the_host_cannot_read_fails_the_collection_instead_of_panicking() -> Result<()> {
    let jvm = test_jvm().await?;

    jvm.push_native_frame();
    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;
    jvm.collect_garbage()?;

    let unreadable = jvm.new_global_ref(&ClassInstanceRef::<()>::new(Some(Box::new(Unreadable(vector.clone())))));

    match jvm.collect_garbage() {
        Err(JavaError::Unraisable(message)) => assert!(message.contains("of a dead guest object"), "{message}"),
        other => panic!("expected the host error back, got {other:?}"),
    }

    // The failed walk freed nothing: the heap is collectable again once the object is gone.
    drop(unreadable);
    assert_eq!(jvm.collect_garbage()?, 0);
    let size: i32 = jvm.invoke_virtual(&vector, "java/util/Vector", "size", "()I", ()).await?;
    assert_eq!(size, 0);

    Ok(())
}
