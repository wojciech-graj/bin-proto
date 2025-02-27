pub mod enums;
pub mod trait_impl;

use crate::attr::{AttrKind, Attrs, Tag};
use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Error};

pub fn decodes(fields: &syn::Fields) -> (TokenStream, TokenStream) {
    match *fields {
        syn::Fields::Named(ref fields) => decode_named_fields(fields),
        syn::Fields::Unnamed(ref fields) => (quote!(), decode_unnamed_fields(fields)),
        syn::Fields::Unit => (quote!(), quote!()),
    }
}

pub fn encodes(fields: &syn::Fields, self_prefix: bool) -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields) => encode_named_fields(fields, self_prefix),
        syn::Fields::Unnamed(ref fields) => encode_unnamed_fields(fields, self_prefix),
        syn::Fields::Unit => quote!(),
    }
}

fn decode_named_fields(fields_named: &syn::FieldsNamed) -> (TokenStream, TokenStream) {
    let fields: Vec<_> = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;

            let decode = decode(field);

            quote!(
                let #field_name : #field_ty = #decode?;
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

fn decode(field: &syn::Field) -> TokenStream {
    let attribs = match Attrs::parse(field.attrs.as_slice(), Some(AttrKind::Field), field.span()) {
        Ok(attribs) => attribs,
        Err(e) => return e.to_compile_error(),
    };

    if let Some(field_width) = attribs.bits {
        quote!(::bin_proto::BitDecode::decode(__io_reader, __byte_order, __ctx, ::bin_proto::Bits(#field_width)))
    } else if attribs.flexible_array_member {
        quote!(::bin_proto::BitDecode::decode(
            __io_reader,
            __byte_order,
            __ctx,
            ::bin_proto::Untagged
        ))
    } else if let Some(tag) = attribs.tag {
        match tag {
            Tag::External(tag) => {
                quote!(::bin_proto::BitDecode::decode(__io_reader, __byte_order, __ctx, ::bin_proto::Tag(#tag)))
            }
            Tag::Prepend {
                typ,
                write_value: _,
            } => {
                quote!({
                    let __tag: #typ = ::bin_proto::BitDecode::decode(__io_reader, __byte_order, __ctx, ())?;
                    ::bin_proto::BitDecode::decode(__io_reader, __byte_order, __ctx, ::bin_proto::Tag(__tag))
                })
            }
        }
    } else {
        quote!(::bin_proto::BitDecode::decode(
            __io_reader,
            __byte_order,
            __ctx,
            (),
        ))
    }
}

fn encode(field: &syn::Field, field_name: &TokenStream) -> TokenStream {
    let attribs = match Attrs::parse(field.attrs.as_slice(), Some(AttrKind::Field), field.span()) {
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
                ::bin_proto::BitEncode::encode(#field_ref, __io_writer, __byte_order, __ctx, ::bin_proto::Bits(#field_width))?
            }
        )
    } else if let Some(tag) = attribs.tag {
        match tag {
            Tag::External(_) => quote!(
                {
                    ::bin_proto::BitEncode::encode(#field_ref, __io_writer, __byte_order, __ctx, ::bin_proto::Untagged)?
                }
            ),
            Tag::Prepend {
                typ,
                write_value: Some(value),
            } => quote!(
                {
                    <#typ as ::bin_proto::BitEncode::<_>>::encode(&{#value}, __io_writer, __byte_order, __ctx, ())?;
                    ::bin_proto::BitEncode::encode(#field_ref, __io_writer, __byte_order, __ctx, ::bin_proto::Untagged)?
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
    } else if attribs.flexible_array_member {
        quote!(
            {
                ::bin_proto::BitEncode::encode(#field_ref, __io_writer, __byte_order, __ctx, ::bin_proto::Untagged)?
            }
        )
    } else {
        quote!(
            {
                ::bin_proto::BitEncode::encode(#field_ref, __io_writer, __byte_order, __ctx, ())?
            }
        )
    }
}

fn encode_named_fields(fields_named: &syn::FieldsNamed, self_prefix: bool) -> TokenStream {
    let field_encoders: Vec<_> = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            encode(
                field,
                &if self_prefix {
                    quote!(&self. #field_name)
                } else {
                    quote!(#field_name)
                },
            )
        })
        .collect();

    quote!( #( #field_encoders );* )
}

fn decode_unnamed_fields(fields_unnamed: &syn::FieldsUnnamed) -> TokenStream {
    let field_initializers: Vec<_> = fields_unnamed
        .unnamed
        .iter()
        .map(|field| {
            let field_ty = &field.ty;
            let decode = decode(field);

            quote!(
                {
                    let res: #field_ty = #decode?;
                    res
                }
            )
        })
        .collect();

    quote!( ( #( #field_initializers ),* ) )
}

fn encode_unnamed_fields(fields_unnamed: &syn::FieldsUnnamed, self_prefix: bool) -> TokenStream {
    let field_encoders: Vec<_> = fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(field_index, field)| {
            let field_index = syn::Index::from(field_index);
            encode(
                field,
                &if self_prefix {
                    quote!(&self. #field_index)
                } else {
                    format!("field_{}", field_index.index)
                        .parse()
                        .unwrap_or_else(|e| Error::from(e).into_compile_error())
                },
            )
        })
        .collect();

    quote!( #( #field_encoders );* )
}
