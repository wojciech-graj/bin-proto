use crate::attr::Attrs;

pub struct Enum {
    pub discriminant_ty: syn::Type,
    pub variants: Vec<EnumVariant>,
}

pub struct EnumVariant {
    pub ident: syn::Ident,
    pub discriminant_value: syn::Expr,
    pub fields: syn::Fields,
}

impl Enum {
    pub fn new(ast: &syn::DeriveInput, e: &syn::DataEnum) -> Enum {
        let attrs = Attrs::from(ast.attrs.as_slice());
        attrs.validate_enum();
        let plan = Self {
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
