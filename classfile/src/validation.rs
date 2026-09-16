use alloc::collections::BTreeMap;

use jvm_types::MethodAccessFlags;

use crate::{AttributeInfo, ClassFileError, ClassInfo, ConstantPoolReference, constant_pool::ConstantPoolItem};

enum MemberKind {
    Field,
    Method,
}

pub(crate) fn validate_class(class: &ClassInfo) -> Result<(), ClassFileError> {
    if !is_internal_class_name(&class.this_class)
        || class.super_class.as_ref().is_some_and(|name| !is_internal_class_name(name))
        || class.interfaces.iter().any(|name| !is_internal_class_name(name))
        || !validate_constant_pool(&class.constant_pool)
        || !constant_pool_tags_fit_the_class_file_version(class)
        || !bootstrap_method_indices_resolve(class)
    {
        return Err(ClassFileError::InvalidFormat);
    }

    for field in &class.fields {
        if !is_field_descriptor(&field.descriptor) {
            return Err(ClassFileError::InvalidFormat);
        }

        let constant_values = field
            .attributes
            .iter()
            .filter_map(|attribute| match attribute {
                AttributeInfo::ConstantValue(value) => Some(value),
                _ => None,
            })
            .collect::<alloc::vec::Vec<_>>();
        if constant_values.len() > 1
            || constant_values.first().is_some_and(|value| {
                !matches!(
                    (field.descriptor.as_str(), *value),
                    ("Z" | "B" | "C" | "S" | "I", ConstantPoolReference::Integer(_))
                        | ("J", ConstantPoolReference::Long(_))
                        | ("F", ConstantPoolReference::Float(_))
                        | ("D", ConstantPoolReference::Double(_))
                        | ("Ljava/lang/String;", ConstantPoolReference::String(_))
                )
            })
        {
            return Err(ClassFileError::InvalidFormat);
        }
    }

    for method in &class.methods {
        if !is_method_descriptor(&method.descriptor) {
            return Err(ClassFileError::InvalidFormat);
        }

        let code_attributes = method
            .attributes
            .iter()
            .filter(|attribute| matches!(attribute, AttributeInfo::Code(_)))
            .count();
        if method.access_flags.intersects(MethodAccessFlags::ABSTRACT | MethodAccessFlags::NATIVE) {
            if code_attributes != 0 {
                return Err(ClassFileError::InvalidFormat);
            }
        } else if code_attributes != 1 {
            return Err(ClassFileError::InvalidFormat);
        }
    }

    Ok(())
}

/// JVMS 4.4: a constant kind is legal only from the class file version that introduced it.
///
/// This lives here rather than in `validate_constant_pool` because it needs `major_version`,
/// which is on `ClassInfo` and not in the pool.
///
/// It exists because widening what `ldc` accepts took away an accidental backstop. Before the
/// method-handle family resolved at all, `from_constant_pool` returned `None` for these tags and
/// the `ldc` arm turned that into a parse failure — so a file carrying one at an impossible
/// version was reported as corrupt, which is what OpenJDK says too. It was reported as corrupt
/// for the wrong reason, but it *was* reported. Widening the arms removed that by-product with
/// nothing in its place, and the class went from "corrupt" to "this runtime does not support
/// that yet" — a sentence this lineage exists to make true, applied to a file no JVM can read.
/// `test-data/ldc/LdcDynamicOldMajor.class` holds that case.
fn constant_pool_tags_fit_the_class_file_version(class: &ClassInfo) -> bool {
    class.constant_pool.values().all(|item| {
        let minimum_major_version = match item {
            // Java 7 (JSR 292) introduced the method handle family.
            ConstantPoolItem::MethodHandle { .. } | ConstantPoolItem::MethodType { .. } | ConstantPoolItem::InvokeDynamic { .. } => 51,
            // Java 11 (JEP 309) added dynamically-computed constants.
            ConstantPoolItem::Dynamic { .. } => 55,
            _ => return true,
        };

        class.major_version >= minimum_major_version
    })
}

