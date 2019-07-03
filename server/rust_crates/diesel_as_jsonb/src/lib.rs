#![cfg_attr(feature = "cargo-clippy", allow(clippy::all))]

#![recursion_limit = "256"]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate heck;

use heck::SnakeCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{DeriveInput, Ident};

#[proc_macro_derive(AsJsonb)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let scope = Ident::new(
        &format!("{}_as_jsonb", name).to_snake_case(),
        Span::call_site(),
    );
    let proxy = Ident::new(&format!("{}ValueProxy", name), Span::call_site());
    let gen = quote! {
        mod #scope {
            use super::*;
            use diesel::sql_types::Jsonb;
            use std::io::Write;
            use diesel::pg::Pg;
            use diesel::serialize::{self, IsNull, Output, ToSql};
            use diesel::deserialize::{self, FromSql};

            #[derive(FromSqlRow, AsExpression)]
            #[diesel(foreign_derive)]
            #[sql_type = "Jsonb"]
            struct #proxy(#name);

            impl FromSql<Jsonb, Pg> for #name {
                fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
                    let bytes = not_none!(bytes);
                    if bytes[0] != 1 {
                        return Err("Unsupported JSONB encoding version".into());
                    }
                    serde_json::from_slice(&bytes[1..]).map_err(Into::into)
                }
            }

            impl ToSql<Jsonb, Pg> for #name {
                fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
                    out.write_all(&[1])?;
                    serde_json::to_writer(out, self)
                        .map(|_| IsNull::No)
                        .map_err(Into::into)
                }
            }
        }
    };
    gen.into()
}
