use crate::attr::{Attrs, Ctx};

use proc_macro2::{Span, TokenStream};
use syn::{parse_quote, punctuated::Punctuated, spanned::Spanned, Token};

#[allow(clippy::large_enum_variant)]
pub enum TraitImplType {
    Decode,
    Encode,
    TaggedDecode(syn::Type),
    UntaggedEncode,
    Discriminable,
}

pub fn impl_trait_for(
    ast: &syn::DeriveInput,
    impl_body: &TokenStream,
    typ: &TraitImplType,
) -> TokenStream {
    let name = &ast.ident;
    let attrs = match Attrs::parse(ast.attrs.as_slice(), None, ast.span()) {
        Ok(attrs) => attrs,
        Err(e) => return e.to_compile_error(),
    };

    let generics = &ast.generics;
    let (_, ty_generics, _) = generics.split_for_impl();
    let mut generics = ast.generics.clone();

    let mut trait_generics: Punctuated<TokenStream, Token![,]> = Punctuated::new();

    if matches!(
        typ,
        TraitImplType::Decode
            | TraitImplType::Encode
            | TraitImplType::TaggedDecode(_)
            | TraitImplType::UntaggedEncode
    ) {
        if let Some(ctx_generics) = attrs.ctx_generics {
            generics.params.extend(ctx_generics);
        }

        trait_generics.push(if let Some(Ctx::Concrete(ctx)) = attrs.ctx {
            quote!(#ctx)
        } else {
            let ident = syn::Ident::new("__Ctx", Span::call_site());
            let bounds = if let Some(Ctx::Bounds(bounds)) = attrs.ctx {
                bounds.into_iter().collect()
            } else {
                Punctuated::new()
            };
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
            quote!(#ident)
        });
    }

    let trait_name = match &typ {
        TraitImplType::Decode => quote!(BitDecode),
        TraitImplType::Encode => quote!(BitEncode),
        TraitImplType::UntaggedEncode => {
            trait_generics.push(quote!(::bin_proto::Untagged));
            quote!(BitEncode)
        }
        TraitImplType::TaggedDecode(discriminant) => {
            let mut bounds = Punctuated::new();
            bounds.push(parse_quote!(::std::convert::TryInto<#discriminant>));
            generics
                .params
                .push(syn::GenericParam::Type(syn::TypeParam {
                    attrs: Vec::new(),
                    ident: syn::Ident::new("__Tag", Span::call_site()),
                    colon_token: None,
                    bounds,
                    eq_token: None,
                    default: None,
                }));
            trait_generics.push(quote!(::bin_proto::Tag<__Tag>));
            quote!(BitDecode)
        }
        TraitImplType::Discriminable => quote!(Discriminable),
    };

    let (impl_generics, _, where_clause) = generics.split_for_impl();
    quote!(
        #[automatically_derived]
        impl #impl_generics ::bin_proto::#trait_name<#trait_generics> for #name #ty_generics
        #where_clause {
            #impl_body
        }
    )
}
