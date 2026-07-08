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
        if field.attrs.ignore.is_some() {
            continue;
        }

        let access = field.access.self_access();

        let body = if let Some(collection) = &field.attrs.collection {
            let elem_ty = match &field.kind {
                crate::model::FieldKind::Vec { elem, .. } => elem,
                _ => unreachable!("collection attribute validation ensures Vec"),
            };

            match collection {
                CollectionAttr::Count(_) => {
                    quote! {
                        #sakka::WriteCollection::<Ctx>::write_slice::<#elem_ty>(
                            writer,
                            &#access,
                        )?;
                    }
                }

                CollectionAttr::Prefix(prefix) => {
                    quote! {
                        #sakka::WriteCollection::<Ctx>::write_prefixed_slice::<#elem_ty, #prefix>(
                            writer,
                            &#access,
                        )?;
                    }
                }
            }
        } else if let Some(encode_with) = &field.attrs.encode_with {
            quote! {
                #encode_with(writer, &#access)?;
            }
        } else {
            quote! {
                #sakka::Encode::encode(&#access, writer)?;
            }
        };

        let with_align = common::wrap_alignment(
            quote!(writer),
            field.attrs.align_before.as_ref(),
            field.attrs.align_after.as_ref(),
            body,
        );

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
