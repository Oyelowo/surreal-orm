/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{collections::BTreeMap, ops::Deref};

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
            .expect("Database not found");
        Ok(info)
    }

    pub async fn get_table_info(&self, table_name: String) -> MigrationResult<TableResourcesData> {
        let info = info_for()
            .table(table_name)
            .get_data::<TableResourcesData>(self.db())
            .await?
            .expect("Table not found");
        Ok(info)
    }

    pub async fn get_all_resources(&self) -> MigrationResult<FullDbInfo> {
        let top_level_resources = self.get_db_info().await?;
        let mut fields_by_table = BTreeMap::new();
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
        migration_name: &String,
        file_manager: &MigrationConfig,
        codebase_resources: impl DbResources,
    ) -> MigrationResult<()> {
        let name = migration_name
            .to_string()
            .split(|c: char| c != '_' && !c.is_alphanumeric())
            .collect::<Vec<_>>()
            .join("_");

        log::info!("Running migrations");

        let mut up_queries = vec![];
        let mut down_queries = vec![];
        // Left = migration directory
        // Right = codebase
        // ### TABLES
        // 1. Get all migrations from migration directory synced with db - Left
        let ComparisonDatabase { left, right } = ComparisonDatabase::init().await;
        match file_manager.migration_flag {
            MigrationFlag::TwoWay => {
                left.run_twoway_up_migrations(file_manager, true).await?;
            }
            MigrationFlag::OneWay => {
                left.run_oneway_migrations(file_manager, true).await?;
            }
        };

        // 2. Get all migrations from codebase synced with db - Right
        right
            .run_codebase_schema_queries(&codebase_resources, file_manager.migration_flag)
            .await?;
        let init = ComparisonsInit {
            left_resources: &left.resources().await,
            right_resources: &right.resources().await,
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

        let up_queries_str = up_queries
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();

        let up_queries_str = if file_manager.is_first_migration()? {
            // Defining before removing is important because
            // removing a table that doesn't exist will throw an error
            // and the transaction will be rolled back
            // so we define the table first, then remove it
            // then define the table again, to be sure it exists
            format!("{}\n{}", Migration::delete_all(), up_queries_str,)
        } else {
            up_queries_str
        };
        let down_queries_str = down_queries
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();

        let timestamp = Utc::now();
        let migration_file = match &file_manager.migration_flag {
            MigrationFlag::TwoWay => {
                let file = MigrationFileTwoWayPair {
                    up: FileMetadata {
                        name: MigrationFilename::create_up(timestamp, &name)?,
                        content: up_queries_str.clone().into(),
                    },
                    down: FileMetadata {
                        name: MigrationFilename::create_down(Utc::now(), &name)?,
                        content: down_queries_str.clone().into(),
                    },
                };
                MigrationFile::TwoWay(file)
            }
            MigrationFlag::OneWay => {
                let file = MigrationFileOneWay::new(FileMetadata {
                    name: MigrationFilename::create_oneway(timestamp, &name)?,
                    content: up_queries_str.clone().into(),
                });
                MigrationFile::OneWay(file)
            }
        };

        let prompt_empty = || {
            let confirmation = inquire::Confirm::new(
                "Are you sure you want to generate an empty migration? (y/n)",
            )
            .with_default(false)
            .with_help_message("This is good if you want to write out some queries manually")
            .prompt();
            confirmation
        };

        let query_str = format!("{}{}", up_queries_str, down_queries_str);
        if query_str.trim().is_empty() {
            match prompt_empty() {
                Ok(true) => {
                    migration_file.create_file(file_manager)?;
                    log::info!("New migration generated.");
                }
                Ok(false) => {
                    log::info!("No migration created");
                }
                Err(e) => {
                    return Err(MigrationError::PromptError(e));
                }
            };
        } else {
            migration_file.create_file(file_manager)?;
            log::info!("New migration generated.");
        };

        Ok(())
    }
}
