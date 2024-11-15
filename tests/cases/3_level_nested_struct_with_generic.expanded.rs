#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct Level3NestedStructWithGenericNameFatherGrandFather {
    age: AGE,
    first: String,
    last: String,
}
struct Level3NestedStructWithGenericNameFather {
    first: String,
    last: String,
    grand_father: Level3NestedStructWithGenericNameFatherGrandFather,
}
struct Level3NestedStructWithGenericName {
    first: String,
    last: String,
    middle: Option<String>,
    father: Level3NestedStructWithGenericNameFather,
}
struct Level3NestedStructWithGeneric<AGE> {
    id: u32,
    name: Level3NestedStructWithGenericName,
}
