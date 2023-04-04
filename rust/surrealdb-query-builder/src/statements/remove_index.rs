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
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::{Database, Namespace, Scope, Table, TableIndex, Token},
};

pub fn remove_index(index: impl Into<TableIndex>) -> RemoveIndexStatement {
    RemoveIndexStatement::new(index)
}
pub struct RemoveIndexStatement {
    index: TableIndex,
    table: Option<Table>,
}

impl RemoveIndexStatement {
    fn new(index: impl Into<TableIndex>) -> Self {
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

impl Display for RemoveIndexStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveIndexStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveIndexStatement {}

impl Queryable for RemoveIndexStatement {}

#[test]
#[cfg(feature = "mock")]
fn test() {}
