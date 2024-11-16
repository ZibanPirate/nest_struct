#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
#[derive(Debug)]
struct Generic<ID, 'a> {
    id: ID,
    // all generic items are used: 'a, ID
    name: nest! {
        first: &'a str,
        last: &'a str,
        middle: Option<&'a str>,
        family_name_count: ID,
    },
    // only one generic item is used: 'a
    family: nest! {
        roots: Vec<&'a str>,
    },
}
