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

// format!("function({}) {}", stringify!($($arg),*), stringify!($code))
macro_rules! function {
    ((), $code:tt) => {
        format!(
            "function() {}",
            stringify!($code)
        )
    };
    (($($arg:expr),*), $code:tt) => {
        format!(
            "function({}) {}",
            vec![$($arg),*].join(", "),
            stringify!($code)
        )
    };
}

#[test]
fn test_function_without_args() {
    let f2 = function!((), {
        return [1,2,3].map(v => v * 10);
    });
    assert_eq!(f2, "function() { return [1, 2, 3].map(v => v * 10) ; }");
}

#[test]
fn test_function_with_args() {
    let f2 = function!(("$first", "$two"), {
        return [1,2,3].map(v => v * 10);
    });
    assert_eq!(
        f2,
        "function($first, $two) { return [1, 2, 3].map(v => v * 10) ; }"
    );
}
