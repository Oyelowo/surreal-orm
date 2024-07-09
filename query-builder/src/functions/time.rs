/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Time functions
// These functions can be used when working with and manipulating datetime values.
//
// Function	Description
// time::day()	Extracts the day as a number from a datetime
// time::floor()	Rounds a datetime down by a specific duration
// time::format() Outputs a datetime according to a specific format
// time::group()	Groups a datetime by a particular time interval
// time::hour()	Extracts the hour as a number from a datetime
// time::minute()	Extracts the minutes as a number from a datetime
// time::month()	Extracts the month as a number from a datetime
// time::nano()	Returns the number of nanoseconds since the UNIX epoch
// time::now()	Returns the current datetime
// time::round()	Rounds a datetime up by a specific duration
// time::second()	Extracts the secs as a number from a datetime
// time::timezone() Returns the current local timezone offset in hours
// time::unix()	Returns the number of seconds since the UNIX epoch
// time::wday()	Extracts the week day as a number from a datetime
// time::week()	Extracts the week as a number from a datetime
// time::yday()	Extracts the yday as a number from a datetime
// time::year()	Extracts the year as a number from a datetime
// time::ceil()	Rounds a datetime up by a specific duration
// time::max()	Finds the most recent datetime in an array
// time::min()	Finds the least recent datetime in an array
// time::from::micros()	Calculates a datetimes based on an amount of microseconds since January 1, 1970 0:00:00 UTC.
// time::from::millis()	Calculates a datetimes based on an amount of milliseconds since January 1, 1970 0:00:00 UTC.
// time::from::secs()	Calculates a datetimes based on an amount of seconds since January 1, 1970 0:00:00 UTC.
// time::from::unix()	Calculates a datetimes based on an amount of seconds since January 1, 1970 0:00:00 UTC.

use crate::{Buildable, DatetimeLike, DurationLike, Erroneous, Function, Parametric, StrandLike};

/// The time::now function returns the current datetime as an ISO8601 timestamp.The time::now function returns the current datetime as an ISO8601 timestamp.
pub fn now_fn() -> Function {
    let query_string = "now()".to_string();

    Function {
        query_string,
        bindings: vec![],
        errors: vec![],
    }
}

