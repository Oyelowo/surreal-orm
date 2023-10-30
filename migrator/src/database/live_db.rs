use surreal_query_builder::{statements::*, *};
use surrealdb::{sql::Thing, Connection, Surreal};

use crate::{
    migration, EmbeddedMigrationOneWay, FileManager, Migration, MigrationError, MigrationFileName,
    MigrationOneWay, MigrationResult, MigrationTwoWay,
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

pub struct PendingMigration {
    id: MigrationFileName,
    name: String,
    timestamp: u64,
    content: String,
}

impl From<MigrationTwoWay> for PendingMigration {
    fn from(m: MigrationTwoWay) -> Self {
        Self {
            id: m.id,
            name: m.name,
            timestamp: m.timestamp,
            content: m.up,
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

pub enum RollbackStrategy {
    Latest,
    ByCount(u32),
    UntilMigrationFileName(MigrationFileName),
}

impl MigrationRunner {
    /// Only two way migrations support rollback
    pub async fn rollback_migrations(
        fm: &FileManager,
        rollback_strategy: RollbackStrategy,
        db: Surreal<impl Connection>,
    ) -> MigrationResult<()> {
        println!("Rolling back migration");

        let all_migrations = fm.get_two_way_migrations(false)?;
        let (queries_to_run, file_paths) = match rollback_strategy {
            RollbackStrategy::Latest => {
                let latest_migration = all_migrations
                    .iter()
                    .max_by_key(|m| m.timestamp)
                    .ok_or(MigrationError::MigrationDoesNotExist)?;
                // If we were to start with the db. Potentially delete later
                // let latest_migration_in_db = select(All)
                //     .from(Migration::table_name())
                //     .order_by(Migration::schema().timestamp.desc())
                //     .limit(1)
                //     .return_one::<Migration>(db.clone())
                //     .await?
                //     .ok_or(MigrationError::MigrationDoesNotExist)?;
                //
                // let latest_migration = all_migrations
                //     .iter()
                //     .find(|m| m.id.to_string() == latest_migration_in_db.name)
                //     .ok_or(MigrationError::MigrationDoesNotExist)?;

                let migration_name = latest_migration.name.clone();
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
            RollbackStrategy::ByCount(count) => {
                let mut migrations = all_migrations.clone();
                // If we were to start with the db
                // let migrations_from_db = select(All)
                //     .from(Migration::table_name())
                //     .order_by(Migration::schema().timestamp.desc())
                //     .limit(count)
                //     .return_many::<Migration>(db.clone())
                //     .await?;
                //
                // let migrations_to_rollback = migrations
                //     .iter()
                //     .filter(|m| {
                //         migrations_from_db
                //             .iter()
                //             .any(|m_from_db| m_from_db.name == m.id.to_string())
                //     })
                //     .collect::<Vec<_>>();
                //
                migrations.sort_unstable_by_key(|m| m.timestamp);
                let migrations_to_rollback = migrations
                    .iter()
                    .rev()
                    .take(count as usize)
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
            RollbackStrategy::UntilMigrationFileName(id_name) => {
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
            println!("Deleting file: {:?}", file_path.to_str());
            std::fs::remove_file(file_path).map_err(|e| {
                MigrationError::IoError(format!(
                    "Failed to delete migration file: {:?}. Error: {}",
                    file_path, e
                ))
            });
        }

        println!("Migration rolled back");

        Ok(())
    }

    pub async fn run_pending_migrations(
        all_migrations: Vec<impl Into<PendingMigration>>,
        db: Surreal<impl Connection>,
    ) -> MigrationResult<()> {
        let migration::Schema {
            name, timestamp, ..
        } = &Migration::schema();
        let migration_table = Migration::table_name();

        // Get the latest migration
        let latest_migration = select(All)
            .from(migration_table.clone())
            .order_by(timestamp.desc())
            .limit(1)
            .return_one::<Migration>(db.clone())
            .await?;

        // Get migrations that are not yet applied
        let migrations_to_run = all_migrations
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

        // Get queries to run
        let migration_queries = migrations_to_run
            .iter()
            .map(|m| m.content.clone())
            .collect::<Vec<_>>()
            .join("\n");

        // Create queries to mark migrations as applied
        let mark_queries_registered_queries = migrations_to_run
            .iter()
            .map(|m| Migration::create_raw(m.id.clone(), m.name.clone(), m.timestamp).build())
            .collect::<Vec<_>>()
            .join("\n");

        println!("Running queries: {}", migration_queries);

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

    // pub async fn get_applied_twoway_migrations(&self) -> MigrationResult<Vec<Migration>> {
    //     // let name = Migration::schema().name;
    //     let migration::Schema { name, .. } = Migration::schema();
    //     let migration = Migration::table_name();
    //
    //     // select [{ name: "Oyelowo" }]
    //     // select value [ "Oyelowo" ]
    //     // select_only. Just on object => { name: "Oyelowo" }
    //     let migration_names = select_value(name)
    //         .from(migration)
    //         .return_many::<Migration>(self.db())
    //         .await?;
    //     Ok(migration_names)
    // }

    // pub async fn mark_migration_as_applied(
    //     &self,
    //     migration_name: impl Into<MigrationFileName>,
    //     db: Surreal<impl Connection>,
    // ) -> MigrationResult<Migration> {
    //     let migration_name: MigrationFileName = migration_name.into();
    //     println!("Applying migration: {}", migration_name.clone());
    //
    //     // let migration = Migration {
    //     //     id: Migration::create_id(migration_name.to_string()),
    //     //     name: migration_name.to_string(),
    //     //     timestamp: migration_name.timestamp(),
    //     // }
    //     // .create()
    //     // .get_one(self.db())
    //     let migration = Migration::create_raw(
    //         migration_name.clone(),
    //         migration_name.to_string(),
    //         migration_name.timestamp(),
    //     )
    //     .get_data::<Migration>(db.clone())
    //     .await?;
    //     println!("Migration applied: {}", migration_name);
    //
    //     Ok(migration.expect("Migration should be applied"))
    // }
    //
    // pub async fn unmark_migration(
    //     &self,
    //     migration_name: MigrationFileName,
    //     db: Surreal<impl Connection>,
    // ) -> MigrationResult<()> {
    //     println!("Unmark migration: {}", migration_name);
    //     db.clone()
    //         .delete::<Option<Migration>>(Migration::create_id(migration_name.clone()))
    //         .await?;
    //     // delete::<Migration>(Migration::create_id(migration_name.to_string()))
    //     //     .run(self.db())
    //     //     .await?;
    //     println!("Migration unmarked: {}", migration_name);
    //     Ok(())
    // }
    //
    // pub async fn rollback_migration(
    //     &self,
    //     migration_name: MigrationFileName,
    //     fm: FileManager,
    //     db: Surreal<impl Connection>,
    // ) -> MigrationResult<()> {
    //     let migration = fm.get_two_way_migration_by_name(migration_name.clone(), false)?;
    //     if let Some(migration) = migration {
    //         let down_migration = migration.down;
    //         if !down_migration.trim().is_empty() {
    //             // Raw::new(down_migration).run(self.db());
    //             db.clone().query(down_migration).await?;
    //         } else {
    //             println!("No down migration found for migration: {}", migration_name);
    //         }
    //         self.unmark_migration(migration.name.try_into()?).await?;
    //     } else {
    //         println!(
    //             "Cannot rollback migration: No migration found with name: {}",
    //             migration_name
    //         );
    //     };
    //     Ok(())
    // }
}
