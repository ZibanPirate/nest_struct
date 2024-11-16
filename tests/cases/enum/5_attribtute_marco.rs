#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
#[derive(Debug)]
enum AttributeMacro {
    Named {
        a: u32,
        b: u32,
    },
    Unnamed(u32, u32),
    None,
    NestedEnum(
        // @TODO-ZM: this is broken
        nest! {
            First,
            Last,
        },
        u32,
    ),
    NestedStruct(
        // @TODO-ZM: this is broken
        nest! {
            first: String,
            last: String,
        },
        u32,
    ),
}