/// The time::now function returns the current datetime as an ISO8601 timestamp.The time::now function returns the current datetime as an ISO8601 timestamp.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::time};
///
/// let result = time::now!();
/// assert_eq!(result.to_raw().build(), "now()");
/// ```
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
            $(#[$attr])*
            pub fn [<$function_name _fn>](datetime: impl Into<$crate::DatetimeLike>) -> $crate::Function {
                let datetime: $crate::DatetimeLike = datetime.into();
                let query_string = format!("time::{}({})", $function_name, datetime.build());

                $crate::Function {
                    query_string,
                    bindings: datetime.get_bindings(),
                    errors: datetime.get_errors(),
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

            #[cfg(test)]
            mod [<test_ $function_name _fn>] {
                use $crate::*;
                use crate::functions::time;


                #[test]
                fn [<test_ $function_name _fn_with_datetime_field>]() {
                    let rebirth_date = $crate::Field::new("rebirth_date");
                    let result = time::[<$function_name>]!(rebirth_date);

                    assert_eq!(result.fine_tune_params(), format!("time::{}(rebirth_date)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("time::{}(rebirth_date)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_plain_datetime>]() {
                    let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
                    let result = time::[<$function_name>]!(dt);
                    assert_eq!(result.fine_tune_params(), format!("time::{}($_param_00000001)", $function_name));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("time::{}('1970-01-01T00:01:01Z')", $function_name)
                    );
                }
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
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    let mut errors = datetime.get_errors();
    bindings.extend(duration.get_bindings());
    errors.extend(duration.get_errors());

    let query_string = format!("time::floor({}, {})", datetime.build(), duration.build(),);

    Function {
        query_string,
        bindings,
        errors,
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
/// # use surreal_query_builder as  surreal_orm;
/// use surreal_orm::{*, functions::time};
/// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
/// let duration = std::time::Duration::from_secs(10);
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

/// The time::ceil function rounds a datetime up by a specific duration.
pub fn ceil_fn(datetime: impl Into<DatetimeLike>, duration: impl Into<DurationLike>) -> Function {
    let datetime: DatetimeLike = datetime.into();
    let duration: DurationLike = duration.into();
    let mut bindings = datetime.get_bindings();
    let mut errors = datetime.get_errors();
    bindings.extend(duration.get_bindings());
    errors.extend(duration.get_errors());

    let query_string = format!("time::ceil({}, {})", datetime.build(), duration.build(),);

    Function {
        query_string,
        bindings,
        errors,
    }
}

/// The time::ceil function rounds a datetime up by a specific duration.
/// The function is also aliased as `time_ceil!`.
/// # Example
/// ```rust
/// # use surreal_query_builder as  surreal_orm;
///   use surreal_orm::{*, functions::time};
///   let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
///   let duration = std::time::Duration::from_secs(10);
///   let result = time::ceil!(dt, duration);
///   assert_eq!(
///   result.to_raw().build(),
///   "time::ceil('1970-01-01T00:01:01Z', 10s)"
///   );
///
///   let rebirth_date = Field::new("rebirth_date");
///   let duration = Field::new("duration");
///   let result = time::ceil!(rebirth_date, duration);
///   assert_eq!(
///   result.to_raw().build(),
///   "time::ceil(rebirth_date, duration)"
///   );
///
///   let param = Param::new("rebirth_date");
///   let duration = Param::new("duration");
///   let result = time::ceil!(param, duration);
///   assert_eq!(
///   result.to_raw().build(),
///   "time::ceil($rebirth_date, $duration)"
///   );
///   ```
#[macro_export]
macro_rules! time_ceil {
    ( $datetime:expr, $duration:expr ) => {
        $crate::functions::time::ceil_fn($datetime, $duration)
    };
}

pub use time_ceil as ceil;

/// The time::format function outputs a datetime according to a specific format.
///
/// time::format(datetime, string) -> string
/// The following example shows this function, and its output, when used in a select statement:
///
/// SELECT * FROM time::format("2021-11-01T08:30:17+00:00", "%Y-%m-%d");
/// "2021-11-01"
pub fn format_fn(datetime: impl Into<DatetimeLike>, format: impl Into<StrandLike>) -> Function {
    let datetime: DatetimeLike = datetime.into();
    let format: StrandLike = format.into();
    let mut bindings = datetime.get_bindings();
    let mut errors = datetime.get_errors();
    bindings.extend(format.get_bindings());
    errors.extend(format.get_errors());

    let query_string = format!("time::format({}, {})", datetime.build(), format.build());

    Function {
        query_string,
        bindings,
        errors,
    }
}

/// The time::format function outputs a datetime according to a specific format.
/// The function is also aliased as `time_format!`.
///
/// # Arguments
/// * `datetime` - The datetime to format. Could also be a field or a parameter representing a datetime.
/// * `format` - The format to use. Could also be a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as  surreal_orm;
/// use surreal_orm::{*, functions::time};
/// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
/// let result = time::format_!(dt, "%Y-%m-%d");
/// assert_eq!(
///     result.to_raw().build(),
///     "time::format('1970-01-01T00:01:01Z', '%Y-%m-%d')"
/// );
///
/// let rebirth_date = Field::new("rebirth_date");
/// let format = Field::new("format");
/// let result = time::format_!(rebirth_date, format);
/// assert_eq!(
///     result.to_raw().build(),
///     "time::format(rebirth_date, format)"
/// );
///
/// let param = Param::new("rebirth_date");
/// let format = Param::new("format");
/// let result = time::format_!(param, format);
/// assert_eq!(
///     result.to_raw().build(),
///     "time::format($rebirth_date, $format)"
/// );
/// ```
#[macro_export]
macro_rules! time_format {
    ( $datetime:expr, $format:expr ) => {
        $crate::functions::time::format_fn($datetime, $format)
    };
}
pub use time_format as format_;

/// The time::timezone function returns the current local timezone offset in hours.
///
/// time::timezone() -> string
/// The following example shows this function, and its output, when used in a select statement:
///
/// SELECT * FROM time::timezone();
/// "+05:30"
pub fn timezone_fn() -> Function {
    Function {
        query_string: "time::timezone()".to_string(),
        bindings: vec![],
        errors: vec![],
    }
}

/// The time::timezone function returns the current local timezone offset in hours.
/// The function is also aliased as `time_timezone!`.
/// # Example
/// ```rust
/// # use surreal_query_builder as  surreal_orm;
/// use surreal_orm::{*, functions::time};
///
/// let result = time::timezone!();
/// assert_eq!(
///    result.to_raw().build(),
///    "time::timezone()"
/// );
/// ```
#[macro_export]
macro_rules! time_timezone {
    () => {
        $crate::functions::time::timezone_fn()
    };
}
pub use time_timezone as timezone;

/// The time::round function rounds a datetime up by a specific duration.
pub fn round_fn(datetime: impl Into<DatetimeLike>, duration: impl Into<DurationLike>) -> Function {
    let datetime: DatetimeLike = datetime.into();
    let duration: DurationLike = duration.into();
    let mut bindings = datetime.get_bindings();
    let mut errors = datetime.get_errors();
    bindings.extend(duration.get_bindings());
    errors.extend(duration.get_errors());

    Function {
        query_string: format!("time::round({}, {})", datetime.build(), duration.build()),
        bindings,
        errors,
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
/// # use surreal_query_builder as  surreal_orm;
/// use surreal_orm::{*, functions::time};
/// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
/// let duration = std::time::Duration::from_secs(10);
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
    let mut errors = datetime.get_errors();
    bindings.extend(interval.get_bindings());
    errors.extend(interval.get_errors());

    let query_string = format!("time::group({}, {})", datetime.build(), interval.build());

    Function {
        query_string,
        bindings,
        errors,
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
/// # use surreal_query_builder as  surreal_orm;
/// use surreal_orm::{*, functions::time};
/// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
/// let result = time::group!(dt, "year");
/// assert_eq!(
///    result.to_raw().build(),
///    "time::group('1970-01-01T00:01:01Z', 'year')"
/// );
///
/// let rebirth_date = Field::new("rebirth_date");
/// let result = time::group!(rebirth_date, "year");
/// assert_eq!(
///   result.to_raw().build(),
///   "time::group(rebirth_date, 'year')"
/// );
///
/// let param = Param::new("rebirth_date");
/// let result = time::group!(param, "month");
/// assert_eq!(
///     result.to_raw().build(),
///     "time::group($rebirth_date, 'month')"
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

#[cfg(test)]
mod tests {
    use crate::functions::time;
    use crate::*;

    #[test]
    fn test_floor_macro_with_datetime_field() {
        let rebirth_date = Field::new("rebirth_date");
        let duration = Field::new("duration");
        let result = time::floor!(rebirth_date, duration);

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
        let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
        let duration = std::time::Duration::from_secs(24 * 60 * 60 * 7);
        let result = time::floor!(dt, duration);
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
        let result = time::round!(rebirth_date, duration);

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
        let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
        let duration = std::time::Duration::from_secs(24 * 60 * 60 * 7);
        let result = time::round!(dt, duration);
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
        let result = time::group!(rebirth_date, duration);

        assert_eq!(
            result.fine_tune_params(),
            "time::group(rebirth_date, duration)"
        );
        assert_eq!(
            result.to_raw().build(),
            "time::group(rebirth_date, duration)"
        );
    }

    #[test]
    fn test_group_macro_with_datetime_params() {
        let rebirth_date = Param::new("rebirth_date");
        let duration = Param::new("duration");
        let result = time::group!(rebirth_date, duration);

        assert_eq!(
            result.fine_tune_params(),
            "time::group($rebirth_date, $duration)"
        );
        assert_eq!(
            result.to_raw().build(),
            "time::group($rebirth_date, $duration)"
        );
    }

    macro_rules! test_group_with_interval {
        ($interval_name:ident, $interval: expr) => {
            paste::paste! {
                #[test]
                fn [<test_group_macro_with_plain_datetime_and_ $interval_name>]() {
                    let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
                    let result = time::group!(dt, $interval);
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
        let result = time::now_fn();
        assert_eq!(result.fine_tune_params(), "now()");
        assert_eq!(result.to_raw().build(), "now()");
    }

    #[test]
    fn test_now_macro() {
        let result = time::now!();
        assert_eq!(result.fine_tune_params(), "now()");
        assert_eq!(result.to_raw().build(), "now()");
    }
}

macro_rules! create_time_fn_with_single_array_of_datetime_arg {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](datetime: impl ::std::convert::Into<$crate::ArrayLike>) -> $crate::Function {
                let datetime: $crate::ArrayLike = datetime.into();
                let query_string = format!("time::{}({})", $function_name, datetime.build());

                $crate::Function {
                    query_string,
                    bindings: datetime.get_bindings(),
                    errors: datetime.get_errors(),
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

            #[cfg(test)]
            mod [<test_ $function_name _fn>] {
                use $crate::*;
                use crate::functions::time;


                #[test]
                fn [<test_ $function_name _fn_with_datetime_field>]() {
                    let rebirth_date = $crate::Field::new("rebirth_date");
                    let result = time::[<$function_name>]!(rebirth_date);

                    assert_eq!(result.fine_tune_params(), format!("time::{}(rebirth_date)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("time::{}(rebirth_date)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_plain_datetime>]() {
                    let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
                    let dt2 = $crate::Param::new("dt2");
                    let result = time::[<$function_name>]!(arr![dt, dt2]);
                    assert_eq!(result.fine_tune_params(), format!("time::{}([$_param_00000001, $dt2])", $function_name));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("time::{}(['1970-01-01T00:01:01Z', $dt2])", $function_name)

                    );
                }
            }
        }
    }

}
create_time_fn_with_single_array_of_datetime_arg!(
    /// The time::max function returns the maximum datetime from an array of datetimes.
    /// The function is also aliased as `time_max!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The array of datetimes to extract the maximum datetime from. Could also be a field or a parameter representing an array of datetimes.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt1 = chrono::DateTime::from_timestamp(61, 0).unwrap();
    /// let dt2 = chrono::DateTime::from_timestamp(62, 0).unwrap();
    /// let dt3 = chrono::DateTime::from_timestamp(63, 0).unwrap();
    ///
    /// let result = time::max!([dt1, dt2, dt3]);
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::max(['1970-01-01T00:01:01Z', '1970-01-01T00:01:02Z', '1970-01-01T00:01:03Z'])"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::max!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::max(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::max!(param);
    /// assert_eq!(result.to_raw().build(), "time::max($rebirth_date)");
    /// ```
    =>
    "max"
);

create_time_fn_with_single_array_of_datetime_arg!(
    /// The time::min function returns the minimum datetime from an array of datetimes.
    /// The function is also aliased as `time_min!`.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The array of datetimes to extract the minimum datetime from. Could also be a field or a parameter representing an array of datetimes.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::time};
    ///
    /// let dt1 = chrono::DateTime::from_timestamp(61, 0).unwrap();
    /// let dt2 = chrono::DateTime::from_timestamp(62, 0).unwrap();
    /// let dt3 = chrono::DateTime::from_timestamp(63, 0).unwrap();
    ///
    /// let result = time::min!([dt1, dt2, dt3]);
    /// assert_eq!(
    ///    result.to_raw().build(),
    ///    "time::min(['1970-01-01T00:01:01Z', '1970-01-01T00:01:02Z', '1970-01-01T00:01:03Z'])"
    /// );
    ///
    /// let rebirth_date = Field::new("rebirth_date");
    /// let result = time::min!(rebirth_date);
    /// assert_eq!(result.to_raw().build(), "time::min(rebirth_date)");
    ///
    /// let param = Param::new("rebirth_date");
    /// let result = time::min!(param);
    /// assert_eq!(result.to_raw().build(), "time::min($rebirth_date)");
    /// ```
    =>
    "min"
);

macro_rules! create_time_fn_with_single_number_arg {
    ($(#[$attr:meta])* => $function_name: expr, $function_path: expr) => {
        paste::paste! {

            $(#[$attr])*
            pub fn [<$function_name _fn>](number: impl ::std::convert::Into<$crate::NumberLike>) -> $crate::Function {
                let number: $crate::NumberLike = number.into();
                let query_string = format!("time::{}({})", $function_path, number.build());

                $crate::Function {
                    query_string,
                    bindings: number.get_bindings(),
                    errors: number.get_errors(),
                }
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<time_from_ $function_name>] {
                ( $number:expr ) => {
                    $crate::functions::time::from::[<$function_name _fn>]($number)
                };
            }

            pub use [<time_from_ $function_name>] as [<$function_name>];

            #[cfg(test)]
            mod [<test_ $function_name _fn>] {
                use $crate::*;
                use crate::functions::time;


                #[test]
                fn [<test_ $function_name _fn_with_number_field>]() {
                    let rebirth_date = $crate::Field::new("rebirth_date");
                    let result = time::from::[<$function_name>]!(rebirth_date);

                    assert_eq!(result.fine_tune_params(), format!("time::{}(rebirth_date)", $function_path));
                    assert_eq!(result.to_raw().build(), format!("time::{}(rebirth_date)", $function_path));
                }

                #[test]
                fn [<test_ $function_name _fn_with_plain_number>]() {
                    let number = $crate::Param::new("number");
                    let result = time::from::[<$function_name>]!(number);
                    assert_eq!(result.fine_tune_params(), format!("time::{}($number)", $function_path));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("time::{}($number)", $function_path)

                    );
                }

                #[test]
                fn [<test_ $function_name _fn_with_plain_number_and_duration>]() {
                    let result = time::from::[<$function_name>]!(20000);
                    assert_eq!(result.fine_tune_params(), format!("time::{}($_param_00000001)", $function_path));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("time::{}(20000)", $function_path)

                    );
                }
            }
        }
    }
}

/// The time::from module contains functions that convert a number into a datetime.
pub mod from {
    use crate::*;

    create_time_fn_with_single_number_arg!(
        /// The time::from::micros function converts a number of microseconds into a datetime.
        /// The function is also aliased as `time_from_micros!`.
        ///
        /// # Arguments
        ///
        /// * `number` - The number of microseconds to convert. Could also be a field or a parameter representing a number of microseconds.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as  surreal_orm;
        /// use surreal_orm::{*, functions::time};
        ///
        /// let result = time::from::micros!(1);
        /// assert_eq!(result.to_raw().build(), "time::from::micros(1)");
        ///
        /// let rebirth_date = Field::new("rebirth_date");
        /// let result = time::from::micros!(rebirth_date);
        /// assert_eq!(result.to_raw().build(), "time::from::micros(rebirth_date)");
        ///
        /// let param = Param::new("rebirth_date");
        /// let result = time::from::micros!(param);
        /// assert_eq!(result.to_raw().build(), "time::from::micros($rebirth_date)");
        /// ```
        =>
        "micros",
        "from::micros"
    );

    create_time_fn_with_single_number_arg!(
        /// The time::from::millis function converts a number of milliseconds into a datetime.
        /// The function is also aliased as `time_from_millis!`.
        ///
        /// # Arguments
        ///
        /// * `number` - The number of milliseconds to convert. Could also be a field or a parameter representing a number of milliseconds.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as  surreal_orm;
        /// use surreal_orm::{*, functions::time};
        ///
        /// let result = time::from::millis!(1);
        /// assert_eq!(result.to_raw().build(), "time::from::millis(1)");
        ///
        /// let rebirth_date = Field::new("rebirth_date");
        /// let result = time::from::millis!(rebirth_date);
        /// assert_eq!(result.to_raw().build(), "time::from::millis(rebirth_date)");
        ///
        /// let param = Param::new("rebirth_date");
        /// let result = time::from::millis!(param);
        /// assert_eq!(result.to_raw().build(), "time::from::millis($rebirth_date)");
        /// ```
        =>
        "millis",
        "from::millis"
    );

    // from::secs
    create_time_fn_with_single_number_arg!(
        /// The time::from::secs function converts a number of seconds into a datetime.
        /// The function is also aliased as `time_from_secs!`.
        ///
        /// # Arguments
        ///
        /// * `number` - The number of seconds to convert. Could also be a field or a parameter representing a number of seconds.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as  surreal_orm;
        /// use surreal_orm::{*, functions::time};
        ///
        /// let result = time::from::secs!(1);
        /// assert_eq!(result.to_raw().build(), "time::from::secs(1)");
        ///
        /// let rebirth_date = Field::new("rebirth_date");
        /// let result = time::from::secs!(rebirth_date);
        /// assert_eq!(result.to_raw().build(), "time::from::secs(rebirth_date)");
        ///
        /// let param = Param::new("rebirth_date");
        /// let result = time::from::secs!(param);
        /// assert_eq!(result.to_raw().build(), "time::from::secs($rebirth_date)");
        /// ```
        =>
        "secs",
        "from::secs"
    );

    // from::unix
    create_time_fn_with_single_number_arg!(
        /// The time::from::unix function converts a number of seconds since the Unix epoch into a datetime.
        /// The function is also aliased as `time_from_unix!`.
        ///
        /// # Arguments
        ///
        /// * `number` - The number of seconds since the Unix epoch to convert. Could also be a field or a parameter representing a number of seconds since the Unix epoch.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as  surreal_orm;
        /// use surreal_orm::{*, functions::time};
        ///
        /// let result = time::from::unix!(1);
        /// assert_eq!(result.to_raw().build(), "time::from::unix(1)");
        ///
        /// let rebirth_date = Field::new("rebirth_date");
        /// let result = time::from::unix!(rebirth_date);
        /// assert_eq!(result.to_raw().build(), "time::from::unix(rebirth_date)");
        ///
        /// let param = Param::new("rebirth_date");
        /// let result = time::from::unix!(param);
        /// assert_eq!(result.to_raw().build(), "time::from::unix($rebirth_date)");
        /// ```
        =>
        "unix",
        "from::unix"
    );
}
