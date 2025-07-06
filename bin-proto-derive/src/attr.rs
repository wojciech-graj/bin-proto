use proc_macro2::{Span, TokenStream};
use std::fmt;
use syn::{parenthesized, punctuated::Punctuated, Error, Result, Token};

#[derive(Default)]
pub struct Attrs {
    pub bits: Option<syn::Expr>,
    pub ctx: Option<Ctx>,
    pub ctx_generics: Option<Vec<syn::GenericParam>>,
    pub default: bool,
    pub discriminant: Option<syn::Expr>,
    pub discriminant_type: Option<syn::Type>,
    pub flexible_array_member: bool,
    pub magic: Option<syn::Expr>,
    pub pad_after: Option<syn::Expr>,
    pub pad_before: Option<syn::Expr>,
    pub tag: Option<Tag>,
    pub write_value: Option<syn::Expr>,
}

pub enum Ctx {
    Concrete(syn::Type),
    Bounds(Vec<syn::TypeParamBound>),
}

#[allow(clippy::large_enum_variant)]
pub enum Tag {
    External(syn::Expr),
    Prepend {
        typ: syn::Type,
        write_value: Option<syn::Expr>,
        bits: Option<syn::Expr>,
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
            Self::Enum => write!(f, "enum"),
            Self::Struct => write!(f, "struct"),
            Self::Variant => write!(f, "variant"),
            Self::Field => write!(f, "field"),
        }
    }
}

