use crate::{attr, format};
use proc_macro2::{Span, TokenStream};

/// The default type used to represent
/// integer discriminants unless explicitly
/// overriden.
const DEFAULT_INT_DISCRIMINANT: &str = "u32";

/// The first integer discriminant assigned,
/// unless an explicit discriminant is given
/// in the first variant.
const DEFAULT_FIRST_INT_DISCRIMINANT: usize = 1;

/// A plan for a Protocol implementation for an enum.
pub struct Enum {
    /// The name of the enum type.
    pub ident: syn::Ident,
    /// The enum format.
    pub explicit_format: Option<format::Enum>,
    /// The `#[repr(..)]` attribute.
    pub repr_attr: Option<syn::Ident>,
    pub variants: Vec<EnumVariant>,
}

/// An enum variant.
pub struct EnumVariant {
    /// The name of this variant.
    pub ident: syn::Ident,
    /// The optional `#[protocol(discriminant(<type>))]` attribute of this variant.
    pub explicit_discriminant_attr: Option<syn::Lit>,
    /// The optional `Variant = <int value>` value.
    pub explicit_int_discriminant_equals: Option<syn::Lit>,
    /// The actual discriminant value used by this variant.
    ///
    /// Filled in by the `resolve` function.
    pub actual_discriminant: Option<syn::Lit>,
    /// The fields of the enum.
    pub fields: syn::Fields,
}

impl Enum {
    /// Creates a layout plan for an enum.
    pub fn new(ast: &syn::DeriveInput, e: &syn::DataEnum) -> Enum {
        let mut plan = Enum {
            ident: ast.ident.clone(),
            repr_attr: attr::repr(&ast.attrs),
            explicit_format: attr::protocol(&ast.attrs).discriminant_format,
            variants: e
                .variants
                .iter()
                .map(|variant| {
                    let equals_discriminant = match variant.discriminant.clone().map(|a| a.1) {
                        Some(syn::Expr::Lit(expr_lit)) => Some(expr_lit.lit),
                        Some(_) => panic!("'VariantName = <expr>' can only be used with literals"),
                        None => None,
                    };

                    EnumVariant {
                        ident: variant.ident.clone(),
                        explicit_discriminant_attr: attr::protocol(&variant.attrs).discriminant,
                        explicit_int_discriminant_equals: equals_discriminant,
                        actual_discriminant: None,
                        fields: variant.fields.clone(),
                    }
                })
                .collect(),
        };
        plan.resolve();
        plan
    }

    pub fn format(&self) -> format::Enum {
        if let Some(ref explicit_format) = self.explicit_format {
            explicit_format.clone()
        } else {
            // no explicit format given, use default
            format::Enum::default()
        }
    }

    /// Gets the type used for the discriminant.
    pub fn discriminant(&self) -> syn::Ident {
        match self.repr_attr.clone() {
            // An explicit discriminant via `#[repr(ty)]`.
            Some(ty) => ty,
            // Use the default discriminant.
            None => match self.format() {
                format::Enum::StringDiscriminant => syn::Ident::new("String", Span::call_site()),
                format::Enum::IntegerDiscriminant => {
                    syn::Ident::new(DEFAULT_INT_DISCRIMINANT, Span::call_site())
                }
            },
        }
    }

    /// Gets an expression that can be used in as the RHS in pattern matching.
    pub fn matchable_discriminant_expr(&self, variable_ident: syn::Ident) -> TokenStream {
        match self.format() {
            format::Enum::IntegerDiscriminant => quote!(#variable_ident),
            format::Enum::StringDiscriminant => quote!(&#variable_ident[..]),
        }
    }

    pub fn resolve(&mut self) {
        let mut current_default_int_discriminant = DEFAULT_FIRST_INT_DISCRIMINANT;
        let format = self.format().clone();

        for variant in self.variants.iter_mut() {
            let actual_discriminant: syn::Lit = match variant.explicit_discriminant() {
                Some(explicit_discriminant) => explicit_discriminant.clone(),
                None => match format {
                    format::Enum::StringDiscriminant => {
                        // By default, assign string discriminants as the name of
                        // the variant itself.
                        syn::LitStr::new(&variant.ident.to_string(), Span::call_site()).into()
                    }
                    format::Enum::IntegerDiscriminant => {
                        // By default, assign integer discriminants the value of the
                        // last discriminant plus one.
                        syn::LitInt::new(
                            &current_default_int_discriminant.to_string(),
                            Span::call_site(),
                        )
                        .into()
                    }
                },
            };

            // Change the default int discriminant value if relevant.
            if let syn::Lit::Int(ref discriminant_value) = actual_discriminant {
                current_default_int_discriminant =
                    discriminant_value.base10_parse::<usize>().unwrap() + 1;
            }

            // Store the actual discriminant in memory for later use.
            variant.actual_discriminant = Some(actual_discriminant);
        }
    }
}

impl EnumVariant {
    /// Gets the discriminant of the variant.
    pub fn discriminant_literal(&self) -> &syn::Lit {
        self.actual_discriminant
            .as_ref()
            .expect("discriminant has not been resolved yet")
    }

    /// Gets the discriminant explicitly specified in the code, if any.
    ///
    /// Handles all possible ways of explicitly setting discriminants.
    pub fn explicit_discriminant(&self) -> Option<&syn::Lit> {
        match (
            self.explicit_discriminant_attr.as_ref(),
            self.explicit_int_discriminant_equals.as_ref(),
        ) {
            // When both are specified, prefer the #[bin_proto] attribute
            (Some(attr), Some(_)) => Some(attr),
            // When one is specified, use it.
            (Some(lit), None) | (None, Some(lit)) => Some(lit),
            (None, None) => None,
        }
    }

    /// Gets an expression representing the discriminant.
    pub fn discriminant_expr(&self) -> TokenStream {
        match self.discriminant_literal() {
            s @ syn::Lit::Str(..) => quote!(#s.to_owned()),
            i @ syn::Lit::Int(..) => quote!(#i),
            _ => unreachable!(),
        }
    }

    /// Gets an expression representing a reference to
    /// the discriminant.
    pub fn discriminant_ref_expr(&self) -> TokenStream {
        match self.discriminant_literal() {
            s @ syn::Lit::Str(..) => quote!(&#s.to_owned()),
            i @ syn::Lit::Int(..) => quote!(&#i),
            _ => unreachable!(),
        }
    }

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
