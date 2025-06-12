#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Nested {
    id: u32,
    name: nest! {
        struct CustomName {
            first: String,
            last: String,
        }

        struct SecondCustomName {
            first: String,
            last: String,
        }

        /// Doc comment for ThirdCustomName
        struct ThirdCustomName {
            first: String,
            last: String,
        }
    },
}

fn main() {}
