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
    Binding, ErrorList,
};

/// Creates a SLEEP statement.
///
/// Examples
///
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use std::time::Duration;
/// use surrealdb_orm::{*, statements::sleep};
///
/// sleep(Duration::from_secs(43));
pub fn sleep(duration: impl Into<DurationLike>) -> SleepStatement {
    let duration: sql::Value = duration.into().into();
    let binding = Binding::new(duration).with_description("Duration of sleep");

    SleepStatement {
        duration: binding.get_param_dollarised(),
        bindings: vec![binding],
    }
}

/// Sleep statement initialization builder
pub struct SleepStatement {
    duration: String,
    bindings: BindingsList,
}

impl Buildable for SleepStatement {
    fn build(&self) -> String {
        format!("SLEEP {};", self.duration)
    }
}

impl Queryable for SleepStatement {}
impl Erroneous for SleepStatement {}

impl Parametric for SleepStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
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

    use crate::ToRaw;

    use super::*;

    #[test]
    fn test_sleep_statement() {
        let statement = sleep(Duration::from_secs(43));
        assert_eq!(statement.fine_tune_params(), "SLEEP $_param_00000001;");
        assert_eq!(statement.to_raw().build(), "SLEEP 43s;");
        assert_eq!(statement.get_bindings().len(), 1);
    }
}
