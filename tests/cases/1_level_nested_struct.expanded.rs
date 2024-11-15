#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct Level1NestedStruct_Name {
    first: String,
    last: String,
    middle: Option<String>,
}
struct Level1NestedStruct {
    id: u32,
    name: Level1NestedStruct_Name,
}
