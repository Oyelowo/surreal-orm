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

impl Display for RemoveTableStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveTableStatement {}

impl Erroneous for RemoveTableStatement {}

impl Runnables for RemoveTableStatement {}

impl Queryable for RemoveTableStatement {}
