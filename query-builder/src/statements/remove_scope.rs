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
    | USER @name ON [ ROOT | NAMESPACE | DATABASE ]
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
    types::Scope,
};

/// Remove scope statement
///
/// # Arguments
///
/// * `scope` - The name of the scope to be removed. Can be a string or a Scope type.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::remove_scope};
/// # let scope = Scope::new("scope");
/// let statement = remove_scope(scope);
/// assert_eq!(statement.build(), "REMOVE SCOPE scope;");
/// ```
pub fn remove_scope(scope: impl Into<Scope>) -> RemoveScopeStatement {
    RemoveScopeStatement::new(scope)
}

/// Remove scope statement
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
        format!("REMOVE SCOPE {};", self.scope)
    }
}

impl Display for RemoveScopeStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_remove_scope() {
        let scope = Scope::new("scope");
        let statement = remove_scope(scope);
        assert_eq!(statement.build(), "REMOVE SCOPE scope;");
    }
}
