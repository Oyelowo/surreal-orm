/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

extern crate proc_macro;

use proc_macro::TokenStream;
mod surrealdb_edge;
mod surrealdb_node;

#[proc_macro_derive(SurrealdbNode, attributes(surrealdb))]
pub fn surreal_node_trait_derive(input: TokenStream) -> TokenStream {
    surrealdb_node::generate_fields_getter_trait(input)
}

#[proc_macro_derive(SurrealdbEdge, attributes(surrealdb))]
pub fn surreal_edge_trait_derive(input: TokenStream) -> TokenStream {
    surrealdb_edge::generate_fields_getter_trait(input)
}
