/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::{self, Display};

use surrealdb::sql;

use crate::{
    traits::{Binding, BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::{Idiomx, Scope, TokenTarget, TokenType},
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

/// Define the API for the Token builder
pub struct DefineTokenStatement {
    name: String,
    token_type: Option<TokenType>,
    value: Option<String>,
    target: Option<TokenTarget>,
    bindings: BindingsList,
}

/// Define a new token.
///
/// SurrealDB can work with third-party OAuth providers. Let's say that your provider issues your service a JWT once it's authenticated. By using the DEFINE TOKEN statement, you can set the public key needed to verify a JWT's authenticity.
///
/// You can specify what TYPE of cryptographic signature algorithm your token uses. The following algorithms are supported:
/// EDDSA, ES256, ES384, ES512, HS256, HS384, HS512, PS256, PS384, PS512, RS256, RS384, RS512.
///
/// Requirements
/// To DEFINE TOKEN ... ON NAMESPACE ... you must have root or namespace level access.
/// To DEFINE TOKEN ... ON DATABASE ... you must have root, namespace, or database level access.
/// To DEFINE TOKEN ... ON SCOPE ... you must have root, namespace, or database level access.
/// You must select your namespace and/or database before you can use the DEFINE DATABASE statement for database or namespace tokens.
///
/// Examples:
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::define_token};
/// let statement = define_token("oyelowo_token")
///             .on_namespace()
///             .type_(TokenType::PS512)
///             .value("abrakradabra");
/// assert!(!statement.build().is_empty());
///
/// let statement = define_token("oyelowo_token")
///             .on_database()
///             .type_(TokenType::EDDSA)
///             .value("abrakradabra");
/// assert!(!statement.build().is_empty());
///
/// let statement = define_token("oyelowo_token")
///             .on_scope("regional")
///             .type_(TokenType::HS256)
///             .value("abrakradabra");
/// assert!(!statement.build().is_empty());
/// ```
pub fn define_token(token_name: impl Into<Idiomx>) -> DefineTokenStatement {
    let token_name: Idiomx = token_name.into();

    DefineTokenStatement {
        name: token_name.to_string(),
        token_type: None,
        value: None,
        target: None,
        bindings: vec![],
    }
}

impl DefineTokenStatement {
    /// Define the token for the namespace
    pub fn on_namespace(mut self) -> Self {
        self.target = Some(TokenTarget::Namespace);
        self
    }

    /// Define the token for currentj database
    pub fn on_database(mut self) -> Self {
        self.target = Some(TokenTarget::Database);
        self
    }
    /// Use this OAuth provider for scope authorization
    pub fn on_scope(mut self, scope: impl Into<Scope>) -> Self {
        self.target = Some(TokenTarget::Scope(scope.into().into()));
        self
    }

    /// Specify the cryptographic signature algorithm used to sign the token
    pub fn type_(mut self, token_type: TokenType) -> Self {
        self.token_type = Some(token_type);
        self
    }

    /// Specify the public key so we can verify the authenticity of the token
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

impl Queryable for DefineTokenStatement {}

impl Erroneous for DefineTokenStatement {}

#[cfg(test)]
mod tests {
    use crate::ToRaw;

    use super::*;

    #[test]
    fn test_define_token_statement_on_namespace() {
        let statement = define_token("oyelowo_token")
            .on_namespace()
            .type_(TokenType::PS512)
            .value("abrakradabra");

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TOKEN oyelowo_token ON NAMESPACE TYPE PS512 VALUE $_param_00000001;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TOKEN oyelowo_token ON NAMESPACE TYPE PS512 VALUE 'abrakradabra';"
        );

        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_define_token_statement_on_database() {
        let statement = define_token("oyelowo_token")
            .on_database()
            .type_(TokenType::HS512)
            .value("anaksunamun");

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TOKEN oyelowo_token ON DATABASE TYPE HS512 VALUE $_param_00000001;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TOKEN oyelowo_token ON DATABASE TYPE HS512 VALUE 'anaksunamun';"
        );

        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_define_token_statement_on_scope() {
        let statement = define_token("oyelowo_token")
            .on_scope("planet")
            .type_(TokenType::EDDSA)
            .value("abcde");

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TOKEN oyelowo_token ON SCOPE planet TYPE EDDSA VALUE $_param_00000001;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TOKEN oyelowo_token ON SCOPE planet TYPE EDDSA VALUE 'abcde';"
        );

        assert_eq!(statement.get_bindings().len(), 1);
    }
}
