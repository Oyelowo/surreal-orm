/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use super::{Erroneous, Parametric};

/// A trait for building a query string
pub trait Buildable {
    /// Build a query string
    fn build(&self) -> String;

    /// Make query string param consistent. Useful in testing.
    fn fine_tune_params(&self) -> String {
        // replace_params(&self.build())
        let mut count = 0;
        let re = regex::Regex::new(r"_param_[[:xdigit:]]+").unwrap();
        re.replace_all(&self.build(), |caps: &regex::Captures<'_>| {
            count += 1;
            format!("_param_{:08}", count)
        })
        .to_string()
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
