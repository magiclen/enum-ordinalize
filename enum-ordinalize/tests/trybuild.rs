#![cfg(all(feature = "derive", feature = "traits"))]

#[test]
fn ui_tests() {
    let tests = trybuild::TestCases::new();

    tests.pass("tests/ui/pass/*.rs");
    tests.compile_fail("tests/ui/fail/*.rs");
}
