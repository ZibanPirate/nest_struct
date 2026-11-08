#[test]
pub fn expand_struct() {
    macrotest::expand_with_source_suffix("tests/cases/struct/*.before.rs", ".before");
}

#[test]
fn expand_enum() {
    macrotest::expand_with_source_suffix("tests/cases/enum/*.before.rs", ".before");
}

#[test]
fn expand_block() {
    macrotest::expand_with_source_suffix("tests/cases/block/*.before.rs", ".before");
}

#[test]
fn parse_error() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/cases/parse_error/*.rs");
}
