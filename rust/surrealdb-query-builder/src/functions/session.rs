/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
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

use surrealdb::sql;

use crate::traits::{Binding, Buildable, Empty, ToRaw};

use crate::types::{Function, NumberLike};

use crate::array;

macro_rules! create_function {
    ($function_name: expr) => {
        paste::paste! {

            fn [<$function_name _fn>]() -> Function {
                let query_string = format!("{}()", $function_name);

                Function {
                    query_string,
                    bindings: vec![],
                }
            }

            #[macro_export]
            macro_rules! [<session_ $function_name>] {
                () => {
                    crate::functions::session::[<$function_name _fn>]()
                };
            }

            pub use [<session_ $function_name>] as [<$function_name>];

            #[test]
            fn [<test_ $function_name _fn>]() {
                let result = [<$function_name _fn>]();
                assert_eq!(result.fine_tune_params(), format!("{}()", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("{}()", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro>]() {
                let result = [<$function_name>]!();
                assert_eq!(result.fine_tune_params(), format!("{}()", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("{}()", $function_name));
            }
        }
    };
}

create_function!("db");
create_function!("id");
create_function!("ip");
create_function!("ns");
create_function!("origin");
create_function!("sc");
