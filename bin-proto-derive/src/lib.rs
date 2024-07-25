#[macro_use]
extern crate quote;

mod attr;
mod codegen;
mod plan;

use attr::Attrs;
use proc_macro2::{Span, TokenStream};
use syn::{parse_quote, punctuated::Punctuated, Token};

use crate::codegen::enums::bind_fields_pattern;

enum ProtocolType {
    Read,
    Write,
}

enum ProtocolImplType {
    ProtocolRead,
    ProtocolWrite,
    ExternallyTaggedRead(syn::Type),
    ExternallyTaggedWrite,
    Discriminable,
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

    impl_protocol_for(
        ast,
        impl_body,
        match protocol_type {
            ProtocolType::Read => ProtocolImplType::ProtocolRead,
            ProtocolType::Write => ProtocolImplType::ProtocolWrite,
        },
    )
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

    match protocol_type {
        ProtocolType::Read => {
            let read_variant = codegen::enums::read_variant(&plan, &attribs);
            let discriminant_ty = plan.discriminant_ty;
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
            let externally_tagged_read_impl = impl_protocol_for(
                ast,
                impl_body,
                ProtocolImplType::ExternallyTaggedRead(discriminant_ty.clone()),
            );

            let read_discriminant = if let Some(bits) = attribs.bits {
                quote!(::bin_proto::BitFieldRead::read(__io_reader, __byte_order, __ctx, #bits))
            } else {
                quote!(::bin_proto::ProtocolRead::read(
                    __io_reader,
                    __byte_order,
                    __ctx
                ))
            };

            let impl_body = quote!(
                #[allow(unused_variables)]
                fn read(__io_reader: &mut dyn ::bin_proto::BitRead,
                        __byte_order: ::bin_proto::ByteOrder,
                        __ctx: &mut #ctx_ty)
                        -> ::bin_proto::Result<Self> {
                    let __tag: #discriminant_ty = #read_discriminant?;
                    <Self as ::bin_proto::ExternallyTaggedRead<_, _>>::read(__io_reader, __byte_order, __ctx, __tag)
                }
            );

            let protocol_read_impl =
                impl_protocol_for(ast, impl_body, ProtocolImplType::ProtocolRead);

            quote!(
                #externally_tagged_read_impl
                #protocol_read_impl
            )
        }
        ProtocolType::Write => {
            let write_variant = codegen::enums::write_variant(&plan);
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
                impl_protocol_for(ast, impl_body, ProtocolImplType::ExternallyTaggedWrite);

            let variant_match_branches: Vec<_> = plan
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;
                    let fields_pattern = bind_fields_pattern(variant_name, &variant.fields);
                    let discriminant_expr = &variant.discriminant_value;
                    let write_variant = if let Some(field_width) = attribs.bits {
                        let error_message = format!(
                            "Discriminant for variant '{}' does not fit in bitfield with width {}.",
                            variant.ident, field_width
                        );
                        quote!(
                            const _: () = ::std::assert!(#discriminant_expr < (1 as #discriminant_ty) << #field_width, #error_message);
                            #discriminant_expr
                        )
                    } else {
                        quote!(#discriminant_expr)
                    };

                    quote!(Self :: #fields_pattern => {
                        #write_variant
                    })
                })
                .collect();

            let impl_body = quote!(
                type Discriminant = #discriminant_ty;

                #[allow(unused_variables)]
                fn discriminant(&self) -> Self::Discriminant {
                    match *self {
                        #(#variant_match_branches,)*
                    }
                }
            );
            let discriminable_impl =
                impl_protocol_for(ast, impl_body, ProtocolImplType::Discriminable);

            let write_tag = if let Some(bits) = attribs.bits {
                quote!(::bin_proto::BitFieldWrite::write(&__tag, __io_writer, __byte_order, __ctx, #bits))
            } else {
                quote!(::bin_proto::ProtocolWrite::write(
                    &__tag,
                    __io_writer,
                    __byte_order,
                    __ctx
                ))
            };

            let impl_body = quote!(
                #[allow(unused_variables)]
                fn write(&self,
                         __io_writer: &mut dyn ::bin_proto::BitWrite,
                         __byte_order: ::bin_proto::ByteOrder,
                         __ctx: &mut #ctx_ty)
                         -> ::bin_proto::Result<()> {
                    {
                        let __tag = <Self as ::bin_proto::Discriminable>::discriminant(self);
                        #write_tag?;
                    }
                    <Self as ::bin_proto::ExternallyTaggedWrite<_>>::write(self, __io_writer, __byte_order, __ctx)
                }
            );
            let protocol_write_impl =
                impl_protocol_for(ast, impl_body, ProtocolImplType::ProtocolWrite);

            quote!(
                #externally_tagged_write_impl
                #discriminable_impl
                #protocol_write_impl
            )
        }
    }
}

fn impl_protocol_for(
    ast: &syn::DeriveInput,
    impl_body: TokenStream,
    protocol_type: ProtocolImplType,
) -> TokenStream {
    let name = &ast.ident;
    let attribs = match Attrs::try_from(ast.attrs.as_slice()) {
        Ok(attribs) => attribs,
        Err(e) => return e.to_compile_error(),
    };

    let generics = &ast.generics;
    let (_, ty_generics, _) = generics.split_for_impl();
    let mut generics = ast.generics.clone();

    let mut trait_generics: Punctuated<TokenStream, Token![,]> = Punctuated::new();

    let trait_name = match &protocol_type {
        ProtocolImplType::ProtocolRead => quote!(ProtocolRead),
        ProtocolImplType::ProtocolWrite => quote!(ProtocolWrite),
        ProtocolImplType::ExternallyTaggedRead(discriminant) => {
            let ident = syn::Ident::new("__Tag", Span::call_site());
            let mut bounds = Punctuated::new();
            bounds.push(parse_quote!(::std::convert::TryInto<#discriminant>));
            generics
                .params
                .push(syn::GenericParam::Type(syn::TypeParam {
                    attrs: Vec::new(),
                    ident: ident.clone(),
                    colon_token: None,
                    bounds,
                    eq_token: None,
                    default: None,
                }));
            trait_generics.push(quote!(#ident));
            quote!(ExternallyTaggedRead)
        }
        ProtocolImplType::ExternallyTaggedWrite => quote!(ExternallyTaggedWrite),
        ProtocolImplType::Discriminable => quote!(Discriminable),
    };

    if matches!(
        protocol_type,
        ProtocolImplType::ProtocolRead
            | ProtocolImplType::ProtocolWrite
            | ProtocolImplType::ExternallyTaggedRead(_)
            | ProtocolImplType::ExternallyTaggedWrite
    ) {
        trait_generics.push(if let Some(ctx) = attribs.ctx {
            quote!(#ctx)
        } else {
            let ident = syn::Ident::new("__Ctx", Span::call_site());
            generics
                .params
                .push(syn::GenericParam::Type(syn::TypeParam {
                    attrs: Vec::new(),
                    ident: ident.clone(),
                    colon_token: None,
                    bounds: attribs.ctx_bounds.unwrap_or(Punctuated::new()),
                    eq_token: None,
                    default: None,
                }));
            quote!(#ident)
        });
    }

    let (impl_generics, _, where_clause) = generics.split_for_impl();
    quote!(
        #[automatically_derived]
        impl #impl_generics ::bin_proto::#trait_name<#trait_generics> for #name #ty_generics #where_clause {
            #impl_body
        }
    )
}
