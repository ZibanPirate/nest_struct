#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct ConfigServer<'a, P> {
    name: &'a str,
    host: &'a str,
    port: P,
    user: &'a str,
    password: &'a str,
}
struct Config<'a, P> {
    version: &'a str,
    main_server: ConfigServer<'a, P>,
    backup_server: ConfigServer<'a, P>,
}
