use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Ident, Result};

pub fn sakka_path() -> Result<TokenStream> {
    match crate_name("sakka").map_err(|e| Error::new(Span::call_site(), e))? {
        FoundCrate::Itself => Ok(quote!(crate)),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            Ok(quote!(::#ident))
        }
    }
}
