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
//
// time::format() Outputs a datetime according to a specific format
//
// time::group()	Groups a datetime by a particular time interval
// time::hour()	Extracts the hour as a number from a datetime
// time::minute()	Extracts the minutes as a number from a datetime
// time::month()	Extracts the month as a number from a datetime
// time::nano()	Returns the number of nanoseconds since the UNIX epoch
// time::now()	Returns the current datetime
// time::round()	Rounds a datetime up by a specific duration
// time::second()	Extracts the secs as a number from a datetime
//
// time::timezone() Returns the current local timezone offset in hours

// time::unix()	Returns the number of seconds since the UNIX epoch
// time::wday()	Extracts the week day as a number from a datetime
// time::week()	Extracts the week as a number from a datetime
// time::yday()	Extracts the yday as a number from a datetime
// time::year()	Extracts the year as a number from a datetime

use std::{fmt::Display, str::FromStr};

use crate::{
    traits::{Binding, Buildable, ToRaw},
    types::{DatetimeLike, DurationLike, Field, Function, Interval, Param},
    StrandLike,
};

use surrealdb::sql;

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
        $crate::functions::time::now_fn()
    };
}

pub use now;

macro_rules! create_time_fn_with_single_datetime_arg {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            use $crate::Buildable as _;
            use $crate::Parametric as _;

            $(#[$attr])*
            fn [<$function_name _fn>](datetime: impl Into<$crate::DatetimeLike>) -> $crate::Function {
                let datetime: $crate::DatetimeLike = datetime.into();
                let query_string = format!("time::{}({})", $function_name, datetime.build());

                $crate::Function {
                    query_string,
                    bindings: datetime.get_bindings(),
                }
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<time_ $function_name>] {
                ( $datetime:expr ) => {
                    $crate::functions::time::[<$function_name _fn>]($datetime)
                };
            }

            pub use [<time_ $function_name>] as [<$function_name>];

            #[test]
            fn [<test_ $function_name _macro_with_datetime_field>]() {
                let rebirth_date = $crate::Field::new("rebirth_date");
                let result = day!(rebirth_date);

                assert_eq!(result.fine_tune_params(), "time::day(rebirth_date)");
                assert_eq!(result.to_raw().build(), "time::day(rebirth_date)");
            }

            #[test]
            fn [<test_ $function_name _macro_with_plain_datetime>]() {
                let dt = chrono::DateTime::<chrono::Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
                    chrono::Utc,
                );
                let result = day!(dt);
                assert_eq!(result.fine_tune_params(), "time::day($_param_00000001)");
                assert_eq!(
                    result.to_raw().build(),
                    "time::day('1970-01-01T00:01:01Z')"
                );
            }
        }
    };
}

create_time_fn_with_single_datetime_arg!(
    /// The time::day function extracts the day as a number from a datetime.
    /// The function is also aliased as `time_day!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to extract the day from. Could also be a field or a parameter representing a datetime.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///     chrono::Utc,
    /// );
    ///
    /// let result = time::day!(dt);
    /// assert_eq!(result.fine_tune_params(), "time::day($_param_00000001)");
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::day('1970-01-01T00:01:01Z')"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::day!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::day(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::day!(param);
    /// assert_eq!(result.to_raw().build(), "time::day($rebirth_date)");
    /// ```
    =>
    "day"
);

create_time_fn_with_single_datetime_arg!(
    /// The time::hour function extracts the hour as a number from a datetime.
    /// The function is also aliased as `time_hour!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to extract the hour from. Could also be a field or a parameter representing a datetime.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///     chrono::Utc,
    /// );
    ///
    /// let result = time::hour!(dt);
    /// assert_eq!(result.fine_tune_params(), "time::hour($_param_00000001)");
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::hour('1970-01-01T00:01:01Z')"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::hour!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::hour(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::hour!(param);
    /// assert_eq!(result.to_raw().build(), "time::hour($rebirth_date)");
    /// ```
    =>
    "hour"
);

