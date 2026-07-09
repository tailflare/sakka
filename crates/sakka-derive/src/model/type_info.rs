use alloc::format;

use syn::{Data, DeriveInput, Error, Generics, Ident, Result};

use crate::model::{StructInfo, TypeAttrs};

pub struct TypeInfo {
    pub name: Ident,
    pub generics: Generics,
    pub attrs: TypeAttrs,
    pub kind: TypeKind,
}

pub enum TypeKind {
    Struct(StructInfo),
}

impl TypeInfo {
    pub fn parse(input: DeriveInput, direction: &str) -> Result<Self> {
        let attrs = TypeAttrs::parse(&input)?;
        let name = input.ident;
        let generics = input.generics;

        let kind = match input.data {
            Data::Struct(data) => TypeKind::Struct(StructInfo::parse(&data)?),
            _ => {
                return Err(Error::new_spanned(name, format!("{direction} only supports structs")));
            }
        };

        Ok(Self { name, generics, attrs, kind })
    }
}
