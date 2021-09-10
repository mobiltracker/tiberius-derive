use darling::{FromDeriveInput, FromMeta};

#[derive(Default, FromMeta)]
pub struct Owned;

#[derive(Default, FromMeta)]
pub struct ByIndex;

#[derive(FromDeriveInput)]
#[darling(attributes(tiberius_derive), forward_attrs(allow, doc, cfg))]
pub struct FromRowOpts<T: darling::FromGenerics, V: darling::FromVariant, F: darling::FromField> {
    pub ident: syn::Ident,
    pub attrs: Vec<syn::Attribute>,
    #[darling(default)]
    pub owned: Option<Owned>,
    #[darling(default)]
    pub by_index: Option<ByIndex>,
    pub data: darling::ast::Data<V, F>,
    pub generics: T,
}
