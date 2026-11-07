#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct CustomNameLast<'a> {
    first: &'a str,
    last: &'a str,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for CustomNameLast<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CustomNameLast",
            "first",
            &self.first,
            "last",
            &&self.last,
        )
    }
}
/// Doc comment for CustomName
enum CustomName<'a> {
    First,
    Last(CustomNameLast<'a>),
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for CustomName<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            CustomName::First => ::core::fmt::Formatter::write_str(f, "First"),
            CustomName::Last(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Last", &__self_0)
            }
        }
    }
}
enum DeepNested<'a> {
    Named { a: u32, b: u32 },
    Unnamed(u32, u32),
    None,
    NestedEnum(CustomName<'a>, u32),
    Id(CustomName<'a>),
}
