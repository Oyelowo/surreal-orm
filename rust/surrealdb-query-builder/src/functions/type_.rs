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

use surrealdb::sql::{self};

use crate::{
    sql::{Binding, Buildable, ToRawStatement},
    Field,
};

use super::{
    array::Function,
    math::Number,
    parse::String,
    time::{Datetime, Duration},
};

pub struct Table(sql::Value);

impl Table {
    fn new(table_name: impl Into<std::string::String>) -> Table {
        Table(sql::Table::from(table_name.into()).into())
    }
}

impl From<Table> for sql::Value {
    fn from(value: Table) -> Self {
        value.0
    }
}

impl<T: Into<sql::Table>> From<T> for Table {
    fn from(value: T) -> Self {
        let value: sql::Table = value.into();
        Self(value.into())
    }
}

impl From<Field> for Table {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}

macro_rules! create_type {
    ($function_ident:ident, $function_name:expr, $value_type: ty, $test_data_input:expr, $test_stringified_data_output: expr) => {
        paste::paste! {
            fn [<$function_name _fn>](value: impl Into<$value_type>) -> Function {
                let binding = Binding::new(value.into());
                let query_string = format!("type::{}({})", $function_name, binding.get_param_dollarised());

                Function {
                    query_string,
                    bindings: vec![binding],
                }
            }

            #[macro_export]
            macro_rules! [<$function_ident>] {
                ( $string:expr ) => {
                    crate::functions::type_::[<$function_name _fn>]($string)
                };
            }
            pub use $function_ident;

            #[test]
            fn [<test_ $function_name _with_macro_with_field>]() {
                let name = Field::new("name");
                let result = $function_ident!(name);
                assert_eq!(result.fine_tune_params(), format!("type::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("type::{}(name)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _with_macro_with_plain_string>]() {
                let result = $function_ident!($test_data_input);
                assert_eq!(result.fine_tune_params(), format!("type::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("type::{}({})", $function_name, $test_stringified_data_output));
            }
        }
    };
}

create_type!(bool, "bool", sql::Value, "toronto", "'toronto'");
create_type!(
    datetime,
    "datetime",
    Datetime,
    chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(61, 0),
        chrono::Utc,
    ),
    "'1970-01-01T00:01:01Z'"
);
create_type!(
    duration,
    "duration",
    Duration,
    std::time::Duration::from_secs(24 * 60 * 60 * 7),
    "1w"
);
create_type!(float_, "float", Number, 43.5, 43.5);
create_type!(int_, "int", Number, 99, 99);
create_type!(number, "number", Number, 5, 5);
create_type!(string_, "string", sql::Value, 5454, "5454");
create_type!(regex, "regex", String, "/[A-Z]{3}/", "'/[A-Z]{3}/'");
create_type!(table, "table", Table, Table::new("user"), "user");

#[test]
fn test_bool_with_macro_with_plain_number() {
    let result = bool!(43545);
    assert_eq!(result.fine_tune_params(), "type::bool($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "type::bool(43545)");
}

#[test]
fn test_bool_with_macro_with_plain_false() {
    let result = bool!(false);
    assert_eq!(result.fine_tune_params(), "type::bool($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "type::bool(false)");
}

#[test]
fn test_bool_with_macro_with_plain_true() {
    let result = bool!(true);
    assert_eq!(result.fine_tune_params(), "type::bool($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "type::bool(true)");
}

#[test]
fn test_datetime_macro_with_plain_datetime() {
    let value = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(61, 0),
        chrono::Utc,
    );
    let result = datetime!(value);
    assert_eq!(
        result.fine_tune_params(),
        "type::datetime($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "type::datetime('1970-01-01T00:01:01Z')"
    );
}

#[test]
fn test_datetime_macro_with_datetime_field() {
    let rebirth_date = Field::new("rebirth_date");
    let result = datetime!(rebirth_date);

    assert_eq!(
        result.fine_tune_params(),
        "type::datetime($_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "type::datetime(rebirth_date)");
}