create_time_fn_with_single_datetime_arg!(
    /// The time::minute function extracts the minute as a number from a datetime.
    /// The function is also aliased as `time_minute!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to extract the minute from. Could also be a field or a parameter representing a datetime.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///     chrono::Utc,
    /// );
    ///
    /// let result = time::minute!(dt);
    /// assert_eq!(result.fine_tune_params(), "time::minute($_param_00000001)");
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::minute('1970-01-01T00:01:01Z')"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::minute!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::minute(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::minute!(param);
    /// assert_eq!(result.to_raw().build(), "time::minute($rebirth_date)");
    /// ```
    =>
    "minute"
);

create_time_fn_with_single_datetime_arg!(
    /// The time::month function extracts the month as a number from a datetime.
    /// The function is also aliased as `time_month!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to extract the month from. Could also be a field or a parameter representing a datetime.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///     chrono::Utc,
    /// );
    ///
    /// let result = time::month!(dt);
    /// assert_eq!(result.fine_tune_params(), "time::month($_param_00000001)");
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::month('1970-01-01T00:01:01Z')"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::month!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::month(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::month!(param);
    /// assert_eq!(result.to_raw().build(), "time::month($rebirth_date)");
    /// ```
    =>
    "month"
);

// nano returns the number of nanoseconds since the UNIX epoch
create_time_fn_with_single_datetime_arg!(
    /// The time::nano returns the number of nanoseconds since the UNIX epoch.
    /// The function is also aliased as `time_nano!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to derive nanoseconds since epoch from. Could also be a field or a parameter representing a datetime.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///     chrono::Utc,
    /// );
    ///
    /// let result = time::nano!(dt);
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::nano('1970-01-01T00:01:01Z')"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::nano!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::nano(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::nano!(param);
    /// assert_eq!(result.to_raw().build(), "time::nano($rebirth_date)");
    /// ```
    =>
    "nano"
);

create_time_fn_with_single_datetime_arg!(
    /// The time::second function extracts the second as a number from a datetime.
    /// The function is also aliased as `time_second!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to extract the second from. Could also be a field or a parameter representing a datetime.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///     chrono::Utc,
    /// );
    ///
    /// let result = time::second!(dt);
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::second('1970-01-01T00:01:01Z')"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::second!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::second(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::second!(param);
    /// assert_eq!(result.to_raw().build(), "time::second($rebirth_date)");
    /// ```
    =>
    "second"
);

create_time_fn_with_single_datetime_arg!(
    /// The time::unix function returns a datetime as an integer representing the number of seconds since the UNIX epoch.
    /// The function is also aliased as `time_unix!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to derive seconds since epoch from. Could also be a field or a parameter representing a datetime.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///     chrono::Utc,
    /// );
    ///
    /// let result = time::unix!(dt);
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::unix('1970-01-01T00:01:01Z')"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::unix!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::unix(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::unix!(param);
    /// assert_eq!(result.to_raw().build(), "time::unix($rebirth_date)");
    /// ```
    =>
    "unix"
);

create_time_fn_with_single_datetime_arg!(
    /// The time::wday function extracts the week day as a number from a datetime.
    /// The function is also aliased as `time_wday!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to extract the week day from. Could also be a field or a parameter representing a datetime.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///     chrono::Utc,
    /// );
    ///
    /// let result = time::wday!(dt);
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::wday('1970-01-01T00:01:01Z')"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::wday!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::wday(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::wday!(param);
    /// assert_eq!(result.to_raw().build(), "time::wday($rebirth_date)");
    /// ```
    =>
    "wday"
);

