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

use surrealdb::sql;

use crate::{
    traits::{Binding, Buildable, ToRaw},
    types::{
        DatetimeLike, DurationLike, Field, Function, NumberLike, Param, StrandLike, Table,
        TableLike,
    },
    Parametric,
};

macro_rules! create_type {
    ($function_name:expr, $value_type: ty, $test_data_input:expr, $test_stringified_data_output: expr) => {
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
            macro_rules! [<type_ $function_name>] {
                ( $string:expr ) => {
                    crate::functions::type_::[<$function_name _fn>]($string)
                };
            }
            pub use [<type_ $function_name>] as [<$function_name>];

            #[test]
            fn [<test_ $function_name _with_macro_with_field>]() {
                let name = Field::new("name");
                let result = [<$function_name>]!(name);
                assert_eq!(result.fine_tune_params(), format!("type::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("type::{}(name)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _with_macro_with_plain_string>]() {
                let result = [<$function_name>]!($test_data_input);
                assert_eq!(result.fine_tune_params(), format!("type::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("type::{}({})", $function_name, $test_stringified_data_output));
            }
        }
    };
}

create_type!("bool", sql::Value, "toronto", "'toronto'");
create_type!(
    "datetime",
    DatetimeLike,
    chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
        chrono::Utc,
    ),
    "'1970-01-01T00:01:01Z'"
);
create_type!(
    "duration",
    DurationLike,
    std::time::Duration::from_secs(24 * 60 * 60 * 7),
    "1w"
);
create_type!("float", NumberLike, 43.5, 43.5);
create_type!("int", NumberLike, 99, 99);
create_type!("number", NumberLike, 5, 5);
create_type!("string", sql::Value, 5454, "5454");
create_type!("regex", StrandLike, "/[A-Z]{3}/", "'/[A-Z]{3}/'");
create_type!("table", TableLike, Table::new("user"), "user");

fn point_fn(point1: impl Into<NumberLike>, point2: impl Into<NumberLike>) -> Function {
    let point1_binding = Binding::new(point1.into());
    let point2_binding = Binding::new(point2.into());
    let query_string = format!(
        "type::point({}, {})",
        point1_binding.get_param_dollarised(),
        point2_binding.get_param_dollarised()
    );

    Function {
        query_string,
        bindings: vec![point1_binding, point2_binding],
    }
}

#[macro_export]
macro_rules! type_point {
    ( $point1:expr, $point2:expr ) => {
        crate::functions::type_::point_fn($point1, $point2)
    };
}

use crate::Valuex;
pub use type_point as point;

fn thing_fn(point1: impl Into<TableLike>, point2: impl Into<Valuex>) -> Function {
    let point1_binding = Binding::new(point1.into());
    let point2: Valuex = point2.into();
    /* let point2_binding = Binding::new(point2.into()); */
    let query_string = format!(
        "type::thing({}, {})",
        point1_binding.get_param_dollarised(),
        point2.build()
    );

    let mut bindings = vec![point1_binding];
    bindings.extend(point2.get_bindings());
    // let point2_binding = point2.get_bindings();
    Function {
        query_string,
        bindings,
    }
}

#[macro_export]
macro_rules! type_thing {
    ( $table:expr, $value:expr ) => {
        crate::functions::type_::thing_fn($table, $value)
    };
}

pub use type_thing as thing;

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
        chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
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

#[test]
fn test_datetime_macro_with_param() {
    let rebirth_date = Param::new("rebirth_date");
    let result = datetime!(rebirth_date);

    assert_eq!(
        result.fine_tune_params(),
        "type::datetime($_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "type::datetime($rebirth_date)");
}

#[test]
fn test_point_macro_with_plain_values() {
    let result = point!(51.509865, -0.118092);
    assert_eq!(
        result.fine_tune_params(),
        "type::point($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "type::point(51.509865, -0.118092)"
    );
}

#[test]
fn test_point_macro_with_fields() {
    let home = Field::new("home");
    let away = Field::new("away");
    assert_eq!(
        result.fine_tune_params(),
        "type::point($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "type::point(home, away)");
}

#[test]
fn test_thing_macro_with_plain_values() {
    let user = Table::from("user");
    let id = "oyelowo";
    let result = thing!(user, id);
    assert_eq!(
        result.fine_tune_params(),
        "type::thing($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "type::thing(user, 'oyelowo')");
}

#[test]
fn test_thing_macro_with_datetime_field() {
    let table = Table::new("table");
    let id = Field::new("id");
    let result = thing!(table, id);

    assert_eq!(
        result.fine_tune_params(),
        "type::thing($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "type::thing(table, id)");
}


