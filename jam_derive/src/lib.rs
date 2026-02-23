use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_derive(Encode, attributes(jam))]
#[proc_macro_error]
pub fn encode_derive(input: TokenStream) -> TokenStream {
    encode::encode(input)
}

#[proc_macro_derive(Decode, attributes(jam))]
#[proc_macro_error]
pub fn decode_derive(input: TokenStream) -> TokenStream {
    decode::decode(input)
}

mod decode;
mod encode;
