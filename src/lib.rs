use proc_macro::TokenStream;
use quote::quote;

/// Nest struct definitions with minimal syntax changes.
#[proc_macro_attribute]
pub fn nest_struct(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let expanded = quote! {
        struct Expanded;
    };

    TokenStream::from(expanded)
}
