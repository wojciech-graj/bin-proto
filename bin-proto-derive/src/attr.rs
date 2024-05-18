use proc_macro2::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, token::Add, TypeParamBound};

#[derive(Debug, Default)]
pub struct Attrs {
    pub discriminant_type: Option<syn::Type>,
    pub discriminant: Option<syn::Expr>,
    pub ctx: Option<syn::Type>,
    pub ctx_bounds: Option<Punctuated<TypeParamBound, Add>>,
    pub write_value: Option<syn::Expr>,
    pub bits: Option<u32>,
    pub flexible_array_member: bool,
    pub length: Option<syn::Expr>,
}

impl Attrs {
    pub fn validate_enum(&self) {
        if self.discriminant_type.is_none() {
            panic!("expected discriminant_type attribute for enum")
        }
        if self.discriminant.is_some() {
            panic!("unexpected discriminant attribute for enum")
        }
        if self.ctx.is_some() && self.ctx_bounds.is_some() {
            panic!("cannot specify ctx and ctx_bounds simultaneously")
        }
        if self.write_value.is_some() {
            panic!("unexpected write_value attribute for enum")
        }
        if self.flexible_array_member {
            panic!("unexpected flexible_array_member attribute for enum")
        }
        if self.length.is_some() {
            panic!("unexpected length attribute for enum")
        }
    }

    pub fn validate_variant(&self) {
        if self.discriminant_type.is_some() {
            panic!("unexpected discriminant_type attribute for variant")
        }
        if self.ctx.is_some() {
            panic!("unexpected ctx attribute for variant")
        }
        if self.ctx_bounds.is_some() {
            panic!("unexpected ctx_bounds attribute for variant")
        }
        if self.write_value.is_some() {
            panic!("unexpected write_value attribute for variant")
        }
        if self.bits.is_some() {
            panic!("unexpected bits attribute for variant")
        }
        if self.flexible_array_member {
            panic!("unexpected flexible_array_member attribute for variant")
        }
        if self.length.is_some() {
            panic!("unexpected length attribute for variant")
        }
    }

    pub fn validate_field(&self) {
        if self.discriminant_type.is_some() {
            panic!("unexpected discriminant_type attribute for field")
        }
        if self.discriminant.is_some() {
            panic!("unexpected discriminant attribute for field")
        }
        if self.ctx.is_some() {
            panic!("unexpected ctx attribute for variant")
        }
        if self.ctx_bounds.is_some() {
            panic!("unexpected ctx_bounds attribute for variant")
        }
        if [
            self.bits.is_some(),
            self.flexible_array_member,
            self.length.is_some(),
        ]
        .iter()
        .filter(|b| **b)
        .count()
            > 1
        {
            panic!("bits, flexible_array_member, and length are mutually-exclusive attributes")
        }
    }

    pub fn ctx_tok(&self) -> TokenStream {
        self.ctx
            .clone()
            .map(|ctx| quote!(#ctx))
            .unwrap_or(quote!(__Ctx))
    }
}

impl From<&[syn::Attribute]> for Attrs {
    fn from(value: &[syn::Attribute]) -> Self {
        let meta_lists = value.iter().filter_map(|attr| match attr.parse_meta() {
            Ok(syn::Meta::List(meta_list)) => {
                if meta_list.path.get_ident()
                    == Some(&syn::Ident::new("protocol", proc_macro2::Span::call_site()))
                {
                    Some(meta_list)
                } else {
                    None
                }
            }
            _ => None,
        });

        let mut attribs = Attrs::default();
        for meta_list in meta_lists {
            for meta in meta_list.nested {
                match meta {
                    syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                        match name_value.path.get_ident() {
                            Some(ident) => match &ident.to_string()[..] {
                                "discriminant_type" => {
                                    attribs.discriminant_type =
                                        Some(meta_name_value_to_parse(name_value))
                                }
                                "discriminant" => {
                                    attribs.discriminant =
                                        Some(meta_name_value_to_parse(name_value))
                                }
                                "ctx" => attribs.ctx = Some(meta_name_value_to_parse(name_value)),
                                "ctx_bounds" => {
                                    attribs.ctx_bounds =
                                        Some(meta_name_value_to_punctuated(name_value))
                                }
                                "bits" => attribs.bits = Some(meta_name_value_to_u32(name_value)),
                                "write_value" => {
                                    attribs.write_value = Some(meta_name_value_to_parse(name_value))
                                }
                                "length" => {
                                    attribs.length = Some(meta_name_value_to_parse(name_value))
                                }
                                ident => panic!("unrecognised #[protocol({})]", ident),
                            },
                            None => panic!("failed to parse #[protocol(...)]"),
                        }
                    }
                    syn::NestedMeta::Meta(syn::Meta::Path(path)) => match path.get_ident() {
                        Some(ident) => match ident.to_string().as_str() {
                            "flexible_array_member" => attribs.flexible_array_member = true,
                            _ => panic!("unrecognised #[protocol({})]", ident),
                        },
                        None => panic!("parsed string was not an identifier"),
                    },
                    _ => {
                        panic!("unrecognized #[protocol(..)] attribute")
                    }
                };
            }
        }
        attribs
    }
}

fn meta_name_value_to_parse<T: syn::parse::Parse>(name_value: syn::MetaNameValue) -> T {
    match name_value.lit {
        syn::Lit::Str(s) => match syn::parse_str::<T>(s.value().as_str()) {
            Ok(f) => f,
            Err(_) => {
                panic!("Failed to parse '{}'", s.value())
            }
        },
        _ => panic!("#[protocol(... = \"...\")] must be string"),
    }
}

fn meta_name_value_to_u32(name_value: syn::MetaNameValue) -> u32 {
    match name_value.lit {
        syn::Lit::Int(i) => match i.base10_parse() {
            Ok(i) => i,
            Err(_) => {
                panic!("Failed to parse integer from '{}'", i)
            }
        },
        _ => panic!("bitfield size must be an integer"),
    }
}

fn meta_name_value_to_punctuated<T: syn::parse::Parse, P: syn::parse::Parse>(
    name_value: syn::MetaNameValue,
) -> Punctuated<T, P> {
    match name_value.lit {
        syn::Lit::Str(s) => match Punctuated::parse_terminated.parse_str(s.value().as_str()) {
            Ok(f) => f,
            Err(_) => {
                panic!("Failed to parse '{}'", s.value())
            }
        },
        _ => panic!("#[protocol(... = \"...\")] must be string"),
    }
}
