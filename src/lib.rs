#![warn(missing_docs)]

//! Nest struct and enum definitions with minimal syntax changes in Rust
//!
//! ## Example
//!
//! ```rust
//! use nest_struct::nest_struct;
//!
//! #[nest_struct]
//! struct Post {
//!     title: String,
//!     summary: String,
//!     author: nest! {
//!         name: String,
//!         handle: String,
//!     },
//! }
//! ```
//!
//! <details>
//!   <summary>See expanded code</summary>
//!
//! ```rust
//! struct Post {
//!     title: String,
//!     summary: String,
//!     author: PostAuthor,
//! }
//!
//! struct PostAuthor {
//!     name: String,
//!     handle: String,
//! }
//! ```
//!
//! </details>
//! <br>
//!
//! You can also overwrite inner struct name, by passing the name itself as macro instead of `nest!`:
//!
//! ```rust
//! use nest_struct::nest_struct;
//!
//! #[nest_struct]
//! struct Post {
//!     title: String,
//!     summary: String,
//!     author: Author! {
//!         name: String,
//!         handle: String,
//!     },
//! }
//! ```
//!
//! <details>
//!  <summary>See expanded code</summary>
//!
//! ```rust
//! struct Post {
//!     title: String,
//!     summary: String,
//!     author: Author,
//! }
//!
//! struct Author {
//!     name: String,
//!     handle: String,
//! }
//! ```
//!
//! </details>
//! <br>
//!
//! <details>
//!  <summary>Another example calling Pokemon API</summary>
//!
//! ```rust
//! use nest_struct::nest_struct;
//!
//! // Define a struct with nested struct definitions all in one place
//! // with minimal syntax changes.
//! #[nest_struct]
//! #[derive(serde::Deserialize)]
//! struct APIResponse {
//!     id: u32,
//!     name: String,
//!     abilities: Vec<nest! {
//!             ability: nest! { name: String, url: String },
//!             is_hidden: bool,
//!             slot: u32,
//!         },
//!     >,
//! }
//!
//! let body = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon/ditto").unwrap().text().unwrap();
//! let api_response: APIResponse = serde_json::from_str(&body).unwrap();
//!
//! assert_eq!(api_response.name, "ditto");
//! // Access nested struct fields
//! assert_eq!(api_response.abilities.first().unwrap().ability.name, "limber");
//! ```
//!
//! </details>
//! <br>
//!
//! For more examples, see the [`./tests/cases`](https://github.com/ZibanPirate/nest_struct/tree/main/tests/cases) directory.
//!
//! ## Features
//!
//! -   [x] deep nesting (no theoretical limit).
//! -   [x] nest `struct` inside another `struct`.
//! -   [x] nest `enum` inside another `enum`.
//! -   [x] nest `enum` inside a `struct` and vice-versa.
//! -   [x] inherit `derive` and other attribute macros from root `struct`.
//! -   [x] auto-generate inner `struct` names.
//! -   [x] overwrite the auto-generated inner struct name.
//!
//! Feature parity with native Rust code:
//!
//! -   [x] `impl` block on inner `struct`s.
//! -   [ ] define `derive` and other attribute macros individually per inner `struct`.
//! -   [ ] define doc comments individually per inner `struct`.
//! -   [ ] useful compiler error messages.
//! -   [x] support generic types.
//! -   [x] support lifetimes.

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Fields,
    FieldsNamed, FieldsUnnamed, Generics, Type,
};

fn find_idents_in_token_tree_and_exit_early(
    token_stream: proc_macro2::TokenStream,
    ident_names: &Vec<String>,
) -> Vec<String> {
    let mut idents: Vec<String> = vec![];

    token_stream.into_iter().for_each(|token| match token {
        TokenTree::Ident(ident) => {
            if ident_names.contains(&ident.to_string()) {
                idents.push(ident.to_string());
            }
        }
        TokenTree::Group(group) => {
            idents.extend(find_idents_in_token_tree_and_exit_early(
                group.stream(),
                ident_names,
            ));
        }
        _ => {}
    });

    idents.dedup();

    // @TODO-ZM: preserve order of found idents
    idents
}

#[derive(Debug)]
enum BodyType {
    Struct,
    Enum,
}

