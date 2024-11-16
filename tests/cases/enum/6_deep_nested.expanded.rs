#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct DeepNestedNestedEnumLast<'a> {
    first: &'a str,
    last: &'a str,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for DeepNestedNestedEnumLast<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "DeepNestedNestedEnumLast",
            "first",
            &self.first,
            "last",
            &&self.last,
        )
    }
}
enum DeepNestedNestedEnum<'a> {
    First,
    Last(DeepNestedNestedEnumLast<'a>),
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for DeepNestedNestedEnum<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DeepNestedNestedEnum::First => ::core::fmt::Formatter::write_str(f, "First"),
            DeepNestedNestedEnum::Last(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Last", &__self_0)
            }
        }
    }
}
enum DeepNested<'a, AGE> {
    Named { a: u32, b: u32 },
    Unnamed(u32, u32),
    None,
    NestedEnum(DeepNestedNestedEnum<'a>, u32),
    id(ID),
}
#[automatically_derived]
impl<'a, AGE: ::core::fmt::Debug> ::core::fmt::Debug for DeepNested<'a, AGE> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DeepNested::Named { a: __self_0, b: __self_1 } => {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Named",
                    "a",
                    __self_0,
                    "b",
                    &__self_1,
                )
            }
            DeepNested::Unnamed(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Unnamed",
                    __self_0,
                    &__self_1,
                )
            }
            DeepNested::None => ::core::fmt::Formatter::write_str(f, "None"),
            DeepNested::NestedEnum(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "NestedEnum",
                    __self_0,
                    &__self_1,
                )
            }
            DeepNested::id(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "id", &__self_0)
            }
        }
    }
}
