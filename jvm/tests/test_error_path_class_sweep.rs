//! Every class construction asks the loader for, hidden one at a time, and what that does.
//!
//! Two earlier rounds each closed exactly one class on the error path -- `java/lang/String` and
//! `java/lang/NoClassDefFoundError` -- and each found the next by reading the code and guessing.
//! Nothing enumerated the candidates, so "is that all of them?" had no answer. This asks the whole
//! question at once: build a JVM with a loader that records every name it is asked for, then rebuild
//! it once per name with that name hidden, and classify what happens.
//!
//! The first run of this sweep answered it: **no**. Of 37 non-array names, five more recursed with
//! no floor -- `java/lang/Throwable`, `java/lang/Error`, `java/lang/LinkageError`,
//! `java/lang/CharSequence` and `java/lang/Comparable` -- which is the supertype and interface
//! closure of the two known ones, less the two of that closure that `bootstrap_classes` already
//! loads. Before: `5 recursed · 7 refused by name · 25 failed cleanly`. After: `0 · 12 · 25`.
//! `Jvm::new` now walks that closure, and this file is what keeps the answer true: a class added to
//! the error path later brings its own supertypes with it, and they land here without anyone
//! thinking to ask.
//!
//! The second axis is *what the refusal says*. (A refusal was a panic until construction learned to
//! return `JavaError::Unraisable`; it is read from either, and the counts below are unchanged by it.) `refused by name` used to mean only that construction
//! had panicked; five of the twelve panicked on `called Option::unwrap() on a None value` from the
//! `bootstrap_classes` loop, which is a refusal a host cannot act on, and this sweep reported them
//! among the good ones. The panic message is now read and matched against the hidden name, so a
//! refusal that does not name its class is its own failure: `0 · 12 · 0 · 25`.
//!
//! Array names are skipped, and the reason is not squeamishness: the bootstrap loader *synthesises*
//! them (`define_array_class`), so no class set can be missing one and hiding them measures a loader
//! that refuses to build an array rather than a gap a host can have. It is not a free skip -- two of
//! them recurse. `[C` is reached through the message (`JavaLangString::from_rust_string`) and
//! `[Ljava/lang/String;` through `Throwable`'s `stackTrace` field, which `fillInStackTrace` fills
//! with `instantiate_array` while the exception reporting the *first* gap is still being built.
//!
//! Hiding `[Ljava/lang/String;` at this cap aborts with `stack overflow`, and the round that first
//! saw that wrote down the wrong reason -- "its recursion does not come back through the loader, so
//! the cap cannot end it". Measured since (`rustjava-error-path-string-array-hiding-overflows-stack-p1`):
//! it does come back, every turn, and the cap ends it exactly like `[C`. Two knobs move the
//! threshold independently, which is what tells bounded-but-deep apart from unbounded: at the
//! default 2 MiB test-thread stack it survives a cap of 18 (19 questions) and overflows at 19, and
//! at this cap of 20 it overflows on 2 MiB but survives on 4 MiB with `asked = 21` -- the same
//! `cap + 1` shape `[C` shows. The difference between the two is cost per turn, not shape: one turn
//! of the `[Ljava/lang/String;` cycle is ~57 stack frames because it runs the whole exception
//! construction (`exception` -> `new_class` -> four nested `invoke_special` for
//! `NoClassDefFoundError` -> `LinkageError` -> `Error` -> `Throwable.<init>` -> `fillInStackTrace`),
//! where `[C` turns inside `from_rust_string`.
//!
//! So arrays stay skipped for the reason at the top of this paragraph -- no class set can lack one --
//! and not because anything here is unmeasurable.

use std::{
    future::Future,
    panic::{self, AssertUnwindSafe},
    sync::atomic::Ordering,
};

use jvm::JavaError;
use test_utils::{test_jvm_hiding, test_jvm_recording};

/// Low on purpose. A name on the error path comes back for the cap's worth of questions and then the
/// harness relents, so the recursion is *observed* rather than ridden into a stack overflow -- the
/// measured floors are 117 and 121 round trips, far above this.
const GIVE_UP_AFTER: u32 = 20;

