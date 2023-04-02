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
    types::{Database, Namespace, Token},
};

use super::NamespaceOrDatabase;

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
impl Display for RemoveTokenStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveTokenStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveTokenStatement {}

impl Runnables for RemoveTokenStatement {}

impl Queryable for RemoveTokenStatement {}
