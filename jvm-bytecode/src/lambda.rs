//! Resolving the `invokedynamic` bootstrap that makes an *object*:
//! `java.lang.invoke.LambdaMetafactory.metafactory`.
//!
//! javac lowers every lambda and method reference to a call site bound to that factory, so this is
//! the other half of what a compiler emits routinely — `string_concat.rs` linked the first half.
//!
//! ## What a real JVM does, and what is done instead
//!
//! `metafactory` returns a `CallSite` holding a `MethodHandle` that yields an instance of the
//! functional interface; the instance's class is spun at runtime and its single method invokes the
//! `implMethod` handle. There is no `java.lang.invoke` package here — no `MethodHandle`, no
//! `CallSite` — so the spinning is done directly: a [`ClassDefinitionImpl`] whose interface method
//! is a Rust body that calls the implementation through `Jvm`. The captured values become fields,
//! which is what javac's own `arg$n` naming describes and what the GC already knows how to trace
//! (it walks `ClassDefinition::fields`).
//!
//! That is the whole of the shortcut: the object is real, the dispatch is real, and the thing that
//! does not exist is the handle chain that would normally deliver them.
//!
//! ## What is not linked, and why that is a refusal rather than a gap
//!
//! The implementation's signature has to line up with the interface method's, position by
//! position, as a **pass-through**: identical primitives, or a reference either way. Where the real
//! factory would insert an adapter — boxing an `int` into an `Object`, unboxing, widening — the
//! call site is left as `Opcode::Invokedynamic`, and the verifier refuses the class as an
//! unsupported feature. A refusal is a worse experience than an adapter and a much better one than
//! a wrong answer, and the boundary is checked where it is decidable: at lowering time, from the
//! descriptors alone, so a class either loads or does not. It never fails halfway through a call.
//!
//! `altMetafactory` — the entry point for serializable and multi-interface lambdas — is not this
//! method and is not linked.

use alloc::{
    boxed::Box,
    collections::BTreeMap,
    format,
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};

use classfile::{AttributeInfo, ClassInfo, ConstantPoolReference, LambdaCallSite, MethodHandleKind, MethodHandleRef, Opcode, method_type_descriptor};
use jvm::{ClassInstance, Field, JavaType, JavaValue, Jvm, JvmCallback, Result};
use jvm_types::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};

use crate::{
    class_definition::ClassDefinitionImpl,
    field::FieldImpl,
    method::{MethodBody, MethodImpl},
};

/// The bootstrap method this module links, spelled out in full — all four axes, for the reason
/// `string_concat.rs` gives: matching on fewer would link call sites bound to someone else's
/// `metafactory`.
const FACTORY_CLASS: &str = "java/lang/invoke/LambdaMetafactory";
const FACTORY_NAME: &str = "metafactory";
const FACTORY_DESCRIPTOR: &str = "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite;";

/// A bootstrap entry that is the factory, with its static arguments resolved.
///
/// `instantiated` — the third static argument — is read but not used for dispatch: the interface
/// method this runtime defines carries the *erased* signature, which is the one callers invoke
/// through the interface. It is resolved anyway because its absence means the entry is not the
/// shape `metafactory` claims to be, and a bootstrap that is not that shape must not be linked.
struct LinkedFactory {
    sam_descriptor: Arc<String>,
    implementation: MethodHandleRef,
}

