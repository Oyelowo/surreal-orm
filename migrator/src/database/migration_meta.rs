/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::collections::HashSet;
use std::{fs::File, io::BufReader};

use sha2::{self, Digest, Sha256};
use std::convert::TryFrom;
use std::env;

use std::{
    collections::BTreeSet,
    fmt::Display,
    fs::{self},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use surreal_query_builder::statements::{define_field, define_table, DefineTableStatement};
use surreal_query_builder::{DbResources, Field, FieldType, Raw, Table, ToRaw};
use surrealdb::sql::Thing;
use surrealdb::{Connection, Surreal};

use crate::cli::Status;
use crate::*;

// #[derive(Node, Serialize, Deserialize, Clone, Debug)]
#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
// #[surreal_orm(table_name = "migration", schemafull)]
pub struct Migration {
    // pub id: SurrealId<Self, String>,
    pub id: Thing,
    pub name: String,
    pub timestamp: Timestamp,
    pub checksum_up: Checksum,
    pub checksum_down: Option<Checksum>,
    // pub timestamp: DateTime<Utc>,
    // status: String,
}

impl Ord for Migration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Migration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.timestamp.cmp(&other.timestamp))
    }
}

impl PartialEq for Migration {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for Migration {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(u64);

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Timestamp(timestamp) = self;
        write!(f, "{}", timestamp)
    }
}

impl From<Timestamp> for u64 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0
    }
}

impl Timestamp {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl From<u64> for Timestamp {
    fn from(timestamp: u64) -> Self {
        Self(timestamp)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Checksum(String);

impl From<String> for Checksum {
    fn from(checksum: String) -> Self {
        Self(checksum)
    }
}

impl Display for Checksum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Checksum(checksum) = self;
        write!(f, "{}", checksum)
    }
}

impl Checksum {
    pub fn generate_from_content(content: &FileContent) -> MigrationResult<Self> {
        let mut hasher = Sha256::new();
        hasher.update(content.to_string().as_bytes());

        let hash = hasher.finalize();
        Ok(format!("{:x}", hash).into())
    }

    pub fn generate_from_path(file_path: impl Into<std::path::PathBuf>) -> MigrationResult<Self> {
        let file_path = file_path.into();
        let file = File::open(&file_path).map_err(|e| {
            MigrationError::IoError(format!(
                "Failed to open migration file: {:?}. Error: {}",
                file_path, e
            ))
        })?;

        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();

        std::io::copy(&mut reader, &mut hasher).map_err(|e| {
            MigrationError::IoError(format!(
                "Failed to read migration file: {:?}. Error: {}",
                file_path, e
            ))
        })?;

        let hash = hasher.finalize();
        Ok(format!("{:x}", hash).into())
    }

    pub fn verify(
        &self,
        content: &FileContent,
        migration_filename: &MigrationFilename,
    ) -> MigrationResult<()> {
        let checksum = Checksum::generate_from_content(content)?;
        if checksum != *self {
            return Err(MigrationError::ChecksumMismatch {
                migration_name: migration_filename.to_string(),
                expected_checksum: self.to_string(),
                actual_checksum: checksum.to_string(),
            });
        }
        Ok(())
    }
}

pub struct MigrationSchema {
    pub id: Field,
    pub name: Field,
    pub timestamp: Field,
    pub checksum_up: Field,
    pub checksum_down: Field,
}

impl Migration {
    pub fn create_id(filename: &MigrationFilename) -> Thing {
        Thing {
            tb: Migration::table_name().to_string(),
            id: filename.to_string().into(),
        }
    }
    pub fn create_raw(
        filename: &MigrationFilename,
        checksum_up: &Checksum,
        checksum_down: Option<&Checksum>,
    ) -> Raw {
        let migration::Schema {
            id: _id_field,
            name: name_field,
            timestamp: timestamp_field,
            checksum_up: checksum_up_field,
            checksum_down: checksum_down_field,
        } = Migration::schema();

        let record_id = Self::create_id(&filename);
        let name = filename.to_string();
        let timestamp = filename.timestamp().into_inner();
        let checksum_up = checksum_up.to_string();
        let checksum_down = checksum_down
            .map(|c| c.to_string())
            .unwrap_or("null".into());

        Raw::new(format!(
            "CREATE {record_id} SET {name_field}='{name}', {timestamp_field}={timestamp}, \
        {checksum_up_field}='{checksum_up}', {checksum_down_field}='{checksum_down}';"
        ))
    }

