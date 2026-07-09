use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Expr, Ident, Result};

use crate::model::{EnumAttrs, FieldAccess, FieldInfo};

pub struct EnumInfo {
    pub attrs: EnumAttrs,
    pub variants: Vec<VariantInfo>,
}

pub struct VariantInfo {
    pub name: Ident,
    pub discriminant: Option<Expr>,
    pub fields: Vec<FieldInfo>,
}

impl EnumInfo {
    pub fn parse(data: &DataEnum, attrs: EnumAttrs) -> Result<Self> {
        let variants = data
            .variants
            .iter()
            .map(|variant| {
                let fields = variant
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, field)| FieldInfo::parse(i, field))
                    .collect::<Result<Vec<_>>>()?;

                Ok(VariantInfo {
                    name: variant.ident.clone(),
                    discriminant: variant.discriminant.as_ref().map(|(_, expr)| expr.clone()),
                    fields,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { attrs, variants })
    }

    pub fn discriminants(&self) -> Vec<TokenStream> {
        let mut discriminants = Vec::with_capacity(self.variants.len());

        for variant in &self.variants {
            if let Some(explicit) = &variant.discriminant {
                discriminants.push(quote!((#explicit)));
            } else if let Some(previous) = discriminants.last() {
                discriminants.push(quote!((#previous) + 1));
            } else {
                discriminants.push(quote!(0));
            }
        }

        discriminants
    }
}

impl VariantInfo {
    pub fn is_struct_variant(&self) -> bool {
        matches!(self.fields.first().map(|f| &f.access), Some(FieldAccess::Named(_)))
    }

    pub fn pattern(&self) -> TokenStream {
        let name = &self.name;

        match self.fields.first() {
            None => quote!(Self::#name),

            Some(_) if self.is_struct_variant() => {
                let fields = self.fields.iter().map(|field| match &field.access {
                    FieldAccess::Named(name) => quote!(#name),
                    _ => unreachable!(),
                });

                quote!(Self::#name { #(#fields),* })
            }

            Some(_) => {
                let fields = self.fields.iter().map(|field| {
                    let local = &field.local;
                    quote!(#local)
                });

                quote!(Self::#name(#(#fields),*))
            }
        }
    }

    pub fn construct(&self) -> TokenStream {
        let name = &self.name;

        match self.fields.first() {
            None => quote!(Self::#name),

            Some(_) if self.is_struct_variant() => {
                let fields = self.fields.iter().map(|field| {
                    let field_name = match &field.access {
                        FieldAccess::Named(name) => name,
                        _ => unreachable!(),
                    };

                    let local = &field.local;

                    quote!(#field_name: #local)
                });

                quote!(Self::#name { #(#fields),* })
            }

            Some(_) => {
                let fields = self.fields.iter().map(|field| {
                    let local = &field.local;
                    quote!(#local)
                });

                quote!(Self::#name(#(#fields),*))
            }
        }
    }
}
