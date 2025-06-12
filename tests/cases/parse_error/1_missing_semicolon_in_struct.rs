#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct NestedStruct {
    id: u32,
    name: nest! {
        first: String
        last: String,
    },
}

fn main() {}
