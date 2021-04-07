use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[proc_macro_derive(TiberiusRow, attributes(Nullable))]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => named
                .into_iter()
                .map(|f| {
                    let field = f.ident.unwrap();

                    let nullable = if f.attrs.len() == 0 {
                        true
                    } else {
                        // let attr = f.attrs.first().unwrap();
                        let mut is_nullable = true;

                        for attr in f.attrs {
                            if attr.path.is_ident("Nullable") {
                                let lit: syn::LitBool = attr.parse_args().unwrap();
                                is_nullable = lit.value
                            }
                        }

                        is_nullable
                    };

                    quote! {
                        #field:  {
                            macro_rules! unwrap_nullable {
                                (true) => {
                                    __row.try_get(stringify!(#field))?
                                };
                                (false) => {
                                    __row.try_get(stringify!(#field))?.unwrap()
                                };
                            };

                            unwrap_nullable!(#nullable)
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
