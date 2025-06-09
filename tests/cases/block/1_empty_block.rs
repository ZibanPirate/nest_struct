#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Config {
    version: &'a str,
    main_server: nest! {
        // nothing here
    },
}
