use alloc::{format, vec::Vec};

use syn::{DeriveInput, Error, Fields, Ident, Result};

use crate::model::FieldInfo;

pub struct StructInfo {
    pub name: Ident,
    pub fields: Vec<FieldInfo>,
}

impl StructInfo {
    pub fn parse(input: DeriveInput, kind: &str) -> Result<Self> {
        let name = input.ident;

        let fields = match input.data {
            syn::Data::Struct(data) => match data.fields {
                Fields::Named(fields) => fields.named,
                _ => {
                    return Err(Error::new_spanned(
                        name,
                        format!("{kind} only supports structs with named fields"),
                    ));
                }
            },
            _ => {
                return Err(Error::new_spanned(name, format!("{kind} only supports structs")));
            }
        };

        let fields =
            fields.into_iter().map(|field| FieldInfo::parse(&field)).collect::<Result<Vec<_>>>()?;

        Ok(Self { name, fields })
    }
}
