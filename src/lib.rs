#![warn(missing_docs)]

//! Nest struct definitions with minimal syntax changes in Rust
//!
//! Example
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
//!         ability: nest! {
//!             name: String,
//!             url: String,
//!         },
//!         is_hidden: bool,
//!         slot: u32,
//!     }>
//! };
//!
//! let body = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon/ditto").unwrap().text().unwrap();
//! let api_response: APIResponse = serde_json::from_str(&body).unwrap();
//!
//! assert_eq!(api_response.name, "ditto");
//! // Access nested struct fields
//! assert_eq!(api_response.abilities.first().unwrap().ability.name, "limber");
//! ```
//!
//! The expanded code for the struct above would look like this:
//!
//! ```rust
//! #[derive(serde::Deserialize)]
//! struct APIResponseAbilitiesAbility {
//!     name: String,
//!     url: String,
//! }
//!
//! #[derive(serde::Deserialize)]
//! struct APIResponseAbilities {
//!     ability: APIResponseAbilitiesAbility,
//!     is_hidden: bool,
//!     slot: u32,
//! }
//!
//! #[derive(serde::Deserialize)]
//! struct APIResponse {
//!     id: u32,
//!     name: String,
//!     abilities: Vec<APIResponseAbilities>,
//! }
//! ```
//!
//! for more examples, see the `tests/cases` directory.
//!

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

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

/// Nest struct definitions with minimal syntax changes.
#[proc_macro_attribute]
pub fn nest_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_item = item.clone();
    let input = parse_macro_input!(item as DeriveInput);

    let root_struct_ident = &input.ident;
    let root_struct_name = root_struct_ident.to_string();
    let root_vis = &input.vis;
    let root_attrs = input.attrs;
    let root_generics = input.generics;
    // @TODO-ZM: should we preserve order of generics?
    // get only the generic names without the extra info, eg 'a will be just a
    let root_generic_names = root_generics
        .clone()
        .into_token_stream()
        .into_iter()
        .filter_map(|token| match token {
            TokenTree::Ident(ident) => Some(ident.to_string()),
            _ => None,
        })
        .collect::<Vec<String>>();

    let root_struct_body = match input.data {
        Data::Struct(data) => data,
        _ => return original_item,
    };

    let root_fields = match root_struct_body.fields {
        Fields::Named(fields) => fields.named,
        _ => return original_item,
    };

    let mut new_root_fields: Vec<syn::Field> = Vec::new();
    let mut additional_structs: Vec<proc_macro2::TokenStream> = vec![];

    for mut field in root_fields {
        let field_name = field.ident.clone().unwrap().to_string();
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
                    if (ident.to_string() == "nest") && punct.as_char() == '!' {
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
                        let struct_ident_maybe_numbered = syn::Ident::new(
                            &struct_name_maybe_numbered,
                            proc_macro2::Span::call_site(),
                        );

                        let generic = quote! { #struct_generic };

                        let struct_name_maybe_numbered_maybe_with_generic = syn::parse_str::<Type>(
                            &format!("{}{}", struct_name_maybe_numbered, generic),
                        )
                        .unwrap();

                        indices_to_replace.push((
                            index,
                            TokenTree::Group(proc_macro2::Group::new(
                                proc_macro2::Delimiter::None,
                                struct_name_maybe_numbered_maybe_with_generic.into_token_stream(),
                            )),
                        ));
                        indices_to_remove.push(index + 1);
                        indices_to_remove.push(index + 2);

                        additional_structs.push(quote! {
                            #[nest_struct]
                            #(#root_attrs)*
                            #root_vis struct #struct_ident_maybe_numbered #generic #group
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

    let expanded = quote! {
        #(#additional_structs)*

        #(#root_attrs)*
        #root_vis struct #root_struct_ident #root_generics {
            #(#new_root_fields),*
        }
    };

    TokenStream::from(expanded)
}
