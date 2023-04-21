/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{statements::SelectStatement, Buildable, Parametric, Valuex};

/// An expression is a value or statement that can be used within another query.
#[derive(Clone, Debug)]
pub struct Expression(Valuex);

impl std::ops::Deref for Expression {
    type Target = Valuex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Expression
where
    T: Into<Valuex>,
{
    fn from(value: T) -> Self {
        Expression(value.into())
    }
}

impl From<SelectStatement> for Expression {
    fn from(select_statement: SelectStatement) -> Self {
        Expression(Valuex {
            // statement already bound
            string: format!("( {} )", select_statement.build().trim_end_matches(";")),
            bindings: select_statement.get_bindings(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{statements::select, All, Table, ToRaw, NULL};

    use super::*;

    #[test]
    fn expression_from_select_statement() {
        let users = Table::new("users");
        let select_statement = select(All).from(users);
        let expression = Expression::from(select_statement);
        assert_eq!(expression.fine_tune_params(), "( SELECT * FROM users )");
        assert_eq!(expression.to_raw().build(), "( SELECT * FROM users )");
    }

    #[test]
    fn expression_from_string() {
        let expression = Expression::from("hello");
        assert_eq!(expression.fine_tune_params(), "$_param_00000001");
        assert_eq!(expression.to_raw().build(), "'hello'");
    }

    #[test]
    fn expression_from_integer() {
        let expression = Expression::from(1);
        assert_eq!(expression.fine_tune_params(), "$_param_00000001");
        assert_eq!(expression.to_raw().build(), "1");
    }

    #[test]
    fn expression_from_float() {
        let expression = Expression::from(1.02);
        assert_eq!(expression.fine_tune_params(), "$_param_00000001");
        assert_eq!(expression.to_raw().build(), "1.02");
    }

    #[test]
    fn expression_from_boolean() {
        let expression = Expression::from(true);
        assert_eq!(expression.fine_tune_params(), "$_param_00000001");
        assert_eq!(expression.to_raw().build(), "true");
    }

    #[test]
    fn expression_from_null() {
        let expression = Expression::from(NULL);
        assert_eq!(expression.fine_tune_params(), "NULL");
        assert_eq!(expression.to_raw().build(), "NULL");
    }
}
