#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct NonNested {
    a: i32,
    b: Option<i32>,
    c: Vec<u32>,
}
