#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct Level1NestedStructWithGenericName<ID> {
    first: String,
    last: String,
    middle: Option<String>,
    family_name_count: ID,
}
struct Level1NestedStructWithGeneric<ID> {
    id: ID,
    name: Level1NestedStructWithGenericName<ID>,
}
