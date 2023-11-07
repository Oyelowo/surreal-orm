// 24 Aug, 2023. Not yet supported in stable error[E0554]: `#![feature]` may not be used on the stable release channel
// #![feature(rustdoc_missing_doc_code_examples)]
/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
#![deny(clippy::all)]
#![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
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

/// Contains math constants, all the casting functions and future.
mod data_model;
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

pub use data_model::*;
pub use errors::*;
pub use helpers::*;
pub use statements::select::CanOrder;
pub use statements::utils::*;
pub use traits::*;
pub use types::*;

pub use serde;
pub use surrealdb::sql;

#[doc(hidden)]
pub mod internal_tools {
    pub use paste::paste;
}
