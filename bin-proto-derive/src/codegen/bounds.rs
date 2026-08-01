use syn::{parse_quote, spanned::Spanned, Result, Type};

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
    operation: Operation,
    predicates: Vec<syn::WherePredicate>,
}

impl<'a> FieldBounds<'a> {
    pub const fn new(parent_attrs: &'a Attrs, operation: Operation) -> Self {
        Self {
            parent_attrs,
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
                    .map(|bits| parse_quote!(#crate_path::Bits<{ #bits }>));
                self.add_bound(typ, tag_tag);
                Some(Some(match self.operation {
                    Operation::Decode => parse_quote!(#crate_path::Tag<#typ>),
                    Operation::Encode => parse_quote!(#crate_path::Untagged),
                }))
            }
            Some(Tag::External(_)) => match self.operation {
                // The tag type is the type of an arbitrary expression, which
                // cannot be named here, so no bound can be generated.
                Operation::Decode => None,
                Operation::Encode => Some(Some(parse_quote!(#crate_path::Untagged))),
            },
            None => Some(if let Some(bits) = &attrs.bits {
                Some(parse_quote!(#crate_path::Bits<{ #bits }>))
            } else if attrs.untagged {
                Some(parse_quote!(#crate_path::Untagged))
            } else {
                None
            }),
        };

        if let Some(tag_ty) = field_tag {
            self.add_bound(&field.ty, tag_ty);
        }

        Ok(())
    }

    /// Adds a `BitDecode`/`BitEncode` bound for `ty` with the given tag type,
    /// if `ty` mentions a generic type parameter.
    pub fn add_bound(&mut self, ty: &Type, tag_ty: Option<Type>) {
        let crate_path = self.parent_attrs.crate_path();
        let ctx_ty = self.parent_attrs.ctx_ty();
        let trait_name = match self.operation {
            Operation::Decode => quote!(BitDecode),
            Operation::Encode => quote!(BitEncode),
        };
        let tag_ty = tag_ty.unwrap_or_else(|| parse_quote!(()));

        self.predicates
            .push(parse_quote!(#ty: #crate_path::#trait_name<__E, #ctx_ty, #tag_ty>));
    }

    pub fn into_predicates(self) -> Vec<syn::WherePredicate> {
        self.predicates
    }
}