/// Rewrite every recognised `metafactory` call site in `class` into a resolved opcode.
///
/// Called before the verifier, like `string_concat::lower` — whatever is still
/// `Opcode::Invokedynamic` afterwards is a bootstrap this runtime does not link.
pub(crate) fn lower(class: &mut ClassInfo) {
    let resolved = resolve_bootstrap_methods(class);
    if resolved.is_empty() {
        return;
    }

    // Numbered per class, not per bootstrap entry: two call sites may share one bootstrap entry
    // while capturing different things, and each needs its own class. The name only has to be
    // unique within the class — it is never parsed, and the registry is keyed by it.
    let host = class.this_class.clone();
    let mut index = 0;

    for method in &mut class.methods {
        for attribute in &mut method.attributes {
            let AttributeInfo::Code(code) = attribute else {
                continue;
            };
            for opcode in code.code.values_mut() {
                let Opcode::Invokedynamic(ConstantPoolReference::InvokeDynamic {
                    bootstrap_method_attr_index,
                    name,
                    descriptor,
                }) = opcode
                else {
                    continue;
                };
                let Some(linked) = resolved.get(bootstrap_method_attr_index) else {
                    continue;
                };
                // The call site's return type is the interface being implemented. Anything else
                // is not a `metafactory` call site whatever its bootstrap says.
                let call_site_type = JavaType::parse(descriptor);
                let (captures, JavaType::Class(interface)) = call_site_type.as_method() else {
                    continue;
                };
                if !adapts(captures, &linked.sam_descriptor, &linked.implementation) {
                    continue;
                }

                *opcode = Opcode::InvokedynamicLambda(LambdaCallSite {
                    class_name: Arc::new(format!("{host}$$Lambda${index}")),
                    interface_name: Arc::new(interface.clone()),
                    descriptor: descriptor.clone(),
                    method_name: name.clone(),
                    method_descriptor: linked.sam_descriptor.clone(),
                    implementation: linked.implementation.clone(),
                });
                index += 1;
            }
        }
    }
}

/// Bootstrap index -> what it links to, for the entries that are this factory and nothing else.
fn resolve_bootstrap_methods(class: &ClassInfo) -> BTreeMap<u16, LinkedFactory> {
    let Some(bootstrap_methods) = class.attributes.iter().find_map(|attribute| match attribute {
        AttributeInfo::BootstrapMethods(methods) => Some(methods),
        _ => None,
    }) else {
        return BTreeMap::new();
    };

    bootstrap_methods
        .iter()
        .enumerate()
        .filter_map(|(index, bootstrap)| {
            if bootstrap.method.kind != MethodHandleKind::InvokeStatic
                || bootstrap.method.member.class.as_str() != FACTORY_CLASS
                || bootstrap.method.member.name.as_str() != FACTORY_NAME
                || bootstrap.method.member.descriptor.as_str() != FACTORY_DESCRIPTOR
            {
                return None;
            }

            // `metafactory` takes exactly three static arguments, of exactly these kinds. A
            // bootstrap naming it with anything else is not the shape it claims to be — the same
            // rule `string_concat.rs` applies to its recipe, and the same consequence: not linked.
            let [sam, implementation, instantiated] = bootstrap.arguments[..] else {
                return None;
            };
            let sam_descriptor = method_type_descriptor(&class.constant_pool, sam)?;
            method_type_descriptor(&class.constant_pool, instantiated)?;
            let implementation = MethodHandleRef::resolve(&class.constant_pool, implementation)?;

            Some((
                index as u16,
                LinkedFactory {
                    sam_descriptor,
                    implementation,
                },
            ))
        })
        .collect()
}

/// Whether the implementation can be called by passing the captured values and the interface
/// method's arguments straight through, with no conversion anywhere.
///
/// This is the boundary named at the top of the file. It is deliberately an exact shape test
/// rather than an assignability test: `Jvm` values carry their own types, so a reference flows
/// into any reference parameter, but an `int` reaching an `Object` parameter needs a box that
/// nothing here would create.
fn adapts(captures: &[JavaType], sam_descriptor: &str, implementation: &MethodHandleRef) -> bool {
    let Some(sam_type) = JavaType::try_parse(sam_descriptor) else {
        return false;
    };
    let JavaType::Method(sam_parameters, sam_return) = &sam_type else {
        return false;
    };
    let Some((parameters, returns)) = implementation_signature(implementation) else {
        return false;
    };

    if captures.len() + sam_parameters.len() != parameters.len() {
        return false;
    }
    if !captures.iter().chain(sam_parameters).zip(&parameters).all(|(from, to)| passes(from, to)) {
        return false;
    }

    // A `void` interface method discards whatever the implementation returned, which is what the
    // real factory does too — `Runnable r = list::clear` is an ordinary method reference.
    **sam_return == JavaType::Void || passes(&returns, sam_return)
}

