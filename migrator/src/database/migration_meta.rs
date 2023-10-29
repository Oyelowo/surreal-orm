/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::{
    collections::HashSet,
    fmt::Display,
    fs::{self, File},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use surreal_query_builder::statements::{define_field, define_table, DefineTableStatement};
use surreal_query_builder::{FieldType, Model, Node, Raw, SurrealId, Table, TableResources, ToRaw};
use surrealdb::sql::Thing;

use crate::*;

// #[derive(Node, Serialize, Deserialize, Clone, Debug)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
// #[surreal_orm(table_name = "migration", schemafull)]
pub struct Migration {
    // pub id: SurrealId<Self, String>,
    id: Thing,
    pub name: String,
    pub timestamp: u64,
    // pub timestamp: Datetime<Utc>,
    // status: String,
}

pub struct MigrationSchema {
    pub id: &'static str,
    pub name: &'static str,
    pub timestamp: &'static str,
}

impl Migration {
    pub fn create_id(id_part: MigrationFileName) -> Thing {
        Thing {
            tb: Migration::table_name().to_string(),
            id: id_part.to_string().into(),
        }
    }
    // pub fn create_raw(m: Self) -> Raw {
    pub fn create_raw(id_part: MigrationFileName, name: String, timestamp: u64) -> Raw {
        let migration_table = Migration::table_name();
        let migration::Schema {
            id,
            name,
            timestamp,
        } = Migration::schema();

        let name_field = name;
        let timestamp_field = timestamp;
        // let record_id = Thing {
        //     tb: migration_table.clone().to_string(),
        //     id: m.id.to_string().into(),
        // };
        // let name = m.name.clone();
        // let timestamp = m.timestamp;
        // let content = m.content.clone();
        let record_id = Self::create_id(id_part);
        Raw::new(format!(
            "CREATE {record_id} SET {name_field}={name}, {timestamp_field}={timestamp}"
        ))
    }

    pub fn schema() -> MigrationSchema {
        MigrationSchema {
            id: "id",
            name: "name",
            timestamp: "timestamp",
        }
    }

    pub fn table_name() -> surreal_query_builder::Table {
        Table::new("migration")
    }

    pub fn define_table() -> DefineTableStatement {
        define_table(Migration::table_name()).schemafull()
    }

    pub fn define_fields() -> Vec<Raw> {
        let migration::Schema {
            id,
            name,
            timestamp,
        } = Migration::schema();
        let id = define_field(id)
            .type_(FieldType::Record(vec![Migration::table_name()]))
            .on_table(Migration::table_name())
            .to_raw();

        let name = define_field(name)
            .type_(FieldType::String)
            .on_table(Migration::table_name())
            .to_raw();

        let timestamp = define_field(timestamp)
            .type_(FieldType::Int)
            .on_table(Migration::table_name())
            .to_raw();

        vec![id, name, timestamp]
    }
}

pub mod migration {
    pub type Schema = super::MigrationSchema;
}

impl Migration {}

struct RenameCreator;

// impl Node for Migration {
//     type NonNullUpdater = Migration;
//
//     type Aliases;
//
//     #[doc(hidden)]
//     type TableNameChecker;
//
//     fn aliases() -> Self::Aliases {
//         todo!()
//     }
//
//     fn get_table_name() -> Table {
//         todo!()
//     }
//
//     fn with(clause: impl Into<NodeClause>) -> Self::Schema {
//         todo!()
//     }
//
//     fn get_fields_relations_aliased() -> Vec<Alias> {
//         todo!()
//     }
// }
// impl Model for Migration {
//     type Id = Thing;
//
//     type NonNullUpdater = Migration;
//
//     type StructRenamedCreator = RenameCreator;
//
//     fn table_name() -> surreal_query_builder::Table {
//         todo!()
//     }
//
//     fn get_id(self) -> Self::Id {
//         todo!()
//     }
//
//     fn get_id_as_thing(&self) -> surrealdb::sql::Thing {
//         todo!()
//     }
//
//     fn get_serializable_fields() -> Vec<surreal_query_builder::Field> {
//         todo!()
//     }
//
//     fn get_linked_fields() -> Vec<surreal_query_builder::Field> {
//         todo!()
//     }
//
//     fn get_link_one_fields() -> Vec<surreal_query_builder::Field> {
//         todo!()
//     }
//
//     fn get_link_self_fields() -> Vec<surreal_query_builder::Field> {
//         todo!()
//     }
//
//     fn get_link_one_and_self_fields() -> Vec<surreal_query_builder::Field> {
//         todo!()
//     }
//
//     fn get_link_many_fields() -> Vec<surreal_query_builder::Field> {
//         todo!()
//     }
//
//     fn define_table() -> surreal_query_builder::Raw {
//         todo!()
//     }
//
//     fn define_fields() -> Vec<surreal_query_builder::Raw> {
//         todo!()
//     }
//
//     fn get_field_meta() -> Vec<surreal_query_builder::FieldMetadata> {
//         todo!()
//     }
// }
//
// impl TableResources for Migration {
//     fn events_definitions() -> Vec<surreal_query_builder::Raw> {
//         vec![]
//     }
//
//     fn indexes_definitions() -> Vec<surreal_query_builder::Raw> {
//         vec![]
//     }
//
//     fn fields_definitions() -> Vec<surreal_query_builder::Raw> {
//         Self::define_fields()
//     }
//
//     fn table_definition() -> surreal_query_builder::Raw {
//         Self::define_table()
//     }
// }

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
pub struct EmbeddedMigrationTwoWay {
    pub id: &'static str,
    pub name: &'static str,
    pub timestamp: u64,
    pub up: &'static str,
    pub down: &'static str,
    // status: String,
}

#[derive(Clone, Debug)]
pub struct MigrationOneWay {
    pub id: MigrationFileName,
    pub name: String,
    pub timestamp: u64,
    pub content: String, // status: String,
}

#[derive(Clone, Debug)]
pub struct EmbeddedMigrationOneWay {
    pub id: &'static str,
    pub name: &'static str,
    pub timestamp: u64,
    pub content: &'static str, // status: String,
}

impl MigrationOneWay {}

// pub struct EmbeddedMigrationsOneWay<const N: usize> {
#[allow(missing_copy_implementations)]
pub struct EmbeddedMigrationsOneWay {
    // migrations: &'static [MigrationOneWay],
    pub migrations: &'static [EmbeddedMigrationOneWay],
}

// impl<const N: usize> EmbeddedMigrationsOneWay<N> {
impl EmbeddedMigrationsOneWay {
    // pub const fn new<const N: usize>(migrations: [EmbeddedMigrationOneWay; N]) -> Self {
    pub const fn new(migrations: &'static [EmbeddedMigrationOneWay]) -> Self {
        Self { migrations }
    }
    // pub fn new(migrations: &'static [MigrationOneWay]) -> Self {
    //     Self { migrations }
    // }
}
// fn erer() {
//     let one_way = EmbeddedMigrationOneWay {
//         id: "sample",
//         name: "".into(),
//         timestamp: 0,
//         content: "".into(),
//     };
//     let migrations = &[one_way];
//     let xx = EmbeddedMigrationsOneWay::new(migrations);
//     // let one_way = &[one_way];
// }

#[allow(missing_copy_implementations)]
pub struct EmbeddedMigrationsTwoWay {
    pub migrations: &'static [EmbeddedMigrationTwoWay],
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

#[derive(Default, Debug, Clone, Copy)]
pub enum MigrationFlag {
    #[default]
    TwoWay,
    OneWay,
}

impl Display for MigrationFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flag = match self {
            Self::TwoWay => "two_way",
            Self::OneWay => "one_way",
        };
        write!(f, "{}", flag)
    }
}