/// Nest struct definitions with minimal syntax changes.
/// eg:
/// ```rust
/// use nest_struct::nest_struct;
/// use serde::Deserialize;
///
/// #[nest_struct]
/// #[derive(Deserialize)]
/// pub struct AIResponse {
///   choices: Vec<nest! { message: nest!{ content: String } }>,
/// }
/// ```
#[proc_macro_attribute]
pub fn nest_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_item = item.clone();
    let input = parse_macro_input!(item as DeriveInput);

    let root_struct_ident = &input.ident;
    let root_vis = &input.vis;
    let root_attrs = input.attrs;
    let root_generics = input.generics;

    match input.data {
        Data::Struct(root_struct_body) => {
            let root_fields = match root_struct_body.fields {
                Fields::Named(fields) => fields.named,
                _ => return original_item,
            };

            let (additional_structs, new_root_fields) = match convert_nest_to_structs(
                root_fields,
                root_struct_ident,
                None,
                &root_generics,
                root_vis,
                &root_attrs,
            ) {
                Ok(tuple) => tuple,
                Err(_) => return original_item,
            };

            let expanded = quote! {
                #(#additional_structs)*

                #(#root_attrs)*
                #root_vis struct #root_struct_ident #root_generics {
                    #(#new_root_fields),*
                }
            };

            TokenStream::from(expanded)
        }
        Data::Enum(root_enum_body) => {
            let root_enum_variants = root_enum_body.variants;
            let mut additional_structs = vec![];
            let mut new_enum_variants = vec![];

            for mut variant in root_enum_variants {
                let variant_fields = match variant.clone().fields {
                    Fields::Named(fields) => fields.named,
                    Fields::Unnamed(fields) => fields.unnamed,
                    _ => {
                        new_enum_variants.push(variant);
                        continue;
                    }
                };

                let (additional_structs_for_variant, new_variant_fields) =
                    match convert_nest_to_structs(
                        variant_fields,
                        root_struct_ident,
                        Some(&variant.ident),
                        &root_generics,
                        root_vis,
                        &root_attrs,
                    ) {
                        Ok(tuple) => tuple,
                        Err(_) => return original_item,
                    };

                variant.fields = match variant.fields {
                    Fields::Named(fields_named) => Fields::Named(FieldsNamed {
                        brace_token: fields_named.brace_token,
                        named: Punctuated::from_iter(new_variant_fields),
                    }),
                    Fields::Unnamed(fields_unnamed) => Fields::Unnamed(FieldsUnnamed {
                        paren_token: fields_unnamed.paren_token,
                        unnamed: Punctuated::from_iter(new_variant_fields),
                    }),
                    _ => {
                        panic!("Should not reach here");
                    }
                };

                additional_structs.extend(additional_structs_for_variant);
                new_enum_variants.push(variant);
            }

            let expanded = quote! {
                #(#additional_structs)*

                #(#root_attrs)*
                #root_vis enum #root_struct_ident #root_generics {
                    #(#new_enum_variants),*
                }
            };

            TokenStream::from(expanded)
        }
        _ => original_item,
    }
}

