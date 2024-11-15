#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct Level3NestedStructNameFatherGrandFather {
    first: String,
    last: String,
}
struct Level3NestedStructNameFather {
    first: String,
    last: String,
    grand_father: Level3NestedStructNameFatherGrandFather,
}
struct Level3NestedStructName {
    first: String,
    last: String,
    middle: Option<String>,
    father: Level3NestedStructNameFather,
}
struct Level3NestedStruct {
    id: u32,
    name: Level3NestedStructName,
}
