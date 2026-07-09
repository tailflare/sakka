use syn::{DeriveInput, Path, Result};

#[derive(Default, Clone)]
pub struct StructAttrs {
    pub error: Option<Path>,
    pub context: Option<Path>,
}

impl StructAttrs {
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
}
