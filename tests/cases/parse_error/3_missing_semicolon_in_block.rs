#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct NestedBlock {
    id: u32,
    nested_block: nest! {
        enum NestedEnum {
            First
            Last,
        },
    },
}

fn main() {}
