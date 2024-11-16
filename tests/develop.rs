// This file is used as playground to develop the macro.
// run `bacon expand` to see the expanded code updated each time you change this file.

#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
enum Enum {}

#[nest_struct]
struct Struct {}
