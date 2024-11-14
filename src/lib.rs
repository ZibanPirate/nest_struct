use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Nest struct definitions with minimal syntax changes.
#[proc_macro_attribute]
pub fn nest_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_item = item.clone();
    let input = parse_macro_input!(item as DeriveInput);

    let root_struct_ident = &input.ident;
    let root_vis = &input.vis;
    let root_attrs = input.attrs;
    let root_generics = input.generics;

    let root_struct_body = match input.data {
        Data::Struct(data) => data,
        _ => return original_item,
    };

    let root_fields = match root_struct_body.fields {
        Fields::Named(fields) => fields.named,
        _ => return original_item,
    };

    let mut new_root_fields = Vec::new();
    for field in root_fields {
        new_root_fields.push(field);
    }

    let expanded = quote! {
        #(#root_attrs)*
        #root_vis struct #root_struct_ident #root_generics {
            #(#new_root_fields),*
        }
    };

    TokenStream::from(expanded)
}
