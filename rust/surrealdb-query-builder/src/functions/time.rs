/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Time functions
// These functions can be used when working with and manipulating datetime values.
//
// Function	Description
// time::day()	Extracts the day as a number from a datetime
// time::floor()	Rounds a datetime down by a specific duration
// time::group()	Groups a datetime by a particular time interval
// time::hour()	Extracts the hour as a number from a datetime
// time::mins()	Extracts the minutes as a number from a datetime
// time::month()	Extracts the month as a number from a datetime
// time::nano()	Returns the number of nanoseconds since the UNIX epoch
// time::now()	Returns the current datetime
// time::round()	Rounds a datetime up by a specific duration
// time::secs()	Extracts the secs as a number from a datetime
// time::unix()	Returns the number of seconds since the UNIX epoch
// time::wday()	Extracts the week day as a number from a datetime
// time::week()	Extracts the week as a number from a datetime
// time::yday()	Extracts the yday as a number from a datetime
// time::year()	Extracts the year as a number from a datetime

use std::{fmt::Display, str::FromStr};

use crate::{
    sql::{Binding, Buildable, ToRawStatement},
    Field,
};

use super::array::Function;
use surrealdb::sql;

pub struct Datetime(sql::Value);

impl From<Datetime> for sql::Value {
    fn from(value: Datetime) -> Self {
        value.0
    }
}

impl<T: Into<sql::Datetime>> From<T> for Datetime {
    fn from(value: T) -> Self {
        let value: sql::Datetime = value.into();
        Self(value.into())
    }
}

impl From<Field> for Datetime {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}

pub struct Duration(sql::Value);

impl From<Duration> for sql::Value {
    fn from(value: Duration) -> Self {
        value.0
    }
}

impl<T: Into<sql::Duration>> From<T> for Duration {
    fn from(value: T) -> Self {
        let value: sql::Duration = value.into();
        Self(value.into())
    }
}

impl From<Field> for Duration {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}

pub fn now_fn() -> Function {
    let query_string = format!("now()");

    Function {
        query_string,
        bindings: vec![],
    }
}

#[macro_export]
macro_rules! now {
    () => {
        crate::functions::time::now_fn()
    };
}

pub use now;

macro_rules! create_time_fn_with_single_datetime_arg {
    ($function_name: expr) => {
        paste::paste! {
            fn [<$function_name _fn>](datetime: impl Into<Datetime>) -> Function {
                let binding = Binding::new(datetime.into());
                let query_string = format!("time::{}({})", $function_name, binding.get_param_dollarised());

                Function {
                    query_string,
                    bindings: vec![binding],
                }
            }

            #[macro_export]
            macro_rules! [<$function_name>] {
                ( $datetime:expr ) => {
                    crate::functions::time::[<$function_name _fn>]($datetime)
                };
            }

            pub use [<$function_name>];

            #[test]
            fn [<test_ $function_name _macro_with_datetime_field>]() {
                let rebirth_date = Field::new("rebirth_date");
                let result = day!(rebirth_date);

                assert_eq!(result.fine_tune_params(), "time::day($_param_00000001)");
                assert_eq!(result.to_raw().to_string(), "time::day(rebirth_date)");
            }
            #[test]
            fn [<test_ $function_name _macro_with_plain_datetime>]() {
                let dt = chrono::DateTime::<chrono::Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp(61, 0),
                    chrono::Utc,
                );
                let result = day!(dt);
                assert_eq!(result.fine_tune_params(), "time::day($_param_00000001)");
                assert_eq!(
                    result.to_raw().to_string(),
                    "time::day('1970-01-01T00:01:01Z')"
                );
            }
        }
    };
}

create_time_fn_with_single_datetime_arg!("day");
create_time_fn_with_single_datetime_arg!("hour");
create_time_fn_with_single_datetime_arg!("mins");
create_time_fn_with_single_datetime_arg!("month");
create_time_fn_with_single_datetime_arg!("nano");
create_time_fn_with_single_datetime_arg!("secs");
create_time_fn_with_single_datetime_arg!("unix");
create_time_fn_with_single_datetime_arg!("wday");
create_time_fn_with_single_datetime_arg!("week");
create_time_fn_with_single_datetime_arg!("yday");
create_time_fn_with_single_datetime_arg!("year");

fn floor_fn(datetime: impl Into<Datetime>, duration: impl Into<Duration>) -> Function {
    let datetime_binding = Binding::new(datetime.into());
    let duration_binding = Binding::new(duration.into());
    let query_string = format!(
        "time::floor({}, {})",
        datetime_binding.get_param_dollarised(),
        duration_binding.get_param_dollarised()
    );

    Function {
        query_string,
        bindings: vec![datetime_binding, duration_binding],
    }
}

#[macro_export]
macro_rules! floor {
    ( $datetime:expr, $duration:expr ) => {
        crate::functions::time::floor_fn($datetime, $duration)
    };
}

pub use floor;

