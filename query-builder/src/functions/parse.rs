/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

// Parse functions
// These functions can be used when parsing email addresses and URL web addresses.
//
// Function	Description
// parse::email::host() Parses and returns an email host from an email address
// parse::email::user()	Parses and returns an email username from an email address
// parse::url::domain()	Parses and returns the domain from a URL
// parse::url::fragment()	Parses and returns the fragment from a URL
// parse::url::host()	Parses and returns the hostname from a URL
// parse::url::path()	Parses and returns the path from a URL
// parse::url::port()	Parses and returns the port number from a URL
// parse::url::query()	Parses and returns the query string from a URL

use crate::{Buildable, Erroneous, Function, Parametric, StrandLike};

fn create_fn_with_single_string_arg(
    string: impl Into<StrandLike>,
    function_name: &str,
) -> Function {
    let string: StrandLike = string.into();
    let query_string = format!("parse::{function_name}({})", string.build());

    Function {
        query_string,
        bindings: string.get_bindings(),
        errors: string.get_errors(),
    }
}

macro_rules! create_test_for_fn_with_single_arg {
    ($(#[$attr:meta])* => $module_name:expr, $function_name:expr, $arg:expr) => {
        ::paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](number: impl Into<$crate::StrandLike>) -> $crate::Function {
                create_fn_with_single_string_arg(number, format!("{}::{}", $module_name, $function_name).as_str())
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<parse_ $module_name _ $function_name>] {
                ( $value:expr ) => {
                    $crate::functions::parse::[<$module_name>]::[<$function_name _fn>]($value)
                };
            }

            pub use [<parse_ $module_name _ $function_name>] as [<$function_name>];

            #[cfg(test)]
            mod [<test_ $function_name>] {
                use crate::*;
                use crate::functions::parse;

                #[test]
                fn [<test_ $function_name _fn_with_field_data >] () {
                    let field = Field::new("field");
                    let result = parse::[<$module_name>]::[<$function_name _fn>](field);

                    let function_path = format!("parse::{}::{}", $module_name, $function_name);
                    assert_eq!(result.fine_tune_params(), format!("{}(field)", function_path));
                    assert_eq!(result.to_raw().build(), format!("{}(field)", function_path));
                }

                #[test]
                fn [<test_ $function_name _fn_with_fraction>]() {
                    let result = parse::[<$module_name>]::[<$function_name _fn>]($arg);
                    let function_path = format!("parse::{}::{}", $module_name, $function_name);
                    assert_eq!(result.fine_tune_params(), format!("{}($_param_00000001)", function_path));
                    assert_eq!(result.to_raw().build(), format!("{}('{}')", function_path, $arg));
                }

                #[test]
                fn [<test_ $function_name _macro_with_field_data >] () {
                    let field = Field::new("field");
                    let result = parse::[<$module_name>]::[<$function_name>]!(field);

                    let function_path = format!("parse::{}::{}", $module_name, $function_name);
                    assert_eq!(result.fine_tune_params(), format!("{}(field)", function_path));
                    assert_eq!(result.to_raw().build(), format!("{}(field)", function_path));
                }

                #[test]
                fn [<test_ $function_name _macro_with_param >] () {
                    let param = Param::new("param");
                    let result = parse::[<$module_name>]::[<$function_name>]!(param);

                    let function_path = format!("parse::{}::{}", $module_name, $function_name);
                    assert_eq!(result.fine_tune_params(), format!("{}($param)", function_path));
                    assert_eq!(result.to_raw().build(), format!("{}($param)", function_path));
                }

                #[test]
                fn [<test_ $function_name _macro_with_fraction>]() {
                    let result = parse::[<$module_name>]::[<$function_name>]!($arg);
                    let function_path = format!("parse::{}::{}", $module_name, $function_name);
                    assert_eq!(result.fine_tune_params(), format!("{}($_param_00000001)", function_path));
                    assert_eq!(result.to_raw().build(), format!("{}('{}')", function_path, $arg));
                }
            }

        }
    };
}

/// This module contains functions for parsing email addresses
pub mod email {
    use super::create_fn_with_single_string_arg;

    create_test_for_fn_with_single_arg!(
        /// The parse::email::host function parses and returns and email host from a valid email address.
        /// This function is also aliased as `parse_email_host!`.
        ///
        /// parse::email::host(string) -> value
        /// The following example shows this function, and its output, when used in a select statement:
        /// ```sql
        /// SELECT * FROM parse::email::host("oyelowo@codebreather.com");
        /// codebreather.com
        /// ```
        /// 
        /// # Arguments
        ///
        /// * `string` - A valid email address. Can be a field or a parameter representing the
        /// email address.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::parse};
        ///
        /// parse::email::host!("oyelowo@codebreather.com");
        ///
        /// # let email_field = Field::new("email_field");
        /// parse::email::host!(email_field);
        ///
        /// # let email_param = Param::new("email_param");
        /// parse::email::host!(email_param);
        /// ```
        => "email", "host", "oyelowo@codebreather.com");

    create_test_for_fn_with_single_arg!(
        /// The parse::email::user function parses and returns and email username from a valid email address.
        /// This function is also aliased as `parse_email_user!`.
        ///
        /// parse::email::user(string) -> value
        /// The following example shows this function, and its output, when used in a select statement:
        /// ```sql
        /// SELECT * FROM parse::email::user("oyelowo@codebreather.com");
        /// oyelowo
        ///
        /// # Arguments
        ///
        /// * `string` - A valid email address. Can be a field or a parameter representing the
        /// email address.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::parse};
        ///
        /// parse::email::user!("oyelowo@codebreather.com");
        ///
        /// # let email_field = Field::new("email_field");
        /// parse::email::user!(email_field);
        ///
        /// # let email_param = Param::new("email_param");
        /// parse::email::user!(email_param);
        /// ```
        =>
        "email", "user", "oyelowo@codebreather.com");
}

