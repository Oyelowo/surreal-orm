/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::Display;

use crate::{
    traits::{BindingsList, Buildable, Parametric},
    Aliasable, Erroneous, ErrorList,
};

/// Represents a subquery function.
#[derive(Debug, Clone)]
pub struct Function {
    pub(crate) query_string: String,
    pub(crate) bindings: BindingsList,
    pub(crate) errors: ErrorList,
}

impl Default for Function {
    fn default() -> Self {
        Self::new()
    }
}

impl Function {
    /// Creates a new function with the given query string.
    pub fn new() -> Self {
        Self {
            query_string: String::new(),
            bindings: vec![],
            errors: vec![],
        }
    }

    /// Creates a new function with the given query string.
    pub fn with_bindings(mut self, bindings: BindingsList) -> Self {
        self.bindings = bindings;
        self
    }

    /// Creates a new function with the given query string.
    pub fn with_args_string(mut self, query_string: String) -> Self {
        self.query_string = query_string;
        self
    }

    /// Gathers error for this function.
    pub fn with_errors(mut self, errors: ErrorList) -> Self {
        self.errors = errors;
        self
    }
}

impl Parametric for Function {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Buildable for Function {
    fn build(&self) -> String {
        self.query_string.to_string()
    }
}

impl Erroneous for Function {}

impl Aliasable for Function {}
