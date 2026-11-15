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
//! Or, you can open a block and define struct like normal Rust code for full flexibility:
//!
//! ```rust
//! use nest_struct::nest_struct;
//!
//! #[nest_struct]
//! struct Post {
//!     title: String,
//!     summary: String,
//!     author: nest! {
//!         /// doc comment for Author struct
//!         #[derive(Debug)]
//!         struct Author {
//!             name: String,
//!             handle: String,
//!         }
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
//! /// doc comment for Author struct
//! #[derive(Debug)]
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
//! - [x] deep nesting (no theoretical limit).
//! - [x] nest `struct` inside another `struct`.
//! - [x] nest `enum` inside another `enum`.
//! - [x] nest `enum` inside a `struct` and vice-versa.
//! - [x] inherit `derive` and other attribute macros from root `struct`.
//! - [x] auto-generate inner `struct` names.
//! - [x] overwrite the auto-generated inner struct name.
//!
//! Feature parity with native Rust code:
//!
//! - [x] `impl` block on inner `struct`s.
//! - [x] define `derive` and other attribute macros individually per inner `struct`.
//! - [x] define doc comments individually per inner `struct`.
//! - [x] useful compiler error messages.
//! - [x] support generic types.
//! - [x] support lifetimes.

use heck::ToPascalCase;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Data, DeriveInput,
    Field, Fields, FieldsNamed, FieldsUnnamed, Generics, Type,
};

const ERR_MSG_MORE_THAN_ONE_STMT_IN_BLOCK: &str =
    "only one statement is allowed inside `nest!` block";
const ERR_MSG_ONLY_STRUCT_AND_ENUM_SUPPORTED_IN_BLOCK: &str =
    "only struct and enum are supported inside nest! block";

fn collect_matching_idents(
    token_stream: proc_macro2::TokenStream,
    ident_names: &[String],
) -> Vec<String> {
    let mut idents: Vec<String> = vec![];
    let mut tokens = token_stream.into_iter().peekable();

    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Ident(ident) => {
                let is_field_name = matches!(
                    tokens.peek(),
                    Some(TokenTree::Punct(punct))
                        if punct.as_char() == ':' && punct.spacing() == proc_macro2::Spacing::Alone
                );
                if !is_field_name && ident_names.contains(&ident.to_string()) {
                    idents.push(ident.to_string());
                }
            }
            TokenTree::Group(group) => {
                idents.extend(collect_matching_idents(group.stream(), ident_names));
            }
            _ => {}
        }
    }

    idents.dedup();

    // @TODO-ZM: preserve order of found idents
    idents
}

enum NestedBody {
    Struct,
    Enum,
    Block {
        item: Box<syn::Item>,
        ident: syn::Ident,
        generics: Generics,
    },
}

struct NestContext<'a> {
    root_ident: &'a syn::Ident,
    middle_ident: Option<&'a syn::Ident>,
    generics: &'a Generics,
    vis: &'a syn::Visibility,
    attrs: &'a [syn::Attribute],
}

impl NestContext<'_> {
    fn root_name(&self) -> String {
        format!(
            "{}{}",
            self.root_ident,
            self.middle_ident
                .map(|ident| ident.to_string().to_pascal_case())
                .unwrap_or_default()
        )
    }

    fn root_attrs(&self) -> Vec<&syn::Attribute> {
        self.attrs
            .iter()
            .filter(|attr| !attr.path().is_ident("doc"))
            .collect()
    }
}

fn generated_ident(
    root_name: &str,
    field_name: &str,
    suffix: Option<usize>,
    explicit_name: Option<&str>,
) -> syn::Ident {
    let name = match explicit_name {
        Some(name) => name.to_string(),
        None => format!(
            "{}{}{}",
            root_name,
            field_name
                .strip_prefix("r#")
                .unwrap_or(field_name)
                .to_pascal_case(),
            suffix.map(|index| index.to_string()).unwrap_or_default()
        ),
    };

    syn::Ident::new(&name, proc_macro2::Span::call_site())
}

