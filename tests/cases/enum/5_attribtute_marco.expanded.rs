#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
enum AttributeMacroNestedEnum {
    First,
    Last,
}
#[automatically_derived]
impl ::core::fmt::Debug for AttributeMacroNestedEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                AttributeMacroNestedEnum::First => "First",
                AttributeMacroNestedEnum::Last => "Last",
            },
        )
    }
}
struct AttributeMacroNestedStruct {
    first: String,
    last: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for AttributeMacroNestedStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "AttributeMacroNestedStruct",
            "first",
            &self.first,
            "last",
            &&self.last,
        )
    }
}
enum AttributeMacro {
    Named { a: u32, b: u32 },
    Unnamed(u32, u32),
    None,
    NestedEnum(AttributeMacroNestedEnum, u32),
    NestedStruct(AttributeMacroNestedStruct, u32),
}
#[automatically_derived]
impl ::core::fmt::Debug for AttributeMacro {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            AttributeMacro::Named { a: __self_0, b: __self_1 } => {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Named",
                    "a",
                    __self_0,
                    "b",
                    &__self_1,
                )
            }
            AttributeMacro::Unnamed(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Unnamed",
                    __self_0,
                    &__self_1,
                )
            }
            AttributeMacro::None => ::core::fmt::Formatter::write_str(f, "None"),
            AttributeMacro::NestedEnum(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "NestedEnum",
                    __self_0,
                    &__self_1,
                )
            }
            AttributeMacro::NestedStruct(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "NestedStruct",
                    __self_0,
                    &__self_1,
                )
            }
        }
    }
}
