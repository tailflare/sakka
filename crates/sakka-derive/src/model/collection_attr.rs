use syn::{Expr, Ident, Result, Type, meta::ParseNestedMeta};

#[derive(Clone)]
pub enum CollectionAttr {
    Count(Expr),
    Prefix(Type),
    Field(Ident),
}

impl CollectionAttr {
    pub fn parse(meta: &ParseNestedMeta<'_>) -> Result<CollectionAttr> {
        let mut result = None;

        meta.parse_nested_meta(|meta| {
            if result.is_some() {
                return Err(meta.error("collection may only have one attribute"));
            }

            result = Some(if meta.path.is_ident("count") {
                CollectionAttr::Count(meta.value()?.parse()?)
            } else if meta.path.is_ident("prefix") {
                CollectionAttr::Prefix(meta.value()?.parse()?)
            } else if meta.path.is_ident("field") {
                CollectionAttr::Field(meta.value()?.parse()?)
            } else {
                return Err(meta.error("unknown collection attribute"));
            });

            Ok(())
        })?;

        result.ok_or_else(|| {
            meta.error(
                "expected collection(count = ...), collection(prefix = ...), or collection(field = ...)",
            )
        })
    }
}
