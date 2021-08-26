#[macro_use]
extern crate darling;
extern crate syn;

pub trait FromRow<'a> {
    fn from_row(__row: &'a tiberius::Row) -> Result<&'a Self, tiberius::error::Error>
    where
        Self: std::marker::Sized;
}

#[derive(Default, FromMeta)]
#[darling(default)]
pub struct Owned(Option<()>);

#[derive(FromDeriveInput)]
#[darling(attributes(tiberius_derive), forward_attrs(allow, doc, cfg))]
pub struct FromRowOpts<V: darling::FromVariant, F: darling::FromField> {
    pub ident: syn::Ident,
    pub attrs: Vec<syn::Attribute>,
    pub owned: Owned,
    pub data: darling::ast::Data<V, F>,
}
