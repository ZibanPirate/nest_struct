#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;

#[nest_struct]
struct Config<P, 'a> {
    version: &'a str,
    // auto-generated name is overwritten to be ConfigServer
    main_server: ConfigServer! {
        name: &'a str,
        host: &'a str,
        port: P,
        user: &'a str,
        password: &'a str,
    },
    // now we reuse the auto-generated name
    backup_server: ConfigServer<'a, P>,
}
