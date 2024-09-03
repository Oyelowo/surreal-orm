/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

// Session functions
// These functions can be used when working with and manipulating text and string values.
//
// Function	Description
// session::db()	Returns the currently selected database
// session::id()	Returns the current user's session ID
// session::ip()	Returns the current user's session IP address
// session::ns()	Returns the currently selected namespace
// session::origin()	Returns the current user's HTTP origin
// session::sc()	Returns the current user's authentication scope
//

macro_rules! create_function {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>]() -> $crate::Function {
                let query_string = format!("{}()", $function_name);

                $crate::Function {
                    query_string,
                    bindings: vec![],
                    errors: vec![],
                }
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<session_ $function_name>] {
                () => {
                    $crate::functions::session::[<$function_name _fn>]()
                };
            }

            pub use [<session_ $function_name>] as [<$function_name>];

            #[cfg(test)]
            mod [<test_ $function_name>] {
                use crate::*;
                use crate::functions::session;

                #[test]
                fn [<test_ $function_name _fn>]() {
                    let result = session::[<$function_name _fn>]();
                    assert_eq!(result.fine_tune_params(), format!("{}()", $function_name));
                    assert_eq!(result.to_raw().build(), format!("{}()", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro>]() {
                    let result = session::[<$function_name>]!();
                    assert_eq!(result.fine_tune_params(), format!("{}()", $function_name));
                    assert_eq!(result.to_raw().build(), format!("{}()", $function_name));
                }
            }
        }
    };
}

create_function!(
    /// Returns the currently selected database
    /// Also aliased as `session_db!
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::session};
    ///
    /// session::db!();
    /// ```
    =>
"db");

create_function!(
    /// Returns the current user's session ID
    /// Also aliased as `session_id!
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::session};
    ///
    /// session::id!();
    /// ```
    =>
"id");

create_function!(
    /// Returns the current user's session IP address
    /// Also aliased as `session_ip!
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::session};
    ///
    /// session::ip!();
    /// ```
    =>
"ip");

create_function!(
    /// Returns the currently selected namespace
    /// Also aliased as `session_ns!
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::session};
    ///
    /// session::ns!();
    /// ```
    =>
"ns");

create_function!(
    /// Returns the current user's HTTP origin
    /// Also aliased as `session_origin!
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::session};
    ///
    /// session::origin!();
    /// ```
    =>
"origin");

create_function!(
    /// Returns the current user's authentication scope
    /// Also aliased as `session_sc!
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::session};
    ///
    /// session::sc!();
    /// ```
    =>
"sc");
