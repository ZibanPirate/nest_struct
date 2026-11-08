#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct ConfigMainServer {}
struct Config<'a> {
    version: &'a str,
    main_server: ConfigMainServer,
}
