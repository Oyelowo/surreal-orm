/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use surrealdb::sql;

use crate::traits::{Buildable, Erroneous, Operatable, Parametric};

use super::Idiomx;

pub struct Param {
    param: sql::Param,
    bindings: BindingsList,
}

impl From<Param> for sql::Value {
    fn from(value: Param) -> Self {
        value.param.into()
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
    pub fn new(param: impl Into<Idiomx>) -> Self {
        let param: Idiomx = param.into();
        let param = sql::Idiom::from(param);
        let param = sql::Param::from(param);

        Self {
            param,
            bindings: vec![].into(),
        }
    }
}

impl Operatable for Param {}
