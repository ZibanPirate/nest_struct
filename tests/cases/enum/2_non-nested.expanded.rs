#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
enum NonNested {
    Named { a: u32, b: u32 },
    Unnamed(u32, u32),
    None,
}
