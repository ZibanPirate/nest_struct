#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
#[derive(Debug)]
struct DeepNested<AGE, 'a> {
    id: u32,
    name: nest! {
        first: String,
        last: String,
        middle: Option<String>,
        father: nest! {
            first: String,
            last: String,
            // generic are only used in the last nest
            grand_father: nest! {
                age: AGE,
                first: &'a str,
                last: &'a str,
            },
        },
    },
}
