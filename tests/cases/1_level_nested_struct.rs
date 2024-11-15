#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Level1NestedStruct {
    id: u32,
    r#type: nest! {
        value: String,
        group: String,
    },
}
