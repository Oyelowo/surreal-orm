/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{collections::HashSet, fmt::Display, fs};

use serde::{Deserialize, Serialize};
use surreal_orm::{Node, SurrealId};

use crate::*;

#[derive(Node, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "migration")]
pub struct Migration {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub timestamp: u64,
    // pub timestamp: Datetime<Utc>,
    // status: String,
}

impl Migration {}

// Warn when id field not included in a model

// Migratiions from migration directory
#[derive(Clone, Debug)]
pub struct MigrationTwoWay {
    pub id: MigrationFileName,
    pub name: String,
    pub timestamp: u64,
    pub up: String,
    pub down: String,
    // status: String,
}

#[derive(Clone, Debug)]
pub struct MigrationOneway {
    pub id: MigrationFileName,
    pub name: String,
    pub timestamp: u64,
    pub content: String, // status: String,
}

impl MigrationOneway {
    pub fn get_all_from_migrations_dir(mode: Mode) -> MigrationResult<Vec<Self>> {
        let migrations = fs::read_dir("migrations/");

        if migrations.is_err() {
            return Ok(vec![]);
        }

        let mut migrations_uni_meta = vec![];
        let mut unidirectional_basenames = vec![];

        for migration in migrations.expect("Problem reading migrations directory") {
            let migration = migration.expect("Problem reading migration");
            let path = migration.path();
            let path = path.to_str().ok_or(MigrationError::PathDoesNotExist)?;

            let migration_name = path.split('/').last().unwrap();
            let migration_up_name = migration_name.to_string();

            let filename: MigrationFileName = migration_up_name.clone().try_into()?;
            match filename {
                MigrationFileName::Up(_) | MigrationFileName::Down(_) => {
                    if mode == Mode::Strict {
                        return Err(MigrationError::InvalidMigrationFileNameForMode(
                            filename.to_string(),
                        ));
                    }
                }
                MigrationFileName::Unidirectional(_) => {
                    unidirectional_basenames.push(filename.basename());
                    let content = fs::read_to_string(path).unwrap();

                    let migration = MigrationOneway {
                        id: filename.clone(),
                        timestamp: filename.timestamp(),
                        name: filename.basename(),
                        content,
                    };

                    migrations_uni_meta.push(migration);
                }
            };
        }

        migrations_uni_meta.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(migrations_uni_meta)
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = match self {
            Self::Up => "up",
            Self::Down => "down",
        };
        write!(f, "{}", direction)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MigrationFlag {
    TwoWay,
    OneWay,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Strict,
    Relaxed,
}

impl MigrationTwoWay {
    pub fn get_all_from_migrations_dir(mode: Mode) -> MigrationResult<Vec<Self>> {
        let migrations = fs::read_dir("migrations/");

        if migrations.is_err() {
            return Ok(vec![]);
        }

        let mut migrations_bi_meta = vec![];

        let mut ups_basenames = vec![];
        let mut downs_basenames = vec![];

        for migration in migrations.expect("Problem reading migrations directory") {
            let migration = migration.expect("Problem reading migration");
            let path = migration.path();
            let parent_dir = path.parent().ok_or(MigrationError::PathDoesNotExist)?;
            let path = path.to_str().unwrap();
            let migration_name = path.split('/').last().unwrap();
            let migration_up_name = migration_name.to_string();

            let filename: MigrationFileName = migration_up_name.clone().try_into()?;
            match filename {
                MigrationFileName::Up(_) => {
                    ups_basenames.push(filename.basename());
                    let content_up = fs::read_to_string(path).unwrap();
                    let content_down =
                        fs::read_to_string(parent_dir.join(filename.to_down().to_string()))
                            .map_err(|_e| {
                                MigrationError::IoError(format!("Filename: {filename}"))
                            })?;

                    let migration = MigrationTwoWay {
                        id: filename.clone(),
                        timestamp: filename.timestamp(),
                        name: filename.basename(),
                        up: content_up,
                        down: content_down,
                    };

                    migrations_bi_meta.push(migration);
                }
                MigrationFileName::Down(_) => {
                    downs_basenames.push(filename.basename());
                }
                MigrationFileName::Unidirectional(_) => {
                    println!("Unidirectional migration found in bidirectional migration directory. This is not allowed");

                    if mode == Mode::Strict {
                        return Err(MigrationError::InvalidMigrationFileNameForMode(
                            filename.to_string(),
                        ));
                    }
                }
            };
        }

        // Validate
        // 1. Length of ups and downs should be equal
        if ups_basenames.len() != downs_basenames.len() {
            return Err(MigrationError::InvalidMigrationName(
                "Unequal number of up and down migrations.".into(),
            ));
        }

        let ups_basenames_as_set = ups_basenames.iter().collect::<HashSet<_>>();
        let downs_basenames_as_set = downs_basenames.iter().collect::<HashSet<_>>();

        let up_down_difference = ups_basenames_as_set
            .symmetric_difference(&downs_basenames_as_set)
            .cloned()
            .collect::<Vec<_>>();
        if !up_down_difference.is_empty() {
            return Err(MigrationError::InvalidUpsVsDownsMigrationFileCount(
                format!(
                    "The following files do not exist for both up and down. only for either: {}",
                    up_down_difference
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", "),
                ),
            ));
        }
        migrations_bi_meta.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(migrations_bi_meta)
    }

    pub fn get_migration_by_name(
        migration_name: impl Into<MigrationFileName>,
        mode: Mode,
    ) -> MigrationResult<Option<Self>> {
        let migration_name: MigrationFileName = migration_name.into();
        Ok(Self::get_all_from_migrations_dir(mode)?
            .into_iter()
            .find(|m| m.name == migration_name.to_string()))
    }
}

pub enum MigrationType {
    OneWay(String),
    TwoWay { up: String, down: String },
}
