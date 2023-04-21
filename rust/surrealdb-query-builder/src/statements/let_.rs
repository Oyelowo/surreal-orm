/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::expression::Expression,
    Param,
};

/// Builds LET statement.
///
/// The LET statement sets and stores a value which can then be used in a subsequent query.
/// A parameter can store any value, including the result of a query.
///
/// Examples
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, statements::{let_, select}};
/// # let user = Table::new("user");
/// // You can assign a value surrealdb value
/// let_("name").equal(5);
///
/// // and even a select statement
/// let_("users").equal(select(All).from(user));
pub fn let_(parameter: impl Into<Param>) -> LetStatement {
    let param: Param = parameter.into();
    LetStatement {
        value: None,
        bindings: vec![],
        parameter: param,
    }
}

/// Let statement builder
pub struct LetStatement {
    parameter: Param,
    value: Option<Expression>,
    bindings: BindingsList,
}

impl LetStatement {
    /// Assigned expression
    /// Examples
    /// ```rust
    /// # use surrealdb_query_builder as surrealdb_orm;
    /// use surrealdb_orm::{*, statements::{let_, select}};
    /// # let user = Table::new("user");
    /// // You can assign a value surrealdb value
    /// # let_("name")
    /// .equal(5);
    ///
    /// // and even a select statement
    /// # let_("users")
    /// .equal(select(All).from(user));
    pub fn equal(mut self, value: impl Into<Expression>) -> Self {
        let value: Expression = value.into();
        self.bindings.extend(value.get_bindings());
        self.value = Some(value);
        self
    }

    /// helper function for getting assigned param from the LET statement
    pub fn get_param(&self) -> Param {
        self.parameter.clone()
    }
}

impl Buildable for LetStatement {
    fn build(&self) -> String {
        let mut query = format!("LET {}", self.get_param().build());

        if let Some(value) = &self.value {
            query = format!("{query} = {};", value.build());
        }

        query
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for LetStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Queryable for LetStatement {}
impl Erroneous for LetStatement {}

#[cfg(test)]
mod tests {
    use crate::{statements::select, All, Table, ToRaw};

    use super::*;

    #[test]
    fn test_let_statement() {
        let statement = let_("name").equal(5);

        assert_eq!(
            statement.fine_tune_params(),
            "LET $name = $_param_00000001;"
        );

        assert_eq!(statement.to_raw().build(), "LET $name = 5;");

        assert_eq!(statement.get_param().build(), "$name");
    }

    #[test]
    fn test_let_statement_with_select_statement() {
        let user = Table::new("user");
        let statement = let_("name").equal(select(All).from(user));

        assert_eq!(
            statement.fine_tune_params(),
            "LET $name = ( SELECT * FROM user );"
        );

        assert_eq!(
            statement.to_raw().build(),
            "LET $name = ( SELECT * FROM user );"
        );

        assert_eq!(statement.get_param().build(), "$name");
    }
}
