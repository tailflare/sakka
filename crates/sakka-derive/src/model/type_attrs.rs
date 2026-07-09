use proc_macro2::TokenStream;
use syn::{DeriveInput, Path, Result, Type, parse_quote};

#[derive(Default, Clone)]
pub struct TypeAttrs {
    pub error: Option<Path>,
    pub context: Option<Path>,
}

impl TypeAttrs {
    pub fn parse(input: &DeriveInput) -> Result<Self> {
        let mut attrs = Self { error: None, context: None };

        for attr in &input.attrs {
            if !attr.path().is_ident("sakka") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("error") {
                    if attrs.error.is_some() {
                        return Err(meta.error("error already specified"));
                    }
                    attrs.error = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("context") {
                    if attrs.context.is_some() {
                        return Err(meta.error("context already specified"));
                    }
                    attrs.context = Some(meta.value()?.parse()?);
                } else {
                    return Err(meta.error("unknown sakka attribute"));
                }

                Ok(())
            })?;
        }

        Ok(attrs)
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
            .unwrap_or_else(|| parse_quote!(Ctx))
    }

    pub fn include_ctx_generic(&self) -> bool {
        self.context.is_none()
    }
}
