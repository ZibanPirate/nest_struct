#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Nested {
    id: u32,
    name: nest! {
        first: String
        last: String,
    },
    nested_enum: nest! {
        First
        Last,
    },
}

fn main() {}
