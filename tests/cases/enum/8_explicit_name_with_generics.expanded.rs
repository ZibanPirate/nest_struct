#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
enum Custom<T> {
    First(T),
    Last,
}
enum Container<T> {
    Value(Custom<T>),
    Empty,
}
