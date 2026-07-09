use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

use crate::model::OptionalAttr;

pub fn wrap_alignment(
    receiver: TokenStream,
    before: Option<&Expr>,
    after: Option<&Expr>,
    body: TokenStream,
) -> TokenStream {
    let before = before.map(|align| {
        quote! {
            #receiver.align(#align)?;
        }
    });

    let after = after.map(|align| {
        quote! {
            #receiver.align(#align)?;
        }
    });

    quote! {
        #before
        #body
        #after
    }
}

pub fn wrap_padding(
    receiver: TokenStream,
    before: Option<&Expr>,
    after: Option<&Expr>,
    body: TokenStream,
    is_writer: bool,
) -> TokenStream {
    let before = if is_writer {
        before.map(|pad| {
            quote! {
                #receiver.write_zeroes(#pad)?;
            }
        })
    } else {
        before.map(|pad| {
            quote! {
                #receiver.skip(#pad)?;
            }
        })
    };

    let after = if is_writer {
        after.map(|pad| {
            quote! {
                #receiver.write_zeroes(#pad)?;
            }
        })
    } else {
        after.map(|pad| {
            quote! {
                #receiver.skip(#pad)?;
            }
        })
    };

    quote! {
        #before
        #body
        #after
    }
}

pub fn wrap_optional(
    sakka: &TokenStream,
    receiver: TokenStream,
    attr: OptionalAttr,
    body: TokenStream,
    is_writer: bool,
) -> TokenStream {
    match (attr, is_writer) {
        (OptionalAttr::Bool, true) => {
            quote! {
                #sakka::WriteOption::write_option_with(
                    #receiver,
                    __sakka_optional_value,
                    |#receiver, __sakka_optional_inner| {
                        #body
                        Ok(())
                    },
                )?;
            }
        }
        (OptionalAttr::Bool, false) => {
            quote! {
                #sakka::ReadOption::read_option_with(#receiver, |#receiver| #body)?
            }
        }
        (OptionalAttr::Eof, true) => {
            quote! {
                if let Some(__sakka_optional_inner) = __sakka_optional_value {
                    #body
                }
            }
        }
        (OptionalAttr::Eof, false) => {
            quote! {
                if #receiver.is_eof() {
                    None
                } else {
                    #body
                }
            }
        }
    }
}
