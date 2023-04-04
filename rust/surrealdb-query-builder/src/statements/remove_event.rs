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

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::{Event, Table},
};

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
impl Display for RemoveEventStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveEventStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveEventStatement {}

impl Queryable for RemoveEventStatement {}
