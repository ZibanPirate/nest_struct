#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct GenericName<'a, ID> {
    first: &'a str,
    last: &'a str,
    middle: Option<&'a str>,
    family_name_count: ID,
}
struct GenericFamily<'a> {
    roots: Vec<&'a str>,
}
struct Generic<'a, ID> {
    id: ID,
    name: GenericName<'a, ID>,
    family: GenericFamily<'a>,
}
