//! Resolving the `invokedynamic` bootstraps this runtime links — both of
//! `java.lang.invoke.StringConcatFactory`'s entry points, and nothing else.
//!
//! javac 9+ lowers `+` on strings to an `invokedynamic` call site bound to that factory, so an
//! ordinary one-liner needs it. Everything else — `LambdaMetafactory`,
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
use jvm::JavaType;

/// The bootstrap methods this runtime links, spelled out in full.
///
/// All four axes are compared. Matching on fewer — say, the method name alone — would link call
/// sites bound to some other class's `makeConcatWithConstants`, which is exactly the "linked
/// everything" change this is meant not to be.
///
/// Two entries, and it stays a written-out list rather than growing into a registry: every
/// addition makes "this runtime links string concatenation" less precise, so the list has to be
/// short enough to read.
const FACTORY_CLASS: &str = "java/lang/invoke/StringConcatFactory";
const FACTORY_NAME: &str = "makeConcatWithConstants";
const FACTORY_DESCRIPTOR: &str = "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite;";
/// The recipe-free sibling: the call site descriptor alone says what to concatenate, so it takes
/// no static arguments and its own descriptor is shorter by exactly those two parameters.
///
/// Measured, because the reason to support it is not the one that is usually given: javac does
/// *not* emit this. Targets 9, 11, 17, 21 and 26 all emit `makeConcatWithConstants`, even for
/// `a + b` with no literal text — the recipe is then just `\u0001\u0001`. Only the non-default
/// `-XDstringConcat=indy` produces `makeConcat`. It is linked because it is a documented entry
/// point of the same factory that any bytecode generator may target, and because the executor
/// needs nothing new for it (below).
const FACTORY_NAME_NO_RECIPE: &str = "makeConcat";
const FACTORY_DESCRIPTOR_NO_RECIPE: &str =
    "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite;";

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
                let Some(linked) = resolved.get(bootstrap_method_attr_index) else {
                    continue;
                };

                // `makeConcat` is `makeConcatWithConstants` whose recipe is "every argument, in
                // order, nothing else" — so it is expressed as that recipe rather than as a second
                // path through the executor. The arity comes from this call site, which is why the
                // synthesis happens here and not where the bootstrap was recognised.
                let (recipe, constants) = match linked {
                    LinkedFactory::WithConstants { recipe, constants } => (recipe.clone(), constants.clone()),
                    LinkedFactory::NoRecipe => {
                        let method_type = JavaType::parse(descriptor);
                        let (parameters, _) = method_type.as_method();
                        (Arc::new("\u{1}".repeat(parameters.len())), Vec::new())
                    }
                };

                *opcode = Opcode::InvokedynamicStringConcat(StringConcatCallSite {
                    recipe,
                    descriptor: descriptor.clone(),
                    constants,
                });
            }
        }
    }
}

/// What a linked bootstrap tells us, which is not the same for the two entry points.
///
/// `makeConcatWithConstants` carries its recipe as a static argument, so it is known here.
/// `makeConcat` has none: the recipe follows from the *call site's* descriptor, and one bootstrap
/// entry can be shared by call sites with different descriptors — so it cannot be resolved to a
/// recipe at this level, only recognised.
enum LinkedFactory {
    WithConstants { recipe: Arc<String>, constants: Vec<Arc<String>> },
    NoRecipe,
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
            if bootstrap.method.kind != MethodHandleKind::InvokeStatic || bootstrap.method.member.class.as_str() != FACTORY_CLASS {
                return None;
            }

            // Name and descriptor are still compared together, one pair per entry point: matching a
            // name against the other entry point's descriptor must not link.
            let linked = match (bootstrap.method.member.name.as_str(), bootstrap.method.member.descriptor.as_str()) {
                (FACTORY_NAME, FACTORY_DESCRIPTOR) => {
                    // The recipe is static argument 0; the rest fill the recipe's `\u{2}` slots. Both
                    // must be String constants — a recipe that is not a string is not a shape javac
                    // emits, and guessing at it would be linking something we have not read.
                    let (recipe, constants) = bootstrap.arguments.split_first()?;
                    let recipe = string_constant(class, *recipe)?;
                    let constants = constants.iter().map(|x| string_constant(class, *x)).collect::<Option<Vec<_>>>()?;

                    LinkedFactory::WithConstants { recipe, constants }
                }
                // Static arguments are not merely unused here, they are a contradiction: this entry
                // point is defined as taking none, so anything else is not the shape it claims.
                (FACTORY_NAME_NO_RECIPE, FACTORY_DESCRIPTOR_NO_RECIPE) if bootstrap.arguments.is_empty() => LinkedFactory::NoRecipe,
                _ => return None,
            };

            Some((index as u16, linked))
        })
        .collect()
}

fn string_constant(class: &ClassInfo, index: u16) -> Option<Arc<String>> {
    match ConstantPoolReference::from_constant_pool(&class.constant_pool, index) {
        Some(ConstantPoolReference::String(x)) => Some(x),
        _ => None,
    }
}
