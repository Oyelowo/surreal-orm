/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::collections::{HashMap, HashSet};

use surreal_query_builder::{Field, Table};

use crate::*;

#[derive(Debug, Clone, Default)]
pub struct FullDbInfo {
    pub all_resources: DbInfo,
    pub table_resources: HashMap<Table, TableResourcesData>,
}

impl FullDbInfo {
    pub fn analyzers(&self) -> Analyzers {
        self.all_resources.analyzers()
    }

    pub fn tables(&self) -> Tables {
        self.all_resources.tables()
    }

    pub fn params(&self) -> Params {
        self.all_resources.params()
    }

    pub fn scopes(&self) -> Scopes {
        self.all_resources.scopes()
    }

    pub fn functions(&self) -> Functions {
        self.all_resources.functions()
    }

    pub fn tokens(&self) -> Tokens {
        self.all_resources.tokens()
    }

    pub fn users(&self) -> Users {
        self.all_resources.users()
    }

    pub fn get_table_info(&self, table_name: Table) -> Option<&TableResourcesData> {
        self.table_resources.get(&table_name)
    }

    pub fn get_table_names(&self) -> Vec<Table> {
        self.table_resources.keys().cloned().collect::<Vec<_>>()
    }

    pub fn get_field_def(
        &self,
        table_name: Table,
        field_name: Field,
    ) -> Option<DefineStatementRaw> {
        self.table_resources.get(&table_name).and_then(|t| {
            let x = t.fields();
            x.get_definition(&field_name.to_string()).cloned()
        })
    }

    pub fn get_table_indexes(&self, table_name: &Table) -> Option<Indexes> {
        self.table_resources
            .get(table_name)
            .map(|t| t.indexes().clone())
    }

    pub fn get_table_events(&self, table_name: &Table) -> Option<Events> {
        self.table_resources
            .get(table_name)
            .map(|t| t.events().clone())
    }

    pub fn get_table_fields(&self, table_name: &Table) -> Option<Fields> {
        self.table_resources
            .get(table_name)
            .map(|t| t.fields().clone())
    }

    pub fn get_table_field_names(&self, table_name: &Table) -> Vec<String> {
        self.table_resources
            .get(table_name)
            .map(|t| t.fields().clone())
            .unwrap_or_default()
            .get_names()
    }

    pub fn get_table_field_names_as_set(&self, table_name: &Table) -> HashSet<String> {
        self.table_resources
            .get(table_name)
            .map(|t| t.fields().clone())
            .unwrap_or_default()
            .get_names_as_set()
    }
}
