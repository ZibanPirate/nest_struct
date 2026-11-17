#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
/// A block-defined enum
enum BlockEnum {
    First,
    Last,
}
struct BlockEnumWrapper {
    value: BlockEnum,
}
