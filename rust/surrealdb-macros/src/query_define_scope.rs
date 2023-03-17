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
    query_define_token::Scope,
    query_delete::DeleteStatement,
    query_ifelse::Expression,
    query_insert::{Buildable, InsertStatement},
    query_relate::RelateStatement,
    query_remove::{RemoveScopeStatement, Runnable},
    query_select::{Duration, SelectStatement},
    query_update::UpdateStatement,
    BindingsList, Field, Filter, Parametric, Queryable,
};

// DEFINE SCOPE statement
// Setting scope access allows SurrealDB to operate as a web database. With scopes you can set authentication and access rules which enable fine-grained access to tables and fields.
//
// Requirements
// To useDEFINE SCOPE you must have root, namespace, or database level access.
// You must select your namespace and database before you can use the DEFINE SCOPE statement.
// Statement syntax
// DEFINE SCOPE @name SESSION @duration SIGNUP @expression SIGNIN @expression
// Example usage
// Below shows how you can create a namespace using the DEFINE NAMESPACE statement.
//
// -- Enable scope authentication directly in SurrealDB
// DEFINE SCOPE account SESSION 24h
// 	SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) )
// 	SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
// ;

// Define the API for the Scope builder
pub struct DefineScopeStatement {
    name: String,
    duration: Option<String>,
    signup_expression: Option<String>,
    signin_expression: Option<String>,
    bindings: BindingsList,
}

pub fn define_scope(scope_name: impl Into<Scope>) -> DefineScopeStatement {
    DefineScopeStatement::new(scope_name)
}

impl DefineScopeStatement {
    // Set the scope name
    pub fn new(scope_name: impl Into<Scope>) -> Self {
        let binding_scope_name = Binding::new(scope_name.into()).with_description("Session scope");
        let name = binding_scope_name.get_param_dollarised();
        Self {
            name,
            duration: None,
            signup_expression: None,
            signin_expression: None,
            bindings: vec![binding_scope_name],
        }
    }

    // Set the session duration
    pub fn session(mut self, duration: impl Into<Duration>) -> Self {
        let binding = Binding::new(duration.into()).with_description("Session durration.");
        let duration_param = format!("{}", binding.get_param_dollarised());
        self.bindings.push(binding);
        self.duration = Some(duration_param);
        self
    }

    // Set the signup expression
    pub fn signup(mut self, expression: impl Into<Expression>) -> Self {
        let expression: Expression = expression.into();
        let bindings = expression.get_bindings();
        self.bindings.extend(bindings);
        self.signup_expression = Some(format!("{expression}"));
        self
    }

    // Set the signin expression
    pub fn signin(mut self, expression: impl Into<Expression>) -> Self {
        let expression: Expression = expression.into();
        let bindings = expression.get_bindings();
        self.bindings.extend(bindings);
        self.signin_expression = Some(format!("{expression}"));
        self
    }
}

impl Buildable for DefineScopeStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE SCOPE {}", self.name);

        if let Some(session_duration) = &self.duration {
            query = format!("{query} SESSION {session_duration}");
        }

        if let Some(signup) = &self.signup_expression {
            query = format!("\n\t {query} SIGNUP ( {signup} )");
        }

        if let Some(signin) = &self.signin_expression {
            query = format!("\n\t {query} SIGNIN ( {signin} )");
        }

        query += ";";
        query
    }
}

impl Display for DefineScopeStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for DefineScopeStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Runnable for DefineScopeStatement {}

#[cfg(test)]
mod tests {

    use std::time::Duration;

    use super::*;

    #[test]
    fn test_define_scope_statement_on_namespace() {
        let token_def = define_scope("oyelowo_scope").session(Duration::from_secs(45));

        assert_eq!(
            token_def.to_string(),
            "DEFINE SCOPE $_param_00000000 SESSION $_param_00000000;"
        );
        insta::assert_debug_snapshot!(token_def.get_bindings());
    }
}
