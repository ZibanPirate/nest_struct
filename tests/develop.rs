// This file is used as playground to develop the macro.
// run `bacon expand` to see the expanded code updated each time you change this file.

#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
pub struct NonNestedStruct<ID> {
    id: ID,
    name: nest! {
        first: String,
        last: String,
        middle: Option<String>,
        father: nest! {
            first: String,
            last: String,
            grand_father: nest! {
                first: String,
                last: String,
            },
        },
    },
    parent: nest_with_generic! {
        id: ID,
        age: u32,
    },
    past_names_with_generic: Vec<
        nest_with_generic! {
            id: ID,
            first: String,
            last: String,
            middle: Option<String>,
        },
    >,
    past_names: Vec<
        nest! {
            id: u32,
            first: String,
            last: String,
            middle: Option<String>,
        },
    >,
    status: Either<
        nest! {
            alive: bool,
            age: u8,
        },
        nest! {
            cause_of_death: String,
            date_of_death: String,
        },
    >,
}

enum Either<L, R> {
    Left(L),
    Right(R),
}
