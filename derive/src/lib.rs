/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
#![allow(
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::type_complexity,
    clippy::needless_doctest_main
)]
#![warn(
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::mut_mut,
    clippy::non_ascii_literal,
    clippy::similar_names,
    clippy::unicode_not_nfc,
    clippy::enum_glob_use,
    clippy::if_not_else,
    clippy::items_after_statements,
    clippy::used_underscore_binding,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![cfg_attr(test, allow(clippy::unwrap_used))]
extern crate proc_macro;

use proc_macro::TokenStream;
mod migrations;
mod models;
mod xz;

#[proc_macro_derive(Node, attributes(surreal_orm))]
pub fn surreal_node_trait_derive(input: TokenStream) -> TokenStream {
    models::node::generate_fields_getter_trait(input)
}

#[proc_macro_derive(Edge, attributes(surreal_orm))]
pub fn surreal_edge_trait_derive(input: TokenStream) -> TokenStream {
    models::edge::generate_fields_getter_trait(input)
}

#[proc_macro_derive(Object, attributes(surreal_orm))]
pub fn surreal_object_trait_derive(input: TokenStream) -> TokenStream {
    models::object::generate_fields_getter_trait(input)
}

#[proc_macro_derive(TableResources, attributes(surreal_orm))]
pub fn surreal_table_resources_derive(input: TokenStream) -> TokenStream {
    migrations::table::generate_table_resources_trait(input)
}
