#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct AttributeMacroName {
    first: String,
    last: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for AttributeMacroName {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "AttributeMacroName",
            "first",
            &self.first,
            "last",
            &&self.last,
        )
    }
}
struct AttributeMacroType {
    value: String,
    group: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for AttributeMacroType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "AttributeMacroType",
            "value",
            &self.value,
            "group",
            &&self.group,
        )
    }
}
struct AttributeMacro {
    id: u32,
    name: AttributeMacroName,
    r#type: AttributeMacroType,
}
#[automatically_derived]
impl ::core::fmt::Debug for AttributeMacro {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "AttributeMacro",
            "id",
            &self.id,
            "name",
            &self.name,
            "type",
            &&self.r#type,
        )
    }
}
