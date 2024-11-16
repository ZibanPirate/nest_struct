#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct Level1NestedStructWithGenericAndLifetimeName<'a, ID> {
    first: &'a str,
    last: &'a str,
    middle: Option<&'a str>,
    family_name_count: ID,
}
struct Level1NestedStructWithGenericAndLifetimeFamily<'a> {
    roots: Vec<&'a str>,
}
struct Level1NestedStructWithGenericAndLifetime<'a, ID> {
    id: ID,
    name: Level1NestedStructWithGenericAndLifetimeName<'a, ID>,
    family: Level1NestedStructWithGenericAndLifetimeFamily<'a>,
}
