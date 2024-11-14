#[test]
pub fn pass() {
    macrotest::expand("tests/cases/*.rs");
}
