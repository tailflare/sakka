use alloc::vec::Vec;

use syn::{DataStruct, Error, Field, Fields, Result};

use crate::model::{CollectionAttr, FieldAccess, FieldInfo};

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
            Fields::Named(fields) => {
                let mut parsed_fields = Vec::with_capacity(fields.named.len());

                for (i, field) in fields.named.iter().enumerate() {
                    let parsed = FieldInfo::parse(i, field)?;
                    validate_collection_field_reference(field, &parsed, &parsed_fields)?;
                    parsed_fields.push(parsed);
                }

                (StructKind::Named, parsed_fields)
            }

            Fields::Unnamed(fields) => {
                let mut parsed_fields = Vec::with_capacity(fields.unnamed.len());

                for (i, field) in fields.unnamed.iter().enumerate() {
                    let parsed = FieldInfo::parse(i, field)?;
                    validate_collection_field_reference(field, &parsed, &parsed_fields)?;
                    parsed_fields.push(parsed);
                }

                (StructKind::Tuple, parsed_fields)
            }

            Fields::Unit => (StructKind::Unit, Vec::new()),
        };

        Ok(Self { kind, fields })
    }
}

fn validate_collection_field_reference(
    field: &Field,
    parsed: &FieldInfo,
    previous: &[FieldInfo],
) -> Result<()> {
    let Some(CollectionAttr::Field(reference)) = parsed.attrs.collection.as_ref() else {
        return Ok(());
    };

    let is_previous_named_field = previous
        .iter()
        .any(|prev| matches!(&prev.access, FieldAccess::Named(name) if name == reference));

    if is_previous_named_field {
        return Ok(());
    }

    Err(Error::new_spanned(
        field,
        "collection(field = ...) must reference a previously declared named field",
    ))
}