create_time_fn_with_single_datetime_arg!(
    /// The time::week function extracts the week as a number from a datetime.
    /// The function is also aliased as `time_week!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to extract the week from. Could also be a field or a parameter representing a datetime.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///     chrono::Utc,
    /// );
    ///
    /// let result = time::week!(dt);
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::week('1970-01-01T00:01:01Z')"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::week!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::week(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::week!(param);
    /// assert_eq!(result.to_raw().build(), "time::week($rebirth_date)");
    /// ```
    =>
    "week"
);

create_time_fn_with_single_datetime_arg!(
    /// The time::yday function extracts the yday as a number from a datetime.
    /// The function is also aliased as `time_yday!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to extract the yday from. Could also be a field or a parameter representing a datetime.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///     chrono::Utc,
    /// );
    ///
    /// let result = time::yday!(dt);
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::yday('1970-01-01T00:01:01Z')"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::yday!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::yday(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::yday!(param);
    /// assert_eq!(result.to_raw().build(), "time::yday($rebirth_date)");
    /// ```
    =>
    "yday"
);

create_time_fn_with_single_datetime_arg!(
    /// The time::year function extracts the year as a number from a datetime.
    /// The function is also aliased as `time_year!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to extract the year from. Could also be a field or a parameter representing a datetime.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///     chrono::Utc,
    /// );
    ///
    /// let result = time::year!(dt);
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::year('1970-01-01T00:01:01Z')"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::year!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::year(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::year!(param);
    /// assert_eq!(result.to_raw().build(), "time::year($rebirth_date)");
    /// ```
    =>
    "year"
);

/// The time::floor function rounds a datetime down by a specific duration.
pub fn floor_fn(datetime: impl Into<DatetimeLike>, duration: impl Into<DurationLike>) -> Function {
    let datetime: DatetimeLike = datetime.into();
    let duration: DurationLike = duration.into();
    let mut bindings = datetime.get_bindings();
    bindings.extend(duration.get_bindings());

    let query_string = format!("time::floor({}, {})", datetime.build(), duration.build(),);

    Function {
        query_string,
        bindings,
    }
}

/// The time::floor function rounds a datetime down by a specific duration.
/// The function is also aliased as `time_floor!`.
///
/// # Arguments
/// * `datetime` - The datetime to round down. Could also be a field or a parameter representing a datetime.
/// * `duration` - The duration to round down by. Could also be a field or a parameter representing a duration.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as  surrealdb_orm;
/// use surrealdb_orm::{*, functions::time};
/// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
///    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
///    chrono::Utc,
/// );
/// let duration = chrono::Duration::seconds(10);
/// let result = time::floor!(dt, duration);
/// assert_eq!(
///   result.to_raw().build(),
///   "time::floor('1970-01-01T00:01:01Z', 10s)"
/// );
///
/// let rebirth_date = Field::new("rebirth_date");
/// let duration = Field::new("duration");
/// let result = time::floor!(rebirth_date, duration);
/// assert_eq!(
///  result.to_raw().build(),
///  "time::floor(rebirth_date, duration)"
///  );
///  
///  let param = Param::new("rebirth_date");
///  let duration = Param::new("duration");
///  let result = time::floor!(param, duration);
///  assert_eq!(
///     result.to_raw().build(),
///     "time::floor($rebirth_date, $duration)"
///  );
///  ```
#[macro_export]
macro_rules! time_floor {
    ( $datetime:expr, $duration:expr ) => {
        $crate::functions::time::floor_fn($datetime, $duration)
    };
}

pub use time_floor as floor;

/// The time::round function rounds a datetime up by a specific duration.
pub fn round_fn(datetime: impl Into<DatetimeLike>, duration: impl Into<DurationLike>) -> Function {
    let datetime: DatetimeLike = datetime.into();
    let duration: DurationLike = duration.into();
    let mut bindings = datetime.get_bindings();
    bindings.extend(duration.get_bindings());

    Function {
        query_string: format!("time::round({}, {})", datetime.build(), duration.build()),
        bindings,
    }
}

