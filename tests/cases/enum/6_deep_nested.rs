#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
#[derive(Debug)]
enum DeepNested<AGE, 'a> {
    Named {
        a: u32,
        b: u32,
    },
    Unnamed(u32, u32),
    None,
    // only one generic item is used: 'a
    NestedEnum(
        nest! {
            First,
            Last (nest! {
                // generic are only used in the last nest
                first: &'a str,
                last: &'a str,
            }),
        },
        u32,
    ),
    id(ID),
}
