#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct FieldNamed<T> {
    id: T,
    inner: nest! {
        T: String,
    },
}
