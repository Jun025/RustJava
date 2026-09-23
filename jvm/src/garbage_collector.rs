use alloc::{boxed::Box, collections::BTreeMap, format, string::String, vec::Vec};
use jvm_types::FieldAccessFlags;

use hashbrown::{HashSet, hash_set::Entry};

use crate::{ClassDefinition, ClassInstance, Field, JavaError, JavaValue, Jvm, Result, class_loader::Class, thread::JvmThread};

/// The objects nothing reachable refers to.
///
/// The walk reads fields through the `ClassInstance`/`ClassDefinition` implementations, which may belong
/// to the host and may fail. A failure ends the walk with that error rather than a partial answer: an
/// object whose fields could not be read might hold the only reference to another, so reporting the rest
/// as garbage would free live objects. Nothing is destroyed before the walk returns, so an error leaves the
/// heap as it was.
pub fn determine_garbage(
    jvm: &Jvm,
    threads: &BTreeMap<u64, JvmThread>,
    global_references: &BTreeMap<u64, Box<dyn ClassInstance>>,
    all_class_instances: &HashSet<Box<dyn ClassInstance>>,
    classes: &BTreeMap<String, Class>,
    interned_strings: &[Box<dyn ClassInstance>],
) -> Result<Vec<Box<dyn ClassInstance>>> {
    let mut reachable_objects = HashSet::new();

    for class in classes.values() {
        find_reachable_objects(jvm, &class.java_class(), &mut reachable_objects)?;
        find_static_reachable_objects(jvm, class, &mut reachable_objects)?;
    }

    for object in threads
        .values()
        .flat_map(|thread| thread.iter_frame().flat_map(|stack| stack.local_variables()))
    {
        find_reachable_objects(jvm, object, &mut reachable_objects)?;
    }

    for object in threads.values().filter_map(|thread| thread.java_thread()) {
        find_reachable_objects(jvm, object, &mut reachable_objects)?;
    }

    for object in global_references.values() {
        find_reachable_objects(jvm, object, &mut reachable_objects)?;
    }

    for object in interned_strings {
        find_reachable_objects(jvm, object, &mut reachable_objects)?;
    }

    Ok(all_class_instances.difference(&reachable_objects).cloned().collect())
}

fn find_static_reachable_objects(jvm: &Jvm, class: &Class, reachable_objects: &mut HashSet<Box<dyn ClassInstance>>) -> Result<()> {
    let fields = find_all_fields(jvm, &*class.definition)?;
    for field in fields {
        if !field.access_flags().contains(FieldAccessFlags::STATIC) {
            continue;
        }

        let descriptor = field.descriptor();

        if (descriptor.starts_with('L') && descriptor.ends_with(';')) || descriptor.starts_with('[') {
            let value = class.definition.get_static_field(&*field)?;
            if let JavaValue::Object(Some(value)) = value {
                find_reachable_objects(jvm, &value, reachable_objects)?;
            }
        }
    }

    Ok(())
}

#[allow(clippy::borrowed_box)]
fn find_reachable_objects(jvm: &Jvm, object: &Box<dyn ClassInstance>, reachable_objects: &mut HashSet<Box<dyn ClassInstance>>) -> Result<()> {
    let entry = reachable_objects.entry(object.clone());
    if let Entry::Occupied(_) = entry {
        return Ok(());
    }
    entry.insert();

    let name = object.class_definition().name();
    if name.starts_with('[') {
        if name.starts_with("[L") || name.starts_with("[[") {
            // is object array
            let array = object
                .as_array_instance()
                .ok_or_else(|| JavaError::Unraisable(format!("{object:?} is named {name} but is not an array instance")))?;
            let values = array.load(0, array.length())?;

            for value in values {
                if let JavaValue::Object(Some(value)) = value {
                    find_reachable_objects(jvm, &value, reachable_objects)?;
                }
            }
        }
        // do nothing for primitive arrays
    } else {
        let fields = find_all_fields(jvm, &*object.class_definition())?;
        for field in fields {
            if field.access_flags().contains(FieldAccessFlags::STATIC) {
                continue;
            }

            let descriptor = field.descriptor();

            if (descriptor.starts_with('L') && descriptor.ends_with(';')) || descriptor.starts_with('[') {
                let value = object.get_field(&*field)?;
                if let JavaValue::Object(Some(value)) = value {
                    find_reachable_objects(jvm, &value, reachable_objects)?;
                }
            }
        }
    }

    Ok(())
}

fn find_all_fields(jvm: &Jvm, class_definition: &dyn ClassDefinition) -> Result<Vec<Box<dyn Field>>> {
    let result = class_definition.fields();
    let super_class_name = class_definition.super_class_name();

    if let Some(x) = super_class_name {
        let super_class = jvm
            .get_class(&x)
            .ok_or_else(|| JavaError::Unraisable(format!("{} has superclass {x}, which is not loaded", class_definition.name())))?;
        let super_fields = find_all_fields(jvm, &*super_class.definition)?;
        Ok(result.into_iter().chain(super_fields).collect())
    } else {
        Ok(result)
    }
}
