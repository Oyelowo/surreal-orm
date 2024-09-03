/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt;

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    Scope, Table,
};

/// Creates statement for INFO for KV(i.e system), NAMESPACE, DATABASE, SCOPE, or TABLE.
///
/// Examples
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::info_for};
/// info_for().kv().build();
/// ```
pub fn info_for() -> InfoStatementInit {
    InfoStatementInit {
        level: InfoLevel::Kv,
    }
}

/// Enum representing the different levels of the SurrealDB system
enum InfoLevel {
    Kv,
    Namespace,
    Database,
    Scope(Scope),
    Table(Table),
}

/// Information statement initialization builder
pub struct InfoStatementInit {
    level: InfoLevel,
}

impl InfoStatementInit {
    /// Creates statement for INFO for the KV(i.e system).
    /// The top-level KV command returns information regarding the namespaces which
    /// exists within the SurrealDB system.
    /// You must be authenticated as a top-level root user to execute this comman
    /// Examples
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::info_for};
    /// info_for().kv().build();
    pub fn kv(mut self) -> InfoStatement {
        self.level = InfoLevel::Kv;
        self.into()
    }

    /// Creates statement for INFO for NAMESPACE.
    /// The NS or NAMESPACE command returns information regarding the logins, tokens, and databases under a specific Namespace.
    ///
    /// You must be authenticated as a top-level root user, or a namespace user to execute this command.
    /// You must have a NAMESPACE selected before running this command.
    /// Examples
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::info_for};
    /// info_for().namespace().build();
    pub fn namespace(mut self) -> InfoStatement {
        self.level = InfoLevel::Namespace;
        self.into()
    }

    /// Creates statement for INFO for DATABASE.
    /// The DB or DATABASE command returns information regarding the logins, tokens, and scopes, and tables under a specific Database.
    ///
    /// You must be authenticated as a top-level root user, a namespace user, or a database user to execute this command.
    /// You must have a NAMESPACE and a DATABASE selected before running this command.
    /// Examples
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::info_for};
    /// info_for().database().build();
    pub fn database(mut self) -> InfoStatement {
        self.level = InfoLevel::Database;
        self.into()
    }

    /// Creates statement for INFO for SCOPE.
    /// The SCOPE command returns information regarding the tokens configured under a specific Scope.
    ///
    /// You must be authenticated as a top-level root user, a namespace user, or a database user to execute this command.
    /// You must have a NAMESPACE and a DATABASE selected before running this command.
    ///
    /// Examples
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::info_for};
    ///  info_for().scope("test_scope").build();
    pub fn scope(mut self, scope: impl Into<Scope>) -> InfoStatement {
        self.level = InfoLevel::Scope(scope.into());
        self.into()
    }

    /// Creates statement for INFO for TABLE.
    /// The TABLE command returns information regarding the events, fields, indexes, and foreign table configurations on a specific Table.
    ///
    /// You must be authenticated as a top-level root user, a namespace user, or a database user to execute this command.
    /// You must have a NAMESPACE and a DATABASE selected before running this command.
    ///
    /// Examples
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::info_for};
    ///  info_for().table("test_table").build();
    pub fn table(mut self, table: impl Into<Table>) -> InfoStatement {
        self.level = InfoLevel::Table(table.into());
        self.into()
    }
}

/// Information statement builder
pub struct InfoStatement(InfoStatementInit);

impl From<InfoStatementInit> for InfoStatement {
    fn from(value: InfoStatementInit) -> Self {
        Self(value)
    }
}

impl Queryable for InfoStatement {}

impl Erroneous for InfoStatement {}

impl Parametric for InfoStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Buildable for InfoStatement {
    fn build(&self) -> String {
        match &self.0.level {
            InfoLevel::Kv => "INFO FOR KV;".to_string(),
            InfoLevel::Namespace => "INFO FOR NS;".to_string(),
            InfoLevel::Database => "INFO FOR DB;".to_string(),
            InfoLevel::Scope(scope) => format!("INFO FOR SCOPE {};", scope),
            InfoLevel::Table(table) => format!("INFO FOR TABLE {};", table),
        }
    }
}

impl fmt::Display for InfoStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Buildable;

    #[test]
    fn test_info_for_kv_build() {
        let statement = info_for().kv().build();
        assert_eq!(statement, "INFO FOR KV;");
    }

    #[test]
    fn test_info_for_namespace_build() {
        let statement = info_for().namespace().build();
        assert_eq!(statement, "INFO FOR NS;");
    }

    #[test]
    fn test_info_for_database_build() {
        let statement = info_for().database().build();
        assert_eq!(statement, "INFO FOR DB;");
    }

    #[test]
    fn test_info_for_scope_build() {
        let statement = info_for().scope("test_scope").build();
        assert_eq!(statement, "INFO FOR SCOPE test_scope;");
    }

    #[test]
    fn test_info_for_table_build() {
        let statement = info_for().table("test_table").build();
        assert_eq!(statement, "INFO FOR TABLE test_table;");
    }
}
