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
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable, Runnable, Runnables},
    types::{Database, Namespace, Scope, Table, TableIndex, Token},
};

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

impl Display for RemoveDatabaseStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveDatabaseStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveDatabaseStatement {}

impl Runnables for RemoveDatabaseStatement {}

impl Queryable for RemoveDatabaseStatement {}
