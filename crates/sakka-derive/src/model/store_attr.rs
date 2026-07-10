use syn::{Expr, Ident, Result, meta::ParseNestedMeta};

#[derive(Clone)]
pub struct StoreAttr {
    pub field: Ident,
    pub expr: Expr,
}

impl StoreAttr {
    pub fn parse(meta: &ParseNestedMeta<'_>) -> Result<Self> {
        // Backward-compatible form: #[sakka(store = target_field)]
        if meta.input.peek(syn::Token![=]) {
            let field: Ident = meta.value()?.parse()?;
            return Ok(Self { field, expr: syn::parse_quote!(value) });
        }

        // Expression form: #[sakka(store(target_field = value_expr))]
        let mut result: Option<Self> = None;

        meta.parse_nested_meta(|nested| {
            if result.is_some() {
                return Err(nested.error("store may only have one assignment"));
            }

            let Some(field) = nested.path.get_ident().cloned() else {
                return Err(nested.error("store target must be an identifier"));
            };

            let expr: Expr = nested.value()?.parse()?;
            result = Some(Self { field, expr });
            Ok(())
        })?;

        result.ok_or_else(|| meta.error("expected store = field or store(field = value_expr)"))
    }
}
