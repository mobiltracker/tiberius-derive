use darling::{FromDeriveInput, ast::Fields, usage::GenericsExt};
use proc_macro::{self, TokenStream};
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Variant};
use tiberius_derive_traits::FromRowOpts;

#[proc_macro_derive(FromRow)]
pub fn from_row(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    let FromRowOpts {
        ident,
        attrs: _,
        borrowed: _,
        data,
        generics,
    } = FromRowOpts::<syn::Generics,Variant, Field>::from_derive_input(&derive_input).unwrap();

    // syn::Fields::Named(FieldsNamed { named, .. }) => named.into_iter(),
    let generics : syn::Generics = generics;

    let lifetimes = generics.declared_lifetimes();
    
    
    let fields = match data {
        darling::ast::Data::Struct(Fields { fields, .. }) => fields.into_iter(),
        _ => unimplemented!(),
    };

    let fields = fields.clone().enumerate().map(|(idx, field)| {
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
    }).collect::<Vec<_>>();

    let expanded = if lifetimes.len() == 1 {
        expand_borrowed(ident,fields)
    } else {
        expand_copy(ident, fields)
    };
    

    expanded.into()
}

fn expand_borrowed(ident: Ident, fields: Vec<proc_macro2::TokenStream>)-> proc_macro2::TokenStream {
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


fn expand_copy(ident: Ident, fields: Vec<proc_macro2::TokenStream>)-> proc_macro2::TokenStream {
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
