#![recursion_limit = "128"]

extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

mod attr;
mod codegen;
mod plan;

use attr::Attrs;
use proc_macro::TokenStream;

#[proc_macro_derive(Protocol, attributes(protocol))]
pub fn protocol(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse(input).expect("Failed to parse input");

    // Build the impl
    let gen = impl_parcel(&ast);

    // Return the generated impl
    gen.to_string()
        .parse()
        .expect("Could not parse generated parcel impl")
}

// The `Protocol` trait is used for data that can be sent/received.
fn impl_parcel(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match ast.data {
        syn::Data::Struct(ref s) => impl_parcel_for_struct(ast, s),
        syn::Data::Enum(ref e) => {
            let plan = plan::Enum::new(ast, e);

            impl_parcel_for_enum(&plan, ast)
        }
        syn::Data::Union(..) => unimplemented!(),
    }
}

/// Builds generics for a new impl.
///
/// Returns `(generics, where_predicates)`
fn build_generics(
    ast: &syn::DeriveInput,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    use quote::ToTokens;

    let mut where_predicates = Vec::new();
    let mut generics = Vec::new();

    generics.extend(ast.generics.type_params().map(|t| {
        let (ident, bounds) = (&t.ident, &t.bounds);
        where_predicates.push(quote!(#ident : #bounds));
        quote!(#ident)
    }));

    generics.extend(ast.generics.lifetimes().enumerate().map(|(i, _)| {
        let letter = (b'a' + i as u8) as char;
        quote!(#letter)
    }));

    if let Some(where_clause) = ast.generics.where_clause.clone() {
        where_predicates.push(where_clause.predicates.into_token_stream());
    }

    assert!(
        ast.generics.const_params().next().is_none(),
        "constant parameters are not supported yet"
    );

    (generics, where_predicates)
}

fn impl_parcel_for_struct(
    ast: &syn::DeriveInput,
    strukt: &syn::DataStruct,
) -> proc_macro2::TokenStream {
    let (reads, initializers) = codegen::reads(&strukt.fields);
    let writes = codegen::writes(&strukt.fields, true);

    impl_trait_for(
        ast,
        quote!(bin_proto::Protocol),
        quote!(
            #[allow(unused_variables)]
            fn read(__io_reader: &mut dyn bin_proto::BitRead,
                          __byte_order: bin_proto::ByteOrder,
                           __ctx: &mut dyn core::any::Any)
                -> bin_proto::Result<Self> {
                #reads
                Ok(Self #initializers)
            }

            #[allow(unused_variables)]
            fn write(&self, __io_writer: &mut dyn bin_proto::BitWrite,
                           __byte_order: bin_proto::ByteOrder,
                           __ctx: &mut dyn core::any::Any)
                -> bin_proto::Result<()> {
                #writes
                Ok(())
            }
        ),
    )
}

/// Generates a `Protocol` trait implementation for an enum.
fn impl_parcel_for_enum(plan: &plan::Enum, ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let discriminant_ty = plan.discriminant_ty.clone();

    let (read_variant, write_variant) = if let Some(field_width) =
        Attrs::from(ast.attrs.as_slice()).bits
    {
        (
            codegen::enums::read_variant(
                plan,
                quote!(bin_proto::BitField::read(
                    __io_reader,
                    __byte_order,
                    __ctx,
                    #field_width,
                )?),
            ),
            codegen::enums::write_variant(plan, &|variant| {
                let discriminant_expr = variant.discriminant_value.clone();
                let error_message = format!(
                    "Discriminant for variant '{}' does not fit in bitfield with width {}.",
                    variant.ident, field_width
                );

                quote!(
                    const _: () = assert!(#discriminant_expr < (1 as #discriminant_ty) << #field_width, #error_message);
                    <#discriminant_ty as bin_proto::BitField>::write(&{#discriminant_expr}, __io_writer, __byte_order, __ctx, #field_width)?;
                )
            }),
        )
    } else {
        (
            codegen::enums::read_variant(
                plan,
                quote!(bin_proto::Protocol::read(__io_reader, __byte_order, __ctx)?),
            ),
            codegen::enums::write_variant(plan, &|variant| {
                let discriminant_expr = variant.discriminant_value.clone();
                quote!( <#discriminant_ty as bin_proto::Protocol>::write(&{#discriminant_expr}, __io_writer, __byte_order, __ctx)?; )
            }),
        )
    };

    impl_trait_for(
        ast,
        quote!(bin_proto::Protocol),
        quote!(
            #[allow(unused_variables)]
            fn read(__io_reader: &mut dyn bin_proto::BitRead,
                          __byte_order: bin_proto::ByteOrder,
                           __ctx: &mut dyn core::any::Any)
                -> bin_proto::Result<Self> {

                Ok(#read_variant)
            }

            #[allow(unused_variables)]
            fn write(&self, __io_writer: &mut dyn bin_proto::BitWrite,
                           __byte_order: bin_proto::ByteOrder,
                           __ctx: &mut dyn core::any::Any)
                -> bin_proto::Result<()> {
                #write_variant
                Ok(())
            }
        ),
    )
}

fn impl_trait_for(
    ast: &syn::DeriveInput,
    trait_name: proc_macro2::TokenStream,
    impl_body: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let item_name = &ast.ident;

    let (generics, where_predicates) = build_generics(ast);
    let (generics, where_predicates) = (&generics, where_predicates);

    quote!(
        impl < #(#generics),* > #trait_name for #item_name < #(#generics),* >
            where #(#where_predicates),* {
            #impl_body
        }
    )
}
