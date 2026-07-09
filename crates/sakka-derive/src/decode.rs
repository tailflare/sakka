extern crate alloc;

use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result, WherePredicate};

use crate::{
    common,
    model::{EnumInfo, FieldAccess, StructInfo, StructKind, TypeInfo, TypeKind},
};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let sakka = common::sakka_path()?;
    let type_info = TypeInfo::parse(input, "Decode")?;

    match &type_info.kind {
        TypeKind::Struct(struct_info) => expand_struct(&sakka, &type_info, struct_info),
        TypeKind::Enum(enum_info) => expand_enum(&sakka, &type_info, enum_info),
    }
}

fn expand_struct(
    sakka: &TokenStream,
    type_info: &TypeInfo,
    struct_info: &StructInfo,
) -> Result<TokenStream> {
    let error_ty = type_info.attrs.error_type(sakka);
    let context_ty = type_info.attrs.context_type();

    let name = &type_info.name;
    let mut extra_predicates: Vec<WherePredicate> = Vec::new();
    let (field_decodes, field_predicates) = common::decode_fields(
        sakka,
        &context_ty,
        &error_ty,
        &type_info.generics,
        &struct_info.fields,
    );
    extra_predicates.extend(field_predicates);

    let impl_generics = common::build_impl_generics(
        &type_info.generics,
        extra_predicates,
        type_info.attrs.include_ctx_generic(),
    );
    let impl_params = &impl_generics.impl_generics;
    let ty_params = &impl_generics.ty_generics;
    let where_clause = &impl_generics.where_clause;

    let magic_decode = common::magic_stmt(sakka, type_info.attrs.magic.as_ref(), false);

    let construct = match struct_info.kind {
        StructKind::Named => {
            let fields = struct_info.fields.iter().map(|field| {
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
            let fields = struct_info.fields.iter().map(|field| {
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
                #magic_decode
                #(#field_decodes)*

                Ok(#construct)
            }
        }
    })
}

fn expand_enum(
    sakka: &TokenStream,
    type_info: &TypeInfo,
    enum_info: &EnumInfo,
) -> Result<TokenStream> {
    let error_ty = type_info.attrs.error_type(sakka);
    let context_ty = type_info.attrs.context_type();
    let tag_ty = enum_info.attrs.tag_type();
    let (read_tag, _) = enum_info.attrs.tag_primitive_methods()?;

    let name = &type_info.name;
    let mut extra_predicates: Vec<WherePredicate> = Vec::new();
    let discriminants = enum_info.discriminants();

    let mut variant_arms = Vec::new();
    for (variant, discriminant) in enum_info.variants.iter().zip(discriminants) {
        let (field_decodes, field_predicates) = common::decode_fields(
            sakka,
            &context_ty,
            &error_ty,
            &type_info.generics,
            &variant.fields,
        );
        extra_predicates.extend(field_predicates);
        let construct = variant.construct();

        variant_arms.push(quote! {
            value if value == (#discriminant) => {
                #(#field_decodes)*
                Ok(#construct)
            }
        });
    }

    let impl_generics = common::build_impl_generics(
        &type_info.generics,
        extra_predicates,
        type_info.attrs.include_ctx_generic(),
    );
    let impl_params = &impl_generics.impl_generics;
    let ty_params = &impl_generics.ty_generics;
    let where_clause = &impl_generics.where_clause;

    let magic_decode = common::magic_stmt(sakka, type_info.attrs.magic.as_ref(), false);

    Ok(quote! {
        impl #impl_params #sakka::Decode<#context_ty> for #name #ty_params #where_clause {
            type Error = #error_ty;

            fn decode(reader: &mut #sakka::Reader<'_, #context_ty>) -> Result<Self, Self::Error> {
                #magic_decode
                let __discriminant: #tag_ty = #sakka::ReadPrimitive::#read_tag(reader)?;
                match __discriminant {
                    #(#variant_arms),*
                    _ => Err(#sakka::Error::InvalidEnumDiscriminant.into()),
                }
            }
        }
    })
}
