/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{collections::BTreeMap, ops::Deref};

use surreal_query_builder::{statements::info_for, *};
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

use crate::*;

// For the migration directory
#[derive(Debug, Clone)]
pub struct LeftFullDbInfo(pub FullDbInfo);

impl Deref for LeftFullDbInfo {
    type Target = FullDbInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Represents codebase resources state
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
        migration_basename: &Basename,
        file_manager: &MigrationConfig,
        codebase_resources: impl DbResources,
        prompter: impl Prompter,
    ) -> MigrationResult<()> {
        let migration_basename = migration_basename.normalize_ensure();

        // Left = migration directory
        // Right = codebase
        // ### TABLES
        // 1. Get all migrations from migration directory synced with db - Left
        let ComparisonDatabase { left, right } = ComparisonDatabase::init().await;
        let migration_flag = file_manager.migration_flag_checked()?;

        match migration_flag {
            MigrationFlag::TwoWay => {
                left.run_twoway_up_migrations(file_manager, true).await?;
            }
            MigrationFlag::OneWay => {
                left.run_oneway_migrations(file_manager, true).await?;
            }
        };

        // 2. Get all migrations from codebase synced with db - Right
        right
            .run_codebase_schema_queries(&codebase_resources, migration_flag)
            .await?;
        let init = ComparisonsInit {
            left_resources: &left.resources().await,
            right_resources: &right.resources().await,
            prompter: &prompter,
        };

        let tables = init.new_tables(&codebase_resources).queries()?;
        let analyzers = init.new_analyzers().queries()?.intersperse_new_lines();
        let params = init.new_params().queries()?.intersperse_new_lines();
        let functions = init.new_functions().queries()?.intersperse_new_lines();
        let scopes = init.new_scopes().queries()?.intersperse_new_lines();
        let tokens = init.new_tokens().queries()?.intersperse_new_lines();
        let users = init.new_users().queries()?.intersperse_new_lines();
        let migration_reset =
            Self::get_migration_reset_queries(file_manager)?.intersperse_new_lines();

        let resources = vec![
            migration_reset,
            tables,
            analyzers,
            params,
            functions,
            scopes,
            tokens,
            users,
        ];

        let mut up_queries = vec![];
        let mut down_queries = vec![];
        for resource in resources {
            let up_is_empty = resource.up_is_empty();
            let down_is_empty = resource.down_is_empty();

            if !up_is_empty {
                up_queries.extend(resource.up);
                up_queries.push(QueryType::NewLine);
                up_queries.push(QueryType::NewLine);
                up_queries.push(QueryType::NewLine);
            }

            if !down_is_empty {
                down_queries.extend(resource.down);
                down_queries.push(QueryType::NewLine);
                down_queries.push(QueryType::NewLine);
                down_queries.push(QueryType::NewLine);
            }
        }

        let up_queries_str = up_queries
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        let down_queries_str = down_queries
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        let query_str = format!("{up_queries_str}{down_queries_str}");

        let migration_file = MigrationFile::new(
            &migration_basename,
            &file_manager.migration_flag_checked()?,
            &up_queries_str.into(),
            &down_queries_str.into(),
        )?;

        if query_str.trim().is_empty() {
            match prompter.prompt_empty_migrations_trigger() {
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

    fn get_migration_reset_queries(file_manager: &MigrationConfig) -> MigrationResult<Queries> {
        if file_manager.is_first_migration()? {
            // Defining before removing is important because
            // removing a table that doesn't exist will throw an error
            // and the transaction will be rolled back
            // so we define the table first, then remove it
            // then define the table again, to be sure it exists
            Ok(Queries {
                up: vec![
                    QueryType::Comment(
                        "Resetting migrations metadata table at initialization".into(),
                    ),
                    QueryType::DeleteAll(Migration::delete_all()),
                    QueryType::Comment(
                        "Resetting migrations metadata table at initialization ending".into(),
                    ),
                ],
                down: vec![],
            })
        } else {
            Ok(Queries::default())
        }
    }
}
