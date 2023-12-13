/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use surreal_query_builder::SurrealOrmError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Migration already exists")]
    MigrationAlreadyExists,

    #[error("Migration does not exist")]
    MigrationDoesNotExist,

    #[error("Migration file(s) does not exist")]
    MigrationFileDoesNotExist,

    #[error("Migration not registered")]
    MigrationNotRegistered,

    #[error("Migration not unregistered")]
    MigrationNotUnregistered,

    #[error("Direction does not exist")]
    DirectionDoesNotExist,

    #[error("Migration name does not exist")]
    MigrationNameDoesNotExist,

    #[error("Invalid migration name. {0}. Make sure it's in the format - <timestamp>_<migration_name>.<up|down|>.surql if two way or <timestamp>_<migration_name.surql if one way")]
    InvalidMigrationName(String),

    #[error("Migration path not found")]
    MigrationPathNotFound,

    #[error("Invalid migration mode: {0}. It must be one of {1}")]
    InvalidMigrationMode(String, String),

    #[error("Invalid migration flag: {0}. It must be one of {1}")]
    InvalidMigrationFlag(String, String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("Invalid migration file name for mode: {0}")]
    InvalidMigrationFileNameForMode(String),

    #[error("No migration directories")]
    MigrationDirectoriesNotExist,

    #[error("Invalid migration directory: {0}")]
    InvalidMigrationDirectory(String),

    #[error("The migration directory ({0}) provided does not exist. You may need to explicitly provide a migration directory path argument. \
        If you are using the default migration directory, make sure you are running the command from the root of your project.
    Check --help for more information")]
    MigrationDirectoryDoesNotExist(String),

    #[error("Invalid migration directory. It must not be empty")]
    MigrationDirectoryEmpty,

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

    #[error("Problem detecting migration mode. One way error: {one_way_error}. Two way error: {two_way_error}")]
    ProblemDetectingMigrationMode {
        one_way_error: String,
        two_way_error: String,
    },

    #[error("Ambiguous migration direction. Both oneway and two file types found in migration directory. Use one or the other.
Up only should not be sufficed with 'up.surql' or 'down.surql' but up and down migrations can be.
There are {one_way_filecount} up only files and {two_way_filecount} up and down files")]
    AmbiguousMigrationDirection {
        one_way_filecount: usize,
        two_way_filecount: usize,
    },

    #[error("There {migration_count} unapplied migration files. Apply the Unapplied migration files to the database instance \
        using the command 'cargo run -- up' to apply it/them or delete all unapplied migrations using 'cargo run -- prune'.")]
    UnappliedMigrationExists { migration_count: usize },

    #[error("Invalid migration flag detection: {0}")]
    MigrationFlagDetectionError(String),

    #[error("Invalid migration file count: {0}")]
    InvalidUpsVsDownsMigrationFileCount(String),

    #[error(transparent)]
    ProblemWithQuery(#[from] SurrealOrmError),

    #[error(transparent)]
    InvalidRegex(#[from] regex::Error),

    #[error("Invalid migration file name: {0}")]
    IoError(String),

    #[error(transparent)]
    PromptError(#[from] inquire::error::InquireError),

    #[error(transparent)]
    DbError(#[from] surrealdb::Error),
}

pub type MigrationResult<T> = Result<T, MigrationError>;
