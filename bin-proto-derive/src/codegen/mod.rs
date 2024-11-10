pub mod enums;
pub mod trait_impl;

use crate::attr::{AttrKind, Attrs, Tag};
use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Error};

pub fn reads(fields: &syn::Fields) -> (TokenStream, TokenStream) {
    match *fields {
        syn::Fields::Named(ref fields) => read_named_fields(fields),
        syn::Fields::Unnamed(ref fields) => (quote!(), read_unnamed_fields(fields)),
        syn::Fields::Unit => (quote!(), quote!()),
    }
}

pub fn writes(fields: &syn::Fields, self_prefix: bool) -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields) => write_named_fields(fields, self_prefix),
        syn::Fields::Unnamed(ref fields) => write_unnamed_fields(fields, self_prefix),
        syn::Fields::Unit => quote!(),
    }
}

fn read_named_fields(fields_named: &syn::FieldsNamed) -> (TokenStream, TokenStream) {
    let fields: Vec<_> = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;

            let read = read(field);

            quote!(
                let #field_name : #field_ty = #read?;
            )
        })
        .collect();

    let field_initializers: Vec<_> = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;

            quote!(#field_name)
        })
        .collect();

    (
        quote!( #( #fields )* ),
        quote!( { #( #field_initializers ),* } ),
    )
}

fn read(field: &syn::Field) -> TokenStream {
    let attribs = match Attrs::for_kind(field.attrs.as_slice(), Some(AttrKind::Field)) {
        Ok(attribs) => attribs,
        Err(e) => return e.to_compile_error(),
    };

    if let Some(field_width) = attribs.bits {
        quote!(::bin_proto::BitFieldRead::read(__io_reader, __byte_order, __ctx, #field_width))
    } else if attribs.flexible_array_member {
        quote!(::bin_proto::FlexibleArrayMemberRead::read(
            __io_reader,
            __byte_order,
            __ctx
        ))
    } else if let Some(tag) = attribs.tag {
        match tag {
            Tag::External(tag) => {
                quote!(::bin_proto::TaggedRead::read(__io_reader, __byte_order, __ctx, #tag))
            }
            Tag::Prepend {
                typ,
                write_value: _,
            } => {
                quote!({
                    let __tag = ::bin_proto::ProtocolRead::read(__io_reader, __byte_order, __ctx)?;
                    ::bin_proto::TaggedRead::<#typ, _>::read(__io_reader, __byte_order, __ctx, __tag)
                })
            }
        }
    } else {
        quote!(::bin_proto::ProtocolRead::read(
            __io_reader,
            __byte_order,
            __ctx
        ))
    }
}

fn write(field: &syn::Field, field_name: &TokenStream) -> TokenStream {
    let attribs = match Attrs::for_kind(field.attrs.as_slice(), Some(AttrKind::Field)) {
        Ok(attribs) => attribs,
        Err(e) => return e.to_compile_error(),
    };

    let field_ref = if let Some(value) = attribs.write_value {
        let ty = &field.ty;
        quote!(&{
            let value: #ty = {#value};
            value
        })
    } else {
        field_name.clone()
    };

    if let Some(field_width) = attribs.bits {
        quote!(
            {
                ::bin_proto::BitFieldWrite::write(#field_ref, __io_writer, __byte_order, __ctx, #field_width)?
            }
        )
    } else if attribs.flexible_array_member {
        quote!(
            {
                ::bin_proto::UntaggedWrite::write(#field_ref, __io_writer, __byte_order, __ctx)?
            }
        )
    } else if let Some(tag) = attribs.tag {
        match tag {
            Tag::External(_) => quote!(
                {
                    ::bin_proto::UntaggedWrite::write(#field_ref, __io_writer, __byte_order, __ctx)?
                }
            ),
            Tag::Prepend {
                typ,
                write_value: Some(value),
            } => quote!(
                {
                    <#typ as ::bin_proto::ProtocolWrite<_>>::write(&{#value}, __io_writer, __byte_order, __ctx)?;
                    ::bin_proto::UntaggedWrite::write(#field_ref, __io_writer, __byte_order, __ctx)?
                }
            ),
            Tag::Prepend {
                typ: _,
                write_value: None,
            } => {
                return Error::new(field.span(), "Tag must specify 'write_value'")
                    .to_compile_error();
            }
        }
    } else {
        quote!(
            {
                ::bin_proto::ProtocolWrite::write(#field_ref, __io_writer, __byte_order, __ctx)?
            }
        )
    }
}

fn write_named_fields(fields_named: &syn::FieldsNamed, self_prefix: bool) -> TokenStream {
    let field_writers: Vec<_> = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            write(
                field,
                &if self_prefix {
                    quote!(&self. #field_name)
                } else {
                    quote!(#field_name)
                },
            )
        })
        .collect();

    quote!( #( #field_writers );* )
}

fn read_unnamed_fields(fields_unnamed: &syn::FieldsUnnamed) -> TokenStream {
    let field_initializers: Vec<_> = fields_unnamed
        .unnamed
        .iter()
        .map(|field| {
            let field_ty = &field.ty;
            let read = read(field);

            quote!(
                {
                    let res: #field_ty = #read?;
                    res
                }
            )
        })
        .collect();

    quote!( ( #( #field_initializers ),* ) )
}

fn write_unnamed_fields(fields_unnamed: &syn::FieldsUnnamed, self_prefix: bool) -> TokenStream {
    let field_writers: Vec<_> = fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(field_index, field)| {
            let field_index = syn::Index::from(field_index);
            write(
                field,
                &if self_prefix {
                    quote!(&self. #field_index)
                } else {
                    format!("field_{}", field_index.index).parse().unwrap()
                },
            )
        })
        .collect();

    quote!( #( #field_writers );* )
}
