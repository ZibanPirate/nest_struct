#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct NonNestedStruct {
    id: u32,
    name: nest! {
        first: String,
        last: String,
        middle: Option<String>,
    },
}
