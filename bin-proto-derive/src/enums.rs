use crate::attr::{AttrKind, Attrs};
use syn::{spanned::Spanned, Error, Result};

pub struct Enum {
    pub discriminant_ty: syn::Type,
    pub variants: Vec<EnumVariant>,
}

pub struct EnumVariant {
    pub ident: syn::Ident,
    pub discriminant_value: Option<syn::Expr>,
    pub discriminant_other: bool,
    pub fields: syn::Fields,
}

impl Enum {
    pub fn try_new(ast: &syn::DeriveInput, e: &syn::DataEnum) -> Result<Self> {
        let attrs = Attrs::parse(ast.attrs.as_slice(), Some(AttrKind::Enum), ast.span())?;

        Ok(Self {
            discriminant_ty: attrs.discriminant_type.ok_or_else(|| {
                Error::new(ast.span(), "enum missing 'discriminant_type' attribute.")
            })?,
            variants: e
                .variants
                .iter()
                .map(|variant| {
                    let attrs = Attrs::parse(
                        variant.attrs.as_slice(),
                        Some(AttrKind::Variant),
                        variant.span(),
                    )?;

                    Ok(EnumVariant {
                        ident: variant.ident.clone(),
                        discriminant_value: attrs
                            .discriminant
                            .or_else(|| variant.discriminant.as_ref().map(|a| a.1.clone())),
                        discriminant_other: attrs.other,
                        fields: variant.fields.clone(),
                    })
                })
                .collect::<Result<_>>()?,
        })
    }
}
