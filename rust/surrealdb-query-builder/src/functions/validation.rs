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
// is::datetime() Checks whether a value matches a datetime format
// is::domain()	Checks whether a value is a domain
// is::email()	Checks whether a value is an email
// is::hexadecimal()	Checks whether a value is hexadecimal
// is::latitude()	Checks whether a value is a latitude value
// is::longitude()	Checks whether a value is a longitude value
// is::numeric()	Checks whether a value has only numeric characters
// is::semver()	Checks whether a value matches a semver version
// is::url() Checks whether a value is a valid URL
// is::uuid()	Checks whether a value is a UUID
//

use crate::{Buildable, Function, Parametric, Valuex};

fn create_validation_function(value: impl Into<Valuex>, function_name: &str) -> Function {
    let value: Valuex = value.into();

    Function {
        query_string: format!("is::{function_name}({})", value.build()),
        bindings: value.get_bindings(),
    }
}

macro_rules! create_validation_with_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](value: impl Into<$crate::Valuex>) -> $crate::Function {
                super::create_validation_function(value, $function_name)
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules!  [<validation_is_ $function_name>]{
                ( $geometry:expr ) => {
                    $crate::functions::validation::is::[<$function_name _fn>]($geometry)
                };
            }
            pub use [<validation_is_ $function_name>] as [<$function_name>];

            #[cfg(test)]
            mod [<test_ $function_name>] {
                use super::*;
                use crate::*;

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

        }
    };
}

/// This module contains functions that validate values
pub mod is {
    // The is::alphanum function checks whether a value has only alphanumeric characters.
    create_validation_with_tests!(
        /// The is::alphanum function checks whether a value has only alphanumeric characters.
        /// Also aliased as `is_alphanum!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surrealdb_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::is, statements::let_};
        ///
        /// let result = is::alphanum!("oyelowo1234");
        /// assert_eq!(result.to_raw().to_string(), "is::alphanum('oyelowo1234')");
        ///
        /// let alphanum_field = Field::new("alphanum_field");
        /// let result = is::alphanum!(alphanum_field);
        /// assert_eq!(result.to_raw().to_string(), "is::alphanum(alphanum_field)");
        ///
        /// let alphanum_param = let_("alphanum_param").equal("oyelowo1234").get_param();
        /// let result = is::alphanum!(alphanum_param);
        /// assert_eq!(result.fine_tune_params(), "is::alphanum($alphanum_param)");
        /// ```
    =>
        "alphanum"
    );

    create_validation_with_tests!(
        /// The is::alpha function checks whether a value has only alpha characters.
        /// Also aliased as `is_alpha!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surrealdb_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::is, statements::let_};
        ///
        /// let result = is::alpha!("oyelowo");
        /// assert_eq!(result.to_raw().to_string(), "is::alpha('oyelowo')");
        ///
        /// let alpha_field = Field::new("alpha_field");
        /// let result = is::alpha!(alpha_field);
        /// assert_eq!(result.to_raw().to_string(), "is::alpha(alpha_field)");
        ///
        /// let alpha_param = let_("alpha_param").equal("oyelowo").get_param();
        /// let result = is::alpha!(alpha_param);
        /// assert_eq!(result.fine_tune_params(), "is::alpha($alpha_param)");
        /// ```
        =>
            "alpha"
    );

    create_validation_with_tests!(
            /// The is::ascii function checks whether a value has only ascii characters.
            /// Also aliased as `is_ascii!`
            ///
            /// # Arguments
            ///
            /// * `value` - The value to check. Could be a field or a parameter that represents the
            /// value.
            ///
            /// # Example
            /// ```rust
            /// # use surrealdb_query_builder as surreal_orm;
            /// use surreal_orm::{*, functions::is, statements::let_};
            ///
            /// let result = is::ascii!("oyelowo");
            /// assert_eq!(result.to_raw().to_string(), "is::ascii('oyelowo')");
            ///
            /// let ascii_field = Field::new("ascii_field");
            /// let result = is::ascii!(ascii_field);
            /// assert_eq!(result.to_raw().to_string(), "is::ascii(ascii_field)");
            ///
            /// let ascii_param = let_("ascii_param").equal("oyelowo").get_param();
            /// let result = is::ascii!(ascii_param);
            /// assert_eq!(result.fine_tune_params(), "is::ascii($ascii_param)");
            /// ```
        =>
            "ascii"
    );

