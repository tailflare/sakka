extern crate alloc;

use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result, WherePredicate, parse_quote};

use crate::{
    common,
    model::{CollectionAttr, TypeInfo, TypeKind},
};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let sakka = common::sakka_path()?;
    let type_info = TypeInfo::parse(input, "Encode")?;

    match &type_info.kind {
        TypeKind::Struct(info) => expand_struct(&sakka, &type_info, info),
    }
}

fn expand_struct(
    sakka: &TokenStream,
    type_info: &TypeInfo,
    info: &crate::model::StructInfo,
) -> Result<TokenStream> {
    let error_ty = type_info.attrs.error_type(sakka);
    let context_ty = type_info.attrs.context_type();

    let name = &type_info.name;
    let mut extra_predicates: Vec<WherePredicate> = Vec::new();

    let mut field_encodes = Vec::new();

    for field in &info.fields {
        if field.attrs.ignore.is_some() {
            continue;
        }

        let access = field.access.self_access();

        let body = if let Some(codec) = &field.attrs.codec {
            quote! {
                #codec::encode(&#access, writer)?;
            }
        } else if let Some(encode_with) = &field.attrs.encode_with {
            quote! {
                #encode_with(writer, &#access)?;
            }
        } else if let Some(collection) = &field.attrs.collection {
            let elem_ty = match &field.kind {
                crate::model::FieldKind::Vec { elem, .. } => elem,
                _ => unreachable!("collection attribute validation ensures Vec"),
            };

            if common::type_depends_on_generics(elem_ty, &type_info.generics) {
                extra_predicates
                    .push(parse_quote!(#elem_ty: #sakka::Encode<#context_ty, Error = #error_ty>));
            }

            match collection {
                CollectionAttr::Count(_) => {
                    quote! {
                        #sakka::WriteCollection::<#context_ty>::write_slice::<#elem_ty>(
                            writer,
                            &#access,
                        )?;
                    }
                }

                CollectionAttr::Prefix(prefix) => {
                    quote! {
                        #sakka::WriteCollection::<#context_ty>::write_prefixed_slice::<#elem_ty, #prefix>(
                            writer,
                            &#access,
                        )?;
                    }
                }
            }
        } else {
            let ty = field.kind.ty();
            if common::type_depends_on_generics(ty, &type_info.generics) {
                extra_predicates
                    .push(parse_quote!(#ty: #sakka::Encode<#context_ty, Error = #error_ty>));
            }

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

    let impl_generics = common::build_impl_generics(
        &type_info.generics,
        extra_predicates,
        type_info.attrs.include_ctx_generic(),
    );
    let impl_params = &impl_generics.impl_generics;
    let ty_params = &impl_generics.ty_generics;
    let where_clause = &impl_generics.where_clause;

    Ok(quote! {
        impl #impl_params #sakka::Encode<#context_ty> for #name #ty_params #where_clause {
            type Error = #error_ty;

            fn encode(
                &self,
                writer: &mut #sakka::Writer<#context_ty>
            ) -> Result<(), Self::Error> {
                #(#field_encodes)*

                Ok(())
            }
        }
    })
}
