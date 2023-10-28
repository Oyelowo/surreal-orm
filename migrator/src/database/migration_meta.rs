/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    collections::HashSet,
    fmt::Display,
    fs::{self, File},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use surreal_orm::{Node, SurrealId, TableResources};

use crate::*;

#[derive(Node, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "migration", schemafull)]
pub struct Migration {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub timestamp: u64,
    // pub timestamp: Datetime<Utc>,
    // status: String,
}

impl Migration {}

impl TableResources for Migration {}

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
pub struct MigrationOneWay {
    pub id: MigrationFileName,
    pub name: String,
    pub timestamp: u64,
    pub content: String, // status: String,
}

impl MigrationOneWay {}

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

#[derive(Default, Debug, Clone, Copy)]
pub enum MigrationFlag {
    #[default]
    TwoWay,
    OneWay,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    #[default]
    Strict,
    Relaxed,
}

impl MigrationTwoWay {}

pub enum MigrationType {
    OneWay(String),
    TwoWay { up: String, down: String },
}

#[derive(Debug, Clone, Default)]
pub struct FileManager {
    // pub migration_name: String,
    pub mode: Mode,
    /// Default path is 'migrations' ralative to the nearest project root where
    /// cargo.toml is defined
    pub custom_path: Option<&'static str>,
    pub migration_flag: MigrationFlag,
}

impl FileManager {
    pub fn resolve_migration_directory(&self) -> MigrationResult<PathBuf> {
        let default_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
        let path = self
            .custom_path
            .map_or(default_path, |fp| Path::new(fp).to_owned());

        if path.exists() && path.is_dir() {
            Ok(path)
        } else {
            fs::create_dir(&path).map_err(|e| MigrationError::IoError(e.to_string()))?;
            Ok(path)
            // Err(MigrationError::InvalidMigrationDirectory(
            //     path.to_string_lossy().to_string(),
            // ))
        }
    }
    pub fn get_oneway_migrations(&self) -> MigrationResult<Vec<MigrationOneWay>> {
        let migration_directory = self.resolve_migration_directory()?;
        let migrations = fs::read_dir(migration_directory);

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
                    if self.mode == Mode::Strict {
                        println!("fafd");
                        return Err(MigrationError::InvalidMigrationFileNameForMode(
                            filename.to_string(),
                        ));
                    }
                }
                MigrationFileName::Unidirectional(_) => {
                    unidirectional_basenames.push(filename.basename());
                    let content = fs::read_to_string(path).unwrap();

                    let migration = MigrationOneWay {
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

    pub fn get_two_way_migrations(&self) -> MigrationResult<Vec<MigrationTwoWay>> {
        let migration_dir_path = self.resolve_migration_directory()?;
        println!("Migration dir path: {:?}", migration_dir_path.clone());
        let migrations = fs::read_dir(migration_dir_path.clone());
        println!("Migration dir path: {:?}", migration_dir_path);

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

                    if self.mode == Mode::Strict {
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

    pub fn get_two_way_migration_by_name(
        &self,
        migration_name: MigrationFileName,
    ) -> MigrationResult<Option<MigrationTwoWay>> {
        let migration_name: MigrationFileName = migration_name.into();
        Ok(self
            .get_two_way_migrations()?
            .into_iter()
            .find(|m| m.name == migration_name.to_string()))
    }
}
