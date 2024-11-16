#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
enum Nested {
    Named {
        a: u32,
        b: u32,
    },
    Unnamed(u32, u32),
    None,
    NestedEnum(
        nest! {
            First,
            Last,
        },
        u32,
        nest! {
            Second,
            Third,
        },
    ),
    NestedStruct(
        nest! {
            first: String,
            last: String,
        },
        u32,
    ),
}
