/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use surrealdb::sql::{self, Ident};

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable, Runnable},
    types::expression::Expression,
    Param,
};

pub fn let_(parameter: impl Into<Param>) -> LetStatement {
    LetStatement::new(parameter)
}

pub struct LetStatement {
    parameter: Param,
    value: Option<Expression>,
    bindings: BindingsList,
}

impl LetStatement {
    pub fn new(parameter: impl Into<Param>) -> Self {
        let param: Param = parameter.into();
        Self {
            value: None,
            bindings: vec![],
            parameter: param,
        }
    }
    pub fn equal(mut self, value: impl Into<Expression>) -> Self {
        let value: Expression = value.into();
        self.bindings.extend(value.get_bindings());
        self.value = Some(value);
        self
    }

    pub fn get_param(&self) -> Param {
        self.parameter.clone()
    }
}

impl Buildable for LetStatement {
    fn build(&self) -> String {
        let mut query = format!("LET {}", self.get_param());

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
#[cfg(feature = "mock")]
mod tests {

    use super::*;

    #[test]
    fn test_let_statement() {
        assert_eq!(
            let_("name").equal(5).build(),
            "LET $name = _param_00000000;"
        );

        assert_eq!(let_("name".to_string()).equal(5).get_param(), "$name");
    }
}
