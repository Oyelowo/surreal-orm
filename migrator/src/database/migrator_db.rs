/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{collections::HashMap, ops::Deref};

use chrono::Utc;
use surreal_query_builder::{statements::info_for, *};
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

use crate::*;

#[derive(Debug, Clone)]
pub struct LeftFullDbInfo(pub FullDbInfo);

impl Deref for LeftFullDbInfo {
    type Target = FullDbInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct RightFullDbInfo(pub FullDbInfo);

impl Deref for RightFullDbInfo {
    type Target = FullDbInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct ComparisonDatabase {
    left: LeftDatabase,
    right: RightDatabase,
}

impl ComparisonDatabase {
    pub async fn init() -> Self {
        let left = LeftDatabase(MigratorDatabase::init().await);
        let right = RightDatabase(MigratorDatabase::init().await);
        Self { left, right }
    }
}

#[derive(Debug, Clone)]
pub struct MigratorDatabase {
    pub db: Surreal<Db>,
}

impl MigratorDatabase {
    pub async fn init() -> Self {
        let db = Surreal::new::<Mem>(()).await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        Self { db }
    }

    pub fn db(&self) -> Surreal<Db> {
        self.db.clone()
    }

    pub async fn get_db_info(&self) -> MigrationResult<DbInfo> {
        let info = info_for()
            .database()
            .get_data::<DbInfo>(self.db())
            .await?
            .unwrap();
        Ok(info)
    }

    pub async fn get_table_info(&self, table_name: String) -> MigrationResult<TableResourcesData> {
        let info = info_for()
            .table(table_name)
            .get_data::<TableResourcesData>(self.db())
            .await?
            .unwrap();
        Ok(info)
    }

    pub async fn get_all_resources(&self) -> MigrationResult<FullDbInfo> {
        let top_level_resources = self.get_db_info().await?;
        let mut fields_by_table = HashMap::new();
        for table_name in top_level_resources.tables().get_names() {
            let table_info = self.get_table_info(table_name.clone()).await?;
            fields_by_table.insert(table_name.into(), table_info);
        }
        let all_resources = FullDbInfo {
            all_resources: top_level_resources,
            table_resources: fields_by_table,
        };
        Ok(all_resources)
    }

    pub async fn execute(&self, query: String) -> MigrationResult<()> {
        log::info!("Executing query: {}", query);
        self.db().query(query).await?;
        Ok(())
    }

    pub async fn generate_migrations(
        migration_name: String,
        file_manager: &FileManager,
        codebase_resources: impl DbResources,
    ) -> MigrationResult<()> {
        let name = migration_name
            .to_string()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("_");
        log::info!("Running migrations");
        let mut up_queries = vec![];
        let mut down_queries = vec![];
        //  DIFFING
        //  LEFT
        //
        // Left = migration directory
        // Right = codebase
        // ### TABLES
        // 1. Get all migrations from migration directory synced with db - Left
        let ComparisonDatabase { left, right } = ComparisonDatabase::init().await;
        match file_manager.migration_flag {
            MigrationFlag::TwoWay => {
                left.run_all_local_dir_up_migrations(file_manager, true)
                    .await?;
            }
            MigrationFlag::OneWay => {
                left.run_all_local_dir_one_way_migrations(file_manager, true)
                    .await?;
            }
        };

        // 2. Get all migrations from codebase synced with db - Right
        // let code_base_resources: Resources = resouces.i;
        right
            .run_codebase_schema_queries(&codebase_resources)
            .await?;
        let init = ComparisonsInit {
            left_resources: &left.resources().await,
            right_resources: &right.resources().await,
            // codebase_resources: &resources,
        };
        let tables = init.new_tables(&codebase_resources).queries();
        let analyzers = init.new_analyzers().queries();
        let params = init.new_params().queries();
        let functions = init.new_functions().queries();
        let scopes = init.new_scopes().queries();
        let tokens = init.new_tokens().queries();
        let users = init.new_users().queries();

        let resources = vec![tables, analyzers, params, functions, scopes, tokens, users];

        for resource in resources {
            let resource = resource?;
            up_queries.extend(resource.up);
            down_queries.extend(resource.down);
        }

        // TODO: Create a warning to prompt user if they truly want to create empty migrations
        let up_queries_str = up_queries
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();
        let down_queries_str = down_queries
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();
        // let mig_type = MigrationType::OneWay(up_queries_str.clone().unwrap_or_default());
        let migration_type = match &file_manager.migration_flag {
            MigrationFlag::TwoWay => MigrationType::TwoWay {
                up: up_queries_str.clone(),
                down: down_queries_str.clone(),
            },
            MigrationFlag::OneWay => MigrationType::OneWay(up_queries_str.clone()),
        };

        let timestamp = Utc::now();

        let prompt_empty = || {
            let confirmation = inquire::Confirm::new(
                "Are you sure you want to generate an empty migration? (y/n)",
            )
            .with_default(false)
            .with_help_message("This is good if you want to write out some queries manually")
            .prompt();
            confirmation
        };

        match migration_type {
            MigrationType::OneWay(query_str) => {
                if query_str.trim().is_empty() {
                    match prompt_empty() {
                        Ok(true) => {
                            MigrationFileName::create_oneway(timestamp, name)?
                                .create_file(query_str, file_manager)?;
                        }
                        Ok(false) => {
                            log::info!("No migration created");
                        }
                        Err(e) => {
                            return Err(MigrationError::PromptError(e));
                        }
                    };
                } else {
                    MigrationFileName::create_oneway(timestamp, name)?
                        .create_file(query_str, file_manager)?;
                };
            }
            MigrationType::TwoWay { up, down } => {
                match (up.is_empty(), down.is_empty()) {
                    (true, true) => {
                        match prompt_empty() {
                            Ok(true) => {
                                MigrationFileName::create_up(timestamp, &name)?
                                    .create_file(up, file_manager)?;
                                MigrationFileName::create_down(timestamp, name)?
                                    .create_file(down, file_manager)?;
                            }
                            Ok(false) => {
                                log::info!("No migration created");
                            }
                            Err(e) => {
                                return Err(MigrationError::PromptError(e));
                            }
                        };
                    }
                    (false, false) => {
                        log::info!("HERE=====");
                        // log::info!("UP MIGRATIOM: \n {}", up_queries_str.clone());
                        // log::info!("DOWN MIGRATIOM: \n {}", down_queries_str.clone());
                        MigrationFileName::create_up(timestamp, &name)?
                            .create_file(up, file_manager)?;
                        MigrationFileName::create_down(timestamp, name)?
                            .create_file(down, file_manager)?;
                    }
                    (true, false) => {
                        return Err(MigrationError::MigrationUpQueriesEmpty);
                    }
                    (false, true) => {
                        return Err(MigrationError::MigrationDownQueriesEmpty);
                    }
                };
            }
        }
        //
        //
        // 4. Aggregate all the new up and down queries
        // 5. Run the queries as a transaction
        // 6. Update the migration directory with the new migrations queries i.e m::create_migration_file(up, down, name);
        // 7. Mark the queries as registered i.e mark_migration_as_applied

        // Run the diff
        // 5. Update the migration directory
        //

        // Old rough implementation
        // let applied_migrations = db.get_applied_migrations_from_db();
        // let all_migrations = Self::get_all_from_migrations_dir();
        //
        // let applied_migrations = applied_migrations.await?;
        // for migration in all_migrations {
        //     if !applied_migrations.contains(&migration.name) {
        //         db.execute(migration.up);
        //         db.mark_migration_as_applied(migration.name);
        //     }
        // }
        Ok(())
    }
}
