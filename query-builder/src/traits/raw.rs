/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::{Display, Formatter};

use super::{BindingsList, Buildable, Erroneous, Parametric, Queryable};

/// A raw query which can usually be converted into from a `Parametric` query.
/// This is useful for debugging purposes.
#[derive(Debug, Clone)]
pub struct Raw(String);

impl Raw {
    /// Creates a new `Raw` query.
    pub fn new(query: impl Into<String>) -> Self {
        Self(query.into())
    }

    /// Returns true if the query is empty.
    pub fn is_empty(&self) -> bool {
        self.build().is_empty()
    }
}

impl Parametric for Raw {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for Raw {}

impl Queryable for Raw {}

impl Buildable for Raw {
    fn build(&self) -> String {
        self.0.to_string()
    }
}

impl Display for Raw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

/// A trait to convert a `Parametric` query into a `Raw` query.
pub trait ToRaw
where
    Self: Sized,
{
    /// Converts a `Parametric` query into a `Raw` query.
    fn to_raw(&self) -> Raw;
}

impl<T> ToRaw for T
where
    T: Parametric + Buildable,
{
    fn to_raw(&self) -> Raw {
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
