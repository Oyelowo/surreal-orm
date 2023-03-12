/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use surrealdb::sql;

use crate::{
    query_insert::Buildable,
    query_remove::{Database, Namespace, Runnable},
    query_select::Duration,
    Queryable,
};

pub fn use_(duration: impl Into<Duration>) -> UseStatement {
    UseStatement::default()
}

#[derive(Default)]
pub struct UseStatement {
    namespace: Option<Namespace>,
    database: Option<Database>,
}

impl UseStatement {
    pub fn namespace(mut self, namespace: impl Into<Namespace>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn database(mut self, database: impl Into<Database>) -> Self {
        self.database = Some(database.into());
        self
    }
}

impl Buildable for UseStatement {
    fn build(&self) -> String {
        let mut query = String::from("USE");

        if let Some(database) = &self.database {
            query.push_str(&format!(" {database}"));
        }

        if let Some(namespace) = &self.namespace {
            query.push_str(&format!(" {namespace}"));
        }

        query.push_str(";");

        query
    }
}

impl Runnable for UseStatement {}

impl Queryable for UseStatement {}
