#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
enum DeepNested<AGE, 'a> {
    Named {
        a: u32,
        b: u32,
    },
    Unnamed(u32, u32),
    None,
    NestedEnum(
        // auto-generated name is overwritten to be CustomName
        nest! {
            #[Derive(Debug)]
            /// Doc comment for CustomName
            enum CustomName {
                First,
                Last (nest! {
                    // generic are only used in the last nest
                    first: &'a str,
                    last: &'a str,
                }),
            }
        },
        u32,
    ),
    // now we reuse the auto-generated name
    id(CustomName<'a>),
}
