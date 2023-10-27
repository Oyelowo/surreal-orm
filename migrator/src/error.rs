/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use surreal_orm::SurrealOrmError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Migration already exists")]
    MigrationAlreadyExists,
    #[error("Migration does not exist")]
    MigrationDoesNotExist,
    #[error("Migration not registered")]
    MigrationNotRegistered,
    #[error("Migration not unregistered")]
    MigrationNotUnregistered,
    #[error("Direction does not exist")]
    DirectionDoesNotExist,
    #[error("Migration name does not exist")]
    MigrationNameDoesNotExist,
    #[error("Invalid migration name. Error: {0}")]
    InvalidMigrationName(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("Invalid migration file name for mode: {0}")]
    InvalidMigrationFileNameForMode(String),

    #[error("No migration directories")]
    MigrationDirectoriesNotExist,

    // invalid migration directory
    #[error("Invalid migration directory: {0}")]
    InvalidMigrationDirectory(String),

    #[error("Invalid migration state. Migration up queries empty")]
    MigrationUpQueriesEmpty,

    #[error("Invalid migration state. Migration down queries empty")]
    MigrationDownQueriesEmpty,

    #[error("Invalid path")]
    PathDoesNotExist,

    #[error("The field - {new_name} - on table - {table} - has an invalid old name - '{old_name}'. \
        It must have already been renamed previously or never existed before or wrongly spelt. \
         Also, make sure you are using the correct case for the field name. It should be one of these: {renamables}", )]
    InvalidOldFieldName {
        new_name: String,
        table: String,
        old_name: String,
        renamables: String,
    },

    #[error("Invalid DefineStatement: {0}")]
    InvalidDefineStatement(String),

    #[error("Invalid migration file count: {0}")]
    InvalidUpsVsDownsMigrationFileCount(String),

    #[error(transparent)]
    ProblemWithQuery(#[from] SurrealOrmError),

    #[error(transparent)]
    InvalidRegex(#[from] regex::Error),

    #[error("Invalid migration file name: {0}")]
    IoError(String),

    // #[error(transparent)]
    // IoError(#[from] std::io::Error),
    #[error(transparent)]
    PromptError(#[from] inquire::error::InquireError),

    #[error(transparent)]
    DbError(#[from] surrealdb::Error),
}

pub type MigrationResult<T> = Result<T, MigrationError>;
