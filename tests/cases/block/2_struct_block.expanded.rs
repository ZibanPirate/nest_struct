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
#[automatically_derived]
impl<'a, P: ::core::fmt::Debug> ::core::fmt::Debug for ConfigServer<'a, P> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "ConfigServer",
            "name",
            &self.name,
            "host",
            &self.host,
            "port",
            &self.port,
            "user",
            &self.user,
            "password",
            &&self.password,
        )
    }
}
struct Config<'a, P> {
    version: &'a str,
    main_server: ConfigServer<'a, P>,
    backup_server: ConfigServer<'a, P>,
}
