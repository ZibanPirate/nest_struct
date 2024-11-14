use proc_macro::TokenStream;

/// Nest struct definitions with minimal syntax changes.
#[proc_macro_attribute]
pub fn nest_struct(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}