#[derive(Debug, Clone, Copy)]
enum Interval {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Interval::Year => "year",
                Interval::Month => "month",
                Interval::Hour => "hour",
                Interval::Minute => "minute",
                Interval::Second => "second",
                Interval::Week => "week",
                Interval::Day => "day",
            }
        )
    }
}

struct IntervalOrField(sql::Value);

impl From<Field> for IntervalOrField {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}

impl From<IntervalOrField> for sql::Value {
    fn from(value: IntervalOrField) -> Self {
        value.0
    }
}

impl From<Interval> for IntervalOrField {
    fn from(value: Interval) -> Self {
        Self(value.to_string().into())
    }
}

fn group_fn(datetime: impl Into<Datetime>, interval: impl Into<IntervalOrField>) -> Function {
    let datetime_binding = Binding::new(datetime.into());
    let interval_binding = Binding::new(interval.into());

    let query_string = format!(
        "time::floor({}, {})",
        datetime_binding.get_param_dollarised(),
        interval_binding.get_param_dollarised()
    );

    Function {
        query_string,
        bindings: vec![datetime_binding, interval_binding],
    }
}
// ::<crate::functions::time::IntervalOrField>
#[macro_export]
macro_rules! group {
    ( $datetime:expr, "year" ) => {
        crate::functions::time::group_fn($datetime, Interval::Year)
    };
    ( $datetime:expr, "month" ) => {
        crate::functions::time::group_fn($datetime, Interval::Month)
    };
    ( $datetime:expr, "week" ) => {
        crate::functions::time::group_fn($datetime, Interval::Week)
    };
    ( $datetime:expr, "day" ) => {
        crate::functions::time::group_fn($datetime, Interval::Day)
    };
    ( $datetime:expr, "hour" ) => {
        crate::functions::time::group_fn($datetime, Interval::Hour)
    };
    ( $datetime:expr, "minute" ) => {
        crate::functions::time::group_fn($datetime, Interval::Minute)
    };
    ( $datetime:expr, "second" ) => {
        crate::functions::time::group_fn($datetime, Interval::Second)
    };
    ( $datetime:expr, $interval:expr ) => {
        crate::functions::time::group_fn($datetime, IntervalOrField::from($interval))
    };
}

pub use group;

#[test]
fn test_floor_macro_with_datetime_field() {
    let rebirth_date = Field::new("rebirth_date");
    let duration = Field::new("duration");
    let result = floor!(rebirth_date, duration);

    assert_eq!(
        result.fine_tune_params(),
        "time::floor($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "time::floor(rebirth_date, duration)"
    );
}

#[test]
fn test_floor_macro_with_plain_datetime_and_duration() {
    let dt = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(61, 0),
        chrono::Utc,
    );
    let duration = std::time::Duration::from_secs(24 * 60 * 60 * 7);
    let result = floor!(dt, duration);
    assert_eq!(
        result.fine_tune_params(),
        "time::floor($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "time::floor('1970-01-01T00:01:01Z', 1w)"
    );
}

#[test]
fn test_group_macro_with_datetime_field() {
    let rebirth_date = Field::new("rebirth_date");
    let duration = Field::new("duration");
    let result = group!(rebirth_date, duration);

    assert_eq!(
        result.fine_tune_params(),
        "time::floor($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "time::floor(rebirth_date, duration)"
    );
}

macro_rules! test_group_with_interval {
    ($interval_name:ident, $interval: expr) => {
        paste::paste! {
            #[test]
            fn [<test_group_macro_with_plain_datetime_and_ $interval_name>]() {
                let dt = chrono::DateTime::<chrono::Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp(61, 0),
                    chrono::Utc,
                );
                let result = group!(dt, $interval);
                assert_eq!(
                    result.fine_tune_params(),
                    "time::floor($_param_00000001, $_param_00000002)"
                );
                assert_eq!(
                    result.to_raw().to_string(),
                    format!("time::floor('1970-01-01T00:01:01Z', '{}')", $interval)
                );
            }
        }
    };
}

test_group_with_interval!(year, "year");
test_group_with_interval!(month, "month");
test_group_with_interval!(week, "week");
test_group_with_interval!(day, "day");
test_group_with_interval!(hour, "hour");
test_group_with_interval!(minute, "minute");
test_group_with_interval!(second, "second");

test_group_with_interval!(year_with_enum, Interval::Year);
test_group_with_interval!(month_with_enum, Interval::Month);
test_group_with_interval!(week_with_enum, Interval::Week);
test_group_with_interval!(day_with_enum, Interval::Day);
test_group_with_interval!(hour_with_enum, Interval::Hour);
test_group_with_interval!(minute_with_enum, Interval::Minute);
test_group_with_interval!(second_with_enum, Interval::Second);

#[test]
fn test_now_fn() {
    let result = now_fn();
    assert_eq!(result.fine_tune_params(), "now()");
    assert_eq!(result.to_raw().to_string(), "now()");
}

#[test]
fn test_now_macro() {
    let result = now!();
    assert_eq!(result.fine_tune_params(), "now()");
    assert_eq!(result.to_raw().to_string(), "now()");
}
