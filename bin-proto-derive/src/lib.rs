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
mod plan;

use attr::{AttrKind, Attrs};
use codegen::trait_impl::{impl_trait_for, TraitImplType};
use proc_macro2::TokenStream;
use syn::{parse_macro_input, spanned::Spanned};

use crate::codegen::enums::{read_discriminant, variant_discriminant, write_discriminant};

#[derive(Clone, Copy)]
enum Operation {
    Read,
    Write,
}

#[proc_macro_derive(ProtocolRead, attributes(protocol))]
pub fn protocol_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    impl_protocol(&ast, Operation::Read).into()
}

#[proc_macro_derive(ProtocolWrite, attributes(protocol))]
pub fn protocol_write(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    impl_protocol(&ast, Operation::Write).into()
}

fn impl_protocol(ast: &syn::DeriveInput, protocol_type: Operation) -> TokenStream {
    match ast.data {
        syn::Data::Struct(ref s) => impl_for_struct(ast, s, protocol_type),
        syn::Data::Enum(ref e) => impl_for_enum(ast, e, protocol_type),
        syn::Data::Union(..) => unimplemented!("Protocol is unimplemented on Unions"),
    }
}

fn impl_for_struct(
    ast: &syn::DeriveInput,
    strukt: &syn::DataStruct,
    protocol_type: Operation,
) -> TokenStream {
    let attribs = match Attrs::parse(ast.attrs.as_slice(), Some(AttrKind::Struct), ast.span()) {
        Ok(attribs) => attribs,
        Err(e) => return e.to_compile_error(),
    };

    let ctx_ty = attribs.ctx_ty();

    let (impl_body, trait_type) = match protocol_type {
        Operation::Read => {
            let (reads, initializers) = codegen::reads(&strukt.fields);
            (
                quote!(
                    #[allow(unused_variables)]
                    fn read(__io_reader: &mut dyn ::bin_proto::BitRead,
                            __byte_order: ::bin_proto::ByteOrder,
                            __ctx: &mut #ctx_ty)
                            -> ::bin_proto::Result<Self> {
                        #reads
                        Ok(Self #initializers)
                    }
                ),
                TraitImplType::ProtocolRead,
            )
        }
        Operation::Write => {
            let writes = codegen::writes(&strukt.fields, true);
            (
                quote!(
                    #[allow(unused_variables)]
                    fn write(&self, __io_writer: &mut dyn ::bin_proto::BitWrite,
                             __byte_order: ::bin_proto::ByteOrder,
                             __ctx: &mut #ctx_ty)
                             -> ::bin_proto::Result<()> {
                        #writes
                        Ok(())
                    }
                ),
                TraitImplType::ProtocolWrite,
            )
        }
    };

    impl_trait_for(ast, &impl_body, &trait_type)
}

fn impl_for_enum(
    ast: &syn::DeriveInput,
    e: &syn::DataEnum,
    protocol_type: Operation,
) -> TokenStream {
    let plan = match plan::Enum::try_new(ast, e) {
        Ok(plan) => plan,
        Err(e) => return e.to_compile_error(),
    };
    let attribs = match Attrs::parse(ast.attrs.as_slice(), Some(AttrKind::Enum), ast.span()) {
        Ok(attribs) => attribs,
        Err(e) => return e.to_compile_error(),
    };
    let discriminant_ty = &plan.discriminant_ty;
    let ctx_ty = attribs.ctx_ty();

    match protocol_type {
        Operation::Read => {
            let read_variant = codegen::enums::read_variant_fields(&plan);
            let impl_body = quote!(
                #[allow(unused_variables)]
                fn read(__io_reader: &mut dyn ::bin_proto::BitRead,
                        __byte_order: ::bin_proto::ByteOrder,
                        __ctx: &mut #ctx_ty,
                        __tag: __Tag)
                        -> ::bin_proto::Result<Self> {
                    Ok(#read_variant)
                }
            );
            let externally_tagged_read_impl = impl_trait_for(
                ast,
                &impl_body,
                &TraitImplType::TaggedRead(discriminant_ty.clone()),
            );

            let read_discriminant = read_discriminant(&attribs);
            let impl_body = quote!(
                #[allow(unused_variables)]
                fn read(__io_reader: &mut dyn ::bin_proto::BitRead,
                        __byte_order: ::bin_proto::ByteOrder,
                        __ctx: &mut #ctx_ty)
                        -> ::bin_proto::Result<Self> {
                    let __tag: #discriminant_ty = #read_discriminant?;
                    <Self as ::bin_proto::TaggedRead<_, _>>::read(__io_reader, __byte_order, __ctx, __tag)
                }
            );
            let protocol_read_impl = impl_trait_for(ast, &impl_body, &TraitImplType::ProtocolRead);

            quote!(
                #externally_tagged_read_impl
                #protocol_read_impl
            )
        }
        Operation::Write => {
            let write_variant = codegen::enums::write_variant_fields(&plan);
            let impl_body = quote!(
                #[allow(unused_variables)]
                fn write(&self,
                         __io_writer: &mut dyn ::bin_proto::BitWrite,
                         __byte_order: ::bin_proto::ByteOrder,
                         __ctx: &mut #ctx_ty)
                         -> ::bin_proto::Result<()> {
                    #write_variant
                    Ok(())
                }
            );
            let externally_tagged_write_impl =
                impl_trait_for(ast, &impl_body, &TraitImplType::UntaggedWrite);

            let variant_discriminant = variant_discriminant(&plan, &attribs);
            let impl_body = quote!(
                type Discriminant = #discriminant_ty;

                #[allow(unused_variables)]
                fn discriminant(&self) -> Self::Discriminant {
                    #variant_discriminant
                }
            );
            let discriminable_impl = impl_trait_for(ast, &impl_body, &TraitImplType::Discriminable);

            let write_discriminant = write_discriminant(&attribs);
            let impl_body = quote!(
                #[allow(unused_variables)]
                fn write(&self,
                         __io_writer: &mut dyn ::bin_proto::BitWrite,
                         __byte_order: ::bin_proto::ByteOrder,
                         __ctx: &mut #ctx_ty)
                         -> ::bin_proto::Result<()> {
                    #write_discriminant
                    <Self as ::bin_proto::UntaggedWrite<_>>::write(self, __io_writer, __byte_order, __ctx)
                }
            );
            let protocol_write_impl =
                impl_trait_for(ast, &impl_body, &TraitImplType::ProtocolWrite);

            quote!(
                #externally_tagged_write_impl
                #discriminable_impl
                #protocol_write_impl
            )
        }
    }
}