    create_validation_with_tests!(
                /// The is::domain function checks whether a value is a domain.
                /// Also aliased as `is_domain!`
                ///
                /// # Arguments
                ///
                /// * `value` - The value to check. Could be a field or a parameter that represents the
                /// value.
                ///
                /// # Example
                /// ```rust
                /// # use surrealdb_query_builder as surreal_orm;
                /// use surreal_orm::{*, functions::is, statements::let_};
                ///
                /// let result = is::domain!("oyelowo.com");
                /// assert_eq!(result.to_raw().to_string(), "is::domain('oyelowo.com')");
                ///
                /// let domain_field = Field::new("domain_field");
                /// let result = is::domain!(domain_field);
                /// assert_eq!(result.to_raw().to_string(), "is::domain(domain_field)");
                ///
                /// let domain_param = let_("domain_param").equal("oyelowo.com").get_param();
                /// let result = is::domain!(domain_param);
                /// assert_eq!(result.fine_tune_params(), "is::domain($domain_param)");
                /// ```
            =>
                "domain"
    );

    create_validation_with_tests!(
                    /// The is::email function checks whether a value is an email.
                    /// Also aliased as `is_email!`
                    ///
                    /// # Arguments
                    ///
                    /// * `value` - The value to check. Could be a field or a parameter that represents the
                    /// value.
                    ///
                    /// # Example
                    /// ```rust
                    /// # use surrealdb_query_builder as surreal_orm;
                    /// use surreal_orm::{*, functions::is, statements::let_};
                    ///
                    /// let result = is::email!("oyelowo@codebreather.com");
                    /// assert_eq!(result.to_raw().to_string(), "is::email('oyelowo@codebreather.com')");
                    ///
                    /// let email_field = Field::new("email_field");
                    /// let result = is::email!(email_field);
                    /// assert_eq!(result.to_raw().to_string(), "is::email(email_field)");
                    ///
                    /// let email_param = let_("email_param").equal("oyelowo@codebreather").get_param();
                    ///
                    /// let result = is::email!(email_param);
                    /// assert_eq!(result.fine_tune_params(), "is::email($email_param)");
                    /// ```
                =>
                    "email"
    );

    create_validation_with_tests!(
                            /// The is::hexadecimal function checks whether a value is hexadecimal.
                            /// Also aliased as `is_hexadecimal!`
                            ///
                            /// # Arguments
                            ///
                            /// * `value` - The value to check. Could be a field or a parameter that represents the
                            /// value.
                            ///
                            /// # Example
                            /// ```rust
                            /// # use surrealdb_query_builder as surreal_orm;
                            /// use surreal_orm::{*, functions::is, statements::let_};
                            ///
                            /// let result = is::hexadecimal!("oyelowo");
                            /// assert_eq!(result.to_raw().to_string(), "is::hexadecimal('oyelowo')");
                            ///
                            /// let hexadecimal_field = Field::new("hexadecimal_field");
                            /// let result = is::hexadecimal!(hexadecimal_field);
                            /// assert_eq!(result.to_raw().to_string(), "is::hexadecimal(hexadecimal_field)");
                            ///
                            /// let hexadecimal_param = let_("hexadecimal_param").equal("oyelowo").get_param();
                            /// let result = is::hexadecimal!(hexadecimal_param);
                            /// assert_eq!(result.fine_tune_params(), "is::hexadecimal($hexadecimal_param)");
                            /// ```
                        =>
                            "hexadecimal"
    );
    create_validation_with_tests!("latitude");
    create_validation_with_tests!("longitude");
    create_validation_with_tests!("numeric");
    create_validation_with_tests!("semver");
    create_validation_with_tests!("uuid");
}
