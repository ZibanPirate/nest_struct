#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct BlockEnumWrapper {
    value: nest! {
        /// A block-defined enum
        enum BlockEnum {
            First,
            Last,
        }
    },
}