/// JVMS 4.4.10 and 4.7.23: `bootstrap_method_attr_index` is an index into the `bootstrap_methods`
/// array of the `BootstrapMethods` attribute, and that attribute must be present whenever the pool
/// holds a Dynamic or InvokeDynamic entry. Both halves are the same sentence — the index has to
/// name a real entry — so they are one predicate rather than two: an absent attribute is a table of
/// no entries, which no index can name.
///
/// This lives in `validate_class` rather than `validate_constant_pool` because it is the only place
/// that holds both the pool and the class attributes. That crossing is the whole reason the check
/// did not exist before; the note it replaces said as much and left it to the round that wanted it.
///
/// What this rejects was already being rejected by every real JVM — OpenJDK 26 answers
/// `ClassFormatError: Missing BootstrapMethods attribute` for the absent case. What we answered
/// was `UnsupportedOperationException`, i.e. *this runtime cannot do that yet*, about a file no
/// runtime can read. That sentence is what the lineage exists to make true, so narrowing it here
/// is the point rather than a side effect.
fn bootstrap_method_indices_resolve(class: &ClassInfo) -> bool {
    let bootstrap_method_count = class.attributes.iter().find_map(|attribute| match attribute {
        AttributeInfo::BootstrapMethods(methods) => Some(methods.len()),
        _ => None,
    });

    class.constant_pool.values().all(|item| {
        let index = match item {
            ConstantPoolItem::Dynamic {
                bootstrap_method_attr_index, ..
            }
            | ConstantPoolItem::InvokeDynamic {
                bootstrap_method_attr_index, ..
            } => *bootstrap_method_attr_index,
            _ => return true,
        };

        bootstrap_method_count.is_some_and(|count| (index as usize) < count)
    })
}

