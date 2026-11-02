#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct MultiNest<'a, T> {
    id: u32,
    // two nested types in the same field type, each using a different generic
    pair: Result<
        nest! {
            left: &'a str,
        },
        nest! {
            right: T,
        },
    >,
}
