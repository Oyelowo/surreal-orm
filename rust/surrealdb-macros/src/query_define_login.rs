/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{self, Display};

use insta::{assert_debug_snapshot, assert_display_snapshot};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::sql;

use crate::{
    field::{cond, Binding},
    query_create::CreateStatement,
    query_define_token::Name,
    query_delete::DeleteStatement,
    query_insert::{Buildable, InsertStatement},
    query_relate::RelateStatement,
    query_remove::{RemoveScopeStatement, Runnable},
    query_select::SelectStatement,
    query_update::UpdateStatement,
    BindingsList, Field, Filter, Parametric, Queryable,
};
// DEFINE LOGIN @name ON [ NAMESPACE | DATABASE ] [ PASSWORD @pass | PASSHASH @hash ]
// DEFINE LOGIN username ON NAMESPACE PASSWORD '123456';

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
        Self(value.into())
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

pub struct Password(String);
impl From<String> for Password {
    fn from(value: String) -> Self {
        Self(value.into())
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

// Define the API for the Login builder
pub struct DefineLoginStatement {
    name: String,
    login_type: Option<LoginType>,
    credential: Option<LoginCredential>,
    bindings: BindingsList,
}

pub fn define_login(name: impl Into<Name>) -> DefineLoginStatement {
    DefineLoginStatement::new(name)
}
impl DefineLoginStatement {
    // Set the login name
    fn new(name: impl Into<Name>) -> Self {
        let binding = Binding::new(name.into()).with_description("login name");
        Self {
            name: binding.get_param_dollarised(),
            login_type: None,
            credential: None,
            bindings: vec![binding],
        }
    }

    // Set the login type
    pub fn on_namespace(mut self) -> Self {
        self.login_type = Some(LoginType::Namespace);
        self
    }

    pub fn on_database(mut self) -> Self {
        self.login_type = Some(LoginType::Database);
        self
    }
    // Set the password credential
    pub fn password(mut self, password: impl Into<Password>) -> Self {
        let password: Password = password.into();
        let binding = Binding::new(password.0.clone()).with_description("login password");
        let password_param = format!("{}", binding.get_param_dollarised());
        self.bindings.push(binding);
        self.credential = Some(LoginCredential::Password(password_param.into()));
        self
    }

    // Set the passhash credential
    pub fn passhash(mut self, passhash: impl Into<Passhash>) -> Self {
        let passhash: Passhash = passhash.into();
        let binding = Binding::new(passhash.0.clone());
        let passhash_param = format!("{}", binding.get_param_dollarised());
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
impl Runnable for DefineLoginStatement {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_define_login_statement_with_password() {
        let login_with_password = define_login("username").on_database().password("oyelowo");

        assert_eq!(
            login_with_password.to_string(),
            "DEFINE LOGIN $_param_00000000 ON DATABASE PASSWORD $_param_00000000;" // "DEFINE LOGIN username ON DATABASE PASSWORD oyelowo"
        );
        insta::assert_debug_snapshot!(login_with_password.get_bindings());
    }

    #[test]
    fn test_define_login_statement_with_passhash() {
        let login_with_hash = define_login("username").on_namespace().password("oyedayo");

        assert_eq!(
            login_with_hash.to_string(),
            "DEFINE LOGIN $_param_00000000 ON NAMESPACE PASSWORD $_param_00000000;"
        );
        insta::assert_debug_snapshot!(login_with_hash.get_bindings());
    }
}
