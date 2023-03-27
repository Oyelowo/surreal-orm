/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Sleep function
// The SLEEP function is used to introduce a delay or pause in the execution of a query or a batch of queries for a specific amount of time.
//
// Function	Description
// sleep(@duration)	Delays or pauses in the execution of a query or a batch of queries.

use core::time;

use surrealdb::sql;

use crate::{
    sql::{Binding, Buildable, ToRawStatement},
    Field,
};

use super::array::Function;

struct Duration(sql::Value);

impl From<Field> for self::Duration {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}

fn sleep(duration: impl Into<sql::Duration>) -> Function {
    let value: sql::Value = duration.into().into();
    let binding = Binding::new(value);

    Function {
        query_string: format!("sleep({})", binding.get_param_dollarised()),
        bindings: vec![binding],
    }
}

#[test]
fn test_sleep_fn() {
    let result = sleep(time::Duration::from_secs(55));
    assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "sleep(55s)");
}

#[test]
fn test_sleep_fn_over_long_period() {
    let result = sleep(time::Duration::from_secs(55340223));
    assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "sleep(1y39w2d12h17m3s)");
}
