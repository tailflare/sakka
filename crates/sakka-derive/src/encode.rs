extern crate alloc;

use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result, WherePredicate};

use crate::{
    common,
    model::{EnumInfo, TypeInfo, TypeKind},
};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let sakka = common::sakka_path()?;
    let type_info = TypeInfo::parse(input, "Encode")?;

    match &type_info.kind {
        TypeKind::Struct(info) => expand_struct(&sakka, &type_info, info),
        TypeKind::Enum(enum_info) => expand_enum(&sakka, &type_info, enum_info),
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
    let (field_encodes, field_predicates) = common::encode_fields(
        sakka,
        &context_ty,
        &error_ty,
        &type_info.generics,
        &info.fields,
        common::FieldAccessMode::SelfAccess,
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

    let magic_encode = common::magic_stmt(sakka, type_info.attrs.magic.as_ref(), true);

    Ok(quote! {
        impl #impl_params #sakka::Encode<#context_ty> for #name #ty_params #where_clause {
            type Error = #error_ty;

            fn encode(
                &self,
                writer: &mut #sakka::Writer<#context_ty>
            ) -> Result<(), Self::Error> {
                #magic_encode
                #(#field_encodes)*

                Ok(())
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
    let (_, write_tag) = enum_info.attrs.tag_primitive_methods()?;

    let name = &type_info.name;
    let mut extra_predicates: Vec<WherePredicate> = Vec::new();
    let discriminants = enum_info.discriminants();

    let mut variant_arms = Vec::new();
    for (variant, discriminant) in enum_info.variants.iter().zip(discriminants) {
        let pattern = variant.pattern();
        let (field_encodes, field_predicates) = common::encode_fields(
            sakka,
            &context_ty,
            &error_ty,
            &type_info.generics,
            &variant.fields,
            common::FieldAccessMode::Binding,
        );
        extra_predicates.extend(field_predicates);

        variant_arms.push(quote! {
            #pattern => {
                let __discriminant: #tag_ty = (#discriminant);
                #sakka::WritePrimitive::#write_tag(writer, __discriminant)?;
                #(#field_encodes)*
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

    let magic_encode = common::magic_stmt(sakka, type_info.attrs.magic.as_ref(), true);

    Ok(quote! {
        impl #impl_params #sakka::Encode<#context_ty> for #name #ty_params #where_clause {
            type Error = #error_ty;

            fn encode(
                &self,
                writer: &mut #sakka::Writer<#context_ty>
            ) -> Result<(), Self::Error> {
                #magic_encode
                match self {
                    #(#variant_arms),*
                }

                Ok(())
            }
        }
    })
}
