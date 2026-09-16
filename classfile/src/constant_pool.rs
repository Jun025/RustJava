use alloc::{collections::BTreeMap, string::String, sync::Arc};

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map_res,
    error::{Error, ErrorKind},
    number::complete::{be_f32, be_f64, be_i32, be_i64, be_u16, u8},
};

fn parse_utf8(data: &[u8]) -> IResult<&[u8], Arc<String>> {
    let (data, length) = be_u16(data)?;
    map_res(take(length as usize), |utf8: &[u8]| String::from_utf8(utf8.to_vec()).map(Arc::new)).parse(data)
}

#[derive(Debug)]
pub enum ConstantPoolItem {
    Utf8(Arc<String>),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class {
        name_index: u16,
    },
    String {
        string_index: u16,
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

impl ConstantPoolItem {
    fn parse_tagged(data: &[u8], tag: u8) -> IResult<&[u8], Self> {
        match tag {
            1 => {
                let (data, utf8) = parse_utf8(data)?;
                Ok((data, Self::Utf8(utf8)))
            }
            3 => {
                let (data, value) = be_i32(data)?;
                Ok((data, Self::Integer(value)))
            }
            4 => {
                let (data, value) = be_f32(data)?;
                Ok((data, Self::Float(value)))
            }
            5 => {
                let (data, value) = be_i64(data)?;
                Ok((data, Self::Long(value)))
            }
            6 => {
                let (data, value) = be_f64(data)?;
                Ok((data, Self::Double(value)))
            }
            7 => {
                let (data, name_index) = be_u16(data)?;
                Ok((data, Self::Class { name_index }))
            }
            8 => {
                let (data, string_index) = be_u16(data)?;
                Ok((data, Self::String { string_index }))
            }
            9 => {
                let (data, class_index) = be_u16(data)?;
                let (data, name_and_type_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::Fieldref {
                        class_index,
                        name_and_type_index,
                    },
                ))
            }
            10 => {
                let (data, class_index) = be_u16(data)?;
                let (data, name_and_type_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::Methodref {
                        class_index,
                        name_and_type_index,
                    },
                ))
            }
            11 => {
                let (data, class_index) = be_u16(data)?;
                let (data, name_and_type_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::InterfaceMethodref {
                        class_index,
                        name_and_type_index,
                    },
                ))
            }
            12 => {
                let (data, name_index) = be_u16(data)?;
                let (data, descriptor_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::NameAndType {
                        name_index,
                        descriptor_index,
                    },
                ))
            }
            15 => {
                let (data, reference_kind) = u8(data)?;
                let (data, reference_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::MethodHandle {
                        reference_kind,
                        reference_index,
                    },
                ))
            }
            16 => {
                let (data, descriptor_index) = be_u16(data)?;
                Ok((data, Self::MethodType { descriptor_index }))
            }
            17 | 18 => {
                let (data, bootstrap_method_attr_index) = be_u16(data)?;
                let (data, name_and_type_index) = be_u16(data)?;
                let item = if tag == 17 {
                    Self::Dynamic {
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    }
                } else {
                    Self::InvokeDynamic {
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    }
                };
                Ok((data, item))
            }
            _ => Err(nom::Err::Error(Error::new(data, ErrorKind::Switch))),
        }
    }

    pub fn parse_all(data: &[u8]) -> IResult<&[u8], BTreeMap<u16, Self>> {
        let (remaining, count) = be_u16(data)?;
        if count == 0 {
            return Err(nom::Err::Error(Error::new(remaining, ErrorKind::Verify)));
        }
        if count == 1 {
            return Ok((remaining, BTreeMap::new()));
        }

        let mut data = remaining;
        let mut result = BTreeMap::new();
        let mut i = 1;
        loop {
            let (remaining, item) = Self::parse_with_tag(data)?;
            let is_double_entry = match &item {
                Self::Long(_) | Self::Double(_) => {
                    // long or double constant takes two constant pool entries....
                    true
                }
                _ => false,
            };
            result.insert(i, item);

            data = remaining;
            i += 1;
            if is_double_entry {
                i += 1;
            }

            if i > count {
                return Err(nom::Err::Error(Error::new(data, ErrorKind::Verify)));
            }
            if i == count {
                break;
            }
        }

        Ok((data, result))
    }

    pub fn parse_with_tag(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, tag) = u8(data)?;
        Self::parse_tagged(data, tag)
    }

    pub fn utf8(&self) -> Option<Arc<String>> {
        if let ConstantPoolItem::Utf8(x) = self { Some(x.clone()) } else { None }
    }

    pub fn class_name_index(&self) -> Option<u16> {
        if let ConstantPoolItem::Class { name_index } = self {
            Some(*name_index)
        } else {
            None
        }
    }

    pub fn name_and_type(&self) -> Option<(u16, u16)> {
        if let ConstantPoolItem::NameAndType {
            name_index,
            descriptor_index,
        } = self
        {
            Some((*name_index, *descriptor_index))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub enum ConstantPoolReference {
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(Arc<String>),
    Class(Arc<String>),
    Method(FieldMethodref),
    InterfaceMethodref(FieldMethodref),
    Field(FieldMethodref),
    // The index is carried verbatim, not dereferenced. `AttributeInfo::BootstrapMethods` is now
    // a parsed `Vec<BootstrapMethod>` rather than a byte blob, so the entry it names *could* be
    // looked up — but nothing links a call site yet (the verifier rejects every `invokedynamic`
    // at class definition time), and this type has no access to the class attributes anyway.
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name: Arc<String>,
        descriptor: Arc<String>,
    },
    // `ldc`-able constants (JVMS 6.5 ldc) that nothing resolves yet. No operand is carried:
    // resolving a method handle, a method type or a condy needs machinery that does not exist,
    // and the only consumer is the verifier, which turns them into "not implemented". They are
    // here so the `ldc` family can say that instead of "corrupt class file" — a real JVM loads
    // these files fine (test-data/ldc/, checked against OpenJDK 26).
    MethodHandle,
    MethodType,
    Dynamic,
}

impl ConstantPoolReference {
    pub fn from_constant_pool(constant_pool: &BTreeMap<u16, ConstantPoolItem>, index: u16) -> Option<Self> {
        match constant_pool.get(&index)? {
            ConstantPoolItem::Integer(x) => Some(Self::Integer(*x)),
            ConstantPoolItem::Float(x) => Some(Self::Float(*x)),
            ConstantPoolItem::Long(x) => Some(Self::Long(*x)),
            ConstantPoolItem::Double(x) => Some(Self::Double(*x)),
            ConstantPoolItem::String { string_index } => Some(Self::String(constant_pool.get(string_index)?.utf8()?)),
            ConstantPoolItem::Class { name_index } => Some(Self::Class(constant_pool.get(name_index)?.utf8()?)),
            ConstantPoolItem::Methodref {
                class_index,
                name_and_type_index,
            } => Some(Self::Method(FieldMethodref::from_reference_info(
                constant_pool,
                *class_index,
                *name_and_type_index,
            )?)),
            ConstantPoolItem::Fieldref {
                class_index,
                name_and_type_index,
            } => Some(Self::Field(FieldMethodref::from_reference_info(
                constant_pool,
                *class_index,
                *name_and_type_index,
            )?)),
            ConstantPoolItem::InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => Some(Self::InterfaceMethodref(FieldMethodref::from_reference_info(
                constant_pool,
                *class_index,
                *name_and_type_index,
            )?)),
            ConstantPoolItem::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                let (name_index, descriptor_index) = constant_pool.get(name_and_type_index)?.name_and_type()?;
                Some(Self::InvokeDynamic {
                    bootstrap_method_attr_index: *bootstrap_method_attr_index,
                    name: constant_pool.get(&name_index)?.utf8()?,
                    descriptor: constant_pool.get(&descriptor_index)?.utf8()?,
                })
            }
            // Shape already checked by `validation::validate_constant_pool` (reference kind vs.
            // target, descriptor well-formedness), so there is nothing left to resolve here.
            ConstantPoolItem::MethodHandle { .. } => Some(Self::MethodHandle),
            ConstantPoolItem::MethodType { .. } => Some(Self::MethodType),
            ConstantPoolItem::Dynamic { .. } => Some(Self::Dynamic),
            _ => None,
        }
    }

    pub fn as_class(&self) -> &str {
        if let Self::Class(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn as_field_ref(&self) -> &FieldMethodref {
        if let Self::Field(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn as_method_ref(&self) -> &FieldMethodref {
        if let Self::Method(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn as_interface_method_ref(&self) -> &FieldMethodref {
        if let Self::InterfaceMethodref(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldMethodref {
    pub class: Arc<String>,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
}

impl FieldMethodref {
    pub fn from_reference_info(constant_pool: &BTreeMap<u16, ConstantPoolItem>, class_index: u16, name_and_type_index: u16) -> Option<Self> {
        let class_name_index = constant_pool.get(&class_index)?.class_name_index()?;
        let class_name = constant_pool.get(&class_name_index)?.utf8()?;

        let (name_index, descriptor_index) = constant_pool.get(&name_and_type_index)?.name_and_type()?;
        let name = constant_pool.get(&name_index)?.utf8()?;
        let descriptor = constant_pool.get(&descriptor_index)?.utf8()?;

        Some(Self {
            class: class_name,
            name,
            descriptor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantPoolItem;

    #[test]
    fn empty_constant_pool_is_valid() {
        let (remaining, constant_pool) = ConstantPoolItem::parse_all(&[0x00, 0x01, 0xff]).unwrap();

        assert!(constant_pool.is_empty());
        assert_eq!(remaining, &[0xff]);
    }

    #[test]
    fn parses_method_handle_family_tags() {
        // 15 MethodHandle, 16 MethodType, 17 Dynamic, 18 InvokeDynamic — each a distinct
        // operand shape, so a copy-pasted arm reading the wrong width shifts every later entry.
        let entries: &[(&[u8], fn(&ConstantPoolItem) -> bool)] = &[
            (&[15, 6, 0x00, 0x2a], |x| {
                matches!(
                    x,
                    ConstantPoolItem::MethodHandle {
                        reference_kind: 6,
                        reference_index: 42
                    }
                )
            }),
            (&[16, 0x00, 0x2a], |x| matches!(x, ConstantPoolItem::MethodType { descriptor_index: 42 })),
            (&[17, 0x00, 0x01, 0x00, 0x2a], |x| {
                matches!(
                    x,
                    ConstantPoolItem::Dynamic {
                        bootstrap_method_attr_index: 1,
                        name_and_type_index: 42
                    }
                )
            }),
            (&[18, 0x00, 0x01, 0x00, 0x2a], |x| {
                matches!(
                    x,
                    ConstantPoolItem::InvokeDynamic {
                        bootstrap_method_attr_index: 1,
                        name_and_type_index: 42
                    }
                )
            }),
        ];

        for (bytes, expected) in entries {
            let (remaining, item) = ConstantPoolItem::parse_with_tag(bytes).unwrap();

            assert!(remaining.is_empty(), "tag {} left {remaining:?} unconsumed", bytes[0]);
            assert!(expected(&item), "tag {} parsed as {item:?}", bytes[0]);
        }
    }

    #[test]
    fn tags_outside_the_accepted_set_are_still_rejected() {
        // 0, 2, 13, 14 and 21.. are unassigned by JVMS 4.4; 19/20 (Module/Package) are assigned
        // but only legal in a module-info, which this parser does not read. Either way, widening
        // the parser to 15..=18 must not have turned the tag switch into a pass-through.
        for tag in [0u8, 2, 13, 14, 19, 20, 21, 255] {
            assert!(ConstantPoolItem::parse_with_tag(&[tag, 0, 0, 0, 0]).is_err(), "tag {tag} must not parse");
        }
    }

    // The tests above feed each tag one at a time, straight into `parse_tagged`. That fixes each
    // operand width but never exercises `parse_all`'s slot accounting, so a mistake there — the
    // classic one is treating an entry as the two-slot kind that only Long and Double are — is
    // invisible to them and only shows up on a pool read end to end.
    //
    // This is also the only place that asserts the fixture still *contains* tags 16 and 17. It is
    // real javac output, and a compiler that stopped emitting them would otherwise turn the
    // end-to-end test in tests/test_class_format.rs into a green assertion about nothing.
    #[test]
    fn real_javac_output_carries_the_method_handle_family_through_a_whole_pool() {
        let class = crate::ClassInfo::parse(include_bytes!("../../test-data/indy/ConstantKinds.class")).unwrap();

        let count = |predicate: fn(&ConstantPoolItem) -> bool| class.constant_pool.values().filter(|x| predicate(x)).count();

        assert_eq!(count(|x| matches!(x, ConstantPoolItem::MethodType { .. })), 1);
        assert_eq!(count(|x| matches!(x, ConstantPoolItem::Dynamic { .. })), 3);
        assert_eq!(count(|x| matches!(x, ConstantPoolItem::MethodHandle { .. })), 7);
        assert_eq!(count(|x| matches!(x, ConstantPoolItem::InvokeDynamic { .. })), 3);

        // A shifted read does not usually lose entries, it misattributes them — so pin that the
        // last entry is still reachable and is what javac put there.
        let last = class.constant_pool.keys().next_back().copied().unwrap();
        assert!(matches!(class.constant_pool.get(&last), Some(ConstantPoolItem::Utf8(_))));
    }

    #[test]
    fn long_must_fit_in_two_constant_pool_slots() {
        assert!(ConstantPoolItem::parse_all(&[0x00, 0x02, 0x05, 0, 0, 0, 0, 0, 0, 0, 0]).is_err());
    }

    // The test above pins one direction from the outside: a Long that overruns the declared count
    // must fail. It says nothing about Double, and nothing about where the *following* entry lands
    // — and that is the real symptom of a miscount, since a shifted pool rarely loses an entry, it
    // misnumbers every later one. Measured on this tree by mutating the two-slot set: dropping
    // Double is caught only by `tests/test_class.rs` (the whole-JVM suite), and widening the set to
    // Integer only by class files read end to end. Both arrive far from the parser that broke.
    #[test]
    fn only_long_and_double_consume_two_constant_pool_slots() {
        // count = 5, so the pool holds: Integer at 1, the two-slot entry at 2 (its tail is 3), and
        // Integer at 4. Any other slot width makes the last entry land somewhere other than 4.
        let pools: [(&str, &[u8]); 2] = [
            ("Long", &[0x00, 0x05, 3, 0, 0, 0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 7, 3, 0, 0, 0, 2]),
            ("Double", &[0x00, 0x05, 3, 0, 0, 0, 1, 6, 0x3f, 0xf0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 2]),
        ];

        for (kind, bytes) in pools {
            let (remaining, pool) = ConstantPoolItem::parse_all(bytes).unwrap_or_else(|x| panic!("{kind} pool did not parse: {x:?}"));

            assert!(remaining.is_empty(), "{kind}: {remaining:?} left unconsumed");
            assert!(
                pool.keys().copied().eq([1u16, 2, 4]),
                "{kind}: slots {:?}, expected 1, 2 and 4",
                pool.keys()
            );
            assert!(
                matches!(pool.get(&4), Some(ConstantPoolItem::Integer(2))),
                "{kind}: the entry after the two-slot one is {:?}",
                pool.get(&4)
            );
        }
    }
}
