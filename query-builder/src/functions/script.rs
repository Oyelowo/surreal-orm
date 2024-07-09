/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Embedded scripting functions
// SurrealDB allows for advanced functions with complicated logic, by allowing embedded functions to be written in JavaScript. These functions support the ES2020 JavaScript specification.
//
// Simple functions
// Embedded JavaScript functions within SurrealDB support all functionality in the ES2020 specification including async / await functions, and generator functions. Any value from SurrealDB is converted into a JavaScript type automatically, and the return value from the JavaScript function is converted to a SurrealDB value.
//
// CREATE person SET scores = function() {
// 	return [1,2,3].map(v => v * 10);
// };
//
//

use crate::{Buildable, Function, Param};

/// SurrealDB allows for advanced functions with complicated logic, by allowing embedded functions to be written in JavaScript. These functions support the ES2020 JavaScript specification.
///
/// [See more](https://surrealdb.com/docs/functions/script)
pub fn function_fn<T: Into<Param>>(args: Vec<T>, jscode_body: impl Into<String>) -> Function {
    let query_string = format!(
        "function({}) {}",
        args.into_iter()
            .map(|a| {
                let a: Param = a.into();
                a.build()
            })
            .collect::<Vec<_>>()
            .join(", "),
        jscode_body.into()
    );

    Function {
        query_string,
        bindings: vec![],
        errors: vec![],
    }
}

/// Creates a new function
///
/// # Arguments
/// * `args` - A list of arguments to pass into the function
/// * `jscode_body` - The body of the function
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::function, statements::let_};
/// // Simple functions
/// // Embedded JavaScript functions within SurrealDB support all functionality in the ES2020 specification including async / await functions, and generator functions. Any value from SurrealDB is converted into a JavaScript type automatically, and the return value from the JavaScript function is converted to a SurrealDB value.
/// let f2 = function!((), {
///    return [1,2,3].map(v => v * 10);
/// });
/// assert_eq!(f2.to_raw().build(), "function() { return [1, 2, 3].map(v => v * 10) ; }");
///
/// // Function arguments
/// // Additional arguments can be passed in to the function from SurrealDB, and these are accessible using the arguments object within the JavaScript function.
/// // Create a new parameter
/// let value = let_("value").equal_to("SurrealDB").get_param();
/// let words = let_("words").equal_to(vec!["awesome", "advanced", "cool"]).get_param();
/// // Pass the parameter values into the function
/// let f3 = function!((value, words), "{
///   return `${arguments[0]} is ${arguments[1]}`;
/// }");
/// assert_eq!(f3.to_raw().build(), "function($value, $words) {
///   return `${arguments[0]} is ${arguments[1]}`;
/// }");
///
/// let name = Param::new("name");
/// let id = Param::new("id");
/// let f5 = function!(
///     (name, id),
///     "{ return [1, 2, 3].map(v => v * 10) ; }"
/// );
/// assert_eq!(f5.to_raw().build(), "function($name, $id) { return [1, 2, 3].map(v => v * 10) ; }");
/// ```
/// [See more](https://surrealdb.com/docs/functions/script)
#[macro_export]
macro_rules! function {
    (($($arg:expr),*), {$($code:tt)*}) => {
        $crate::functions::function_fn(vec![$($arg),*] as ::std::vec::Vec<$crate::Param>, stringify!({$($code)*}))
    };
    (($($arg:expr),*), $($code:tt)*) => {
        $crate::functions::function_fn(vec![$($arg),*] as ::std::vec::Vec<$crate::Param>, concat!($($code)*))
    };
}
pub use function;

#[cfg(test)]
mod tests {
    use crate::{statements::let_, ToRaw};

    use super::*;

    #[test]
    fn test_function_without_args() {
        let f2 = function!((), {
        return [1, 2, 3].map(v => v * 10);
    });
        assert_eq!(f2.build(), "function() { return [1, 2, 3].map(v => v * 10); }");
        assert_eq!(
            f2.to_raw().build(),
            "function() { return [1, 2, 3].map(v => v * 10); }"
        );
    }

    #[test]
    fn test_function_with_params_with_variable_interpolation_in_the_function() {
        let value = let_("value").equal_to("SurrealDB").get_param();
        let words = let_("words")
            .equal_to(vec!["awesome", "advanced", "cool"])
            .get_param();

        let f2 = function!(
            (value, words),
            "{ return `${arguments[0]} is ${arguments[1]}`; }"
        );

        assert_eq!(
            f2.build(),
            "function($value, $words) { return `${arguments[0]} is ${arguments[1]}`; }"
        );

        assert_eq!(
            f2.to_raw().build(),
            "function($value, $words) { return `${arguments[0]} is ${arguments[1]}`; }"
        );
    }

    #[test]
    fn test_function_with_params_without_variable_interpolation_in_the_function() {
        let value = let_("value").equal_to("SurrealDB").get_param();
        let words = let_("words")
            .equal_to(vec!["awesome", "advanced", "cool"])
            .get_param();

        let f2 = function!((value, words), {
            return arguments[0] + " is " + arguments[1];
        });

        assert_eq!(
            f2.build(),
            "function($value, $words) { return arguments[0] + \" is \" + arguments[1]; }"
        );

        assert_eq!(
            f2.to_raw().build(),
            "function($value, $words) { return arguments[0] + \" is \" + arguments[1]; }"
        );
    }

    #[test]
    fn test_function_with_args() {
        let name = Param::new("name");
        let id = Param::new("id");

        let f2 = function!((name, id), {
        return [1, 2, 3].map(v => v * 10);
    });
        assert_eq!(
            f2.build(),
            "function($name, $id) { return [1, 2, 3].map(v => v * 10); }"
        );
        assert_eq!(
            f2.to_raw().build(),
            "function($name, $id) { return [1, 2, 3].map(v => v * 10); }"
        );
    }

    #[test]
    fn test_function_with_args_code_str() {
        let name = Param::new("name");
        let id = Param::new("id");

        let f2 = function!(
            (name, id),
            "{ return [1,2,3].map(v => v * 10 * $name * $id) ; }"
        );
        insta::assert_snapshot!(f2);
        insta::assert_snapshot!(f2.to_raw());
    }
}
