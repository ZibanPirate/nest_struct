#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct MultiVariantPair {
    left: String,
}
struct MultiVariantPair1 {
    right: String,
}
enum MultiVariant {
    Pair(MultiVariantPair, MultiVariantPair1),
}
