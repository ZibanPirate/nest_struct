#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
#[allow(non_camel_case_types)]
struct Level1NestedStruct_Name_Father_GrandFather {
    first: String,
    last: String,
}
#[allow(non_camel_case_types)]
struct Level1NestedStruct_Name_Father {
    first: String,
    last: String,
    grand_father: Level1NestedStruct_Name_Father_GrandFather,
}
#[allow(non_camel_case_types)]
struct Level1NestedStruct_Name {
    first: String,
    last: String,
    middle: Option<String>,
    father: Level1NestedStruct_Name_Father,
}
#[allow(non_camel_case_types)]
struct Level1NestedStruct {
    id: u32,
    name: Level1NestedStruct_Name,
}
