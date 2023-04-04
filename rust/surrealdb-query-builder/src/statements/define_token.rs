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
    traits::{
        Binding, BindingsList, Buildable, Erroneous, Parametric, Queryable, Runnable, Runnable,  
    },
    types::{Database, Idiomx, Namespace, Scope, Table, Token, TokenTarget, TokenType},
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

pub struct DefineTokenStatement {
    name: String,
    scope: Option<String>,
    token_type: Option<TokenType>,
    value: Option<String>,
    target: Option<TokenTarget>,
    bindings: BindingsList,
}

pub fn define_token(token_name: impl Into<Idiomx>) -> DefineTokenStatement {
    DefineTokenStatement::new(token_name.into())
}

impl DefineTokenStatement {
    pub fn new(token_name: impl Into<Idiomx>) -> Self {
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

impl Runnable for  DefineTokenStatement {}
impl Erroneous for DefineTokenStatement {}

#[cfg(test)]
#[cfg(feature = "mock")]
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
