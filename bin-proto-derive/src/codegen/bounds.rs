use std::collections::HashSet;

use proc_macro2::TokenStream;
use syn::{
    parse_quote,
    spanned::Spanned,
    visit::{self, Visit},
    Ident, Result, Type,
};

use crate::{
    attr::{AttrKind, Attrs, Tag},
    Operation,
};

/// Computes the `where`-clause predicates required for a derived impl to
/// encode or decode the given fields.
///
/// A bound is only emitted for types that mention one of the deriving type's
/// generic type parameters. Fully concrete field types are already checked at
/// their point of use inside the generated method body, and bounding them
/// would unnecessarily restrict the impl (and break recursive types).
pub struct FieldBounds<'a> {
    parent_attrs: &'a Attrs,
    type_params: HashSet<&'a Ident>,
    operation: Operation,
    predicates: Vec<syn::WherePredicate>,
}

impl<'a> FieldBounds<'a> {
    pub fn new(parent_attrs: &'a Attrs, generics: &'a syn::Generics, operation: Operation) -> Self {
        Self {
            parent_attrs,
            type_params: generics.type_params().map(|param| &param.ident).collect(),
            operation,
            predicates: Vec::new(),
        }
    }

    /// Adds the bounds required to encode or decode every relevant field.
    pub fn add_fields(&mut self, fields: &syn::Fields) -> Result<()> {
        for field in fields {
            self.add_field(field)?;
        }
        Ok(())
    }

    fn add_field(&mut self, field: &syn::Field) -> Result<()> {
        let attrs = Attrs::parse(
            Some(self.parent_attrs),
            field.attrs.as_slice(),
            Some(AttrKind::Field),
            field.span(),
        )?;

        let skipped = match self.operation {
            Operation::Decode => attrs.skip_decode,
            Operation::Encode => attrs.skip_encode,
        };
        if skipped {
            return Ok(());
        }

        let crate_path = attrs.crate_path();

        let field_tag = match &attrs.tag {
            Some(Tag::Prepend { typ, bits, .. }) => {
                // The prepended tag itself is encoded/decoded with a unit or
                // bitfield tag.
                let tag_tag = bits
                    .as_ref()
                    .map_or_else(|| quote!(()), |bits| quote!(#crate_path::Bits<{ #bits }>));
                self.add_bound(typ, &tag_tag);
                match self.operation {
                    Operation::Decode => Some(quote!(#crate_path::Tag<#typ>)),
                    Operation::Encode => Some(quote!(#crate_path::Untagged)),
                }
            }
            Some(Tag::External(_)) => match self.operation {
                // The tag type is the type of an arbitrary expression, which
                // cannot be named here, so no bound can be generated.
                Operation::Decode => None,
                Operation::Encode => Some(quote!(#crate_path::Untagged)),
            },
            None => Some(if let Some(bits) = &attrs.bits {
                quote!(#crate_path::Bits<{ #bits }>)
            } else if attrs.untagged {
                quote!(#crate_path::Untagged)
            } else {
                quote!(())
            }),
        };

        if let Some(tag_ty) = field_tag {
            self.add_bound(&field.ty, &tag_ty);
        }

        Ok(())
    }

    /// Adds a `BitDecode`/`BitEncode` bound for `ty` with the given tag type,
    /// if `ty` mentions a generic type parameter.
    pub fn add_bound(&mut self, ty: &Type, tag_ty: &TokenStream) {
        if !mentions_ident(ty, &self.type_params) {
            return;
        }

        let crate_path = self.parent_attrs.crate_path();
        let ctx_ty = self.parent_attrs.ctx_ty();
        let trait_name = match self.operation {
            Operation::Decode => quote!(BitDecode),
            Operation::Encode => quote!(BitEncode),
        };

        self.predicates
            .push(parse_quote!(#ty: #crate_path::#trait_name<#ctx_ty, #tag_ty>));
    }

    pub fn into_predicates(self) -> Vec<syn::WherePredicate> {
        self.predicates
    }
}

struct TypeParamVisitor<'a> {
    type_params: &'a HashSet<&'a Ident>,
    mentions: bool,
}

impl<'ast> Visit<'ast> for TypeParamVisitor<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        if self.mentions {
            return;
        }

        if node.leading_colon.is_none() {
            if let Some(first_segment) = node.segments.first() {
                if self.type_params.contains(&first_segment.ident) {
                    self.mentions = true;
                    return;
                }
            }
        }

        visit::visit_path(self, node);
    }
}

/// Whether any identifier in `tokens` structurally represents one of the `type_params`.
fn mentions_ident(typ: &Type, type_params: &HashSet<&Ident>) -> bool {
    let mut visitor = TypeParamVisitor {
        type_params,
        mentions: false,
    };
    visitor.visit_type(typ);
    visitor.mentions
}
