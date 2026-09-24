use std::sync::atomic::Ordering;

use jvm::JavaError;
use test_utils::test_jvm_hiding;

/// The message of the `Unraisable` construction returned, or a failure saying what came back instead.
async fn unraisable_hiding(hidden: &str, give_up_after: u32) -> (String, u32) {
    let (result, requests) = test_jvm_hiding(hidden, give_up_after).await;
    let asked = requests.load(Ordering::SeqCst);
    match result {
        Err(JavaError::Unraisable(message)) => (message, asked),
        Err(JavaError::JavaException(e)) => panic!("hiding {hidden} raised a Java exception {e:?} after {asked} question(s)"),
        Ok(_) => panic!("hiding {hidden} built a JVM after {asked} question(s)"),
    }
}

// The cycle: `load_class` reports a class it cannot provide by calling
// `Jvm::exception("java/lang/NoClassDefFoundError", …)`, building that exception goes back through
// the loader, and if the loader cannot provide *that* class either the two call each other with no
// floor. Measured on the form before this test existed, with the same harness: 121 round trips
// survived and somewhere before 160 the process died of `stack overflow, aborting` (SIGABRT).
//
// `Jvm::new` checks the class and returns `Unraisable` naming it; it used to panic, which a host
// could not handle either.
#[tokio::test]
async fn a_class_set_missing_the_reporter_fails_at_construction_rather_than_on_the_stack() {
    // 200 is the measuring device the harness offers, left high on purpose: the check under test
    // must fire on the *first* question, so any run that reaches the cap has already regressed.
    let (message, asked) = unraisable_hiding("java/lang/NoClassDefFoundError", 200).await;
    assert!(message.contains("has no java/lang/NoClassDefFoundError"), "{message}");
    assert_eq!(asked, 1);
}

// The same cycle reached through the other class on the error path. `Jvm::exception` builds its
// message with `JavaLangString::from_rust_string` *before* it builds the exception instance, so a
// class set without `java/lang/String` cannot report the absence of String either -- the report
// needs the thing that is missing.
//
// This one was a question, not an assumption: the round that closed the NoClassDefFoundError cycle
// deliberately claimed nothing about String, and it could have been any of three things -- recursing,
// failing cleanly, or already resident by construction time. Measured with this harness before the
// check existed: caps up to 116 survived (the loader gives the real class up and construction ends
// in a plain NoClassDefFoundError), 117 died of `stack overflow, aborting` (SIGABRT), and the
// boundary reproduced exactly across repeats. Raising the cap to 100000 aborts the same way, so the
// cap is the harness relenting and not a floor. It recurses.
#[tokio::test]
async fn a_class_set_missing_string_fails_at_construction_rather_than_on_the_stack() {
    let (message, asked) = unraisable_hiding("java/lang/String", 200).await;
    assert!(message.contains("has no java/lang/String"), "{message}");
    assert_eq!(asked, 1);
}

// A class loaded before anything else (`bootstrap_classes` in `Jvm::new`). Nothing can be raised yet,
// so this is `Unraisable` too -- and it has to name the class, or the host bisects its class set.
#[tokio::test]
async fn a_class_set_missing_a_bootstrap_class_is_an_error_naming_it() {
    let (message, _) = unraisable_hiding("java/lang/Thread", 200).await;
    assert!(message.contains("has no java/lang/Thread"), "{message}");
}

// The cycle that no class set can reach but nothing in the product ended: `fillInStackTrace` asks for
// `[Ljava/lang/String;`, a loader that cannot provide it raises NoClassDefFoundError, and building
// that calls `fillInStackTrace` again. Before `Jvm::exception` refused to build an exception it was
// already building on the same thread, this cap (the sweep's 20) overflowed the default 2 MiB test
// stack -- `stack overflow, aborting`, rc 134 -- and the run said nothing about the first failure. Now
// the repeat returns `Unraisable` naming the exception the thread started with. The invariant is
// that this happens before the cap: how many questions the guard needs first is its business
// (today 2 -- the first failure and the repeat), and pinning that number would pin the guard's depth.
#[tokio::test]
async fn a_failure_while_raising_is_reported_instead_of_recursing() {
    let (message, asked) = unraisable_hiding("[Ljava/lang/String;", 20).await;
    assert!(
        message.contains("into raising java/lang/NoClassDefFoundError ([Ljava/lang/String;), which is the first failure"),
        "{message}"
    );
    assert!(asked < 20, "asked {asked} times: the guard did not stop it before the cap");
}