fn convert_nest_to_structs(
    fields: Punctuated<Field, Comma>,
    root_struct_ident: &syn::Ident,
    middle_ident: Option<&syn::Ident>,
    root_generics: &Generics,
    root_vis: &syn::Visibility,
    root_attrs: &Vec<syn::Attribute>,
) -> Result<(Vec<proc_macro2::TokenStream>, Vec<syn::Field>), ()> {
    let root_struct_name = format!(
        "{}{}",
        root_struct_ident,
        match middle_ident {
            Some(ident) => ident.to_string().to_case(Case::Pascal),
            None => "".to_string(),
        }
    );
    let root_generic_names = root_generics
        .clone()
        .into_token_stream()
        .into_iter()
        .filter_map(|token| match token {
            TokenTree::Ident(ident) => Some(ident.to_string()),
            _ => None,
        })
        .collect::<Vec<String>>();

    let mut new_root_fields: Vec<syn::Field> = Vec::new();
    let mut additional_structs: Vec<proc_macro2::TokenStream> = vec![];

    let mut field_name_index = 0;
    for mut field in fields {
        let field_name = match field.ident {
            Some(ref ident) => ident.to_string(),
            None => {
                let name = match field_name_index {
                    0 => "".to_string(),
                    index => format!("{}", index),
                }
                .to_string();
                field_name_index += 1;
                name
            }
        };

        let mut token_tree = field
            .ty
            .clone()
            .into_token_stream()
            .into_iter()
            .collect::<Vec<TokenTree>>();

        let mut indices_to_remove: Vec<usize> = vec![];
        let mut indices_to_replace: Vec<(usize, TokenTree)> = vec![];

        let mut index = 0;
        while index < token_tree.len() {
            // find all token trees combo `[ident=nest][punct=!][group]` which means find all `nest! { ... }`
            // patterns, this way we handle the case where nest! is used as a generic type, e.g. Vec<nest!{ ... }>
            // or even used multiple times in a single field, e.g. Either<nest!{ ... }, nest!{ ... }>
            let (ident, punct, group) = (
                token_tree.get(index),
                token_tree.get(index + 1),
                token_tree.get(index + 2),
            );
            match (ident.clone(), punct, group) {
                (
                    Some(TokenTree::Ident(ident)),
                    Some(TokenTree::Punct(punct)),
                    Some(TokenTree::Group(group)),
                ) => {
                    let ident_str = ident.to_string();
                    if (ident_str == "nest" || ident_str.is_case(Case::Pascal))
                        && punct.as_char() == '!'
                    {
                        let inner_struct_name = match ident_str.is_case(Case::Pascal) {
                            true => syn::Ident::new(&ident_str, proc_macro2::Span::call_site()),
                            false => {
                                let struct_name_index = match indices_to_replace.len() {
                                    0 => "",
                                    n => &n.to_string(),
                                };
                                let struct_name_maybe_numbered = format!(
                                    "{}{}{}",
                                    root_struct_name,
                                    field_name.replace("r#", "").to_case(Case::Pascal),
                                    struct_name_index
                                );
                                syn::Ident::new(
                                    &struct_name_maybe_numbered,
                                    proc_macro2::Span::call_site(),
                                )
                            }
                        };

                        let body_type =
                            match syn::parse2::<DeriveInput>(quote! { struct Foo #group }.into()) {
                                Ok(_) => BodyType::Struct,
                                Err(_) => match syn::parse2::<DeriveInput>(
                                    quote! { enum Foo #group }.into(),
                                ) {
                                    Ok(_) => BodyType::Enum,
                                    // in case of error, we print struct error not enum error
                                    // @TODO-ZM: return parsing error
                                    Err(_) => return Err(()),
                                },
                            };

                        let body_type_syn = match body_type {
                            BodyType::Struct => quote! { struct },
                            BodyType::Enum => quote! { enum },
                        };

                        let found_ident_names_for_generics =
                            find_idents_in_token_tree_and_exit_early(
                                group.stream(),
                                &root_generic_names,
                            );

                        // clone and reconstruct the root generics for the new struct, cherry-picking only the generics
                        // that are used in the nested struct, identified by their names
                        let mut struct_generic = root_generics.clone();
                        struct_generic.params = struct_generic
                            .params
                            .into_iter()
                            .filter(|param| {
                                param.into_token_stream().to_token_stream().into_iter().any(
                                    |token| match token {
                                        TokenTree::Ident(ident) => found_ident_names_for_generics
                                            .contains(&ident.to_string()),
                                        _ => false,
                                    },
                                )
                            })
                            .collect();

                        let generic = quote! { #struct_generic };

                        let inner_struct_name_maybe_with_generic =
                            syn::parse_str::<Type>(&format!("{}{}", inner_struct_name, generic))
                                .unwrap();

                        indices_to_replace.push((
                            index,
                            TokenTree::Group(proc_macro2::Group::new(
                                proc_macro2::Delimiter::None,
                                inner_struct_name_maybe_with_generic.into_token_stream(),
                            )),
                        ));
                        indices_to_remove.push(index + 1);
                        indices_to_remove.push(index + 2);

                        additional_structs.push(quote! {
                            #[nest_struct]
                            #(#root_attrs)*
                            #root_vis #body_type_syn #inner_struct_name #generic #group
                        });

                        index += 2;
                    }
                }
                _ => {}
            }

            index += 1;
        }

        // replace `nest` with struct_name_field_name
        for (index, token) in indices_to_replace {
            token_tree.remove(index);
            token_tree.insert(index, token);
        }
        // and remove `!` and `{ ... }`, starting from the last index and back to avoid index shifting
        indices_to_remove.reverse();
        for index in indices_to_remove {
            token_tree.remove(index);
        }

        field.ty = syn::parse2(quote! { #(#token_tree)* }).unwrap();
        new_root_fields.push(field);
    }

    Ok((additional_structs, new_root_fields))
}
