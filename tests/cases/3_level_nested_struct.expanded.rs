#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
#[allow(non_camel_case_types)]
struct Level3NestedStruct_Name_Father_GrandFather {
    first: String,
    last: String,
}
#[allow(non_camel_case_types)]
struct Level3NestedStruct_Name_Father {
    first: String,
    last: String,
    grand_father: Level3NestedStruct_Name_Father_GrandFather,
}
#[allow(non_camel_case_types)]
struct Level3NestedStruct_Name {
    first: String,
    last: String,
    middle: Option<String>,
    father: Level3NestedStruct_Name_Father,
}
#[allow(non_camel_case_types)]
struct Level3NestedStruct {
    id: u32,
    name: Level3NestedStruct_Name,
}
