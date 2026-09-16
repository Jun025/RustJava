#![no_std]
extern crate alloc;

mod attribute;
mod class;
mod constant_pool;
mod error;
mod field;
mod interface;
mod method;
mod opcode;
mod validation;

pub use {
    attribute::{AttributeInfo, AttributeInfoCode, BootstrapMethod, MethodHandleKind, MethodHandleRef},
    class::ClassInfo,
    constant_pool::{ConstantPoolReference, FieldMethodref},
    error::ClassFileError,
    field::FieldInfo,
    method::MethodInfo,
    opcode::Opcode,
};
