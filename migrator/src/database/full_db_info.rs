/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::collections::{BTreeMap, BTreeSet};

use surreal_query_builder::{Field, Table};

use crate::*;

#[derive(Debug, Clone, Default)]
pub struct FullDbInfo {
    pub all_resources: DbInfo,
    pub table_resources: BTreeMap<Table, TableResourcesData>,
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

    pub fn get_table_info(&self, table: Table) -> Option<&TableResourcesData> {
        self.table_resources.get(&table)
    }

    pub fn get_tables(&self) -> Vec<Table> {
        self.table_resources.keys().cloned().collect::<Vec<_>>()
    }

    pub fn get_field_def(&self, table: Table, field_name: Field) -> Option<DefineStatementRaw> {
        self.table_resources
            .get(&table)
            .and_then(|t| t.fields().get_definition(&field_name).cloned())
    }

    pub fn get_table_indexes(&self, table: &Table) -> Option<Indexes> {
        self.table_resources.get(table).map(|t| t.indexes().clone())
    }

    pub fn get_table_events(&self, table: &Table) -> Option<Events> {
        self.table_resources.get(table).map(|t| t.events().clone())
    }

    pub fn get_table_fields(&self, table: &Table) -> Option<Fields> {
        self.table_resources.get(table).map(|t| t.fields().clone())
    }

    pub fn get_table_field_names(&self, table: &Table) -> Vec<String> {
        self.table_resources
            .get(table)
            .map(|t| t.fields().clone())
            .unwrap_or_default()
            .get_names()
    }

    pub fn get_table_field_names_as_set(&self, table: &Table) -> BTreeSet<String> {
        self.table_resources
            .get(table)
            .map(|t| t.fields().clone())
            .unwrap_or_default()
            .get_names_as_set()
    }
}
