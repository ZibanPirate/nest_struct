#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
#[allow(non_camel_case_types)]
struct Level1NestedStruct_Type {
    value: String,
    group: String,
}
#[allow(non_camel_case_types)]
struct Level1NestedStruct {
    id: u32,
    r#type: Level1NestedStruct_Type,
}
