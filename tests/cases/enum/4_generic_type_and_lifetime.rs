#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
enum Generic<ID, 'a> {
    Named {
        a: u32,
        b: u32,
    },
    Unnamed(u32, u32),
    None,
    // only one generic item is used: 'a
    NestedEnum(
        // @TODO-ZM: this is broken
        nest! {
            First,
            Last,
        },
        ID,
    ),
    // all generic items are used: 'a, ID
    NestedStruct(
        // @TODO-ZM: this is broken
        nest! {
            first: &'a str,
            last: &'a str,
        },
        ID,
    ),
    id(ID),
}
