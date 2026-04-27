use crate::{attr::Attrs, codegen, enums};
use proc_macro2::{Span, TokenStream};
use syn::{parse_quote, Error, Result};

pub fn decode_discriminant(attrs: &Attrs) -> TokenStream {
    let crate_path = attrs.crate_path();
    if let Some(bits) = &attrs.bits {
        quote!(#crate_path::BitDecode::decode::<_, __E>(
            __io_reader,
            __ctx,
            #crate_path::Bits::<#bits>,
        ))
    } else {
        quote!(#crate_path::BitDecode::decode::<_, __E>(
            __io_reader,
            __ctx,
            (),
        ))
    }
}

pub fn encode_discriminant(attrs: &Attrs) -> TokenStream {
    let crate_path = attrs.crate_path();
    let encode_tag = if let Some(bits) = &attrs.bits {
        quote!(#crate_path::BitEncode::encode::<_, __E>(
            &__tag,
            __io_writer,
            __ctx,
            #crate_path::Bits::<#bits>,
        ))
    } else {
        quote!(#crate_path::BitEncode::encode::<_, __E>(
            &__tag,
            __io_writer,
            __ctx,
            (),
        ))
    };
    quote!({
        let __tag = <Self as #crate_path::Discriminable>::discriminant(self).ok_or(#crate_path::Error::EncodeSkipped)?;
        #encode_tag?;
    })
}

pub fn encode_variant_fields(plan: &enums::Enum) -> Result<TokenStream> {
    let crate_path = &plan.crate_path;
    let variant_match_branches = plan
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let fields_pattern = bind_fields_pattern(variant_name, &variant.fields);
            let encodes = if variant.skip_encode {
                quote!(return ::core::result::Result::Err(#crate_path::Error::EncodeSkipped))
            } else {
                codegen::encodes(crate_path, &variant.fields, false)?
            };

            Ok(quote!(Self :: #fields_pattern => {
                #encodes
            }))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote!(
        match self {
            #(#variant_match_branches,)*
        }
    ))
}

pub fn variant_discriminant(plan: &enums::Enum) -> Result<TokenStream> {
    let variant_match_branches = plan
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let fields_pattern = bind_fields_pattern(variant_name, &variant.fields);
            let discriminant_expr = if variant.skip_encode {
                quote!(::core::option::Option::None)
            } else {
                let discriminant = variant
                    .discriminant_value
                    .as_ref()
                    .ok_or_else(|| Error::new(variant.ident.span(), "missing discriminant"))?;
                quote!(::core::option::Option::Some(#discriminant))
            };

            Ok(quote!(Self :: #fields_pattern => {
                #discriminant_expr
            }))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(quote!(match self {
        #(#variant_match_branches,)*
    }))
}

pub fn decode_variant_fields(plan: &enums::Enum) -> Result<TokenStream> {
    let crate_path = &plan.crate_path;
    let discriminant_match_branches = plan
        .variants
        .iter()
        .filter(|variant| !variant.discriminant_other)
        .chain(
            plan.variants
                .iter()
                .filter(|variant| variant.discriminant_other),
        )
        .filter(|variant| !variant.skip_decode)
        .map(|variant| {
            let variant_name = &variant.ident;
            let discriminant_literal = variant
                .discriminant_other
                .then(|| parse_quote!(_))
                .or_else(|| variant.discriminant_value.clone())
                .ok_or_else(|| Error::new(variant.ident.span(), "missing discriminant"))?;
            let (decoder, initializer) = codegen::decodes(crate_path, &variant.fields)?;

            Ok(quote!(
                #discriminant_literal => {
                    #decoder
                    Self::#variant_name #initializer
                }
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    let discriminant_ty = &plan.discriminant_ty;

    Ok(quote!(
        {
            match ::core::convert::TryInto::<#discriminant_ty>::try_into(__tag.0)
                .map_err(|_| #crate_path::Error::TagConvert)? {
                #(#discriminant_match_branches,)*
                unknown_discriminant => {
                    return Err(#crate_path::Error::Discriminant);
                },
            }
        }
    ))
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
