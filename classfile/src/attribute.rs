use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{flat_map, map, map_res},
    multi::length_count,
    number::complete::{be_u16, be_u32},
};

use crate::{ConstantPoolReference, FieldMethodref, constant_pool::ConstantPoolItem, opcode::Opcode};

/// The `reference_kind` of a CONSTANT_MethodHandle (JVMS 4.4.8).
///
/// Named rather than carried as a `u8` because linking a call site branches on it, and `6` does
/// not say `invokestatic` to anyone reading the call site code. Which kinds pair with which
/// member reference is *not* checked here — `validation::validate_constant_pool` owns that rule,
/// and duplicating it would give the two places a chance to disagree.
///
/// Lives in the attribute file, not in `constant_pool.rs`, and that was re-decided rather than
/// inherited. `constant_pool.rs` has already settled how much of a CONSTANT_MethodHandle a
/// constant pool reader gets: `ConstantPoolReference::MethodHandle`, carrying no operand, because
/// nothing resolves one. This deeper decode exists for exactly one reader — the `BootstrapMethods`
/// attribute below, which is the only thing that calls `MethodHandleRef::resolve`. Moving it over
/// would stand two different decodings of the same constant side by side in one file and read as a
/// contradiction. Re-open the question when something outside this attribute decodes a method
/// handle — the `ldc` path is the likely one, and today it answers `UnsupportedFeature` instead
/// (`jvm_bytecode::verifier`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MethodHandleKind {
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}

impl MethodHandleKind {
    fn from_reference_kind(reference_kind: u8) -> Option<Self> {
        Some(match reference_kind {
            1 => Self::GetField,
            2 => Self::GetStatic,
            3 => Self::PutField,
            4 => Self::PutStatic,
            5 => Self::InvokeVirtual,
            6 => Self::InvokeStatic,
            7 => Self::InvokeSpecial,
            8 => Self::NewInvokeSpecial,
            9 => Self::InvokeInterface,
            _ => return None,
        })
    }
}

/// A CONSTANT_MethodHandle decoded down to the member it names.
///
/// This is as far as "resolution" goes today, and the boundary is worth stating: JVMS 5.4.3.5
/// resolution produces a live `java.lang.invoke.MethodHandle` — it loads the owning class, looks
/// the member up, and checks access. None of that happens here. What you get is the class, name
/// and descriptor written in the class file, unverified against any loaded class.
#[derive(Clone, Debug)]
pub struct MethodHandleRef {
    pub kind: MethodHandleKind,
    pub member: FieldMethodref,
}

impl MethodHandleRef {
    fn resolve(constant_pool: &BTreeMap<u16, ConstantPoolItem>, index: u16) -> Option<Self> {
        let ConstantPoolItem::MethodHandle {
            reference_kind,
            reference_index,
        } = constant_pool.get(&index)?
        else {
            return None;
        };

        let (class_index, name_and_type_index) = match constant_pool.get(reference_index)? {
            ConstantPoolItem::Fieldref {
                class_index,
                name_and_type_index,
            }
            | ConstantPoolItem::Methodref {
                class_index,
                name_and_type_index,
            }
            | ConstantPoolItem::InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => (*class_index, *name_and_type_index),
            _ => return None,
        };

        Some(Self {
            kind: MethodHandleKind::from_reference_kind(*reference_kind)?,
            member: FieldMethodref::from_reference_info(constant_pool, class_index, name_and_type_index)?,
        })
    }
}