#[derive(Debug, PartialEq)]
enum Outcome {
    /// Came back for every question the cap allowed: the reporting path needs this class to report
    /// that this class is missing. Without a check, the real loader never relents and the process
    /// aborts on a stack overflow.
    Recursed,
    /// Construction refused -- `Unraisable`, or a panic -- *and the message contains the hidden name*.
    /// What the check produces. The message is read rather than assumed: for five of the names below this outcome
    /// used to be `called Option::unwrap() on a None value`, which is a refusal that tells the host
    /// nothing, and this arm counted it as a good one. A dead process and a read message are two
    /// facts, so they are measured separately.
    Panicked,
    /// Construction refused without saying which class. The defect this file no longer accepts.
    PanickedAnonymously,
    /// Construction returned a Java exception after a single question. Also fine: nothing looped.
    Failed,
    /// Construction succeeded without the class. It is asked for but not needed.
    Built,
}

fn block_on<F: Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future)
}

fn hide(name: &str) -> Outcome {
    match panic::catch_unwind(AssertUnwindSafe(|| block_on(test_jvm_hiding(name, GIVE_UP_AFTER)))) {
        Err(payload) => {
            // `panic!("...")` with arguments carries a `String`; a bare literal carries a `&str`.
            // Both shapes appear here, so both are read before concluding the name is absent.
            let message = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("");
            named(message, name)
        }
        Ok((result, requests)) => {
            if requests.load(Ordering::SeqCst) > GIVE_UP_AFTER {
                Outcome::Recursed
            } else if let Err(JavaError::Unraisable(message)) = &result {
                named(message, name)
            } else if result.is_err() {
                Outcome::Failed
            } else {
                Outcome::Built
            }
        }
    }
}

fn named(message: &str, name: &str) -> Outcome {
    if message.contains(name) {
        Outcome::Panicked
    } else {
        Outcome::PanickedAnonymously
    }
}

// A plain `#[test]`, not `#[tokio::test]`: each candidate gets its own runtime inside `block_on`,
// and nesting one runtime in another panics before any of this can run.
#[test]
fn no_class_construction_asks_for_can_only_be_reported_with_itself() {
    let (_jvm, names) = block_on(test_jvm_recording()).unwrap();
    let mut candidates = names.lock().unwrap().clone();
    candidates.sort();
    candidates.dedup();
    candidates.retain(|name| !name.starts_with('['));
    assert!(
        candidates.len() > 30,
        "the recording loader saw {} names -- too few to be the whole of construction",
        candidates.len()
    );

    // The hidden-class runs panic by design; their default output would bury the report below.
    let hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let outcomes = candidates.iter().map(|name| (name.clone(), hide(name))).collect::<Vec<_>>();
    panic::set_hook(hook);

    // Never silent: the counts are the measurement, and a green run that reports nothing cannot be
    // told apart from a green run that looked at nothing.
    let count = |want| outcomes.iter().filter(|(_, outcome)| *outcome == want).count();
    println!(
        "{} candidate(s): {} recursed · {} refused by name · {} refused anonymously · {} failed cleanly · {} not needed",
        outcomes.len(),
        count(Outcome::Recursed),
        count(Outcome::Panicked),
        count(Outcome::PanickedAnonymously),
        count(Outcome::Failed),
        count(Outcome::Built)
    );

    let anonymous = outcomes
        .iter()
        .filter(|(_, outcome)| *outcome == Outcome::PanickedAnonymously)
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    assert!(
        anonymous.is_empty(),
        "hiding {anonymous:?} takes construction down without the panic message saying which class is \
         missing, so the host is left bisecting the class set. Name it where it is refused."
    );

    let recursed = outcomes
        .iter()
        .filter(|(_, outcome)| *outcome == Outcome::Recursed)
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    assert!(
        recursed.is_empty(),
        "hiding {recursed:?} makes construction ask for the same class more than {GIVE_UP_AFTER} times: \
         reporting the gap needs the class that is missing. Add it to the closure `Jvm::new` walks, or \
         explain here why it cannot be reached through a class set."
    );
}
