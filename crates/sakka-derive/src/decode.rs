extern crate alloc;

use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result, Type, WherePredicate, parse_quote};

use crate::{
    common,
    model::{CollectionAttr, FieldAccess, IgnoreAttr, StructInfo, StructKind},
};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let sakka = common::sakka_path()?;
    let info = StructInfo::parse(input, "Decode")?;
    let error_ty = info.attrs.error.clone().unwrap_or_else(|| parse_quote!(#sakka::Error));
    let context_ty: Type = info
        .attrs
        .context
        .clone()
        .map(|context| parse_quote!(#context))
        .unwrap_or_else(|| parse_quote!(Ctx));

    let name = &info.name;
    let mut extra_predicates: Vec<WherePredicate> = Vec::new();

    let mut field_decodes = Vec::new();

    for field in &info.fields {
        let name = &field.local;
        let ty = field.kind.ty();

        let body = if let Some(ignore) = &field.attrs.ignore {
            match ignore {
                IgnoreAttr::Default => {
                    if common::type_depends_on_generics(ty, &info.generics) {
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
            // For collections, use the element type, not the full type
            let elem_ty = match &field.kind {
                crate::model::FieldKind::Vec { elem, .. } => elem,
                _ => unreachable!("collection attribute validation ensures Vec"),
            };

            if common::type_depends_on_generics(elem_ty, &info.generics) {
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
            let ty = &field.kind.ty();
            if common::type_depends_on_generics(ty, &info.generics) {
                extra_predicates
                    .push(parse_quote!(#ty: #sakka::Decode<#context_ty, Error = #error_ty>));
            }

            quote! {
                let #name = <#ty as #sakka::Decode<#context_ty>>::decode(reader)?;
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

    let impl_generics =
        common::build_impl_generics(&info.generics, extra_predicates, info.attrs.context.is_none());
    let impl_params = &impl_generics.impl_generics;
    let ty_params = &impl_generics.ty_generics;
    let where_clause = &impl_generics.where_clause;

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
        impl #impl_params #sakka::Decode<#context_ty> for #name #ty_params #where_clause {
            type Error = #error_ty;

            fn decode(reader: &mut #sakka::Reader<'_, #context_ty>) -> Result<Self, Self::Error> {
                #(#field_decodes)*

                Ok(#construct)
            }
        }
    })
}
