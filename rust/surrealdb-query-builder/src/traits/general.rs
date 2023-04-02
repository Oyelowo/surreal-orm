/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use crate::internal::replace_params;

use super::{Erroneous, Parametric};

pub trait Buildable {
    fn build(&self) -> String;

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

pub trait Queryable: Parametric + Buildable + Display + Erroneous {}

pub trait Conditional: Parametric + Buildable + Erroneous {
    fn get_condition_query_string(&self) -> String {
        self.build()
    }
}