/// The time::round function rounds a datetime up by a specific duration.
///
/// The function is also aliased as `time_round!`.
///
/// # Arguments
///
/// * `datetime` - The datetime to round up. Could also be a field or a parameter representing a datetime.
/// * `duration` - The duration to round up by. Could also be a field or a parameter representing a duration.
/// # Example
/// ```rust
/// # use surrealdb_query_builder as  surrealdb_orm;
/// use surrealdb_orm::{*, functions::time};
/// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
///   chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
///   chrono::Utc,
/// );
/// let duration = chrono::Duration::seconds(10);
/// let result = time::round!(dt, duration);
/// assert_eq!(
///     result.to_raw().build(),
///     "time::round('1970-01-01T00:01:01Z', 10s)"
/// );
/// let rebirth_date = Field::new("rebirth_date");
/// let duration = Field::new("duration");
/// let result = time::round!(rebirth_date, duration);
/// assert_eq!(
///     result.to_raw().build(),
///     "time::round(rebirth_date, duration)"
/// );
/// let param = Param::new("rebirth_date");
/// let duration = Param::new("duration");
/// let result = time::round!(param, duration);
/// assert_eq!(
///     result.to_raw().build(),
///     "time::round($rebirth_date, $duration)"
/// );
/// ```
#[macro_export]
macro_rules! time_round {
    ( $datetime:expr, $duration:expr ) => {
        $crate::functions::time::round_fn($datetime, $duration)
    };
}

pub use time_round as round;

/// The time::group function reduces and rounds a datetime down to a particular time interval. The
/// second argument must be a string, and can be one of the following values: year, month, day,
/// hour, minute, second.
pub fn group_fn(datetime: impl Into<DatetimeLike>, interval: impl Into<StrandLike>) -> Function {
    let datetime: DatetimeLike = datetime.into();
    let interval: StrandLike = interval.into();
    let mut bindings = datetime.get_bindings();
    bindings.extend(interval.get_bindings());

    let query_string = format!("time::group({}, {})", datetime.build(), interval.build());

    Function {
        query_string,
        bindings,
    }
}

/// The time::group function reduces and rounds a datetime down to a particular time interval. The
/// second argument must be a string, and can be one of the following values: year, month, day,
/// hour, minute, second.
/// The function is also aliased as `time_group!`.
///
/// # Arguments
/// * `datetime` - The datetime to round down. Could also be a field or a parameter representing a datetime.
/// * `interval` - The interval to round down to. Should be one of the following: year, month, day, hour, minute, second.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as  surrealdb_orm;
/// use surrealdb_orm::{*, functions::time};
/// let dt = chrono::DateTime::<chrono::Utc>::from_utc(
///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
///     chrono::Utc,
/// );
/// let result = time::group!(dt, "year");
/// assert_eq!(
///    result.to_raw().build(),
///    "time::group('1970-01-01T00:01:01Z', "year")"
/// );
///
/// let rebirth_date = Field::new("rebirth_date");
/// let result = time::group!(rebirth_date, "year");
/// assert_eq!(
///   result.to_raw().build(),
///   "time::group(rebirth_date, "year")"
/// );
///
/// let param = Param::new("rebirth_date");
/// let result = time::group!(param, "month");
/// assert_eq!(
///     result.to_raw().build(),
///     "time::group($rebirth_date, "month")"
///  );
#[macro_export]
macro_rules! time_group {
    ( $datetime:expr, "year" ) => {
        $crate::functions::time::group_fn($datetime, $crate::Interval::Year)
    };
    ( $datetime:expr, "month" ) => {
        $crate::functions::time::group_fn($datetime, $crate::Interval::Month)
    };
    ( $datetime:expr, "week" ) => {
        $crate::functions::time::group_fn($datetime, $crate::Interval::Week)
    };
    ( $datetime:expr, "day" ) => {
        $crate::functions::time::group_fn($datetime, $crate::Interval::Day)
    };
    ( $datetime:expr, "hour" ) => {
        $crate::functions::time::group_fn($datetime, $crate::Interval::Hour)
    };
    ( $datetime:expr, "minute" ) => {
        $crate::functions::time::group_fn($datetime, $crate::Interval::Minute)
    };
    ( $datetime:expr, "second" ) => {
        $crate::functions::time::group_fn($datetime, $crate::Interval::Second)
    };
    ( $datetime:expr, $interval:expr ) => {
        $crate::functions::time::group_fn($datetime, $crate::StrandLike::from($interval))
    };
}

