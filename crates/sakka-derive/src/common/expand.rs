use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

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
