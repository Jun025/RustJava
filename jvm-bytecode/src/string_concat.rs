//! Resolving the one `invokedynamic` bootstrap this runtime links:
//! `java.lang.invoke.StringConcatFactory.makeConcatWithConstants`.
//!
//! javac 9+ lowers `+` on strings to an `invokedynamic` call site bound to that factory, so an
//! ordinary one-liner needs it. Everything else — `makeConcat`, `LambdaMetafactory`,
//! `ConstantBootstraps` — stays unsupported, and that is deliberate: the verifier rejects any
//! `Opcode::Invokedynamic` that is still an `Invokedynamic` by the time it runs, so *not*
//! recognising a call site here is what keeps the refusal honest.
//!
//! ## Why the rewrite happens at class definition time
//!
//! `Interpreter::run` is handed a `Code` attribute and nothing else, but `BootstrapMethods` is a
//! *class* attribute. There is no path from the interpreter to the bootstrap table. The only place
//! holding both is `ClassDefinitionImpl::from_classfile`, so that is where the call site is
//! resolved and written into the code as `Opcode::InvokedynamicStringConcat`.

use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

use classfile::{AttributeInfo, ClassInfo, ConstantPoolReference, MethodHandleKind, Opcode, StringConcatCallSite};

/// The bootstrap method this runtime links, spelled out in full.
///
/// All four axes are compared. Matching on fewer — say, the method name alone — would link call
/// sites bound to some other class's `makeConcatWithConstants`, which is exactly the "linked
/// everything" change this is meant not to be.
const FACTORY_CLASS: &str = "java/lang/invoke/StringConcatFactory";
const FACTORY_NAME: &str = "makeConcatWithConstants";
const FACTORY_DESCRIPTOR: &str = "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite;";

/// Rewrite every recognised `makeConcatWithConstants` call site in `class` into a resolved opcode.
///
/// Call this *before* the verifier. Anything left as `Opcode::Invokedynamic` afterwards is a
/// bootstrap this runtime does not link, and the verifier turns it into `UnsupportedFeature`.
pub(crate) fn lower(class: &mut ClassInfo) {
    let resolved = resolve_bootstrap_methods(class);
    if resolved.is_empty() {
        return;
    }

    for method in &mut class.methods {
        for attribute in &mut method.attributes {
            let AttributeInfo::Code(code) = attribute else {
                continue;
            };
            for opcode in code.code.values_mut() {
                let Opcode::Invokedynamic(ConstantPoolReference::InvokeDynamic {
                    bootstrap_method_attr_index,
                    descriptor,
                    ..
                }) = opcode
                else {
                    continue;
                };
                let Some((recipe, constants)) = resolved.get(bootstrap_method_attr_index) else {
                    continue;
                };

                *opcode = Opcode::InvokedynamicStringConcat(StringConcatCallSite {
                    recipe: recipe.clone(),
                    descriptor: descriptor.clone(),
                    constants: constants.clone(),
                });
            }
        }
    }
}

/// Bootstrap index -> (recipe, constants), for the entries that are this factory and nothing else.
fn resolve_bootstrap_methods(class: &ClassInfo) -> BTreeMap<u16, (Arc<String>, Vec<Arc<String>>)> {
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

            // The recipe is static argument 0; the rest fill the recipe's `\u{2}` slots. Both must
            // be String constants — a recipe that is not a string is not a shape javac emits, and
            // guessing at it would be linking something we have not read.
            let (recipe, constants) = bootstrap.arguments.split_first()?;
            let recipe = string_constant(class, *recipe)?;
            let constants = constants.iter().map(|x| string_constant(class, *x)).collect::<Option<Vec<_>>>()?;

            Some((index as u16, (recipe, constants)))
        })
        .collect()
}

fn string_constant(class: &ClassInfo, index: u16) -> Option<Arc<String>> {
    match ConstantPoolReference::from_constant_pool(&class.constant_pool, index) {
        Some(ConstantPoolReference::String(x)) => Some(x),
        _ => None,
    }
}
