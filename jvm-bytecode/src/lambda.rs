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
//! ## What is linked, and why everything else is a refusal rather than a gap
//!
//! The implementation's signature has to line up with the interface method's, position by
//! position. Most positions are a **pass-through**: identical primitives, or a reference either
//! way. The one adapter inserted is the `int` ↔ `Integer` pair, the conversion the generic
//! functional interfaces force (`Supplier<Integer>` reaching a method that returns `int`):
//! `Integer.valueOf` where an `int` meets a reference, `intValue` after a check that the value is
//! an `Integer` where a reference meets an `int`. [`Adapter`] lists it and [`parameter_adapter`] /
//! [`return_adapter`] say where it applies.
//!
//! Every other adapter the real factory would insert (`long`/`double` boxing, unboxing followed by
//! widening such as `Short` into an `int` parameter, boxing into `Number`) is not inserted. The
//! call site is left as `Opcode::Invokedynamic`, and the verifier refuses the class as an
//! unsupported feature. A refusal is a worse experience than an adapter and a much better one than
//! a wrong answer, which is why pairs are added one at a time. The boundary is checked where it is
//! decidable: at lowering time, from the descriptors alone, so a class either loads or does not.
//! It never fails halfway through a call on account of a shape. (It can throw inside a call, as
//! the real factory does: `NullPointerException` unboxing `null`, `ClassCastException` unboxing
//! something that is not an `Integer`.)
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
/// `instantiated_descriptor` — the third static argument — is not used for dispatch: the interface
/// method this runtime defines carries the *erased* signature, which is the one callers invoke
/// through the interface. It is read where an adapter decides what a value is: an erased `Object`
/// parameter is unboxed only when this says it is an `Integer`.
struct LinkedFactory {
    sam_descriptor: Arc<String>,
    instantiated_descriptor: Arc<String>,
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
                //
                // Parsed with `try_parse`, not `parse`, and that is not a style choice: a call
                // site's descriptor is a string out of the class file. `validation.rs` now requires
                // it to be a method descriptor at this usage site (JVMS 4.4.10), so a bad one is
                // refused before reaching here — but this stays `try_parse` anyway, because the
                // cost of being wrong is not a guest exception, it is a host abort:
                // `JavaType::parse` panics with "Invalid type". `verifier.rs` writes the same
                // sentence about `ldc` — "reaching it would abort the host, not the guest". Two
                // checks for one rule is cheap; one missing check is a crashed process.
                let Some(JavaType::Method(captures, returns)) = JavaType::try_parse(descriptor) else {
                    continue;
                };
                let JavaType::Class(interface) = &*returns else {
                    continue;
                };
                let captures = &captures[..];
                if plan(captures, &linked.sam_descriptor, &linked.instantiated_descriptor, &linked.implementation).is_none() {
                    continue;
                }

                *opcode = Opcode::InvokedynamicLambda(LambdaCallSite {
                    class_name: Arc::new(format!("{host}$$Lambda${index}")),
                    interface_name: Arc::new(interface.clone()),
                    descriptor: descriptor.clone(),
                    method_name: name.clone(),
                    method_descriptor: linked.sam_descriptor.clone(),
                    instantiated_method_descriptor: linked.instantiated_descriptor.clone(),
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
            let instantiated_descriptor = method_type_descriptor(&class.constant_pool, instantiated)?;
            let implementation = MethodHandleRef::resolve(&class.constant_pool, implementation)?;

            Some((
                index as u16,
                LinkedFactory {
                    sam_descriptor,
                    instantiated_descriptor,
                    implementation,
                },
            ))
        })
        .collect()
}

/// What happens to one value on its way between the interface method and the implementation.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Adapter {
    /// Nothing: identical primitives, or a reference either way (`Jvm` values carry their own
    /// types, so a reference flows into any reference parameter).
    Pass,
    /// An `int` meeting a reference: `Integer.valueOf`.
    BoxInt,
    /// A reference meeting an `int`: a check that it is an `Integer`, then `intValue`.
    UnboxInt,
}

const INTEGER: &str = "java/lang/Integer";

/// The one adapter plan for a call site: one entry per interface method parameter, and one for
/// the return (`None` when the interface method is `void` and the result is discarded).
///
/// Captured values are not adapted: they are passed straight through or the call site is refused.
/// javac captures a value with the implementation's own type, so no call site it emits needs more.
#[derive(Debug, PartialEq)]
struct Plan {
    parameters: Vec<Adapter>,
    returns: Option<Adapter>,
}

/// The plan, or `None` where some position needs an adapter this runtime does not insert.
///
/// This is the boundary named at the top of the file. Lowering asks it whether to link at all, and
/// the synthesised method asks it again for what to do, so the two can never disagree.
fn plan(captures: &[JavaType], sam_descriptor: &str, instantiated_descriptor: &str, implementation: &MethodHandleRef) -> Option<Plan> {
    let JavaType::Method(sam_parameters, sam_return) = JavaType::try_parse(sam_descriptor)? else {
        return None;
    };
    let JavaType::Method(instantiated_parameters, instantiated_return) = JavaType::try_parse(instantiated_descriptor)? else {
        return None;
    };
    let (parameters, returns) = implementation_signature(implementation)?;

    if captures.len() + sam_parameters.len() != parameters.len() || instantiated_parameters.len() != sam_parameters.len() {
        return None;
    }
    let (captured, passed) = parameters.split_at(captures.len());
    if !captures.iter().zip(captured).all(|(from, to)| passes(from, to)) {
        return None;
    }
    let parameters = sam_parameters
        .iter()
        .zip(&instantiated_parameters)
        .zip(passed)
        .map(|((sam, instantiated), implementation)| parameter_adapter(sam, instantiated, implementation))
        .collect::<Option<Vec<_>>>()?;

    // A `void` interface method discards whatever the implementation returned, which is what the
    // real factory does too — `Runnable r = list::clear` is an ordinary method reference.
    let returns = if *sam_return == JavaType::Void {
        None
    } else {
        Some(return_adapter(&returns, &sam_return, &instantiated_return)?)
    };

    Some(Plan { parameters, returns })
}

