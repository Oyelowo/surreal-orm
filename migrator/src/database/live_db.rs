use std::{collections::BTreeSet, path::PathBuf};

use std::ops::Deref;
use surreal_query_builder::{statements::*, *};
use surrealdb::{Connection, Surreal};

use crate::cli::Status;
use crate::{
    cli::RollbackStrategy, FileContent, FileManager, Migration, MigrationError, MigrationFilename,
    MigrationOneWay, MigrationResult, MigrationSchema, MigrationTwoWay,
};
use crate::{MigrationConfig, MigrationFilenames, UpdateStrategy};

// pub struct MigrationRunner<C: Connection> {
pub struct MigrationRunner {
    // db: Surreal<C>,
    // file_manager: FileManager,
}

impl From<MigrationTwoWay> for MigrationOneWay {
    fn from(m: MigrationTwoWay) -> Self {
        Self {
            name: m.name,
            content: m.up,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MigrationFile {
    OneWay(MigrationOneWay),
    TwoWay(MigrationTwoWay),
}

#[derive(Debug, Clone)]
pub struct PendingMigrationFile(MigrationFile);

impl Deref for PendingMigrationFile {
    type Target = MigrationFile;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PendingMigrationFile> for MigrationFile {
    fn from(m: PendingMigrationFile) -> Self {
        m.0
    }
}

impl From<MigrationFile> for PendingMigrationFile {
    fn from(m: MigrationFile) -> Self {
        Self(m)
    }
}

impl MigrationFile {
    pub fn name(&self) -> &MigrationFilename {
        match self {
            Self::OneWay(m) => &m.name,
            Self::TwoWay(m) => &m.name,
        }
    }

    pub fn up_content(&self) -> &FileContent {
        match self {
            Self::OneWay(m) => &m.content,
            Self::TwoWay(m) => &m.up,
        }
    }

    pub fn down_content(&self) -> Option<&FileContent> {
        match self {
            Self::OneWay(_) => None,
            Self::TwoWay(m) => Some(&m.down),
        }
    }
}

impl From<MigrationOneWay> for MigrationFile {
    fn from(m: MigrationOneWay) -> Self {
        Self::OneWay(m)
    }
}

impl From<MigrationTwoWay> for MigrationFile {
    fn from(m: MigrationTwoWay) -> Self {
        Self::TwoWay(m)
    }
}

pub struct RollbackOptions {
    pub rollback_strategy: RollbackStrategy,
    pub strictness: StrictNessLevel,
    pub prune_files_after_rollback: bool,
}

impl Default for RollbackOptions {
    fn default() -> Self {
        Self {
            rollback_strategy: RollbackStrategy::Previous,
            strictness: StrictNessLevel::Strict,
            prune_files_after_rollback: false,
        }
    }
}

impl RollbackOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_strict(&self) -> bool {
        self.strictness == StrictNessLevel::Strict
    }

    pub fn strategy(mut self, rollback_strategy: RollbackStrategy) -> Self {
        self.rollback_strategy = rollback_strategy;
        self
    }

    pub fn strictness(mut self, strictness: StrictNessLevel) -> Self {
        self.strictness = strictness;
        self
    }

    pub fn prune_files_after_rollback(mut self, prune_files_after_rollback: bool) -> Self {
        self.prune_files_after_rollback = prune_files_after_rollback;
        self
    }
}

impl MigrationRunner {
    /// Only two way migrations support rollback
    pub async fn rollback_migrations(
        db: Surreal<impl Connection>,
        fm: &FileManager,
        rollback_options: RollbackOptions,
    ) -> MigrationResult<()> {
        let RollbackOptions {
            ref rollback_strategy,
            ref strictness,
            prune_files_after_rollback,
        } = rollback_options;

        log::info!("Rolling back migration");

        let all_migrations_from_dir = fm.get_two_way_migrations(false)?;
        let latest_migration = Self::get_latest_migration(db.clone())
            .await?
            .ok_or(MigrationError::MigrationDoesNotExist)?;

        let migrations_from_dir = all_migrations_from_dir
            .iter()
            .find(|m| m.name == latest_migration.name.clone().try_into().unwrap())
            .ok_or(MigrationError::MigrationFileDoesNotExist)?;

        if rollback_options.is_strict() {
            let pending_migrations =
                Self::get_pending_migrations(all_migrations_from_dir.clone(), db.clone()).await?;

            let is_valid_rollback_state =
                pending_migrations.is_empty() || pending_migrations.len() % 2 == 0;

            if !is_valid_rollback_state && rollback_options.prune_files_after_rollback {
                return Err(MigrationError::UnappliedMigrationExists {
                    migration_count: pending_migrations.len(),
                });
            }
        }

        let (queries_to_run, file_paths) = match rollback_strategy {
            RollbackStrategy::Previous => Self::generate_rollback_queries_and_filepaths(
                fm,
                vec![migrations_from_dir.clone()],
                vec![latest_migration],
                strictness,
            )?,
            RollbackStrategy::Number(count) => {
                let migrations_from_db = select(All)
                    .from(Migration::table_name())
                    .order_by(Migration::schema().timestamp.desc())
                    .limit(*count)
                    .return_many::<Migration>(db.clone())
                    .await?;

                let latest_migration = migrations_from_db
                    .first()
                    .ok_or(MigrationError::MigrationDoesNotExist)?;

                let migrations_to_rollback = all_migrations_from_dir
                    .into_iter()
                    .filter(|m| {
                        let is_before_db_latest_migration =
                            m.name.timestamp() < latest_migration.timestamp;

                        let is_latest = m.name
                            == latest_migration
                                .name
                                .clone()
                                .try_into()
                                .expect("Invalid migration name");

                        is_before_db_latest_migration || is_latest
                    })
                    .take(*count as usize)
                    .collect::<Vec<_>>();

                Self::generate_rollback_queries_and_filepaths(
                    fm,
                    migrations_to_rollback,
                    migrations_from_db,
                    strictness,
                )?
            }
            RollbackStrategy::Till(file_cursor) => {
                let MigrationSchema {
                    timestamp, name, ..
                } = &Migration::schema();

                let timestamp_value = file_cursor.timestamp().into_inner();

                let migrations_from_db = select(All)
                    .from(Migration::table_name())
                    .where_(cond(timestamp.gt(timestamp_value)).or(
                        cond(timestamp.eq(timestamp_value)).and(name.eq(file_cursor.to_string())),
                    ))
                    .order_by(timestamp.desc())
                    .return_many::<Migration>(db.clone())
                    .await?;

                let latest_migration = migrations_from_db
                    .first()
                    .ok_or(MigrationError::MigrationDoesNotExist)?;

                let migrations_files_to_rollback = all_migrations_from_dir
                    .clone()
                    .into_iter()
                    .filter(|m| {
                        let is_latest = m.name
                            == latest_migration
                                .name
                                .clone()
                                .try_into()
                                .expect("Invalid migration name");
                        let is_before_db_latest_migration =
                            m.name.timestamp() < latest_migration.timestamp;

                        let is_after_file_cursor = m.name.timestamp() > file_cursor.timestamp();
                        let is_file_cursor = m.name == *file_cursor;

                        (is_before_db_latest_migration || is_latest)
                            && (is_after_file_cursor || is_file_cursor)
                    })
                    .collect::<Vec<_>>();

                Self::generate_rollback_queries_and_filepaths(
                    fm,
                    migrations_files_to_rollback,
                    migrations_from_db,
                    strictness,
                )?
            }
        };

        begin_transaction()
            .query(queries_to_run)
            .commit_transaction()
            .run(db)
            .await?;

        if prune_files_after_rollback {
            for file_path in &file_paths {
                let file_path_str = file_path.to_string_lossy();

                log::info!("Deleting file: {}", file_path_str);

                std::fs::remove_file(file_path).map_err(|e| {
                    MigrationError::IoError(format!(
                        "Failed to delete migration file: {}. Error: {}",
                        file_path_str, e
                    ))
                })?;

                log::info!("Deleted file: {}", file_path_str);
            }
        }

        log::info!("Migration rolled back");

        Ok(())
    }

    async fn get_latest_migration(
        db: Surreal<impl Connection>,
    ) -> SurrealOrmResult<Option<Migration>> {
        select(All)
            .from(Migration::table_name())
            .order_by(Migration::schema().timestamp.desc())
            .limit(1)
            .return_one::<Migration>(db.clone())
            .await
    }

    pub async fn get_pending_migrations(
        all_migrations: Vec<impl Into<MigrationFile>>,
        db: Surreal<impl Connection>,
    ) -> SurrealOrmResult<Vec<PendingMigrationFile>> {
        let latest_migration = Self::get_latest_migration(db.clone()).await?;

        let pending_migrations = all_migrations
            .into_iter()
            .map(|m| {
                let m: MigrationFile = m.into();
                PendingMigrationFile::from(m)
            })
            .filter(|m| {
                latest_migration.as_ref().map_or(true, |latest_migration| {
                    m.name().timestamp() > latest_migration.timestamp
                })
            })
            .map(PendingMigrationFile::from)
            .collect::<Vec<_>>();

        Ok(pending_migrations)
    }

    pub async fn get_pending_migration_filenames(
        db: Surreal<impl Connection>,
        mig_config: &MigrationConfig,
    ) -> MigrationResult<MigrationFilenames> {
        let latest_migration = Self::get_latest_migration(db.clone()).await?;

        let pending_migrations = mig_config
            .clone()
            .into_inner()
            .get_migrations_filenames(false)?
            .all()
            .into_iter()
            .filter(|filename| {
                latest_migration.as_ref().map_or(true, |latest_migration| {
                    filename.timestamp() > latest_migration.timestamp
                })
            })
            .collect::<Vec<_>>();

        Ok(pending_migrations.into())
    }

    pub async fn delete_unapplied_migration_files(
        db: Surreal<impl Connection>,
        mig_config: &MigrationConfig,
    ) -> MigrationResult<()> {
        let dir = mig_config.get_migration_dir()?;

        let pending_migration_filenames =
            Self::get_pending_migration_filenames(db.clone(), mig_config).await?;

        let pending_migrations_paths = pending_migration_filenames
            .iter()
            .map(|filename| dir.join(filename.to_string()))
            .collect::<Vec<_>>();

        log::info!(
            "Deleting {} unapplied migration file(s)",
            pending_migrations_paths.len()
        );

        for mig_path in pending_migrations_paths {
            let mig_path_str = mig_path.to_string_lossy();
            log::info!("Deleting file: {}", &mig_path_str);
            std::fs::remove_file(&mig_path).map_err(|e| {
                MigrationError::IoError(format!(
                    "Failed to delete migration file: {}. Error: {}",
                    &mig_path_str, e
                ))
            })?;
            log::info!("Deleted file: {}", &mig_path_str);
        }

        Ok(())
    }

    fn generate_rollback_queries_and_filepaths(
        fm: &FileManager,
        migrations_to_rollback: Vec<MigrationTwoWay>,
        migrations_from_db: Vec<Migration>,
        strictness: &StrictNessLevel,
    ) -> MigrationResult<(Raw, Vec<PathBuf>)> {
        if *strictness == StrictNessLevel::Strict {
            for (m_from_file, m_from_db) in
                migrations_to_rollback.iter().zip(migrations_from_db.iter())
            {
                let db_mig_name = m_from_db
                    .name
                    .clone()
                    .try_into()
                    .expect("Invalid migration name");

                m_from_db
                    .checksum_up
                    .verify(&m_from_file.up, &m_from_file.name)?;

                m_from_db
                    .clone()
                    .checksum_down
                    .ok_or(MigrationError::NoChecksumInDb {
                        migration_name: m_from_db.name.clone(),
                    })?
                    .verify(&m_from_file.down, &m_from_file.name)?;

                if m_from_file.name != db_mig_name {
                    return Err(MigrationError::MigrationFileDoesNotExist);
                }
            }
        }

        let rollback_queries = migrations_to_rollback
            .clone()
            .into_iter()
            .map(|m| m.down.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let rollbacked_migration_deletion_queries = migrations_to_rollback
            .iter()
            .map(|m| Migration::delete_raw(&m.name).build())
            .collect::<Vec<_>>()
            .join("\n");

        let all = format!(
            "{}\n{}",
            rollback_queries, rollbacked_migration_deletion_queries
        );

        let file_paths = migrations_to_rollback
            .iter()
            .map(|m| {
                fm.get_migration_dir().map(|d| {
                    vec![
                        d.join(m.name.to_up().to_string()),
                        d.join(m.name.to_down().to_string()),
                    ]
                })
            })
            .collect::<MigrationResult<Vec<_>>>();

        Ok((
            Raw::new(all),
            file_paths?.iter().flatten().cloned().collect::<Vec<_>>(),
        ))
    }

    async fn run_up_pending_migrations(
        db: Surreal<impl Connection>,
        filtered_pending_migrations: Vec<PendingMigrationFile>,
    ) -> MigrationResult<()> {
        let mut migration_queries: Vec<FileContent> = vec![];
        let mut mark_queries_registered_queries: Vec<Raw> = vec![];

        for mf in filtered_pending_migrations.into_iter() {
            match mf.into() {
                MigrationFile::OneWay(m) => {
                    let created_registered_mig =
                        Migration::create_raw(&m.name, &m.content.as_checksum()?, None);

                    migration_queries.push(m.content);
                    mark_queries_registered_queries.push(created_registered_mig);
                }
                MigrationFile::TwoWay(m) => {
                    let created_registered_mig = Migration::create_raw(
                        &m.name,
                        &m.up.as_checksum()?,
                        Some(&m.down.as_checksum()?),
                    );

                    migration_queries.push(m.up);
                    mark_queries_registered_queries.push(created_registered_mig);
                }
            }
        }

        let migration_queries_str = migration_queries
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");

        // Create queries to mark migrations as applied
        let mark_queries_registered_queries_str = mark_queries_registered_queries
            .iter()
            .map(|q| q.build())
            .collect::<Vec<_>>()
            .join("\n");

        log::info!("Running {} migrations", migration_queries.len());
        log::info!(
            "Marking {} query(ies) as registered",
            mark_queries_registered_queries.len()
        );

        // Join migrations with mark queries
        let all = format!(
            "{}\n{}",
            migration_queries_str, mark_queries_registered_queries_str
        );

        if all.trim().is_empty() {
            log::info!("No new migrations to apply");
        } else {
            begin_transaction()
                .query(Raw::new(all))
                .commit_transaction()
                .run(db.clone())
                .await?;

            log::info!("Applied {} migrations", migration_queries.len());
        }

        Ok(())
    }

    pub async fn apply_pending_migrations(
        db: Surreal<impl Connection>,
        all_migrations: Vec<impl Into<MigrationFile>>,
        update_strategy: UpdateStrategy,
    ) -> MigrationResult<()> {
        log::info!("Running pending migrations");

        let filtered_pending_migrations = match update_strategy {
            UpdateStrategy::Latest => {
                Self::get_pending_migrations(all_migrations, db.clone()).await?
            }
            UpdateStrategy::Number(count) => {
                Self::get_pending_migrations(all_migrations, db.clone())
                    .await?
                    .into_iter()
                    .take(count as usize)
                    .collect::<Vec<_>>()
            }
            UpdateStrategy::Till(mig_filename) => {
                let pending_migs = Self::get_pending_migrations(all_migrations, db.clone()).await?;

                let mut migration_found = false;
                let mut filtered_migs: Vec<PendingMigrationFile> = vec![];

                for mig in pending_migs {
                    filtered_migs.push(mig.clone());
                    if *mig.name() == mig_filename {
                        migration_found = true;
                        break;
                    }
                }

                if !migration_found {
                    return Err(MigrationError::MigrationDoesNotExist);
                }

                filtered_migs
            }
        };

        Self::run_up_pending_migrations(db.clone(), filtered_pending_migrations).await
    }

    pub(crate) async fn list_migrations(
        db: Surreal<impl Connection>,
        migrations_local_dir: Vec<MigrationFilename>,
        status: Status,
        strictness: StrictNessLevel,
    ) -> MigrationResult<Vec<MigrationFilename>> {
        let migrations = match status {
            Status::All => {
                let mut migrations = migrations_local_dir;
                migrations.sort_by_key(|name| name.timestamp());
                migrations.into_iter().map(|name| name).collect::<Vec<_>>()
            }
            Status::Pending => {
                let latest_applied_migration = select(All)
                    .from(Migration::table_name())
                    .order_by(Migration::schema().timestamp.desc())
                    .limit(1)
                    .return_one::<Migration>(db.clone())
                    .await?;

                let mut migrations = migrations_local_dir
                    .into_iter()
                    .filter(|name| {
                        latest_applied_migration
                            .as_ref()
                            .map_or(true, |latest_migration| {
                                name.timestamp() > latest_migration.timestamp
                            })
                    })
                    .collect::<Vec<_>>();
                migrations.sort_by_key(|name| name.timestamp());
                migrations
            }
            Status::Applied => {
                let db_migs = select(All)
                    .from(Migration::table_name())
                    .order_by(Migration::schema().timestamp.asc())
                    .return_many::<Migration>(db.clone())
                    .await?;
                Self::_get_db_migrations_meta_from_mig_files(
                    migrations_local_dir,
                    db_migs,
                    strictness,
                )?
            }
        };
        Ok(migrations)
    }

    fn _get_db_migrations_meta_from_mig_files(
        local_migrations: Vec<MigrationFilename>,
        db_migs: Vec<Migration>,
        strictness: StrictNessLevel,
    ) -> MigrationResult<Vec<MigrationFilename>> {
        let db_migs: BTreeSet<MigrationFilename> = db_migs
            .into_iter()
            .map(|dbm| dbm.name.try_into().expect("Invalid migration file name."))
            .collect();

        let filtered_local_migrations = local_migrations
            .into_iter()
            .filter(|name| db_migs.contains(&name))
            .collect::<Vec<_>>();

        // TODO:: Check that the files are contiguous?
        if strictness == StrictNessLevel::Strict {
            if filtered_local_migrations.len() != db_migs.len() {
                return Err(MigrationError::InvalidMigrationState {
                    db_migration_count: db_migs.len(),
                    local_dir_migration_count: filtered_local_migrations.len(),
                });
            }
        }

        Ok(filtered_local_migrations)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrictNessLevel {
    Strict,
    Lax,
}

impl StrictNessLevel {
    pub fn is_strict(&self) -> bool {
        *self == Self::Strict
    }

    pub fn is_lax(&self) -> bool {
        *self == Self::Lax
    }
}

impl From<bool> for StrictNessLevel {
    fn from(value: bool) -> Self {
        if value {
            Self::Strict
        } else {
            Self::Lax
        }
    }
}
