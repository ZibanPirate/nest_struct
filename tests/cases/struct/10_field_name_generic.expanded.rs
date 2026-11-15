#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct FieldNamedInner {
    T: String,
}
struct FieldNamed<T> {
    id: T,
    inner: FieldNamedInner,
}
