#![recursion_limit = "256"]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::all))]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate heck;
extern crate proc_macro2;

use heck::SnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use syn::Ident;

#[proc_macro_derive(DieselEnum)]
pub fn diesel_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = ast.ident;

    if let syn::Data::Enum(enum_data) = ast.data {
        let variants = enum_data
            .variants
            .iter()
            .map(|vs| Ident::new(&vs.ident.to_string(), Span::call_site()))
            .collect::<Vec<_>>();

        let variants_literal = enum_data
            .variants
            .iter()
            .map(|vs| vs.ident.to_string().to_snake_case())
            .collect::<Vec<_>>();

        impl_diesel_enum(name, &variants, &variants_literal)
    } else {
        panic!("#[derive(DieselEnum)] works with enums only!");
    }
}

fn impl_diesel_enum(
    name: Ident,
    variants: &Vec<Ident>,
    variants_literal: &Vec<String>,
) -> TokenStream {
    let name_iter = std::iter::repeat(&name); // need an iterator for proc macro repeat pattern
    let name_iter1 = std::iter::repeat(&name);
    let name_iter2 = std::iter::repeat(&name);
    let name_iter3 = std::iter::repeat(&name);

    let scope = Ident::new(&format!("diesel_enum_{}", name), Span::call_site());

    let bytes_literal = &variants_literal
        .iter()
        .map(|vl| Literal::byte_string(vl.as_bytes()))
        .collect::<Vec<_>>();

    let expanded = quote! {
        mod #scope {
            use super::*;
            use diesel::deserialize::{self, FromSql, FromSqlRow, Queryable};
            use diesel::dsl::AsExprOf;
            use diesel::expression::AsExpression;
            use diesel::pg::Pg;
            use diesel::row::Row;
            use diesel::serialize::{self, IsNull, Output, ToSql};
            use diesel::sql_types::{VarChar, Nullable};
            use std::error::Error;
            use std::fmt;
            use std::io::Write;

            impl fmt::Display for #name {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(
                        f,
                        "{}",
                        match *self {
                            #(#name_iter::#variants => #variants_literal,)*
                        }
                    )
                }
            }

            impl AsExpression<VarChar> for #name {
                type Expression = AsExprOf<String, VarChar>;
                fn as_expression(self) -> Self::Expression {
                    <String as AsExpression<VarChar>>::as_expression(self.to_string())
                }
            }

            impl<'a> AsExpression<VarChar> for &'a #name {
                type Expression = AsExprOf<String, VarChar>;
                fn as_expression(self) -> Self::Expression {
                    <String as AsExpression<VarChar>>::as_expression(self.to_string())
                }
            }

            impl AsExpression<Nullable<VarChar>> for #name {
                type Expression = AsExprOf<String, Nullable<VarChar>>;
                fn as_expression(self) -> Self::Expression {
                    <String as AsExpression<Nullable<VarChar>>>::as_expression(self.to_string())
                }
            }

            impl<'a> AsExpression<Nullable<VarChar>> for &'a #name {
                type Expression = AsExprOf<String, Nullable<VarChar>>;
                fn as_expression(self) -> Self::Expression {
                    <String as AsExpression<Nullable<VarChar>>>::as_expression(self.to_string())
                }
            }

            impl ToSql<VarChar, Pg> for #name {
                fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
                    match *self {
                        #(#name_iter1::#variants => out.write_all(#bytes_literal)?,)*
                    }
                    Ok(IsNull::No)
                }
            }

            impl FromSql<VarChar, Pg> for #name {
                fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
                    match not_none!(bytes) {
                        #(#bytes_literal => Ok(#name_iter2::#variants),)*
                        v => Err(format!("Unknown value {:?} for {}", v, stringify!(#name)).into()),
                    }
                }
            }

            impl FromSqlRow<VarChar, Pg> for #name {
                fn build_from_row<R: Row<Pg>>(row: &mut R) -> Result<Self, Box<Error + Send + Sync>> {
                    match String::build_from_row(row)?.as_ref() {
                        #(#variants_literal => Ok(#name_iter3::#variants),)*
                        v => Err(format!("Unknown value {} for {}", v, stringify!(#name)).into()),
                    }
                }
            }

            impl Queryable<VarChar, Pg> for #name {
                type Row = Self;

                fn build(row: Self::Row) -> Self {
                    row
                }
            }
        }
    };
    expanded.into()
}
