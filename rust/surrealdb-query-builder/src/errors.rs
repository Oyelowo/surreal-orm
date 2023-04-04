/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use thiserror::Error;

struct ExpectedLength(u8);

impl From<u8> for ExpectedLength {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Error, Debug)]
pub enum SurrealdbOrmError {
    #[error("there is an issue with one of your inputs while building the query.")]
    QueryBuilder(String),
    // #[error("the id - `{0}` - you have provided is invalid or belongs to another table. Surrealdb Is should be in format: <table_name:column>")]
    // #[error("The id provided within the graph in the query belongs to another table. Please, make sure you use the right table when building a graph e.g student::with(student:1).writes->book(book:2). Within these compound ids. for student, it should be in the format - `student:<id>` and `book:<id>`")]
    // WrongIdUsedInQuery(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
    #[error("Expected at most {0}, but more returned")]
    TooManyItemsReturned(ExpectedLength),

    #[error("Problem runnning query. Check that there is no issue with your query.")]
    QueryRun(#[source] surrealdb::Error),

    #[error("Unable to parse data returned from the database. Check that all fields are complete and the types are able to deserialize surrealdb data types properly.")]
    Deserialization(#[source] surrealdb::Error),
}

pub type Result<T> = std::result::Result<T, SurrealdbOrmError>;
