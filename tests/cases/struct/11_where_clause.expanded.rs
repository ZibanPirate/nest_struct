#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct WhereInner<T>
where
    T: Clone,
{
    value: T,
}
struct Where<T>
where
    T: Clone,
{
    inner: WhereInner<T>,
}
