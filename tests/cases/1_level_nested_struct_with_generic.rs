#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Level1NestedStructWithGeneric<ID> {
    id: ID,
    name: nest_with_generic! {
        first: String,
        last: String,
        middle: Option<String>,
        family_name_count: ID,
    },
}
