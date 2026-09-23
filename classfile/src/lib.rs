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
    attribute::{AttributeInfo, AttributeInfoCode, BootstrapMethod, MethodHandleKind, MethodHandleRef, method_type_descriptor},
    class::ClassInfo,
    constant_pool::{ConstantPoolReference, FieldMethodref},
    error::{ClassFileError, Location},
    field::FieldInfo,
    method::MethodInfo,
    opcode::{LambdaCallSite, Opcode, StringConcatCallSite},
};
