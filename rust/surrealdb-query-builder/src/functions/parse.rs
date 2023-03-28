/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Parse functions
// These functions can be used when parsing email addresses and URL web addresses.
//
// Function	Description
// parse::email::domain()	Parses and returns an email domain from an email address
// parse::email::user()	Parses and returns an email username from an email address
// parse::url::domain()	Parses and returns the domain from a URL
// parse::url::fragment()	Parses and returns the fragment from a URL
// parse::url::host()	Parses and returns the hostname from a URL
// parse::url::path()	Parses and returns the path from a URL
// parse::url::port()	Parses and returns the port number from a URL
// parse::url::query()	Parses and returns the query string from a URL

use surrealdb::sql;

use crate::{
    sql::{Binding, Buildable, ToRawStatement},
    Field,
};

use super::array::Function;

pub struct String(sql::Value);

impl From<String> for sql::Value {
    fn from(value: String) -> Self {
        value.0
    }
}

impl<T: Into<sql::Number>> From<T> for String {
    fn from(value: T) -> Self {
        let value: sql::Number = value.into();
        Self(value.into())
    }
}

impl From<Field> for String {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}

fn create_fn_with_single_string_arg(number: impl Into<String>, function_name: &str) -> Function {
    let binding = Binding::new(number.into());
    let query_string = format!("parse::{function_name}({})", binding.get_param_dollarised());

    Function {
        query_string,
        bindings: vec![binding],
    }
}

pub mod email {
    use crate::functions::array::Function;

    use super::{create_fn_with_single_string_arg, String};

    pub fn domain(number: impl Into<String>) -> Function {
        create_fn_with_single_string_arg(number, "email::domain")
    }
}

use paste::paste;

macro_rules! create_test_for_fn_with_single_arg {
    ($test_suffix: ident, $function_path: path, $arg: expr) => {
        paste! {
            #[test]
            fn [<test_ $test_suffix _fn_with_field_data >] () {
                let field = Field::new("field");
                let result = $function_path(field);

                assert_eq!(result.fine_tune_params(), format!("parse::{}($_param_00000001)", $function_path));
                assert_eq!(result.to_raw().to_string(), format!("parse::{}(temperature)", $function_path));
            }

            #[test]
            fn [<test_ $test_suffix _fn_with_fraction>]() {
                let result = $function_path($arg);
                assert_eq!(result.fine_tune_params(), format!("parse::{}($_param_00000001)", $function_path));
                assert_eq!(result.to_raw().to_string(), format!("parse::{}({})", $function_path, $arg));
            }

        }
    };
}

create_test_for_fn_with_single_arg!(email_domain, email::domain, "oyelowo@codebreather.com");
