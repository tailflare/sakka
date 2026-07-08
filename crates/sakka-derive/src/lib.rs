#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

mod common;
mod decode;
mod encode;
mod model;

extern crate alloc;

#[proc_macro_derive(Decode, attributes(sakka))]
pub fn decode_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    decode::expand(input).unwrap_or_else(syn::Error::into_compile_error).into()
}

#[proc_macro_derive(Encode, attributes(sakka))]
pub fn encode_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    encode::expand(input).unwrap_or_else(syn::Error::into_compile_error).into()
}
