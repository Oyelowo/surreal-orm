/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::ops::Deref;

use surreal_query_builder::{statements::begin_transaction, *};

use crate::*;

pub enum By {
    NewName(String),
    OldName(String),
}

// For the codebase
#[derive(Debug, Clone)]
pub struct RightDatabase(pub MigratorDatabase);

impl RightDatabase {
    pub async fn resources(&self) -> RightFullDbInfo {
        // TODO: log the error also
        RightFullDbInfo(self.0.get_all_resources().await.expect(
            "Something went wrong getting resources for the codebase. Please check your connection to the database.",
        ))
    }

    pub async fn run_codebase_schema_queries(
        &self,
        code_resources: &impl DbResources,
        migration_type: MigrationFlag,
    ) -> MigrationResult<()> {
        let delete_all_migrations = Migration::delete_all();
        let migration_meta_table_def = Migration::define_table().to_raw();
        let migration_meta_table_fields_def = Migration::define_fields(migration_type)
            .iter()
            .map(|f| f.to_raw().build())
            .collect::<Vec<_>>()
            .join("\n");
        let queries = Self::get_codebase_schema_queries(code_resources);
        // Defining before removing is important because
        // removing a table that doesn't exist will throw an error
        // and the transaction will be rolled back
        // so we define the table first, then remove it
        // then define the table again, to be sure it exists
        let queries =
            format!("{delete_all_migrations}\n{migration_meta_table_def}\n{migration_meta_table_fields_def}\n{queries}");

        if !queries.is_empty() {
            let queries = begin_transaction()
                .query(Raw::new(queries))
                .commit_transaction()
                .run(self.db())
                .await
                .expect("Something went wrong running the codebase schema queries");
        }

        Ok(())
    }

    pub fn find_field_with_oldname_attr(
        table_name: Table,
        field_name: Field,
        resources: impl DbResources,
    ) -> Option<FieldMetadata> {
        resources
            .tables_fields_meta()
            .get(&table_name)
            .unwrap_or(&vec![])
            .clone()
            .into_iter()
            .find(|f| f.name.to_string() == field_name.to_string() && f.old_name.is_some())
    }

    pub fn find_field_has_old_name(
        resources: &impl DbResources,
        table_name: &Table,
        by: By,
    ) -> Option<FieldMetadata> {
        resources
            .tables_fields_meta()
            .get(table_name)
            .unwrap_or(&vec![])
            .clone()
            .into_iter()
            .filter(|field_meta| {
                field_meta
                    .old_name
                    .clone()
                    .is_some_and(|o| !o.to_string().is_empty())
            })
            .find(|f| match &by {
                By::NewName(new_name) => f.name.to_string() == *new_name,
                By::OldName(old_name) => f
                    .old_name
                    .clone()
                    .filter(|n| n.to_string() == *old_name)
                    .is_some(),
            })
    }

    pub fn get_codebase_schema_queries(db_resources: &impl DbResources) -> String {
        let queries_joined = [
            db_resources.tokens(),
            db_resources.scopes(),
            db_resources.analyzers(),
            db_resources.params(),
            db_resources.functions(),
            db_resources.users(),
            db_resources.tables(),
        ]
        .iter()
        .flat_map(|res_raw| res_raw.iter().map(|r| r.to_raw().build()))
        .collect::<Vec<_>>()
        .join(";\n");

        queries_joined
    }
}

impl Deref for RightDatabase {
    type Target = MigratorDatabase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
