#[macro_use]
extern crate darling;
extern crate syn;

pub trait FromRow<'a> {
    fn from_row(__row: &'a tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: std::marker::Sized;
}

pub trait FromRowCopy {
    fn from_row(__row: &tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: std::marker::Sized;
}

pub trait FromRowOwned {
    fn from_row(__row: tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: std::marker::Sized;
}

#[allow(dead_code, unused_must_use)]
fn pad(row: tiberius::Row) -> Result<(), tiberius::error::Error> {
    Ok(())
}

#[derive(Default, FromMeta)]
pub struct Borrowed;

#[derive(FromDeriveInput)]
#[darling(attributes(tiberius_derive), forward_attrs(allow, doc, cfg))]
pub struct FromRowOpts<T: darling::FromGenerics, V: darling::FromVariant, F: darling::FromField> {
    pub ident: syn::Ident,
    pub attrs: Vec<syn::Attribute>,
    #[darling(default)]
    pub borrowed: Option<Borrowed>,
    pub data: darling::ast::Data<V, F>,
    pub generics: T,
}
