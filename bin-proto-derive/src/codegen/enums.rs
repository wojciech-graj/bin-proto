use crate::{
    codegen,
    plan::{self, EnumVariant},
};
use proc_macro2::{Span, TokenStream};

/// Generates code that reads one of a set of
/// parcel variants and returns an expression
/// of the same type as the enum.
pub fn write_variant(
    plan: &plan::Enum,
    write_discriminant: &dyn Fn(&EnumVariant) -> TokenStream,
) -> TokenStream {
    let enum_name = &plan.ident;

    let variant_match_branches: Vec<_> = plan
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            let write_discriminant = write_discriminant(variant);

            let (binding_names, fields_pattern) =
                bind_fields_pattern(variant_name, &variant.fields);

            quote!(#enum_name :: #fields_pattern => {
                #write_discriminant

                #(
                    bin_proto::Protocol::write(#binding_names, __io_writer, __settings, __ctx)?;
                )*
            })
        })
        .collect();

    quote! {
        match *self {
            #(#variant_match_branches,)*
            _ => return Err(bin_proto::Error::UnknownEnumDiscriminant(String::new())),
        }
    }
}

pub fn read_variant(plan: &plan::Enum, read_discriminant: TokenStream) -> TokenStream {
    let enum_name = &plan.ident;
    let discriminant_ty = plan.discriminant();
    let discriminant_var = syn::Ident::new("discriminant", Span::call_site());
    let discriminant_for_pattern_matching =
        plan.matchable_discriminant_expr(discriminant_var.clone());

    let discriminant_match_branches = plan.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let discriminant_literal = variant.discriminant_literal();
        let initializer = codegen::reads(&variant.fields);

        quote! {
            #discriminant_literal => {
                #enum_name::#variant_name # initializer
            }
        }
    });

    quote! {
        {
            let discriminant: #discriminant_ty = #read_discriminant;

            match #discriminant_for_pattern_matching {
                #(#discriminant_match_branches,)*
                unknown_discriminant => {
                    return Err(bin_proto::Error::UnknownEnumDiscriminant(
                        format!("{:?}", unknown_discriminant),
                    ));
                },
            }
        }
    }
}

/// Generates code for a pattern that binds a set of fields by reference.
///
/// Returns a tuple of the pattern tokens and the field binding names.
pub fn bind_fields_pattern(
    parent_name: &syn::Ident,
    fields: &syn::Fields,
) -> (Vec<syn::Ident>, TokenStream) {
    match *fields {
        syn::Fields::Named(ref fields_named) => {
            let field_names: Vec<_> = fields_named
                .named
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect();
            let field_name_refs = fields_named
                .named
                .iter()
                .map(|f| &f.ident)
                .map(|n| quote! { ref #n });

            (
                field_names,
                quote! {
                    #parent_name { #( #field_name_refs ),* }
                },
            )
        }
        syn::Fields::Unnamed(ref fields_unnamed) => {
            let binding_names: Vec<_> = (0..fields_unnamed.unnamed.len())
                .map(|i| syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site()))
                .collect();

            let field_refs: Vec<_> = binding_names.iter().map(|i| quote! { ref #i }).collect();

            (
                binding_names,
                quote! {
                    #parent_name ( #( #field_refs ),* )
                },
            )
        }
        syn::Fields::Unit => (Vec::new(), quote!(#parent_name)),
    }
}
