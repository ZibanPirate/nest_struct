#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Empty;

#[nest_struct]
struct EmptyBraces {}
