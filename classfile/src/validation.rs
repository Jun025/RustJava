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
        || !bootstrap_method_static_arguments_are_in_the_pool(class)
        || !bootstrap_method_indices_resolve(class)
        || !at_most_one_of_each_single_class_attribute(class)
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

/// JVMS 4.7.23: every entry of a bootstrap method's `bootstrap_arguments` is an index into the
/// constant pool, so each one has to name an entry that is there.
///
/// ★ This is a bounds check and must stay one. `BootstrapMethod::arguments` is deliberately kept as
/// raw indices — `attribute.rs` says why at length: *resolving* them would turn every class holding
/// a lambda from "unsupported" back into "corrupt", because the kinds that turn up there are method
/// types and handles this crate has no payload for. Asking "is this index in the pool?" is not that
/// question. It reads no entry, needs no payload, and cannot fail for a class whose arguments point
/// somewhere real — which is every class any compiler emits.
///
/// The entry also has to be a **loadable constant** (JVMS 4.7.23, and the loadable column of the
/// 4.4 table: Integer, Float, Long, Double, Class, String, MethodHandle, MethodType, Dynamic).
/// A file whose argument names a Utf8 or a Methodref is rejected here.
///
/// ★ That is still not resolution, and the distinction is the whole reason this is safe. Reading a
/// *tag* asks which variant the entry is; it never looks inside one. `attribute.rs` keeps the
/// arguments unresolved because a lambda's are method handles and types this crate has no payload
/// for — and this check is exactly what does not need that payload. Measured rather than argued:
/// with the rule in place, every committed fixture still parses, lambdas included (144 parse / 12
/// fail, unchanged before and after).
///
/// Why it is worth having: without it, a malformed file is not diagnosed here but *later*, where
/// `ConstantPoolReference::from_constant_pool` returns `None` and the linker declines to link it —
/// which this runtime reports as `UnsupportedOperationException`. That says "we do not support
/// this file" about a file that is simply broken, and keeping those two apart is a line this
/// repository has drawn repeatedly. OpenJDK 26 agrees it is the file:
/// `ClassFormatError: argument_index 4 has bad constant type in class file StringConcat`.
///
/// One consequence worth naming: a long or double occupies two pool slots and only the first is
/// usable (JVMS 4.4.5), so the second has no entry in this map and an argument naming it is
/// rejected. That is the intended reading of "valid index" rather than an accident of the map.
fn bootstrap_method_static_arguments_are_in_the_pool(class: &ClassInfo) -> bool {
    class.attributes.iter().all(|attribute| match attribute {
        AttributeInfo::BootstrapMethods(methods) => methods.iter().all(|method| {
            method.arguments.iter().all(|index| {
                class.constant_pool.get(index).is_some_and(|item| {
                    matches!(
                        item,
                        ConstantPoolItem::Integer(_)
                            | ConstantPoolItem::Float(_)
                            | ConstantPoolItem::Long(_)
                            | ConstantPoolItem::Double(_)
                            | ConstantPoolItem::Class { .. }
                            | ConstantPoolItem::String { .. }
                            | ConstantPoolItem::MethodHandle { .. }
                            | ConstantPoolItem::MethodType { .. }
                            | ConstantPoolItem::Dynamic { .. }
                    )
                })
            })
        }),
        _ => true,
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

/// JVMS 4.7: several ClassFile attributes may appear at most once. A file carrying two is broken,
/// not a file using a feature this runtime lacks.
///
/// Kept separate from `bootstrap_method_indices_resolve` because it is a different sentence: that
/// one asks whether an index names a real entry, this one asks how many tables exist. Counting is
/// the same shape `validate_class` already uses for the per-member "at most one" rules
/// (`ConstantValue` on a field, `Code` on a method).
///
/// `BootstrapMethods` came first, because a duplicate there makes an arbitrary choice observable:
/// `bootstrap_method_indices_resolve` and `string_concat`'s `resolve_bootstrap_methods` both take
/// the table with `find_map`, which stops at the first one. That argument covers one attribute, and
/// the round that added it left the others as an open question.
///
/// ## Which others, and why not "all of them"
///
/// The list below is **what OpenJDK 26.0.1 actually rejects**, measured one attribute at a time
/// rather than read off the spec, because the two do not agree:
///
/// | duplicated attribute | class file version | OpenJDK 26.0.1 |
/// |---|---|---|
/// | `SourceFile`, `InnerClasses`, `SourceDebugExtension`, `BootstrapMethods` | 52 | `ClassFormatError: Multiple … attributes` |
/// | `NestHost`, `NestMembers` | 55 | `ClassFormatError: Multiple … attributes` |
/// | `NestHost` | **52** | **loads** — the attribute is not defined before 55, so it is ignored (JVMS 4.7.1) |
/// | `Synthetic` | 52 | **loads**, though JVMS 4.7.8 marks it at-most-one |
///
/// Two of those rows are the whole reason this is a table and not a `matches!` over every variant:
///
/// * **The version gate is load-bearing.** Counting `NestHost` at major 52 would reject a file every
///   real JVM accepts. An attribute that does not exist at that version is not duplicated, it is
///   unrecognised, and unrecognised attributes are ignored.
/// * **`Synthetic` is excluded on evidence, not oversight.** The spec says at most one; HotSpot
///   takes two. Rejecting it would make us stricter than the JVM we are trying to agree with, and
///   nothing here reads the attribute, so there is no arbitrary choice to make observable either.
///
/// The other direction is deliberate too: attributes that belong to a field, method or Code table
/// (`ConstantValue`, `Code`, `Exceptions`, `MethodParameters`, `StackMapTable`, `LineNumberTable`,
/// `LocalVariableTable`) are not listed even when they turn up in a class's attribute table, because
/// there they are attributes in a place they are not defined for — ignored, not counted.
fn at_most_one_of_each_single_class_attribute(class: &ClassInfo) -> bool {
    // (discriminant, the class file version that introduced the attribute)
    fn single_valued(attribute: &AttributeInfo) -> Option<(u8, u16)> {
        Some(match attribute {
            AttributeInfo::SourceFile(_) => (0, 45),
            AttributeInfo::InnerClasses(_) => (1, 45),
            AttributeInfo::SourceDebugExtension => (2, 49),
            AttributeInfo::BootstrapMethods(_) => (3, 51),
            AttributeInfo::NestHost(_) => (4, 55),
            AttributeInfo::NestMembers(_) => (5, 55),
            _ => return None,
        })
    }

    class.attributes.iter().enumerate().all(|(position, attribute)| {
        let Some((kind, introduced_in)) = single_valued(attribute) else {
            return true;
        };
        if class.major_version < introduced_in {
            return true;
        }

        // Only the first of each kind looks behind it, so one duplicate is reported once.
        !class.attributes[..position]
            .iter()
            .any(|earlier| single_valued(earlier).is_some_and(|(earlier_kind, _)| earlier_kind == kind))
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
