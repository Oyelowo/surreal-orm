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

use crate::{sql::Binding, Field};

use super::array::Function;

pub struct String(sql::Value);

impl From<String> for sql::Value {
    fn from(value: String) -> Self {
        value.0
    }
}

impl<T: Into<sql::Strand>> From<T> for String {
    fn from(value: T) -> Self {
        let value: sql::Strand = value.into();
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

#[macro_use]
macro_rules! create_test_for_fn_with_single_arg {
    ($function_ident: ident, $function_name_str: expr, $arg: expr) => {
        ::paste::paste! {
            use crate::{
                sql::{Binding as _, Buildable as _, ToRawStatement as _},
            };


            #[test]
            fn [<test_ $function_ident _fn_with_field_data >] () {
                let field = crate::Field::new("field");
                let result = $function_ident(field);

                assert_eq!(result.fine_tune_params(), format!("parse::{}($_param_00000001)", $function_name_str));
                assert_eq!(result.to_raw().to_string(), format!("parse::{}(field)", $function_name_str));
            }

            #[test]
            fn [<test_ $function_ident _fn_with_fraction>]() {
                let result = $function_ident($arg);
                assert_eq!(result.fine_tune_params(), format!("parse::{}($_param_00000001)", $function_name_str));
                assert_eq!(result.to_raw().to_string(), format!("parse::{}('{}')", $function_name_str, $arg));
            }

        }
    };
}

pub mod email {
    use crate::functions::array::Function;

    use super::{create_fn_with_single_string_arg, String};

    pub fn domain(number: impl Into<String>) -> Function {
        create_fn_with_single_string_arg(number, "email::domain")
    }

    pub fn user(number: impl Into<String>) -> Function {
        create_fn_with_single_string_arg(number, "email::user")
    }

    create_test_for_fn_with_single_arg!(domain, "email::domain", "oyelowo@codebreather.com");
    create_test_for_fn_with_single_arg!(user, "email::user", "oyelowo@codebreather.com");
}

pub mod url {
    use crate::functions::array::Function;

    use super::{create_fn_with_single_string_arg, String};

    pub fn domain(value: impl Into<String>) -> Function {
        create_fn_with_single_string_arg(value, "url::domain")
    }

    pub fn fragment(value: impl Into<String>) -> Function {
        create_fn_with_single_string_arg(value, "url::fragment")
    }

    pub fn host(value: impl Into<String>) -> Function {
        create_fn_with_single_string_arg(value, "url::host")
    }

    pub fn path(value: impl Into<String>) -> Function {
        create_fn_with_single_string_arg(value, "url::path")
    }

    pub fn port(value: impl Into<String>) -> Function {
        create_fn_with_single_string_arg(value, "url::port")
    }

    pub fn query(value: impl Into<String>) -> Function {
        create_fn_with_single_string_arg(value, "url::query")
    }

    create_test_for_fn_with_single_arg!(
        domain,
        "url::domain",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg!(
        fragment,
        "url::fragment",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg!(
        host,
        "url::host",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg!(
        path,
        "url::path",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg!(
        port,
        "url::port",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg!(
        query,
        "url::query",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
}
