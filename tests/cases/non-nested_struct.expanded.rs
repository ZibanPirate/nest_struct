#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
#[allow(non_camel_case_types)]
struct NonNestedStruct {
    a: i32,
    b: Option<i32>,
    c: Vec<u32>,
}
