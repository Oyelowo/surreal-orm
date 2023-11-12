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

use migration::generate_embedded_migrations;
use proc_macro::TokenStream;
use surreal_query_builder::sql;
use syn::{parse_macro_input, LitStr};
mod migration;
mod models;
mod query;
mod query_builder;

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

#[proc_macro]
pub fn query_raw(raw_input: TokenStream) -> TokenStream {
    let r_input = raw_input.clone();
    let input = parse_macro_input!(r_input as LitStr);
    let input = input.value();
    let sql = sql::parse(input.as_str());

    match sql {
        Ok(value) => value,
        Err(value) => {
            return syn::Error::new_spanned(input, value)
                .to_compile_error()
                .into()
        }
    };
    raw_input
}

#[proc_macro]
pub fn query(raw_input: TokenStream) -> TokenStream {
    query::query(raw_input.into()).into()
}

#[proc_macro]
pub fn query_turbo(input: TokenStream) -> TokenStream {
    query_builder::block(input.into()).into()
}

/// embed_migrations!() is a macro that embeds migrations in the binary at compile time.
/// It takes 3 arguments:
/// 1. The path to the migrations directory
/// 2. The migration flag
/// 3. The migration mode
/// The path to the migrations directory is optional. If not provided, it defaults to 'migrations'.
/// The migration flag is optional. If not provided, it defaults to MigrationFlag::TwoWay.
/// The migration mode is optional. If not provided, it defaults to Mode::Strict.
///
/// # Example
/// ```
/// use surreal_orm::migrator::{self, embed_migrations};
/// use surreal_orm::migrator::{FileManager, MigrationFlag, MigratorDatabase, Mode};
///
/// // Embed migrations as constant
/// const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay = embed_migrations!();
/// const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay = embed_migrations!("migrations");
/// const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay = embed_migrations!("migrations", one_way);
/// const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay = embed_migrations!("migrations", one_way, strict);
///
/// const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay = embed_migrations!();
/// const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("migrations");
/// const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("migrations", two_way);
/// const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("migrations", two_way, strict);
/// ```
#[proc_macro]
pub fn embed_migrations(input: TokenStream) -> TokenStream {
    generate_embedded_migrations(input)
}
