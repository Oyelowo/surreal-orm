/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use surreal_query_builder::{Field, SurrealOrmError, Table};
use thiserror::Error;

use crate::MigrationFilename;

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Checksum mismaatch in {migration_name}. Expected checksum: {expected_checksum}. Actual checksum: {actual_checksum}")]
    ChecksumMismatch {
        migration_name: String,
        expected_checksum: String,
        actual_checksum: String,
    },

    #[error("Checksum not found in database for migration: {migration_name}")]
    NoChecksumInDb { migration_name: String },

    #[error("Migration already exists")]
    MigrationAlreadyExists,

    #[error("No migrations have yet been registered in the database")]
    NoMigrationsRegisteredYetInDb,

    #[error("Migration - {filename} does not exist")]
    MigrationDoesNotExist { filename: MigrationFilename },

    #[error("Migration with name: {0} not found in pending migrations. It has either been applied or does not exist.")]
    MigrationNotFoundFromPendingMigrations(MigrationFilename),

    #[error("Migration file - {0} - does not exist")]
    MigrationFileDoesNotExist(String),

    #[error("Rollback failed. Error: {0}")]
    RollbackFailed(String),

    #[error("Migration file name and database name mismatch. Migration file name: {migration_file_name}. Migration database name: {migration_db_name}")]
    MigrationFileVsDbNamesMismatch {
        migration_file_name: String,
        migration_db_name: String,
    },

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

    #[error("Migration file not found in path: {path} with error: {error}")]
    MigrationFilePathDoesNotExist { path: String, error: String },

    #[error("Invalid migration mode: {0}. It must be one of: {1}")]
    InvalidMigrationMode(String, String),

    #[error("Invalid migration flag: {0}. It must be one of {1}")]
    InvalidMigrationFlag(String, String),

    #[error("Migration flag not set")]
    MigrationFlagNotSet,

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

    #[error("Problem reading migration file. Error: {0}")]
    ProblemReadingMigrationFile(String),

    #[error("The field - {new_name} - on table - {table} - has an invalid old name - '{old_name}'. \
        It must have already been renamed previously or never existed before or wrongly spelt. \
         Also, make sure you are using the correct case for the field name. It should be one of these: {renamables}", )]
    InvalidOldFieldName {
        new_name: Field,
        table: Table,
        old_name: Field,
        renamables: String,
    },

    #[error("The field - {field_expected} - on table - {table} - does not exist. \
        It must have already been renamed previously or never existed before or wrongly spelt. \
         Also, make sure you are using the correct case for the field name. It should be one of these: {valid_fields}", )]
    FieldNameDoesNotExist {
        field_expected: Field,
        table: Table,
        valid_fields: String,
    },

    #[error("You are trying to rename the field - {field} - on table - {table} - to the same old field name - {field}. \
        This is likely not intentional. Use a different name for the new field")]
    RenamingToSameOldFieldDisallowed { field: Field, table: Table },

    #[error(
        "You are trying to use the same old field name - {field} - for new field name - {field}. \
        This is likely not intentional. Use a different name for the new field"
    )]
    FieldNameReused { field: Field, table: Table },

    #[error("Invalid DefineStatement: {0}")]
    InvalidDefineStatement(String),

    // TODO: Decide on how to handle suggestions in scenarios where there is a mismatch.
    #[error("Invalid migration state. The number of registered migrations in the database ({db_migration_count}) does not match the number of migration files in the migration directory ({local_dir_migration_count}). \
        This could be because you have deleted some migration files from the migration directory or you have deleted some migration records from the database. \
        If you have deleted some migration files from the migration directory, you can run the command 'cargo run -- prune' to delete all unapplied migrations from the database. \
        If you have deleted some migration records from the database, you can run the command 'cargo run -- reset' to delete all migration records from the database. \
        If you have deleted some migration files from the migration directory and some migration records from the database, you can run the command 'cargo run -- reset --prune' to delete all migration records from the database and all unapplied migrations from the database.")]
    InvalidMigrationState {
        db_migration_count: usize,
        local_dir_migration_count: usize,
    },

    #[error("Problem detecting migration mode. One way error: {one_way_error}. Two way error: {two_way_error}")]
    ProblemDetectingMigrationMode {
        one_way_error: String,
        two_way_error: String,
    },

    #[error("Ambiguous migration direction. Both or neither one-way and two-way file types found in migration directory. Use one or the other.
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
