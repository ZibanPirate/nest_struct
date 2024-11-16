#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct NestedName {
    first: String,
    last: String,
}
struct NestedType {
    value: String,
    group: String,
}
struct Nested {
    id: u32,
    name: NestedName,
    r#type: NestedType,
}
