/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use surrealdb::sql;

use crate::{
    sql::{Buildable, Database, Namespace, Runnable},
    Parametric, Queryable,
};

pub fn use_() -> UseStatement {
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
            query.push_str(&format!(" DB {database}"));
        }

        if let Some(namespace) = &self.namespace {
            query.push_str(&format!(" NS {namespace}"));
        }

        query.push_str(";");

        query
    }
}

impl Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Runnable for UseStatement {}

impl Queryable for UseStatement {}

impl Parametric for UseStatement {
    fn get_bindings(&self) -> crate::BindingsList {
        vec![]
    }
}

#[cfg(test)]
mod tests {

    use crate::sql::Database;

    use super::*;

    #[test]
    fn test_use_statement() {
        assert_eq!(
            use_().database(Database::from("root".to_string())).build(),
            "USE DB root;"
        );
        assert_eq!(
            use_()
                .namespace(Namespace::from("mars".to_string()))
                .to_string(),
            "USE NS mars;"
        );

        assert_eq!(
            use_()
                .database(Database::from("root".to_string()))
                .namespace(Namespace::from("mars".to_string()))
                .build(),
            "USE DB root NS mars;"
        );
    }
}
