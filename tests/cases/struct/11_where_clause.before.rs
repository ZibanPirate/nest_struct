#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Where<T>
where
    T: Clone,
{
    inner: nest! {
        value: T,
    },
}