macro_rules! expect_attr_kind {
    ($pat:pat, $kind:expr, $meta:expr) => {
        if let Some(kind) = $kind {
            if !matches!(kind, $pat) {
                return Err($meta.error(format!(
                    "attribute '{}' cannot be applied to {}",
                    $meta.path.require_ident()?,
                    kind
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

    pub fn decode_magic(&self) -> TokenStream {
        if let Some(magic) = &self.magic {
            quote!(
                let magic: [u8; (#magic).len()] = ::bin_proto::BitDecode::decode::<_, __E>(
                    __io_reader,
                    __ctx,
                    ()
                )?;
                if magic != *(#magic) {
                    return ::core::result::Result::Err(
                        ::bin_proto::Error::Magic{ expected: #magic, actual: magic.to_vec() }
                    );
                }
            )
        } else {
            TokenStream::new()
        }
    }

    pub fn encode_magic(&self) -> TokenStream {
        if let Some(magic) = &self.magic {
            quote!(::bin_proto::BitEncode::encode::<_, __E>(#magic, __io_writer, __ctx, ())?;)
        } else {
            TokenStream::new()
        }
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    pub fn parse(attribs: &[syn::Attribute], kind: Option<AttrKind>, span: Span) -> Result<Self> {
        let mut attrs = Self::default();

        let mut tag = None;
        let mut tag_type = None;
        let mut tag_value = None;
        let mut tag_bits = None;

        let mut ctx = None;
        let mut ctx_bounds = None;

        for attr in attribs {
            if !attr.path().is_ident("codec") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                let Some(ident) = meta.path.get_ident() else {
                    return Err(meta.error("unrecognized attribute"));
                };

                match ident.to_string().as_str() {
                    "flexible_array_member" => {
                        expect_attr_kind!(AttrKind::Field, kind, meta);
                        attrs.flexible_array_member = true;
                    }
                    "discriminant_type" => {
                        expect_attr_kind!(AttrKind::Enum, kind, meta);
                        attrs.discriminant_type = Some(meta.value()?.parse()?);
                    }
                    "discriminant" => {
                        expect_attr_kind!(AttrKind::Variant, kind, meta);
                        attrs.discriminant = Some(meta.value()?.parse()?);
                    }
                    "ctx" => {
                        expect_attr_kind!(AttrKind::Enum | AttrKind::Struct, kind, meta);
                        ctx = Some(meta.value()?.parse()?);
                    }
                    "ctx_generics" => {
                        expect_attr_kind!(AttrKind::Enum | AttrKind::Struct, kind, meta);
                        let content;
                        parenthesized!(content in meta.input);
                        attrs.ctx_generics = Some(
                            Punctuated::<syn::GenericParam, Token![,]>::parse_separated_nonempty(
                                &content,
                            )?
                            .into_iter()
                            .collect(),
                        );
                    }
                    "ctx_bounds" => {
                        expect_attr_kind!(AttrKind::Enum | AttrKind::Struct, kind, meta);
                        let content;
                        parenthesized!(content in meta.input);
                        ctx_bounds = Some(
                            Punctuated::<syn::TypeParamBound, Token![,]>::parse_separated_nonempty(
                                &content,
                            )?
                            .into_iter()
                            .collect(),
                        );
                    }
                    "bits" => {
                        expect_attr_kind!(AttrKind::Enum | AttrKind::Field, kind, meta);
                        attrs.bits = Some(meta.value()?.parse()?);
                    }
                    "write_value" => {
                        expect_attr_kind!(AttrKind::Field, kind, meta);
                        attrs.write_value = Some(meta.value()?.parse()?);
                    }
                    "tag" => {
                        expect_attr_kind!(AttrKind::Field, kind, meta);
                        tag = Some(meta.value()?.parse()?);
                    }
                    "tag_type" => {
                        expect_attr_kind!(AttrKind::Field, kind, meta);
                        tag_type = Some(meta.value()?.parse()?);
                    }
                    "tag_value" => {
                        expect_attr_kind!(AttrKind::Field, kind, meta);
                        tag_value = Some(meta.value()?.parse()?);
                    }
                    "tag_bits" => {
                        expect_attr_kind!(AttrKind::Field, kind, meta);
                        tag_bits = Some(meta.value()?.parse()?);
                    }
                    "default" => {
                        expect_attr_kind!(AttrKind::Field, kind, meta);
                        attrs.default = true;
                    }
                    "pad_before" => {
                        expect_attr_kind!(
                            AttrKind::Enum | AttrKind::Struct | AttrKind::Field,
                            kind,
                            meta
                        );
                        attrs.pad_before = Some(meta.value()?.parse()?);
                    }
                    "pad_after" => {
                        expect_attr_kind!(
                            AttrKind::Enum | AttrKind::Struct | AttrKind::Field,
                            kind,
                            meta
                        );
                        attrs.pad_after = Some(meta.value()?.parse()?);
                    }
                    "magic" => {
                        expect_attr_kind!(AttrKind::Struct | AttrKind::Field, kind, meta);
                        attrs.magic = Some(meta.value()?.parse()?);
                    }
                    _ => {
                        return Err(meta.error("unrecognized attribute"));
                    }
                }

                Ok(())
            })?;
        }

        match (tag, tag_type, tag_value, tag_bits) {
            (Some(tag), None, None, None) => attrs.tag = Some(Tag::External(tag)),
            (None, Some(tag_type), tag_value, tag_bits) => {
                attrs.tag = Some(Tag::Prepend {
                    typ: tag_type,
                    write_value: tag_value,
                    bits: tag_bits,
                });
            }
            (None, None, None, None) => {}
            _ => {
                return Err(Error::new(
                    span,
                    "invalid configuration of 'tag', 'tag_type', or 'tag_value' attributes.",
                ));
            }
        }

        match (ctx, ctx_bounds) {
            (Some(ctx), None) => attrs.ctx = Some(Ctx::Concrete(ctx)),
            (None, Some(ctx_bounds)) => attrs.ctx = Some(Ctx::Bounds(ctx_bounds)),
            (None, None) => {}
            _ => {
                return Err(Error::new(
                    span,
                    "use of mutually exclusive 'ctx' and 'ctx_bounds' attributes.",
                ));
            }
        }

        if [
            attrs.bits.is_some(),
            attrs.flexible_array_member,
            attrs.tag.is_some(),
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

        Ok(attrs)
    }
}
