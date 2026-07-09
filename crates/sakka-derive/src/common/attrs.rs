use alloc::{format, vec::Vec};

use syn::{DeriveInput, Error, Expr, Path, Result};

pub struct PendingAttrs {
    entries: Vec<PendingAttr>,
}

#[derive(Clone)]
struct PendingAttr {
    key: Path,
    value: Option<Expr>,
}

impl PendingAttrs {
    pub fn collect(input: &DeriveInput) -> Result<Self> {
        let mut entries = Vec::new();

        for attr in &input.attrs {
            if !attr.path().is_ident("sakka") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                let value =
                    if meta.input.is_empty() { None } else { Some(meta.value()?.parse::<Expr>()?) };

                entries.push(PendingAttr { key: meta.path.clone(), value });
                Ok(())
            })?;
        }

        Ok(Self { entries })
    }

    fn take_value(&mut self, name: &str) -> Result<Option<Expr>> {
        let matches: Vec<_> = self
            .entries
            .iter()
            .enumerate()
            .filter_map(|(idx, attr)| attr.key.is_ident(name).then_some(idx))
            .collect();

        if matches.is_empty() {
            return Ok(None);
        }

        if matches.len() > 1 {
            let duplicate = &self.entries[matches[1]];
            return Err(Error::new_spanned(&duplicate.key, format!("{name} already specified")));
        }

        let index = matches[0];
        let attr = self.entries.remove(index);

        match attr.value {
            Some(value) => Ok(Some(value)),
            None => Err(Error::new_spanned(&attr.key, format!("{name} requires a value"))),
        }
    }

    pub fn take_path(&mut self, name: &str) -> Result<Option<Path>> {
        self.take_value(name)?
            .map(|value| match value {
                Expr::Path(expr_path) => Ok(expr_path.path),
                _ => Err(Error::new_spanned(value, format!("{name} must be a path"))),
            })
            .transpose()
    }

    pub fn take_expr(&mut self, name: &str) -> Result<Option<Expr>> {
        self.take_value(name)
    }

    pub fn ensure_empty(self) -> Result<()> {
        if let Some(unknown) = self.entries.first() {
            return Err(Error::new_spanned(&unknown.key, "unknown sakka attribute"));
        }

        Ok(())
    }
}
