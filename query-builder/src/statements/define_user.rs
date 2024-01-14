/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{self, Display};

use crate::{
    traits::{Binding, BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::Idiomx,
};

use super::define_login::Password;

/// Define a new database user.
/// Requirements
/// You must be authenticated with a user that has enough permissions. Only the OWNER built-in role grants permissions to create users.
/// You must be authenticated with a user that has permissions on the level where you are creating the user:
/// Root users can create Root, Namespace and Database users.
/// Namespace users can create Namespace and Database users
/// Database user can create Database users.
/// To select the level where you want to create the user, you may need to select a namespace and/or database before you can use the DEFINE USER statement for database or namespace tokens.
/// Note: You cannot use the DEFINE USER statement to create a SCOPE user.
///
/// Examples:
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, CrudType::*, statements::{define_user}};
/// let username = User::new("username");
///
/// let statement = define_user(username).on_root().password("123456").role(Role::Owner);
///
/// let statement = define_user("username")
///     .on_namespace()
///     .password("123456")
///     .role(Role::Viewer);
///
/// assert!(!statement.build().is_empty());
/// ```
pub fn define_user(name: impl Into<Idiomx>) -> DefineUserStatement {
    DefineUserStatement::new(name)
}

pub enum UserRole {
    Owner,
    Editor,
    Viewer,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            UserRole::Owner => "OWNER",
            UserRole::Editor => "EDITOR",
            UserRole::Viewer => "VIEWER",
        };
        write!(f, "{string}")
    }
}

pub enum UserType {
    Root,
    Namespace,
    Database,
}

impl Display for UserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            UserType::Root => "ROOT",
            UserType::Namespace => "NAMESPACE",
            UserType::Database => "DATABASE",
        };
        write!(f, "{string}")
    }
}

/// Define a new database user.
pub struct DefineUserStatement {
    name: String,
    user_type: Option<UserType>,
    password: Option<Password>,
    role: Option<UserRole>,
    bindings: BindingsList,
}

impl DefineUserStatement {
    // Set the user name
    fn new(name: impl Into<Idiomx>) -> Self {
        let binding = Binding::new(name.into()).with_description("user name");
        Self {
            name: binding.get_param_dollarised(),
            user_type: None,
            password: None,
            role: None,
            bindings: vec![binding],
        }
    }

    /// Set user on root
    pub fn on_root(mut self) -> Self {
        self.user_type = Some(UserType::Root);
        self
    }

    /// Set user on namespace
    pub fn on_namespace(mut self) -> Self {
        self.user_type = Some(UserType::Namespace);
        self
    }

    /// Set user on database
    pub fn on_database(mut self) -> Self {
        self.user_type = Some(UserType::Database);
        self
    }

    /// Set the password
    pub fn password(mut self, password: impl Into<Password>) -> Self {
        let password: Password = password.into();
        let binding = Binding::new(password.into_inner()).with_description("user password");
        let password_param = binding.get_param_dollarised();
        self.bindings.push(binding);
        self.password = Some(password_param.into());
        self
    }

    /// Set the user role
    pub fn role(mut self, role: UserRole) -> Self {
        self.role = Some(role);
        self
    }
}

impl Buildable for DefineUserStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE USER {}", self.name);

        if let Some(user_type) = &self.user_type {
            query.push_str(&format!(" ON {user_type}"));
        }
        if let Some(password) = &self.password {
            query.push_str(&format!(" PASSWORD {password}"));
        }
        if let Some(role) = &self.role {
            query.push_str(&format!(" ROLES {role}"));
        }
        query += ";";
        query
    }
}

impl Display for DefineUserStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for DefineUserStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Queryable for DefineUserStatement {}

impl Erroneous for DefineUserStatement {}

#[cfg(test)]
mod tests {
    use crate::{ToRaw, User};

    use super::*;

    #[test]
    fn test_define_user_statement_with_password_and_role() {
        let username = User::new("username");
        let user_with_password_and_role = define_user(username)
            .on_root()
            .password("123456")
            .role(UserRole::Owner);

        assert_eq!(
            user_with_password_and_role.fine_tune_params(),
            "DEFINE USER $_param_00000001 ON ROOT PASSWORD $_param_00000002 ROLES OWNER;"
        );

        assert_eq!(
            user_with_password_and_role.to_raw().build(),
            "DEFINE USER username ON ROOT PASSWORD '123456' ROLES OWNER;"
        );
        assert_eq!(user_with_password_and_role.get_bindings().len(), 2);
    }

    #[test]
    fn test_define_user_statement_with_namespace_and_editor_role() {
        let user_with_namespace_and_editor = define_user("username")
            .on_namespace()
            .password("123456")
            .role(UserRole::Editor);

        assert_eq!(
            user_with_namespace_and_editor.fine_tune_params(),
            "DEFINE USER $_param_00000001 ON NAMESPACE PASSWORD $_param_00000002 ROLES EDITOR;"
        );

        assert_eq!(
            user_with_namespace_and_editor.to_raw().build(),
            "DEFINE USER username ON NAMESPACE PASSWORD '123456' ROLES EDITOR;"
        );

        assert_eq!(user_with_namespace_and_editor.get_bindings().len(), 2);
    }

    #[test]
    fn test_define_user_statement_with_database_and_viewer_role() {
        let user_with_database_and_viewer = define_user("username")
            .on_database()
            .password("123456")
            .role(UserRole::Viewer);

        assert_eq!(
            user_with_database_and_viewer.fine_tune_params(),
            "DEFINE USER $_param_00000001 ON DATABASE PASSWORD $_param_00000002 ROLES VIEWER;"
        );

        assert_eq!(
            user_with_database_and_viewer.to_raw().build(),
            "DEFINE USER username ON DATABASE PASSWORD '123456' ROLES VIEWER;"
        );

        assert_eq!(user_with_database_and_viewer.get_bindings().len(), 2);
    }
}
