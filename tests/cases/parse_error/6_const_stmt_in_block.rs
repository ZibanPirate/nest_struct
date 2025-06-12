#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Nested {
    id: u32,
    name: nest! {
        const NAME: &str = "John Doe";
    },
}

fn main() {}