    pub fn delete_raw(filename: &MigrationFilename) -> Raw {
        let _migration_table = Migration::table_name();
        let migration::Schema { .. } = Migration::schema();
        let record_id = Self::create_id(filename);
        Raw::new(format!("DELETE {record_id};"))
    }

    pub fn schema() -> MigrationSchema {
        MigrationSchema {
            id: "id".into(),
            name: Field::new("name"),
            timestamp: Field::new("timestamp"),
            checksum_up: Field::new("checksum_up"),
            checksum_down: Field::new("checksum_down"),
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
            checksum_up,
            checksum_down,
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

        let checksum_up = define_field(checksum_up)
            .type_(FieldType::String)
            .on_table(Migration::table_name())
            .to_raw();

        let checksum_down = define_field(checksum_down)
            .type_(FieldType::String)
            .on_table(Migration::table_name())
            .to_raw();

        vec![id, name, timestamp, checksum_up, checksum_down]
    }
}

pub mod migration {
    pub type Schema = super::MigrationSchema;
}

impl Migration {}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileContent(String);

impl Display for FileContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FileContent(content) = self;
        write!(f, "{}", content)
    }
}

impl FileContent {
    pub fn empty() -> Self {
        Self("".into())
    }

    pub fn as_checksum(&self) -> MigrationResult<Checksum> {
        Checksum::generate_from_content(self)
    }

    pub fn as_checksum_from_path(
        &self,
        file_path: impl Into<PathBuf>,
    ) -> MigrationResult<Checksum> {
        Checksum::generate_from_path(file_path)
    }

    pub fn from_file(file_path: impl Into<PathBuf>) -> MigrationResult<Self> {
        let file_path = file_path.into();
        let content = fs::read_to_string(&file_path)
            .map_err(|e| MigrationError::IoError(format!("Error: {}", e)))?;
        Ok(Self(content))
    }
}

impl From<String> for FileContent {
    fn from(content: String) -> Self {
        Self(content)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MigrationTwoWay {
    pub name: MigrationFilename,
    pub up: FileContent,
    pub down: FileContent,
    // status: String,
}

impl TryFrom<MigrationTwoWay> for Migration {
    type Error = MigrationError;

    fn try_from(migration: MigrationTwoWay) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Migration::create_id(&migration.name.to_up()),
            name: migration.name.to_up().to_string(),
            timestamp: migration.name.timestamp(),
            checksum_up: migration.up.as_checksum()?,
            checksum_down: Some(migration.down.as_checksum()?),
        })
    }
}

#[derive(Clone, Debug)]
pub struct MigrationOneWay {
    pub name: MigrationFilename,
    pub content: FileContent, // status: String,
}

impl MigrationOneWay {}

impl TryFrom<MigrationOneWay> for Migration {
    type Error = MigrationError;

    fn try_from(migration: MigrationOneWay) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Migration::create_id(&migration.name),
            name: migration.name.to_string(),
            timestamp: migration.name.timestamp(),
            checksum_up: Checksum::generate_from_content(&migration.content)?.into(),
            checksum_down: None,
        })
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

    pub fn is_strict(&self) -> bool {
        matches!(self, Self::Strict)
    }

    pub fn is_relaxed(&self) -> bool {
        matches!(self, Self::Relaxed)
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
    pub custom_path: Option<String>,
    pub(crate) migration_flag: MigrationFlag,
}

#[derive(Debug, Clone)]
pub struct MigrationConfig(FileManager);

impl Default for MigrationConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl MigrationConfig {
    pub fn new() -> Self {
        Self(FileManager::default())
    }

    pub fn mode(&self, mode: Mode) -> Self {
        Self(self.0.mode(mode))
    }

    pub fn make_strict(&self) -> Self {
        Self(self.0.mode(Mode::Strict))
    }

    pub fn relax(&self) -> Self {
        Self(self.0.mode(Mode::Relaxed))
    }

    /// Default path is 'migrations' ralative to the nearest project root where
    pub fn custom_path(&self, custom_path: impl Into<String>) -> Self {
        let custom_path = custom_path.into();
        Self(self.0.custom_path(custom_path))
    }

    pub fn detect_migration_type(&self) -> MigrationResult<MigrationFlag> {
        self.0.detect_migration_type()
    }

