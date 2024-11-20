#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
#[Derive(Debug)]
struct CustomNameLast {
    first: &'a str,
    last: &'a str,
}
#[Derive(Debug)]
/// Doc comment for CustomName
enum CustomName {
    First,
    Last(CustomNameLast),
}
enum DeepNested<'a, AGE> {
    Named { a: u32, b: u32 },
    Unnamed(u32, u32),
    None,
    NestedEnum(CustomName, u32),
    id(CustomName<'a>),
}
