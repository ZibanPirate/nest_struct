#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct NestedEnum {
    id: u32,
    nested_enum: nest! {
        First
        Last,
    },
}

fn main() {}
