use std::{collections::BTreeSet, ops::Deref};

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

// let file_content = std::fs::read_to_string(file_path).map_err(|e| {
//     MigrationError::IoError(format!(
//         "Failed to read migration file: {:?}. Error: {}",
//         file_path, e
//     ))
// })?;
//
// sha2::Sha256::digest(file_content.as_bytes())
//     .iter()
//     .map(|b| format!("{:02x}", b))
//     .collect::<Vec<_>>()
//     .join("")
//     .parse::<String>()
//     .map_err(|e| {
//         MigrationError::IoError(format!(
//             "Failed to generate checksum for migration file: {:?}. Error: {}",
//             file_path, e
//         ))
//     });
// hasher.update(file_content);
// let hash = hasher.finalize();
// Ok(hash.to_string())
impl MigrationRunner {
    /// Only two way migrations support rollback
    pub async fn rollback_migrations(
        db: Surreal<impl Connection>,
        fm: &FileManager,
        rollback_strategy: RollbackStrategy,
        strictness: StrictNessLevel,
    ) -> MigrationResult<()> {
        log::info!("Rolling back migration");

        let all_migrations_from_dir = fm.get_two_way_migrations(false)?;
        let (queries_to_run, file_paths) = match rollback_strategy {
            // b7a7e95b763875743b243e0930e46f22833208f58ef68032d14619ae2dfe883b
            // 16dfc18a5d5a508eee4ca1084a62518d6f6152ed2f483e3b98fee0e69f74d63a
            // 224bb451dfafda16efb615cbd331d139097d780a44727355174dc72db46c7005
            RollbackStrategy::Previous => {
                // 1. Check the latest applied/registered migration in the db
                // 2. Validate that the migration file exists
                // 3. Check if there are subsequent unapplied migration files to the live db
                // 4. If there are, panic with a message suggesting two options a and b:
                //   a. Apply the subsequent migrations and then do the rollback after
                //   that.(Suggest the command to do that).
                //   b. Delete the subsequent migration files and then do the rollback after
                //   that.(Suggest the command to do that). e.g cargo run -- reset/restore/prune
                //
                // 5. If there are no subsequent unapplied migration files to the live db,
                // then rollback the latest migration.
                // 6. If there are no applied migrations in the db, then panic with a message
                //   stating that there are no migrations to rollback. Generate new ones and apply
                //   them.
                //
                // 7. If there are no migration files in the migration directory, then panic with a
                //  message stating that there are no migrations to rollback
                //

                // 1.
                let latest_migration = Self::get_latest_migration(db.clone())
                    .await?
                    .ok_or(MigrationError::MigrationDoesNotExist)?;

                // 2.
                let latest_migration = all_migrations_from_dir
                    .iter()
                    .find(|m| m.name == latest_migration.name.clone().try_into().unwrap())
                    .ok_or(MigrationError::MigrationFileDoesNotExist)?;

                // 3.
                let pending_migrations =
                    Self::get_pending_migrations(all_migrations_from_dir.clone(), db.clone())
                        .await?;

                let is_valid_rollback_state =
                    pending_migrations.is_empty() || pending_migrations.len() % 2 == 0;

                if !is_valid_rollback_state {
                    return Err(MigrationError::UnappliedMigrationExists {
                        migration_count: pending_migrations.len() / 2,
                    });
                }

                let rollback_query = latest_migration.down.clone();
                let rollbacked_migration_deletion_query =
                    Migration::delete_raw(&latest_migration.name).build();
                let all = format!(
                    "{}\n{}",
                    rollback_query, rollbacked_migration_deletion_query
                );
                let file_paths = latest_migration
                    .directory
                    .clone()
                    .map(|d| {
                        vec![
                            d.join(latest_migration.name.to_up().to_string()),
                            d.join(latest_migration.name.to_down().to_string()),
                        ]
                    })
                    .ok_or(MigrationError::MigrationPathNotFound)?;
                (all, file_paths)
            }
            RollbackStrategy::Number(count) => {
                // New implementation: cargo run -- down -n 3

                // Perhaps, we should start with the db, getting tbe test (n) migrations from the db and then
                // getting the corresponding migration files from the migration directory.
                // Also, making sure that it tallies up chronologicaly with what's in the migration directory.
                // If it doesn't tally up, then we should panic with a message stating that the migration
                // directory and the db are out of sync. Suggest that the user should run the command
                //
                // // Implementing a checksum to make sure that the migration directory and the db are in sync
                // is probaly the way to go
                //
                // original files -> 1, 2, 3, 4, 5
                // compromised dir - > files -> 1, 2, 3, 4, 4.5, 5
                // compromisable latest -1 from db -> 5
                // less compromisable latest -n from db -> 4, 5
                // cargo run -- down -n 2
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
                    .clone()
                    .into_iter()
                    // .map(|m| {
                    //     let m: MigrationFileMeta = m.into();
                    //     m
                    // })
                    .filter(|m| {
                        (m.name.timestamp() < latest_migration.timestamp)
                            && (m.name
                                == latest_migration
                                    .name
                                    .clone()
                                    .try_into()
                                    .expect("Invalid migration name"))
                    })
                    .take(count as usize)
                    .collect::<Vec<_>>();

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
                // Self::run_filtered_migrations(db.clone(), filtered_pending_migrations).await

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

                (
                    all,
                    file_paths.iter().flatten().cloned().collect::<Vec<_>>(),
                )
            }
            RollbackStrategy::Till(file_cursor) => {
                let MigrationSchema {
                    timestamp, name, ..
                } = &Migration::schema();

                let timestamp_value = file_cursor.timestamp().into_inner();
                let simple_name = file_cursor.simple_name().into_inner();

                let migrations_from_db = select(All)
                    .from(Migration::table_name())
                    .where_(
                        cond(timestamp.gt(timestamp_value))
                            .or(cond(timestamp.eq(timestamp_value)).and(name.eq(simple_name))),
                    )
                    .order_by(timestamp.desc())
                    .return_many::<Migration>(db.clone())
                    .await?;

                let MigrationSchema {
                    timestamp, name, ..
                } = &Migration::schema();

                let timestamp_value = file_cursor.timestamp().into_inner();
                let simple_name = file_cursor.simple_name().into_inner();

                let migrations_from_db = select(All)
                    .from(Migration::table_name())
                    .where_(
                        cond(timestamp.gt(timestamp_value))
                            .or(cond(timestamp.eq(timestamp_value)).and(name.eq(simple_name))),
                    )
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

                if strictness == StrictNessLevel::Strict {
                    for (m_from_file, m_from_db) in migrations_files_to_rollback
                        .iter()
                        .zip(migrations_from_db.iter())
                    {
                        let db_mig_name: MigrationFilename = m_from_db
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

                let rollback_queries = migrations_files_to_rollback
                    .iter()
                    .map(|m| m.down.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");

                let rollbacked_migration_deletion_queries = migrations_files_to_rollback
                    .iter()
                    .map(|m| Migration::delete_raw(&m.name).build())
                    .collect::<Vec<_>>()
                    .join("\n");

                let all = format!(
                    "{}\n{}",
                    rollback_queries, rollbacked_migration_deletion_queries
                );

                let file_paths = migrations_files_to_rollback
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

                (
                    all,
                    file_paths.iter().flatten().cloned().collect::<Vec<_>>(),
                )
            }
        };

        begin_transaction()
            .query(Raw::new(queries_to_run))
            .commit_transaction()
            .run(db)
            .await?;

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

    async fn run_down_migrations_in_tx(
        db: Surreal<impl Connection>,
        migration_files: Vec<MigrationFileMeta>,
    ) -> MigrationResult<()> {
        let migration_queries = migration_files
            .iter()
            .map(|m| m.content.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        log::info!(
            "Rolling back {} migrations",
            migration_queries.split(';').count()
        );

        if !migration_queries.trim().is_empty() {
            begin_transaction()
                .query(Raw::new(migration_queries))
                .commit_transaction()
                .run(db.clone())
                .await?;
        }
        Ok(())
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
