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

use crate::{types::Function, Buildable, Parametric, Valuex};

fn create_validation_function(value: impl Into<Valuex>, function_name: &str) -> Function {
    let value: Valuex = value.into();

    Function {
        query_string: format!("is::{function_name}({})", value.build()),
        bindings: value.get_bindings(),
    }
}

macro_rules! create_validation_with_tests {
    ($function_name: expr) => {
        paste::paste! {
            pub fn [<$function_name _fn>](value: impl Into<$crate::Valuex>) -> $crate::Function {
                super::create_validation_function(value, $function_name)
            }

            #[macro_export]
            macro_rules!  [<validation_is_ $function_name>]{
                ( $geometry:expr ) => {
                    crate::functions::validation::is::[<$function_name _fn>]($geometry)
                };
            }
            pub use [<validation_is_ $function_name>] as [<$function_name>];

            #[test]
            fn [<test_ $function_name _with_field>] ()  {
                let username = Field::new("username");
                let result = [<$function_name _fn>](username);

                assert_eq!(result.fine_tune_params(), format!("is::{}(username)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}(username)", $function_name));
                }

            #[test]
            fn [<test_ $function_name _string_username>] ()  {
                let result = [<$function_name _fn>]("oyelowo1234");

                assert_eq!(result.fine_tune_params(), format!("is::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}('oyelowo1234')", $function_name));
            }

            #[test]
            fn [<test_ $function_name _with_number>] ()  {
                let result = [<$function_name _fn>](123456423);

                assert_eq!(result.fine_tune_params(), format!("is::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}(123456423)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _with_fraction>] ()  {
                let result = [<$function_name _fn>](12.3456423);

                assert_eq!(result.fine_tune_params(), format!("is::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}(12.3456423)", $function_name));
            }

            // Macro versions
            #[test]
            fn [<test_ $function_name _macro_with_field>] ()  {
                let username = Field::new("username");
                let result = [<$function_name>]!(username);

                assert_eq!(result.fine_tune_params(), format!("is::{}(username)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}(username)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_string_username>] ()  {
                let result = [<$function_name>]!("oyelowo1234");

                assert_eq!(result.fine_tune_params(), format!("is::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}('oyelowo1234')", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_with_number>] ()  {
                let result = [<$function_name>]!(123456423);

                assert_eq!(result.fine_tune_params(), format!("is::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}(123456423)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_with_fraction>] ()  {
                let result = [<$function_name>]!(12.3456423);

                assert_eq!(result.fine_tune_params(), format!("is::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("is::{}(12.3456423)", $function_name));
            }

        }
    };
}

pub mod is {
    use surrealdb::sql;

    use crate::traits::{Binding, Buildable, ToRaw};
    use crate::types::{Field, Function};

    create_validation_with_tests!("alphanum");
    create_validation_with_tests!("alpha");
    create_validation_with_tests!("ascii");
    create_validation_with_tests!("domain");
    create_validation_with_tests!("email");
    create_validation_with_tests!("hexadecimal");
    create_validation_with_tests!("latitude");
    create_validation_with_tests!("longitude");
    create_validation_with_tests!("numeric");
    create_validation_with_tests!("semver");
    create_validation_with_tests!("uuid");
}
