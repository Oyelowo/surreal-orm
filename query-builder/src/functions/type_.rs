/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
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
// type::field()	Projects a single field within a SELECT statement
// type::fields()	Projects a multiple fields within a SELECT statement
// type::float()	Converts a value into a floating point number
// type::int()	Converts a value into an integer
// type::number()	Converts a value into a number
// type::point()	Converts a value into a geometry point
// type::regex()	Converts a value into a regular expression
// type::string()	Converts a value into a string
// type::table()	Converts a value into a table
// type::thing()	Converts a value into a record pointer
// type::is::array()	Checks if given value is of type array
// type::is::bool()	Checks if given value is of type bool
// type::is::bytes()	Checks if given value is of type bytes
// type::is::collection()	Checks if given value is of type collection
// type::is::datetime()	Checks if given value is of type datetime
// type::is::decimal()	Checks if given value is of type decimal
// type::is::duration()	Checks if given value is of type duration
// type::is::float()	Checks if given value is of type float
// type::is::geometry()	Checks if given value is of type geometry
// type::is::int()	Checks if given value is of type int
// type::is::line()	Checks if given value is of type line
// type::is::null()	Checks if given value is of type null
// type::is::multiline()	Checks if given value is of type multiline
// type::is::multipoint()	Checks if given value is of type multipoint
// type::is::multipolygon()	Checks if given value is of type multipolygon
// type::is::number()	Checks if given value is of type number
// type::is::object()	Checks if given value is of type object
// type::is::point()	Checks if given value is of type point
// type::is::polygon()	Checks if given value is of type polygon
// type::is::record()	Checks if given value is of type record
// type::is::string()	Checks if given value is of type string
// type::is::uuid()	Checks if given value is of type uuid

use crate::{
    ArrayLike, Buildable, DatetimeLike, DurationLike, Erroneous, Function, NumberLike, Parametric,
    StrandLike, TableLike, ValueLike,
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
    "bool", ValueLike, "toronto", "'toronto'");

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
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
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
    chrono::DateTime::from_timestamp(61, 0).unwrap(),
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
    /// The type::field function projects a single field within a SELECT statement.
    /// Also aliased as `type_field!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a field. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::type_};
    /// let result = type_::field!("name");
    /// assert_eq!(result.to_raw().build(), "type::field('name')");
    ///
    /// let field_field = Field::new("field_field");
    /// let result = type_::field!(field_field);
    /// assert_eq!(result.to_raw().build(), "type::field(field_field)");
    ///
    /// let field_param = Param::new("field_param");
    /// let result = type_::field!(field_param);
    /// assert_eq!(result.to_raw().build(), "type::field($field_param)");
    /// ```
    =>
    "field", ValueLike, "name", "'name'"
);

