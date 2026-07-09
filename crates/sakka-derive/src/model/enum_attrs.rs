use alloc::string::ToString;

use quote::format_ident;
use syn::{Error, Ident, Path, Result, Type, parse_quote};

use crate::common;

#[derive(Default, Clone)]
pub struct EnumAttrs {
    pub tag: Option<Path>,
}

impl EnumAttrs {
    pub fn consume(pending: &mut common::PendingAttrs) -> Result<Self> {
        Ok(Self { tag: pending.take_path("tag")? })
    }

    pub fn tag_type(&self) -> Type {
        self.tag.clone().map(|tag| parse_quote!(#tag)).unwrap_or_else(|| parse_quote!(u8))
    }

    pub fn tag_primitive_methods(&self) -> Result<(Ident, Ident)> {
        let tag_name = match &self.tag {
            Some(tag) => {
                let segment = tag.segments.last().ok_or_else(|| {
                    Error::new_spanned(tag, "enum tag must be a primitive integer type")
                })?;

                if !segment.arguments.is_empty() {
                    return Err(Error::new_spanned(
                        tag,
                        "enum tag must be a primitive integer type",
                    ));
                }

                segment.ident.to_string()
            }
            None => "u8".to_string(),
        };

        let supported = matches!(
            tag_name.as_str(),
            "u8" | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "usize"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "isize"
        );

        if !supported {
            if let Some(tag) = &self.tag {
                return Err(Error::new_spanned(
                    tag,
                    "enum tag must be one of: u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize",
                ));
            }

            return Err(Error::new(proc_macro2::Span::call_site(), "unsupported enum tag type"));
        }

        let read = format_ident!("read_{}", tag_name);
        let write = format_ident!("write_{}", tag_name);
        Ok((read, write))
    }
}
