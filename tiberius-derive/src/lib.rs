use crate::from_row_opts::FromRowOpts;
use darling::{ast::Fields, usage::GenericsExt, FromDeriveInput};
use from_row_opts::RenameRule;
use proc_macro::{self, TokenStream};
use proc_macro2::{Ident, Literal};
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
        by_index,
        generics,
        rename_all
    } = FromRowOpts::<syn::Generics, Variant, Field>::from_derive_input(&derive_input).unwrap();

    let generics: syn::Generics = generics;

    let lifetimes = generics.declared_lifetimes();

    let fields = match data {
        darling::ast::Data::Struct(Fields { fields, .. }) => fields.into_iter(),
        _ => unimplemented!(),
    };

    let fields = if owned.is_some() {
        try_get_rows_from_iter_owned(fields)
    } else if by_index.is_some() {
        try_get_rows_by_index(fields)
    }else{
        try_get_rows_by_key(fields,rename_all)
    };

    let expanded = if owned.is_some() {
        expand_owned(ident, fields)
    } else if lifetimes.len() == 1 {
        expand_borrowed(ident, fields)
    } else {
        expand_copy(ident, fields)
    };

    expanded.into()
}

fn try_get_rows_from_iter_owned(fields: std::vec::IntoIter<Field>) -> Vec<proc_macro2::TokenStream> {
    fields.enumerate().map(|(idx, field)| {
        let f_ident = field.ident.unwrap();
        let f_type = field.ty;

        
        quote! {
        #f_ident: {
            macro_rules! unwrap_nullable {
                (Option<$f_type: ty>) => {
                    <String as tiberius::FromSqlOwned>::from_sql_owned(row_iter.next().ok_or_else(
                        || tiberius::error::Error::Conversion(
                            format!("Could not find field {} from column with index {}", stringify!(#f_ident), #idx).into()
                        )
                    )?)?
                    };
                ($f_type: ty) => {
                    (<String as tiberius::FromSqlOwned>::from_sql_owned(row_iter.next().ok_or_else(
                        || tiberius::error::Error::Conversion(
                            format!("Could not find field {} from column with index {}", stringify!(#f_ident), #idx).into()
                        )
                    )?)?).ok_or_else(
                        || tiberius::error::Error::Conversion(
                            format!(r" None value for non optional field {} from column with index {}", stringify!(#f_ident), #idx).into()
                        )
                    )?
                };
            };

            unwrap_nullable!(#f_type)
            }
        }
    }).collect::<Vec<_>>()
}

fn try_get_rows_by_index(fields: std::vec::IntoIter<Field>) -> Vec<proc_macro2::TokenStream> {
    fields.enumerate().map(|(idx,field)| {
        let f_ident = field.ident.unwrap();
        let f_type = field.ty;

        let idx_lit = Literal::usize_suffixed(idx);
        quote! {
        #f_ident: {
            macro_rules! unwrap_nullable {
                (Option<$f_type: ty>) => {
                    row.try_get(#idx_lit)?
                };
                ($f_type: ty) => {
                    row.try_get(#idx_lit)?.ok_or_else(
                        || tiberius::error::Error::Conversion(
                            format!(r" None value for non optional field {} from column with index {}", stringify!(#f_ident), #idx_lit).into()
                            )
                        )?
                };
            };
            unwrap_nullable!(#f_type)
            }
        }
    }).collect::<Vec<_>>()
}

fn try_get_rows_by_key(fields: std::vec::IntoIter<Field>, rename_rule: RenameRule) -> Vec<proc_macro2::TokenStream> {
    fields.map(|field| {
        let f_ident =  field.ident.unwrap();
        let f_type = field.ty;
        let f_ident_string = &f_ident.to_string();
        let field_renamed = &rename_rule.0.apply_to_field(f_ident_string);


        quote! {
        #f_ident: {
            macro_rules! unwrap_nullable {
                (Option<$f_type: ty>) => {
                    row.try_get(#field_renamed)?
                };
                ($f_type: ty) => {
                    row.try_get(#field_renamed)?
                         .ok_or_else(
                            || tiberius::error::Error::Conversion(
                                format!(r" None value for non optional field {}", stringify!(#f_ident)).into()
                            )
                        )?          
                };
            };

            unwrap_nullable!(#f_type)
            }
        }
    }).collect::<Vec<_>>()
}

fn expand_owned(ident: Ident, fields: Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    quote! {
        impl #ident{
            pub fn from_row(row: tiberius::Row) -> Result<Self, tiberius::error::Error> {
                let mut row_iter = row.into_iter();

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
        impl<'a> #ident<'a>{
            pub fn from_row(row: &'a tiberius::Row) -> Result<Self, tiberius::error::Error> {
                Ok(Self{
                    #(#fields,)*
                })
            }
        }
    }
}

fn expand_copy(ident: Ident, fields: Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    quote! {
        impl #ident{
            pub fn from_row(row: &tiberius::Row) -> Result<Self, tiberius::error::Error> {
                Ok(Self{
                    #(#fields,)*
                })
            }
        }
    }
}
