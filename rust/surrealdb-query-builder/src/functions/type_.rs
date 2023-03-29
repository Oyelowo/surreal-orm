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
    sql::{Binding, Buildable, ToRawStatement},
    Field,
};

use super::array::Function;

fn bool_fn(value: impl Into<sql::Value>) -> Function {
    let binding = Binding::new(value.into());
    let query_string = format!("type::bool({})", binding.get_param_dollarised());

    Function {
        query_string,
        bindings: vec![binding],
    }
}

#[macro_export]
macro_rules! bool {
    ( $string:expr ) => {
        crate::functions::type_::bool_fn($string)
    };
}
pub use bool;

#[test]
fn test_bool_with_macro_with_field() {
    let name = Field::new("name");
    let result = bool!(name);
    assert_eq!(result.fine_tune_params(), "type::bool($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "type::bool(name)");
}

#[test]
fn test_bool_with_macro_with_plain_string() {
    let result = bool!("toronto");
    assert_eq!(result.fine_tune_params(), "type::bool($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "type::bool('toronto')");
}

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