fn auto_named_nest_count(ty: &Type) -> usize {
    let tokens = ty
        .clone()
        .into_token_stream()
        .into_iter()
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut index = 0;
    while index + 2 < tokens.len() {
        if let (TokenTree::Ident(ident), TokenTree::Punct(punct), TokenTree::Group(_)) =
            (&tokens[index], &tokens[index + 1], &tokens[index + 2])
        {
            if ident == "nest" && punct.as_char() == '!' {
                count += 1;
                index += 3;
                continue;
            }
        }
        index += 1;
    }
    count
}

fn filter_generics(generics: &Generics, found_names: &[String]) -> Generics {
    let mut filtered = generics.clone();
    filtered.params = filtered
        .params
        .into_iter()
        .filter(|param| {
            param
                .to_token_stream()
                .into_iter()
                .any(|token| match token {
                    TokenTree::Ident(ident) => found_names.contains(&ident.to_string()),
                    _ => false,
                })
        })
        .collect();
    filtered
}

fn classify_nested_body(group: &proc_macro2::Group) -> syn::Result<NestedBody> {
    let struct_parse_err = match syn::parse2::<DeriveInput>(quote! { struct Foo #group }) {
        Ok(_) => return Ok(NestedBody::Struct),
        Err(error) => error,
    };

    let enum_parse_err = match syn::parse2::<DeriveInput>(quote! { enum Foo #group }) {
        Ok(_) => return Ok(NestedBody::Enum),
        Err(error) => error,
    };

    let block = match syn::parse2::<syn::Block>(group.into_token_stream()) {
        Ok(block) => block,
        Err(block_parse_err) => {
            let mut combined_error = syn::Error::new(
                struct_parse_err.span(),
                format!("if nesting a struct: {}", struct_parse_err),
            );
            combined_error.combine(syn::Error::new(
                enum_parse_err.span(),
                format!("if nesting an enum: {}", enum_parse_err),
            ));
            combined_error.combine(syn::Error::new(
                block_parse_err.span(),
                format!("if nesting a block: {}", block_parse_err),
            ));
            return Err(combined_error);
        }
    };

    if block.stmts.len() > 1 {
        let mut combined_error = syn::Error::new(
            block.stmts.get(1).unwrap().span(),
            ERR_MSG_MORE_THAN_ONE_STMT_IN_BLOCK,
        );
        block.stmts.iter().skip(1).for_each(|stmt| {
            combined_error.combine(syn::Error::new(
                stmt.span(),
                ERR_MSG_MORE_THAN_ONE_STMT_IN_BLOCK,
            ));
        });
        return Err(combined_error);
    }

    let only_stmt = block.stmts.first().unwrap();
    let item = match only_stmt {
        syn::Stmt::Item(item) => item,
        _ => {
            return Err(syn::Error::new(
                only_stmt.span(),
                ERR_MSG_ONLY_STRUCT_AND_ENUM_SUPPORTED_IN_BLOCK,
            ));
        }
    };

    match item {
        syn::Item::Struct(struct_item) => Ok(NestedBody::Block {
            item: Box::new(item.clone()),
            ident: struct_item.ident.clone(),
            generics: struct_item.generics.clone(),
        }),
        syn::Item::Enum(enum_item) => Ok(NestedBody::Block {
            item: Box::new(item.clone()),
            ident: enum_item.ident.clone(),
            generics: enum_item.generics.clone(),
        }),
        _ => Err(syn::Error::new(
            item.span(),
            ERR_MSG_ONLY_STRUCT_AND_ENUM_SUPPORTED_IN_BLOCK,
        )),
    }
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

    expand(input, original_item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand(input: DeriveInput, original_item: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let original_item = proc_macro2::TokenStream::from(original_item);
    let root_struct_ident = &input.ident;
    let root_vis = &input.vis;
    let root_attrs = input.attrs;
    let root_generics = input.generics;

    match input.data {
        Data::Struct(root_struct_body) => {
            let root_fields = match root_struct_body.fields {
                Fields::Named(fields) => fields.named,
                _ => {
                    return Err(syn::Error::new(
                        root_struct_ident.span(),
                        "nest_struct requires a named-field struct or enum",
                    ));
                }
            };

            let (additional_structs, new_root_fields) = convert_nest_to_structs(
                root_fields,
                NestContext {
                    root_ident: root_struct_ident,
                    middle_ident: None,
                    generics: &root_generics,
                    vis: root_vis,
                    attrs: &root_attrs,
                },
            )?;

            let expanded = quote! {
                #(#additional_structs)*

                #(#root_attrs)*
                #root_vis struct #root_struct_ident #root_generics {
                    #(#new_root_fields),*
                }
            };

            Ok(expanded)
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

                let (additional_structs_for_variant, new_variant_fields) = convert_nest_to_structs(
                    variant_fields,
                    NestContext {
                        root_ident: root_struct_ident,
                        middle_ident: Some(&variant.ident),
                        generics: &root_generics,
                        vis: root_vis,
                        attrs: &root_attrs,
                    },
                )?;

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

            Ok(expanded)
        }
        _ => Ok(original_item),
    }
}

fn convert_nest_to_structs(
    fields: Punctuated<Field, Comma>,
    context: NestContext<'_>,
) -> syn::Result<(Vec<proc_macro2::TokenStream>, Vec<syn::Field>)> {
    let root_attrs = context.root_attrs();
    let root_struct_name = context.root_name();
    let root_vis = context.vis;

    let root_generic_names = context
        .generics
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

    let tuple_scope = !fields.is_empty() && fields.iter().all(|field| field.ident.is_none());
    let tuple_nest_count = if tuple_scope {
        fields
            .iter()
            .map(|field| auto_named_nest_count(&field.ty))
            .sum()
    } else {
        0
    };

    let mut field_name_index = 0;
    for mut field in fields {
        let tuple_field_index = field_name_index;
        let field_nest_count = auto_named_nest_count(&field.ty);
        let numbered = if field.ident.is_none() {
            tuple_nest_count > 1
        } else {
            field_nest_count > 1
        };
        let mut auto_nest_in_field = 0;
        let field_name = match field.ident {
            Some(ref ident) => ident.to_string(),
            None => {
                field_name_index += 1;
                String::new()
            }
        };

        let token_tree = field
            .ty
            .clone()
            .into_token_stream()
            .into_iter()
            .collect::<Vec<TokenTree>>();

        let mut new_tokens = Vec::with_capacity(token_tree.len());
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
            if let (
                Some(TokenTree::Ident(ident)),
                Some(TokenTree::Punct(punct)),
                Some(TokenTree::Group(group)),
            ) = (ident, punct, group)
            {
                let ident_str = ident.to_string();
                let is_pascal = ident_str.to_pascal_case() == ident_str;
                if (ident_str == "nest" || is_pascal) && punct.as_char() == '!' {
                    let body_type_syn = match classify_nested_body(group)? {
                        NestedBody::Block {
                            item,
                            ident,
                            generics,
                        } => {
                            let item = *item;
                            let (_, ty_generics, _) = generics.split_for_impl();
                            let replacement_ty =
                                syn::parse2::<Type>(quote! { #ident #ty_generics }).unwrap();
                            new_tokens.push(TokenTree::Group(proc_macro2::Group::new(
                                proc_macro2::Delimiter::None,
                                replacement_ty.into_token_stream(),
                            )));
                            additional_structs.push(quote! {
                                #[nest_struct]
                                #item
                            });
                            index += 3;
                            auto_nest_in_field += 1;
                            continue;
                        }
                        NestedBody::Struct => quote! { struct },
                        NestedBody::Enum => quote! { enum },
                    };

                    let suffix = if is_pascal || !numbered {
                        None
                    } else if field.ident.is_none() {
                        if field_nest_count > 1 {
                            Some(auto_nest_in_field)
                        } else {
                            Some(tuple_field_index)
                        }
                    } else {
                        Some(auto_nest_in_field)
                    };
                    let inner_struct_name = generated_ident(
                        &root_struct_name,
                        &field_name,
                        suffix,
                        is_pascal.then_some(ident_str.as_str()),
                    );

                    let found_ident_names_for_generics =
                        collect_matching_idents(group.stream(), &root_generic_names);
                    let struct_generic =
                        filter_generics(context.generics, &found_ident_names_for_generics);
                    let (_, ty_generics, _) = struct_generic.split_for_impl();
                    let inner_struct_name_maybe_with_generic =
                        syn::parse2::<Type>(quote! { #inner_struct_name #ty_generics }).unwrap();

                    new_tokens.push(TokenTree::Group(proc_macro2::Group::new(
                        proc_macro2::Delimiter::None,
                        inner_struct_name_maybe_with_generic.into_token_stream(),
                    )));
                    additional_structs.push(quote! {
                        #[nest_struct]
                        #(#root_attrs)*
                        #root_vis #body_type_syn #inner_struct_name #struct_generic #group
                    });

                    if ident_str == "nest" {
                        auto_nest_in_field += 1;
                    }
                    index += 3;
                    continue;
                }
            }

            new_tokens.push(token_tree[index].clone());
            index += 1;
        }

        field.ty = syn::parse2(quote! { #(#new_tokens)* }).unwrap();
        new_root_fields.push(field);
    }

    Ok((additional_structs, new_root_fields))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::{Delimiter, Group};
    use quote::quote;

    #[test]
    fn generated_ident_uses_root_field_and_suffix() {
        assert_eq!(
            generated_ident("Post", "author", None, None).to_string(),
            "PostAuthor"
        );
        assert_eq!(
            generated_ident("Post", "author", Some(0), None).to_string(),
            "PostAuthor0"
        );
        assert_eq!(
            generated_ident("Post", "author", Some(1), None).to_string(),
            "PostAuthor1"
        );
        assert_eq!(
            generated_ident("Post", "r#type", None, None).to_string(),
            "PostType"
        );
    }

    #[test]
    fn generated_ident_honors_explicit_name() {
        assert_eq!(
            generated_ident("Post", "author", Some(2), Some("Custom")).to_string(),
            "Custom"
        );
    }

    #[test]
    fn counts_only_auto_named_nests() {
        let ty: Type = syn::parse_quote! {
            Either<nest! { a: u8 }, Custom! { b: u8 }, nest! { c: u8 }>
        };
        assert_eq!(auto_named_nest_count(&ty), 2);
    }

    #[test]
    fn filter_generics_keeps_only_used_params() {
        let input: DeriveInput = syn::parse_quote! { struct S<T, U> { first: T, second: String } };
        let filtered = filter_generics(&input.generics, &["T".to_string()]);
        let names = filtered
            .params
            .iter()
            .map(|param| quote! { #param }.to_string())
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["T"]);
    }

    #[test]
    fn classify_nested_body_kinds() {
        let struct_group = Group::new(Delimiter::Brace, quote! { value: String });
        assert!(matches!(
            classify_nested_body(&struct_group).unwrap(),
            NestedBody::Struct
        ));

        let enum_group = Group::new(Delimiter::Brace, quote! { First, Last });
        assert!(matches!(
            classify_nested_body(&enum_group).unwrap(),
            NestedBody::Enum
        ));

        let block_group = Group::new(Delimiter::Brace, quote! { struct Inner { value: String } });
        assert!(matches!(
            classify_nested_body(&block_group).unwrap(),
            NestedBody::Block { .. }
        ));
    }

    #[test]
    fn classify_nested_body_rejects_non_item_block() {
        let group = Group::new(Delimiter::Brace, quote! { 1 + 1 });
        assert!(classify_nested_body(&group).is_err());
    }
}
