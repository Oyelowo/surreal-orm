/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::{statements::Subquery, Buildable, Erroneous, ErrorList, Parametric, ValueLike};

/// An expression is a value or statement that can be used within another query.
#[derive(Clone, Debug)]
pub enum Expression {
    Value(ValueLike),
    Subquery(Subquery),
}

impl Buildable for Expression {
    fn build(&self) -> String {
        match self {
            Expression::Value(value) => value.build(),
            Expression::Subquery(subquery) => subquery.build(),
        }
    }
}

impl Parametric for Expression {
    fn get_bindings(&self) -> Vec<crate::traits::Binding> {
        match self {
            Expression::Value(value) => value.get_bindings(),
            Expression::Subquery(subquery) => subquery.get_bindings(),
        }
    }
}

impl Erroneous for Expression {
    fn get_errors(&self) -> ErrorList {
        match self {
            Expression::Value(_value) => vec![],
            Expression::Subquery(subquery) => subquery.get_errors(),
        }
    }
}

impl<T> From<T> for Expression
where
    T: Into<ValueLike>,
{
    fn from(value: T) -> Self {
        Expression::Value(value.into())
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
        assert_eq!(expression.fine_tune_params(), "$_param_00000001");
        assert_eq!(expression.to_raw().build(), "(SELECT * FROM users)");
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
        assert_eq!(expression.to_raw().build(), "1.02f");
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
