use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Field, Ident, Index, Result, Type};

use crate::{common, model::FieldAttrs};

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
    Value {
        ty: Type,
    },
    Vec {
        ty: Type,
        elem: Type,
    },
    #[allow(dead_code)]
    Option {
        ty: Type,
        inner: Type,
    },
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
        if attrs.ignore.is_some() && attrs.codec.is_some() {
            return Err(Error::new_spanned(
                field,
                "Cannot use #[sakka(ignore)] with #[sakka(codec(...))]",
            ));
        }

        let is_vec = matches!(kind, FieldKind::Vec { .. });
        let is_optional_vec = match kind {
            FieldKind::Option { inner, .. } => common::generic_inner_type(inner, "Vec").is_some(),
            _ => false,
        };

        if is_vec && attrs.collection.is_none() {
            return Err(Error::new_spanned(
                field,
                "Vec fields must have a #[sakka(collection(...))] attribute",
            ));
        }

        if is_optional_vec && attrs.optional.is_some() && attrs.collection.is_none() {
            return Err(Error::new_spanned(
                field,
                "Option<Vec<_>> fields with #[sakka(optional(...))] must also have a #[sakka(collection(...))] attribute",
            ));
        }

        if attrs.collection.is_some()
            && !(kind.is_collection() || (is_optional_vec && attrs.optional.is_some()))
        {
            return Err(Error::new_spanned(
                field,
                "Only collection fields can have a #[sakka(collection(...))] attribute",
            ));
        }

        if attrs.optional.is_some() && !matches!(kind, FieldKind::Option { .. }) {
            return Err(Error::new_spanned(
                field,
                "Only Option fields can have a #[sakka(optional(...))] attribute",
            ));
        }

        Ok(())
    }
}

impl FieldKind {
    pub fn from_field(field: &Field) -> Result<Self> {
        let ty = field.ty.clone();

        if let Some(inner) = common::generic_inner_type(&ty, "Option") {
            return Ok(FieldKind::Option { ty, inner });
        }

        if let Some(elem) = common::generic_inner_type(&ty, "Vec") {
            return Ok(FieldKind::Vec { ty, elem });
        }

        Ok(FieldKind::Value { ty })
    }

    pub fn ty(&self) -> &Type {
        match self {
            FieldKind::Value { ty } => ty,
            FieldKind::Vec { ty, .. } => ty,
            FieldKind::Option { ty, .. } => ty,
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
