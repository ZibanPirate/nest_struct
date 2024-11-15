#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
#[allow(non_camel_case_types)]
struct Level1NestedStructWithGeneric_Name<ID> {
    first: String,
    last: String,
    middle: Option<String>,
    family_name_count: ID,
}
#[allow(non_camel_case_types)]
struct Level1NestedStructWithGeneric<ID> {
    id: ID,
    name: Level1NestedStructWithGeneric_Name<ID>,
}
