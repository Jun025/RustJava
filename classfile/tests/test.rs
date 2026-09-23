use std::collections::BTreeMap;

use jvm_types::ClassAccessFlags;

use classfile::{AttributeInfo, BootstrapMethod, ClassFileError, ClassInfo, ConstantPoolReference, Location, MethodHandleKind, Opcode};

#[test]
fn test_hello() {
    let hello = include_bytes!("../../test-data/Hello.class");

    let class = ClassInfo::parse(hello).unwrap();

    assert_eq!(class.magic, 0xCAFEBABE);
    assert_eq!(class.major_version, 65);
    assert_eq!(class.minor_version, 0);
    assert_eq!(class.constant_pool.len(), 28);
    assert!(class.access_flags == ClassAccessFlags::SUPER);
    assert_eq!(class.this_class, "Hello".to_string().into());
    assert_eq!(class.super_class, Some("java/lang/Object".to_string().into()));
    assert_eq!(class.interfaces.len(), 0);
    assert_eq!(class.fields.len(), 0);
    assert_eq!(class.methods.len(), 2);
    assert_eq!(class.attributes.len(), 1);

    assert_eq!(class.methods[0].name, "<init>".to_string().into());
    assert_eq!(class.methods[0].descriptor, "()V".to_string().into());
    assert!(matches!(class.methods[0].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(x) = &class.methods[0].attributes[0] {
        assert_eq!(x.code.len(), 3);
        assert!(matches!(x.code.get(&0).unwrap(), Opcode::Aload(0)));
        assert!(matches!(x.code.get(&1).unwrap(),
            Opcode::Invokespecial(
                ConstantPoolReference::Method(x)) if x.class == "java/lang/Object".to_string().into() && x.name == "<init>".to_string().into() && x.descriptor == "()V".to_string().into()));
        assert!(matches!(x.code.get(&4).unwrap(), Opcode::Return));
    } else {
        panic!("Expected code attribute");
    }

    assert_eq!(class.methods[1].name, "main".to_string().into());
    assert_eq!(class.methods[1].descriptor, "([Ljava/lang/String;)V".to_string().into());
    assert!(matches!(class.methods[1].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(x) = &class.methods[1].attributes[0] {
        assert_eq!(x.code.len(), 4);
        assert!(matches!(x.code.get(&0).unwrap(),
            Opcode::Getstatic(ConstantPoolReference::Field(x)) if x.class == "java/lang/System".to_string().into() && x.name == "out".to_string().into() && x.descriptor == "Ljava/io/PrintStream;".to_string().into()));
        assert!(matches!(x.code.get(&3).unwrap(),
            Opcode::Ldc(x) if matches!(x, ConstantPoolReference::String(y) if *y == "Hello, world!".to_string().into())));
        assert!(matches!(x.code.get(&5).unwrap(),
            Opcode::Invokevirtual(ConstantPoolReference::Method(x)) if x.class == "java/io/PrintStream".to_string().into() && x.name == "println".to_string().into() && x.descriptor == "(Ljava/lang/String;)V".to_string().into()));
        assert!(matches!(x.code.get(&8).unwrap(), Opcode::Return));
    } else {
        panic!("Expected code attribute");
    }

    assert!(matches!(class.attributes[0], AttributeInfo::SourceFile { .. }));
}

#[test]
fn test_odd_even() {
    let odd_even = include_bytes!("../../test-data/OddEven.class");

    let class = ClassInfo::parse(odd_even).unwrap();

    assert_eq!(class.methods[2].name, "run".to_string().into());
    assert!(matches!(class.methods[2].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(code_attribute) = &class.methods[2].attributes[0] {
        assert!(matches!(code_attribute.attributes[0], AttributeInfo::LineNumberTable { .. }));
        assert!(matches!(code_attribute.attributes[1], AttributeInfo::LocalVariableTable { .. }));
        assert!(matches!(code_attribute.attributes[2], AttributeInfo::StackMapTable { .. }));

        if let AttributeInfo::LocalVariableTable(local_variable_table) = &code_attribute.attributes[1] {
            assert_eq!(local_variable_table.len(), 3);
            assert_eq!(local_variable_table[0].name, "this".to_string().into());
            assert_eq!(local_variable_table[0].descriptor, "LOddEven;".to_string().into());
            assert_eq!(local_variable_table[0].index, 0);
            assert_eq!(local_variable_table[1].name, "arg".to_string().into());
            assert_eq!(local_variable_table[1].descriptor, "Ljava/lang/String;".to_string().into());
            assert_eq!(local_variable_table[1].index, 1);
            assert_eq!(local_variable_table[2].name, "i".to_string().into());
            assert_eq!(local_variable_table[2].descriptor, "I".to_string().into());
            assert_eq!(local_variable_table[2].index, 2);
        }
    }
}

#[test]
fn test_superclass() {
    let super_class = include_bytes!("../../test-data/SuperClass.class");

    let class = ClassInfo::parse(super_class).unwrap();

    assert_eq!(class.methods[1].name, "run".to_string().into());
    assert!(matches!(class.methods[1].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(code_attribute) = &class.methods[2].attributes[0] {
        assert!(matches!(code_attribute.attributes[0], AttributeInfo::LineNumberTable { .. }));
    }
}

#[test]
fn test_switch() {
    let super_class = include_bytes!("../../test-data/Switch.class");

    let class = ClassInfo::parse(super_class).unwrap();

    assert_eq!(class.methods[2].name, "run".to_string().into());
    assert!(matches!(class.methods[2].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(code_attribute) = &class.methods[2].attributes[0] {
        assert!(matches!(
            code_attribute.code.get(&6).unwrap(),
            Opcode::Tableswitch(default, pairs) if *default == 68 && *pairs == vec![(1, 30), (2, 41), (3, 52), (4, 60)]
        ));

        assert!(matches!(
            code_attribute.code.get(&75).unwrap(),
            Opcode::Lookupswitch(default, pairs) if *default == 82 && *pairs == vec![(1, 41), (10, 52), (100, 63), (1000, 74)]));
    }
}

#[test]
fn test_switch_rejects_entry_counts_larger_than_remaining_input() {
    let constant_pool = BTreeMap::new();
    let lookup_switch = [0xab, 0, 0, 0, 0, 0, 0, 0, 0x7f, 0xff, 0xff, 0xff];
    assert!(Opcode::parse(&lookup_switch, 0, &constant_pool).is_err());

    let table_switch = [0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x7f, 0xff, 0xff, 0xff];
    assert!(Opcode::parse(&table_switch, 0, &constant_pool).is_err());
}

#[test]
fn test_invokeinterface() {
    let interface = include_bytes!("../../test-data/Interface.class");

    let class = ClassInfo::parse(interface).unwrap();

    assert_eq!(class.methods[1].name, "main".to_string().into());
    if let AttributeInfo::Code(x) = &class.methods[1].attributes[0] {
        assert_eq!(x.code.len(), 7);
        assert!(matches!(x.code.get(&9).unwrap(),
            Opcode::Invokeinterface(ConstantPoolReference::InterfaceMethodref(m), 1, 0) if m.class == "Interface$IInterface".to_string().into() && m.name == "test".to_string().into()));
        assert!(!x.code.contains_key(&12));
        assert!(!x.code.contains_key(&13));
        assert!(matches!(x.code.get(&14).unwrap(), Opcode::Return));
    } else {
        panic!("Expected code attribute");
    }
}

#[test]
fn test_malformed_class_files_return_structured_errors() {
    let hello = include_bytes!("../../test-data/Hello.class");

    assert_eq!(
        ClassInfo::parse(&[]).err(),
        Some(ClassFileError::InvalidFormat("truncated or unparsable class file"))
    );

    let mut invalid_magic = hello.to_vec();
    invalid_magic[0] = 0;
    assert_eq!(
        ClassInfo::parse(&invalid_magic).err(),
        Some(ClassFileError::InvalidFormat("truncated or unparsable class file"))
    );

    let mut unsupported_version = hello.to_vec();
    unsupported_version[6..8].copy_from_slice(&71u16.to_be_bytes());
    assert_eq!(ClassInfo::parse(&unsupported_version).err(), Some(ClassFileError::UnsupportedVersion(71)));

    assert_eq!(
        ClassInfo::parse(&hello[..hello.len() / 2]).err(),
        Some(ClassFileError::InvalidFormat("truncated or unparsable class file"))
    );

    let minimal_class = vec![
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x2d, 0x00, 0x05, 0x01, 0x00, 0x04, b'T', b'e', b's', b't', 0x07, 0x00, 0x01, 0x01, 0x00, 0x10,
        b'j', b'a', b'v', b'a', b'/', b'l', b'a', b'n', b'g', b'/', b'O', b'b', b'j', b'e', b'c', b't', 0x07, 0x00, 0x03, 0x00, 0x21, 0x00, 0x02,
        0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    assert!(ClassInfo::parse(&minimal_class).is_ok());

    let mut invalid_constant_pool_index = minimal_class.clone();
    invalid_constant_pool_index[44..46].copy_from_slice(&99u16.to_be_bytes());
    assert_eq!(
        ClassInfo::parse(&invalid_constant_pool_index).err(),
        Some(ClassFileError::InvalidFormat("truncated or unparsable class file")),
        "an index past the end of the pool stops the parse, before validation sees it"
    );

    let mut invalid_constant_pool_type = minimal_class;
    invalid_constant_pool_type[44..46].copy_from_slice(&1u16.to_be_bytes());
    assert_eq!(
        ClassInfo::parse(&invalid_constant_pool_type).err(),
        Some(ClassFileError::InvalidFormat("truncated or unparsable class file")),
        "measured, not assumed: this one is refused by the parser, not by validation"
    );
}

#[test]
fn test_class_info_validation_rejects_invalid_names_descriptors_and_code_layout() {
    let hello = include_bytes!("../../test-data/Hello.class");

    let mut invalid_name = ClassInfo::parse(hello).unwrap();
    invalid_name.this_class = "[I".to_string().into();
    assert_eq!(
        invalid_name.validate(),
        Err(ClassFileError::InvalidFormat("this_class does not name a class"))
    );

    let mut invalid_descriptor = ClassInfo::parse(hello).unwrap();
    invalid_descriptor.methods[0].descriptor = "(V)V".to_string().into();
    assert_eq!(
        invalid_descriptor.validate(),
        Err(ClassFileError::InvalidFormatAt {
            cause: "a method descriptor is malformed",
            location: Location::Method(0),
        })
    );

    let mut missing_code = ClassInfo::parse(hello).unwrap();
    missing_code.methods[0].attributes.clear();
    assert_eq!(
        missing_code.validate(),
        Err(ClassFileError::InvalidFormatAt {
            cause: "a method does not have exactly one Code attribute",
            location: Location::Method(0),
        })
    );
}

#[test]
fn test_array_clone_method_owner_is_a_valid_class_constant() {
    assert!(ClassInfo::parse(include_bytes!("../../test-data/Array.class")).is_ok());
}

fn bootstrap_methods(class: &ClassInfo) -> &[BootstrapMethod] {
    class
        .attributes
        .iter()
        .find_map(|attribute| match attribute {
            AttributeInfo::BootstrapMethods(x) => Some(x.as_slice()),
            _ => None,
        })
        .expect("class has no BootstrapMethods attribute")
}

// Every field is asserted, and that is the point: the attribute is nested counted arrays
// (JVMS 4.7.23), so reading any one width at the wrong offset shifts everything after it into
// garbage. An assertion on the entry count alone survives that; these do not.
#[test]
fn test_bootstrap_methods_parses_into_structure() {
    let class = ClassInfo::parse(include_bytes!("../../test-data/indy/StringConcat.class")).unwrap();

    let methods = bootstrap_methods(&class);

    assert_eq!(methods.len(), 1);
    assert_eq!(methods[0].method.kind, MethodHandleKind::InvokeStatic);
    assert_eq!(methods[0].method.member.class, "java/lang/invoke/StringConcatFactory".to_string().into());
    assert_eq!(methods[0].method.member.name, "makeConcatWithConstants".to_string().into());
    assert_eq!(
        methods[0].method.member.descriptor,
        "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite;"
            .to_string()
            .into()
    );

    // The single static argument is the concat recipe. Asserting what the index *points at*,
    // not just its value, is what makes a shifted read visible here rather than three files away.
    assert_eq!(methods[0].arguments.len(), 1);
    assert!(matches!(
        ConstantPoolReference::from_constant_pool(&class.constant_pool, methods[0].arguments[0]),
        Some(ConstantPoolReference::String(x)) if x == "a\u{1}".to_string().into()
    ));
}

// The arguments stay indices on purpose (see `BootstrapMethod::arguments`), and a lambda is the
// case that makes the difference load-bearing: its bootstrap arguments are MethodType and
// MethodHandle constants. If a later round resolves arguments eagerly, this class stops parsing
// and a lambda goes from "unsupported" back to "corrupt file".
#[test]
fn test_bootstrap_method_arguments_of_a_lambda_survive_as_indices() {
    let class = ClassInfo::parse(include_bytes!("../../test-data/indy/Lambda.class")).unwrap();

    let methods = bootstrap_methods(&class);

    assert_eq!(methods.len(), 1);
    assert_eq!(methods[0].method.member.class, "java/lang/invoke/LambdaMetafactory".to_string().into());
    assert_eq!(methods[0].method.member.name, "metafactory".to_string().into());
    // samMethodType, implMethod, instantiatedMethodType — the first and last are the same entry
    assert_eq!(methods[0].arguments.len(), 3);
    assert_eq!(methods[0].arguments[0], methods[0].arguments[2]);
    assert_ne!(methods[0].arguments[0], methods[0].arguments[1]);
}

// Two rules meet at the same byte, and they are owned by different code. `MethodHandleKind`
// covers exactly reference kinds 1..=9, so anything outside that is a handle this parser cannot
// describe. *Which* kind goes with *which* member reference (JVMS 4.4.8) is `validation`'s rule,
// deliberately not duplicated here — this pins that it really is enforced there, so the
// duplication stays unnecessary rather than merely absent.
#[test]
fn test_bootstrap_method_reference_kinds_outside_the_set_and_mispaired_kinds_are_both_rejected() {
    let string_concat = include_bytes!("../../test-data/indy/StringConcat.class");
    let class = ClassInfo::parse(string_concat).unwrap();
    assert_eq!(bootstrap_methods(&class)[0].method.kind, MethodHandleKind::InvokeStatic);

    // constant pool entry #34 is `MethodHandle 6:#35`, where #35 is a Methodref
    let handle = string_concat
        .windows(4)
        .position(|window| window == [15, 6, 0, 35])
        .expect("test-data/indy/StringConcat.class layout changed; the MethodHandle entry moved");

    let parse_with_kind = |reference_kind: u8| {
        let mut mutated = string_concat.to_vec();
        mutated[handle + 1] = reference_kind;
        ClassInfo::parse(&mutated).err()
    };

    // outside 1..=9 — rejected by `MethodHandleKind`
    for reference_kind in [0u8, 10, 255] {
        assert_eq!(
            parse_with_kind(reference_kind),
            Some(ClassFileError::InvalidFormat("truncated or unparsable class file")),
            "reference kind {reference_kind} must not parse"
        );
    }

    // inside the set but mispaired with a Methodref target — rejected by `validation`
    for reference_kind in [1u8, 4, 9] {
        assert_eq!(
            parse_with_kind(reference_kind),
            Some(ClassFileError::InvalidFormatAt {
                cause: "a constant pool entry names a missing or wrong-kind entry",
                // the mutated MethodHandle itself — byte 383 of the fixture is pool entry #34
                location: Location::ConstantPoolEntry(34),
            }),
            "reference kind {reference_kind} does not pair with a Methodref — and the cause says it was validation, \
             not the parser, that refused it (the loop above is the parser's)"
        );
    }

    // and a kind that does pair with a Methodref still parses, so the above is not vacuous
    assert_eq!(parse_with_kind(5), None);
}

// JVMS 4.7.23 requires each `bootstrap_arguments` entry to be a **loadable** constant, not merely
// an index that lands somewhere. The bound was checked before; the kind was not, so a file whose
// argument named a Utf8 parsed fine and only fell over later — at the linker, which reports
// "unsupported" about a file that is actually broken.
//
// OpenJDK 26.0.1 on this exact mutation: `ClassFormatError: argument_index 4 has bad constant type
// in class file StringConcat`. So the rule is real and it is a *format* error.
//
// The mutation repoints the single bootstrap argument at pool entry #4, a Utf8. That keeps every
// other byte — including the length fields — exactly as it was, so the argument's kind is the only
// thing wrong with the file, which is what makes this test able to fail for the right reason.
#[test]
fn test_a_bootstrap_argument_that_is_not_a_loadable_constant_is_rejected() {
    let string_concat = include_bytes!("../../test-data/indy/StringConcat.class");
    // unmutated: parses, and the argument really is the String this repoints away from
    let class = ClassInfo::parse(string_concat).unwrap();
    assert!(matches!(
        ConstantPoolReference::from_constant_pool(&class.constant_pool, bootstrap_methods(&class)[0].arguments[0]),
        Some(ConstantPoolReference::String(_))
    ));

    // `num_bootstrap_methods=1, bootstrap_method_ref=#34, num_arguments=1, argument=#32`
    let window = [0u8, 1, 0, 34, 0, 1, 0, 32];
    let at = string_concat
        .windows(window.len())
        .position(|candidate| candidate == window)
        .expect("test-data/indy/StringConcat.class layout changed; the BootstrapMethods entry moved");

    let mut mutated = string_concat.to_vec();
    mutated[at + 6..at + 8].copy_from_slice(&[0, 4]); // entry #4 is a Utf8 ("java/lang/Object")
    assert!(
        matches!(class.constant_pool.get(&4), Some(_)),
        "the replacement index must be in the pool"
    );

    assert_eq!(
        ClassInfo::parse(&mutated).err(),
        Some(ClassFileError::InvalidBootstrapArgument {
            method_index: 0,
            argument_index: 0,
            actual: Some("Utf8"),
        }),
        "a bootstrap argument naming a Utf8 is not a loadable constant, and the refusal says which argument and what it found"
    );
}

/// ★ The index has to *follow the input*, not be a constant that happens to read correctly.
///
/// `StringConcat` above has a single argument, so its `argument_index` is 0 whether the code
/// computes it or hardcodes it — that test cannot tell the two apart. A lambda's bootstrap method
/// takes three (samMethodType, implMethod, instantiatedMethodType), so breaking a different one has
/// to move the number. Both mutations are the same edit at a different offset, which is what makes
/// the pair a control for each other.
#[test]
fn test_the_reported_bootstrap_argument_index_follows_the_broken_argument() {
    let lambda = include_bytes!("../../test-data/indy/Lambda.class");
    let class = ClassInfo::parse(lambda).unwrap();
    let arguments = &bootstrap_methods(&class)[0].arguments;
    assert_eq!(arguments.len(), 3, "a LambdaMetafactory call site carries three static arguments");

    // Derived from the parse rather than hardcoded, so a regenerated fixture moves the window with it.
    let mut window = vec![0u8, arguments.len() as u8];
    for argument in arguments {
        window.extend_from_slice(&argument.to_be_bytes());
    }
    let at = lambda
        .windows(window.len())
        .position(|candidate| candidate == window)
        .expect("test-data/indy/Lambda.class layout changed; the BootstrapMethods argument list moved");

    // Entry #4 is a Utf8 ("java/lang/Object"): a real pool entry that is not a loadable constant.
    // The kind is not assumed — the assertion below reads it back, so a regenerated fixture whose
    // #4 is something else fails here rather than passing for the wrong reason.
    let utf8: u16 = 4;
    assert!(class.constant_pool.get(&utf8).is_some(), "the replacement index must be in the pool");

    for broken in [0usize, 2usize] {
        let mut mutated = lambda.to_vec();
        let offset = at + 2 + broken * 2;
        mutated[offset..offset + 2].copy_from_slice(&utf8.to_be_bytes());

        assert_eq!(
            ClassInfo::parse(&mutated).err(),
            Some(ClassFileError::InvalidBootstrapArgument {
                method_index: 0,
                argument_index: broken as u16,
                actual: Some("Utf8"),
            }),
            "breaking argument #{broken} must report argument #{broken}"
        );
    }
}

/// The other half of the rule: an index that names no entry at all reports `actual: None`, and the
/// message says "names no constant pool entry" rather than guessing a kind.
#[test]
fn test_a_bootstrap_argument_naming_nothing_reports_no_kind() {
    let string_concat = include_bytes!("../../test-data/indy/StringConcat.class");
    let class = ClassInfo::parse(string_concat).unwrap();
    let window = [0u8, 1, 0, 34, 0, 1, 0, 32];
    let at = string_concat
        .windows(window.len())
        .position(|candidate| candidate == window)
        .expect("test-data/indy/StringConcat.class layout changed; the BootstrapMethods entry moved");

    let past_end = u16::MAX;
    assert!(class.constant_pool.get(&past_end).is_none(), "the index must name nothing");

    let mut mutated = string_concat.to_vec();
    mutated[at + 6..at + 8].copy_from_slice(&past_end.to_be_bytes());

    assert_eq!(
        ClassInfo::parse(&mutated).err(),
        Some(ClassFileError::InvalidBootstrapArgument {
            method_index: 0,
            argument_index: 0,
            actual: None,
        })
    );
}

/// The cost of the variant, measured rather than asserted in prose: it stays `Copy` and the enum
/// does not grow, because two `u16`s and a `&'static str` fit in the space `InvalidFormat` already
/// needed for its string.
#[test]
fn test_the_structured_variant_does_not_grow_the_error_type() {
    assert_eq!(size_of::<ClassFileError>(), size_of::<&'static str>() + size_of::<usize>());
    fn assert_copy<T: Copy>() {}
    assert_copy::<ClassFileError>();
}

/// Every table kind a rule can stop in, each asserted with the position the rule stopped at.
///
/// ★ The mutations put the fault at a position that is *not* zero where the table allows it, so a
/// rule that reported "the first one" instead of "the one that failed" would not pass by accident.
#[test]
fn test_a_rule_that_walks_a_table_names_the_position_it_stopped_at() {
    let hello = include_bytes!("../../test-data/Hello.class");

    let mut interface = ClassInfo::parse(hello).unwrap();
    let position = interface.interfaces.len() as u16;
    interface.interfaces.push("[I".to_string().into());
    assert_eq!(
        interface.validate(),
        Err(ClassFileError::InvalidFormatAt {
            cause: "an interface entry does not name a class",
            location: Location::Interface(position),
        })
    );

    let mut method = ClassInfo::parse(hello).unwrap();
    let last = method.methods.len() - 1;
    method.methods[last].descriptor = "(V)V".to_string().into();
    assert_eq!(
        method.validate(),
        Err(ClassFileError::InvalidFormatAt {
            cause: "a method descriptor is malformed",
            location: Location::Method(last as u16),
        })
    );

    let mut field = ClassInfo::parse(include_bytes!("../../test-data/Field.class")).unwrap();
    let last = field.fields.len() - 1;
    field.fields[last].descriptor = "V".to_string().into();
    assert_eq!(
        field.validate(),
        Err(ClassFileError::InvalidFormatAt {
            cause: "a field descriptor is malformed",
            location: Location::Field(last as u16),
        })
    );

    // From committed fixtures rather than mutations: the pool index and attribute position are the
    // fixture's own, so these pin that the number comes out of the file and not out of the loop.
    for (bytes, expected) in [
        (
            &include_bytes!("../../test-data/ldc/LdcDynamicOldMajor.class")[..],
            ClassFileError::InvalidFormatAt {
                cause: "class file version does not support a constant tag it carries",
                location: Location::ConstantPoolEntry(18),
            },
        ),
        (
            &include_bytes!("../../test-data/ldc/LdcDynamicNoBSM.class")[..],
            ClassFileError::InvalidFormatAt {
                cause: "a dynamic constant names no bootstrap method",
                location: Location::ConstantPoolEntry(11),
            },
        ),
        (
            &include_bytes!("../../test-data/ldc/LdcDynamicDuplicateBSM.class")[..],
            ClassFileError::InvalidFormatAt {
                cause: "a single-valued class attribute appears more than once",
                location: Location::ClassAttribute(1),
            },
        ),
    ] {
        assert_eq!(ClassInfo::parse(bytes).err(), Some(expected));
    }
}
