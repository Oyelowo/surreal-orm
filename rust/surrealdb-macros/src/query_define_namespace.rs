/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use surrealdb::sql::{self, Ident};

use crate::{
    db_field::Binding,
    query_ifelse::Expression,
    query_insert::Buildable,
    query_remove::{Database, Namespace, Runnable},
    query_select::Duration,
    BindingsList, Parametric, Queryable,
};

pub fn define_namespace(namespace: impl Into<Namespace>) -> DefineNamespaceStatement {
    DefineNamespaceStatement::new(namespace)
}

// DEFINE NAMESPACE @name
pub struct DefineNamespaceStatement {
    namespace: String,
    bindings: BindingsList,
}

// Musings: Perhaps, definitions should not be parametized
impl DefineNamespaceStatement {
    pub fn new(namespace: impl Into<Namespace>) -> Self {
        Self {
            namespace: namespace.into().into(),
            bindings: vec![],
        }
    }
}

impl Buildable for DefineNamespaceStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE NAMESPACE {};", self.namespace);

        query
    }
}

impl Display for DefineNamespaceStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Runnable for DefineNamespaceStatement {}

impl Parametric for DefineNamespaceStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Queryable for DefineNamespaceStatement {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_let_statement() {
        assert_eq!(
            define_namespace("oyelowo").build(),
            "DEFINE NAMESPACE oyelowo;"
        );
    }
}
