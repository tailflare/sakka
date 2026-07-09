use syn::{Result, Type, meta::ParseNestedMeta};

#[derive(Clone)]
pub enum OptionalAttr {
    Bool,
    Eof,
}

impl OptionalAttr {
    pub fn parse(meta: &ParseNestedMeta<'_>) -> Result<Self> {
        if meta.input.peek(syn::Token![=]) {
            let value: Type = meta.value()?.parse()?;
            return Self::from_type(&value)
                .ok_or_else(|| meta.error("expected optional = bool|eof"));
        }

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

        result.ok_or_else(|| {
            meta.error("expected optional(bool), optional(eof), optional = bool, or optional = eof")
        })
    }

    fn from_type(value: &Type) -> Option<Self> {
        let Type::Path(path) = value else {
            return None;
        };

        if path.path.is_ident("bool") {
            Some(OptionalAttr::Bool)
        } else if path.path.is_ident("eof") {
            Some(OptionalAttr::Eof)
        } else {
            None
        }
    }
}
