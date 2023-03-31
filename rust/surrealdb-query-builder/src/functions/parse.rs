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
    sql::{Binding, Param},
    Field,
};

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

impl From<Param> for String {
    fn from(value: Param) -> Self {
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
    ($module_name:expr, $function_name:expr, $arg:expr) => {
        ::paste::paste! {
            use crate::{
                sql::{Binding as _, Buildable as _, ToRawStatement as _},
            };

            pub fn [<$function_name _fn>](number: impl Into<String>) -> Function {
                create_fn_with_single_string_arg(number, format!("{}::{}", $module_name, $function_name).as_str())
            }

            #[macro_export]
            macro_rules! [<parse_ $module_name _ $function_name>] {
                ( $value:expr ) => {
                    crate::functions::parse::[<$module_name>]::[<$function_name _fn>]($value)
                };
            }

            pub use [<parse_ $module_name _ $function_name>] as [<$function_name>];

            #[test]
            fn [<test_ $function_name _fn_with_field_data >] () {
                let field = crate::Field::new("field");
                let result = [<$function_name _fn>](field);

                let function_path = format!("parse::{}::{}", $module_name, $function_name);
                assert_eq!(result.fine_tune_params(), format!("{}($_param_00000001)", function_path));
                assert_eq!(result.to_raw().to_string(), format!("{}(field)", function_path));
            }

            #[test]
            fn [<test_ $function_name _fn_with_fraction>]() {
                let result = [<$function_name _fn>]($arg);
                let function_path = format!("parse::{}::{}", $module_name, $function_name);
                assert_eq!(result.fine_tune_params(), format!("{}($_param_00000001)", function_path));
                assert_eq!(result.to_raw().to_string(), format!("{}('{}')", function_path, $arg));
            }

            #[test]
            fn [<test_ $function_name _macro_with_field_data >] () {
                let field = crate::Field::new("field");
                let result = [<$function_name>]!(field);

                let function_path = format!("parse::{}::{}", $module_name, $function_name);
                assert_eq!(result.fine_tune_params(), format!("{}($_param_00000001)", function_path));
                assert_eq!(result.to_raw().to_string(), format!("{}(field)", function_path));
            }

            #[test]
            fn [<test_ $function_name _macro_with_param >] () {
                let param = crate::sql::Param::new("param");
                let result = [<$function_name>]!(param);

                let function_path = format!("parse::{}::{}", $module_name, $function_name);
                assert_eq!(result.fine_tune_params(), format!("{}($_param_00000001)", function_path));
                assert_eq!(result.to_raw().to_string(), format!("{}($param)", function_path));
            }

            #[test]
            fn [<test_ $function_name _macro_with_fraction>]() {
                let result = [<$function_name>]!($arg);
                let function_path = format!("parse::{}::{}", $module_name, $function_name);
                assert_eq!(result.fine_tune_params(), format!("{}($_param_00000001)", function_path));
                assert_eq!(result.to_raw().to_string(), format!("{}('{}')", function_path, $arg));
            }

        }
    };
}

pub mod email {
    use crate::functions::array::Function;

    use super::{create_fn_with_single_string_arg, String};

    create_test_for_fn_with_single_arg!("email", "domain", "oyelowo@codebreather.com");
    create_test_for_fn_with_single_arg!("email", "user", "oyelowo@codebreather.com");
}

pub mod url {
    use crate::functions::array::Function;

    use super::{create_fn_with_single_string_arg, String};

    create_test_for_fn_with_single_arg!(
        "url",
        "domain",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg!(
        "url",
        "fragment",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg!(
        "url",
        "host",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg!(
        "url",
        "path",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg!(
        "url",
        "port",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg!(
        "url",
        "query",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
}
