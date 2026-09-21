use test_utils::test_jvm_hiding;

// The cycle: `load_class` reports a class it cannot provide by calling
// `Jvm::exception("java/lang/NoClassDefFoundError", …)`, building that exception goes back through
// the loader, and if the loader cannot provide *that* class either the two call each other with no
// floor. Measured on the form before this test existed, with the same harness: 121 round trips
// survived and somewhere before 160 the process died of `stack overflow, aborting` (SIGABRT).
//
// Reverting the check in `Jvm::new` does not merely fail this test -- it takes the test binary down
// with it, which is the point: an abort is what a host embedding this runtime used to get.
#[tokio::test]
#[should_panic(expected = "has no java/lang/NoClassDefFoundError")]
async fn a_class_set_missing_the_reporter_fails_at_construction_rather_than_on_the_stack() {
    // 200 is the measuring device the harness offers, left high on purpose: the check under test
    // must fire on the *first* question, so any run that reaches the cap has already regressed.
    let _ = test_jvm_hiding("java/lang/NoClassDefFoundError", 200).await;
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
#[should_panic(expected = "has no java/lang/String")]
async fn a_class_set_missing_string_fails_at_construction_rather_than_on_the_stack() {
    // Above the measured 117 on purpose: reaching the cap at all means the check stopped firing on
    // the first question, and past that number the run aborts instead of failing.
    let _ = test_jvm_hiding("java/lang/String", 200).await;
}
