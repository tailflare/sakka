use alloc::{format, vec::Vec};

use syn::{Data, DeriveInput, Error, Fields, Generics, Ident, Result};

use crate::model::{FieldInfo, StructAttrs};

pub struct StructInfo {
    pub name: Ident,
    pub generics: Generics,
    pub attrs: StructAttrs,
    pub kind: StructKind,
    pub fields: Vec<FieldInfo>,
}

pub enum StructKind {
    Named,
    Tuple,
    Unit,
}

impl StructInfo {
    pub fn parse(input: DeriveInput, direction: &str) -> Result<Self> {
        let attrs = StructAttrs::parse(&input)?;
        let name = input.ident;
        let generics = input.generics;

        let (kind, fields) = match input.data {
            Data::Struct(data) => match data.fields {
                Fields::Named(fields) => (
                    StructKind::Named,
                    fields
                        .named
                        .into_iter()
                        .enumerate()
                        .map(|(i, field)| FieldInfo::parse(i, &field))
                        .collect::<Result<Vec<_>>>()?,
                ),

                Fields::Unnamed(fields) => (
                    StructKind::Tuple,
                    fields
                        .unnamed
                        .into_iter()
                        .enumerate()
                        .map(|(i, field)| FieldInfo::parse(i, &field))
                        .collect::<Result<Vec<_>>>()?,
                ),

                Fields::Unit => (StructKind::Unit, Vec::new()),
            },

            _ => {
                return Err(Error::new_spanned(name, format!("{direction} only supports structs")));
            }
        };

        Ok(Self { name, generics, attrs, kind, fields })
    }
}