impl MigrationFlag {
    pub fn options() -> Vec<String> {
        vec![Self::TwoWay.to_string(), Self::OneWay.to_string()]
    }
}

impl TryFrom<String> for MigrationFlag {
    type Error = MigrationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "two_way" => Ok(Self::TwoWay),
            "one_way" => Ok(Self::OneWay),
            _ => Err(MigrationError::InvalidMigrationFlag(
                value,
                Self::options().join(", "),
            )),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    #[default]
    Strict,
    Relaxed,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            Self::Strict => "strict",
            Self::Relaxed => "relaxed",
        };
        write!(f, "{}", mode)
    }
}

impl Mode {
    pub fn options() -> Vec<String> {
        vec![Self::Strict.to_string(), Self::Relaxed.to_string()]
    }
}

impl TryFrom<String> for Mode {
    type Error = MigrationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "strict" => Ok(Self::Strict),
            "relaxed" => Ok(Self::Relaxed),
            _ => Err(MigrationError::InvalidMigrationMode(
                value,
                Self::options().join(", "),
            )),
        }
    }
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
    /// 
    /// ```rust
    /// custom_path: Some("../custom-path".to_string())
    /// ```
    pub custom_path: Option<String>,
    pub migration_flag: MigrationFlag,
    // pub crea
}

