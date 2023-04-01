/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use super::{Erroneous, Parametric};

pub trait Buildable {
    fn build(&self) -> String;

    fn fine_tune_params(&self) -> String {
        replace_params(&self.build())
    }
}

pub trait Queryable: Parametric + Buildable + Display + Erroneous {}

pub trait Conditional: Parametric + Buildable + Erroneous {
    fn get_condition_query_string(&self) -> String {
        self.build()
    }
}
