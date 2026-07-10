use syn::{Expr, Field, Path, Result};

use crate::model::{CollectionAttr, IgnoreAttr, OptionalAttr, StoreAttr};

#[derive(Default, Clone)]
pub struct FieldAttrs {
    pub ignore: Option<IgnoreAttr>,
    pub codec: Option<Path>,
    pub encode_with: Option<Path>,
    pub decode_with: Option<Path>,
    pub align_before: Option<Expr>,
    pub align_after: Option<Expr>,
    pub pad_before: Option<Expr>,
    pub pad_after: Option<Expr>,
    pub collection: Option<CollectionAttr>,
    pub optional: Option<OptionalAttr>,
    pub store: Option<StoreAttr>,
}

impl FieldAttrs {
    pub fn parse(field: &Field) -> Result<Self> {
        let mut attrs = Self {
            ignore: None,
            codec: None,
            encode_with: None,
            decode_with: None,
            align_before: None,
            align_after: None,
            pad_before: None,
            pad_after: None,
            collection: None,
            optional: None,
            store: None,
        };

        for attr in &field.attrs {
            if !attr.path().is_ident("sakka") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("ignore") {
                    if attrs.ignore.is_some() {
                        return Err(meta.error("ignore already specified"));
                    }
                    attrs.ignore = Some(IgnoreAttr::parse(&meta)?);
                } else if meta.path.is_ident("codec") {
                    if attrs.codec.is_some() {
                        return Err(meta.error("codec already specified"));
                    }
                    attrs.codec = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("encode_with") {
                    if attrs.encode_with.is_some() {
                        return Err(meta.error("encode_with already specified"));
                    }
                    attrs.encode_with = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("decode_with") {
                    if attrs.decode_with.is_some() {
                        return Err(meta.error("decode_with already specified"));
                    }
                    attrs.decode_with = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("align_before") {
                    if attrs.align_before.is_some() {
                        return Err(meta.error("align_before already specified"));
                    }
                    attrs.align_before = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("align_after") {
                    if attrs.align_after.is_some() {
                        return Err(meta.error("align_after already specified"));
                    }
                    attrs.align_after = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("pad_before") {
                    if attrs.pad_before.is_some() {
                        return Err(meta.error("pad_before already specified"));
                    }
                    attrs.pad_before = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("pad_after") {
                    if attrs.pad_after.is_some() {
                        return Err(meta.error("pad_after already specified"));
                    }
                    attrs.pad_after = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("collection") {
                    if attrs.collection.is_some() {
                        return Err(meta.error("collection already specified"));
                    }
                    attrs.collection = Some(CollectionAttr::parse(&meta)?);
                } else if meta.path.is_ident("optional") {
                    if attrs.optional.is_some() {
                        return Err(meta.error("optional already specified"));
                    }
                    attrs.optional = Some(OptionalAttr::parse(&meta)?);
                } else if meta.path.is_ident("store") {
                    if attrs.store.is_some() {
                        return Err(meta.error("store already specified"));
                    }
                    attrs.store = Some(StoreAttr::parse(&meta)?);
                } else {
                    return Err(meta.error("unknown sakka attribute"));
                }

                Ok(())
            })?;
        }

        Ok(attrs)
    }
}
