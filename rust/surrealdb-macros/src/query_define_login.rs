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
    db_field::{cond, Binding},
    query_create::CreateStatement,
    query_delete::DeleteStatement,
    query_insert::{Buildable, InsertStatement},
    query_relate::RelateStatement,
    query_remove::{RemoveScopeStatement, Runnable},
    query_select::SelectStatement,
    query_update::UpdateStatement,
    BindingsList, DbField, DbFilter, Parametric, Queryable,
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

pub struct Passhash(sql::Strand);

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
        write!(f, "{}", self)
    }
}

pub struct Password(sql::Strand);
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
        write!(f, "{}", self)
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

pub fn define_login(name: impl Into<sql::Strand>) -> DefineLoginStatement {
    DefineLoginStatement::new(name)
}
impl DefineLoginStatement {
    // Set the login name
    fn new(name: impl Into<sql::Strand>) -> Self {
        Self {
            name: name.into().into(),
            login_type: None,
            credential: None,
            bindings: vec![],
        }
    }

    // Set the login type
    pub fn on(mut self, login_type: LoginType) -> Self {
        self.login_type = Some(login_type);
        self
    }

    // Set the password credential
    pub fn password(mut self, password: impl Into<Password>) -> Self {
        self.credential = Some(LoginCredential::Password(password.into()));
        self
    }

    // Set the passhash credential
    pub fn passhash(mut self, passhash: impl Into<Passhash>) -> Self {
        self.credential = Some(LoginCredential::Passhash(passhash.into()));
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
        let login_with_password = define_login("username")
            .on(LoginType::Database)
            .password("oyelowo");

        assert_eq!(login_with_password.build(), "LET $name = _param_00000000;");
    }

    #[test]
    fn test_define_login_statement_with_passhash() {
        let login_with_hash = define_login("username")
            .on(LoginType::Namespace)
            .password("oyedayo");

        assert_eq!(login_with_hash.build(), "LET $name = _param_00000000;");
    }
}
