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
    BindingsList, Field, DbFilter, Parametric, Queryable,
};

// DEFINE TOKEN statement
// SurrealDB can work with third-party OAuth providers. Let's say that your provider issues your service a JWT once it's authenticated. By using the DEFINE TOKEN statement, you can set the public key needed to verify a JWT's authenticity.
//
// You can specify what TYPE of cryptographic signature algorithm your token uses. The following algorithms are supported:
// EDDSA, ES256, ES384, ES512, HS256, HS384, HS512, PS256, PS384, PS512, RS256, RS384, RS512.
//
// Requirements
// To DEFINE TOKEN ... ON NAMESPACE ... you must have root or namespace level access.
// To DEFINE TOKEN ... ON DATABASE ... you must have root, namespace, or database level access.
// To DEFINE TOKEN ... ON SCOPE ... you must have root, namespace, or database level access.
// You must select your namespace and/or database before you can use the DEFINE DATABASE statement for database or namespace tokens.
// Statement syntax
// DEFINE TOKEN @name ON [ NAMESPACE | DATABASE | SCOPE @scope ] TYPE @type VALUE @value
// -- Specify the namespace and database for the token
// USE NS abcum DB app_vitalsense;
//
// -- Set the name of the token
// DEFINE TOKEN token_name
//   -- Use this OAuth provider for database authorization
//   ON DATABASE
//   -- Specify the cryptographic signature algorithm used to sign the token
//   TYPE HS512
//   -- Specify the public key so we can verify the authenticity of the token
//   VALUE "sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8"
// ;

pub enum TokenType {
    EDDSA,
    ES256,
    ES384,
    ES512,
    HS256,
    HS384,
    HS512,
    PS256,
    PS384,
    PS512,
    RS256,
    RS384,
    RS512,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::EDDSA => write!(f, "EDDSA"),
            TokenType::ES256 => write!(f, "ES256"),
            TokenType::ES384 => write!(f, "ES384"),
            TokenType::ES512 => write!(f, "ES512"),
            TokenType::HS256 => write!(f, "HS256"),
            TokenType::HS384 => write!(f, "HS384"),
            TokenType::HS512 => write!(f, "HS512"),
            TokenType::PS256 => write!(f, "PS256"),
            TokenType::PS384 => write!(f, "PS384"),
            TokenType::PS512 => write!(f, "PS512"),
            TokenType::RS256 => write!(f, "RS256"),
            TokenType::RS384 => write!(f, "RS384"),
            TokenType::RS512 => write!(f, "RS512"),
        }
    }
}

pub enum TokenTarget {
    Namespace,
    Database,
    Scope(String),
}

impl Display for TokenTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let target_str = match self {
            TokenTarget::Namespace => "NAMESPACE".into(),
            TokenTarget::Database => "DATABASE".into(),
            TokenTarget::Scope(scope) => format!("SCOPE {}", scope),
        };
        write!(f, "{}", target_str)
    }
}

pub struct Name(sql::Idiom);

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl Name {
    pub fn new(name: sql::Idiom) -> Self {
        Self(name)
    }
}

// impl From<sql::Idiom> for Name {
//     fn from(value: sql::Idiom) -> Self {
//         todo!()
//     }
// }

impl From<Name> for sql::Idiom {
    fn from(value: Name) -> Self {
        value.0
    }
}

pub type Scope = Name;

impl From<Name> for sql::Value {
    fn from(value: Name) -> Self {
        value.0.into()
    }
}

impl<T> From<T> for Name
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into().into())
    }
}

pub struct DefineTokenStatement {
    name: String,
    scope: Option<String>,
    token_type: Option<TokenType>,
    value: Option<String>,
    target: Option<TokenTarget>,
    bindings: BindingsList,
}

pub fn define_token(token_name: impl Into<Name>) -> DefineTokenStatement {
    DefineTokenStatement::new(token_name.into())
}

impl DefineTokenStatement {
    pub fn new(token_name: impl Into<Name>) -> Self {
        let binding = Binding::new(token_name.into());
        Self {
            name: binding.get_param_dollarised().to_owned(),
            scope: None,
            token_type: None,
            value: None,
            target: None,
            bindings: vec![binding],
        }
    }

    pub fn on_namespace(mut self) -> Self {
        self.target = Some(TokenTarget::Namespace);
        self
    }

    pub fn on_database(mut self) -> Self {
        self.target = Some(TokenTarget::Database);
        self
    }

    pub fn on_scope(mut self, scope: impl Into<Scope>) -> Self {
        let binding = Binding::new(scope.into()).with_description("token definition scope");
        self.bindings.push(binding.clone());
        self.target = Some(TokenTarget::Scope(binding.get_param_dollarised()));
        self
    }

    pub fn type_(mut self, token_type: TokenType) -> Self {
        self.token_type = Some(token_type);
        self
    }

    pub fn value(mut self, value: impl Into<sql::Strand>) -> Self {
        let binding = Binding::new(value.into());
        self.bindings.push(binding.clone());
        self.value = Some(binding.get_param_dollarised().to_owned());

        self
    }
}

impl Buildable for DefineTokenStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE TOKEN {}", self.name);

        if let Some(target) = &self.target {
            query = format!("{query} ON {target}");
        }

        if let Some(ty) = &self.token_type {
            query = format!("{query} TYPE {ty}");
        }

        if let Some(value) = &self.value {
            query = format!("{query} VALUE {value}");
        }

        query += ";";
        query
    }
}

impl Display for DefineTokenStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for DefineTokenStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Runnable for DefineTokenStatement {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_define_token_statement_on_namespace() {
        let token_def = define_token("oyelowo_token")
            .on_namespace()
            .type_(TokenType::PS512)
            .value("abrakradabra");

        assert_eq!(
            token_def.to_string(),
            "DEFINE TOKEN $_param_00000000 ON NAMESPACE TYPE PS512 VALUE $_param_00000000;"
        );
        insta::assert_debug_snapshot!(token_def.get_bindings());
    }

    #[test]
    fn test_define_token_statement_on_database() {
        let token_def = define_token("oyelowo_token")
            .on_database()
            .type_(TokenType::HS512)
            .value("anaksunamun");

        assert_eq!(
            token_def.to_string(),
            "DEFINE TOKEN $_param_00000000 ON DATABASE TYPE HS512 VALUE $_param_00000000;"
        );
        insta::assert_debug_snapshot!(token_def.get_bindings());
    }

    #[test]
    fn test_define_token_statement_on_scope() {
        let token_def = define_token("oyelowo_token")
            .on_scope("planet")
            .type_(TokenType::EDDSA)
            .value("abcde");

        assert_eq!(
            token_def.to_string(),
            "DEFINE TOKEN $_param_00000000 ON SCOPE $_param_00000000 TYPE EDDSA VALUE $_param_00000000;"
        );
        insta::assert_debug_snapshot!(token_def.get_bindings());
    }
}