/// This module contains functions for parsing URLs
pub mod url {
    use super::create_fn_with_single_string_arg;

    create_test_for_fn_with_single_arg!(
        /// The parse::url::domain function parses and returns the domain from a valid URL.
        /// This function is also aliased as `parse_url_domain!`.
        ///
        /// parse::url::domain(string) -> value
        /// The following example shows this function, and its output, when used in a select statement:
        /// ```sql
        /// SELECT * FROM parse::url::domain("https://codebreather.com:443/topics?arg=value#fragment");
        /// codebreather.com
        /// ```
        ///
        /// # Arguments
        /// * `string` - A valid URL. Can be a field or a parameter representing the URL.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::parse};
        /// parse::url::domain!("https://codebreather.com:443/topics?arg=value#fragment");
        /// # let url_field = Field::new("url_field");
        /// parse::url::domain!(url_field);
        /// # let url_param = Param::new("url_param");
        /// parse::url::domain!(url_param);
        /// ```
        =>
        "url",
        "domain",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );

    create_test_for_fn_with_single_arg!(
        /// The parse::url::fragment function parses and returns the fragment from a valid URL.
        /// This function is also aliased as `parse_url_fragment!`.
        ///
        /// parse::url::fragment(string) -> value
        /// The following example shows this function, and its output, when used in a select statement:
        /// ```sql
        /// SELECT * FROM parse::url::fragment("https://codebreather.com:443/topics?arg=value#fragment");
        /// fragment
        /// ```
        ///
        /// # Arguments
        /// * `string` - A valid URL. Can be a field or a parameter representing the URL.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::parse};
        /// parse::url::fragment!("https://codebreather.com:443/topics?arg=value#fragment");
        /// # let url_field = Field::new("url_field");
        /// parse::url::fragment!(url_field);
        /// # let url_param = Param::new("url_param");
        /// parse::url::fragment!(url_param);
        /// ```
        =>
        "url",
        "fragment",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );

    create_test_for_fn_with_single_arg!(
        /// The parse::url::host function parses and returns the hostname from a valid URL.
        /// This function is also aliased as `parse_url_host!`.
        ///
        /// parse::url::host(string) -> value
        /// The following example shows this function, and its output, when used in a select statement:
        /// ```sql
        /// SELECT * FROM parse::url::host("https://codebreather.com:443/topics?arg=value#fragment");
        /// codebreather.com
        /// ```
        /// # Arguments
        ///
        /// * `string` - A valid URL. Can be a field or a parameter representing the URL.
        /// # Example
        ///
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::parse};
        /// parse::url::host!("https://codebreather.com:443/topics?arg=value#fragment");
        /// # let url_field = Field::new("url_field");
        /// parse::url::host!(url_field);
        /// # let url_param = Param::new("url_param");
        /// parse::url::host!(url_param);
        /// ```
        =>
        "url",
        "host",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg!(
        /// The parse::url::path function parses and returns the path from a valid URL.
        /// This function is also aliased as `parse_url_path!`.
        /// parse::url::path(string) -> value
        /// The following example shows this function, and its output, when used in a select statement:
        /// ```sql
        /// SELECT * FROM parse::url::path("https://codebreather.com:443/topics?arg=value#fragment");
        /// /topics
        /// ```
        /// # Arguments
        ///
        /// * `string` - A valid URL. Can be a field or a parameter representing the URL.
        ///
        /// # Example
        ///
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::parse};
        /// parse::url::path!("https://codebreather.com:443/topics?arg=value#fragment");
        /// # let url_field = Field::new("url_field");
        /// parse::url::path!(url_field);
        /// # let url_param = Param::new("url_param");
        /// parse::url::path!(url_param);
        /// ```
        =>
        "url",
        "path",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );

    create_test_for_fn_with_single_arg!(
        /// The parse::url::port function parses and returns the port from a valid URL.
        /// This function is also aliased as `parse_url_port!`.
        /// parse::url::port(string) -> value
        /// The following example shows this function, and its output, when used in a select statement:
        /// ```sql
        /// SELECT * FROM parse::url::port("https://codebreather.com:443/topics?arg=value#fragment");
        /// 443
        /// ```
        /// # Arguments
        ///
        /// * `string` - A valid URL. Can be a field or a parameter representing the URL.
        /// # Example
        ///
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::parse};
        /// parse::url::port!("https://codebreather.com:443/topics?arg=value#fragment");
        /// # let url_field = Field::new("url_field");
        /// parse::url::port!(url_field);
        /// # let url_param = Param::new("url_param");
        /// parse::url::port!(url_param);
        /// ```
        =>
        "url",
        "port",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );

    create_test_for_fn_with_single_arg!(
        /// The parse::url::protocol function parses and returns the query from a valid URL.
        /// This function is also aliased as `parse_url_query!`.
        /// parse::url::protocol(string) -> value
        /// The following example shows this function, and its output, when used in a select statement:
        /// ```sql
        /// SELECT * FROM parse::url::query("https://codebreather.com:443/topics?arg=value#fragment");
        /// arg=value
        /// ```
        /// # Arguments
        ///
        /// * `string` - A valid URL. Can be a field or a parameter representing the URL.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::parse};
        /// parse::url::query!("https://codebreather.com:443/topics?arg=value#fragment");
        /// # let url_field = Field::new("url_field");
        /// parse::url::query!(url_field);
        /// # let url_param = Param::new("url_param");
        /// parse::url::query!(url_param);
        /// ```
        =>
        "url",
        "query",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
}
