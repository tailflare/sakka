use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Field, GenericArgument, Ident, Index, PathArguments, Result, Type};

use crate::model::FieldAttrs;

pub struct FieldInfo {
    pub access: FieldAccess,
    pub local: Ident,
    pub kind: FieldKind,
    pub attrs: FieldAttrs,
}

pub enum FieldAccess {
    Named(Ident),
    Index(Index),
}

#[allow(clippy::large_enum_variant)]
pub enum FieldKind {
    Value { ty: Type },
    Vec { ty: Type, elem: Type },
}

impl FieldInfo {
    pub fn parse(index: usize, field: &Field) -> Result<Self> {
        let access = match &field.ident {
            Some(ident) => FieldAccess::Named(ident.clone()),
            None => FieldAccess::Index(Index::from(index)),
        };

        let local = match &access {
            FieldAccess::Named(ident) => ident.clone(),
            FieldAccess::Index(index) => format_ident!("__field{}", index),
        };

        let kind = FieldKind::from_field(field)?;
        let attrs = FieldAttrs::parse(field)?;

        Self::validate(field, &kind, &attrs)?;

        Ok(Self { access, local, kind, attrs })
    }

    fn validate(field: &Field, kind: &FieldKind, attrs: &FieldAttrs) -> Result<()> {
        if attrs.ignore.is_some()
            && (attrs.codec.is_some() || attrs.decode_with.is_some() || attrs.encode_with.is_some())
        {
            return Err(Error::new_spanned(
                field,
                "Cannot use #[sakka(ignore)] with #[sakka(codec(...))], #[sakka(decode_with(...))] or #[sakka(encode_with(...))]",
            ));
        }

        if attrs.codec.is_some() && (attrs.decode_with.is_some() || attrs.encode_with.is_some()) {
            return Err(Error::new_spanned(
                field,
                "Cannot use #[sakka(codec(...))] with #[sakka(decode_with(...))] or #[sakka(encode_with(...))]",
            ));
        }

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

        Ok(())
    }
}

impl FieldKind {
    pub fn from_field(field: &Field) -> Result<Self> {
        match &field.ty {
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
        }
    }

    pub fn is_collection(&self) -> bool {
        matches!(self, FieldKind::Vec { .. })
    }
}

impl FieldAccess {
    pub fn self_access(&self) -> TokenStream {
        match self {
            FieldAccess::Named(name) => quote!(self.#name),
            FieldAccess::Index(index) => quote!(self.#index),
        }
    }
}