fn validate_constant_pool(constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> bool {
    constant_pool.values().all(|item| match item {
        ConstantPoolItem::Class { name_index } => constant_pool
            .get(name_index)
            .and_then(ConstantPoolItem::utf8)
            .is_some_and(|name| is_class_constant_name(&name)),
        ConstantPoolItem::String { string_index } => constant_pool.get(string_index).and_then(ConstantPoolItem::utf8).is_some(),
        ConstantPoolItem::Fieldref {
            class_index,
            name_and_type_index,
        } => validate_member_reference(constant_pool, *class_index, *name_and_type_index, MemberKind::Field),
        ConstantPoolItem::Methodref {
            class_index,
            name_and_type_index,
        }
        | ConstantPoolItem::InterfaceMethodref {
            class_index,
            name_and_type_index,
        } => validate_member_reference(constant_pool, *class_index, *name_and_type_index, MemberKind::Method),
        ConstantPoolItem::NameAndType {
            name_index,
            descriptor_index,
        } => {
            let name = constant_pool.get(name_index).and_then(ConstantPoolItem::utf8);
            let descriptor = constant_pool.get(descriptor_index).and_then(ConstantPoolItem::utf8);
            name.is_some_and(|name| !name.is_empty())
                && descriptor.is_some_and(|descriptor| is_field_descriptor(&descriptor) || is_method_descriptor(&descriptor))
        }
        // JVMS 4.4.8: the reference kind selects which kind of member reference is legal.
        ConstantPoolItem::MethodHandle {
            reference_kind,
            reference_index,
        } => matches!(
            (reference_kind, constant_pool.get(reference_index)),
            (1..=4, Some(ConstantPoolItem::Fieldref { .. }))
                | (
                    5..=8,
                    Some(ConstantPoolItem::Methodref { .. } | ConstantPoolItem::InterfaceMethodref { .. })
                )
                | (9, Some(ConstantPoolItem::InterfaceMethodref { .. }))
        ),
        ConstantPoolItem::MethodType { descriptor_index } => constant_pool
            .get(descriptor_index)
            .and_then(ConstantPoolItem::utf8)
            .is_some_and(|descriptor| is_method_descriptor(&descriptor)),
        // The bootstrap method index is bounded by `bootstrap_method_indices_resolve`, not here:
        // it needs the class attributes, and this function only gets the pool. What is left for
        // this arm is the half that the pool alone can answer.
        ConstantPoolItem::Dynamic { name_and_type_index, .. } | ConstantPoolItem::InvokeDynamic { name_and_type_index, .. } => {
            constant_pool.get(name_and_type_index).and_then(ConstantPoolItem::name_and_type).is_some()
        }
        _ => true,
    })
}

fn validate_member_reference(constant_pool: &BTreeMap<u16, ConstantPoolItem>, class_index: u16, name_and_type_index: u16, kind: MemberKind) -> bool {
    let class_name = constant_pool
        .get(&class_index)
        .and_then(ConstantPoolItem::class_name_index)
        .and_then(|index| constant_pool.get(&index))
        .and_then(ConstantPoolItem::utf8);
    let name_and_type = constant_pool.get(&name_and_type_index).and_then(ConstantPoolItem::name_and_type);
    let Some((name_index, descriptor_index)) = name_and_type else {
        return false;
    };
    let name = constant_pool.get(&name_index).and_then(ConstantPoolItem::utf8);
    let descriptor = constant_pool.get(&descriptor_index).and_then(ConstantPoolItem::utf8);

    class_name.is_some_and(|name| is_class_constant_name(&name))
        && name.is_some_and(|name| !name.is_empty())
        && descriptor.is_some_and(|descriptor| match kind {
            MemberKind::Field => is_field_descriptor(&descriptor),
            MemberKind::Method => is_method_descriptor(&descriptor),
        })
}

fn is_internal_class_name(name: &str) -> bool {
    !name.is_empty() && !name.starts_with('[') && !name.contains(['.', ';', '['])
}

fn is_class_constant_name(name: &str) -> bool {
    is_internal_class_name(name) || array_dimensions(name).is_some()
}

fn is_field_descriptor(descriptor: &str) -> bool {
    let mut cursor = 0;
    parse_field_type(descriptor.as_bytes(), &mut cursor) && cursor == descriptor.len()
}

fn is_method_descriptor(descriptor: &str) -> bool {
    let bytes = descriptor.as_bytes();
    if bytes.first() != Some(&b'(') {
        return false;
    }

    let mut cursor = 1;
    while bytes.get(cursor).is_some_and(|byte| *byte != b')') {
        if !parse_field_type(bytes, &mut cursor) {
            return false;
        }
    }
    if bytes.get(cursor) != Some(&b')') {
        return false;
    }
    cursor += 1;

    if bytes.get(cursor) == Some(&b'V') {
        cursor += 1;
    } else if !parse_field_type(bytes, &mut cursor) {
        return false;
    }

    cursor == bytes.len()
}

fn array_dimensions(descriptor: &str) -> Option<usize> {
    let bytes = descriptor.as_bytes();
    let dimensions = bytes.iter().take_while(|byte| **byte == b'[').count();
    if dimensions == 0 || dimensions > u8::MAX as usize {
        return None;
    }

    let mut cursor = 0;
    if parse_field_type(bytes, &mut cursor) && cursor == bytes.len() {
        Some(dimensions)
    } else {
        None
    }
}

fn parse_field_type(bytes: &[u8], cursor: &mut usize) -> bool {
    let mut dimensions = 0;
    while bytes.get(*cursor) == Some(&b'[') {
        dimensions += 1;
        if dimensions > u8::MAX as usize {
            return false;
        }
        *cursor += 1;
    }

    match bytes.get(*cursor) {
        Some(b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z') => {
            *cursor += 1;
            true
        }
        Some(b'L') => {
            let name_start = *cursor + 1;
            let Some(relative_end) = bytes[name_start..].iter().position(|byte| *byte == b';') else {
                return false;
            };
            let name_end = name_start + relative_end;
            if name_end == name_start || bytes[name_start..name_end].iter().any(|byte| matches!(byte, b'.' | b'[' | b';')) {
                return false;
            }
            *cursor = name_end + 1;
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{array_dimensions, is_field_descriptor, is_method_descriptor};

    #[test]
    fn validates_field_and_method_descriptors() {
        assert!(is_field_descriptor("Ljava/lang/String;"));
        assert!(is_field_descriptor("[[I"));
        assert!(!is_field_descriptor("V"));
        assert!(!is_field_descriptor("[V"));
        assert!(!is_field_descriptor("Igarbage"));

        assert!(is_method_descriptor("([Ljava/lang/String;I)V"));
        assert!(!is_method_descriptor("(V)V"));
        assert!(!is_method_descriptor("(I"));
        assert!(!is_method_descriptor("()"));
    }

    #[test]
    fn counts_valid_array_dimensions() {
        assert_eq!(array_dimensions("[[Ljava/lang/String;"), Some(2));
        assert_eq!(array_dimensions("java/lang/String"), None);
        assert_eq!(array_dimensions("[V"), None);
    }
}
