use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};

#[proc_macro_derive(TiberiusRow)]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => named
                .into_iter()
                .map(|f| f.ident.unwrap())
                .collect::<Vec<_>>(),
            _ => panic!("Named struct fields only"),
        },
        _ => panic!("Expected struct"),
    };

    let output = quote! {

    impl TryFrom<tiberius::Row> for #ident {
        type Error = tiberius::error::Error;

        fn try_from(__row: tiberius::Row) -> Result<Self, Self::Error> {
                Ok(Self{
                    #(#fields : __row.try_get(stringify!(#fields))?,)*
                })
            }
        }
    };

    output.into()
}
