#![recursion_limit = "128"]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::all))]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use crate::proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(EventTs)]
pub fn event_ts_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // type name
    let name = &input.ident;

    // generics
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics eventsourcing::EventData for #name #ty_generics #where_clause {
            fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
                return self.timestamp;
            }
        }
    };

    TokenStream::from(expanded)
}


#[proc_macro_derive(Aggregate)]
pub fn aggregate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // type name
    let name = &input.ident;

    // generics
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics eventsourcing::AggregateData for #name #ty_generics #where_clause {
            fn increment_version(&mut self) {
                self.version += 1;
            }

            fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
                self.updated_at = timestamp;
            }
        }
    };

    TokenStream::from(expanded)
}
