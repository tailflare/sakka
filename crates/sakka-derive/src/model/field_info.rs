use syn::{Error, Expr, Field, GenericArgument, Ident, PathArguments, Result, Type};

use crate::model::FieldAttrs;

pub struct FieldInfo {
    pub name: Ident,
    pub kind: FieldKind,
    pub attrs: FieldAttrs,
}

pub enum FieldKind {
    Value {
        ty: Type,
    },
    Vec {
        ty: Type,
        elem: Type,
    },
    #[allow(dead_code)]
    Array {
        ty: Type,
        elem: Type,
        len: Expr,
    },
}

impl FieldInfo {
    pub fn parse(field: &Field) -> Result<Self> {
        let kind = FieldKind::from_field(field)?;
        let attrs = FieldAttrs::parse(field)?;

        Self::validate(field, &kind, &attrs)?;

        Ok(Self { name: field.ident.clone().unwrap(), kind, attrs })
    }

    fn validate(field: &Field, kind: &FieldKind, attrs: &FieldAttrs) -> Result<()> {
        if let FieldKind::Vec { .. } = kind
            && attrs.collection.is_none()
        {
            return Err(Error::new_spanned(
                field,
                "Vec fields must have a #[sakka(collection(...))] attribute",
            ));
        }

        if attrs.collection.is_some() && !kind.is_collection() {
            return Err(Error::new_spanned(
                field,
                "Only collection fields can have a #[sakka(collection(...))] attribute",
            ));
        }

        if attrs.ignore.is_some() && (attrs.decode_with.is_some() || attrs.encode_with.is_some()) {
            return Err(Error::new_spanned(
                field,
                "Cannot use #[sakka(ignore)] with #[sakka(decode_with(...))] or #[sakka(encode_with(...))]",
            ));
        }

        Ok(())
    }
}

impl FieldKind {
    pub fn from_field(field: &Field) -> Result<Self> {
        match &field.ty {
            Type::Array(array) => Ok(FieldKind::Array {
                ty: field.ty.clone(),
                elem: (*array.elem).clone(),
                len: array.len.clone(),
            }),
            Type::Path(type_path) => {
                // Check if this is Vec<T>
                if let Some(segment) = type_path.path.segments.last()
                    && segment.ident == "Vec"
                    && let PathArguments::AngleBracketed(args) = &segment.arguments
                    && let Some(GenericArgument::Type(inner_ty)) = args.args.first()
                {
                    return Ok(FieldKind::Vec { ty: field.ty.clone(), elem: inner_ty.clone() });
                }

                Ok(FieldKind::Value { ty: field.ty.clone() })
            }
            _ => Ok(FieldKind::Value { ty: field.ty.clone() }),
        }
    }

    pub fn ty(&self) -> &Type {
        match self {
            FieldKind::Value { ty } => ty,
            FieldKind::Vec { ty, .. } => ty,
            FieldKind::Array { ty, .. } => ty,
        }
    }

    pub fn is_collection(&self) -> bool {
        matches!(self, FieldKind::Vec { .. })
    }
}
