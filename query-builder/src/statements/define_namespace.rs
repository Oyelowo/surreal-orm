/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::Display;

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::Namespace,
};

/// Define a new namespace .
/// SurrealDB has a multi-tenancy model which allows you to scope databases to a namespace.
/// There is no limit to the number of databases that can be in a namespace,
/// nor is there a limit to the number of namespaces allowed. Only users root users are
/// authorized to create namespaces.
///
/// Let's say that you're using SurrealDB to create a multi-tenant SaaS application. You can guarantee that the data of each tenant will be kept separate from other tenants if you put each tenant's databases into separate namespaces. In other words, this will ensure that information will remain siloed so user will only have access the information in the namespace they are a member of.
///
/// Requirements
/// You must be authenticated as a root user to use the DEFINE NAMESPACE statement.
///
/// Examples:
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, CrudType::*, statements::{define_namespace}};
/// let oyelowo = Namespace::new("oyelowo");
///
/// let statement = define_namespace(oyelowo);
///
/// assert!(!statement.build().is_empty());
/// ```
pub fn define_namespace(namespace: impl Into<Namespace>) -> DefineNamespaceStatement {
    DefineNamespaceStatement {
        namespace: namespace.into().into(),
        bindings: vec![],
    }
}

/// Define namespace
pub struct DefineNamespaceStatement {
    namespace: String,
    bindings: BindingsList,
}

impl Buildable for DefineNamespaceStatement {
    fn build(&self) -> String {
        format!("DEFINE NAMESPACE {};", self.namespace)
    }
}

impl Display for DefineNamespaceStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for DefineNamespaceStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Queryable for DefineNamespaceStatement {}
impl Erroneous for DefineNamespaceStatement {}

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
