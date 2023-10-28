/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::ops::Deref;

use surreal_orm::{
    statements::{begin_transaction, delete, info_for, select_value},
    *,
};
use surrealdb::{Connection, Surreal};

use crate::*;

// For the migration directory
#[derive(Debug, Clone)]
pub struct LeftDatabase(pub MigratorDatabase);

impl Deref for LeftDatabase {
    type Target = MigratorDatabase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LeftDatabase {
    pub async fn resources(&self) -> LeftFullDbInfo {
        LeftFullDbInfo(
            self.0
                .get_all_resources()
                .await
                .expect("nothing for u on left"),
        )
    }

    pub async fn run_against_db<C: Connection>(
        db: Surreal<C>,
        migrations: Vec<MigrationOneWay>,
    ) -> MigrationResult<()> {
        let queries = migrations
            .iter()
            .map(|m| m.content.clone())
            .collect::<Vec<_>>()
            .join("\n");
        println!("Running queries: {}", queries);

        let marked_up_migrations = migrations
            .iter()
            .map(|m| {
                Migration {
                    id: Migration::create_id(m.id.clone().to_string()),
                    name: m.name.clone(),
                    timestamp: m.timestamp,
                }
                .create()
                .to_raw()
                .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");

        let all = format!("{}\n{}", queries, marked_up_migrations);

        // Run them as a transaction against a local in-memory database
        if !queries.trim().is_empty() {
            begin_transaction()
                .query(Raw::new(all))
                .commit_transaction()
                .run(db)
                .await?;
        }
        Ok(())
    }

    pub async fn run_local_dir_up_migrations(
        &self,
        // db: Surreal<impl Connection>,
        migrations: Vec<MigrationTwoWay>,
    ) -> MigrationResult<()> {
        Ok(())
    }

    pub async fn run_all_local_dir_up_migrations(
        &self,
        file_manager: &FileManager,
    ) -> MigrationResult<()> {
        let all_migrations = file_manager.get_two_way_migrations()?;

        let queries = all_migrations
            .iter()
            .map(|m| m.up.clone())
            .collect::<Vec<_>>()
            .join("\n");

        // Run them as a transaction against a local in-memory database
        if !queries.trim().is_empty() {
            begin_transaction()
                .query(Raw::new(queries))
                .commit_transaction()
                .run(self.db())
                .await?;
        }
        Ok(())
    }

    pub async fn run_local_dir_oneway_content_migrations<C: Connection>(
        db: Surreal<C>,
        migrations: Vec<MigrationOneWay>,
    ) -> MigrationResult<()> {
        let queries = migrations
            .iter()
            .map(|m| m.content.clone())
            .collect::<Vec<_>>()
            .join("\n");

        println!("Running queries: {}", queries);

        // Run them as a transaction against a local in-memory database
        if !queries.trim().is_empty() {
            begin_transaction()
                .query(Raw::new(queries))
                .commit_transaction()
                .run(db)
                .await?;
        }
        Ok(())
    }
    pub async fn run_all_local_dir_one_way_migrations(
        &self,
        fm: &FileManager,
    ) -> MigrationResult<&Self> {
        let all_migrations = fm.get_oneway_migrations()?;
        let queries = all_migrations
            .into_iter()
            .map(|m| m.content)
            .collect::<Vec<_>>()
            .join("\n");

        // Run them as a transaction against a local in-memory database
        if !queries.trim().is_empty() {
            begin_transaction()
                .query(Raw::new(queries))
                .commit_transaction()
                .run(self.db())
                .await?;
        }
        Ok(self)
    }

    pub async fn get_applied_bi_migrations_from_db(&self) -> MigrationResult<Vec<Migration>> {
        // let name = Migration::schema().name;
        let migration::Schema { name, .. } = Migration::schema();
        let migration = Migration::table_name();

        // select [{ name: "Oyelowo" }]
        // select value [ "Oyelowo" ]
        // select_only. Just on object => { name: "Oyelowo" }
        let migration_names = select_value(name)
            .from(migration)
            .return_many::<Migration>(self.db())
            .await?;
        Ok(migration_names)
    }

    // pub async fn get_applied_uni_migrations_from_db(&self) -> MigrationResult<Vec<String>> {
    //     let migration_unidirectional::Schema { name, .. } = MigrationUnidirectional::schema();
    //     let migration = MigrationUnidirectional::table_name();
    //
    //     // select [{ name: "Oyelowo" }]
    //     // select value [ "Oyelowo" ]
    //     // select_only. Just on object => { name: "Oyelowo" }
    //     let migration_names = select_value(name)
    //         .from(migration)
    //         .return_many::<String>(self.db())
    //         .await?;
    //     Ok(migration_names)
    // }

    pub async fn mark_migration_as_applied(
        &self,
        migration_name: impl Into<MigrationFileName>,
    ) -> MigrationResult<Migration> {
        let migration_name: MigrationFileName = migration_name.into();
        println!("Applying migration: {}", migration_name);

        let migration = Migration {
            id: Migration::create_id(migration_name.to_string()),
            name: migration_name.to_string(),
            timestamp: migration_name.timestamp(),
        }
        .create()
        .get_one(self.db())
        .await?;
        println!("Migration applied: {}", migration_name);

        Ok(migration)
    }

    pub async fn unmark_migration(&self, migration_name: MigrationFileName) -> MigrationResult<()> {
        println!("Unmark migration: {}", migration_name);
        delete::<Migration>(Migration::create_id(migration_name.to_string()))
            .run(self.db())
            .await?;
        println!("Migration unmarked: {}", migration_name);
        Ok(())
    }

    pub async fn rollback_migration(
        db: &mut Self,
        migration_name: MigrationFileName,
        fm: FileManager,
    ) -> MigrationResult<()> {
        let migration = fm.get_two_way_migration_by_name(migration_name.clone())?;
        if let Some(migration) = migration {
            let down_migration = migration.down;
            if !down_migration.trim().is_empty() {
                // Raw::new(down_migration).run(db);
                db.execute(down_migration).await?;
            } else {
                println!("No down migration found for migration: {}", migration_name);
            }
            db.unmark_migration(migration.name.try_into()?).await?;
        } else {
            println!(
                "Cannot rollback migration: No migration found with name: {}",
                migration_name
            );
        };
        Ok(())
    }
}
