#[macro_use]
extern crate quote;

mod attr;
mod codegen;
mod plan;

use attr::Attrs;
use proc_macro2::{Span, TokenStream};
use syn::punctuated::Punctuated;

#[proc_macro_derive(Protocol, attributes(protocol))]
pub fn protocol(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("Failed to parse input");
    impl_protocol(&ast).into()
}

fn impl_protocol(ast: &syn::DeriveInput) -> TokenStream {
    match ast.data {
        syn::Data::Struct(ref s) => impl_parcel_for_struct(ast, s),
        syn::Data::Enum(ref e) => impl_parcel_for_enum(ast, e),
        syn::Data::Union(..) => unimplemented!("Protocol is unimplemented on Unions"),
    }
}

fn impl_parcel_for_struct(ast: &syn::DeriveInput, strukt: &syn::DataStruct) -> TokenStream {
    let attribs = match Attrs::try_from(ast.attrs.as_slice()) {
        Ok(attribs) => attribs,
        Err(e) => return e.to_compile_error(),
    };
    let (reads, initializers) = codegen::reads(&strukt.fields, &attribs);
    let writes = codegen::writes(&strukt.fields, true);

    let ctx_ty = attribs.ctx_ty();

    impl_protocol_for(
        ast,
        quote!(
            #[allow(unused_variables)]
            fn read(__io_reader: &mut dyn bin_proto::BitRead,
                    __byte_order: bin_proto::ByteOrder,
                    __ctx: &mut #ctx_ty)
                    -> bin_proto::Result<Self> {
                #reads
                Ok(Self #initializers)
            }

            #[allow(unused_variables)]
            fn write(&self, __io_writer: &mut dyn bin_proto::BitWrite,
                     __byte_order: bin_proto::ByteOrder,
                     __ctx: &mut #ctx_ty)
                     -> bin_proto::Result<()> {
                #writes
                Ok(())
            }
        ),
    )
}

fn impl_parcel_for_enum(ast: &syn::DeriveInput, e: &syn::DataEnum) -> TokenStream {
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

    let (read_variant, write_variant) = if let Some(field_width) = attribs.bits {
        (
            codegen::enums::read_variant(
                &plan,
                quote!(::bin_proto::BitField::<#ctx_ty>::read(
                    __io_reader,
                    __byte_order,
                    __ctx,
                    #field_width,
                )?),
                &attribs,
            ),
            codegen::enums::write_variant(&plan, &|variant| {
                let discriminant_expr = &variant.discriminant_value;
                let error_message = format!(
                    "Discriminant for variant '{}' does not fit in bitfield with width {}.",
                    variant.ident, field_width
                );

                quote!(
                    const _: () = ::std::assert!(#discriminant_expr < (1 as #discriminant_ty) << #field_width, #error_message);
                    <#discriminant_ty as ::bin_proto::BitField<#ctx_ty>>::write(&{#discriminant_expr}, __io_writer, __byte_order, __ctx, #field_width)?;
                )
            }),
        )
    } else {
        (
            codegen::enums::read_variant(
                &plan,
                quote!(::bin_proto::Protocol::<#ctx_ty>::read(
                    __io_reader,
                    __byte_order,
                    __ctx
                )?),
                &attribs,
            ),
            codegen::enums::write_variant(&plan, &|variant| {
                let discriminant_expr = &variant.discriminant_value;
                quote!( <#discriminant_ty as ::bin_proto::Protocol<#ctx_ty>>::write(&{#discriminant_expr}, __io_writer, __byte_order, __ctx)?; )
            }),
        )
    };

    impl_protocol_for(
        ast,
        quote!(
            #[allow(unused_variables)]
            fn read(__io_reader: &mut dyn ::bin_proto::BitRead,
                    __byte_order: ::bin_proto::ByteOrder,
                    __ctx: &mut #ctx_ty)
                    -> ::bin_proto::Result<Self> {

                Ok(#read_variant)
            }

            #[allow(unused_variables)]
            fn write(&self,
                     __io_writer: &mut dyn ::bin_proto::BitWrite,
                     __byte_order: ::bin_proto::ByteOrder,
                     __ctx: &mut #ctx_ty)
                     -> ::bin_proto::Result<()> {
                #write_variant
                Ok(())
            }
        ),
    )
}

fn impl_protocol_for(ast: &syn::DeriveInput, impl_body: TokenStream) -> TokenStream {
    let name = &ast.ident;
    let attribs = match Attrs::try_from(ast.attrs.as_slice()) {
        Ok(attribs) => attribs,
        Err(e) => return e.to_compile_error(),
    };

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    if let Some(ctx) = attribs.ctx {
        quote!(
            #[automatically_derived]
            impl #impl_generics ::bin_proto::Protocol<#ctx> for #name #ty_generics #where_clause {
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
            impl #impl_generics ::bin_proto::Protocol<__Ctx> for #name #ty_generics #where_clause {
                #impl_body
            }
        )
    }
}
