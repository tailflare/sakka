use syn::{Result, meta::ParseNestedMeta};

#[derive(Clone)]
pub enum OptionalAttr {
    Bool,
    Eof,
}

impl OptionalAttr {
    pub fn parse(meta: &ParseNestedMeta<'_>) -> Result<Self> {
        let mut result = None;

        meta.parse_nested_meta(|meta| {
            if result.is_some() {
                return Err(meta.error("optional may only have one condition"));
            }

            result = Some(if meta.path.is_ident("bool") {
                OptionalAttr::Bool
            } else if meta.path.is_ident("eof") {
                OptionalAttr::Eof
            } else {
                return Err(meta.error("unknown optional condition"));
            });

            Ok(())
        })?;

        result.ok_or_else(|| meta.error("expected optional(bool) or optional(eof)"))
    }
}
