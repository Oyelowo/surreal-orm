/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
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

/// Remove namespace statement
///
/// # Arguments
///
/// * `namespace` - The name of the namespace to be removed. Can be a string or a Namespace type.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::remove_namespace};
///
/// # let namespace = Namespace::new("namespace");
/// let statement = remove_namespace(namespace);
/// assert_eq!(statement.build(), "REMOVE NAMESPACE namespace;");
/// ```
pub fn remove_namespace(namespace: impl Into<Namespace>) -> RemoveNamespaceStatement {
    let namespace = namespace.into();
    RemoveNamespaceStatement { namespace }
}

/// Remove namespace statement
pub struct RemoveNamespaceStatement {
    namespace: Namespace,
}

impl Buildable for RemoveNamespaceStatement {
    fn build(&self) -> String {
        format!("REMOVE NAMESPACE {};", self.namespace)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_namespace() {
        let namespace = Namespace::new("namespace");
        let statement = remove_namespace(namespace);
        assert_eq!(statement.build(), "REMOVE NAMESPACE namespace;");
    }
}
