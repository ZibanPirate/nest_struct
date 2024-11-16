#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct DeepNestedNameFatherGrandFather<'a, AGE> {
    age: AGE,
    first: &'a str,
    last: &'a str,
}
#[automatically_derived]
impl<'a, AGE: ::core::fmt::Debug> ::core::fmt::Debug
for DeepNestedNameFatherGrandFather<'a, AGE> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "DeepNestedNameFatherGrandFather",
            "age",
            &self.age,
            "first",
            &self.first,
            "last",
            &&self.last,
        )
    }
}
struct DeepNestedNameFather<'a, AGE> {
    first: String,
    last: String,
    grand_father: DeepNestedNameFatherGrandFather<'a, AGE>,
}
#[automatically_derived]
impl<'a, AGE: ::core::fmt::Debug> ::core::fmt::Debug for DeepNestedNameFather<'a, AGE> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "DeepNestedNameFather",
            "first",
            &self.first,
            "last",
            &self.last,
            "grand_father",
            &&self.grand_father,
        )
    }
}
struct DeepNestedName<'a, AGE> {
    first: String,
    last: String,
    middle: Option<String>,
    father: DeepNestedNameFather<'a, AGE>,
}
#[automatically_derived]
impl<'a, AGE: ::core::fmt::Debug> ::core::fmt::Debug for DeepNestedName<'a, AGE> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "DeepNestedName",
            "first",
            &self.first,
            "last",
            &self.last,
            "middle",
            &self.middle,
            "father",
            &&self.father,
        )
    }
}
struct DeepNested<'a, AGE> {
    id: u32,
    name: DeepNestedName<'a, AGE>,
}
#[automatically_derived]
impl<'a, AGE: ::core::fmt::Debug> ::core::fmt::Debug for DeepNested<'a, AGE> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "DeepNested",
            "id",
            &self.id,
            "name",
            &&self.name,
        )
    }
}
