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
    binding::{BindingsList, Parametric},
    sql::{
        Buildable, Database, Event, Login, Namespace, Queryable, Runnables, Scope, Table,
        TableIndex, Token,
    },
    Erroneous, Field,
};

use super::NamespaceOrDatabase;

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

impl Display for RemoveLoginStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveLoginStatement {}

impl Erroneous for RemoveLoginStatement {}

impl Runnables for RemoveLoginStatement {}

impl Queryable for RemoveLoginStatement {}
