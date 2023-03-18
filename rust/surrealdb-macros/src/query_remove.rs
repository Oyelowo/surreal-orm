/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

/*
 *
 *
REMOVE statement

Statement syntax
REMOVE [
    NAMESPACE @name
    | DATABASE @name
    | LOGIN @name ON [ NAMESPACE | DATABASE ]
    | TOKEN @name ON [ NAMESPACE | DATABASE ]
    | SCOPE @name
    | TABLE @name
    | EVENT @name ON [ TABLE ] @table
    | FIELD @name ON [ TABLE ] @table
    | INDEX @name ON [ TABLE ] @table
]
 * */

use std::fmt::{self, Display};

use surrealdb::sql;

use crate::{
    sql::{Buildable, Namespace, Runnables},
    Field, Parametric, Queryable,
};

pub fn remove_namespace(namespace: impl Into<Namespace>) -> RemoveNamespaceStatement {
    RemoveNamespaceStatement::new(namespace)
}
pub struct RemoveNamespaceStatement {
    namespace: Namespace,
}

impl RemoveNamespaceStatement {
    fn new(namespace: impl Into<Namespace>) -> Self {
        let namespace = namespace.into();
        Self { namespace }
    }
}

impl Buildable for RemoveNamespaceStatement {
    fn build(&self) -> String {
        format!("REMOVE NAMESPACE {}", self.namespace)
    }
}
impl Runnables for RemoveNamespaceStatement {}

pub fn remove_database(database: impl Into<Database>) -> RemoveDatabaseStatement {
    RemoveDatabaseStatement::new(database)
}
pub struct RemoveDatabaseStatement {
    database: Database,
}

impl RemoveDatabaseStatement {
    fn new(database: impl Into<Database>) -> Self {
        Self {
            database: database.into(),
        }
    }
}

impl Buildable for RemoveDatabaseStatement {
    fn build(&self) -> String {
        format!("REMOVE DATABASE {}", self.database)
    }
}
impl Runnables for RemoveDatabaseStatement {}

pub fn remove_login(login: impl Into<Login>) -> RemoveLoginStatement {
    RemoveLoginStatement::new(login)
}
pub struct RemoveLoginStatement {
    login: Login,
    on: Option<NamespaceOrDatabase>,
}

impl RemoveLoginStatement {
    fn new(login: impl Into<Login>) -> Self {
        Self {
            login: login.into(),
            on: None,
        }
    }

    pub fn on_namespace(mut self) -> Self {
        self.on = Some(NamespaceOrDatabase::Namespace);
        self
    }

    pub fn on_database(mut self) -> Self {
        self.on = Some(NamespaceOrDatabase::Database);
        self
    }
}

impl Buildable for RemoveLoginStatement {
    fn build(&self) -> String {
        let mut query = format!("REMOVE LOGIN {}", self.login);

        if let Some(on) = &self.on {
            query = format!("{} ON {}", query, on);
        }
        query
    }
}
impl Runnables for RemoveLoginStatement {}

pub fn remove_token(token: impl Into<Token>) -> RemoveTokenStatement {
    RemoveTokenStatement::new(token)
}
pub struct RemoveTokenStatement {
    token: Token,
    on: Option<NamespaceOrDatabase>,
}

impl RemoveTokenStatement {
    fn new(token: impl Into<Token>) -> Self {
        Self {
            token: token.into(),
            on: None,
        }
    }

    pub fn on_namespace(mut self) -> Self {
        self.on = Some(NamespaceOrDatabase::Namespace);
        self
    }

    pub fn on_database(mut self) -> Self {
        self.on = Some(NamespaceOrDatabase::Database);
        self
    }
}

impl Buildable for RemoveTokenStatement {
    fn build(&self) -> String {
        let mut query = format!("REMOVE TOKEN {}", self.token);

        if let Some(on) = &self.on {
            query = format!("{} ON {}", query, on);
        }
        query
    }
}
impl Runnables for RemoveTokenStatement {}

pub fn remove_scope(scope: impl Into<Scope>) -> RemoveScopeStatement {
    RemoveScopeStatement::new(scope)
}
pub struct RemoveScopeStatement {
    scope: Scope,
}

impl RemoveScopeStatement {
    fn new(scope: impl Into<Scope>) -> Self {
        Self {
            scope: scope.into(),
        }
    }
}

impl Queryable for RemoveScopeStatement {}

impl Parametric for RemoveScopeStatement {
    fn get_bindings(&self) -> crate::BindingsList {
        vec![]
    }
}

impl Buildable for RemoveScopeStatement {
    fn build(&self) -> String {
        format!("REMOVE SCOPE {}", self.scope)
    }
}

impl Display for RemoveScopeStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Runnables for RemoveScopeStatement {}

pub fn remove_table(table: impl Into<Table>) -> RemoveTableStatement {
    RemoveTableStatement::new(table)
}
pub struct RemoveTableStatement {
    table: Table,
}

impl RemoveTableStatement {
    fn new(table: impl Into<Table>) -> Self {
        Self {
            table: table.into(),
        }
    }
}

impl Buildable for RemoveTableStatement {
    fn build(&self) -> String {
        format!("REMOVE TABLE {}", self.table)
    }
}
impl Runnables for RemoveTableStatement {}

pub fn remove_event(event: impl Into<Event>) -> RemoveEventStatement {
    RemoveEventStatement::new(event)
}
pub struct RemoveEventStatement {
    event: Event,
    table: Option<Table>,
}

impl RemoveEventStatement {
    fn new(event: impl Into<Event>) -> Self {
        Self {
            table: None,
            event: event.into(),
        }
    }

    fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Buildable for RemoveEventStatement {
    fn build(&self) -> String {
        let query = format!("REMOVE EVENT {}", self.event);
        if let Some(table) = &self.table {
            let query = format!("{} ON TABLE {}", query, table);
        }
        query
    }
}
impl Runnables for RemoveEventStatement {}

pub fn remove_field(field: impl Into<Field>) -> RemoveFieldStatement {
    RemoveFieldStatement::new(field)
}
pub struct RemoveFieldStatement {
    field: Field,
    table: Option<Table>,
}

impl RemoveFieldStatement {
    fn new(field: impl Into<Field>) -> Self {
        Self {
            field: field.into(),
            table: None,
        }
    }

    fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Buildable for RemoveFieldStatement {
    fn build(&self) -> String {
        let query = format!("REMOVE FIELD {}", self.field);
        if let Some(table) = &self.table {
            let query = format!("{} ON TABLE {}", query, table);
        }
        query
    }
}
impl Runnables for RemoveFieldStatement {}

pub fn remove_index(index: impl Into<Index>) -> RemoveIndexStatement {
    RemoveIndexStatement::new(index)
}
pub struct RemoveIndexStatement {
    index: Index,
    table: Option<Table>,
}

impl RemoveIndexStatement {
    fn new(index: impl Into<Index>) -> Self {
        Self {
            index: index.into(),
            table: None,
        }
    }

    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Buildable for RemoveIndexStatement {
    fn build(&self) -> String {
        let query = format!("REMOVE INDEX {}", self.index);
        if let Some(table) = &self.table {
            let query = format!("{} ON TABLE {}", query, table);
        }
        query
    }
}
impl Runnables for RemoveIndexStatement {}

#[test]
fn test() {}
