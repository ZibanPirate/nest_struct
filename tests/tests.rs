#[test]
pub fn expand_struct() {
    macrotest::expand("tests/cases/struct/*.rs");
}

#[test]
fn expand_enum() {
    macrotest::expand("tests/cases/enum/*.rs");
}

#[test]
fn expand_block() {
    macrotest::expand("tests/cases/block/*.rs");
}

#[test]
fn parse_error() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/cases/parse_error/*.rs");
}
