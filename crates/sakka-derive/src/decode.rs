extern crate alloc;

use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::{
    common,
    model::{CollectionAttr, FieldAccess, IgnoreAttr, StructInfo, StructKind},
};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let sakka = common::sakka_path()?;
    let info = StructInfo::parse(input, "Decode")?;

    let name = &info.name;

    let mut field_decodes = Vec::new();

    for field in &info.fields {
        let name = &field.local;
        let ty = field.kind.ty();

        let body = if let Some(ignore) = &field.attrs.ignore {
            match ignore {
                IgnoreAttr::Default => {
                    quote! {
                        let #name: #ty = Default::default();
                    }
                }
                IgnoreAttr::Value(value) => {
                    quote! {
                        let #name: #ty = #value;
                    }
                }
            }
        } else if let Some(collection) = &field.attrs.collection {
            // For collections, use the element type, not the full type
            let elem_ty = match &field.kind {
                crate::model::FieldKind::Vec { elem, .. } => elem,
                _ => unreachable!("collection attribute validation ensures Vec"),
            };

            match collection {
                CollectionAttr::Count(len) => {
                    quote! {
                        let #name = #sakka::ReadCollection::<Ctx>::read_vec::<#elem_ty>(reader, #len)?;
                    }
                }
                CollectionAttr::Prefix(prefix) => {
                    quote! {
                        let #name = #sakka::ReadCollection::<Ctx>::read_prefixed_vec::<#elem_ty, #prefix>(reader)?;
                    }
                }
            }
        } else if let Some(decode_with) = &field.attrs.decode_with {
            quote! {
                let #name = #decode_with(reader)?;
            }
        } else {
            let ty = &field.kind.ty();
            quote! {
                let #name = <#ty as #sakka::Decode<Ctx>>::decode(reader)?;
            }
        };

        // Alignment
        let with_align = common::wrap_alignment(
            quote!(reader),
            field.attrs.align_before.as_ref(),
            field.attrs.align_after.as_ref(),
            body,
        );

        // Padding
        field_decodes.push(common::wrap_padding(
            quote!(reader),
            field.attrs.pad_before.as_ref(),
            field.attrs.pad_after.as_ref(),
            with_align,
            false,
        ));
    }

    let construct = match info.kind {
        StructKind::Named => {
            let fields = info.fields.iter().map(|field| {
                let name = match &field.access {
                    FieldAccess::Named(name) => name,
                    _ => unreachable!(),
                };
                let local = &field.local;

                quote!(#name: #local)
            });

            quote!(Self { #(#fields),* })
        }

        StructKind::Tuple => {
            let fields = info.fields.iter().map(|field| {
                let local = &field.local;
                quote!(#local)
            });

            quote!(Self(#(#fields),*))
        }

        StructKind::Unit => quote!(Self),
    };

    Ok(quote! {
        impl<Ctx> #sakka::Decode<Ctx> for #name {
            fn decode(reader: &mut #sakka::Reader<'_, Ctx>) -> Result<Self, #sakka::Error> {
                #(#field_decodes)*

                Ok(#construct)
            }
        }
    })
}
