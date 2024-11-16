#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
enum NestedNestedEnum {
    First,
    Last,
}
enum NestedNestedEnum2 {
    Second,
    Third,
}
struct NestedNestedStruct {
    first: String,
    last: String,
}
enum Nested {
    Named { a: u32, b: u32 },
    Unnamed(u32, u32),
    None,
    NestedEnum(NestedNestedEnum, u32, NestedNestedEnum2),
    NestedStruct(NestedNestedStruct, u32),
}
