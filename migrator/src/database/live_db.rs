use std::{collections::BTreeSet, ops::Deref, path::PathBuf};

use surreal_query_builder::{statements::*, *};
use surrealdb::{Connection, Surreal};

use crate::{
    cli::Status, EmbeddedMigrationOneWay, FileContent, FileManager, Migration, MigrationError,
    MigrationFilename, MigrationOneWay, MigrationResult, MigrationSchema, MigrationTwoWay,
};

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
pub struct MigrationFileMeta {
    name: MigrationFilename,
    content: FileContent,
    // checksum_up: Checksum,
    // checksum_down: Option<Checksum>,
}

pub struct PendingMigration(MigrationFileMeta);

impl Deref for PendingMigration {
    type Target = MigrationFileMeta;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<MigrationFileMeta> for PendingMigration {
    fn from(m: MigrationFileMeta) -> Self {
        Self(m)
    }
}

impl From<MigrationTwoWay> for PendingMigration {
    fn from(m: MigrationTwoWay) -> Self {
        MigrationFileMeta {
            name: m.name,
            content: m.up,
            // checksum_up: m.checksum_up,
            // checksum_down: Some(m.checksum_down),
        }
        .into()
    }
}

impl From<MigrationOneWay> for PendingMigration {
    fn from(m: MigrationOneWay) -> Self {
        MigrationFileMeta {
            name: m.name,
            content: m.content,
        }
        .into()
    }
}

impl From<EmbeddedMigrationOneWay> for PendingMigration {
    fn from(m: EmbeddedMigrationOneWay) -> Self {
        MigrationFileMeta {
            name: m.name.to_string().try_into().expect("Invalid migration id"),
            content: m.content.to_string().into(),
        }
        .into()
    }
}

pub enum UpdateStrategy {
    // Default
    // cargo run -- up
    // cargo run -- up -latest
    // cargo run -- up -l
    Latest,
    // cargo run -- up -number 34
    // cargo run -- up -n 34
    Number(u32),
    // cargo run -- up -till 234y3498349304
    // cargo run -- up -t 234y3498349304
    Till(MigrationFilename),
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

pub enum RollbackStrategy {
    // Default
    // cargo run -- down
    // cargo run -- down -n 1
    Previous,
    // cargo run -- down -number 34
    // cargo run -- down -n 34
    Number(u32),
    // cargo run -- down -till 234y3498349304
    // cargo run -- down -t 234y3498349304
    Till(MigrationFilename),
}

impl MigrationRunner {
    /// Only two way migrations support rollback
    pub async fn rollback_migrations(
        db: Surreal<impl Connection>,
        fm: &FileManager,
        rollback_options: RollbackOptions,
    ) -> MigrationResult<()> {
        let RollbackOptions {
            rollback_strategy,
            strictness,
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

        if strictness == StrictNessLevel::Strict {
            let pending_migrations =
                Self::get_pending_migrations(all_migrations_from_dir.clone(), db.clone()).await?;

            let is_valid_rollback_state =
                pending_migrations.is_empty() || pending_migrations.len() % 2 == 0;

            if !is_valid_rollback_state {
                return Err(MigrationError::UnappliedMigrationExists {
                    migration_count: pending_migrations.len() / 2,
                });
            }
        }

        let (queries_to_run, file_paths) = match rollback_strategy {
            RollbackStrategy::Previous => Self::generate_rollback_queries_and_filepaths(
                vec![migrations_from_dir.clone()],
                vec![latest_migration],
                strictness,
            )?,
            RollbackStrategy::Number(count) => {
                let migrations_from_db = select(All)
                    .from(Migration::table_name())
                    .order_by(Migration::schema().timestamp.desc())
                    .limit(count)
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
                    .take(count as usize)
                    .collect::<Vec<_>>();

                Self::generate_rollback_queries_and_filepaths(
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
                        let is_file_cursor = m.name == file_cursor;

                        (is_before_db_latest_migration || is_latest)
                            && (is_after_file_cursor || is_file_cursor)
                    })
                    .collect::<Vec<_>>();

                Self::generate_rollback_queries_and_filepaths(
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
                log::info!("Deleting file: {:?}", file_path.to_str());
                std::fs::remove_file(file_path).map_err(|e| {
                    MigrationError::IoError(format!(
                        "Failed to delete migration file: {:?}. Error: {}",
                        file_path, e
                    ))
                })?;
                log::info!("Deleted file: {:?}", file_path.to_str());
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

    async fn get_pending_migrations(
        all_migrations: Vec<impl Into<PendingMigration>>,
        db: Surreal<impl Connection>,
    ) -> SurrealOrmResult<Vec<PendingMigration>> {
        let latest_migration = Self::get_latest_migration(db.clone()).await?;

        let pending_migrations = all_migrations
            .into_iter()
            .map(|m| {
                let m: PendingMigration = m.into();
                m
            })
            .filter(|m| {
                latest_migration.as_ref().map_or(true, |latest_migration| {
                    m.name.timestamp() > latest_migration.timestamp
                })
            })
            .collect::<Vec<_>>();

        Ok(pending_migrations)
    }

    fn generate_rollback_queries_and_filepaths(
        migrations_to_rollback: Vec<MigrationTwoWay>,
        migrations_from_db: Vec<Migration>,
        strictness: StrictNessLevel,
    ) -> MigrationResult<(Raw, Vec<PathBuf>)> {
        if strictness == StrictNessLevel::Strict {
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
                m.directory
                    .clone()
                    .map(|d| {
                        vec![
                            d.join(m.name.to_up().to_string()),
                            d.join(m.name.to_down().to_string()),
                        ]
                    })
                    .ok_or(MigrationError::MigrationPathNotFound)
            })
            .collect::<MigrationResult<Vec<_>>>()?;

        Ok((
            Raw::new(all),
            file_paths.iter().flatten().cloned().collect::<Vec<_>>(),
        ))
    }

    async fn run_pending_migrations(
        db: Surreal<impl Connection>,
        filtered_pending_migrations: Vec<PendingMigration>,
    ) -> MigrationResult<()> {
        let migration_queries = filtered_pending_migrations
            .iter()
            .map(|m| m.content.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        // Create queries to mark migrations as applied
        let mark_queries_registered_queries = filtered_pending_migrations
            .iter()
            .map(|m| Migration::create_raw(m.name.clone()).build())
            .collect::<Vec<_>>()
            .join("\n");

        log::info!(
            "Running {} migrations",
            migration_queries.split(';').count()
        );
        log::info!(
            "Marking {} query(ies) as registered",
            mark_queries_registered_queries
                .trim()
                .split(';')
                .filter(|q| q.trim().is_empty())
                .count()
        );

        // Join migrations with mark queries
        let all = format!("{}\n{}", migration_queries, mark_queries_registered_queries);

        // Run them as a transaction against a local in-memory database
        if !all.trim().is_empty() {
            begin_transaction()
                .query(Raw::new(all))
                .commit_transaction()
                .run(db.clone())
                .await?;
        }
        Ok(())
    }

    pub async fn apply_pending_migrations(
        db: Surreal<impl Connection>,
        all_migrations: Vec<impl Into<PendingMigration>>,
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
                let mut filtered_migs: Vec<PendingMigration> = vec![];

                for mig in pending_migs {
                    filtered_migs.push(mig.clone().into());
                    if mig.name == mig_filename {
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

        Self::run_pending_migrations(db.clone(), filtered_pending_migrations).await
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

impl From<bool> for StrictNessLevel {
    fn from(value: bool) -> Self {
        if value {
            Self::Strict
        } else {
            Self::Lax
        }
    }
}
