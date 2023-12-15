/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::ops::Deref;

use surreal_query_builder::{statements::begin_transaction, *};
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

    pub async fn run_all_local_dir_up_migrations(
        &self,
        file_manager: &FileManager,
        create_migration_table: bool,
    ) -> MigrationResult<()> {
        let all_migrations = file_manager.get_two_way_migrations(create_migration_table)?;

        let queries = all_migrations
            .iter()
            .map(|m: &MigrationTwoWay| m.up.to_string())
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
            .map(|m| m.content.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        log::info!("Running queries: {}", queries);

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
        create_migration_table: bool,
    ) -> MigrationResult<&Self> {
        let all_migrations = fm.get_oneway_migrations(create_migration_table)?;
        let queries = all_migrations
            .into_iter()
            .map(|m| m.content.to_string())
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
}
