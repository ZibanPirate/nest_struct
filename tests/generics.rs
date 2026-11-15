#![allow(dead_code)]

use nest_struct::nest_struct;

#[nest_struct]
struct Bounded<T: Clone> {
    inner: nest! {
        value: T,
    },
}

#[nest_struct]
struct Defaulted<T = String> {
    inner: nest! {
        value: T,
    },
}

#[nest_struct]
struct ConstGeneric<const N: usize> {
    inner: nest! {
        value: [u8; N],
    },
}
