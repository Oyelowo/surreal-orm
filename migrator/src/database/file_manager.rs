use std::{
    env, fs,
    path::{Path, PathBuf},
};

use surreal_query_builder::DbResources;
use surrealdb::{Connection, Surreal};
use typed_builder::TypedBuilder;

use crate::*;

#[derive(Debug, Clone, Default)]
struct FileManager {}

///
impl FileManager {}

#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct MigrationConfig {
    // pub migration_name: String,
    pub mode: Mode,
    /// Default path is 'migrations' ralative to the nearest project root where
    /// cargo.toml is defined
    pub custom_path: Option<PathBuf>,
    // This is optional because, we cannot infer
    // the migration type from the migration directory
    // before initializing the migration directory.
    // Before init => None,
    // After init => Some(MigrationFlag)
    #[builder(default)]
    pub migration_flag: Option<MigrationFlag>,
}

impl MigrationConfig {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn from_cli(cli: Cli) -> MigrationResult<Self> {
    //     let rc = cli.runtime_config;
    //     let conf_init = Self::builder()
    //         .mode(rc.mode)
    //         .custom_path(cli.migrations_dir);
    //     let conf = conf_init.build();
    //     let res = conf_init
    //         .migration_flag(conf.detect_migration_type()?)
    //         .build();
    //     Ok(res)
    // }

    pub fn set_flag(mut self, flag: MigrationFlag) -> Self {
        self.migration_flag = Some(flag);
        self
    }

    pub fn migration_flag_checked(&self) -> MigrationResult<MigrationFlag> {
        self.migration_flag
            .clone()
            .ok_or(MigrationError::MigrationFlagNotSet)
    }

    pub fn set_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn make_strict(mut self) -> Self {
        self.mode = Mode::Strict;
        self
    }

    pub fn relax(mut self) -> Self {
        self.mode = Mode::Lax;
        self
    }

    /// Default path is 'migrations' ralative to the nearest project root where
    pub fn set_custom_path(mut self, custom_path: impl Into<PathBuf>) -> Self {
        let custom_path = custom_path.into();
        self.custom_path = Some(custom_path);
        self
    }

    pub fn one_way(&self) -> FileManagerUni {
        let mut config = self.clone();
        config.migration_flag = Some(MigrationFlag::OneWay);

        FileManagerUni::new(config)
    }

    pub fn two_way(&self) -> FileManagerBi {
        let mut config = self.clone();
        config.migration_flag = Some(MigrationFlag::TwoWay);

        FileManagerBi::new(config)
    }

    pub fn get_migration_dir_create_if_none(&self) -> MigrationResult<PathBuf> {
        self.resolve_migration_directory(true)
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

        log::info!("Migration dir path: {migration_dir_path_str}");

        let migrations = fs::read_dir(&migration_dir_path).map_err(|e| {
            MigrationError::IoError(format!(
                "Failed to read migration directory: {migration_dir_path_str}. Error: {e}"
            ))
        })?;

        log::info!("Migration dir path: {migration_dir_path_str}");

        let mut filenames = vec![];

        for migration in migrations {
            let migration = migration.map_err(|e| {
                MigrationError::IoError(format!(
                    "Failed to read migration directory: {migration_dir_path_str}. Error: {e}"
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
    pub fn get_two_way_migrations_sorted_desc(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<Vec<MigrationFileTwoWayPair>> {
        self.get_migrations_filenames(create_dir_if_not_exists)?
            .bidirectional_pair_meta_sorted_desc_checked(&self.resolve_migration_directory(false)?)
    }

    pub fn get_oneway_migrations(
        &self,
        create_dir_if_not_exists: bool,
    ) -> MigrationResult<Vec<MigrationFileOneWay>> {
        self.get_migrations_filenames(create_dir_if_not_exists)?
            .unidirectional_pair_meta(&self.resolve_migration_directory(false)?)
    }
}

#[derive(Debug, Clone)]
pub struct FileManagerUni(MigrationConfig);

impl FileManagerUni {
    pub(crate) fn new(file_manager: MigrationConfig) -> Self {
        Self(file_manager)
    }

    pub(crate) fn into_inner(&self) -> MigrationConfig {
        self.0.clone()
    }

    pub fn get_migrations(&self) -> MigrationResult<Vec<MigrationFileOneWay>> {
        self.0.get_oneway_migrations(false)
    }

    /// Generate migration directory if it does not exist
    pub async fn generate_migrations(
        &self,
        migration_name: impl Into<String>,
        codebase_resources: impl DbResources,
        prompter: impl Prompter,
    ) -> MigrationResult<()> {
        let migration_name = migration_name.into();
        let file_manager = self.into_inner();

        MigratorDatabase::generate_migrations(
            &migration_name,
            &file_manager,
            codebase_resources,
            prompter,
        )
        .await?;
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
            .map(|m| m.name().to_owned())
            .collect::<Vec<_>>();

        let migrations =
            MigrationRunner::list_migrations(db.clone(), migrations, status, mode).await?;

        Ok(migrations)
    }
}

#[derive(Debug, Clone)]
pub struct FileManagerBi(MigrationConfig);

impl FileManagerBi {
    pub(crate) fn new(file_manager: MigrationConfig) -> Self {
        Self(file_manager)
    }

    pub(crate) fn into_inner(&self) -> MigrationConfig {
        self.0.clone()
    }

    /// Get all migrations
    pub fn get_migrations(&self) -> MigrationResult<Vec<MigrationFileTwoWayPair>> {
        self.0
            .get_migrations_filenames(false)?
            .bidirectional_pair_meta_sorted_desc_checked(
                &self.0.resolve_migration_directory(false)?,
            )
    }

    /// Generate migration directory if it does not exist
    pub async fn generate_migrations(
        &self,
        migration_name: &String,
        codebase_resources: impl DbResources,
        prompter: impl Prompter,
    ) -> MigrationResult<()> {
        let migration_name = migration_name.into();
        let file_manager = self.0.clone();
        MigratorDatabase::generate_migrations(
            &migration_name,
            &file_manager,
            codebase_resources,
            prompter,
        )
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
        MigrationRunner::rollback_migrations(db, &self.into_inner(), rollback_options).await?;

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
