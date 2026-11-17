#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
enum Container<T> {
    // explicit name, with the used generic auto-appended
    Value(Custom! {
        First(T),
        Last,
    }),
    Empty,
}
