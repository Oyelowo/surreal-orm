/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Validation functions
// These functions can be used when checking and validating the format of fields and values.
//
// Function	Description
// is::alphanum()	Checks whether a value has only alphanumeric characters
// is::alpha()	Checks whether a value has only alpha characters
// is::ascii()	Checks whether a value has only ascii characters
// is::domain()	Checks whether a value is a domain
// is::email()	Checks whether a value is an email
// is::hexadecimal()	Checks whether a value is hexadecimal
// is::latitude()	Checks whether a value is a latitude value
// is::longitude()	Checks whether a value is a longitude value
// is::numeric()	Checks whether a value has only numeric characters
// is::semver()	Checks whether a value matches a semver version
// is::uuid()	Checks whether a value is a UUID
//

use surrealdb::sql;

use crate::{
    sql::{Binding, Buildable, Name, ToRawStatement},
    Field,
};

use super::array::Function;

fn fun_name(value: impl Into<sql::Value>, function_name: &str) -> Function {
    let binding = Binding::new(value);

    Function {
        query_string: format!("is::{function_name}({})", binding.get_param_dollarised()),
        bindings: vec![binding],
    }
}

pub fn alphanum(value: impl Into<sql::Value>) -> Function {
    fun_name(value, "alphanum")
}

pub fn alpha(value: impl Into<sql::Value>) -> Function {
    fun_name(value, "alpha")
}

pub fn ascii(value: impl Into<sql::Value>) -> Function {
    fun_name(value, "ascii")
}

pub fn domain(value: impl Into<sql::Value>) -> Function {
    fun_name(value, "domain")
}

pub fn email(value: impl Into<sql::Value>) -> Function {
    fun_name(value, "email")
}

pub fn hexadecimal(value: impl Into<sql::Value>) -> Function {
    fun_name(value, "hexadecimal")
}

pub fn latitude(value: impl Into<sql::Value>) -> Function {
    fun_name(value, "latitude")
}

pub fn longitude(value: impl Into<sql::Value>) -> Function {
    fun_name(value, "longitude")
}

pub fn numeric(value: impl Into<sql::Value>) -> Function {
    fun_name(value, "numeric")
}

pub fn semver(value: impl Into<sql::Value>) -> Function {
    fun_name(value, "semver")
}

pub fn uuid(value: impl Into<sql::Value>) -> Function {
    fun_name(value, "uuid")
}

use paste::paste;

macro_rules! test_validator {
    ($function_ident: ident, $function_name: expr) => {
        paste! {
            #[test]
            fn [<test_ $function_name _with_field>] ()  {
                let username = Field::new("username");
                let result = $function_ident(username);

                assert_eq!(result.fine_tune_params(), format!("is::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}(username)", $function_name));
                }

            #[test]
            fn [<test_ $function_name _string_username>] ()  {
                let result = $function_ident("oyelowo1234");

                assert_eq!(result.fine_tune_params(), format!("is::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}('oyelowo1234')", $function_name));
            }

            #[test]
            fn [<test_ $function_name _with_number>] ()  {
                let result = $function_ident(123456423);

                assert_eq!(result.fine_tune_params(), format!("is::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}(123456423)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _with_fraction>] ()  {
                let result = $function_ident(12.3456423);

                assert_eq!(result.fine_tune_params(), format!("is::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}(12.3456423)", $function_name));
            }
        }
    };
}

test_validator!(alphanum, "alphanum");
test_validator!(alpha, "alpha");
test_validator!(ascii, "ascii");
test_validator!(domain, "domain");
test_validator!(email, "email");
test_validator!(hexadecimal, "hexadecimal");
test_validator!(latitude, "latitude");
test_validator!(longitude, "longitude");
test_validator!(numeric, "numeric");
test_validator!(semver, "semver");
test_validator!(uuid, "uuid");
