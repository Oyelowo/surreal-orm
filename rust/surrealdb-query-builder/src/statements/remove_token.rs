/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

/*
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

use crate::{BindingsList, Buildable, Erroneous, Parametric, Queryable, Token};

use super::NamespaceOrDatabase;

/// Remove token statement
///
/// # Arguments
///
/// * `token` - The name of the token to be removed. Can be a string or a Token type.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, statements::remove_token};
/// let token = Token::new("token");
/// let statement = remove_token(token).on_namespace();
/// assert_eq!(statement.build(), "REMOVE TOKEN token ON NAMESPACE;");
///
/// let token = Token::new("token");
/// let statement = remove_token(token).on_database();
/// assert_eq!(statement.build(), "REMOVE TOKEN token ON DATABASE;");
pub fn remove_token(token: impl Into<Token>) -> RemoveTokenStatementInit {
    RemoveTokenStatementInit {
        token: token.into(),
        on: None,
    }
}

/// Remove token statement
pub struct RemoveTokenStatementInit {
    token: Token,
    on: Option<NamespaceOrDatabase>,
}

/// Remove token statement
pub struct RemoveTokenStatement(RemoveTokenStatementInit);

impl std::ops::Deref for RemoveTokenStatement {
    type Target = RemoveTokenStatementInit;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<RemoveTokenStatementInit> for RemoveTokenStatement {
    fn from(init: RemoveTokenStatementInit) -> Self {
        Self(init)
    }
}

impl RemoveTokenStatementInit {
    /// Specify to remove the token from namespace
    pub fn on_namespace(mut self) -> RemoveTokenStatement {
        self.on = Some(NamespaceOrDatabase::Namespace);
        self.into()
    }

    /// Specify to remove the token from database
    pub fn on_database(mut self) -> RemoveTokenStatement {
        self.on = Some(NamespaceOrDatabase::Database);
        self.into()
    }
}

impl Buildable for RemoveTokenStatement {
    fn build(&self) -> String {
        let mut query = format!("REMOVE TOKEN {}", self.token);

        if let Some(on) = &self.on {
            query = format!("{} ON {}", query, on);
        }
        format!("{};", query)
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

impl Queryable for RemoveTokenStatement {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn remove_token_on_namespace() {
        let login = Token::new("login");
        let statement = remove_token(login).on_namespace();
        assert_eq!(statement.build(), "REMOVE TOKEN login ON NAMESPACE;");
    }

    #[test]
    fn remove_token_on_database() {
        let login = Token::new("login");
        let statement = remove_token(login).on_database();
        assert_eq!(statement.build(), "REMOVE TOKEN login ON DATABASE;");
    }
}