    pub fn one_way(&self) -> OneWayGetter {
        let fm = FileManager {
            migration_flag: MigrationFlag::OneWay,
            ..self.0.clone()
        };
        OneWayGetter::new(fm)
    }

    pub fn two_way(&self) -> TwoWayGetter {
        let fm = FileManager {
            migration_flag: MigrationFlag::TwoWay,
            ..self.0.clone()
        };
        TwoWayGetter::new(fm)
    }

    pub fn get_migration_dir(&self) -> MigrationResult<PathBuf> {
        self.0.get_migration_dir()
    }
}

#[derive(Debug, Clone)]
pub struct OneWayGetter(FileManager);

impl OneWayGetter {
    pub(crate) fn new(file_manager: FileManager) -> Self {
        Self(file_manager)
    }

    pub fn get_migrations(&self) -> MigrationResult<Vec<MigrationOneWay>> {
        self.0.get_oneway_migrations(false)
    }

    /// Generate migration directory if it does not exist
    pub async fn generate_migrations(
        &self,
        migration_name: impl Into<String>,
        codebase_resources: impl DbResources,
    ) -> MigrationResult<()> {
        let migration_name = migration_name.into();
        let file_manager = self.0.clone();

        MigratorDatabase::generate_migrations(migration_name, &file_manager, codebase_resources)
            .await
            .expect("Failed to generate migrations");
        Ok(())
    }

    /// Runs migrations at runtime against a database
    /// Make sure the migration directory exists when running migrations
    pub async fn run_pending_migrations(
        &self,
        db: Surreal<impl Connection>,
        update_strategy: UpdateStrategy,
    ) -> MigrationResult<()> {
        let migrations = self.get_migrations()?;
        MigrationRunner::apply_pending_migrations(db, migrations, update_strategy).await?;

        Ok(())
    }

    pub async fn run_embedded_pending_migrations(
        &self,
        db: Surreal<impl Connection>,
        one_way_embedded_migrations: EmbeddedMigrationsOneWay,
        update_strategy: UpdateStrategy,
    ) -> MigrationResult<()> {
        let migrations = one_way_embedded_migrations.to_migrations_one_way()?;
        MigrationRunner::apply_pending_migrations(db, migrations, update_strategy).await?;

        Ok(())
    }

    /// List all migrations
    pub async fn list_migrations(
        &self,
        db: Surreal<impl Connection>,
        status: Status,
        strictness: StrictNessLevel,
    ) -> MigrationResult<Vec<MigrationFilename>> {
        let migrations = self
            .get_migrations()?
            .into_iter()
            .map(|m| m.name)
            .collect::<Vec<_>>();

        let migrations =
            MigrationRunner::list_migrations(db.clone(), migrations, status, strictness).await?;

        Ok(migrations)
    }
}

#[derive(Debug, Clone)]
pub struct TwoWayGetter(FileManager);

impl TwoWayGetter {
    pub(crate) fn new(file_manager: FileManager) -> Self {
        Self(file_manager)
    }

    /// Get all migrations
    pub fn get_migrations(&self) -> MigrationResult<Vec<MigrationTwoWay>> {
        self.0.get_two_way_migrations(false)
    }

    /// Generate migration directory if it does not exist
    pub async fn generate_migrations(
        &self,
        migration_name: impl Into<String>,
        codebase_resources: impl DbResources,
    ) -> MigrationResult<()> {
        let migration_name = migration_name.into();
        let file_manager = self.0.clone();
        MigratorDatabase::generate_migrations(migration_name, &file_manager, codebase_resources)
            .await
    }

    /// Make sure the migration directory exists when running migrations
    pub async fn run_up_pending_migrations(
        &self,
        db: Surreal<impl Connection>,
        update_strategy: UpdateStrategy,
    ) -> MigrationResult<()> {
        let migrations = self.get_migrations()?;
        MigrationRunner::apply_pending_migrations(db.clone(), migrations, update_strategy).await?;

        Ok(())
    }

    /// For running embedded migrations
    pub async fn run_up_embedded_pending_migrations(
        &self,
        db: Surreal<impl Connection>,
        two_way_embedded_migrations: EmbeddedMigrationsTwoWay,
        update_strategy: UpdateStrategy,
    ) -> MigrationResult<()> {
        let migrations = two_way_embedded_migrations.to_migrations_two_way()?;
        MigrationRunner::apply_pending_migrations(db.clone(), migrations, update_strategy).await?;

        Ok(())
    }