impl FileManager {
    pub fn resolve_migration_directory(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<PathBuf> {
        // let cargo_manigests_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let cargo_toml_directory =
            env::var("CARGO_MANIFEST_DIR").map_err(|_| MigrationError::PathDoesNotExist)?;
        // let cargo_manigests_dir = Path::new(env::var("CARGO_MANIFEST_DIR").unwrap());
        let cargo_manifests_dir = Path::new(&cargo_toml_directory);
        println!("cargo_manigests_dir: {:?}", cargo_manifests_dir);
        let default_path = cargo_manifests_dir.join("migrations");
        let path = self.custom_path.as_ref().map_or(default_path, |fp| {
            cargo_manifests_dir.join(Path::new(&fp).to_owned())
        });

        if path.exists() && path.is_dir() {
            Ok(path)
        } else {
            if create_dir_if_not_exists {
                fs::create_dir(&path).map_err(|e| MigrationError::IoError(e.to_string()))?;
                return Ok(path);
            }
            Err(MigrationError::InvalidMigrationDirectory(
                path.to_string_lossy().to_string(),
            ))
        }
    }

    pub fn migration_directory_from_given_path(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<PathBuf> {
        let cargo_toml_directory =
            env::var("CARGO_MANIFEST_DIR").map_err(|_| MigrationError::PathDoesNotExist)?;
        let cargo_manifest_path = Path::new(&cargo_toml_directory);
        let migrations_path = self.custom_path.as_ref().map(Path::new);
        let x = Self::resolve_migrations_directory(
            cargo_manifest_path,
            migrations_path,
            create_dir_if_not_exists,
        );
        // panic!("x: {:?}", x);
        x
    }

    fn resolve_migrations_directory(
        cargo_manifest_dir: &Path,
        relative_path_to_migrations: Option<&Path>,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<PathBuf> {
        let result = match relative_path_to_migrations {
            Some(dir) => cargo_manifest_dir.join(dir),
            None => {
                let src_dir = cargo_manifest_dir.join("src");
                Self::search_for_migrations_directory(&src_dir)
                    .ok_or(MigrationError::MigrationDirectoriesNotExist)?
            }
        };

        if result.canonicalize().is_ok() {
            return Ok(result);
        } else {
            if create_dir_if_not_exists {
                return Ok(result);
            }
            // fs::create_dir(&result).map_err(|e| MigrationError::IoError(e.to_string()))?;
            return Err(MigrationError::InvalidMigrationDirectory(
                result.to_string_lossy().to_string(),
            ));
            // return Err(MigrationError::InvalidMigrationDirectory(
            //     result.to_string_lossy().to_string(),
            // ));
        }
        // result.canonicalize().map_err(|_| {
        //     MigrationError::InvalidMigrationDirectory(result.to_string_lossy().to_string())
        // })
    }

    pub fn search_for_migrations_directory(path: &Path) -> Option<PathBuf> {
        let migration_path = path.join("migrations");
        if migration_path.is_dir() {
            Some(migration_path)
        } else {
            path.parent()
                .and_then(Self::search_for_migrations_directory)
        }
    }

    pub fn get_oneway_migrations(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<Vec<MigrationOneWay>> {
        // let migration_directory = self.resolve_migration_directory()?;
        // let migration_directory =
        //     self.migration_directory_from_given_path(create_dir_if_not_exists)?;
        let migration_directory = self.resolve_migration_directory(create_dir_if_not_exists)?;
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

    pub fn get_two_way_migrations(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<Vec<MigrationTwoWay>> {
        // let migration_dir_path =
        //     self.migration_directory_from_given_path(create_dir_if_not_exists)?;
        let migration_dir_path = self.resolve_migration_directory(create_dir_if_not_exists)?;
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
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<Option<MigrationTwoWay>> {
        let migration_name: MigrationFileName = migration_name.into();
        Ok(self
            .get_two_way_migrations(create_dir_if_not_exists)?
            .into_iter()
            .find(|m| m.name == migration_name.to_string()))
    }
}
