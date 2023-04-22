/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
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

use crate::{Buildable, ToRaw};
use crate::{Function, Param};

// SurrealDB allows for advanced functions with complicated logic, by allowing embedded functions to be written in JavaScript. These functions support the ES2020 JavaScript specification.
//
// Simple functions
// Embedded JavaScript functions within SurrealDB support all functionality in the ES2020 specification including async / await functions, and generator functions. Any value from SurrealDB is converted into a JavaScript type automatically, and the return value from the JavaScript function is converted to a SurrealDB value.
//
// CREATE person SET scores = function() {
// 	return [1,2,3].map(v => v * 10);
// };
// Function context
// The this context of each embedded function is automatically set to the current document on every invocation. This allows the function to access the properties and fields of the current record being accessed / modified.
//
// CREATE film SET
// 	ratings = [
// 		{ rating: 6, user: user:bt8e39uh1ouhfm8ko8s0 },
// 		{ rating: 8, user: user:bsilfhu88j04rgs0ga70 },
// 	],
// 	featured = function() {
// 		return this.ratings.filter(r => {
// 			return r.rating >= 7;
// 		}).map(r => {
// 			return { ...r, rating: r.rating * 10 };
// 		});
// 	}
// ;
// Function arguments
// Additional arguments can be passed in to the function from SurrealDB, and these are accessible using the arguments object within the JavaScript function.
//
// -- Create a new parameter
// LET $value = "SurrealDB";
// -- Create a new parameter
// LET $words = ["awesome", "advanced", "cool"];
// -- Pass the parameter values into the function
// CREATE article SET summary = function($value, $words) {
// 	return `${arguments[0]} is ${arguments[1].join(', ')}`;
// };
// JavaScript types
// Any value from SurrealDB is converted into a JavaScript type automatically, and the return value from the JavaScript function is converted to a SurrealDB value. Boolean values, Integers, Floats, Strings, Arrays, Objects, and Date objects are all converted automatically to and from SurrealDB values.
//
// CREATE user:test SET created_at = function() {
// 	return new Date();
// };
// In addition a number of special classes are included within the JavaScript functions, for the additional types which are not built into JavaScript. These enable the creation of duration values, record values, and UUID values from within JavaScript.
//
// CREATE user:test SET
// 	session_timeout = function() {
// 		return new Duration('1w');
// 	},
// 	best_friend = function() {
// 		return new Record('user', 'joanna');
// 	},
// 	identifier = function() {
// 		return new Uuid('03412258-988f-47cd-82db-549902cdaffe');
// 	}
// ;
//
// [See more](https://surrealdb.com/docs/functions/script)
pub fn function_fn<T: Into<Param>>(args: Vec<T>, jscode_body: impl Into<String>) -> Function {
    let query_string = format!(
        "function({}) {}",
        args.into_iter()
            .map(|a| {
                let a: Param = a.into();
                let a = a.build();
                a
            })
            .collect::<Vec<_>>()
            .join(", "),
        jscode_body.into()
    );

    Function {
        query_string,
        bindings: vec![],
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
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::function};
/// // Simple functions
/// // Embedded JavaScript functions within SurrealDB support all functionality in the ES2020 specification including async / await functions, and generator functions. Any value from SurrealDB is converted into a JavaScript type automatically, and the return value from the JavaScript function is converted to a SurrealDB value.
/// let f2 = function!((), {
///    return [1,2,3].map(v => v * 10);
/// });
/// assert_eq!(f2.to_raw().build(), "function() { return [1, 2, 3].map(v => v * 10) ; }");
///
/// let name = Param::new("name");
/// let id = Param::new("id");
/// let f3 = function!((name, id), "{
///   return `${arguments[0]} is ${arguments[1]}`;
/// }");
/// assert_eq!(f3.to_raw().build(), "function($name, $id) {
///   return `${arguments[0]} is ${arguments[1]}`;
/// }");
///
/// let name = Param::new("name");
/// let id = Param::new("id");
/// let f4 = function!((name, id), {
///     return [1, 2, 3].map(v => v * 10 * $name * $id);
/// });
/// assert_eq!(f4.to_raw().build(), "function($name, $id) { return [1, 2, 3].map(v => v * 10 * $name * $id) ; }");
///
/// let name = Param::new("name");
/// let id = Param::new("id");
/// let f5 = function!(
///     (name, id),
///     "{ return [1, 2, 3].map(v => v * 10 * $name * $id) ; }"
/// );
/// assert_eq!(f5.to_raw().build(), "function($name, $id) { return [1, 2, 3].map(v => v * 10 * $name * $id) ; }");
/// ```
#[macro_export]
macro_rules! function {
    (($($arg:expr),*), {$($code:tt)*}) => {
        $crate::functions::function_fn(vec![$($arg),*] as Vec<$crate::Param>, stringify!({$($code)*}))
    };
    (($($arg:expr),*), $($code:tt)*) => {
        $crate::functions::function_fn(vec![$($arg),*] as Vec<$crate::Param>, concat!($($code)*))
    };
}
pub use function;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_without_args() {
        let f2 = function!((), {
        return [1,2,3].map(v => v * 10);
    });
        assert_eq!(f2.build(), "function() { return [1, 2, 3].map(v => v * 10) ; }");
        assert_eq!(
            f2.to_raw().build(),
            "function() { return [1, 2, 3].map(v => v * 10) ; }"
        );
    }

    #[test]
    fn test_function_with_args() {
        let name = Param::new("name");
        let id = Param::new("id");

        let f2 = function!((name, id), {
        return [1,2,3].map(v => v * 10 * $name * $id);
    });
        assert_eq!(
            f2.build(),
            "function($name, $id) { return [1, 2, 3].map(v => v * 10 * $name * $id) ; }"
        );
        assert_eq!(
            f2.to_raw().build(),
            "function($name, $id) { return [1, 2, 3].map(v => v * 10 * $name * $id) ; }"
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
        insta::assert_display_snapshot!(f2);
        insta::assert_display_snapshot!(f2.to_raw());
    }
}
