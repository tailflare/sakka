extern crate alloc;

use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::{
    common,
    model::{CollectionAttr, StructInfo},
};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let sakka = common::sakka_path()?;
    let info = StructInfo::parse(input, "Encode")?;

    let name = &info.name;

    let mut field_encodes = Vec::new();

    for field in &info.fields {
        let name = &field.name;

        // Encode the field
        let body = if field.attrs.ignore.is_some() {
            continue;
        } else if let Some(collection) = &field.attrs.collection {
            // For collections, use the element type, not the full type
            let elem_ty = match &field.kind {
                crate::model::FieldKind::Vec { elem, .. } => elem,
                _ => unreachable!("collection attribute validation ensures Vec"),
            };

            match collection {
                CollectionAttr::Count(_len) => {
                    quote! {
                        #sakka::WriteCollection::<Ctx>::write_slice::<#elem_ty>(writer, &self.#name)?;
                    }
                }
                CollectionAttr::Prefix(prefix) => {
                    quote! {
                        #sakka::WriteCollection::<Ctx>::write_prefixed_slice::<#elem_ty, #prefix>(writer, &self.#name)?;
                    }
                }
            }
        } else if let Some(encode_with) = &field.attrs.encode_with {
            quote! {
                #encode_with(writer, &self.#name)?;
            }
        } else {
            quote! {
                #sakka::Encode::encode(&self.#name, writer)?;
            }
        };

        // Alignment
        let with_align = common::wrap_alignment(
            quote!(writer),
            field.attrs.align_before.as_ref(),
            field.attrs.align_after.as_ref(),
            body,
        );

        // Padding
        field_encodes.push(common::wrap_padding(
            quote!(writer),
            field.attrs.pad_before.as_ref(),
            field.attrs.pad_after.as_ref(),
            with_align,
            true,
        ));
    }

    Ok(quote! {
        impl<Ctx> #sakka::Encode<Ctx> for #name {
            fn encode(
                &self,
                writer: &mut #sakka::Writer<Ctx>
            ) -> Result<(), #sakka::Error> {
                #(#field_encodes)*

                Ok(())
            }
        }
    })
}
