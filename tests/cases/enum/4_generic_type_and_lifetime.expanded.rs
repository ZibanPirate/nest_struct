#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
enum GenericNestedEnum {
    First,
    Last,
}
struct GenericNestedStruct<'a> {
    first: &'a str,
    last: &'a str,
}
enum Generic<'a, ID> {
    Named { a: u32, b: u32 },
    Unnamed(u32, u32),
    None,
    NestedEnum(GenericNestedEnum, ID),
    NestedStruct(GenericNestedStruct<'a>, ID),
    id(ID),
}
