use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Lit};

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

pub fn magic_stmt(
    sakka: &TokenStream,
    magic: Option<&Expr>,
    is_writer: bool,
) -> Option<TokenStream> {
    magic.map(|magic| {
        let encode_magic_arg = if matches!(magic, Expr::Lit(expr_lit) if matches!(&expr_lit.lit, Lit::ByteStr(_))) {
            quote!(&*(#magic))
        } else {
            quote!(&(#magic))
        };

        let decode_magic_expected = if matches!(magic, Expr::Lit(expr_lit) if matches!(&expr_lit.lit, Lit::ByteStr(_))) {
            quote!(*(#magic))
        } else {
            quote!((#magic))
        };

        if is_writer {
            quote! {
                fn __sakka_encode_magic<__SakkaCtx, __SakkaError, __SakkaMagic>(
                    writer: &mut #sakka::Writer<__SakkaCtx>,
                    value: &__SakkaMagic,
                ) -> Result<(), __SakkaError>
                where
                    __SakkaMagic: #sakka::Encode<__SakkaCtx, Error = __SakkaError>,
                    __SakkaError: From<#sakka::Error>,
                {
                    value.encode(writer)
                }

                __sakka_encode_magic(writer, #encode_magic_arg)?;
            }
        } else {
            quote! {
                fn __sakka_decode_magic<__SakkaCtx, __SakkaError, __SakkaMagic>(
                    reader: &mut #sakka::Reader<'_, __SakkaCtx>,
                    expected: __SakkaMagic,
                ) -> Result<(), __SakkaError>
                where
                    __SakkaMagic: #sakka::Decode<__SakkaCtx, Error = __SakkaError>
                        + ::core::cmp::PartialEq,
                    __SakkaError: From<#sakka::Error>,
                {
                    let actual = __SakkaMagic::decode(reader)?;
                    if actual == expected {
                        Ok(())
                    } else {
                        Err(#sakka::Error::InvalidMagic(::core::stringify!(#magic)).into())
                    }
                }

                __sakka_decode_magic(reader, #decode_magic_expected)?;
            }
        }
    })
}
