#[macro_use]
extern crate quote;

mod attr;
mod codegen;
mod plan;

use attr::Attrs;
use proc_macro2::{Span, TokenStream};
use syn::punctuated::Punctuated;

enum ProtocolType {
    Read,
    Write,
}

#[proc_macro_derive(ProtocolRead, attributes(protocol))]
pub fn protocol_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("Failed to parse input");
    impl_protocol(&ast, ProtocolType::Read).into()
}

#[proc_macro_derive(ProtocolWrite, attributes(protocol))]
pub fn protocol_write(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("Failed to parse input");
    impl_protocol(&ast, ProtocolType::Write).into()
}

fn impl_protocol(ast: &syn::DeriveInput, protocol_type: ProtocolType) -> TokenStream {
    match ast.data {
        syn::Data::Struct(ref s) => impl_for_struct(ast, s, protocol_type),
        syn::Data::Enum(ref e) => impl_for_enum(ast, e, protocol_type),
        syn::Data::Union(..) => unimplemented!("Protocol is unimplemented on Unions"),
    }
}

fn impl_for_struct(
    ast: &syn::DeriveInput,
    strukt: &syn::DataStruct,
    protocol_type: ProtocolType,
) -> TokenStream {
    let attribs = match Attrs::try_from(ast.attrs.as_slice()) {
        Ok(attribs) => attribs,
        Err(e) => return e.to_compile_error(),
    };

    let ctx_ty = attribs.ctx_ty();

    let impl_body = match protocol_type {
        ProtocolType::Read => {
            let (reads, initializers) = codegen::reads(&strukt.fields, &attribs);
            quote!(
                #[allow(unused_variables)]
                fn read(__io_reader: &mut dyn ::bin_proto::BitRead,
                        __byte_order: ::bin_proto::ByteOrder,
                        __ctx: &mut #ctx_ty)
                        -> ::bin_proto::Result<Self> {
                    #reads
                    Ok(Self #initializers)
                }
            )
        }
        ProtocolType::Write => {
            let writes = codegen::writes(&strukt.fields, true);
            quote!(
                #[allow(unused_variables)]
                fn write(&self, __io_writer: &mut dyn ::bin_proto::BitWrite,
                         __byte_order: ::bin_proto::ByteOrder,
                         __ctx: &mut #ctx_ty)
                         -> ::bin_proto::Result<()> {
                    #writes
                    Ok(())
                }
            )
        }
    };

    impl_protocol_for(ast, impl_body, protocol_type)
}

fn impl_for_enum(
    ast: &syn::DeriveInput,
    e: &syn::DataEnum,
    protocol_type: ProtocolType,
) -> TokenStream {
    let plan = match plan::Enum::try_new(ast, e) {
        Ok(plan) => plan,
        Err(e) => return e.to_compile_error(),
    };
    let attribs = match Attrs::try_from(ast.attrs.as_slice()) {
        Ok(attribs) => attribs,
        Err(e) => return e.to_compile_error(),
    };
    let discriminant_ty = &plan.discriminant_ty;
    let ctx_ty = attribs.ctx_ty();

    let impl_body = match protocol_type {
        ProtocolType::Read => {
            let read_variant = if let Some(field_width) = attribs.bits {
                codegen::enums::read_variant(
                    &plan,
                    quote!(::bin_proto::BitFieldRead::<#ctx_ty>::read(
                    __io_reader,
                    __byte_order,
                    __ctx,
                    #field_width,
                )?),
                    &attribs,
                )
            } else {
                codegen::enums::read_variant(
                    &plan,
                    quote!(::bin_proto::ProtocolRead::<#ctx_ty>::read(
                    __io_reader,
                    __byte_order,
                    __ctx
                )?),
                    &attribs,
                )
            };
            quote!(
                #[allow(unused_variables)]
                fn read(__io_reader: &mut dyn ::bin_proto::BitRead,
                        __byte_order: ::bin_proto::ByteOrder,
                        __ctx: &mut #ctx_ty)
                        -> ::bin_proto::Result<Self> {

                    Ok(#read_variant)
                }
            )
        }
        ProtocolType::Write => {
            let write_variant = if let Some(field_width) = attribs.bits {
                codegen::enums::write_variant(&plan, &|variant| {
                    let discriminant_expr = &variant.discriminant_value;
                    let error_message = format!(
                        "Discriminant for variant '{}' does not fit in bitfield with width {}.",
                        variant.ident, field_width
                    );

                    quote!(
                        const _: () = ::std::assert!(#discriminant_expr < (1 as #discriminant_ty) << #field_width, #error_message);
                        <#discriminant_ty as ::bin_proto::BitFieldWrite<#ctx_ty>>::write(&{#discriminant_expr}, __io_writer, __byte_order, __ctx, #field_width)?;
                    )
                })
            } else {
                codegen::enums::write_variant(&plan, &|variant| {
                    let discriminant_expr = &variant.discriminant_value;
                    quote!( <#discriminant_ty as ::bin_proto::ProtocolWrite<#ctx_ty>>::write(&{#discriminant_expr}, __io_writer, __byte_order, __ctx)?; )
                })
            };
            quote!(
                #[allow(unused_variables)]
                fn write(&self,
                         __io_writer: &mut dyn ::bin_proto::BitWrite,
                         __byte_order: ::bin_proto::ByteOrder,
                         __ctx: &mut #ctx_ty)
                         -> ::bin_proto::Result<()> {
                    #write_variant
                    Ok(())
                }
            )
        }
    };

    impl_protocol_for(ast, impl_body, protocol_type)
}

fn impl_protocol_for(
    ast: &syn::DeriveInput,
    impl_body: TokenStream,
    protocol_type: ProtocolType,
) -> TokenStream {
    let name = &ast.ident;
    let attribs = match Attrs::try_from(ast.attrs.as_slice()) {
        Ok(attribs) => attribs,
        Err(e) => return e.to_compile_error(),
    };

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let trait_name = match protocol_type {
        ProtocolType::Read => quote!(ProtocolRead),
        ProtocolType::Write => quote!(ProtocolWrite),
    };

    if let Some(ctx) = attribs.ctx {
        quote!(
            #[automatically_derived]
            impl #impl_generics ::bin_proto::#trait_name<#ctx> for #name #ty_generics #where_clause {
                #impl_body
            }
        )
    } else {
        let mut generics = ast.generics.clone();
        generics
            .params
            .push(syn::GenericParam::Type(syn::TypeParam {
                attrs: Vec::new(),
                ident: syn::Ident::new("__Ctx", Span::call_site()),
                colon_token: None,
                bounds: attribs.ctx_bounds.unwrap_or(Punctuated::new()),
                eq_token: None,
                default: None,
            }));
        let (impl_generics, _, where_clause) = generics.split_for_impl();
        quote!(
            #[automatically_derived]
            impl #impl_generics ::bin_proto::#trait_name<__Ctx> for #name #ty_generics #where_clause {
                #impl_body
            }
        )
    }
}
