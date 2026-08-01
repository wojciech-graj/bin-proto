pub mod bounds;
pub mod enums;
pub mod trait_impl;

use crate::attr::{AttrKind, Attrs, Tag};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Error, Result};

pub fn decodes(parent_attrs: &Attrs, fields: &syn::Fields) -> Result<(TokenStream, TokenStream)> {
    match fields {
        syn::Fields::Named(fields) => decode_named_fields(parent_attrs, fields),
        syn::Fields::Unnamed(fields) => decode_unnamed_fields(parent_attrs, fields),
        syn::Fields::Unit => Ok((TokenStream::new(), TokenStream::new())),
    }
}

pub fn encodes(parent_attrs: &Attrs, fields: &syn::Fields) -> Result<TokenStream> {
    match fields {
        syn::Fields::Named(fields) => encode_named_fields(parent_attrs, fields),
        syn::Fields::Unnamed(fields) => encode_unnamed_fields(parent_attrs, fields),
        syn::Fields::Unit => Ok(TokenStream::new()),
    }
}

fn decode_named_fields(
    parent_attrs: &Attrs,
    fields_named: &syn::FieldsNamed,
) -> Result<(TokenStream, TokenStream)> {
    let fields = fields_named
        .named
        .iter()
        .map(|field| {
            decode(
                parent_attrs,
                field,
                field.ident.as_ref().unwrap_or_else(|| unreachable!()),
            )
        })
        .collect::<Result<Vec<_>>>()?;

    let field_initializers: Vec<_> = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_name_priv = format_ident!("__{}", field_name.to_token_stream().to_string());

            quote!(#field_name: #field_name_priv)
        })
        .collect();

    Ok((
        quote!( #( #fields )* ),
        quote!( { #( #field_initializers ),* } ),
    ))
}

pub fn decode_pad(crate_path: &TokenStream, pad: &syn::Expr) -> TokenStream {
    quote!(#crate_path::BitRead::skip(__io_reader, #pad)?;)
}

fn decode(
    parent_attrs: &Attrs,
    field: &syn::Field,
    field_name: &syn::Ident,
) -> Result<TokenStream> {
    let attrs = Attrs::parse(
        Some(parent_attrs),
        field.attrs.as_slice(),
        Some(AttrKind::Field),
        field.span(),
    )?;

    let field_ty = &field.ty;
    let field_name_priv = format_ident!("__{field_name}");

    if attrs.skip_decode {
        return Ok(quote!(
            let #field_name_priv: #field_ty = ::core::default::Default::default();
            let #field_name = &#field_name_priv;
        ));
    }

    let crate_path = attrs.crate_path();

    let pad_before = attrs
        .pad_before
        .as_ref()
        .map(|pad| decode_pad(&crate_path, pad));
    let pad_after = attrs
        .pad_after
        .as_ref()
        .map(|pad| decode_pad(&crate_path, pad));
    let magic = attrs.decode_magic();
    let assert = attrs.codec_assert();

    let decode = if let Some(Tag::Prepend { typ, bits, .. }) = attrs.tag {
        let tag = if let Some(bits) = bits {
            quote!(#crate_path::Bits::<#bits>)
        } else {
            quote!(())
        };
        quote!({
            let __tag: #typ = #crate_path::BitDecode::<__E, _, _>::decode(__io_reader, __ctx, #tag)?;
            #crate_path::BitDecode::<__E, _, _>::decode(
                __io_reader,
                __ctx,
                #crate_path::Tag(__tag)
            )?
        })
    } else {
        let tag = if let Some(field_width) = attrs.bits {
            quote!(#crate_path::Bits::<#field_width>)
        } else if attrs.untagged {
            quote!(#crate_path::Untagged)
        } else if let Some(Tag::External(tag)) = attrs.tag {
            quote!(#crate_path::Tag(#tag))
        } else {
            quote!(())
        };
        quote!(#crate_path::BitDecode::<__E, _, _>::decode(__io_reader, __ctx, #tag)?)
    };

    Ok(quote!(
        #pad_before
        #magic
        let #field_name_priv: #field_ty = #decode;
        let #field_name = &#field_name_priv;
        #assert
        #pad_after
    ))
}

pub fn encode_pad(crate_path: &TokenStream, pad: &syn::Expr) -> TokenStream {
    quote!(#crate_path::BitWrite::pad(__io_writer, #pad)?;)
}

fn encode(parent: &Attrs, field: &syn::Field, field_name: &TokenStream) -> Result<TokenStream> {
    let attrs = Attrs::parse(
        Some(parent),
        field.attrs.as_slice(),
        Some(AttrKind::Field),
        field.span(),
    )?;

    if attrs.skip_encode {
        return Ok(TokenStream::new());
    }

    let crate_path = attrs.crate_path();

    let pad_before = attrs
        .pad_before
        .as_ref()
        .map(|pad| encode_pad(&crate_path, pad));
    let pad_after = attrs
        .pad_after
        .as_ref()
        .map(|pad| encode_pad(&crate_path, pad));
    let magic = attrs.encode_magic();
    let assert = attrs.codec_assert();

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
            quote!(#crate_path::Bits::<#bits>)
        } else {
            quote!(())
        };
        quote!(
            {
                <#typ as #crate_path::BitEncode::<__E, _, _>>::encode(
                    &{#write_value},
                    __io_writer,
                    __ctx,
                    #tag
                )?;
                #crate_path::BitEncode::<__E, _, _>::encode(
                    #field_ref,
                    __io_writer,
                    __ctx,
                    #crate_path::Untagged
                )?
            }
        )
    } else {
        let tag = if let Some(field_width) = attrs.bits {
            quote!(#crate_path::Bits::<#field_width>)
        } else if matches!(attrs.tag, Some(Tag::External(_))) || attrs.untagged {
            quote!(#crate_path::Untagged)
        } else {
            quote!(())
        };
        quote!(
            {
                #crate_path::BitEncode::<__E, _, _>::encode(#field_ref, __io_writer, __ctx, #tag)?
            }
        )
    };

    Ok(quote!(
        #assert
        #pad_before
        #magic
        #encode;
        #pad_after
    ))
}

fn encode_named_fields(
    parent_attrs: &Attrs,
    fields_named: &syn::FieldsNamed,
) -> Result<TokenStream> {
    let field_encoders = fields_named
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            encode(parent_attrs, field, &quote!(#field_name))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote!( #( #field_encoders )* ))
}

fn decode_unnamed_fields(
    parent_attrs: &Attrs,
    fields_unnamed: &syn::FieldsUnnamed,
) -> Result<(TokenStream, TokenStream)> {
    let fields = fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, field)| decode(parent_attrs, field, &format_ident!("field_{i}")))
        .collect::<Result<Vec<_>>>()?;

    let field_initializers: Vec<_> = (0..fields_unnamed.unnamed.len())
        .map(|i| {
            let idx = syn::Index::from(i).to_token_stream();
            let bind = format_ident!("__field_{i}");
            quote!(#idx: #bind)
        })
        .collect();

    Ok((
        quote!( #( #fields )* ),
        quote!( { #( #field_initializers ),* } ),
    ))
}

fn encode_unnamed_fields(
    parent_attrs: &Attrs,
    fields_unnamed: &syn::FieldsUnnamed,
) -> Result<TokenStream> {
    let field_encoders: Vec<_> = fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(field_index, field)| {
            let field_index = syn::Index::from(field_index);
            encode(
                parent_attrs,
                field,
                &format!("field_{}", field_index.index).parse()?,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote!( #( #field_encoders )* ))
}
