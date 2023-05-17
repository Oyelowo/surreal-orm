/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![deny(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]
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

//! This library includes augmented surrealdb types, custom types, statements, functions, operators, castings, and other utilities to
//! to make working with surrealdb a joy.

mod errors;

/// This module contains the different types of casts that can be used to cast values to
/// different types.
pub mod cast;
pub mod functions;
mod helpers;
mod operators_macros;
/// This module contains the different types of statements that can be used to query the
/// database.
pub mod statements;
mod traits;
mod types;
/// For compile time validations
pub mod validators;

pub use errors::*;
pub use helpers::*;
pub use traits::*;
pub use types::*;

pub use surrealdb::sql;

#[doc(hidden)]
pub mod internal_tools {
    pub use paste;
}