/// The signature the implementation is *called* with, which is not the descriptor written in the
/// class file: an instance method takes its receiver first, and a constructor returns its class.
fn implementation_signature(implementation: &MethodHandleRef) -> Option<(Vec<JavaType>, JavaType)> {
    let descriptor = JavaType::try_parse(&implementation.member.descriptor)?;
    let JavaType::Method(parameters, returns) = descriptor else {
        return None;
    };
    let receiver = JavaType::from_class_name(&implementation.member.class);

    Some(match implementation.kind {
        MethodHandleKind::InvokeStatic => (parameters, *returns),
        MethodHandleKind::InvokeVirtual | MethodHandleKind::InvokeSpecial | MethodHandleKind::InvokeInterface => {
            ([receiver].into_iter().chain(parameters).collect(), *returns)
        }
        MethodHandleKind::NewInvokeSpecial => (parameters, receiver),
        // Field kinds cannot be an `implMethod`: JVMS 5.4.3.5 resolves them to a handle that reads
        // or writes a field, and `metafactory` documents a *method* handle.
        _ => return None,
    })
}

fn passes(from: &JavaType, to: &JavaType) -> bool {
    match (from, to) {
        (JavaType::Class(_) | JavaType::Array(_), JavaType::Class(_) | JavaType::Array(_)) => true,
        _ => from == to,
    }
}

/// Execute a resolved call site: make the object it evaluates to.
///
/// The class is registered on first execution rather than built at lowering time because lowering
/// has no `Jvm` — `ClassDefinitionImpl::from_classfile` is handed bytes and nothing else. Two
/// threads reaching the same call site at once both build one; `register_class` keeps the first
/// and drops the other, and the two are identical, so the loser costs a construction and nothing
/// else.
pub(crate) async fn instantiate(jvm: &Jvm, call_site: &LambdaCallSite, captures: Vec<JavaValue>) -> Result<Box<dyn ClassInstance>> {
    if !jvm.has_class(&call_site.class_name) {
        jvm.register_class(Box::new(synthesise(call_site)), None).await?;
    }

    let mut instance = jvm.instantiate_class(&call_site.class_name).await?;
    for (index, (value, descriptor)) in captures.into_iter().zip(capture_descriptors(call_site)).enumerate() {
        jvm.put_field(&mut instance, &capture_name(index), &descriptor, value).await?;
    }

    Ok(instance)
}

/// The class the call site evaluates an instance of: the interface, one field per captured value,
/// and one method — the interface's, implemented by [`LambdaBody`].
fn synthesise(call_site: &LambdaCallSite) -> ClassDefinitionImpl {
    let fields = capture_descriptors(call_site)
        .into_iter()
        .enumerate()
        .map(|(index, descriptor)| FieldImpl::new(&capture_name(index), &descriptor, FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL))
        .collect::<Vec<_>>();

    let body = LambdaBody {
        implementation: call_site.implementation.clone(),
        captures: fields
            .iter()
            .enumerate()
            .map(|(index, x)| (capture_name(index), x.descriptor()))
            .collect(),
        returns_void: matches!(JavaType::parse(&call_site.method_descriptor).as_method().1, JavaType::Void),
    };
    let method = MethodImpl::new(
        &call_site.method_name,
        &call_site.method_descriptor,
        MethodBody::from_rust(Box::new(body)),
        MethodAccessFlags::PUBLIC,
    );

    ClassDefinitionImpl::new(
        &call_site.class_name,
        Some("java/lang/Object".to_string()),
        vec![call_site.interface_name.to_string()],
        ClassAccessFlags::FINAL | ClassAccessFlags::SYNTHETIC,
        vec![method],
        fields,
    )
}

