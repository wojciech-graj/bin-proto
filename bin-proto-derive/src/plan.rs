use crate::attr::Attrs;
use proc_macro2::TokenStream;

/// A plan for a Protocol implementation for an enum.
pub struct Enum {
    pub ident: syn::Ident,
    pub discriminant_ty: syn::Ident,
    pub variants: Vec<EnumVariant>,
}

/// An enum variant.
pub struct EnumVariant {
    pub ident: syn::Ident,
    pub discriminant_value: syn::Expr,
    pub fields: syn::Fields,
}

impl Enum {
    /// Creates a layout plan for an enum.
    pub fn new(ast: &syn::DeriveInput, e: &syn::DataEnum) -> Enum {
        let attrs = Attrs::from(ast.attrs.as_slice());
        attrs.validate_enum();
        let plan = Self {
            ident: ast.ident.clone(),
            discriminant_ty: attrs.discriminant_type.unwrap(),
            variants: e
                .variants
                .iter()
                .map(|variant| {
                    let attrs = Attrs::from(variant.attrs.as_slice());
                    attrs.validate_variant();
                    let equals_discriminant = match variant.discriminant.clone().map(|a| a.1) {
                        Some(expr_lit) => expr_lit,
                        None => attrs
                            .discriminant
                            .expect("No discriminant provided for variant."),
                    };

                    EnumVariant {
                        ident: variant.ident.clone(),
                        discriminant_value: equals_discriminant,
                        fields: variant.fields.clone(),
                    }
                })
                .collect(),
        };
        plan
    }
}

impl EnumVariant {
    /// Gets a pattern expression that ignores the fields of
    /// this variant.
    pub fn ignore_fields_pattern_expr(&self) -> TokenStream {
        match self.fields {
            syn::Fields::Named(..) => quote!({ .. }),
            syn::Fields::Unnamed(..) => quote!((..)),
            syn::Fields::Unit => quote!(),
        }
    }
}
