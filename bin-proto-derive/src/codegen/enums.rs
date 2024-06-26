use crate::{
    attr::Attrs,
    codegen,
    plan::{self, EnumVariant},
};
use proc_macro2::{Span, TokenStream};

pub fn write_variant(
    plan: &plan::Enum,
    write_discriminant: &dyn Fn(&EnumVariant) -> TokenStream,
) -> TokenStream {
    let variant_match_branches: Vec<_> = plan
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let write_discriminant = write_discriminant(variant);
            let fields_pattern = bind_fields_pattern(variant_name, &variant.fields);
            let writes = codegen::writes(&variant.fields, false);

            quote!(Self :: #fields_pattern => {
                #write_discriminant
                #writes
            })
        })
        .collect();

    quote!(
        match *self {
            #(#variant_match_branches,)*
        }
    )
}

pub fn read_variant(
    plan: &plan::Enum,
    read_discriminant: TokenStream,
    attribs: &Attrs,
) -> TokenStream {
    let discriminant_ty = &plan.discriminant_ty;

    let discriminant_match_branches = plan.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let discriminant_literal = &variant.discriminant_value;
        let (reader, initializer) = codegen::reads(&variant.fields, attribs);

        quote!(
            #discriminant_literal => {
                #reader
                Self::#variant_name #initializer
            }
        )
    });

    quote!(
        {
            let discriminant: #discriminant_ty = #read_discriminant;

            match discriminant {
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
                .map(|i| syn::Ident::new(&format!("field_{}", i), Span::call_site()))
                .collect();

            let field_refs: Vec<_> = binding_names.iter().map(|i| quote!( ref #i )).collect();
            quote!(
                #parent_name ( #( #field_refs ),* )
            )
        }
        syn::Fields::Unit => quote!(#parent_name),
    }
}
