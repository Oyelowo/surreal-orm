use std::{
    fs::File,
    io::{BufReader, Read},
};

use sha2::{self, Digest, Sha256};
use surreal_query_builder::{statements::*, *};
use surrealdb::{Connection, Surreal};

use crate::{
    cli::Status, EmbeddedMigrationOneWay, FileManager, Migration, MigrationError,
    MigrationFileName, MigrationOneWay, MigrationResult, MigrationTwoWay,
};

// pub struct MigrationRunner<C: Connection> {
pub struct MigrationRunner {
    // db: Surreal<C>,
    // file_manager: FileManager,
}

impl From<MigrationTwoWay> for MigrationOneWay {
    fn from(m: MigrationTwoWay) -> Self {
        Self {
            id: m.id,
            name: m.name,
            timestamp: m.timestamp,
            content: m.up,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PendingMigration {
    id: MigrationFileName,
    name: String,
    timestamp: u64,
    content: String,
    // checksum_up: Checksum,
    // checksum_down: Option<Checksum>,
}

type MigrationFileMeta = PendingMigration;

impl From<MigrationTwoWay> for PendingMigration {
    fn from(m: MigrationTwoWay) -> Self {
        Self {
            id: m.id,
            name: m.name,
            timestamp: m.timestamp,
            content: m.up,
            // checksum_up: m.checksum_up,
            // checksum_down: Some(m.checksum_down),
        }
    }
}

impl From<MigrationOneWay> for PendingMigration {
    fn from(m: MigrationOneWay) -> Self {
        Self {
            id: m.id,
            name: m.name,
            timestamp: m.timestamp,
            content: m.content,
        }
    }
}

impl From<EmbeddedMigrationOneWay> for PendingMigration {
    fn from(m: EmbeddedMigrationOneWay) -> Self {
        Self {
            id: m.id.to_string().try_into().expect("Invalid migration id"),
            name: m.name.to_string(),
            timestamp: m.timestamp,
            content: m.content.to_string(),
        }
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
    Till(MigrationFileName),
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
    Till(MigrationFileName),
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
        fm: &FileManager,
        rollback_strategy: RollbackStrategy,
        db: Surreal<impl Connection>,
    ) -> MigrationResult<()> {
        log::info!("Rolling back migration");

        let all_migrations = fm.get_two_way_migrations(false)?;
        let (queries_to_run, file_paths) = match rollback_strategy {
            // b7a7e95b763875743b243e0930e46f22833208f58ef68032d14619ae2dfe883b
            // 16dfc18a5d5a508eee4ca1084a62518d6f6152ed2f483e3b98fee0e69f74d63a
            // 224bb451dfafda16efb615cbd331d139097d780a44727355174dc72db46c7005
            RollbackStrategy::Previous => {
                for m in &all_migrations {
                    let path = m
                        .directory
                        .clone()
                        .map(|d| d.join(m.id.to_down().to_string()))
                        .ok_or(MigrationError::MigrationPathNotFound)?;
                    // let x = Self::_generate_checksum(path)?;
                    // log::warn!("Migration file shaaaaa: {:?}", x);
                    panic!();
                }
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
                let latest_migration = all_migrations
                    .iter()
                    .find(|m| m.id.to_string() == latest_migration.name)
                    .ok_or(MigrationError::MigrationFileDoesNotExist)?;

                // 3.

                let pending_migrations =
                    Self::get_pending_migrations(all_migrations.clone(), db.clone()).await?;

                let is_valid_rollback_state =
                    pending_migrations.is_empty() || pending_migrations.len() % 2 == 0;

                if !is_valid_rollback_state {
                    return Err(MigrationError::UnappliedMigrationExists {
                        migration_count: pending_migrations.len() / 2,
                    });
                }

                let rollback_query = latest_migration.down.clone();
                let rollbacked_migration_deletion_query =
                    Migration::delete_raw(latest_migration.id.clone()).build();
                let all = format!(
                    "{}\n{}",
                    rollback_query, rollbacked_migration_deletion_query
                );
                let file_paths = latest_migration
                    .directory
                    .clone()
                    .map(|d| {
                        vec![
                            d.join(latest_migration.id.to_up().to_string()),
                            d.join(latest_migration.id.to_down().to_string()),
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

                let migrations_files_to_rollback = all_migrations
                    .clone()
                    .into_iter()
                    // .map(|m| {
                    //     let m: MigrationFileMeta = m.into();
                    //     m
                    // })
                    .filter(|m| m.timestamp < latest_migration.timestamp)
                    .take(count as usize)
                    .collect::<Vec<_>>();

                for (m_from_file, m_from_db) in migrations_files_to_rollback
                    .into_iter()
                    .zip(migrations_from_db.into_iter())
                {
                    let x = m_from_db
                        .checksum_up
                        .verify(&m_from_file.up, &m_from_file.id)?;

                    let x = m_from_db
                        .checksum_down
                        .ok_or(MigrationError::NoChecksumInDb {
                            migration_name: m_from_db.id.to_string(),
                        })?
                        .verify(&m_from_file.down, &m_from_file.id)?;

                    if m_from_file.name != m_from_db.name {
                        return Err(MigrationError::MigrationFileDoesNotExist);
                    }
                }

                // left nth migration files metadata, right nth live db migration metadata
                // Compare the file names(which includes timestamps) and the checksum

                // Check the migration meta from the live db against the older migration files in the migration directory meta
                // Compare the file names(which includes timestamps) and the checksum

                // let latest_migration = Self::get_latest_migration(db.clone())
                //     .await?
                //     .ok_or(MigrationError::MigrationDoesNotExist)?;
                //
                // // 2.
                // let latest_migration = all_migrations
                //     .iter()
                //     .find(|m| m.id.to_string() == latest_migration.name)
                //     .ok_or(MigrationError::MigrationFileDoesNotExist)?;
                //
                // // 3.
                //
                // let pending_migrations =
                //     Self::get_pending_migrations(all_migrations.clone(), db.clone()).await?;
                //
                // let is_valid_rollback_state =
                //     pending_migrations.is_empty() || pending_migrations.len() % 2 == 0;
                //
                // if !is_valid_rollback_state {
                //     return Err(MigrationError::UnappliedMigrationExists {
                //         migration_count: pending_migrations.len() / 2,
                //     });
                // }
                //
                // let rollback_query = latest_migration.down.clone();
                // let rollbacked_migration_deletion_query =
                //     Migration::delete_raw(latest_migration.id.clone()).build();
                // let all = format!(
                //     "{}\n{}",
                //     rollback_query, rollbacked_migration_deletion_query
                // );
                // let file_paths = latest_migration
                //     .directory
                //     .clone()
                //     .map(|d| {
                //         vec![
                //             d.join(latest_migration.id.to_up().to_string()),
                //             d.join(latest_migration.id.to_down().to_string()),
                //         ]
                //     })
                //     .ok_or(MigrationError::MigrationPathNotFound)?;
                // // (all, file_paths)
                //
                // ///////////
                //
                // let mut migrations = all_migrations.clone();
                // // If we were to start with the db
                // // let migrations_from_db = select(All)
                // //     .from(Migration::table_name())
                // //     .order_by(Migration::schema().timestamp.desc())
                // //     .limit(count)
                // //     .return_many::<Migration>(db.clone())
                // //     .await?;
                // //
                // // let migrations_to_rollback = migrations
                // //     .iter()
                // //     .filter(|m| {
                // //         migrations_from_db
                // //             .iter()
                // //             .any(|m_from_db| m_from_db.name == m.id.to_string())
                // //     })
                // //     .collect::<Vec<_>>();
                // //
                // migrations.sort_unstable_by_key(|m| m.timestamp);
                // let migrations_to_rollback = migrations
                //     .iter()
                //     .rev()
                //     .take(count as usize)
                //     .collect::<Vec<_>>();
                // let rollback_queries = migrations_to_rollback
                //     .iter()
                //     .map(|m| m.down.clone())
                //     .collect::<Vec<_>>()
                //     .join("\n");
                //
                // let rollbacked_migration_deletion_queries = migrations_to_rollback
                //     .iter()
                //     .map(|m| Migration::delete_raw(m.id.clone()).build())
                //     .collect::<Vec<_>>()
                //     .join("\n");
                //
                // let all = format!(
                //     "{}\n{}",
                //     rollback_queries, rollbacked_migration_deletion_queries
                // );
                //
                // let file_paths = migrations_to_rollback
                //     .iter()
                //     .map(|m| {
                //         m.directory
                //             .clone()
                //             .map(|d| {
                //                 vec![
                //                     d.join(m.id.to_up().to_string()),
                //                     d.join(m.id.to_down().to_string()),
                //                 ]
                //             })
                //             .ok_or(MigrationError::MigrationPathNotFound)
                //     })
                //     .collect::<MigrationResult<Vec<_>>>()?;
                //
                // (
                //     all,
                //     file_paths.iter().flatten().cloned().collect::<Vec<_>>(),
                // )
                todo!()
            }
            RollbackStrategy::Till(id_name) => {
                let mut migrations = all_migrations.clone();
                migrations.sort_by_key(|m| m.timestamp);
                let migrations_to_rollback = migrations
                    .iter()
                    .rev()
                    .take_while(|m| m.name != id_name.to_string())
                    .collect::<Vec<_>>();

                let rollback_queries = migrations_to_rollback
                    .iter()
                    .map(|m| m.down.clone())
                    .collect::<Vec<_>>()
                    .join("\n");

                let rollbacked_migration_deletion_queries = migrations_to_rollback
                    .iter()
                    .map(|m| Migration::delete_raw(m.id.clone()).build())
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
                                    d.join(m.id.to_up().to_string()),
                                    d.join(m.id.to_down().to_string()),
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
                    m.timestamp > latest_migration.timestamp
                })
            })
            .collect::<Vec<_>>();

        Ok(pending_migrations)
    }

    async fn run_filtered_migrations(
        filtered_pending_migrations: Vec<PendingMigration>,
        db: Surreal<impl Connection>,
    ) -> MigrationResult<()> {
        let migration_queries = filtered_pending_migrations
            .iter()
            .map(|m| m.content.clone())
            .collect::<Vec<_>>()
            .join("\n");

        // Create queries to mark migrations as applied
        let mark_queries_registered_queries = filtered_pending_migrations
            .iter()
            .map(|m| Migration::create_raw(m.id.clone(), m.name.clone(), m.timestamp).build())
            .collect::<Vec<_>>()
            .join("\n");

        log::info!(
            "Running {} migrations and",
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
                let mut filtered_migs = vec![];

                for mig in pending_migs {
                    filtered_migs.push(mig.clone());
                    if mig.id.to_string() == mig_filename.to_string() {
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

        Self::run_filtered_migrations(filtered_pending_migrations, db.clone()).await
    }

    pub(crate) async fn list_migrations(
        migrations_local_dir: Vec<Migration>,
        db: Surreal<impl Connection>,
        status: Status,
    ) -> MigrationResult<Vec<Migration>> {
        let migrations = match status {
            Status::All => {
                let mut migrations = migrations_local_dir;
                migrations.sort_by_key(|m| m.timestamp);
                migrations
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
                    .map(|m| {
                        let m: Migration = m.into();
                        m
                    })
                    .filter(|m| {
                        latest_applied_migration
                            .as_ref()
                            .map_or(true, |latest_migration| {
                                m.timestamp > latest_migration.timestamp
                            })
                    })
                    .collect::<Vec<_>>();
                migrations.sort_by_key(|m| m.timestamp);
                migrations
            }
            Status::Applied => {
                select(All)
                    .from(Migration::table_name())
                    .order_by(Migration::schema().timestamp.asc())
                    .return_many::<Migration>(db.clone())
                    .await?
            }
        };
        Ok(migrations)
    }
}
