#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Level3NestedStruct {
    id: u32,
    name: nest! {
        first: String,
        last: String,
        middle: Option<String>,
        father: nest! {
            first: String,
            last: String,
            grand_father: nest! {
                first: String,
                last: String,
            },
        },
    },
}
