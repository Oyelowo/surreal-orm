/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::Field;

use super::{Erroneous, Parametric};

/// A trait for building a query string
pub trait Buildable {
    /// Build a query string
    fn build(&self) -> String;

    /// Make query string param consistent. Useful in testing.
    fn fine_tune_params(&self) -> String {
        let mut count = 0;
        let re = regex::Regex::new(r"_param_[[:xdigit:]]+").unwrap();
        re.replace_all(&self.build(), |_caps: &regex::Captures<'_>| {
            count += 1;
            format!("_param_{:08}", count)
        })
        .to_string()
    }
}

/// Denoted by `.*`. Used for accessing all nested fields and arrays and links
pub trait AllGetter {
    /// Appends `.*` to the current string. Get all nested fields and arrays and links
    fn all(&self) -> Field;
}

impl<B> AllGetter for B
where
    B: Buildable + Parametric,
{
    fn all(&self) -> Field {
        let asteriked = format!("{}.*", self.build());
        Field::new(asteriked.clone()).with_bindings(self.get_bindings())
    }
}

/// Used for statements
pub trait Queryable: Parametric + Buildable + Erroneous {}

/// Used for filters
pub trait Conditional: Parametric + Buildable + Erroneous {
    /// Get query string of the filter
    fn get_condition_query_string(&self) -> String {
        self.build()
    }
}

/// Used for marking a struct used in UPDATE MERGE statement.
pub trait DataUpdater {}
