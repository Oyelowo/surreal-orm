/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

// Sleep function
// The SLEEP function is used to introduce a delay or pause in the execution of a query or a batch of queries for a specific amount of time.
//
// Function	Description
// sleep(@duration)	Delays or pauses in the execution of a query or a batch of queries.

use crate::{Buildable, DurationLike, Erroneous, Function, Parametric};

/// The SLEEP function is used to introduce a delay or pause in the execution of a query or a batch
/// of queries for a specific amount of time.
pub fn sleep_fn(duration: impl Into<DurationLike>) -> Function {
    let duration: DurationLike = duration.into();

    Function {
        query_string: format!("sleep({})", duration.build()),
        bindings: duration.get_bindings(),
        errors: duration.get_errors(),
    }
}

/// Creates a new sleep function
/// The SLEEP function is used to introduce a delay or pause in the execution of a query or a batch
/// of queries for a specific amount of time.
///
/// # Arguments
///
/// * `duration` - The duration to sleep for. This can be a `Duration` or a `Field` or a `Param`
/// that represents a duration.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::sleep, statements::let_};
/// use std::time;
/// let sleep = sleep!(time::Duration::from_secs(55));
/// assert_eq!(sleep.to_raw().build(), "sleep(55s)");
///
/// let waiting_time_field = Field::new("waiting_time_field");
/// let sleep = sleep!(waiting_time_field);
/// assert_eq!(sleep.to_raw().build(), "sleep(waiting_time_field)");
///
/// let waiting_time_param = Param::new("waiting_time_param");
/// let sleep = sleep!(waiting_time_param);
/// assert_eq!(sleep.to_raw().build(), "sleep($waiting_time_param)");
/// ```
#[macro_export]
macro_rules! sleep {
    ( $duration:expr ) => {
        $crate::functions::sleep::sleep_fn($duration)
    };
}

pub use sleep;

#[cfg(test)]
mod tests {
    use std::time;

    use super::*;
    use crate::*;

    #[test]
    fn test_sleep_fn_with_field_data() {
        let waiting_time = Field::new("waiting_time");
        let result = sleep_fn(waiting_time);

        assert_eq!(result.fine_tune_params(), "sleep(waiting_time)");
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

        assert_eq!(result.fine_tune_params(), "sleep(waiting_time)");
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
}
