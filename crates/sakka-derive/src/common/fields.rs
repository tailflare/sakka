use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, Type, WherePredicate, parse_quote};

use crate::{
    common,
    model::{CollectionAttr, FieldInfo, IgnoreAttr},
};

pub enum FieldAccessMode {
    SelfAccess,
    Binding,
}

pub fn encode_fields(
    sakka: &TokenStream,
    context_ty: &Type,
    error_ty: &Type,
    generics: &Generics,
    fields: &[FieldInfo],
    access_mode: FieldAccessMode,
) -> (Vec<TokenStream>, Vec<WherePredicate>) {
    let mut field_encodes = Vec::new();
    let mut extra_predicates = Vec::new();

    for field in fields {
        if field.attrs.ignore.is_some() {
            continue;
        }

        let access = match access_mode {
            FieldAccessMode::SelfAccess => field.access.self_access(),
            FieldAccessMode::Binding => {
                let local = &field.local;
                quote!(#local)
            }
        };

        let access_ref = match access_mode {
            FieldAccessMode::SelfAccess => quote!(&#access),
            FieldAccessMode::Binding => quote!(#access),
        };

        let body = if let Some(codec) = &field.attrs.codec {
            quote! {
                #codec::encode(#access_ref, writer)?;
            }
        } else if let Some(encode_with) = &field.attrs.encode_with {
            quote! {
                #encode_with(writer, #access_ref)?;
            }
        } else if let Some(collection) = &field.attrs.collection {
            let elem_ty = match &field.kind {
                crate::model::FieldKind::Vec { elem, .. } => elem,
                _ => unreachable!("collection attribute validation ensures Vec"),
            };

            if common::type_depends_on_generics(elem_ty, generics) {
                extra_predicates
                    .push(parse_quote!(#elem_ty: #sakka::Encode<#context_ty, Error = #error_ty>));
            }

            match collection {
                CollectionAttr::Count(_) => {
                    quote! {
                        #sakka::WriteCollection::<#context_ty>::write_slice::<#elem_ty>(
                            writer,
                            #access_ref,
                        )?;
                    }
                }

                CollectionAttr::Prefix(prefix) => {
                    quote! {
                        #sakka::WriteCollection::<#context_ty>::write_prefixed_slice::<#elem_ty, #prefix>(
                            writer,
                            #access_ref,
                        )?;
                    }
                }
            }
        } else {
            let ty = field.kind.ty();
            if common::type_depends_on_generics(ty, generics) {
                extra_predicates
                    .push(parse_quote!(#ty: #sakka::Encode<#context_ty, Error = #error_ty>));
            }

            quote! {
                #sakka::Encode::encode(#access_ref, writer)?;
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

    (field_encodes, extra_predicates)
}

pub fn decode_fields(
    sakka: &TokenStream,
    context_ty: &Type,
    error_ty: &Type,
    generics: &Generics,
    fields: &[FieldInfo],
) -> (Vec<TokenStream>, Vec<WherePredicate>) {
    let mut field_decodes = Vec::new();
    let mut extra_predicates = Vec::new();

    for field in fields {
        let name = &field.local;
        let ty = field.kind.ty();

        let body = if let Some(ignore) = &field.attrs.ignore {
            match ignore {
                IgnoreAttr::Default => {
                    if common::type_depends_on_generics(ty, generics) {
                        extra_predicates.push(parse_quote!(#ty: ::core::default::Default));
                    }

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
        } else if let Some(codec) = &field.attrs.codec {
            quote! {
                let #name = #codec::decode(reader)?;
            }
        } else if let Some(decode_with) = &field.attrs.decode_with {
            quote! {
                let #name = #decode_with(reader)?;
            }
        } else if let Some(collection) = &field.attrs.collection {
            let elem_ty = match &field.kind {
                crate::model::FieldKind::Vec { elem, .. } => elem,
                _ => unreachable!("collection attribute validation ensures Vec"),
            };

            if common::type_depends_on_generics(elem_ty, generics) {
                extra_predicates
                    .push(parse_quote!(#elem_ty: #sakka::Decode<#context_ty, Error = #error_ty>));
            }

            match collection {
                CollectionAttr::Count(len) => {
                    quote! {
                        let #name = #sakka::ReadCollection::<#context_ty>::read_vec::<#elem_ty>(reader, #len)?;
                    }
                }
                CollectionAttr::Prefix(prefix) => {
                    quote! {
                        let #name = #sakka::ReadCollection::<#context_ty>::read_prefixed_vec::<#elem_ty, #prefix>(reader)?;
                    }
                }
            }
        } else {
            if common::type_depends_on_generics(ty, generics) {
                extra_predicates
                    .push(parse_quote!(#ty: #sakka::Decode<#context_ty, Error = #error_ty>));
            }

            quote! {
                let #name = <#ty as #sakka::Decode<#context_ty>>::decode(reader)?;
            }
        };

        let with_align = common::wrap_alignment(
            quote!(reader),
            field.attrs.align_before.as_ref(),
            field.attrs.align_after.as_ref(),
            body,
        );

        field_decodes.push(common::wrap_padding(
            quote!(reader),
            field.attrs.pad_before.as_ref(),
            field.attrs.pad_after.as_ref(),
            with_align,
            false,
        ));
    }

    (field_decodes, extra_predicates)
}
