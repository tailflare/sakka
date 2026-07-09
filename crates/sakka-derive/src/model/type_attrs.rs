use proc_macro2::TokenStream;
use syn::{Path, Result, Type, parse_quote};

use crate::common;

#[derive(Default, Clone)]
pub struct TypeAttrs {
    pub error: Option<Path>,
    pub context: Option<Path>,
}

impl TypeAttrs {
    pub fn consume(pending: &mut common::PendingAttrs) -> Result<Self> {
        Ok(Self { error: pending.take_path("error")?, context: pending.take_path("context")? })
    }

    pub fn error_type(&self, sakka: &TokenStream) -> Type {
        self.error
            .clone()
            .map(|error| parse_quote!(#error))
            .unwrap_or_else(|| parse_quote!(#sakka::Error))
    }

    pub fn context_type(&self) -> Type {
        self.context
            .clone()
            .map(|context| parse_quote!(#context))
            .unwrap_or_else(|| parse_quote!(__SakkaCtx))
    }

    pub fn include_ctx_generic(&self) -> bool {
        self.context.is_none()
    }
}
