#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Nested {
    id: u32,
    name: nest! {
        some_func_call("test", 123);
    },
}

fn main() {}
