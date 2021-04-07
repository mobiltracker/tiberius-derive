use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[proc_macro_derive(TiberiusRow, attributes(Owned))]
pub fn tiberius_row(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => named
                .into_iter()
                .map(|f| {
                    let field = f.ident.unwrap();
                    let f_type = f.ty;

                    quote! {
                        #field:  {
                            macro_rules! unwrap_nullable {
                                (String) => {
                                    __row.try_get::<&str, &str>(stringify!(#field))?.expect(&format!("Failed to get field {}",stringify!(#field))).to_owned()

                                };
                                (Option<String>) => {
                                    __row.try_get::<&str, &str>(stringify!(#field))?.map(|f|f.to_owned())
                                };
                                (Option<$f_type: ty>) => {
                                    __row.try_get(stringify!(#field))?
                                };
                                ($f_type: ty) => {
                                    __row.try_get(stringify!(#field))?.expect(&format!("Failed to get field {}",stringify!(#field)))
                                };
                            }
                            unwrap_nullable!(#f_type)
                        }
                    }
                })
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
                    #(#fields,)*
                })
            }
        }
    };

    output.into()
}
