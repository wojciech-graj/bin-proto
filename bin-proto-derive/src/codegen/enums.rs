use crate::{attr::Attrs, codegen, enums};
use proc_macro2::TokenStream;
use syn::{parse_quote, Error, Result};

use crate::field::FieldsExt;

impl Attrs {
    pub fn decode_discriminant(&self) -> TokenStream {
        let crate_path = self.crate_path();
        if let Some(bits) = &self.bits {
            quote!(#crate_path::BitDecode::<__E, _, _>::decode(
                __io_reader,
                __ctx,
                #crate_path::Bits::<#bits>,
            ))
        } else {
            quote!(#crate_path::BitDecode::<__E, _, _>::decode(
                __io_reader,
                __ctx,
                (),
            ))
        }
    }

    pub fn encode_discriminant(&self) -> TokenStream {
        let crate_path = self.crate_path();
        let encode_tag = if let Some(bits) = &self.bits {
            quote!(#crate_path::BitEncode::<__E, _, _>::encode(
                &__tag,
                __io_writer,
                __ctx,
                #crate_path::Bits::<#bits>,
            ))
        } else {
            quote!(#crate_path::BitEncode::<__E, _, _>::encode(
                &__tag,
                __io_writer,
                __ctx,
                (),
            ))
        };
        quote!({
            let __tag = <Self as #crate_path::Discriminable>::discriminant(self).ok_or(#crate_path::Error::from_inner(#crate_path::error::ErrorCause::EncodeSkipped))?;
            #encode_tag?;
        })
    }
}

pub fn encode_variant_fields(plan: &enums::Enum) -> Result<TokenStream> {
    let crate_path = plan.parent_attrs.crate_path();
    let variant_match_branches = plan
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let fields_pattern = fields_pattern(&variant.fields);
            let encodes = if variant.skip_encode {
                quote!(return ::core::result::Result::Err(#crate_path::Error::from_inner(#crate_path::error::ErrorCause::EncodeSkipped)))
            } else {
                codegen::encodes(plan.parent_attrs, &variant.fields)?
            };

            Ok(quote!(Self :: #variant_name #fields_pattern => {
                #encodes
            }))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote!(
        #[allow(non_shorthand_field_patterns)]
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
            let fields_pattern = fields_pattern(&variant.fields);
            let discriminant_expr = if variant.skip_encode {
                quote!(::core::option::Option::None)
            } else {
                let discriminant = variant
                    .discriminant_value
                    .as_ref()
                    .ok_or_else(|| Error::new(variant.ident.span(), "missing discriminant"))?;
                quote!(::core::option::Option::Some(#discriminant))
            };

            Ok(quote!(Self :: #variant_name #fields_pattern => {
                #discriminant_expr
            }))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote!(
        #[allow(non_shorthand_field_patterns)]
        match self {
            #(#variant_match_branches,)*
        }
    ))
}

pub fn decode_variant_fields(plan: &enums::Enum) -> Result<TokenStream> {
    let crate_path = plan.parent_attrs.crate_path();
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
            let (decoder, initializer) = codegen::decodes(plan.parent_attrs, &variant.fields)?;

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
                .map_err(|_| #crate_path::Error::from_inner(#crate_path::error::ErrorCause::TagConvert))? {
                #(#discriminant_match_branches,)*
                _ => {
                    return Err(#crate_path::Error::from_inner(#crate_path::error::ErrorCause::Discriminant));
                },
            }
        }
    ))
}

fn fields_pattern(fields: &syn::Fields) -> TokenStream {
    let fields = fields
        .fields()
        .map(|field| field.field_value())
        .collect::<Vec<_>>();
    quote!( { #(#fields),* } )
}
