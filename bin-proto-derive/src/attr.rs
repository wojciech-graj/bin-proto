use crate::format::{self, Format};

use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct LengthPrefix {
    pub kind: LengthPrefixKind,
    pub prefix_field_name: syn::Ident,
}

#[derive(Debug, Default)]
pub struct Attrs {
    pub discriminant_format: Option<format::Enum>,
    pub discriminant: Option<syn::Lit>,
    pub bit_field: Option<u32>,
    pub flexible_array_member: bool,
    pub length_prefix: Option<LengthPrefix>,
}

impl Attrs {
    fn validate(&self) {
        if [
            self.bit_field.is_some(),
            self.flexible_array_member,
            self.length_prefix.is_some(),
        ]
        .iter()
        .filter(|b| **b)
        .count()
            > 1
        {
            panic!("'bits', 'flexible_array_member', and 'length_prefix' attributes are mutually-exclusive.")
        }
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
                    syn::NestedMeta::Meta(syn::Meta::List(nested_list)) => {
                        match &nested_list
                            .path
                            .get_ident()
                            .expect("meta is not an ident")
                            .to_string()[..]
                        {
                            "length_prefix" => {
                                attribs.length_prefix = Some(attr_length_prefix(nested_list))
                            }
                            "discriminant" => {
                                attribs.discriminant = Some(attr_field_discriminant(nested_list))
                            }
                            name => panic!("#[protocol({})] is not valid", name),
                        }
                    }
                    syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                        match name_value.path.get_ident() {
                            Some(ident) => match &ident.to_string()[..] {
                                "discriminant" => {
                                    attribs.discriminant_format =
                                        Some(attr_enum_discriminant(name_value))
                                }
                                "bits" => attribs.bit_field = Some(attr_bits(name_value)),
                                ident => panic!("got unexpected '{}'", ident),
                            },
                            None => panic!("parsed string was not an identifier"),
                        }
                    }
                    syn::NestedMeta::Meta(syn::Meta::Path(path)) => match path.get_ident() {
                        Some(ident) => match ident.to_string().as_str() {
                            "flexible_array_member" => attribs.flexible_array_member = true,
                            _ => panic!("got unexpected '{}'", ident),
                        },
                        None => panic!("parsed string was not an identifier"),
                    },
                    _ => {
                        panic!("unrecognized #[protocol(..)] attribute")
                    }
                };
            }
        }
        attribs.validate();
        attribs
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LengthPrefixKind {
    Bytes,
    Elements,
}

impl LengthPrefixKind {
    /// Gets a path to the length prefix in the bin_proto crate.
    pub fn path_expr(&self) -> TokenStream {
        match *self {
            LengthPrefixKind::Bytes => {
                quote!(bin_proto::externally_length_prefixed::LengthPrefixKind::Bytes)
            }
            LengthPrefixKind::Elements => {
                quote!(bin_proto::externally_length_prefixed::LengthPrefixKind::Elements)
            }
        }
    }
}

fn attr_length_prefix(nested_list: syn::MetaList) -> LengthPrefix {
    let nested_list = expect::meta_list::nested_list(nested_list).expect("expected a nested list");
    let prefix_kind = match &nested_list
        .path
        .get_ident()
        .expect("nested list is not an ident")
        .to_string()[..]
    {
        "bytes" => LengthPrefixKind::Bytes,
        "elements" => LengthPrefixKind::Elements,
        invalid_prefix => {
            panic!("invalid length prefix type: '{}'", invalid_prefix)
        }
    };

    let length_prefix_expr = expect::meta_list::single_element(nested_list).unwrap();
    let prefix_field_name = match length_prefix_expr {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => match path.get_ident() {
            Some(field_ident) => field_ident.clone(),
            None => panic!("path is not an ident"),
        },
        _ => panic!("unexpected format for length prefix attribute"),
    };

    LengthPrefix {
        kind: prefix_kind,
        prefix_field_name,
    }
}

fn attr_field_discriminant(nested_list: syn::MetaList) -> syn::Lit {
    expect::meta_list::single_literal(nested_list).expect("expected a single literal")
}

fn attr_enum_discriminant(name_value: syn::MetaNameValue) -> format::Enum {
    match name_value.lit {
        syn::Lit::Str(s) => match format::Enum::from_str(&s.value()) {
            Ok(f) => f,
            Err(()) => {
                panic!("invalid enum discriminant format: '{}'", s.value())
            }
        },
        _ => panic!("discriminant format mut be string"),
    }
}

fn attr_bits(name_value: syn::MetaNameValue) -> u32 {
    match name_value.lit {
        syn::Lit::Int(i) => match i.base10_parse() {
            Ok(i) => i,
            Err(_) => {
                panic!("bitfield must have constant unsigned size.")
            }
        },
        _ => panic!("bitfield size must be an integer"),
    }
}

mod expect {
    pub mod meta_list {
        pub fn nested_list(list: syn::MetaList) -> Result<syn::MetaList, ()> {
            assert!(list.nested.len() == 1, "list should only have one item");
            match list.nested.into_iter().next().unwrap() {
                syn::NestedMeta::Meta(syn::Meta::List(nested)) => Ok(nested),
                _ => Err(()),
            }
        }

        /// Expects a list with a single element.
        pub fn single_element(list: syn::MetaList) -> Result<syn::NestedMeta, ()> {
            assert!(list.nested.len() == 1, "list should only have one item");
            Ok(list.nested.into_iter().next().unwrap())
        }

        /// A single word `name(literal)`.
        pub fn single_literal(list: syn::MetaList) -> Result<syn::Lit, ()> {
            single_element(list).and_then(|nested| match nested {
                syn::NestedMeta::Lit(lit) => Ok(lit),
                _ => Err(()),
            })
        }
    }
}
