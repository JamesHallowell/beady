use trybuild::TestCases;

#[test]
fn invalid_sections() {
    let test_cases = TestCases::new();
    test_cases.compile_fail("tests/try/given_then.rs");
    test_cases.compile_fail("tests/try/when_given.rs");
    test_cases.compile_fail("tests/try/then_given.rs");
    test_cases.compile_fail("tests/try/then_when.rs");
}
