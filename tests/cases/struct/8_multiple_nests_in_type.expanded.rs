#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct MultiNestPair<'a> {
    left: &'a str,
}
struct MultiNestPair1<T> {
    right: T,
}
struct MultiNest<'a, T> {
    id: u32,
    pair: Result<MultiNestPair<'a>, MultiNestPair1<T>>,
}
