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
    | User @name ON [ NAMESPACE | DATABASE ]
    | USER @name ON [ ROOT | NAMESPACE | DATABASE ]
    | SCOPE @name
    | TABLE @name
    | EVENT @name ON [ TABLE ] @table
    | FIELD @name ON [ TABLE ] @table
    | INDEX @name ON [ TABLE ] @table
]
 * */

use std::fmt::{self, Display};

use crate::{BindingsList, Buildable, Erroneous, Parametric, Queryable, User};

/// Remove user statement
///
/// # Arguments
///
/// * `user` - User to be removed. Can be a string or a User type.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::remove_User};
/// let User = User::new("oyelowo");
/// let statement = remove_user(user).on_namespace();
/// assert_eq!(statement.build(), "REMOVE USER User ON NAMESPACE;");
///
/// let User = User::new("oyelowo");
/// let statement = remove_user(user).on_database();
/// assert_eq!(statement.build(), "REMOVE User User ON DATABASE;");
/// ```
pub fn remove_user(user: impl Into<User>) -> RemoveUserStatementInit {
    RemoveUserStatementInit {
        user: user.into(),
        on: None,
    }
}

#[allow(missing_docs)]
pub enum UserScopeSpace {
    Namespace,
    Database,
    Root,
}

impl Display for UserScopeSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UserScopeSpace::Namespace => "NAMESPACE",
                UserScopeSpace::Database => "DATABASE",
                UserScopeSpace::Root => "ROOT",
            }
        )
    }
}

/// Remove User statement
pub struct RemoveUserStatementInit {
    user: User,
    on: Option<UserScopeSpace>,
}

/// Remove User statement
pub struct RemoveUserStatement(RemoveUserStatementInit);

impl std::ops::Deref for RemoveUserStatement {
    type Target = RemoveUserStatementInit;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<RemoveUserStatementInit> for RemoveUserStatement {
    fn from(init: RemoveUserStatementInit) -> Self {
        Self(init)
    }
}

impl RemoveUserStatementInit {
    /// Specify to remove the User from root
    pub fn on_root(mut self) -> RemoveUserStatement {
        self.on = Some(UserScopeSpace::Root);
        self.into()
    }

    /// Specify to remove the User from namespace
    pub fn on_namespace(mut self) -> RemoveUserStatement {
        self.on = Some(UserScopeSpace::Namespace);
        self.into()
    }

    /// Specify to remove the User from database
    pub fn on_database(mut self) -> RemoveUserStatement {
        self.on = Some(UserScopeSpace::Database);
        self.into()
    }
}

impl Buildable for RemoveUserStatement {
    fn build(&self) -> String {
        let mut query = format!("REMOVE USER {}", self.user);

        if let Some(on) = &self.on {
            query = format!("{} ON {}", query, on);
        }
        format!("{};", query)
    }
}
impl Display for RemoveUserStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveUserStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveUserStatement {}

impl Queryable for RemoveUserStatement {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn remove_user_on_namespace() {
        let login = User::new("login");
        let statement = remove_user(login).on_namespace();
        assert_eq!(statement.build(), "REMOVE User login ON NAMESPACE;");
    }

    #[test]
    fn remove_user_on_database() {
        let login = User::new("login");
        let statement = remove_user(login).on_database();
        assert_eq!(statement.build(), "REMOVE User login ON DATABASE;");
    }
}
