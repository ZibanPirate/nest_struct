#![allow(dead_code)]
#[macro_use]
extern crate nest_struct;
struct DeepNestedWithGenericNameFatherGrandFather<'a, AGE> {
    age: AGE,
    first: &'a str,
    last: &'a str,
}
#[automatically_derived]
impl<'a, AGE: ::core::fmt::Debug> ::core::fmt::Debug
for DeepNestedWithGenericNameFatherGrandFather<'a, AGE> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "DeepNestedWithGenericNameFatherGrandFather",
            "age",
            &self.age,
            "first",
            &self.first,
            "last",
            &&self.last,
        )
    }
}
struct DeepNestedWithGenericNameFather<'a, AGE> {
    first: String,
    last: String,
    grand_father: DeepNestedWithGenericNameFatherGrandFather<'a, AGE>,
}
#[automatically_derived]
impl<'a, AGE: ::core::fmt::Debug> ::core::fmt::Debug
for DeepNestedWithGenericNameFather<'a, AGE> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "DeepNestedWithGenericNameFather",
            "first",
            &self.first,
            "last",
            &self.last,
            "grand_father",
            &&self.grand_father,
        )
    }
}
struct DeepNestedWithGenericName<'a, AGE> {
    first: String,
    last: String,
    middle: Option<String>,
    father: DeepNestedWithGenericNameFather<'a, AGE>,
}
#[automatically_derived]
impl<'a, AGE: ::core::fmt::Debug> ::core::fmt::Debug
for DeepNestedWithGenericName<'a, AGE> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "DeepNestedWithGenericName",
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
struct DeepNestedWithGeneric<'a, AGE> {
    id: u32,
    name: DeepNestedWithGenericName<'a, AGE>,
}
#[automatically_derived]
impl<'a, AGE: ::core::fmt::Debug> ::core::fmt::Debug for DeepNestedWithGeneric<'a, AGE> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "DeepNestedWithGeneric",
            "id",
            &self.id,
            "name",
            &&self.name,
        )
    }
}
