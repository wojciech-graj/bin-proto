pub mod enums;

use crate::attr::{self, LengthPrefix};
use proc_macro2::TokenStream;

pub fn hints(fields: &syn::Fields) -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields) => {
            fields
                .named
                .iter()
                .filter_map(|field| {
                    let field_name = &field.ident;

                    if attr::protocol(&field.attrs).length_prefix.is_some() {Some(quote! {
                        let mut #field_name: Option<bin_proto::externally_length_prefixed::FieldLength> = None;
                    })} else {None}
                })
                .collect()
        }
        _ => quote!(),
    }
}

pub fn reads(fields: &syn::Fields) -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields_named) => read_named_fields(fields_named),
        syn::Fields::Unnamed(ref fields_unnamed) => read_unnamed_fields(fields_unnamed),
        syn::Fields::Unit => quote!(),
    }
}

pub fn writes(fields: &syn::Fields) -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields_named) => write_named_fields(fields_named),
        syn::Fields::Unnamed(ref fields_unnamed) => write_unnamed_fields(fields_unnamed),
        syn::Fields::Unit => quote!(),
    }
}

/// Generates code that builds a initializes
/// an item with named fields by parsing
/// each of the fields.
///
/// Returns  `{ ..field initializers.. }`.
fn read_named_fields(fields_named: &syn::FieldsNamed) -> TokenStream {
    let field_initializers: Vec<_> = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;

            let read = read(field);
            let post = update_hints_after_read(field, &fields_named.named);

            quote! {
                #field_name : {
                    let res: #field_ty = #read?;
                    #post
                    res
                }
            }
        })
        .collect();

    quote! { { #( #field_initializers ),* } }
}

fn read(field: &syn::Field) -> TokenStream {
    let attribs = attr::protocol(&field.attrs);
    if let Some(field_width) = attribs.bit_field {
        return quote! {
            bin_proto::BitField::read(__io_reader, __settings, __ctx, #field_width)
        };
    }
    if attribs.flexible_array_member {
        return quote! {
            bin_proto::FlexibleArrayMember::read(__io_reader, __settings, __ctx)
        };
    }
    if attribs.length_prefix.is_some() {
        let field_ident = field.clone().ident.unwrap().clone();
        quote! {
            bin_proto::ExternallyLengthPrefixed::read(__io_reader, __settings, __ctx, &#field_ident .unwrap())
        }
    } else {
        quote! {
            bin_proto::Protocol::read(__io_reader, __settings, __ctx)
        }
    }
}

fn write<T: quote::ToTokens>(field: &syn::Field, field_name: &T) -> TokenStream {
    let attribs = attr::protocol(&field.attrs);
    if let Some(field_width) = attribs.bit_field {
        return quote! {
            bin_proto::BitField::write(&self. #field_name, __io_writer, __settings, __ctx, #field_width)
        };
    }
    if attribs.flexible_array_member {
        return quote! {
            bin_proto::FlexibleArrayMember::write(&self. #field_name, __io_writer, __settings, __ctx)
        };
    }
    if attribs.length_prefix.is_some() {
        quote! {
            bin_proto::ExternallyLengthPrefixed::write(&self. #field_name, __io_writer, __settings, __ctx)
        }
    } else {
        quote! {
            bin_proto::Protocol::write(&self. #field_name, __io_writer, __settings, __ctx)
        }
    }
}

fn update_hints_after_read<'a>(
    field: &syn::Field,
    fields: impl IntoIterator<Item = &'a syn::Field> + Clone,
) -> TokenStream {
    if let Some(length_prefix_of) = length_prefix_of(field, fields.clone()) {
        let attrs = attr::protocol(&length_prefix_of.attrs);
        let kind = attrs.length_prefix.unwrap().kind.path_expr();
        let field_name = length_prefix_of.clone().ident.unwrap();

        quote! {
                #field_name = Some(bin_proto::externally_length_prefixed::FieldLength{
                    kind: #kind,
                    length: res as usize,
            });
        }
    } else {
        quote! {}
    }
}

/// If the given field is a length prefix of another field, that other field
/// returned here.
///
/// Returns `None` if the given field is not a disjoint length prefix.
///
/// Returns the field index of the field whose length is specified.
fn length_prefix_of<'a>(
    field: &syn::Field,
    fields: impl IntoIterator<Item = &'a syn::Field> + Clone,
) -> Option<syn::Field> {
    let potential_prefix = field.ident.as_ref();
    fields
        .clone()
        .into_iter()
        .find(|potential_prefix_of| {
            if let Some(LengthPrefix {
                ref prefix_field_name,
                ..
            }) = attr::protocol(&potential_prefix_of.attrs).length_prefix
            {
                if !fields
                    .clone()
                    .into_iter()
                    .any(|f| f.ident.as_ref() == Some(prefix_field_name))
                {
                    panic!(
                        "length prefix is invalid: there is no sibling field named '{}",
                        prefix_field_name
                    );
                }

                potential_prefix == Some(prefix_field_name)
            } else {
                false
            }
        })
        .cloned()
}

fn write_named_fields(fields_named: &syn::FieldsNamed) -> TokenStream {
    let field_writers: Vec<_> = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;

            let write = write(field, field_name);

            quote! {
                {
                    let res = #write;
                    res?
                }
            }
        })
        .collect();

    quote! { #( #field_writers );* }
}

fn read_unnamed_fields(fields_unnamed: &syn::FieldsUnnamed) -> TokenStream {
    let field_initializers: Vec<_> = fields_unnamed
        .unnamed
        .iter()
        .map(|field| {
            let field_ty = &field.ty;
            let read = read(field);

            quote! {
                {
                    let res: bin_proto::Result<#field_ty> = #read;
                    res?
                }
            }
        })
        .collect();

    quote! { ( #( #field_initializers ),* ) }
}

fn write_unnamed_fields(fields_unnamed: &syn::FieldsUnnamed) -> TokenStream {
    let field_writers: Vec<_> = fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(field_index, field)| {
            let field_index = syn::Index::from(field_index);
            let write = write(field, &field_index);

            quote! {
                {
                    let res = #write;
                    res?
                }
            }
        })
        .collect();

    quote! { #( #field_writers );* }
}
