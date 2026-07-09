use alloc::format;

use syn::{Data, DeriveInput, Error, Generics, Ident, Result};

use crate::{
    common,
    model::{EnumAttrs, EnumInfo, StructInfo, TypeAttrs},
};

pub struct TypeInfo {
    pub name: Ident,
    pub generics: Generics,
    pub attrs: TypeAttrs,
    pub kind: TypeKind,
}

pub enum TypeKind {
    Struct(StructInfo),
    Enum(EnumInfo),
}

impl TypeInfo {
    pub fn parse(input: DeriveInput, direction: &str) -> Result<Self> {
        let mut pending = common::PendingAttrs::collect(&input)?;
        let attrs = TypeAttrs::consume(&mut pending)?;

        let name = input.ident;
        let generics = input.generics;

        let kind = match input.data {
            Data::Struct(data) => TypeKind::Struct(StructInfo::parse(&data)?),
            Data::Enum(data) => {
                let enum_attrs = EnumAttrs::consume(&mut pending)?;
                TypeKind::Enum(EnumInfo::parse(&data, enum_attrs)?)
            }
            _ => {
                return Err(Error::new_spanned(
                    name,
                    format!("{direction} only supports structs and enums"),
                ));
            }
        };

        pending.ensure_empty()?;

        Ok(Self { name, generics, attrs, kind })
    }
}
