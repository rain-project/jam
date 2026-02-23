use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn decode(_input: TokenStream) -> TokenStream {
    quote! {}.into()
}