/// One entry of the `BootstrapMethods` attribute (JVMS 4.7.23).
pub struct BootstrapMethod {
    pub method: MethodHandleRef,
    /// Raw constant pool indices of the static arguments, **deliberately left unresolved**.
    ///
    /// Resolving them through `ConstantPoolReference::from_constant_pool` is the obvious next
    /// line, and it costs something while buying nothing. The kinds that actually turn up here
    /// are method types and method handles — `LambdaMetafactory.metafactory` takes
    /// MethodType/MethodHandle/MethodType — which this crate has no payload for, so resolution
    /// yields either a parse failure or an empty placeholder. A parse failure is the expensive
    /// one: it would turn every class containing a lambda from *unsupported* back into
    /// *corrupt*, undoing the distinction this attribute was parsed to serve. Nothing reads the
    /// arguments yet; the round that links a call site can resolve them when it can also
    /// represent them. The lambda case is locked in `classfile/tests/test.rs` (the indices
    /// survive) and `tests/test_class_format.rs` (the sentence the user reads).
    pub arguments: Vec<u16>,
}

impl BootstrapMethod {
    fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        map(
            (
                map_res(be_u16, |x| MethodHandleRef::resolve(constant_pool, x).ok_or(())),
                length_count(be_u16, be_u16),
            ),
            |(method, arguments)| Self { method, arguments },
        )
        .parse(data)
    }
}

pub struct CodeAttributeExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: Option<Arc<String>>,
}

impl CodeAttributeExceptionTable {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        map_res((be_u16, be_u16, be_u16, be_u16), |(start_pc, end_pc, handler_pc, catch_type)| {
            let catch_type = if catch_type != 0 {
                let index = constant_pool.get(&catch_type).and_then(ConstantPoolItem::class_name_index).ok_or(())?;
                Some(constant_pool.get(&index).and_then(ConstantPoolItem::utf8).ok_or(())?)
            } else {
                None
            };

            Ok::<_, ()>(Self {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            })
        })
        .parse(data)
    }
}

pub struct AttributeInfoCode {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: BTreeMap<u32, Opcode>, // TODO we can store it Vec<u8> and create code iterator..
    pub exception_table: Vec<CodeAttributeExceptionTable>,
    pub attributes: Vec<AttributeInfo>,
}

impl AttributeInfoCode {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        map(
            (
                be_u16,
                be_u16,
                map_res(flat_map(be_u32, take), |x: &[u8]| Self::parse_code(x, constant_pool)),
                length_count(be_u16, |x| CodeAttributeExceptionTable::parse(x, constant_pool)),
                length_count(be_u16, |x| AttributeInfo::parse(x, constant_pool)),
            ),
            |(max_stack, max_locals, code, exception_table, attributes)| Self {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
            },
        )
        .parse(data)
    }

    fn parse_code(code: &[u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> Result<BTreeMap<u32, Opcode>, ()> {
        let mut result = BTreeMap::new();

        let mut data = code;
        while !data.is_empty() {
            let offset = unsafe { data.as_ptr().offset_from(code.as_ptr()) } as usize;
            let (remaining, opcode) = Opcode::parse(data, offset, constant_pool).map_err(|_| ())?;
            if remaining.len() >= data.len() {
                return Err(());
            }
            result.insert(offset as _, opcode);
            data = remaining;
        }

        Ok(result)
    }
}

pub struct AttributeInfoLineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

impl AttributeInfoLineNumberTableEntry {
    pub fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, start_pc) = be_u16(data)?;
        let (data, line_number) = be_u16(data)?;

        Ok((data, Self { start_pc, line_number }))
    }
}

pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
    pub index: u16,
}

impl LocalVariableTableEntry {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        map(
            (
                be_u16,
                be_u16,
                map_res(be_u16, |x| constant_pool.get(&x).and_then(ConstantPoolItem::utf8).ok_or(())),
                map_res(be_u16, |x| constant_pool.get(&x).and_then(ConstantPoolItem::utf8).ok_or(())),
                be_u16,
            ),
            |(start_pc, length, name, descriptor, index)| Self {
                start_pc,
                length,
                name,
                descriptor,
                index,
            },
        )
        .parse(data)
    }
}

