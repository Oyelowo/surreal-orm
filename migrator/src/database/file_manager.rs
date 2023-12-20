use std::{
    env, fs,
    path::{Path, PathBuf},
};

use surreal_query_builder::DbResources;
use surrealdb::{Connection, Surreal};

use crate::*;

#[derive(Debug, Clone, Default)]
pub struct FileManager {
    // pub migration_name: String,
    pub mode: Mode,
    /// Default path is 'migrations' ralative to the nearest project root where
    /// cargo.toml is defined
    pub custom_path: Option<String>,
    pub(crate) migration_flag: MigrationFlag,
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

    pub fn is_first_migration(&self) -> MigrationResult<bool> {
        Ok(self.get_migrations_filenames(false)?.all().is_empty())
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

    pub fn get_migrations_filenames(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<MigrationFilenames> {
        let migration_dir_path = self.resolve_migration_directory(create_dir_if_not_exists)?;
        let migration_dir_path_str = migration_dir_path.to_string_lossy().to_string();
        log::info!("Migration dir path: {}", &migration_dir_path_str);
        let migrations = fs::read_dir(&migration_dir_path).map_err(|e| {
            MigrationError::IoError(format!(
                "Failed to read migration directory: {}. Error: {}",
                &migration_dir_path_str, e
            ))
        })?;
        log::info!("Migration dir path: {}", migration_dir_path_str);

        let mut filenames = vec![];

        for migration in migrations {
            let migration = migration.map_err(|e| {
                MigrationError::IoError(format!(
                    "Failed to read migration directory: {}. Error: {}",
                    migration_dir_path_str, e
                ))
            })?;
            let path = migration.path();
            let filename: MigrationFilename = path
                .components()
                .last()
                .expect("Problem reading migration name")
                .as_os_str()
                .to_string_lossy()
                .to_string()
                .try_into()?;

            filenames.push(filename);
        }

        filenames.sort_by(|a, b| a.cmp(b));
        Ok(filenames.into())
    }

    // Validate
    pub fn get_two_way_migrations(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<Vec<MigrationFileBiPair>> {
        self.get_migrations_filenames(create_dir_if_not_exists)?
            .bidirectional_pair_meta_checked(&self.resolve_migration_directory(false)?)
    }

    pub fn get_oneway_migrations(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<Vec<MigrationFileUni>> {
        self.get_migrations_filenames(create_dir_if_not_exists)?
            .unidirectional_pair_meta(&self.resolve_migration_directory(false)?)
    }
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

    pub fn into_inner(self) -> FileManager {
        self.0
    }

    pub fn set_mode(&self, mode: Mode) -> Self {
        Self(self.0.mode(mode))
    }

    pub fn mode(&self) -> Mode {
        self.0.mode
    }

    pub fn make_strict(&self) -> Self {
        Self(self.0.mode(Mode::Strict))
    }

    pub fn relax(&self) -> Self {
        Self(self.0.mode(Mode::Lax))
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

    pub fn get_migration_dir_create_if_none(&self) -> MigrationResult<PathBuf> {
        self.0.resolve_migration_directory(true)
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

    pub fn get_migrations(&self) -> MigrationResult<Vec<MigrationFileUni>> {
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

        MigratorDatabase::generate_migrations(&migration_name, &file_manager, codebase_resources)
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
        mode: Mode,
    ) -> MigrationResult<Vec<MigrationFilename>> {
        let migrations = self
            .get_migrations()?
            .into_iter()
            .map(|m| m.name())
            .collect::<Vec<_>>();

        let migrations =
            MigrationRunner::list_migrations(db.clone(), migrations, status, mode).await?;

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
    pub fn get_migrations(&self) -> MigrationResult<Vec<MigrationFileBiPair>> {
        self.0
            .get_migrations_filenames(false)?
            .bidirectional_pair_meta_checked(&self.0.resolve_migration_directory(false)?)
    }

    /// Generate migration directory if it does not exist
    pub async fn generate_migrations(
        &self,
        migration_name: &String,
        codebase_resources: impl DbResources,
    ) -> MigrationResult<()> {
        let migration_name = migration_name.into();
        let file_manager = self.0.clone();
        MigratorDatabase::generate_migrations(&migration_name, &file_manager, codebase_resources)
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
        mode: Mode,
    ) -> MigrationResult<Vec<MigrationFilename>> {
        let migrations = self
            .get_migrations()?
            .into_iter()
            .map(|m| m.up.name)
            .collect::<Vec<_>>();

        let migrations =
            MigrationRunner::list_migrations(db.clone(), migrations, status, mode).await?;

        Ok(migrations)
    }
}
