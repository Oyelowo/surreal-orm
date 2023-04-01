/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{Display, Formatter};

use super::{BindingsList, Buildable, Erroneous, Parametric, Queryable};

#[derive(Debug, Clone)]
pub struct Raw(String);

impl Raw {
    pub fn new(query: String) -> Self {
        Self(query)
    }
}

impl Parametric for Raw {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for Raw {}

impl Queryable for Raw {}

impl Display for Raw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Buildable for Raw {
    fn build(&self) -> String {
        self.0.to_string()
    }
}

pub trait ToRawStatement
where
    Self: Sized,
{
    fn to_raw(self) -> Raw;
}

impl<T> ToRawStatement for T
where
    T: Parametric + Buildable,
{
    fn to_raw(self) -> Raw {
        let query_raw =
            self.get_bindings()
                .into_iter()
                .fold(self.build(), |query_parametized, binding| {
                    query_parametized.replace(
                        binding.get_param_dollarised().as_str(),
                        binding.get_raw_value().as_str(),
                    )
                });

        Raw(query_raw)
    }
}
