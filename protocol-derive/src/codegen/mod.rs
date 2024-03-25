pub mod enums;

use crate::attr::{self, LengthPrefix};
use proc_macro2::TokenStream;

pub fn read_fields(fields: &syn::Fields) -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields_named) => read_named_fields(fields_named),
        syn::Fields::Unnamed(ref fields_unnamed) => read_unnamed_fields(fields_unnamed),
        syn::Fields::Unit => quote!(),
    }
}

pub fn write_fields(fields: &syn::Fields) -> TokenStream {
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

            let read_field = read_field(field);
            let post = update_hints_after_read(field, &fields_named.named);

            quote! {
                #field_name : {
                    let res: protocol::Result<#field_ty> = #read_field;
                    #post
                    __hints.next_field();
                    res?
                }
            }
        })
        .collect();

    quote! { { #( #field_initializers ),* } }
}

fn read_field(field: &syn::Field) -> TokenStream {
    let attribs = attr::protocol(&field.attrs);
    if let Some(field_width) = attribs.bit_field {
        return quote! {
            protocol::BitField::read_field(__io_reader, __settings, &mut __hints, #field_width)
        };
    }
    if attribs.flexible_array_member {
        return quote! {
            protocol::FlexibleArrayMember::read_field(__io_reader, __settings, &mut __hints)
        };
    }
    if attribs.length_prefix.is_some() {
        quote! {
            protocol::WithLengthPrefix::read_field(__io_reader, __settings, &mut __hints)
        }
    } else {
        quote! {
            protocol::Parcel::read_field(__io_reader, __settings, &mut __hints)
        }
    }
}

fn write_field<T: quote::ToTokens>(field: &syn::Field, field_name: &T) -> TokenStream {
    let attribs = attr::protocol(&field.attrs);
    if let Some(field_width) = attribs.bit_field {
        return quote! {
            protocol::BitField::write_field(&self. #field_name, __io_writer, __settings, &mut __hints, #field_width)
        };
    }
    if attribs.flexible_array_member {
        return quote! {
            protocol::FlexibleArrayMember::write_field(&self. #field_name, __io_writer, __settings, &mut __hints)
        };
    }
    if attribs.length_prefix.is_some() {
        quote! {
            protocol::WithLengthPrefix::write_field(&self. #field_name, __io_writer, __settings, &mut __hints)
        }
    } else {
        quote! {
            protocol::Parcel::write_field(&self. #field_name, __io_writer, __settings, &mut __hints)
        }
    }
}

fn update_hints_after_read<'a>(
    field: &'a syn::Field,
    fields: impl IntoIterator<Item = &'a syn::Field> + Clone,
) -> TokenStream {
    if let Some((length_prefix_of, kind, prefix_subfield_names)) =
        length_prefix_of(field, fields.clone())
    {
        let kind = kind.path_expr();

        quote! {
            if let Ok(parcel) = res.as_ref() {
                __hints.set_field_length(#length_prefix_of,
                                         (parcel #(.#prefix_subfield_names)* ).clone() as usize,
                                         #kind);
            }
        }
    } else {
        quote! {}
    }
}

fn update_hints_after_write<'a>(
    field: &'a syn::Field,
    fields: impl IntoIterator<Item = &'a syn::Field> + Clone,
) -> TokenStream {
    if let Some((length_prefix_of, kind, prefix_subfield_names)) =
        length_prefix_of(field, fields.clone())
    {
        let field_name = &field.ident;
        let kind = kind.path_expr();

        quote! {
            if let Ok(()) = res {
                __hints.set_field_length(#length_prefix_of,
                                         (self.#field_name #(.#prefix_subfield_names)* ).clone() as usize,
                                         #kind);
            }
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
    field: &'a syn::Field,
    fields: impl IntoIterator<Item = &'a syn::Field> + Clone,
) -> Option<(usize, attr::LengthPrefixKind, Vec<syn::Ident>)> {
    let potential_prefix = field.ident.as_ref();

    let prefix_of = fields.clone().into_iter().find(|potential_prefix_of| {
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
    });

    if let Some(prefix_of) = prefix_of {
        let prefix_of_index = fields
            .clone()
            .into_iter()
            .position(|f| f == prefix_of)
            .unwrap();
        if let Some(LengthPrefix {
            kind,
            prefix_subfield_names,
            ..
        }) = attr::protocol(&prefix_of.attrs).length_prefix
        {
            Some((prefix_of_index, kind, prefix_subfield_names))
        } else {
            unreachable!()
        }
    } else {
        None
    }
}

fn write_named_fields(fields_named: &syn::FieldsNamed) -> TokenStream {
    let field_writers: Vec<_> = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;

            let write_field = write_field(field, field_name);
            let post = update_hints_after_write(field, &fields_named.named);

            quote! {
                {
                    let res = #write_field;
                    #post
                    __hints.next_field();
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
            let read_field = read_field(field);

            quote! {
                {
                    let res: protocol::Result<#field_ty> = #read_field;
                    __hints.next_field();
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
            let write_field = write_field(field, &field_index);

            quote! {
                {
                    let res = #write_field;
                    __hints.next_field();
                    res?
                }
            }
        })
        .collect();

    quote! { #( #field_writers );* }
}
