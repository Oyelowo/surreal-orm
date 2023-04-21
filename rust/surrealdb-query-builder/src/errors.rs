/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use thiserror::Error;

/// The length of length of the returned list of items from the database
#[derive(Debug, Clone, Copy)]
pub struct ExpectedLength(u8);

impl Display for ExpectedLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u8> for ExpectedLength {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

/// The error type for the SurrealdbOrm
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum SurrealdbOrmError {
    #[error("there is an issue with one of your inputs while building the query.")]
    QueryBuilder(String),

    #[error("Expected at most {0}, but more returned")]
    TooManyItemsReturned(ExpectedLength),

    #[error("Problem runnning query. Check that there is no issue with your query.")]
    QueryRun(#[source] surrealdb::Error),

    #[error("Unable to parse data returned from the database. Check that all fields are complete and the types are able to deserialize surrealdb data types properly.")]
    Deserialization(#[source] surrealdb::Error),

    #[error("Invalid id. Problem deserializing string to surrealdb::sql::Thing. Check that the id is in the format 'table_name:id'")]
    InvalidId(#[source] surrealdb::Error),
}

pub type SurrealdbOrmResult<T> = std::result::Result<T, SurrealdbOrmError>;

