use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn encode(_input: TokenStream) -> TokenStream {
    quote! {}.into()
}
