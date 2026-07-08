use syn::{DeriveInput, Path, Result};

#[derive(Default, Clone)]
pub struct StructAttrs {
    pub error: Option<Path>,
}

impl StructAttrs {
    pub fn parse(input: &DeriveInput) -> Result<Self> {
        let mut attrs = Self { error: None };

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
                } else {
                    return Err(meta.error("unknown sakka attribute"));
                }

                Ok(())
            })?;
        }

        Ok(attrs)
    }
}
