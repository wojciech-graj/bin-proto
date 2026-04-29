use syn::{spanned::Spanned, Error, Result};

use crate::attr::{AttrKind, Attrs};

pub struct Enum<'a> {
    pub discriminant_ty: syn::Type,
    pub variants: Vec<EnumVariant>,
    pub parent_attrs: &'a Attrs,
}

pub struct EnumVariant {
    pub ident: syn::Ident,
    pub discriminant_value: Option<syn::Expr>,
    pub discriminant_other: bool,
    pub skip_encode: bool,
    pub skip_decode: bool,
    pub fields: syn::Fields,
}

impl<'a> Enum<'a> {
    pub fn try_new(
        parent_attrs: &'a Attrs,
        ast: &syn::DeriveInput,
        e: &syn::DataEnum,
    ) -> Result<Self> {
        let attrs = Attrs::parse(
            Some(parent_attrs),
            ast.attrs.as_slice(),
            Some(AttrKind::Enum),
            ast.span(),
        )?;
        Ok(Self {
            discriminant_ty: attrs.discriminant_type.ok_or_else(|| {
                Error::new(ast.span(), "enum missing 'discriminant_type' attribute.")
            })?,
            variants: e
                .variants
                .iter()
                .map(|variant| {
                    let attrs = Attrs::parse(
                        Some(parent_attrs),
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
                        skip_encode: attrs.skip_encode,
                        skip_decode: attrs.skip_decode,
                        fields: variant.fields.clone(),
                    })
                })
                .collect::<Result<_>>()?,
            parent_attrs,
        })
    }
}
