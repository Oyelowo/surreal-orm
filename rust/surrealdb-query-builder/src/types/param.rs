/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use surrealdb::sql;

use crate::{BindingsList, Buildable, Erroneous, Operatable, Parametric};

/// Represents a surrogate parameter
#[derive(Debug, Clone)]
pub struct Param {
    param: sql::Param,
    bindings: BindingsList,
}

impl<T> From<T> for Param
where
    T: Into<sql::Param>,
{
    fn from(value: T) -> Self {
        let param: sql::Param = value.into();
        Self {
            param,
            bindings: vec![],
        }
    }
}

impl Erroneous for Param {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl Buildable for Param {
    fn build(&self) -> String {
        self.param.to_string()
    }
}

impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for Param {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Param {
    /// Creates a new instance of `Param`
    pub fn new(param: impl Into<sql::Param>) -> Self {
        let param = sql::Param::from(param.into());

        Self {
            param,
            bindings: vec![].into(),
        }
    }
}

impl Operatable for Param {}
