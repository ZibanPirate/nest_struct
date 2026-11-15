#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Parenthesized {
    inner: Option<(nest! { value: String })>,
}
