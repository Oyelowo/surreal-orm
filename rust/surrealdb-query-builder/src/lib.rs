/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

//! This is it

mod errors;

pub mod functions;
mod helpers;
mod operators_macros;
/// This module contains the different types of statements that can be used to query the
/// database.
pub mod statements;
mod traits;
mod types;

pub use errors::*;
pub use helpers::*;
pub use traits::*;
pub use types::*;

pub use surrealdb::opt::RecordId;
pub use surrealdb::sql::json;
pub use surrealdb::sql::Value;

/// Exports everything from the prelude
pub mod prelude {
    use super::errors::*;
    use super::function::*;
    use super::helpers::*;
    use super::statements::*;
    use super::traits::*;
    use super::types::*;
}
