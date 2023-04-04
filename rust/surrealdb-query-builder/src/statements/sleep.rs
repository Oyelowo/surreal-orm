/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use surrealdb::sql;

use crate::{
    traits::BindingsList,
    traits::{Buildable, Erroneous, Parametric, Queryable},
    types::DurationLike,
};

pub fn sleep(duration: impl Into<DurationLike>) -> SleepStatement {
    SleepStatement::new(duration)
}

pub struct SleepStatement(String);

impl SleepStatement {
    fn new(duration: impl Into<DurationLike>) -> Self {
        let duration: sql::Value = duration.into().into();
        // let duration = sql::Duration::from(duration);
        Self(duration.to_string())
    }
}
impl Buildable for SleepStatement {
    fn build(&self) -> String {
        format!("SLEEP {};", self.0)
    }
}

impl Queryable for SleepStatement {}
impl Erroneous for SleepStatement {}

impl Parametric for SleepStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Display for SleepStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {

    use std::time::Duration;

    use super::*;

    #[test]
    fn test_sleep_statement() {
        assert_eq!(sleep(Duration::from_secs(43)).build(), "SLEEP 43s;");
    }
}
