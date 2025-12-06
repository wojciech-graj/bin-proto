#![deny(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    unsafe_code
)]
#![allow(clippy::module_name_repetitions, clippy::option_if_let_else)]

#[macro_use]
extern crate quote;

mod attr;
mod codegen;
mod enums;

use attr::{AttrKind, Attrs};
use codegen::{
    decode_pad, encode_pad,
    trait_impl::{impl_trait_for, TraitImplType},
};
use proc_macro2::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Error, Result};

use crate::codegen::enums::{decode_discriminant, encode_discriminant, variant_discriminant};

#[derive(Clone, Copy)]
enum Operation {
    Decode,
    Encode,
}

#[proc_macro_derive(BitDecode, attributes(codec))]
pub fn decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    match impl_codec(&ast, Operation::Decode) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

#[proc_macro_derive(BitEncode, attributes(codec))]
pub fn encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    match impl_codec(&ast, Operation::Encode) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

fn impl_codec(ast: &syn::DeriveInput, codec_type: Operation) -> Result<TokenStream> {
    match ast.data {
        syn::Data::Struct(ref s) => impl_for_struct(ast, s, codec_type),
        syn::Data::Enum(ref e) => impl_for_enum(ast, e, codec_type),
        syn::Data::Union(..) => Err(Error::new(
            ast.span(),
            "bin-proto traits are not derivable on unions",
        )),
    }
}