    /// Rollback migration using various strategies
    pub async fn run_down_migrations(
        &self,
        db: Surreal<impl Connection>,
        rollback_options: RollbackOptions,
    ) -> MigrationResult<()> {
        // let _migrations = self.get_migrations()?;
        MigrationRunner::rollback_migrations(db, &self.0, rollback_options).await?;

        Ok(())
    }

    /// List all migrations
    pub async fn list_migrations(
        &self,
        db: Surreal<impl Connection>,
        status: Status,
        strictness: StrictNessLevel,
    ) -> MigrationResult<Vec<MigrationFilename>> {
        let migrations = self
            .get_migrations()?
            .into_iter()
            .map(|m| m.name)
            .collect::<Vec<_>>();

        let migrations =
            MigrationRunner::list_migrations(db.clone(), migrations, status, strictness).await?;

        Ok(migrations)
    }
}

///
impl FileManager {
    pub fn mode(&self, mode: Mode) -> Self {
        Self {
            mode,
            ..self.clone()
        }
    }

    pub fn custom_path(&self, custom_path: String) -> Self {
        Self {
            custom_path: Some(custom_path),
            ..self.clone()
        }
    }

    pub fn migration_flag(&self, migration_flag: MigrationFlag) -> Self {
        Self {
            migration_flag,
            ..self.clone()
        }
    }

    pub fn one_way(&mut self) -> OneWayGetter {
        self.migration_flag = MigrationFlag::OneWay;
        OneWayGetter::new(self.clone())
    }

    pub fn two_way(&mut self) -> TwoWayGetter {
        self.migration_flag = MigrationFlag::TwoWay;
        TwoWayGetter::new(self.clone())
    }

    pub fn detect_migration_type(&self) -> MigrationResult<MigrationFlag> {
        let filenames = self.get_migrations_filenames(false)?;
        let oneway = filenames.unidirectional();
        let twoway = filenames.bidirectional();

        match (oneway.is_empty(), twoway.is_empty()) {
            (false, true) => Ok(MigrationFlag::OneWay),
            (true, false) => Ok(MigrationFlag::TwoWay),
            (false, false) | (true, true) => {
                return Err(MigrationError::AmbiguousMigrationDirection {
                    one_way_filecount: oneway.len(),
                    two_way_filecount: twoway.len(),
                });
            }
        }
    }

    pub fn get_migration_dir(&self) -> MigrationResult<PathBuf> {
        self.resolve_migration_directory(false)
    }

