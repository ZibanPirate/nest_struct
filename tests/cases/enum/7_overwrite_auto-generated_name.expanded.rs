#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct CustomNameLast<'a> {
    first: &'a str,
    last: &'a str,
}
enum CustomName<'a> {
    First,
    Last(CustomNameLast<'a>),
}
enum DeepNested<'a, AGE> {
    Named { a: u32, b: u32 },
    Unnamed(u32, u32),
    None,
    NestedEnum(CustomName<'a>, u32),
    id(CustomName<'a>),
}
