#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct ConfigMainServer {}
struct Config {
    version: &'a str,
    main_server: ConfigMainServer,
}
