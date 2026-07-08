use syn::{Expr, Result, Token, meta::ParseNestedMeta};

#[derive(Clone)]
pub enum IgnoreAttr {
    Default,
    Value(Expr),
}

impl IgnoreAttr {
    pub fn parse(meta: &ParseNestedMeta<'_>) -> Result<Self> {
        if meta.input.peek(Token![=]) {
            let value = meta.value()?.parse()?;
            Ok(IgnoreAttr::Value(value))
        } else {
            Ok(IgnoreAttr::Default)
        }
    }
}
