use alloc::vec::Vec;

use syn::{DataStruct, Fields, Result};

use crate::model::FieldInfo;

pub struct StructInfo {
    pub kind: StructKind,
    pub fields: Vec<FieldInfo>,
}

pub enum StructKind {
    Named,
    Tuple,
    Unit,
}

impl StructInfo {
    pub fn parse(data: &DataStruct) -> Result<Self> {
        let (kind, fields) = match &data.fields {
            Fields::Named(fields) => (
                StructKind::Named,
                fields
                    .named
                    .iter()
                    .enumerate()
                    .map(|(i, field)| FieldInfo::parse(i, field))
                    .collect::<Result<Vec<_>>>()?,
            ),

            Fields::Unnamed(fields) => (
                StructKind::Tuple,
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, field)| FieldInfo::parse(i, field))
                    .collect::<Result<Vec<_>>>()?,
            ),

            Fields::Unit => (StructKind::Unit, Vec::new()),
        };

        Ok(Self { kind, fields })
    }
}