create_type!(
    /// The type::fields function projects multiple fields within a SELECT statement.
    /// Also aliased as `type_fields!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a field. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{
    ///     *,
    ///     functions::type_,
    ///     statements::let_,
    /// };
    /// let result = type_::fields!(["name", "age"]);
    /// assert_eq!(result.to_raw().build(), "type::fields(['name', 'age'])");
    ///
    /// let field_field = Field::new("field_field");
    /// let result = type_::fields!(field_field);
    /// assert_eq!(result.to_raw().build(), "type::fields(field_field)");
    ///
    /// let field_param = Param::new("field_param");
    /// let result = type_::fields!(field_param);
    /// assert_eq!(result.to_raw().build(), "type::fields($field_param)");
    /// ```
    =>
    "fields", ArrayLike, ["name"], "['name']"
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
    "string", ValueLike, 5454, "5454"
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
pub fn thing_fn(table: impl Into<TableLike>, value: impl Into<ValueLike>) -> Function {
    let table: TableLike = table.into();
    let value: ValueLike = value.into();
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
        let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
        let result = type_::datetime!(dt);
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

// create_is_function macro
macro_rules! create_is_function {
    ($(#[$attr:meta])* => $function_name:expr, $value_type: ty, $test_data_input:expr, $test_stringified_data_output: expr) => {
        paste::paste! {

            $(#[$attr])*
            pub fn [<$function_name _fn>](value: impl Into<$value_type>) -> $crate::Function {
                let value: $value_type = value.into();
                let query_string = format!("type::is::{}({})", $function_name, value.build());

                $crate::Function {
                    query_string,
                    bindings: value.get_bindings(),
                    errors: value.get_errors(),
                }
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<type_is_ $function_name>] {
                ( $string:expr ) => {
                    $crate::functions::type_::is::[<$function_name _fn>]($string)
                };
            }
            pub use [<type_is_ $function_name>] as [<$function_name>];

            #[cfg(test)]
            mod [<test_is_ $function_name>] {
                use super::*;
                use crate::*;
                use crate::functions::type_;

                #[test]
                fn [<test_is_ $function_name _with_field>]() {
                    let name = Field::new("name");
                    let result = type_::is::[<$function_name _fn>](name);
                    assert_eq!(result.fine_tune_params(), format!("type::is::{}(name)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("type::is::{}(name)", $function_name));
                }

                #[test]
                fn [<test_is_ $function_name _with_plain_string>]() {
                    let result = type_::is::[<$function_name _fn>]($test_data_input);
                    assert_eq!(result.fine_tune_params(), format!("type::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("type::is::{}({})", $function_name, $test_stringified_data_output));
                }
            }
        }
    };
}

/// The type_::is functions check if given value is of a specific type.
pub mod is {
    use crate::{
        Buildable, DatetimeLike, DurationLike, Erroneous, NumberLike, ObjectLike, Parametric,
        StrandLike, ThingLike, ValueLike,
    };

    create_is_function!(
        /// The type::is::array function checks if given value is of type array.
        /// Also aliased as `type_is_array!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type array. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        /// let result = type_::is::array!(1234);
        /// assert_eq!(result.to_raw().build(), "type::is::array(1234)");
        ///
        /// let array_field = Field::new("array_field");
        /// let result = type_::is::array!(array_field);
        /// assert_eq!(result.to_raw().build(), "type::is::array(array_field)");
        ///
        /// let array_param = Param::new("array_param");
        /// let result = type_::is::array!(array_param);
        /// assert_eq!(result.to_raw().build(), "type::is::array($array_param)");
        /// ```
        =>
        "array", ValueLike, 5454, "5454"
    );

    create_is_function!(
        /// The type::is::bool function checks if given value is of type bool.
        /// Also aliased as `type_is_bool!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type bool. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        /// let result = type_::is::bool!(1234);
        /// assert_eq!(result.to_raw().build(), "type::is::bool(1234)");
        ///
        /// let bool_field = Field::new("bool_field");
        /// let result = type_::is::bool!(bool_field);
        /// assert_eq!(result.to_raw().build(), "type::is::bool(bool_field)");
        ///
        /// let bool_param = Param::new("bool_param");
        /// let result = type_::is::bool!(bool_param);
        /// assert_eq!(result.to_raw().build(), "type::is::bool($bool_param)");
        /// ```
        =>
        "bool", ValueLike, 5454, "5454"
    );

    create_is_function!(
        /// The type::is::bytes function checks if given value is of type bytes.
        /// Also aliased as `type_is_bytes!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type bytes. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        /// let result = type_::is::bytes!(1234);
        /// assert_eq!(result.to_raw().build(), "type::is::bytes(1234)");
        ///
        /// let bytes_field = Field::new("bytes_field");
        /// let result = type_::is::bytes!(bytes_field);
        /// assert_eq!(result.to_raw().build(), "type::is::bytes(bytes_field)");
        ///
        /// let bytes_param = Param::new("bytes_param");
        /// let result = type_::is::bytes!(bytes_param);
        /// assert_eq!(result.to_raw().build(), "type::is::bytes($bytes_param)");
        /// ```
        =>
        "bytes", ValueLike, 5454, "5454"
    );

    create_is_function!(
        /// The type::is::datetime function checks if given value is of type datetime.
        /// Also aliased as `type_is_datetime!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type datetime. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
        /// let result = type_::is::datetime!(datetime);
        /// assert_eq!(result.to_raw().build(), "type::is::datetime('1970-01-01T00:01:01Z')");
        ///
        /// let datetime_field = Field::new("datetime_field");
        /// let result = type_::is::datetime!(datetime_field);
        /// assert_eq!(result.to_raw().build(), "type::is::datetime(datetime_field)");
        ///
        /// let datetime_param = Param::new("datetime_param");
        /// let result = type_::is::datetime!(datetime_param);
        /// assert_eq!(result.to_raw().build(), "type::is::datetime($datetime_param)");
        /// ```
        =>
        "datetime",
        DatetimeLike,
        chrono::DateTime::from_timestamp(61, 0).unwrap(),
        "'1970-01-01T00:01:01Z'"
    );

    create_is_function!(
        /// The type::is::collection function checks if given value is of type collection.
        /// Also aliased as `type_is_collection!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type collection. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let result = type_::is::collection!(1234);
        /// assert_eq!(result.to_raw().build(), "type::is::collection(1234)");
        ///
        /// let collection_field = Field::new("collection_field");
        /// let result = type_::is::collection!(collection_field);
        /// assert_eq!(result.to_raw().build(), "type::is::collection(collection_field)");
        ///
        /// let collection_param = Param::new("collection_param");
        /// let result = type_::is::collection!(collection_param);
        /// assert_eq!(result.to_raw().build(), "type::is::collection($collection_param)");
        /// ```
        =>
        "collection", ValueLike, 5454, "5454"
    );

    create_is_function!(
        /// The type::is::decimal function checks if given value is of type decimal.
        /// Also aliased as `type_is_decimal!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type decimal. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        /// let result = type_::is::decimal!(1234);
        /// assert_eq!(result.to_raw().build(), "type::is::decimal(1234)");
        ///
        /// let decimal_field = Field::new("decimal_field");
        /// let result = type_::is::decimal!(decimal_field);
        /// assert_eq!(result.to_raw().build(), "type::is::decimal(decimal_field)");
        ///
        /// let decimal_param = Param::new("decimal_param");
        /// let result = type_::is::decimal!(decimal_param);
        /// assert_eq!(result.to_raw().build(), "type::is::decimal($decimal_param)");
        /// ```
        =>
        "decimal", ValueLike, 5454, "5454"
    );

    create_is_function!(
        /// The type::is::duration function checks if given value is of type duration.
        /// Also aliased as `type_is_duration!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type duration. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let duration = std::time::Duration::from_secs(60 * 60);
        /// let result = type_::is::duration!(duration);
        /// assert_eq!(result.to_raw().build(), "type::is::duration(1h)");
        ///
        /// let duration_field = Field::new("duration_field");
        /// let result = type_::is::duration!(duration_field);
        /// assert_eq!(result.to_raw().build(), "type::is::duration(duration_field)");
        ///
        /// let duration_param = Param::new("duration_param");
        /// let result = type_::is::duration!(duration_param);
        /// assert_eq!(result.to_raw().build(), "type::is::duration($duration_param)");
        /// ```
        =>
        "duration",
        DurationLike,
        std::time::Duration::from_secs(24 * 60 * 60 * 7),
        "1w"
    );

    create_is_function!(
        /// The type::is::float function checks if given value is of type float.
        /// Also aliased as `type_is_float!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type float. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        /// let result = type_::is::float!(1234);
        /// assert_eq!(result.to_raw().build(), "type::is::float(1234)");
        ///
        /// let float_field = Field::new("float_field");
        /// let result = type_::is::float!(float_field);
        /// assert_eq!(result.to_raw().build(), "type::is::float(float_field)");
        ///
        /// let float_param = Param::new("float_param");
        /// let result = type_::is::float!(float_param);
        /// assert_eq!(result.to_raw().build(), "type::is::float($float_param)");
        /// ```
        =>
        "float", NumberLike, 5454, "5454"
    );

    create_is_function!(
        /// The type::is::int function checks if given value is of type int.
        /// Also aliased as `type_is_int!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type int. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        /// let result = type_::is::int!(1234);
        /// assert_eq!(result.to_raw().build(), "type::is::int(1234)");
        ///
        /// let int_field = Field::new("int_field");
        /// let result = type_::is::int!(int_field);
        /// assert_eq!(result.to_raw().build(), "type::is::int(int_field)");
        ///
        /// let int_param = Param::new("int_param");
        /// let result = type_::is::int!(int_param);
        /// assert_eq!(result.to_raw().build(), "type::is::int($int_param)");
        /// ```
        =>
        "int", NumberLike, 5454, "5454"
    );

    create_is_function!(
        /// The type::is::null function checks if given value is of type null.
        /// Also aliased as `type_is_null!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type null. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let result = type_::is::null!(1234);
        /// assert_eq!(result.to_raw().build(), "type::is::null(1234)");
        ///
        /// let null_field = Field::new("null_field");
        /// let result = type_::is::null!(null_field);
        /// assert_eq!(result.to_raw().build(), "type::is::null(null_field)");
        ///
        /// let null_param = Param::new("null_param");
        /// let result = type_::is::null!(null_param);
        /// assert_eq!(result.to_raw().build(), "type::is::null($null_param)");
        /// ```
        =>
        "null", ValueLike, 5454, "5454"
    );

    create_is_function!(
        /// The type::is::number function checks if given value is of type number.
        /// Also aliased as `type_is_number!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type number. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        /// let result = type_::is::number!(1234);
        /// assert_eq!(result.to_raw().build(), "type::is::number(1234)");
        ///
        /// let number_field = Field::new("number_field");
        /// let result = type_::is::number!(number_field);
        /// assert_eq!(result.to_raw().build(), "type::is::number(number_field)");
        ///
        /// let number_param = Param::new("number_param");
        /// let result = type_::is::number!(number_param);
        /// assert_eq!(result.to_raw().build(), "type::is::number($number_param)");
        /// ```
        =>
        "number", NumberLike, 5454, "5454"
    );

    create_is_function!(
        /// The type::is::string function checks if given value is of type string.
        /// Also aliased as `type_is_string!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type string. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        ///
        /// use surreal_orm::{*, functions::type_};
        /// let result = type_::is::string!("Oyelowo");
        /// assert_eq!(result.to_raw().build(), "type::is::string('Oyelowo')");
        ///
        /// let string_field = Field::new("string_field");
        /// let result = type_::is::string!(string_field);
        /// assert_eq!(result.to_raw().build(), "type::is::string(string_field)");
        ///
        /// let string_param = Param::new("string_param");
        /// let result = type_::is::string!(string_param);
        /// assert_eq!(result.to_raw().build(), "type::is::string($string_param)");
        /// ```
        =>
        "string", StrandLike, "Oyelowo", "'Oyelowo'"
    );

    create_is_function!(
        /// The type::is::geometry function checks if given value is of type geometry.
        /// Also aliased as `type_is_geometry!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type geometry. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let result = type_::is::geometry!(geo::point!(x: 51.509865, y: -0.118092));
        /// assert_eq!(result.to_raw().build(), "type::is::geometry((51.509865, -0.118092))");
        ///
        /// let geometry_field = Field::new("geometry_field");
        /// let result = type_::is::geometry!(geometry_field);
        /// assert_eq!(result.to_raw().build(), "type::is::geometry(geometry_field)");
        ///
        /// let geometry_param = Param::new("geometry_param");
        /// let result = type_::is::geometry!(geometry_param);
        /// assert_eq!(result.to_raw().build(), "type::is::geometry($geometry_param)");
        /// ```
        =>
        "geometry", crate::GeometryLike, geo::point!(x: 51.509865, y: -0.118092), "(51.509865, -0.118092)"
    );

    create_is_function!(
        /// The type::is::line function checks if given value is of type line.
        /// Also aliased as `type_is_line!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type line. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let result = type_::is::line!(geo::LineString(vec![
        ///    geo::Coord {
        ///    x: -122.33583,
        ///    y: 47.60621,
        ///    },
        ///    geo::Coord {
        ///    x: -122.33583,
        ///    y: 47.60622,
        ///    },
        /// ]));
        /// assert_eq!(result.to_raw().build(),
        /// "type::is::line({ type: 'LineString', coordinates: [[-122.33583, 47.60621], [-122.33583, 47.60622]] })");
        ///
        /// let line_field = Field::new("line_field");
        /// let result = type_::is::line!(line_field);
        /// assert_eq!(result.to_raw().build(), "type::is::line(line_field)");
        ///
        /// let line_param = Param::new("line_param");
        /// let result = type_::is::line!(line_param);
        /// assert_eq!(result.to_raw().build(), "type::is::line($line_param)");
        /// ```
        =>
        "line", crate::GeometryLike, geo::LineString(vec![
        geo::Coord {
            x: -122.33583,
            y: 47.60621,
        },
        geo::Coord {
            x: -122.33583,
            y: 47.60622,
        },
        geo::Coord {
            x: -122.33584,
            y: 47.60622,
        },
        geo::Coord {
            x: -122.33584,
            y: 47.60621,
        },
        geo::Coord {
            x: -122.33583,
            y: 47.60621,
        },
    ]), "{ type: 'LineString', coordinates: [[-122.33583, 47.60621], [-122.33583, 47.60622], [-122.33584, 47.60622], [-122.33584, 47.60621], [-122.33583, 47.60621]] }"
    );

    create_is_function!(
        /// The type::is::multiline function checks if given value is of type multiline.
        /// Also aliased as `type_is_multiline!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type multiline. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let result = type_::is::multiline!(geo::MultiLineString(vec![]));
        /// println!("{}", result.to_raw().build());
        ///
        /// let multiline_field = Field::new("multiline_field");
        /// let result = type_::is::multiline!(multiline_field);
        /// assert_eq!(result.to_raw().build(), "type::is::multiline(multiline_field)");
        ///
        /// let multiline_param = Param::new("multiline_param");
        /// let result = type_::is::multiline!(multiline_param);
        /// assert_eq!(result.to_raw().build(), "type::is::multiline($multiline_param)");
        /// ```
        =>
        "multiline", crate::GeometryLike, geo::MultiLineString(vec![]), "{ type: 'MultiLineString', coordinates: [] }"
    );

    create_is_function!(
        /// The type::is::multipoint function checks if given value is of type multipoint.
        /// Also aliased as `type_is_multipoint!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type multipoint. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let result = type_::is::multipoint!(geo::MultiPoint(vec![(0.0, 0.0).into(), (1.0, 1.0).into(), (2.0, 35.0).into()]));
        /// assert_eq!(result.to_raw().build(),
        /// "type::is::multipoint({ type: 'MultiPoint', coordinates: [[0, 0], [1, 1], [2, 35]] })");
        ///
        /// let multipoint_field = Field::new("multipoint_field");
        /// let result = type_::is::multipoint!(multipoint_field);
        /// assert_eq!(result.to_raw().build(), "type::is::multipoint(multipoint_field)");
        ///
        /// let multipoint_param = Param::new("multipoint_param");
        /// let result = type_::is::multipoint!(multipoint_param);
        /// assert_eq!(result.to_raw().build(), "type::is::multipoint($multipoint_param)");
        /// ```
        =>
        "multipoint", crate::GeometryLike, geo::MultiPoint(vec![
        geo::Point::new(0.0, 0.0),
        geo::Point::new(1.0, 1.0),
        (2.0, 35.0).into(),
    ]), "{ type: 'MultiPoint', coordinates: [[0, 0], [1, 1], [2, 35]] }"
    );

    create_is_function!(
        /// The type::is::multipolygon function checks if given value is of type multipolygon.
        /// Also aliased as `type_is_multipolygon!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type multipolygon. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let result = type_::is::multipolygon!(geo::MultiPolygon(vec![]));
        ///
        /// assert_eq!(result.to_raw().build(),
        /// "type::is::multipolygon({ type: 'MultiPolygon', coordinates: [] })");
        /// let multipolygon_field = Field::new("multipolygon_field");
        /// let result = type_::is::multipolygon!(multipolygon_field);
        /// assert_eq!(result.to_raw().build(), "type::is::multipolygon(multipolygon_field)");
        /// let multipolygon_param = Param::new("multipolygon_param");
        /// let result = type_::is::multipolygon!(multipolygon_param);
        /// assert_eq!(result.to_raw().build(), "type::is::multipolygon($multipolygon_param)");
        /// ```
    =>
    "multipolygon", crate::GeometryLike, geo::MultiPolygon(vec![]), "{ type: 'MultiPolygon', coordinates: [] }"
    );

    create_is_function!(
        /// The type::is::point function checks if given value is of type point.
        /// Also aliased as `type_is_point!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type point. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let result = type_::is::point!(geo::Point::new(0.0, 0.0));
        /// assert_eq!(result.to_raw().build(), "type::is::point((0, 0))");
        ///
        /// let point_field = Field::new("point_field");
        /// let result = type_::is::point!(point_field);
        /// assert_eq!(result.to_raw().build(), "type::is::point(point_field)");
        ///
        /// let point_param = Param::new("point_param");
        /// let result = type_::is::point!(point_param);
        /// assert_eq!(result.to_raw().build(), "type::is::point($point_param)");
        /// ```
        =>
        "point", crate::GeometryLike, geo::Point::new(0.0, 0.0), "(0, 0)"
    );

    create_is_function!(
    /// The type::is::polygon function checks if given value is of type polygon.
    /// Also aliased as `type_is_polygon!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type polygon. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::type_};
    ///
    /// let result = type_::is::polygon!(geo::Polygon::new(
    ///    geo::LineString(vec![
    ///         geo::Coord {
    ///             x: -122.33583,
    ///             y: 47.60621,
    ///         },
    ///         geo::Coord {
    ///             x: -122.33583,
    ///             y: 47.60622,
    ///         },
    ///    ]),
    ///    vec![geo::LineString(vec![
    ///             geo::Coord {
    ///                 x: -122.33583,
    ///                 y: 47.60621,
    ///             },
    ///             geo::Coord {
    ///                 x: -122.33583,
    ///                 y: 47.60622,
    ///             },
    ///         ])],
    ///    ));
    ///    println!("{}", result.to_raw().build());
    ///
    ///    let polygon_field = Field::new("polygon_field");
    ///    let result = type_::is::polygon!(polygon_field);
    ///    assert_eq!(result.to_raw().build(), "type::is::polygon(polygon_field)");
    ///
    ///    let polygon_param = Param::new("polygon_param");
    ///    let result = type_::is::polygon!(polygon_param);
    ///    assert_eq!(result.to_raw().build(), "type::is::polygon($polygon_param)");
    ///    ```
    =>
    "polygon", crate::GeometryLike, geo::Polygon::new(
        geo::LineString(vec![
            geo::Coord {
                x: -122.33583,
                y: 47.60621,
            },
            geo::Coord {
                x: -122.33583,
                y: 47.60622,
            },
        ]),
        vec![geo::LineString(vec![
            geo::Coord {
                x: -122.33583,
                y: 47.60621,
            },
            geo::Coord {
                x: -122.33583,
                y: 47.60622,
            },
        ])],
    ), "{ type: 'Polygon', coordinates: [[[-122.33583, 47.60621], [-122.33583, 47.60622], [-122.33583, 47.60621]], [[[-122.33583, 47.60621], [-122.33583, 47.60622], [-122.33583, 47.60621]]]] }"
    );

    #[derive(serde::Serialize, serde::Deserialize)]
    struct ObjectTest {
        name: i32,
    }

    create_is_function!(
        /// The type::is::object function checks if given value is of type object.
        /// Also aliased as `type_is_object!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type object. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let result = type_::is::object!(sql::Object::default());
        /// println!("{}", result.to_raw().build());
        ///
        /// let object_field = Field::new("object_field");
        /// let result = type_::is::object!(object_field);
        /// assert_eq!(result.to_raw().build(), "type::is::object(object_field)");
        ///
        /// let object_param = Param::new("object_param");
        /// let result = type_::is::object!(object_param);
        /// assert_eq!(result.to_raw().build(), "type::is::object($object_param)");
        /// ```
        =>
        "object", ObjectLike, sql::Object::default(), "{  }"
    );

    create_is_function!(
        /// The type::is::record function checks if given value is of type record.
        /// Also aliased as `type_is_record!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type record. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let result = type_::is::record!(sql::thing("user:oye").unwrap());
        /// assert_eq!(result.to_raw().build(), "type::is::record(user:oye)");
        ///
        /// let record_field = Field::new("record_field");
        /// let result = type_::is::record!(record_field);
        /// assert_eq!(result.to_raw().build(), "type::is::record(record_field)");
        ///
        /// let record_param = Param::new("record_param");
        /// let result = type_::is::record!(record_param);
        /// assert_eq!(result.to_raw().build(), "type::is::record($record_param)");
        /// ```
        =>
        "record", ThingLike, sql::thing("user:1").unwrap(), "user:1"
    );

    create_is_function!(
        /// The type::is::uuid function checks if given value is of type uuid.
        /// Also aliased as `type_is_uuid!`
        ///
        /// # Arguments
        /// * `value` - The value to be checked if it is of type uuid. Could also be a field or a parameter
        /// representing the value.
        ///
        /// # Example
        /// ```rust
        /// use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::type_};
        ///
        /// let result = type_::is::uuid!(uuid::Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap());
        /// assert_eq!(result.to_raw().build(), "type::is::uuid('936da01f-9abd-4d9d-80c7-02af85c822a8')");
        ///
        /// let uuid_field = Field::new("uuid_field");
        /// let result = type_::is::uuid!(uuid_field);
        /// assert_eq!(result.to_raw().build(), "type::is::uuid(uuid_field)");
        ///
        /// let uuid_param = Param::new("uuid_param");
        /// let result = type_::is::uuid!(uuid_param);
        /// assert_eq!(result.to_raw().build(), "type::is::uuid($uuid_param)");
        /// ```
        =>
        "uuid", ValueLike, uuid::Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(), "'936da01f-9abd-4d9d-80c7-02af85c822a8'"
    );
}