pub use time_group as group;

#[test]
fn test_floor_macro_with_datetime_field() {
    let rebirth_date = Field::new("rebirth_date");
    let duration = Field::new("duration");
    let result = floor!(rebirth_date, duration);

    assert_eq!(
        result.fine_tune_params(),
        "time::floor(rebirth_date, duration)"
    );
    assert_eq!(
        result.to_raw().build(),
        "time::floor(rebirth_date, duration)"
    );
}

#[test]
fn test_floor_macro_with_plain_datetime_and_duration() {
    let dt = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
        chrono::Utc,
    );
    let duration = std::time::Duration::from_secs(24 * 60 * 60 * 7);
    let result = floor!(dt, duration);
    assert_eq!(
        result.fine_tune_params(),
        "time::floor($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().build(),
        "time::floor('1970-01-01T00:01:01Z', 1w)"
    );
}

#[test]
fn test_round_macro_with_datetime_field() {
    let rebirth_date = Field::new("rebirth_date");
    let duration = Field::new("duration");
    let result = round!(rebirth_date, duration);

    assert_eq!(
        result.fine_tune_params(),
        "time::round(rebirth_date, duration)"
    );
    assert_eq!(
        result.to_raw().build(),
        "time::round(rebirth_date, duration)"
    );
}

#[test]
fn test_round_macro_with_plain_datetime_and_duration() {
    let dt = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
        chrono::Utc,
    );
    let duration = std::time::Duration::from_secs(24 * 60 * 60 * 7);
    let result = round!(dt, duration);
    assert_eq!(
        result.fine_tune_params(),
        "time::round($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().build(),
        "time::round('1970-01-01T00:01:01Z', 1w)"
    );
}

#[test]
fn test_group_macro_with_datetime_field() {
    let rebirth_date = Field::new("rebirth_date");
    let duration = Field::new("duration");
    let result = group!(rebirth_date, duration);

    assert_eq!(
        result.fine_tune_params(),
        "time::floor(rebirth_date, duration)"
    );
    assert_eq!(
        result.to_raw().build(),
        "time::floor(rebirth_date, duration)"
    );
}

#[test]
fn test_group_macro_with_datetime_params() {
    let rebirth_date = Param::new("rebirth_date");
    let duration = Param::new("duration");
    let result = group!(rebirth_date, duration);

    assert_eq!(
        result.fine_tune_params(),
        "time::floor($rebirth_date, $duration)"
    );
    assert_eq!(
        result.to_raw().build(),
        "time::floor($rebirth_date, $duration)"
    );
}

macro_rules! test_group_with_interval {
    ($interval_name:ident, $interval: expr) => {
        paste::paste! {
            #[test]
            fn [<test_group_macro_with_plain_datetime_and_ $interval_name>]() {
                let dt = chrono::DateTime::<chrono::Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
                    chrono::Utc,
                );
                let result = group!(dt, $interval);
                assert_eq!(
                    result.fine_tune_params(),
                    "time::group($_param_00000001, $_param_00000002)"
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!("time::group('1970-01-01T00:01:01Z', '{}')", $interval)
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
    assert_eq!(result.to_raw().build(), "now()");
}

#[test]
fn test_now_macro() {
    let result = now!();
    assert_eq!(result.fine_tune_params(), "now()");
    assert_eq!(result.to_raw().build(), "now()");
}