/// A caller's argument, typed `sam` (erased) and `instantiated` (generic), reaching an
/// implementation parameter typed `implementation`.
fn parameter_adapter(sam: &JavaType, instantiated: &JavaType, implementation: &JavaType) -> Option<Adapter> {
    if passes(sam, implementation) {
        return Some(Adapter::Pass);
    }
    match (sam, instantiated, implementation) {
        (JavaType::Int, JavaType::Int, to) if holds_boxed_int(to) => Some(Adapter::BoxInt),
        // The instantiated type is what makes this an `Integer` rather than a `Short` or a
        // `Character`: the erased `Object` says nothing, and unboxing a `Short` as an `Integer`
        // would throw where the real factory widens.
        (from, JavaType::Class(boxed), JavaType::Int) if holds_boxed_int(from) && boxed == INTEGER => Some(Adapter::UnboxInt),
        _ => None,
    }
}

/// The implementation's result, typed `implementation`, returned through an interface method typed
/// `sam` (erased) and `instantiated` (generic).
fn return_adapter(implementation: &JavaType, sam: &JavaType, instantiated: &JavaType) -> Option<Adapter> {
    if passes(implementation, sam) {
        return Some(Adapter::Pass);
    }
    match (implementation, sam, instantiated) {
        (JavaType::Int, to, generic) if holds_boxed_int(to) && holds_boxed_int(generic) => Some(Adapter::BoxInt),
        (JavaType::Class(boxed), JavaType::Int, JavaType::Int) if boxed == INTEGER => Some(Adapter::UnboxInt),
        _ => None,
    }
}

/// A reference type an `Integer` is known to fit without asking the class hierarchy: `Integer`
/// itself and `Object`. `Number`, `Comparable` and `Serializable` would be right too, and are
/// left out until something measures them.
fn holds_boxed_int(r#type: &JavaType) -> bool {
    matches!(r#type, JavaType::Class(name) if name == INTEGER || name == "java/lang/Object")
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
        let captured = JavaType::parse(&call_site.descriptor).as_method().0.to_vec();
        let Some(plan) = plan(
            &captured,
            &call_site.method_descriptor,
            &call_site.instantiated_method_descriptor,
            &call_site.implementation,
        ) else {
            // Lowering linked this call site only because the same function said yes, so a `None`
            // here is an opcode written by something that did not run that check.
            return Err(jvm
                .exception("java/lang/InternalError", "lambda call site needs an adapter that is not inserted")
                .await);
        };
        jvm.register_class(Box::new(synthesise(call_site, plan)), None).await?;
    }

    let mut instance = jvm.instantiate_class(&call_site.class_name).await?;
    for (index, (value, descriptor)) in captures.into_iter().zip(capture_descriptors(call_site)).enumerate() {
        jvm.put_field(&mut instance, &capture_name(index), &descriptor, value).await?;
    }

    Ok(instance)
}

/// The class the call site evaluates an instance of: the interface, one field per captured value,
/// and one method — the interface's, implemented by [`LambdaBody`].
fn synthesise(call_site: &LambdaCallSite, plan: Plan) -> ClassDefinitionImpl {
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
        plan,
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
    plan: Plan,
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
        for (value, adapter) in args.into_vec().into_iter().skip(1).zip(&self.plan.parameters) {
            arguments.push(adapt(jvm, *adapter, value).await?);
        }

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

        match self.plan.returns {
            None => Ok(JavaValue::Void),
            Some(adapter) => adapt(jvm, adapter, result).await,
        }
    }
}

/// Apply one [`Adapter`] to a value. The exceptions are the ones the real factory's adapter throws:
/// it casts to `Integer` before unboxing, so a wrong type is a `ClassCastException` and `null` is a
/// `NullPointerException`.
async fn adapt(jvm: &Jvm, adapter: Adapter, value: JavaValue) -> Result<JavaValue> {
    match (adapter, value) {
        (Adapter::Pass, value) => Ok(value),
        (Adapter::BoxInt, JavaValue::Int(value)) => {
            let boxed: Box<dyn ClassInstance> = jvm.invoke_static(INTEGER, "valueOf", "(I)Ljava/lang/Integer;", (value,)).await?;
            Ok(JavaValue::Object(Some(boxed)))
        }
        (Adapter::UnboxInt, JavaValue::Object(None)) => Err(jvm
            .exception("java/lang/NullPointerException", "Cannot unbox null to int in a lambda adapter")
            .await),
        (Adapter::UnboxInt, JavaValue::Object(Some(object))) => {
            if !jvm.is_instance(&*object, INTEGER) {
                let class_name = object.class_definition().name().replace('/', ".");
                return Err(jvm
                    .exception(
                        "java/lang/ClassCastException",
                        &format!("class {class_name} cannot be cast to class java.lang.Integer"),
                    )
                    .await);
            }
            let value: i32 = jvm.invoke_virtual(&object, INTEGER, "intValue", "()I", ()).await?;
            Ok(JavaValue::Int(value))
        }
        // The plan was made from the same descriptors the values were typed by; a mismatch here
        // is this runtime's mistake, not the guest's.
        (adapter, value) => Err(jvm
            .exception("java/lang/InternalError", &format!("lambda adapter {adapter:?} given {value:?}"))
            .await),
    }
}
