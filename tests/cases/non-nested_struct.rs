#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct NonNestedStruct {
    a: i32,
    b: Option<i32>,
    c: Vec<u32>,
}
