use darling::{ast::Fields, FromDeriveInput};
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, FieldsNamed, Variant};
use tiberius_derive_traits::FromRowOpts;

// impl<'a> FromRow<'a> for Foobar<'a> {
//     fn from_row(__row: &'a tiberius::Row) -> Result<Foobar<'a>, tiberius::error::Error> {
//         let foo = __row.get(0);
//         let bar = __row.get(1);
//         let xar = __row.get::<&'a str, usize>(2).expect("foobar");

//         Ok(Self { bar, foo, xar })
//     }
// }

#[proc_macro_derive(FromRow)]
pub fn from_row(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    let FromRowOpts {
        ident,
        attrs,
        owned,
        data,
    } = FromRowOpts::<Variant, Field>::from_derive_input(&derive_input).unwrap();

    // syn::Fields::Named(FieldsNamed { named, .. }) => named.into_iter(),

    let fields = match data {
        darling::ast::Data::Struct(Fields { fields, .. }) => fields.into_iter(),
        _ => unimplemented!(),
    };

    let try_get_field = fields.clone().enumerate().map(|(idx, field)| {
        let f_ident = field.ident.unwrap();
        let f_type = field.ty;

        quote! {
            #f_ident: {
                let #f_ident = __row.try_get(#idx)?;
                #f_ident
            }
        }
    });

    let expanded = quote! {
        impl<'a> tiberius_derive_traits::FromRow<'a> for #ident<'a>{
            fn from_row(__row: &'a tiberius::Row) -> Result<#ident<'a>, tiberius::error::Error> {
                Ok(Self{#(
                    #try_get_field,
                )*})
            }
        }

    };

    expanded.into()
}
