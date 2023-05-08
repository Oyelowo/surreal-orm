/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use crate::{
    traits::{BindingsList, Buildable, Parametric},
    Aliasable, Erroneous,
};

/// Represents a subquery function.
#[derive(Debug, Clone)]
pub struct Function {
    pub(crate) query_string: String,
    pub(crate) bindings: BindingsList,
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
