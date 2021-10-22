use darling::{FromDeriveInput, FromMeta};

#[derive(Default, FromMeta)]
pub struct Owned;

#[derive(Default, FromMeta)]
pub struct ByPosition;

#[derive(Debug, Default)]
pub struct RenameRule(pub ident_case::RenameRule);

impl FromMeta for RenameRule {
    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(RenameRule(ident_case::RenameRule::from_string(value)?))
    }
}

#[derive(FromDeriveInput)]
#[darling(attributes(tiberius_derive), forward_attrs(allow, doc, cfg))]
pub struct FromRowOpts<T: darling::FromGenerics, V: darling::FromVariant, F: darling::FromField> {
    pub ident: syn::Ident,
    pub attrs: Vec<syn::Attribute>,
    #[darling(default)]
    pub owned: Option<Owned>,
    #[darling(default)]
    pub by_position: Option<ByPosition>,
    pub data: darling::ast::Data<V, F>,
    pub generics: T,
    #[darling(default)]
    pub rename_all: RenameRule,
}
