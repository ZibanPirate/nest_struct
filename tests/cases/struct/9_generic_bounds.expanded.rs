#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct BoundedInner<T: Clone> {
    value: T,
}
struct Bounded<T: Clone> {
    inner: BoundedInner<T>,
}
struct DefaultedInner<T> {
    value: T,
}
struct Defaulted<T> {
    inner: DefaultedInner<T>,
}
struct ConstGenericInner<const N: usize> {
    value: [u8; N],
}
struct ConstGeneric<const N: usize> {
    inner: ConstGenericInner<N>,
}
