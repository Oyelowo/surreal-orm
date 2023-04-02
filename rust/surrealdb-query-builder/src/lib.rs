/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![allow(unused_imports)]

use std::fmt::Display;
use std::ops::Deref;

// pub(crate) mod binding;
pub(crate) mod errors;
pub mod functions;
pub(crate) mod helpers;
mod operators_macros;
mod statements;
pub mod traits;
pub mod types;

pub mod query {
    // TODO: remove this, Here just for testing purpose
    pub use super::functions::array::concat;
    // pub use super::functions::array::concatx as concat_;
    // pub use super::raw_statements::{Raw, To};
    pub use super::statements::*;
}

pub use surrealdb::sql::json;
pub use surrealdb::sql::Value;

pub mod sql {
    // pub use super::clause::*;
    // pub use super::field::*;
    // pub use super::field_updater::*;
    // pub use super::param::*;
    // pub use super::raw_statements::*;
    // pub use super::sql_components::*;
    // pub use super::sql_traits::*;
    // // pub use super::types::binding::*;
}

pub mod utils {
    // pub use super::filter::cond;
}
pub mod prelude {
    use super::statements;
}

// pub use field::Param;
// pub use field::ParamsExtractor;
pub use surrealdb::opt::RecordId;
