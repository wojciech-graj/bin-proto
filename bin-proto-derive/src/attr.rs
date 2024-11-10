use std::fmt;

use proc_macro2::TokenStream;
use syn::{parenthesized, punctuated::Punctuated, spanned::Spanned, Error, Result, Token};

#[derive(Default)]
pub struct Attrs {
    pub discriminant_type: Option<syn::Type>,
    pub discriminant: Option<syn::Expr>,
    pub ctx: Option<Ctx>,
    pub ctx_generics: Option<Vec<syn::GenericParam>>,
    pub write_value: Option<syn::Expr>,
    pub bits: Option<syn::Expr>,
    pub flexible_array_member: bool,
    pub tag: Option<Tag>,
}

pub enum Ctx {
    Concrete(syn::Type),
    Bounds(Vec<syn::TypeParamBound>),
}

pub enum Tag {
    External(syn::Expr),
    Prepend {
        typ: syn::Type,
        write_value: Option<syn::Expr>,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AttrKind {
    Enum,
    Struct,
    Variant,
    Field,
}

impl fmt::Display for AttrKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttrKind::Enum => write!(f, "enum"),
            AttrKind::Struct => write!(f, "struct"),
            AttrKind::Variant => write!(f, "variant"),
            AttrKind::Field => write!(f, "field"),
        }
    }
}

macro_rules! validate_attr_kind {
    ($pat:pat, $kind:expr, $meta:expr, $attr:expr) => {
        if let Some(kind) = $kind {
            if !matches!(kind, $pat) {
                return Err($meta.error(format!(
                    "attribute '{}' cannot be applied to {}",
                    $attr, kind
                )));
            }
        }
    };
}

impl Attrs {
    pub fn ctx_ty(&self) -> TokenStream {
        if let Some(Ctx::Concrete(ctx)) = &self.ctx {
            quote!(#ctx)
        } else {
            quote!(__Ctx)
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn for_kind(attrs: &[syn::Attribute], kind: Option<AttrKind>) -> Result<Self> {
        let mut attribs = Attrs::default();

        let mut tag = None;
        let mut tag_type = None;
        let mut tag_value = None;

        let mut ctx = None;
        let mut ctx_bounds = None;

        for attr in attrs {
            if attr.path().is_ident("protocol") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("flexible_array_member") {
                        validate_attr_kind!(AttrKind::Field, kind, meta, "flexible_array_member");
                        attribs.flexible_array_member = true;
                    } else if meta.path.is_ident("discriminant_type") {
                        validate_attr_kind!(AttrKind::Enum, kind, meta, "discriminant_type");
                        attribs.discriminant_type = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("discriminant") {
                        validate_attr_kind!(AttrKind::Variant, kind, meta, "discriminant");
                        attribs.discriminant = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("ctx") {
                        validate_attr_kind!(AttrKind::Enum | AttrKind::Struct, kind, meta, "ctx");
                        ctx = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("ctx_generics") {
                        validate_attr_kind!(
                            AttrKind::Enum | AttrKind::Struct,
                            kind,
                            meta,
                            "ctx_generics"
                        );
                        let content;
                        parenthesized!(content in meta.input);
                        attribs.ctx_generics = Some(
                            Punctuated::<syn::GenericParam, Token![,]>::parse_separated_nonempty(
                                &content,
                            )?
                            .into_iter()
                            .collect(),
                        );
                    } else if meta.path.is_ident("ctx_bounds") {
                        validate_attr_kind!(
                            AttrKind::Enum | AttrKind::Struct,
                            kind,
                            meta,
                            "ctx_bounds"
                        );
                        let content;
                        parenthesized!(content in meta.input);
                        ctx_bounds = Some(
                            Punctuated::<syn::TypeParamBound, Token![,]>::parse_separated_nonempty(
                                &content,
                            )?
                            .into_iter()
                            .collect(),
                        );
                    } else if meta.path.is_ident("bits") {
                        validate_attr_kind!(AttrKind::Enum | AttrKind::Field, kind, meta, "bits");
                        attribs.bits = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("write_value") {
                        validate_attr_kind!(AttrKind::Field, kind, meta, "write_value");
                        attribs.write_value = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("tag") {
                        validate_attr_kind!(AttrKind::Field, kind, meta, "tag");
                        tag = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("tag_type") {
                        validate_attr_kind!(AttrKind::Field, kind, meta, "tag_type");
                        tag_type = Some(meta.value()?.parse()?);
                    } else if meta.path.is_ident("tag_value") {
                        validate_attr_kind!(AttrKind::Field, kind, meta, "tag_value");
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
                });
            }
            (None, None, None) => {}
            _ => return Err(Error::new(attrs[0].span(), "TODO")),
        }

        match (ctx, ctx_bounds) {
            (Some(ctx), None) => attribs.ctx = Some(Ctx::Concrete(ctx)),
            (None, Some(ctx_bounds)) => attribs.ctx = Some(Ctx::Bounds(ctx_bounds)),
            (None, None) => {}
            _ => return Err(Error::new(attrs[0].span(), "TODO")),
        }

        if [
            attribs.bits.is_some(),
            attribs.flexible_array_member,
            attribs.tag.is_some(),
        ]
        .iter()
        .filter(|b| **b)
        .count()
            > 1
        {
            return Err(Error::new(
                attrs[0].span(),
                "bits, flexible_array_member, and tag are mutually-exclusive attributes",
            ));
        }

        Ok(attribs)
    }
}
