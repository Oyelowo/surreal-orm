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
impl Erroneous for RemoveScopeStatement {}

impl Parametric for RemoveScopeStatement {
    fn get_bindings(&self) -> BindingsList {
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

impl Parametric for RemoveScopeStatement {}

impl Erroneous for RemoveScopeStatement {}

impl Runnables for RemoveScopeStatement {}

impl Runnables for RemoveScopeStatement {}
