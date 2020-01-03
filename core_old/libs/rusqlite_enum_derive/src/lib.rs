#![recursion_limit = "128"]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::all))]

// TODO(z0mbie42)
extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use crate::proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(RusqliteEnum)]
pub fn enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // type name
    let name = &input.ident;

    // generics
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics rusqlite::ToSql for #name #ty_generics #where_clause {
            fn to_sql(&self) -> rusqlite::Result<ruqlite::types::ToSqlOutput> {
                let text = format!("{}", self);
                return Ok(rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Text(text)));
            }
        }

        impl #impl_generics rusqlite::FromSql for #name #ty_generics #where_clause {
            fn column_result(value: ruqlite::types::ValueRef) -> ruqlite::types::FromSqlResult<Self> {
                if let types::ValueRef::Text(s) = value {
                    return Ok(s.to_owned());

                } else {
                    return Err(rusqlite::types::FromSqlError::InvalidType);
                }
            }
        }
    };

    TokenStream::from(expanded)
}
