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

use crate::traits::{Binding, Buildable, ToRaw};

use crate::types::{DurationLike, Field, Function};

fn sleep_fn(duration: impl Into<DurationLike>) -> Function {
    let value: sql::Value = duration.into().into();
    let binding = Binding::new(value);

    Function {
        query_string: format!("sleep({})", binding.get_param_dollarised()),
        bindings: vec![binding],
    }
}

#[macro_export]
macro_rules! sleep {
    ( $duration:expr ) => {
        crate::functions::sleep::sleep_fn($duration)
    };
}

pub use sleep;

#[test]
fn test_sleep_fn_with_field_data() {
    let waiting_time = Field::new("waiting_time");
    let result = sleep_fn(waiting_time);

    assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "sleep(waiting_time)");
}

#[test]
fn test_sleep_fn() {
    let result = sleep_fn(time::Duration::from_secs(55));
    assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "sleep(55s)");
}

#[test]
fn test_sleep_fn_over_long_period() {
    let result = sleep_fn(time::Duration::from_secs(55340223));
    assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "sleep(1y39w2d12h17m3s)");
}

// macro versions
#[test]
fn test_sleep_macro_with_field_data() {
    let waiting_time = Field::new("waiting_time");
    let result = sleep!(waiting_time);

    assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "sleep(waiting_time)");
}

#[test]
fn test_sleep_macro() {
    let result = sleep!(time::Duration::from_secs(55));
    assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "sleep(55s)");
}

#[test]
fn test_sleep_macro_over_long_period() {
    let result = sleep!(time::Duration::from_secs(55340223));
    assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "sleep(1y39w2d12h17m3s)");
}
