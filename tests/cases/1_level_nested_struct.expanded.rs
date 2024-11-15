#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct Level1NestedStructType {
    value: String,
    group: String,
}
struct Level1NestedStruct {
    id: u32,
    r#type: Level1NestedStructType,
}
