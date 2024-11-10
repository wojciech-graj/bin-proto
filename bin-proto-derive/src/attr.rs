use proc_macro2::{Span, TokenStream};
use syn::{punctuated::Punctuated, spanned::Spanned, token::Plus, Error, Result};

#[derive(Default)]
pub struct Attrs {
    pub discriminant_type: Option<syn::Type>,
    pub discriminant: Option<syn::Expr>,
    pub ctx: Option<syn::Type>,
    pub ctx_bounds: Option<Punctuated<syn::TypeParamBound, Plus>>,
    pub write_value: Option<syn::Expr>,
    pub bits: Option<syn::Expr>,
    pub flexible_array_member: bool,
    pub tag: Option<Tag>,
}

pub enum Tag {
    External(syn::Expr),
    Prepend {
        typ: syn::Type,
        write_value: Option<syn::Expr>,
    },
}

impl Attrs {
    #[allow(clippy::too_many_lines)]
    pub fn validate_enum(&self, span: Span) -> Result<()> {
        if self.discriminant_type.is_none() {
            return Err(Error::new(
                span,
                "expected discriminant_type attribute for enum",
            ));
        }
        if self.discriminant.is_some() {
            return Err(Error::new(
                span,
                "unexpected discriminant attribute for enum",
            ));
        }
        if self.ctx.is_some() && self.ctx_bounds.is_some() {
            return Err(Error::new(
                span,
                "cannot specify ctx and ctx_bounds simultaneously",
            ));
        }
        if self.write_value.is_some() {
            return Err(Error::new(
                span,
                "unexpected write_value attribute for enum",
            ));
        }
        if self.flexible_array_member {
            return Err(Error::new(
                span,
                "unexpected flexible_array_member attribute for enum",
            ));
        }
        if self.tag.is_some() {
            return Err(Error::new(span, "unexpected tag attribute for enum"));
        }
        Ok(())
    }

    pub fn validate_variant(&self, span: Span) -> Result<()> {
        if self.discriminant_type.is_some() {
            return Err(Error::new(
                span,
                "unexpected discriminant_type attribute for variant",
            ));
        }
        if self.ctx.is_some() {
            return Err(Error::new(span, "unexpected ctx attribute for variant"));
        }
        if self.ctx_bounds.is_some() {
            return Err(Error::new(
                span,
                "unexpected ctx_bounds attribute for variant",
            ));
        }
        if self.write_value.is_some() {
            return Err(Error::new(
                span,
                "unexpected write_value attribute for variant",
            ));
        }
        if self.bits.is_some() {
            return Err(Error::new(span, "unexpected bits attribute for variant"));
        }
        if self.flexible_array_member {
            return Err(Error::new(
                span,
                "unexpected flexible_array_member attribute for variant",
            ));
        }
        if self.tag.is_some() {
            return Err(Error::new(span, "unexpected tag attribute for variant"));
        }
        Ok(())
    }

    pub fn validate_field(&self, span: Span) -> Result<()> {
        if self.discriminant_type.is_some() {
            return Err(Error::new(
                span,
                "unexpected discriminant_type attribute for field",
            ));
        }
        if self.discriminant.is_some() {
            return Err(Error::new(
                span,
                "unexpected discriminant attribute for field",
            ));
        }
        if self.ctx.is_some() {
            return Err(Error::new(span, "unexpected ctx attribute for variant"));
        }
        if self.ctx_bounds.is_some() {
            return Err(Error::new(
                span,
                "unexpected ctx_bounds attribute for variant",
            ));
        }
        if [
            self.bits.is_some(),
            self.flexible_array_member,
            self.tag.is_some(),
        ]
        .iter()
        .filter(|b| **b)
        .count()
            > 1
        {
            return Err(Error::new(
                span,
                "bits, flexible_array_member, and tag are mutually-exclusive attributes",
            ));
        }
        Ok(())
    }

    pub fn ctx_ty(&self) -> TokenStream {
        self.ctx
            .as_ref()
            .map(|ctx| quote!(#ctx))
            .unwrap_or(quote!(__Ctx))
    }
}

impl TryFrom<&[syn::Attribute]> for Attrs {
    type Error = syn::Error;

    fn try_from(attrs: &[syn::Attribute]) -> Result<Self> {
        let mut attribs = Attrs::default();
        let mut tag = None;
        let mut tag_type = None;
        let mut tag_value = None;

        for attr in attrs {
            if attr.path().is_ident("protocol") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("flexible_array_member") {
                        attribs.flexible_array_member = true;
                    } else if meta.path.is_ident("discriminant_type") {
                        attribs.discriminant_type = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("discriminant") {
                        attribs.discriminant = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("ctx") {
                        attribs.ctx = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("ctx_bounds") {
                        attribs.ctx_bounds =
                            Some(Punctuated::parse_separated_nonempty(meta.value()?)?);
                    } else if meta.path.is_ident("bits") {
                        attribs.bits = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("write_value") {
                        attribs.write_value = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("tag") {
                        tag = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("tag_type") {
                        tag_type = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("tag_value") {
                        tag_value = Some(meta.value()?.parse()?);
                    } else {
                        return Err(meta.error("unrecognized protocol"));
                    }
                    Ok(())
                })?;
            }
        }

        match (tag, tag_type, tag_value) {
            (Some(tag), None, None) => attribs.tag = Some(Tag::External(tag)),
            (None, Some(tag_type), tag_value) => {
                attribs.tag = Some(Tag::Prepend {
                    typ: tag_type,
                    write_value: tag_value,
                })
            }
            (None, None, None) => {}
            _ => return Err(Error::new(attrs[0].span(), "TODO")),
        }

        Ok(attribs)
    }
}
