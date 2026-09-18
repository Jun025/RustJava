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
