use syn::parse_quote;

pub struct Field<'a> {
    pub member: syn::Member,
    pub field: &'a syn::Field,
}

pub trait FieldsExt {
    fn fields(&self) -> impl Iterator<Item = Field<'_>>;
}

impl FieldsExt for syn::Fields {
    fn fields(&self) -> impl Iterator<Item = Field<'_>> {
        self.iter().enumerate().map(|(i, field)| Field {
            member: if let Some(ident) = field.ident.clone() {
                syn::Member::Named(ident)
            } else {
                syn::Member::Unnamed(syn::Index::from(i))
            },
            field,
        })
    }
}

impl Field<'_> {
    pub fn binding(&self) -> syn::Ident {
        match &self.member {
            syn::Member::Named(ident) => ident.clone(),
            syn::Member::Unnamed(index) => format_ident!("field_{}", index.index),
        }
    }

    pub fn field_value(&self) -> syn::FieldValue {
        let binding = self.binding();
        let member = &self.member;
        parse_quote!(#member: #binding)
    }
}
