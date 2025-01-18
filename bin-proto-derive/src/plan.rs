use crate::attr::{AttrKind, Attrs};
use syn::{spanned::Spanned, Error, Result};

pub struct Enum {
    pub discriminant_ty: syn::Type,
    pub variants: Vec<EnumVariant>,
}

pub struct EnumVariant {
    pub ident: syn::Ident,
    pub discriminant_value: syn::Expr,
    pub fields: syn::Fields,
}

impl Enum {
    pub fn try_new(ast: &syn::DeriveInput, e: &syn::DataEnum) -> Result<Self> {
        let attrs = Attrs::parse(ast.attrs.as_slice(), Some(AttrKind::Enum), ast.span())?;

        let plan = Self {
            discriminant_ty: attrs.discriminant_type.unwrap(),
            variants: e
                .variants
                .iter()
                .map(|variant| {
                    let attrs = Attrs::parse(
                        variant.attrs.as_slice(),
                        Some(AttrKind::Variant),
                        variant.span(),
                    )?;

                    let discriminant_value = match variant.discriminant.as_ref().map(|a| &a.1) {
                        Some(expr_lit) => expr_lit.clone(),
                        None => attrs.discriminant.ok_or_else(|| {
                            Error::new(variant.span(), "No discriminant for variant")
                        })?,
                    };

                    let variant = EnumVariant {
                        ident: variant.ident.clone(),
                        discriminant_value,
                        fields: variant.fields.clone(),
                    };
                    Ok(variant)
                })
                .collect::<Result<_>>()?,
        };
        Ok(plan)
    }
}
