#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
enum MultiVariant {
    // two nests in two different tuple-variant fields: the field position is
    // part of the generated name, so these must not collide
    Pair(
        nest! {
            left: String,
        },
        nest! {
            right: String,
        },
    ),
}
