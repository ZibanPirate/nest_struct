#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct Level3NestedStructWithGenericNameFatherGrandFather<AGE> {
    age: AGE,
    first: String,
    last: String,
}
struct Level3NestedStructWithGenericNameFather<AGE> {
    first: String,
    last: String,
    grand_father: Level3NestedStructWithGenericNameFatherGrandFather<AGE>,
}
struct Level3NestedStructWithGenericName<AGE> {
    first: String,
    last: String,
    middle: Option<String>,
    father: Level3NestedStructWithGenericNameFather<AGE>,
}
struct Level3NestedStructWithGeneric<AGE> {
    id: u32,
    name: Level3NestedStructWithGenericName<AGE>,
}
