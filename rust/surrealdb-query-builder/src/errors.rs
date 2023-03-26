/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SurrealdbOrmError {
    #[error("the id - `{0}` - you have provided is invalid or belongs to another table. Surrealdb Is should be in format: <table_name:column>")]
    InvalidId(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}
