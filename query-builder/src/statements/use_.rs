/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::Display;

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::{Database, Namespace},
};

/// Creates a USE statement.
/// The USE statement specifies a namespace and / or a database to use for the subsequent SurrealQL statements.
///
/// Examples
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use std::time::Duration;
/// use surreal_orm::{*, statements::use_};
///
/// // Switch to the test Namespace
/// use_().database(Database::from("test".to_string()));
///
/// // Switch to the test Namespace
/// use_().namespace(Namespace::from("test".to_string()));
///
/// // Switch to the test Namespace and test database
/// use_()
///     .database(Database::from("test".to_string()))
///     .namespace(Namespace::from("test".to_string()));
/// ```
pub fn use_() -> UseStatement {
    UseStatement::default()
}

/// Use statement builder
#[derive(Default, Clone)]
pub struct UseStatement {
    namespace: Option<Namespace>,
    database: Option<Database>,
}

impl UseStatement {
    /// Switch to the Namespace
    pub fn namespace(mut self, namespace: impl Into<Namespace>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    /// Switch to the Database
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

        query.push(';');

        query
    }
}

impl Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Queryable for UseStatement {}

impl Erroneous for UseStatement {}

impl Parametric for UseStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

#[cfg(test)]
mod tests {

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