/// javac's own name for a captured value, kept because a stack trace or a debugger reading these
/// fields should see what it would see on a real JVM.
fn capture_name(index: usize) -> String {
    format!("arg${index}")
}

fn capture_descriptors(call_site: &LambdaCallSite) -> Vec<String> {
    let call_site_type = JavaType::parse(&call_site.descriptor);

    call_site_type.as_method().0.iter().map(descriptor_of).collect()
}

fn descriptor_of(r#type: &JavaType) -> String {
    match r#type {
        JavaType::Void => "V".to_string(),
        JavaType::Boolean => "Z".to_string(),
        JavaType::Byte => "B".to_string(),
        JavaType::Char => "C".to_string(),
        JavaType::Short => "S".to_string(),
        JavaType::Int => "I".to_string(),
        JavaType::Long => "J".to_string(),
        JavaType::Float => "F".to_string(),
        JavaType::Double => "D".to_string(),
        JavaType::Class(name) => format!("L{name};"),
        JavaType::Array(element) => format!("[{}", descriptor_of(element)),
        JavaType::Method(..) => unreachable!("a method type cannot be a captured value's type"),
    }
}

/// The interface method's body: read the captures back out, put the caller's arguments after them,
/// and invoke the implementation the way its reference kind says it is invoked.
struct LambdaBody {
    implementation: MethodHandleRef,
    captures: Vec<(String, String)>,
    returns_void: bool,
}

#[async_trait::async_trait]
impl JvmCallback for LambdaBody {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        let JavaValue::Object(Some(this)) = &args[0] else {
            // `this` is prepended by `Jvm::execute_method` for every non-static method, so a
            // missing receiver is this runtime having called the wrong thing, not a guest error.
            return Err(jvm.exception("java/lang/InternalError", "lambda body called without a receiver").await);
        };
        let this = this.clone();

        let mut arguments = Vec::with_capacity(self.captures.len() + args.len() - 1);
        for (name, descriptor) in &self.captures {
            arguments.push(jvm.get_field(&this, name, descriptor).await?);
        }
        arguments.extend(args.into_vec().into_iter().skip(1));

        let member = &self.implementation.member;
        let result: JavaValue = match self.implementation.kind {
            MethodHandleKind::InvokeStatic => jvm.invoke_static(&member.class, &member.name, &member.descriptor, arguments).await?,
            MethodHandleKind::NewInvokeSpecial => JavaValue::Object(Some(jvm.new_class(&member.class, &member.descriptor, arguments).await?)),
            MethodHandleKind::InvokeVirtual | MethodHandleKind::InvokeInterface | MethodHandleKind::InvokeSpecial => {
                let JavaValue::Object(Some(receiver)) = arguments.remove(0) else {
                    return Err(jvm
                        .exception(
                            "java/lang/NullPointerException",
                            &format!("Method {}::{}:{} is called on null", member.class, member.name, member.descriptor),
                        )
                        .await);
                };

                if self.implementation.kind == MethodHandleKind::InvokeSpecial {
                    jvm.invoke_special(&receiver, &member.class, &member.name, &member.descriptor, arguments)
                        .await?
                } else {
                    // Virtual, so the receiver's own class selects the method — a method reference
                    // is dispatched at the call, exactly as writing the call would have been.
                    let class_name = receiver.class_definition().name();
                    jvm.invoke_virtual(&receiver, &class_name, &member.name, &member.descriptor, arguments)
                        .await?
                }
            }
            // Refused at lowering time; reaching this would mean the opcode was written by
            // something that did not run that check.
            _ => return Err(jvm.exception("java/lang/InternalError", "lambda implementation is not a method").await),
        };

        Ok(if self.returns_void { JavaValue::Void } else { result })
    }
}
