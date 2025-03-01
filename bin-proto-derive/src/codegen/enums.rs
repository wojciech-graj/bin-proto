use crate::{attr::Attrs, codegen, plan};
use proc_macro2::{Span, TokenStream};

pub fn decode_discriminant(attribs: &Attrs) -> TokenStream {
    if let Some(bits) = &attribs.bits {
        quote!(::bin_proto::BitDecode::decode(
            __io_reader,
            __byte_order,
            __ctx,
            ::bin_proto::Bits(#bits),
        ))
    } else {
        quote!(::bin_proto::BitDecode::decode(
            __io_reader,
            __byte_order,
            __ctx,
            (),
        ))
    }
}

pub fn encode_discriminant(attribs: &Attrs) -> TokenStream {
    let encode_tag = if let Some(bits) = &attribs.bits {
        quote!(::bin_proto::BitEncode::encode(
            &__tag,
            __io_writer,
            __byte_order,
            __ctx,
            ::bin_proto::Bits(#bits),
        ))
    } else {
        quote!(::bin_proto::BitEncode::encode(
            &__tag,
            __io_writer,
            __byte_order,
            __ctx,
            (),
        ))
    };
    quote!({
        let __tag = <Self as ::bin_proto::Discriminable>::discriminant(self);
        #encode_tag?;
    })
}

pub fn encode_variant_fields(plan: &plan::Enum) -> TokenStream {
    let variant_match_branches: Vec<_> = plan
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let fields_pattern = bind_fields_pattern(variant_name, &variant.fields);
            let encodes = codegen::encodes(&variant.fields, false);

            quote!(Self :: #fields_pattern => {
                #encodes
            })
        })
        .collect();

    quote!(
        match *self {
            #(#variant_match_branches,)*
        }
    )
}

pub fn variant_discriminant(plan: &plan::Enum, attribs: &Attrs) -> TokenStream {
    let discriminant_ty = &plan.discriminant_ty;
    let variant_match_branches: Vec<_> = plan
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let fields_pattern = bind_fields_pattern(variant_name, &variant.fields);
            let discriminant_expr = &variant.discriminant_value;
            let encode_variant = if let Some(field_width) = &attribs.bits {
                let error_message = format!(
                    "Discriminant for variant '{}' does not fit in bitfield.",
                    variant.ident
                );
                quote!(
                    const _: () = ::std::assert!(
                        #discriminant_expr < (1 as #discriminant_ty) << #field_width, #error_message
                    );

                    #discriminant_expr
                )
            } else {
                quote!(#discriminant_expr)
            };

            quote!(Self :: #fields_pattern => {
                #encode_variant
            })
        })
        .collect();
    quote!(match *self {
        #(#variant_match_branches,)*
    })
}

pub fn decode_variant_fields(plan: &plan::Enum) -> TokenStream {
    let discriminant_match_branches = plan.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let discriminant_literal = &variant.discriminant_value;
        let (decoder, initializer) = codegen::decodes(&variant.fields);

        quote!(
            #discriminant_literal => {
                #decoder
                Self::#variant_name #initializer
            }
        )
    });

    let discriminant_ty = &plan.discriminant_ty;

    quote!(
        {
            match ::core::convert::TryInto::<#discriminant_ty>::try_into(__tag.0)
                .map_err(|_| ::bin_proto::Error::TagConvert)? {
                #(#discriminant_match_branches,)*
                unknown_discriminant => {
                    return Err(::bin_proto::Error::UnknownEnumDiscriminant(
                        ::std::format!("{:?}", unknown_discriminant),
                    ));
                },
            }
        }
    )
}

pub fn bind_fields_pattern(parent_name: &syn::Ident, fields: &syn::Fields) -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields_named) => {
            let field_name_refs = fields_named
                .named
                .iter()
                .map(|f| &f.ident)
                .map(|n| quote!( ref #n ));
            quote!(
                #parent_name { #( #field_name_refs ),* }
            )
        }
        syn::Fields::Unnamed(ref fields_unnamed) => {
            let binding_names: Vec<_> = (0..fields_unnamed.unnamed.len())
                .map(|i| syn::Ident::new(format!("field_{i}").as_str(), Span::call_site()))
                .collect();

            let field_refs: Vec<_> = binding_names.iter().map(|i| quote!( ref #i )).collect();
            quote!(
                #parent_name ( #( #field_refs ),* )
            )
        }
        syn::Fields::Unit => quote!(#parent_name),
    }
}
