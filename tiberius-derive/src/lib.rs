use crate::from_row_opts::FromRowOpts;
use darling::{ast::Fields, usage::GenericsExt, FromDeriveInput};
use proc_macro::{self, TokenStream};
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Variant};
mod from_row_opts;

#[proc_macro_derive(FromRow, attributes(tiberius_derive))]
pub fn from_row(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    let FromRowOpts {
        ident,
        attrs: _,
        owned,
        data,
        generics,
    } = FromRowOpts::<syn::Generics, Variant, Field>::from_derive_input(&derive_input).unwrap();

    let generics: syn::Generics = generics;

    let lifetimes = generics.declared_lifetimes();

    let fields = match data {
        darling::ast::Data::Struct(Fields { fields, .. }) => fields.into_iter(),
        _ => unimplemented!(),
    };

    let fields = if owned.is_some() {
        row_from_iter_owned(fields)
    } else {
        try_get_rows(fields)
    };

    let expanded = if owned.is_some() {
        expand_owned(ident, fields)
    } else {
        if lifetimes.len() == 1 {
            expand_borrowed(ident, fields)
        } else {
            expand_copy(ident, fields)
        }
    };

    expanded.into()
}

fn row_from_iter_owned(fields: std::vec::IntoIter<Field>) -> Vec<proc_macro2::TokenStream> {
    fields.clone().enumerate().map(|(idx, field)| {
        let f_ident = field.ident.unwrap();
        let f_type = field.ty;
        quote! {
        #f_ident: {
            macro_rules! unwrap_nullable {
                (Option<$f_type: ty>) => {
                    <String as tiberius::FromSqlOwned>::from_sql_owned(row_iter.next().ok_or_else(
                        || tiberius::error::Error::Io {
                            kind: std::io::ErrorKind::InvalidData,
                            message: format!("Failed to get from row: field {} not found in position {}", stringify!(#f_ident), #idx),
                        }
                    )?)?
                    };
                ($f_type: ty) => {
                    (<String as tiberius::FromSqlOwned>::from_sql_owned(row_iter.next().ok_or_else(
                        || tiberius::error::Error::Io {
                            kind: std::io::ErrorKind::InvalidData,
                            message: format!("Failed to get from row: field {} not found in position {}", stringify!(#f_ident), #idx),
                        }
                    )?)?).ok_or_else(
                        || tiberius::error::Error::Io {
                            kind: std::io::ErrorKind::InvalidData,
                            message: format!("Failed to parse row: 'None' value for non optional field {} in position {}", stringify!(#f_ident), #idx),
                    })?
                };
            };

            unwrap_nullable!(#f_type)
            }
        }
    }).collect::<Vec<_>>()
}

fn try_get_rows(fields: std::vec::IntoIter<Field>) -> Vec<proc_macro2::TokenStream> {
    fields.clone().map(|field| {
        let f_ident = field.ident.unwrap();
        let f_type = field.ty;

        quote! {
        #f_ident: {
            macro_rules! unwrap_nullable {
                (Option<$f_type: ty>) => {
                    __row.try_get(stringify!(#f_ident))?
                };
                ($f_type: ty) => {
                    __row.try_get(stringify!(#f_ident))?.expect(&format!("Failed to get field {}",stringify!(#f_ident)))
                };
            };

            unwrap_nullable!(#f_type)
            }
        }
    }).collect::<Vec<_>>()
}

fn expand_owned(ident: Ident, fields: Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    quote! {
        impl tiberius_derive_traits::FromRowOwned for #ident{
            fn from_row(__row: tiberius::Row) -> Result<#ident, tiberius::error::Error> {
                let mut row_iter = __row.into_iter();

                Ok(Self{
                    #(#fields,)*
                })
            }
        }
    }
}

fn expand_borrowed(
    ident: Ident,
    fields: Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote! {
        impl<'a> tiberius_derive_traits::FromRow<'a> for #ident<'a>{
            fn from_row(__row: &'a tiberius::Row) -> Result<#ident<'a>, tiberius::error::Error> {
                Ok(Self{
                    #(#fields,)*
                })
            }
        }
    }
}

fn expand_copy(ident: Ident, fields: Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    quote! {
        impl tiberius_derive_traits::FromRowCopy for #ident{
            fn from_row(__row: &tiberius::Row) -> Result<#ident, tiberius::error::Error> {
                Ok(Self{
                    #(#fields,)*
                })
            }
        }
    }
}
