use syn::{Expr, Result, Type, meta::ParseNestedMeta};

#[derive(Clone)]
pub enum CollectionAttrs {
    Count(Expr),
    Prefix(Type),
}

impl CollectionAttrs {
    pub fn parse(meta: &ParseNestedMeta<'_>) -> Result<CollectionAttrs> {
        let mut result = None;

        meta.parse_nested_meta(|meta| {
            if result.is_some() {
                return Err(meta.error("collection may only have one attribute"));
            }

            result = Some(if meta.path.is_ident("count") {
                CollectionAttrs::Count(meta.value()?.parse()?)
            } else if meta.path.is_ident("prefix") {
                CollectionAttrs::Prefix(meta.value()?.parse()?)
            } else {
                return Err(meta.error("unknown collection attribute"));
            });

            Ok(())
        })?;

        result.ok_or_else(|| {
            meta.error("expected collection(count = ...) or collection(prefix = ...)")
        })
    }
}
