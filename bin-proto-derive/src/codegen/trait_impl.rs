use crate::attr::Attrs;

use proc_macro2::{Span, TokenStream};
use syn::{parse_quote, punctuated::Punctuated, Token};

pub enum TraitImplType {
    ProtocolRead,
    ProtocolWrite,
    TaggedRead(syn::Type),
    UntaggedWrite,
    Discriminable,
}

pub fn impl_trait_for(
    ast: &syn::DeriveInput,
    impl_body: &TokenStream,
    typ: &TraitImplType,
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

    let trait_name = match &typ {
        TraitImplType::ProtocolRead => quote!(ProtocolRead),
        TraitImplType::ProtocolWrite => quote!(ProtocolWrite),
        TraitImplType::TaggedRead(discriminant) => {
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
            quote!(TaggedRead)
        }
        TraitImplType::UntaggedWrite => quote!(UntaggedWrite),
        TraitImplType::Discriminable => quote!(Discriminable),
    };

    if matches!(
        typ,
        TraitImplType::ProtocolRead
            | TraitImplType::ProtocolWrite
            | TraitImplType::TaggedRead(_)
            | TraitImplType::UntaggedWrite
    ) {
        trait_generics.push(if let Some(ctx) = attribs.ctx {
            if let Some(ctx_generics) = attribs.ctx_generics {
                generics.params.extend(ctx_generics);
            }
            quote!(#ctx)
        } else {
            let ident = syn::Ident::new("__Ctx", Span::call_site());
            generics
                .params
                .push(syn::GenericParam::Type(syn::TypeParam {
                    attrs: Vec::new(),
                    ident: ident.clone(),
                    colon_token: None,
                    bounds: attribs
                        .ctx_bounds
                        .map(|ctx_bounds| ctx_bounds.into_iter().collect())
                        .unwrap_or_default(),
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
