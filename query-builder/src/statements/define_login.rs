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

// DEFINE LOGIN @name ON [ NAMESPACE | DATABASE ] [ PASSWORD @pass | PASSHASH @hash ]
// DEFINE LOGIN username ON NAMESPACE PASSWORD '123456';

/// Define a new database login.
/// Requirements
/// You must be authenticated as a root or Namespace user to create a Namespace level account using the DEFINE LOGIN statement.
/// You must be authenticated as a root, Namespace, or Database user to create a Database level account using the DEFINE LOGIN statement.
/// You must select your namespace and/or database before you can use the DEFINE LOGIN statement.
/// Note: You cannot use the DEFINE LOGIN statement to create a root or SCOPE user.
///
/// Examples:
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, CrudType::*, statements::{define_login}};
/// let username = Login::new("username");
///
/// let statement = define_login(username).on_database().password("oyelowo");
///
/// let statement = define_login("username")
///     .on_namespace()
///     .passhash("reiiereroyedayo");
///
/// assert!(!statement.build().is_empty());
/// ```
pub fn define_login(name: impl Into<Idiomx>) -> DefineLoginStatement {
    DefineLoginStatement::new(name)
}

// Define the types for the possible login types
pub enum LoginType {
    Namespace,
    Database,
}

impl Display for LoginType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            LoginType::Namespace => "NAMESPACE",
            LoginType::Database => "DATABASE",
        };
        write!(f, "{}", string)
    }
}

pub struct Passhash(String);

impl From<String> for Passhash {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Passhash {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl Display for Passhash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Password(pub(crate) String);

impl Password {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Password {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Define the types for the possible login credentials
pub enum LoginCredential {
    Password(Password),
    Passhash(Passhash),
}

/// Define the API for the Login builder
pub struct DefineLoginStatement {
    name: String,
    login_type: Option<LoginType>,
    credential: Option<LoginCredential>,
    bindings: BindingsList,
}

impl DefineLoginStatement {
    // Set the login name
    fn new(name: impl Into<Idiomx>) -> Self {
        let binding = Binding::new(name.into()).with_description("login name");
        Self {
            name: binding.get_param_dollarised(),
            login_type: None,
            credential: None,
            bindings: vec![binding],
        }
    }

    /// Set login on namespace
    pub fn on_namespace(mut self) -> Self {
        self.login_type = Some(LoginType::Namespace);
        self
    }

    /// Set login on database
    pub fn on_database(mut self) -> Self {
        self.login_type = Some(LoginType::Database);
        self
    }

    /// Set the password credential
    pub fn password(mut self, password: impl Into<Password>) -> Self {
        let password: Password = password.into();
        let binding = Binding::new(password.0.clone()).with_description("login password");
        let password_param = binding.get_param_dollarised();
        self.bindings.push(binding);
        self.credential = Some(LoginCredential::Password(password_param.into()));
        self
    }

    /// Set the passhash credential
    pub fn passhash(mut self, passhash: impl Into<Passhash>) -> Self {
        let passhash: Passhash = passhash.into();
        let binding = Binding::new(passhash.0.clone());
        let passhash_param = binding.get_param_dollarised();
        self.bindings.push(binding);
        self.credential = Some(LoginCredential::Passhash(passhash_param.into()));
        self
    }
}

impl Buildable for DefineLoginStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE LOGIN {}", self.name);

        if let Some(login_type) = &self.login_type {
            query.push_str(&format!(" ON {}", &login_type));
        }
        if let Some(credential) = &self.credential {
            match credential {
                LoginCredential::Password(password) => {
                    query.push_str(&format!(" PASSWORD {password}"));
                }
                LoginCredential::Passhash(hash) => {
                    query.push_str(&format!(" PASSHASH {hash}"));
                }
            };
        };
        query += ";";
        query
    }
}

impl Display for DefineLoginStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for DefineLoginStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Queryable for DefineLoginStatement {}

impl Erroneous for DefineLoginStatement {}

#[cfg(test)]
mod tests {
    use crate::{Login, ToRaw};

    use super::*;

    #[test]
    fn test_define_login_statement_with_password() {
        let username = Login::new("username");
        let login_with_password = define_login(username).on_database().password("oyelowo");

        assert_eq!(
            login_with_password.fine_tune_params(),
            "DEFINE LOGIN $_param_00000001 ON DATABASE PASSWORD $_param_00000002;"
        );

        assert_eq!(
            login_with_password.to_raw().build(),
            "DEFINE LOGIN username ON DATABASE PASSWORD 'oyelowo';"
        );
        assert_eq!(login_with_password.get_bindings().len(), 2);
    }

    #[test]
    fn test_define_login_statement_with_passhash() {
        let login_with_hash = define_login("username")
            .on_namespace()
            .passhash("reiiereroyedayo");

        assert_eq!(
            login_with_hash.fine_tune_params(),
            "DEFINE LOGIN $_param_00000001 ON NAMESPACE PASSHASH $_param_00000002;"
        );

        assert_eq!(
            login_with_hash.to_raw().build(),
            "DEFINE LOGIN username ON NAMESPACE PASSHASH 'reiiereroyedayo';"
        );

        assert_eq!(login_with_hash.get_bindings().len(), 2);
    }
}
