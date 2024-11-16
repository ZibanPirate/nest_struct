#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Nested {
    id: u32,
    name: nest! {
        first: String,
        last: String,
    },
    // escaped `type` keyword
    r#type: nest! {
        value: String,
        group: String,
    },
    // @TODO-ZM: this is broken
    nested_enum: nest! {
        First,
        Last,
    },
}
