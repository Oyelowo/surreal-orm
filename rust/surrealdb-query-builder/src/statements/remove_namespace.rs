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
    types::Namespace,
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

impl Display for RemoveNamespaceStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveNamespaceStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveNamespaceStatement {}

impl Queryable for RemoveNamespaceStatement {}
