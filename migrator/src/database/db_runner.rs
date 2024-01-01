use std::{collections::BTreeSet, path::PathBuf};

use surreal_query_builder::{statements::*, *};
use surrealdb::{Connection, Surreal};

use crate::*;

// pub struct MigrationRunner<C: Connection> {
pub struct MigrationRunner {
    // db: Surreal<C>,
    // file_manager: FileManager,
}

pub struct RollbackOptions {
    pub rollback_strategy: RollbackStrategy,
    pub mode: Mode,
    pub prune_files_after_rollback: bool,
}

impl Default for RollbackOptions {
    fn default() -> Self {
        Self {
            rollback_strategy: RollbackStrategy::Previous,
            mode: Mode::default(),
            prune_files_after_rollback: false,
        }
    }
}

impl RollbackOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_strict(&self) -> bool {
        self.mode == Mode::Strict
    }

    pub fn strategy(mut self, rollback_strategy: RollbackStrategy) -> Self {
        self.rollback_strategy = rollback_strategy;
        self
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
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
        fm: &MigrationConfig,
        rollback_options: RollbackOptions,
    ) -> MigrationResult<()> {
        let RollbackOptions {
            ref rollback_strategy,
            mode: ref strictness,
            prune_files_after_rollback,
        } = rollback_options;

        log::info!("Rolling back migration");

        let all_migrations_from_dir = fm.get_two_way_migrations_sorted_desc(false)?;
        let latest_migration = Self::get_latest_migration(db.clone())
            .await?
            .ok_or(MigrationError::MigrationDoesNotExist)?;

        let latest_migration_name: MigrationFilename = latest_migration.name.clone().try_into()?;
        let migrations_from_dir = all_migrations_from_dir
            .iter()
            .find(|m| m.up.name == latest_migration_name.to_up())
            .ok_or(MigrationError::RollbackFailed(format!(
                "The latest migration - {} - does not have a corresponding down migration file",
                latest_migration_name.to_string()
            )))?;

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
                    .filter(|m| m.up.name.timestamp() <= latest_migration.timestamp)
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
                        let is_latest = m.up.name
                            == latest_migration
                                .name
                                .clone()
                                .try_into()
                                .expect("Invalid migration name");
                        let is_before_db_latest_migration =
                            m.up.name.timestamp() < latest_migration.timestamp;

                        let is_after_file_cursor = m.up.name.timestamp() > file_cursor.timestamp();
                        let is_file_cursor = m.up.name == *file_cursor;

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
        dbg!(queries_to_run.clone());

        begin_transaction()
            .query(queries_to_run)
            .commit_transaction()
            .run(db.clone())
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

        let mut pending_migrations = all_migrations
            .into_iter()
            .map(|m| {
                let m: MigrationFile = m.into();
                PendingMigrationFile::from(m)
            })
            .filter(|m| {
                latest_migration.as_ref().map_or(true, |latest_migration| {
                    m.name_forward().timestamp() > latest_migration.timestamp
                })
            })
            .map(PendingMigrationFile::from)
            .collect::<Vec<_>>();

        pending_migrations.sort_by_key(|m| m.name_forward().timestamp());

        Ok(pending_migrations)
    }

    pub async fn get_pending_migration_filenames(
        db: Surreal<impl Connection>,
        mig_config: &MigrationConfig,
    ) -> MigrationResult<MigrationFilenames> {
        let latest_migration = Self::get_latest_migration(db.clone()).await?;

        let pending_migrations = mig_config
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
        fm: &MigrationConfig,
        migrations_to_rollback: Vec<MigrationFileTwoWayPair>,
        migrations_from_db: Vec<Migration>,
        mode: &Mode,
    ) -> MigrationResult<(Raw, Vec<PathBuf>)> {
        if mode.is_strict() {
            for (m_from_file, m_from_db) in
                migrations_to_rollback.iter().zip(migrations_from_db.iter())
            {
                let db_mig_name: MigrationFilename = m_from_db.name.clone().try_into()?;

                m_from_db
                    .checksum_up
                    .verify(&m_from_file.up.name, &m_from_file.up.content)?;

                m_from_db
                    .clone()
                    .checksum_down
                    .ok_or(MigrationError::NoChecksumInDb {
                        migration_name: m_from_db.name.clone(),
                    })?
                    .verify(&m_from_file.down.name, &m_from_file.down.content)?;

                if m_from_file.up.name != db_mig_name.to_up() {
                    return Err(MigrationError::MigrationFileVsDbNamesMismatch {
                        migration_file_name: m_from_file.up.name.to_string(),
                        migration_db_name: db_mig_name.to_string(),
                    });
                }
            }
        }

        log::info!("Rolling back {} migration(s)", migrations_to_rollback.len());

        let rollback_queries = migrations_to_rollback
            .clone()
            .into_iter()
            .map(|m| m.down.content.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let rollbacked_migration_deletion_queries = migrations_from_db
            .iter()
            // We are deleting by upname because that's how theyre are stored
            .map(|m| Migration::delete_raw(&m.id).build())
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
                        d.join(m.up.name.to_string()),
                        d.join(m.down.name.to_string()),
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
                        Migration::create_raw(&m.name(), &m.content().as_checksum()?, None);

                    migration_queries.push(m.content().to_owned());
                    mark_queries_registered_queries.push(created_registered_mig);
                }
                MigrationFile::TwoWay(m) => {
                    let created_registered_mig = Migration::create_raw(
                        &m.up.name,
                        &m.up.content.as_checksum()?,
                        Some(&m.down.content.as_checksum()?),
                    );

                    migration_queries.push(m.up.content);
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
                    if *mig.name_forward() == mig_filename {
                        migration_found = true;
                        break;
                    }
                }

                if !migration_found {
                    return Err(MigrationError::MigrationNotFoundFromPendingMigrations(
                        mig_filename,
                    ));
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
        mode: Mode,
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
                Self::_get_db_migrations_meta_from_mig_files(migrations_local_dir, db_migs, mode)?
            }
        };
        Ok(migrations)
    }

    fn _get_db_migrations_meta_from_mig_files(
        local_migrations: Vec<MigrationFilename>,
        db_migs: Vec<Migration>,
        mode: Mode,
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
        if mode.is_strict() {
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
