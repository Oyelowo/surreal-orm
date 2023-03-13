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
    query_remove::{RemoveScopeStatement, Runnable, Scope},
    query_select::SelectStatement,
    query_update::UpdateStatement,
    BindingsList, DbField, DbFilter, Parametric, Queryable,
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

pub enum DefineTokenTarget {
    Namespace,
    Database,
    Scope(String),
}
pub struct DefineTokenStatement {
    name: String,
    scope: Option<String>,
    token_type: TokenType,
    value: Option<String>,
    target: DefineTokenTarget,
    bindings: BindingsList,
}

impl DefineTokenStatement {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            scope: None,
            token_type: TokenType::HS512,
            value: Some("".to_string()),
            target: DefineTokenTarget::Namespace,
            bindings: todo!(),
        }
    }

    pub fn on_namespace(mut self) -> Self {
        self.target = DefineTokenTarget::Namespace;
        self
    }

    pub fn on_database(mut self, database: &str) -> Self {
        self.target = DefineTokenTarget::Database;
        self
    }

    pub fn on_scope(mut self, scope: Scope) -> Self {
        self.target = DefineTokenTarget::Scope(scope.to_string());
        self
    }

    pub fn type_(mut self, token_type: TokenType) -> Self {
        self.token_type = token_type;
        self
    }

    pub fn value(mut self, value: impl Into<sql::Strand>) -> Self {
        self.value = Some(value.into().to_string());
        self
    }
}
impl Buildable for DefineTokenStatement {
    fn build(&self) -> String {
        let target_str = match &self.target {
            TokenTarget::Namespace => "NAMESPACE",
            TokenTarget::Database => "DATABASE",
            TokenTarget::Scope(scope) => &format!("SCOPE {}", scope),
        };

        format!(
            "DEFINE TOKEN {} ON {} TYPE {} VALUE {};\n",
            self.name, target_str, self.token_type, self.value
        )
        todo!()
    }
}
// let statement = DefineTokenStatementBuilder::new("token_name")
//     .on_database("app_vitalsense")
//     .with_token_type(TokenType::HS512)
//     .with_value("sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8")
//     .build();

