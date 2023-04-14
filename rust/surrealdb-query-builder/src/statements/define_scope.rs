/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

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
use std::fmt::{self, Display};

use crate::{
    traits::{Binding, BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::{DurationLike, Scope},
};

/// Define a new scope .
/// Setting scope access allows SurrealDB to operate as a web database. With scopes you can set authentication and access rules which enable fine-grained access to tables and fields.
///
/// Requirements
/// To use DEFINE SCOPE you must have root, namespace, or database level access.
/// You must select your namespace and database before you can use the DEFINE SCOPE statement.
///
/// Examples:
///
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, CrudType::*, statements::{define_scope}};
/// let regional = Scope::new("regional");
///
/// let statement = define_scope(regional);
///
/// assert!(!statement.build().is_empty());
/// ```
pub fn define_scope(scope_name: impl Into<Scope>) -> DefineScopeStatement {
    let binding_scope_name = Binding::new(scope_name.into()).with_description("Session scope");
    let name = binding_scope_name.get_param_dollarised();
    DefineScopeStatement {
        name,
        duration: None,
        signup_expression: None,
        signin_expression: None,
        bindings: vec![binding_scope_name],
    }
}

/// Define the API for the Scope builder
pub struct DefineScopeStatement {
    name: String,
    duration: Option<String>,
    signup_expression: Option<String>,
    signin_expression: Option<String>,
    bindings: BindingsList,
}

impl DefineScopeStatement {
    /// Set the session duration
    pub fn session(mut self, duration: impl Into<DurationLike>) -> Self {
        let binding = Binding::new(duration.into()).with_description("Session durration.");
        let duration_param = format!("{}", binding.get_param_dollarised());
        self.bindings.push(binding);
        self.duration = Some(duration_param);
        self
    }

    /// Set the signup expression
    pub fn signup(mut self, expression: impl Queryable) -> Self {
        let bindings = expression.get_bindings();
        self.bindings.extend(bindings);
        self.signup_expression = Some(format!("{}", expression.build()));
        self
    }

    /// Set the signin expression
    pub fn signin(mut self, expression: impl Queryable) -> Self {
        let bindings = expression.get_bindings();
        self.bindings.extend(bindings);
        self.signin_expression = Some(format!("{}", expression.build()));
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

impl Queryable for DefineScopeStatement {}

impl Erroneous for DefineScopeStatement {}

#[cfg(test)]
mod tests {
    use crate::{
        statements::{create, define_scope},
        *,
    };
    use std::time::Duration;

    // SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) )
    // SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
    #[test]
    fn test_define_scope_statement_on_namespace() {
        let user_table = Table::new("user");
        let age = Field::new("age");
        let token_def = define_scope("oyelowo_scope")
            .session(Duration::from_secs(45))
            .signup(create(user_table).set(updater(age).equal(198)));

        assert_eq!(
            token_def.fine_tune_params(),
            "DEFINE SCOPE $_param_00000001 SESSION $_param_00000002;"
        );
        assert_eq!(
            token_def.to_raw().build(),
            "DEFINE SCOPE oyelowo_scope SESSION 45s;"
        );
        assert_eq!(token_def.get_bindings().len(), 2);
    }
}
