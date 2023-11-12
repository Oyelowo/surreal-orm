// 24 Aug, 2023. Not yet supported in stable error[E0554]: `#![feature]` may not be used on the stable release channel
// #![feature(rustdoc_missing_doc_code_examples)]
/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

//! # surreal-orm is a hyper expressive and intuitive query builder and ORM for surrealdb implemented in Rust.
//! If you know raw SurrealQl, you know surreal-orm.
//!
//! <div align="center">
//! <!-- CI -->
//! <img src="https://github.com/oyelowo/surreal-orm/workflows/CI/badge.svg" />
//! <!-- codecov -->
//! <img src="https://codecov.io/gh/surreal-orm/surreal-orm/branch/master/graph/badge.svg" />
//! <!-- Crates version -->
//! <a href="https://crates.io/crates/surreal-orm">
//! <img src="https://img.shields.io/crates/v/surreal-orm.svg?style=flat-square"
//! alt="Crates.io version" />
//! </a>
//! <!-- Downloads -->
//! <a href="https://crates.io/crates/surreal-orm">
//! <img src="https://img.shields.io/crates/d/surreal-orm.svg?style=flat-square"
//! alt="Download" />
//! </a>
//! <!-- docs.rs docs -->
//! <a href="https://docs.rs/surreal-orm">
//! <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
//! alt="docs.rs docs" />
//! </a>
//! <a href="https://github.com/rust-secure-code/safety-dance/">
//! <img src="https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square"
//! alt="Unsafe Rust forbidden" />
//! </a>
//! </div>
//!
//! ## Documentation
//!
//! * [Book](https://surreal-orm.github.io/surreal-orm/en/index.html)
//! * [fr-placeholder](https://surreal-orm.github.io/surreal-orm/fr/index.html)
//! * [Docs](https://docs.rs/surreal-orm)
//! * [GitHub repository](https://github.com/oyelowo/surreal-orm)
//! * [Cargo package](https://crates.io/crates/surreal-orm)
//! * Minimum supported Rust version: 1.60.0 or later
//!
//! # Table of contents
//!
//! - [Features](#high-level-features)
//! - [Getting Started](#getting-started)
//! - [Example](#example)
//! - [Surreal Model](#surreal-model)
//! - [Surreal Node](#Node)
//! - [Surreal Edge](#surreal-edge)
//! - [Surreal Object](#surreal-object)
//! - [Query Execution](#query-execution)
//! - [Examples](#examples)
//!
//!
//!
//! ## Features
//!
//! * Fully supports for surrealdb specifications
//! * Compile-time Type safety
//! * Intuitive, innovative and idiomatic API. If you know surrealql, you know surreal-orm
//! * Rustfmt friendly (Procedural Macro)
//! * Complex query of any nesting level
//! * Automatic parameter binding and sql injection handling
//! * Automatic Struct mapping for insert statement parameters
//! * Automatic return type for query return types
//! * Fully typed dynamic filterable graph building of any depth
//! * Fully typed dynamic filterable node
//! * Fully typed nested filterable object
//! * Fully compile-time checked schema type definition
//! * Complex Transaction
//! * Query Chaining
//! * All surrealdb Statements including:
//! USE, LET, BEGIN, CANCEL, COMMIT, IF ELSE, SELECT, INSERT, CREATE, UPDATE, RELATE,
//! DELETE, DEFINE, REMOVE, INFO, SLEEP
//! * Query execution
//! * All Surreal Operators e.g CONTAINSALL, INTERSECTS, == etc
//! * Array functions
//! * Count function
//! * Crypto functions
//! * Geo functions
//! * HTTP functions
//! * Validation functions
//! * Math functions
//! * Meta functions
//! * Parse functions
//! * Rand functions
//! * Session functions
//! * Sleep function
//! * String functions
//! * Time functions
//! * Type functions
//! * Scripting functions
//! * All Surreal types
//! * Surreal parameters
//! * All Surreal cast functions
//!
//! ## Getting Started
//! ## How to Install
//!```cargo.toml
//! [dependencies]
//! surreal-orm = "1.0"!
//!```
//!
//! ## Integrations
//!
//! * Surreal [surrealdb](https://crates.io/crates/surrealdb)
//!
//! ## License
//!
//! Licensed under either of
//!
//! * Apache License, Version 2.0,
//! (./LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license (./LICENSE-MIT or <http://opensource.org/licenses/MIT>)
//! at your option.
//!
//! ## References
//!
//! * [Surreal](https://surrealdb.com)
//!
//! ## Examples
//!
//! All examples are in the [sub-repository](https://github.com/oyelowo/surreal-orm/examples), located in the examples directory.
//!
//! **Run an example:**
//!
//! ```shell
//! git submodule update # update the examples repo
//! cd examples && cargo run --bin [name]
//! ```
//!
//! ## Benchmarks
//!
//! Ensure that there is no CPU-heavy process in background!
//!
//! ```shell script
//! cd benchmark
//! cargo bench
//! ```
//!
//! Now a HTML report is available at `benchmark/target/criterion/report`.

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::inefficient_to_string)]
#![deny(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::if_not_else)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::needless_pass_by_value)]
#![deny(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::map_flatten)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::unused_self)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::use_self)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::future_not_send)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::useless_let_if_seq)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::upper_case_acronyms)]
#![recursion_limit = "256"]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[doc = include_str!("docs/edge_description.md")]
#[doc = include_str!("docs/edge_struct_attributes.md")]
#[doc = include_str!("docs/edge_field_attributes.md")]
pub use surreal_derive::Edge;

#[doc = include_str!("docs/node_description.md")]
#[doc = include_str!("docs/node_struct_attributes.md")]
#[doc = include_str!("docs/node_field_attributes.md")]
pub use surreal_derive::Node;

#[doc = include_str!("docs/object_description.md")]
#[doc = include_str!("docs/object_struct_attributes.md")]
#[doc = include_str!("docs/object_field_attributes.md")]
pub use surreal_derive::Object;

/// #[doc = include_str!("docs/query_description.md")]
pub use surreal_derive::query;

/// #[doc = include_str!("docs/query_raw_description.md")]
pub use surreal_derive::query_raw;

pub use surreal_derive::block;

pub use surreal_derive::query_turbo;

pub use surreal_derive::transaction;

#[doc(hidden)]
// pub use serde;
pub use surreal_query_builder::*;

///
pub mod migrator {
    pub use migrator::*;
    pub use surreal_derive::embed_migrations;
}