    pub(crate) fn resolve_migration_directory(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<PathBuf> {
        let cargo_toml_directory =
            env::var("CARGO_MANIFEST_DIR").map_err(|_| MigrationError::PathDoesNotExist)?;
        let cargo_manifests_dir = Path::new(&cargo_toml_directory);
        let default_path = cargo_manifests_dir.join("migrations");
        let path = self
            .custom_path
            .as_ref()
            .map_or(default_path, |fp| cargo_manifests_dir.join(Path::new(&fp)));

        if path.exists() && path.is_dir() {
            Ok(path)
        } else {
            if create_dir_if_not_exists {
                fs::create_dir(&path).map_err(|e| MigrationError::IoError(e.to_string()))?;
                return Ok(path);
            }
            Err(MigrationError::MigrationDirectoryDoesNotExist(
                path.to_string_lossy().to_string(),
            ))
        }
    }

    pub fn get_oneway_migrations(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<Vec<MigrationOneWay>> {
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
            let path_str = path.to_str().ok_or(MigrationError::PathDoesNotExist)?;

            let migration_name = path.file_name().expect("Problem reading migration name");
            let migration_up_name = migration_name.to_string_lossy().to_string();

            let filename: MigrationFilename = migration_up_name.clone().try_into()?;
            match filename {
                MigrationFilename::Up(_) | MigrationFilename::Down(_) => {
                    if self.mode.is_strict() {
                        return Err(MigrationError::InvalidMigrationFileNameForMode(
                            filename.to_string(),
                        ));
                    }
                }
                MigrationFilename::Unidirectional(_) => {
                    unidirectional_basenames.push(filename.basename());
                    let content = fs::read_to_string(path_str)
                        .map_err(|e| {
                            MigrationError::IoError(format!(
                                "Problem reading migration file: Error: {e}"
                            ))
                        })?
                        .into();

                    let migration = MigrationOneWay {
                        name: filename,
                        content,
                    };

                    migrations_uni_meta.push(migration);
                }
            };
        }

        migrations_uni_meta.sort_by(|a, b| a.name.timestamp().cmp(&b.name.timestamp()));
        Ok(migrations_uni_meta)
    }

    pub fn get_migrations_filenames(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<MigrationFilenames> {
        let migration_dir_path = self.resolve_migration_directory(create_dir_if_not_exists)?;
        log::info!("Migration dir path: {:?}", migration_dir_path.clone());
        let migrations = fs::read_dir(migration_dir_path.clone());
        log::info!("Migration dir path: {:?}", migration_dir_path);

        let mut filenames = vec![];

        for migration in migrations.expect("Problem reading migrations directory") {
            let migration = migration.expect("Problem reading migration");
            let path = migration.path();
            let migration_name = path
                .components()
                .last()
                .expect("Problem reading migration name")
                .as_os_str()
                .to_string_lossy()
                .to_string();

            let filename: MigrationFilename = migration_name.clone().try_into()?;
            filenames.push(filename);
        }

        filenames.sort_by(|a, b| a.cmp(b));
        Ok(filenames.into())
    }

    // Validate
    pub fn get_two_way_migrations(
        &self,
        create_dir_if_not_exists: bool,
        // strictness: StrictNessLevel,
    ) -> MigrationResult<MigrationFilenames> {
        let migration_dir_path = self.resolve_migration_directory(create_dir_if_not_exists)?;
        log::info!("Migration dir path: {:?}", migration_dir_path.clone());
        let migrations = fs::read_dir(migration_dir_path.clone())
            .map_err(|e| MigrationError::MigrationDirectoryDoesNotExist(e.to_string()))?;
        log::info!("Migration dir path: {:?}", migration_dir_path);

        let mut migrations_bi_meta = HashSet::new();
        let mut ups_basenames = vec![];
        let mut downs_basenames = vec![];

        for migration in migrations {
            let migration = migration
                .map_err(|e| MigrationError::ProblemReadingMigrationFile(e.to_string()))?;
            let path = migration.path();
            let parent_dir = path.parent().ok_or(MigrationError::PathDoesNotExist)?;
            let migration_name = path
                .components()
                .last()
                .expect("Problem reading migration name")
                .as_os_str()
                .to_string_lossy()
                .to_string();

            let filename: MigrationFilename = migration_name.clone().try_into()?;
            let get_content = |filename: &MigrationFilename| -> MigrationResult<FileContent> {
                let content = fs::read_to_string(parent_dir.join(filename.to_string()))
                    .map_err(|_e| {
                        MigrationError::IoError(format!("Filename: {filename} does not exist"))
                    })?
                    .into();
                Ok(content)
            };

            match filename {
                MigrationFilename::Up(_) | MigrationFilename::Down(_) => {
                    match filename {
                        MigrationFilename::Up(_) => {
                            ups_basenames.push(filename.basename());
                        }
                        MigrationFilename::Down(_) => {
                            downs_basenames.push(filename.basename());
                        }
                        _ => {}
                    }

                    let content_up = get_content(&filename.to_up())?;
                    let content_down = get_content(&filename.to_down())?;

                    let migration = MigrationTwoWay {
                        name: filename.clone(),
                        up: content_up,
                        down: content_down,
                    };

                    migrations_bi_meta.insert(migration);
                }
                MigrationFilename::Unidirectional(_) => {
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
        if self.mode.is_strict() {
            if ups_basenames.len() != downs_basenames.len() {
                return Err(MigrationError::InvalidUpsVsDownsMigrationFileCount(
                    "Unequal number of up and down migrations.".into(),
                ));
            }

            let ups_basenames_as_set = ups_basenames.iter().collect::<BTreeSet<_>>();
            let downs_basenames_as_set = downs_basenames.iter().collect::<BTreeSet<_>>();

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
        }

        let mut migrations_bi_meta = migrations_bi_meta.into_iter().collect::<Vec<_>>();
        migrations_bi_meta.push(MigrationTwoWay {
            name: "20231216000000_create_migration_table.up.surql"
                .to_string()
                .try_into()
                .expect("xrer"),
            up: FileContent("upananan".into()),
            down: FileContent("downwnw".into()),
        });

        migrations_bi_meta.sort_by(|a, b| a.name.timestamp().cmp(&b.name.timestamp()));

        log::info!("Successfully read {} migrations", migrations_bi_meta.len());

        Ok(migrations_bi_meta)
    }
}
