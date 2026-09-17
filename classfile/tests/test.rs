use std::collections::BTreeMap;

use jvm_types::ClassAccessFlags;

use classfile::{AttributeInfo, BootstrapMethod, ClassFileError, ClassInfo, ConstantPoolReference, MethodHandleKind, Opcode};

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
        Err(ClassFileError::InvalidFormat("a method descriptor is malformed"))
    );

    let mut missing_code = ClassInfo::parse(hello).unwrap();
    missing_code.methods[0].attributes.clear();
    assert_eq!(
        missing_code.validate(),
        Err(ClassFileError::InvalidFormat("a method does not have exactly one Code attribute"))
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
            Some(ClassFileError::InvalidFormat("a constant pool entry names a missing or wrong-kind entry")),
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
        Some(ClassFileError::InvalidFormat(
            "a bootstrap method argument names nothing or is not a loadable constant"
        )),
        "a bootstrap argument naming a Utf8 is not a loadable constant"
    );
}
