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
    types::{Database, Field, Namespace, Scope, Table, TableIndex, Token},
};

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

impl Display for RemoveFieldStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveFieldStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveFieldStatement {}

impl Runnables for RemoveFieldStatement {}

impl Queryable for RemoveFieldStatement {}
