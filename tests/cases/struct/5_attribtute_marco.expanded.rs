#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct GenericName<'a, ID> {
    first: &'a str,
    last: &'a str,
    middle: Option<&'a str>,
    family_name_count: ID,
}
#[automatically_derived]
impl<'a, ID: ::core::fmt::Debug> ::core::fmt::Debug for GenericName<'a, ID> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "GenericName",
            "first",
            &self.first,
            "last",
            &self.last,
            "middle",
            &self.middle,
            "family_name_count",
            &&self.family_name_count,
        )
    }
}
struct GenericFamily<'a> {
    roots: Vec<&'a str>,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for GenericFamily<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "GenericFamily",
            "roots",
            &&self.roots,
        )
    }
}
struct Generic<'a, ID> {
    id: ID,
    name: GenericName<'a, ID>,
    family: GenericFamily<'a>,
}
#[automatically_derived]
impl<'a, ID: ::core::fmt::Debug> ::core::fmt::Debug for Generic<'a, ID> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Generic",
            "id",
            &self.id,
            "name",
            &self.name,
            "family",
            &&self.family,
        )
    }
}
