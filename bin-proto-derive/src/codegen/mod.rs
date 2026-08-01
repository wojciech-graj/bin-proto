pub mod bounds;
pub mod enums;
pub mod trait_impl;

use crate::{
    attr::{AttrKind, Attrs, Tag},
    field::{Field, FieldsExt},
    TemporalDirection,
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Error, Result};

pub fn decodes(parent_attrs: &Attrs, fields: &syn::Fields) -> Result<(TokenStream, TokenStream)> {
    let decodes = fields
        .fields()
        .map(|field| field.decode(parent_attrs))
        .collect::<Result<Vec<_>>>()?;

    let initializers = fields
        .fields()
        .map(|field| {
            let private = field.priv_binding();
            let member = field.member;
            quote!(#member: #private)
        })
        .collect::<Vec<_>>();

    Ok((quote!( #( #decodes )* ), quote!( { #( #initializers ),* } )))
}

pub fn encodes(parent_attrs: &Attrs, fields: &syn::Fields) -> Result<TokenStream> {
    let encodes = fields
        .fields()
        .map(|field| field.encode(parent_attrs))
        .collect::<Result<Vec<_>>>()?;

    Ok(quote!( #( #encodes )* ))
}

impl Field<'_> {
    fn priv_binding(&self) -> syn::Ident {
        format_ident!("__{}", self.binding())
    }

    fn decode(&self, parent_attrs: &Attrs) -> Result<TokenStream> {
        let attrs = Attrs::parse(
            Some(parent_attrs),
            self.field.attrs.as_slice(),
            Some(AttrKind::Field),
            self.field.span(),
        )?;

        let field_ty = &self.field.ty;
        let field_name_priv = self.priv_binding();
        let field_name = self.binding();

        if attrs.skip_decode {
            return Ok(quote!(
                let #field_name_priv: #field_ty = ::core::default::Default::default();
                let #field_name = &#field_name_priv;
            ));
        }

        let crate_path = attrs.crate_path();

        let pad_before = attrs.decode_pad(TemporalDirection::Before);
        let pad_after = attrs.decode_pad(TemporalDirection::After);
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

    fn encode(&self, parent: &Attrs) -> Result<TokenStream> {
        let attrs = Attrs::parse(
            Some(parent),
            self.field.attrs.as_slice(),
            Some(AttrKind::Field),
            self.field.span(),
        )?;

        if attrs.skip_encode {
            return Ok(TokenStream::new());
        }

        let crate_path = attrs.crate_path();

        let pad_before = attrs.encode_pad(TemporalDirection::Before);
        let pad_after = attrs.encode_pad(TemporalDirection::After);
        let magic = attrs.encode_magic();
        let assert = attrs.codec_assert();
        let field_name = self.binding();

        let field_ref = if let Some(value) = attrs.write_value {
            let ty = &self.field.ty;
            quote!(&{
                let value: #ty = {#value};
                value
            })
        } else {
            field_name.to_token_stream()
        };

        let encode = if let Some(Tag::Prepend {
            typ,
            write_value,
            bits,
        }) = attrs.tag
        {
            let Some(write_value) = write_value else {
                return Err(Error::new(
                    self.field.span(),
                    "Tag must specify 'write_value'",
                ));
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
}
