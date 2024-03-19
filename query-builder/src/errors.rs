#![allow(missing_docs)]

/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
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

type IdString = String;
type TableString = String;

/// The error type for the SurrealOrm
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum SurrealOrmError {
    #[error("There is an issue with one of your inputs while building the query. {0}")]
    QueryBuilder(String),

    #[error("Expected at most {0}, but more returned")]
    TooManyItemsReturned(ExpectedLength),

    #[error("Problem runnning query. Check that there is no issue with your query. {0}")]
    QueryRun(#[source] surrealdb::Error),

    #[error("Unable to parse data returned from the database. Check that all fields are complete and the types are able to deserialize surrealdb data types properly. {0}")]
    Deserialization(#[source] surrealdb::Error),

    #[error("Invalid id. Problem deserializing string to surrealdb::sql::Thing. Check that the id is in the format 'table:id'. {0}")]
    InvalidId(#[source] surrealdb::Error),

    #[error("The id - {0} provided does not belong to the table {1}. Please ensure that the id provided is for the table you are trying to fetch from.")]
    IdBelongsToAnotherTable(IdString, TableString),

    #[error("The following fields could not be fetched as they are not linked to a foreign table: {0}. Please ensure that all fields provided are of types 'link_self', 'link_one' or 'link_many' to allow fetching of linked values from other tables.")]
    FieldsUnfetchableNotARecordLink(String),

    #[error("No record returned from {0}. Check that all fields in this table are selected.")]
    RecordNotFound(String),

    #[error("Invalid subquery. {0}")]
    InvalidSubquery(String),
}

pub type SurrealOrmResult<T> = std::result::Result<T, SurrealOrmError>;
