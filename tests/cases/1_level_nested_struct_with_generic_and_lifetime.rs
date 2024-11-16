#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Level1NestedStructWithGenericAndLifetime<ID, 'a> {
    id: ID,
    name: nest! {
        first: &'a str,
        last: &'a str,
        middle: Option<&'a str>,
        family_name_count: ID,
    },
    family: nest! {
        roots: Vec<&'a str>,
    },
}
