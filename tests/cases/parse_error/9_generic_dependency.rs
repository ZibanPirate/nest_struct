#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Dep<T, U: Into<T>> {
    inner: nest! {
        value: U,
    },
}