pub enum AttributeInfo {
    ConstantValue(ConstantPoolReference),
    Code(AttributeInfoCode),
    StackMap(Vec<u8>),      // TODO Older variant of StackMapTable
    StackMapTable(Vec<u8>), // TODO
    Exceptions(Vec<u8>),    // TODO
    InnerClasses(Vec<u8>),  // TODO
    Synthetic(Vec<u8>),     // TODO
    SourceFile(Arc<String>),
    SourceDebugExtension,
    LineNumberTable(Vec<AttributeInfoLineNumberTableEntry>),
    LocalVariableTable(Vec<LocalVariableTableEntry>),
    BootstrapMethods(Vec<BootstrapMethod>),
    MethodParameters(Vec<u8>), // TODO
    NestMembers(Vec<u8>),      // TODO
    NestHost(Vec<u8>),         // TODO
    Unknown(Arc<String>, Vec<u8>),
}

impl AttributeInfo {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        map_res(
            (
                map_res(be_u16, |x| constant_pool.get(&x).and_then(ConstantPoolItem::utf8).ok_or(())),
                flat_map(be_u32, take),
            ),
            |(name, info): (_, &[u8])| {
                Ok::<_, nom::Err<_>>(match name.as_str() {
                    "ConstantValue" => AttributeInfo::ConstantValue(Self::parse_constant_value(info, constant_pool)?.1),
                    "Code" => AttributeInfo::Code(AttributeInfoCode::parse(info, constant_pool)?.1),
                    "LineNumberTable" => {
                        AttributeInfo::LineNumberTable(length_count(be_u16, AttributeInfoLineNumberTableEntry::parse).parse(info)?.1)
                    }
                    "SourceFile" => AttributeInfo::SourceFile(Self::parse_source_file(info, constant_pool)?.1),
                    // The body is an unstructured UTF-8 blob nothing here reads; the variant exists so the
                    // at-most-one rule in `validation.rs` can see it. Without this arm it lands in `Unknown`,
                    // where a duplicate is indistinguishable from two attributes we do not recognise.
                    "SourceDebugExtension" => AttributeInfo::SourceDebugExtension,
                    "LocalVariableTable" => AttributeInfo::LocalVariableTable(Self::parse_local_variable_table(info, constant_pool)?.1),
                    "StackMap" => AttributeInfo::StackMap(info.to_vec()),
                    "StackMapTable" => AttributeInfo::StackMapTable(info.to_vec()),
                    "Exceptions" => AttributeInfo::Exceptions(info.to_vec()),
                    "InnerClasses" => AttributeInfo::InnerClasses(info.to_vec()),
                    "Synthetic" => AttributeInfo::Synthetic(info.to_vec()),
                    "BootstrapMethods" => AttributeInfo::BootstrapMethods(Self::parse_bootstrap_methods(info, constant_pool)?.1),
                    "MethodParameters" => AttributeInfo::MethodParameters(info.to_vec()),
                    "NestMembers" => AttributeInfo::NestMembers(info.to_vec()),
                    "NestHost" => AttributeInfo::NestHost(info.to_vec()),
                    // unrecognized attributes must be silently ignored (JVMS 4.7.1)
                    _ => AttributeInfo::Unknown(name.clone(), info.to_vec()),
                })
            },
        )
        .parse(data)
    }

    fn parse_source_file<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Arc<String>> {
        map_res(be_u16, |x| constant_pool.get(&x).and_then(ConstantPoolItem::utf8).ok_or(())).parse(data)
    }

    fn parse_constant_value<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], ConstantPoolReference> {
        map_res(be_u16, |x| ConstantPoolReference::from_constant_pool(constant_pool, x).ok_or(())).parse(data)
    }

    fn parse_bootstrap_methods<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Vec<BootstrapMethod>> {
        length_count(be_u16, |x| BootstrapMethod::parse(x, constant_pool)).parse(data)
    }

    fn parse_local_variable_table<'a>(
        data: &'a [u8],
        constant_pool: &BTreeMap<u16, ConstantPoolItem>,
    ) -> IResult<&'a [u8], Vec<LocalVariableTableEntry>> {
        length_count(be_u16, |x| LocalVariableTableEntry::parse(x, constant_pool)).parse(data)
    }
}
