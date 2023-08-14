/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Type functions
// These functions can be used for generating and coercing data to specific data types. These functions are useful when accepting input values in client libraries, and ensuring that they are the desired type within SQL statements.
//
// Function	Description
// type::bool()	Converts a value into a boolean
// type::datetime()	Converts a value into a datetime
// type::decimal()	Converts a value into a decimal
// type::duration()	Converts a value into a duration
// type::float()	Converts a value into a floating point number
// type::int()	Converts a value into an integer
// type::number()	Converts a value into a number
// type::point()	Converts a value into a geometry point
// type::regex()	Converts a value into a regular expression
// type::string()	Converts a value into a string
// type::table()	Converts a value into a table
// type::thing()	Converts a value into a record pointer

use crate::{
    Buildable, DatetimeLike, DurationLike, Erroneous, Function, NumberLike, Parametric, StrandLike,
    TableLike, Valuex,
};

macro_rules! create_type {
    ($(#[$attr:meta])* => $function_name:expr, $value_type: ty, $test_data_input:expr, $test_stringified_data_output: expr) => {
        paste::paste! {

            $(#[$attr])*
            pub fn [<$function_name _fn>](value: impl Into<$value_type>) -> $crate::Function {
                let value: $value_type = value.into();
                let query_string = format!("type::{}({})", $function_name, value.build());

                $crate::Function {
                    query_string,
                    bindings: value.get_bindings(),
                    errors: value.get_errors(),
                }
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<type_ $function_name>] {
                ( $string:expr ) => {
                    $crate::functions::type_::[<$function_name _fn>]($string)
                };
            }
            pub use [<type_ $function_name>] as [<$function_name>];

            #[cfg(test)]
            mod [<test_ $function_name>] {
                use super::*;
                use crate::*;
                use crate::functions::type_;

                #[test]
                fn [<test_ $function_name _with_field>]() {
                    let name = Field::new("name");
                    let result = type_::[<$function_name _fn>](name);
                    assert_eq!(result.fine_tune_params(), format!("type::{}(name)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("type::{}(name)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _with_plain_string>]() {
                    let result = type_::[<$function_name _fn>]($test_data_input);
                    assert_eq!(result.fine_tune_params(), format!("type::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("type::{}({})", $function_name, $test_stringified_data_output));
                }
            }
        }
    };
}

// The type::bool function converts a value into a bool, if the value is truthy.
create_type!(
    /// The type::bool function converts a value into a bool, if the value is truthy.
    /// Also aliased as `type_bool!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a bool. Could also be a field or a parameter representing the value.
    /// 
    /// # Example
    /// ```
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::type_};
    /// let result = type_::bool!(1234);
    /// assert_eq!(result.to_raw().build(), "type::bool(1234)");
    ///
    /// let bool_field = Field::new("bool_field");
    /// let result = type_::bool!(bool_field);
    /// assert_eq!(result.to_raw().build(), "type::bool(bool_field)");
    ///
    /// let bool_param = Param::new("bool_param");
    /// let result = type_::bool!(bool_param);
    /// assert_eq!(result.to_raw().build(), "type::bool($bool_param)");
    /// ```
    => 
    "bool", Valuex, "toronto", "'toronto'");

create_type!(
    /// The type::datetime function converts a value into a datetime.
    /// Also aliased as `type_datetime!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a datetime. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::type_};
    /// let datetime = ::chrono::DateTime::<chrono::Utc>::from_utc(
    ///     chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    ///    chrono::Utc,
    /// );
    /// let result = type_::datetime!(datetime);
    /// assert_eq!(result.to_raw().build(), "type::datetime('1970-01-01T00:01:01Z')");
    ///
    /// let datetime_field = Field::new("datetime_field");
    /// let result = type_::datetime!(datetime_field);
    /// assert_eq!(result.to_raw().build(), "type::datetime(datetime_field)");
    ///
    /// let datetime_param = Param::new("datetime_param");
    /// let result = type_::datetime!(datetime_param);
    /// assert_eq!(result.to_raw().build(), "type::datetime($datetime_param)");
    /// ```
    =>
    "datetime",
    DatetimeLike,
    chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
        chrono::Utc,
    ),
    "'1970-01-01T00:01:01Z'"
);

// The type::duration function converts a value into a duration.
create_type!(
    /// The type::duration function converts a value into a duration.
    /// Also aliased as `type_duration!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a duration. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::type_};
    ///
    /// let duration = std::time::Duration::from_secs(60 * 60);
    /// let result = type_::duration!(duration);
    /// assert_eq!(result.to_raw().build(), "type::duration(1h)");
    ///
    /// let duration_field = Field::new("duration_field");
    /// let result = type_::duration!(duration_field);
    /// assert_eq!(result.to_raw().build(), "type::duration(duration_field)");
    ///
    /// let duration_param = Param::new("duration_param");
    /// let result = type_::duration!(duration_param);
    /// assert_eq!(result.to_raw().build(), "type::duration($duration_param)");
    /// ```
    =>
    "duration",
    DurationLike,
    std::time::Duration::from_secs(24 * 60 * 60 * 7),
    "1w"
);

create_type!(
    /// The type::float function converts a value into a float.
    /// Also aliased as `type_float!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a float. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::type_};
    /// let result = type_::float!(1234);
    /// assert_eq!(result.to_raw().build(), "type::float(1234)");
    ///
    /// let float_field = Field::new("float_field");
    /// let result = type_::float!(float_field);
    /// assert_eq!(result.to_raw().build(), "type::float(float_field)");
    ///
    /// let float_param = Param::new("float_param");
    /// let result = type_::float!(float_param);
    /// assert_eq!(result.to_raw().build(), "type::float($float_param)");
    /// ```
    =>
    "float", NumberLike, 43.5, "43.5f"
);

create_type!(
    /// The type::int function converts a value into an int.
    /// Also aliased as `type_int!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to an int. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::type_};
    /// let result = type_::int!(1234);
    /// assert_eq!(result.to_raw().build(), "type::int(1234)");
    ///
    /// let int_field = Field::new("int_field");
    /// let result = type_::int!(int_field);
    /// assert_eq!(result.to_raw().build(), "type::int(int_field)");
    ///
    /// let int_param = Param::new("int_param");
    /// let result = type_::int!(int_param);
    /// assert_eq!(result.to_raw().build(), "type::int($int_param)");
    /// ```
    =>
    "int", NumberLike, 99, 99
);

create_type!(
    /// The type::string function converts a value into a string.
    /// Also aliased as `type_string!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a string. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::type_};
    /// let result = type_::string!(1234);
    /// assert_eq!(result.to_raw().build(), "type::string(1234)");
    ///
    /// let string_field = Field::new("string_field");
    /// let result = type_::string!(string_field);
    /// assert_eq!(result.to_raw().build(), "type::string(string_field)");
    ///
    /// let string_param = Param::new("string_param");
    /// let result = type_::string!(string_param);
    /// assert_eq!(result.to_raw().build(), "type::string($string_param)");
    /// ```
    =>
    "number", NumberLike, 5, 5
);

create_type!(
    /// The type::string function converts a value into a string.
    /// Also aliased as `type_string!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a string. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::type_};
    /// let result = type_::string!(1234);
    /// assert_eq!(result.to_raw().build(), "type::string(1234)");
    ///
    /// let string_field = Field::new("string_field");
    /// let result = type_::string!(string_field);
    /// assert_eq!(result.to_raw().build(), "type::string(string_field)");
    ///
    /// let string_param = Param::new("string_param");
    /// let result = type_::string!(string_param);
    /// assert_eq!(result.to_raw().build(), "type::string($string_param)");
    /// ```
    =>
    "string", Valuex, 5454, "5454"
);

create_type!(
    /// The type::regex function converts a value into a regex.
    /// Also aliased as `type_regex!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a regex. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::type_};
    /// let result = type_::regex!("/[A-Z]{3}/");
    /// assert_eq!(result.to_raw().build(), "type::regex('/[A-Z]{3}/')");
    ///
    /// let regex_field = Field::new("regex_field");
    /// let result = type_::regex!(regex_field);
    /// assert_eq!(result.to_raw().build(), "type::regex(regex_field)");
    ///
    /// let regex_param = Param::new("regex_param");
    /// let result = type_::regex!(regex_param);
    /// assert_eq!(result.to_raw().build(), "type::regex($regex_param)");
    /// ```
    =>
    "regex", StrandLike, "/[A-Z]{3}/", "'/[A-Z]{3}/'"
);

create_type!(
    /// The type::table function converts a value into a table definition.
    /// Also aliased as `type_table!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a table definition. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::type_, statements::let_};
    /// let result = type_::table!("user");
    /// assert_eq!(result.to_raw().build(), "type::table(user)");
    ///
    /// let table_field = Field::new("table_field");
    /// let result = type_::table!(table_field);
    /// assert_eq!(result.to_raw().build(), "type::table(table_field)");
    ///
    /// let table_param = let_("table_param").equal_to("user").get_param();
    /// let result = type_::table!(table_param);
    /// assert_eq!(result.to_raw().build(), "type::table($table_param)");
    /// ```
    =>
    "table", TableLike, Table::new("user"), "user"
);

/// The type::point function converts a value into a geometry point.
pub fn point_fn(point1: impl Into<NumberLike>, point2: impl Into<NumberLike>) -> Function {
    let point1: NumberLike = point1.into();
    let point2: NumberLike = point2.into();
    let mut bindings = point1.get_bindings();
    let mut errors = point1.get_errors();

    bindings.extend(point2.get_bindings());
    errors.extend(point2.get_errors());

    let query_string = format!("type::point({}, {})", point1.build(), point2.build());

    Function {
        query_string,
        bindings,
        errors,
    }
}

/// The type::point function converts a value into a geometry point.
/// Also aliased as `type_point!`
///
/// # Arguments
/// * `point1` - The first point of the geometry point. Could also be a field or a parameter
/// representing the value.
/// * `point2` - The second point of the geometry point. Could also be a field or a parameter
/// representing the value.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::type_, statements::let_};
/// let result = type_::point!(1234, 1234);
/// assert_eq!(result.to_raw().build(), "type::point(1234, 1234)");
///
/// let point1_field = Field::new("point1_field");
/// let point2_field = Field::new("point2_field");
/// let result = type_::point!(point1_field, point2_field);
/// assert_eq!(result.to_raw().build(), "type::point(point1_field, point2_field)");
///
/// let point1_param = let_("point1_param").equal_to(1234).get_param();
/// let point2_param = let_("point2_param").equal_to(1234).get_param();
/// let result = type_::point!(point1_param, point2_param);
/// assert_eq!(result.to_raw().build(), "type::point($point1_param, $point2_param)");
/// ```
#[macro_export]
macro_rules! type_point {
    ( $point1:expr, $point2:expr ) => {
        $crate::functions::type_::point_fn($point1, $point2)
    };
}

pub use type_point as point;

/// The type::thing function converts a value into a record pointer definition.
pub fn thing_fn(table: impl Into<TableLike>, value: impl Into<Valuex>) -> Function {
    let table: TableLike = table.into();
    let value: Valuex = value.into();
    let mut bindings = table.get_bindings();
    let mut errors = table.get_errors();

    bindings.extend(value.get_bindings());
    errors.extend(value.get_errors());

    let query_string = format!("type::thing({}, {})", table.build(), value.build());

    Function {
        query_string,
        bindings,
        errors,
    }
}

/// The type::thing function converts a value into a record pointer definition.
/// Also aliased as `type_thing!`
///
/// # Arguments
/// * `table` - The table to be converted to a record pointer definition. Could also be a field or a parameter
/// representing the value.
/// * `value` - The value to be converted to a record pointer definition. Could also be a field or a parameter
/// representing the value.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::type_, statements::let_};
/// let result = type_::thing!("user", 1234);
/// assert_eq!(result.to_raw().build(), "type::thing(user, 1234)");
///
/// let table_field = Field::new("table_field");
/// let value_field = Field::new("value_field");
/// let result = type_::thing!(table_field, value_field);
/// assert_eq!(result.to_raw().build(), "type::thing(table_field, value_field)");
///
/// let table_param = let_("table_param").equal_to("user").get_param();
/// let value_param = let_("value_param").equal_to(1234).get_param();
/// let result = type_::thing!(table_param, value_param);
/// assert_eq!(result.to_raw().build(), "type::thing($table_param, $value_param)");
/// ```
#[macro_export]
macro_rules! type_thing {
    ( $table:expr, $value:expr ) => {
        $crate::functions::type_::thing_fn($table, $value)
    };
}

pub use type_thing as thing;

#[cfg(test)]
mod tests {
    use crate::functions::type_;
    use crate::*;

    #[test]
    fn test_bool_with_macro_with_plain_number() {
        let result = type_::bool!(43545);
        assert_eq!(result.fine_tune_params(), "type::bool($_param_00000001)");
        assert_eq!(result.to_raw().build(), "type::bool(43545)");
    }

    #[test]
    fn test_bool_with_macro_with_plain_false() {
        let result = type_::bool!(false);
        assert_eq!(result.fine_tune_params(), "type::bool($_param_00000001)");
        assert_eq!(result.to_raw().build(), "type::bool(false)");
    }

    #[test]
    fn test_bool_with_macro_with_plain_true() {
        let result = type_::bool!(true);
        assert_eq!(result.fine_tune_params(), "type::bool($_param_00000001)");
        assert_eq!(result.to_raw().build(), "type::bool(true)");
    }

    #[test]
    fn test_datetime_macro_with_plain_datetime() {
        let value = chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
            chrono::Utc,
        );
        let result = type_::datetime!(value);
        assert_eq!(
            result.fine_tune_params(),
            "type::datetime($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "type::datetime('1970-01-01T00:01:01Z')"
        );
    }

    #[test]
    fn test_datetime_macro_with_datetime_field() {
        let rebirth_date = Field::new("rebirth_date");
        let result = type_::datetime!(rebirth_date);

        assert_eq!(result.fine_tune_params(), "type::datetime(rebirth_date)");
        assert_eq!(result.to_raw().build(), "type::datetime(rebirth_date)");
    }

    #[test]
    fn test_datetime_macro_with_param() {
        let rebirth_date = Param::new("rebirth_date");
        let result = type_::datetime!(rebirth_date);

        assert_eq!(result.fine_tune_params(), "type::datetime($rebirth_date)");
        assert_eq!(result.to_raw().build(), "type::datetime($rebirth_date)");
    }

    #[test]
    fn test_point_macro_with_plain_values() {
        let result = type_::point!(51.509865, -0.118092);
        assert_eq!(
            result.fine_tune_params(),
            "type::point($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "type::point(51.509865f, -0.118092f)"
        );
    }

    #[test]
    fn test_point_macro_with_fields() {
        let home = Field::new("home");
        let away = Field::new("away");
        let result = type_::point!(home, away);
        assert_eq!(result.fine_tune_params(), "type::point(home, away)");
        assert_eq!(result.to_raw().build(), "type::point(home, away)");
    }

    #[test]
    fn test_thing_macro_with_plain_values() {
        let user = Table::from("user");
        let id = "oyelowo";
        let result = type_::thing!(user, id);
        assert_eq!(
            result.fine_tune_params(),
            "type::thing($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "type::thing(user, 'oyelowo')");
    }

    #[test]
    fn test_thing_macro_with_datetime_field() {
        let table = Table::new("table");
        let id = Field::new("id");
        let result = type_::thing!(table, id);

        assert_eq!(
            result.fine_tune_params(),
            "type::thing($_param_00000001, id)"
        );
        assert_eq!(result.to_raw().build(), "type::thing(table, id)");
    }
}
