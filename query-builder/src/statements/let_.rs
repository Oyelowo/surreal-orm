/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::expression::Expression,
    Clause, Conditional, Operatable, Operation, Param, SchemaGetter, ValueLike,
};

/// Builds LET statement.
///
/// The LET statement sets and stores a value which can then be used in a subsequent query.
/// A parameter can store any value, including the result of a query.
///
/// Examples
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::{let_, select}};
/// # let user = Table::new("user");
/// // You can assign a value surrealdb value
/// let_("name").equal_to(5);
///
/// // and even a select statement
/// let_("users").equal_to(select(All).from(user));
/// ```
pub fn let_(parameter: impl Into<Param>) -> LetStatement {
    let param: Param = parameter.into();
    LetStatement {
        value: None,
        bindings: vec![],
        parameter: param,
    }
}

#[macro_export]
/// Macro for creating a LET statement
///
/// Examples
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::{let_, select}};
/// # let ref user = Table::new("user");
/// // You can assign a value surrealdb value
/// let_!(name = 5);
/// // or
/// let_statement!(name = 5);
///
/// // and even a select statement
/// let_!(users = select(All).from(user));
/// // or
/// let_statement!(users = select(All).from(user));
/// ```
macro_rules! let_statement {
    ($param: ident =  $expr: expr) => {
        let $param = $crate::statements::let_(stringify!($param)).equal_to($expr);
    };
}

pub use let_statement as let_;

/// Let statement builder
#[derive(Debug, Clone)]
pub struct LetStatement {
    parameter: Param,
    value: Option<Expression>,
    bindings: BindingsList,
}

impl Operatable for LetStatement {
    fn generate_query<T>(&self, operator: impl std::fmt::Display, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        let value: ValueLike = value.into();
        let condition = format!(
            "{} {} {}",
            self.get_param().build(),
            operator,
            &value.build()
        );
        let updated_bindings = [
            self.get_bindings().as_slice(),
            value.get_bindings().as_slice(),
        ]
        .concat();
        Operation {
            query_string: condition,
            bindings: updated_bindings,
            errors: vec![],
        }
    }
}

impl LetStatement {
    /// Assigned expression
    /// Examples
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::{let_, select}};
    /// # let user = Table::new("user");
    /// // You can assign a value surrealdb value
    /// # let_("name")
    /// .equal_to(5);
    ///
    /// // and even a select statement
    /// # let_("users")
    /// .equal_to(select(All).from(user));
    pub fn equal_to(mut self, value: impl Into<Expression>) -> Self {
        let value: Expression = value.into();
        self.bindings.extend(value.get_bindings());
        self.value = Some(value);
        self
    }

    /// helper function for getting assigned param from the LET statement
    pub fn get_param(&self) -> Param {
        self.parameter.clone()
    }

    /// For traversing from the param
    pub fn with_path<T: SchemaGetter>(&self, clause: impl Into<Clause>) -> T::Schema {
        let clause: Clause = clause.into();
        let value = ValueLike {
            string: format!("{}{}", self.get_param().build(), clause.build()),
            bindings: self
                .get_bindings()
                .into_iter()
                .chain(clause.get_bindings())
                .collect::<Vec<_>>(),
            errors: self.get_errors(),
        };

        T::schema_prefixed(value)
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

impl Buildable for &LetStatement {
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
impl Parametric for &LetStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Queryable for LetStatement {}
impl Queryable for &LetStatement {}
impl Erroneous for LetStatement {}
impl Erroneous for &LetStatement {}

impl Conditional for &LetStatement {
    fn get_condition_query_string(&self) -> String {
        self.get_param().build()
    }
}

impl Conditional for LetStatement {
    fn get_condition_query_string(&self) -> String {
        self.get_param().build()
    }
}

#[cfg(test)]
mod tests {
    use crate::{statements::select, All, Table, ToRaw};

    use super::*;

    #[test]
    fn test_let_statement() {
        let statement = let_("name").equal_to(5);

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
        let statement = let_("name").equal_to(select(All).from(user));

        assert_eq!(
            statement.fine_tune_params(),
            "LET $name = $_param_00000001;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "LET $name = (SELECT * FROM user);"
        );

        assert_eq!(statement.get_param().build(), "$name");
    }
}