fn impl_for_struct(
    ast: &syn::DeriveInput,
    strukt: &syn::DataStruct,
    codec_type: Operation,
) -> Result<TokenStream> {
    let attrs = Attrs::parse(ast.attrs.as_slice(), Some(AttrKind::Struct), ast.span())?;

    let ctx_ty = attrs.ctx_ty();

    let (impl_body, trait_type) = match codec_type {
        Operation::Decode => {
            let (decodes, initializers) = codegen::decodes(&strukt.fields)?;
            let pad_before = attrs.pad_before.as_ref().map(decode_pad);
            let pad_after = attrs.pad_after.as_ref().map(decode_pad);
            let magic = attrs.decode_magic();

            (
                quote!(
                    fn decode<__R, __E>(
                        __io_reader: &mut __R,
                        __ctx: &mut #ctx_ty,
                        __tag: (),
                    ) -> ::bin_proto::Result<Self>
                    where
                        __R: ::bin_proto::BitRead,
                        __E: ::bin_proto::Endianness,
                    {
                        #pad_before
                        #magic
                        #decodes
                        #pad_after
                        ::core::result::Result::Ok(Self #initializers)
                    }
                ),
                TraitImplType::Decode,
            )
        }
        Operation::Encode => {
            let encodes = codegen::encodes(&strukt.fields, true)?;
            let pad_before = attrs.pad_before.as_ref().map(encode_pad);
            let pad_after = attrs.pad_after.as_ref().map(encode_pad);
            let magic = attrs.encode_magic();

            (
                quote!(
                    fn encode<__W, __E>(
                        &self,
                        __io_writer: &mut __W,
                        __ctx: &mut #ctx_ty,
                        (): (),
                    ) -> ::bin_proto::Result<()>
                    where
                        __W: ::bin_proto::BitWrite,
                        __E: ::bin_proto::Endianness,
                    {
                        #pad_before
                        #magic
                        #encodes
                        #pad_after
                        ::core::result::Result::Ok(())
                    }
                ),
                TraitImplType::Encode,
            )
        }
    };

    impl_trait_for(ast, &impl_body, &trait_type)
}

#[allow(clippy::too_many_lines)]
fn impl_for_enum(
    ast: &syn::DeriveInput,
    e: &syn::DataEnum,
    codec_type: Operation,
) -> Result<TokenStream> {
    let plan = enums::Enum::try_new(ast, e)?;
    let attrs = Attrs::parse(ast.attrs.as_slice(), Some(AttrKind::Enum), ast.span())?;
    let discriminant_ty = &plan.discriminant_ty;
    let ctx_ty = attrs.ctx_ty();

    Ok(match codec_type {
        Operation::Decode => {
            let decode_variant = codegen::enums::decode_variant_fields(&plan)?;
            let impl_body = quote!(
                fn decode<__R, __E>(
                    __io_reader: &mut __R,
                    __ctx: &mut #ctx_ty,
                    __tag: ::bin_proto::Tag<__Tag>,
                ) -> ::bin_proto::Result<Self>
                where
                    __R: ::bin_proto::BitRead,
                    __E: ::bin_proto::Endianness,
                {
                    ::core::result::Result::Ok(#decode_variant)
                }
            );
            let tagged_decode_impl = impl_trait_for(
                ast,
                &impl_body,
                &TraitImplType::TaggedDecode(discriminant_ty.clone()),
            )?;

            let decode_discriminant = decode_discriminant(&attrs);
            let impl_body = quote!(
                fn decode<__R, __E>(
                    __io_reader: &mut __R,
                    __ctx: &mut #ctx_ty,
                    __tag: (),
                ) -> ::bin_proto::Result<Self>
                where
                    __R: ::bin_proto::BitRead,
                    __E: ::bin_proto::Endianness,
                {
                    let __tag: #discriminant_ty = #decode_discriminant?;
                    <Self as ::bin_proto::BitDecode<_, ::bin_proto::Tag<#discriminant_ty>>>::decode::<_, __E>(
                        __io_reader,
                        __ctx,
                        ::bin_proto::Tag(__tag)
                    )
                }
            );
            let decode_impl = impl_trait_for(ast, &impl_body, &TraitImplType::Decode)?;

            quote!(
                #tagged_decode_impl
                #decode_impl
            )
        }
        Operation::Encode => {
            let encode_variant = codegen::enums::encode_variant_fields(&plan)?;
            let pad_before = attrs.pad_before.as_ref().map(encode_pad);
            let pad_after = attrs.pad_after.as_ref().map(encode_pad);
            let impl_body = quote!(
                fn encode<__W, __E>(
                    &self,
                    __io_writer: &mut __W,
                    __ctx: &mut #ctx_ty,
                    __tag: ::bin_proto::Untagged,
                ) -> ::bin_proto::Result<()>
                where
                    __W: ::bin_proto::BitWrite,
                    __E: ::bin_proto::Endianness,
                {
                    #pad_before
                    #encode_variant
                    #pad_after
                    ::core::result::Result::Ok(())
                }
            );
            let untagged_encode_impl =
                impl_trait_for(ast, &impl_body, &TraitImplType::UntaggedEncode)?;

            let variant_discriminant = variant_discriminant(&plan)?;
            let impl_body = quote!(
                type Discriminant = #discriminant_ty;

                fn discriminant(&self) -> ::core::option::Option<Self::Discriminant> {
                    #variant_discriminant
                }
            );
            let discriminable_impl =
                impl_trait_for(ast, &impl_body, &TraitImplType::Discriminable)?;

            let encode_discriminant = encode_discriminant(&attrs);
            let impl_body = quote!(
                fn encode<__W, __E>(
                    &self,
                    __io_writer: &mut __W,
                    __ctx: &mut #ctx_ty,
                    (): (),
                ) -> ::bin_proto::Result<()>
                where
                    __W: ::bin_proto::BitWrite,
                    __E: ::bin_proto::Endianness,
                {
                    #pad_before
                    #encode_discriminant
                    let res = <Self as ::bin_proto::BitEncode<_, _>>::encode::<_, __E>(
                        self,
                        __io_writer,
                        __ctx,
                        ::bin_proto::Untagged
                    )?;
                    #pad_after
                    ::core::result::Result::Ok(res)
                }
            );
            let encode_impl = impl_trait_for(ast, &impl_body, &TraitImplType::Encode)?;

            quote!(
                #untagged_encode_impl
                #discriminable_impl
                #encode_impl
            )
        }
    })
}
