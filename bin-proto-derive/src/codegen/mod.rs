pub mod enums;
pub mod trait_impl;

use crate::attr::{AttrKind, Attrs, Tag};
use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Error, Result};

pub fn decodes(fields: &syn::Fields) -> Result<(TokenStream, TokenStream)> {
    match fields {
        syn::Fields::Named(fields) => decode_named_fields(fields),
        syn::Fields::Unnamed(fields) => Ok((TokenStream::new(), decode_unnamed_fields(fields)?)),
        syn::Fields::Unit => Ok((TokenStream::new(), TokenStream::new())),
    }
}

pub fn encodes(fields: &syn::Fields, self_prefix: bool) -> Result<TokenStream> {
    match fields {
        syn::Fields::Named(fields) => encode_named_fields(fields, self_prefix),
        syn::Fields::Unnamed(fields) => encode_unnamed_fields(fields, self_prefix),
        syn::Fields::Unit => Ok(TokenStream::new()),
    }
}

fn decode_named_fields(fields_named: &syn::FieldsNamed) -> Result<(TokenStream, TokenStream)> {
    let fields = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;

            let decode = decode(field)?;

            Ok(quote!(
                let #field_name : #field_ty = #decode;
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    let field_initializers: Vec<_> = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;

            quote!(#field_name)
        })
        .collect();

    Ok((
        quote!( #( #fields )* ),
        quote!( { #( #field_initializers ),* } ),
    ))
}

pub fn decode_pad(pad: &syn::Expr) -> TokenStream {
    quote!(::bin_proto::BitRead::skip(__io_reader, #pad)?;)
}

fn decode(field: &syn::Field) -> Result<TokenStream> {
    let attrs = Attrs::parse(field.attrs.as_slice(), Some(AttrKind::Field), field.span())?;

    if attrs.skip_decode {
        return Ok(quote!(::core::default::Default::default()));
    }

    let pad_before = attrs.pad_before.as_ref().map(decode_pad);
    let pad_after = attrs.pad_after.as_ref().map(decode_pad);
    let magic = attrs.decode_magic();

    let decode = if let Some(Tag::Prepend { typ, bits, .. }) = attrs.tag {
        let tag = if let Some(bits) = bits {
            quote!(::bin_proto::Bits::<#bits>)
        } else {
            quote!(())
        };
        quote!({
            let __tag: #typ = ::bin_proto::BitDecode::decode::<_, __E>(__io_reader, __ctx, #tag)?;
            ::bin_proto::BitDecode::decode::<_, __E>(
                __io_reader,
                __ctx,
                ::bin_proto::Tag(__tag)
            )?
        })
    } else {
        let tag = if let Some(field_width) = attrs.bits {
            quote!(::bin_proto::Bits::<#field_width>)
        } else if attrs.flexible_array_member {
            quote!(::bin_proto::Untagged)
        } else if let Some(Tag::External(tag)) = attrs.tag {
            quote!(::bin_proto::Tag(#tag))
        } else {
            quote!(())
        };
        quote!(::bin_proto::BitDecode::decode::<_, __E>(__io_reader, __ctx, #tag)?)
    };

    Ok(quote!({
        #pad_before
        #magic
        let decoded = #decode;
        #pad_after
        decoded
    }))
}

pub fn encode_pad(pad: &syn::Expr) -> TokenStream {
    quote!(::bin_proto::BitWrite::pad(__io_writer, #pad)?;)
}

fn encode(field: &syn::Field, field_name: &TokenStream) -> Result<TokenStream> {
    let attrs = Attrs::parse(field.attrs.as_slice(), Some(AttrKind::Field), field.span())?;

    if attrs.skip_encode {
        return Ok(TokenStream::new());
    }

    let pad_before = attrs.pad_before.as_ref().map(encode_pad);
    let pad_after = attrs.pad_after.as_ref().map(encode_pad);
    let magic = attrs.encode_magic();

    let field_ref = if let Some(value) = attrs.write_value {
        let ty = &field.ty;
        quote!(&{
            let value: #ty = {#value};
            value
        })
    } else {
        field_name.clone()
    };

    let encode = if let Some(Tag::Prepend {
        typ,
        write_value,
        bits,
    }) = attrs.tag
    {
        let Some(write_value) = write_value else {
            return Err(Error::new(field.span(), "Tag must specify 'write_value'"));
        };
        let tag = if let Some(bits) = bits {
            quote!(::bin_proto::Bits::<#bits>)
        } else {
            quote!(())
        };
        quote!(
            {
                <#typ as ::bin_proto::BitEncode::<_, _>>::encode::<_, __E>(
                    &{#write_value},
                    __io_writer,
                    __ctx,
                    #tag
                )?;
                ::bin_proto::BitEncode::encode::<_, __E>(
                    #field_ref,
                    __io_writer,
                    __ctx,
                    ::bin_proto::Untagged
                )?
            }
        )
    } else {
        let tag = if let Some(field_width) = attrs.bits {
            quote!(::bin_proto::Bits::<#field_width>)
        } else if matches!(attrs.tag, Some(Tag::External(_))) || attrs.flexible_array_member {
            quote!(::bin_proto::Untagged)
        } else {
            quote!(())
        };
        quote!(
            {
                ::bin_proto::BitEncode::encode::<_, __E>(#field_ref, __io_writer, __ctx, #tag)?
            }
        )
    };

    Ok(quote!(
        #pad_before
        #magic
        #encode;
        #pad_after
    ))
}

fn encode_named_fields(fields_named: &syn::FieldsNamed, self_prefix: bool) -> Result<TokenStream> {
    let field_encoders = fields_named
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
        .collect::<Result<Vec<_>>>()?;

    Ok(quote!( #( #field_encoders )* ))
}

fn decode_unnamed_fields(fields_unnamed: &syn::FieldsUnnamed) -> Result<TokenStream> {
    let field_initializers = fields_unnamed
        .unnamed
        .iter()
        .map(|field| {
            let field_ty = &field.ty;
            let decode = decode(field)?;

            Ok(quote!(
                {
                    let res: #field_ty = #decode;
                    res
                }
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote!( ( #( #field_initializers ),* ) ))
}

fn encode_unnamed_fields(
    fields_unnamed: &syn::FieldsUnnamed,
    self_prefix: bool,
) -> Result<TokenStream> {
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
                    format!("field_{}", field_index.index).parse()?
                },
            )
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote!( #( #field_encoders )* ))
}
