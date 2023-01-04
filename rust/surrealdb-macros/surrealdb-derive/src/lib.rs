/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

extern crate proc_macro;

use proc_macro::TokenStream;
mod surrealdb_model;

#[proc_macro_derive(SurrealdbModel, attributes(surrealdb))]
pub fn surreal_model_trait_derive(input: TokenStream) -> TokenStream {
    surrealdb_model::generate_fields_getter_trait(input)
}
