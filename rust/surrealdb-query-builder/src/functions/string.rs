/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// String functions
// These functions can be used when working with and manipulating text and string values.
//
// Function	Description
// string::concat()	Concatenates strings together
// string::endsWith()	Checks whether a string ends with another string
// string::join()	Joins strings together with a delimiter
// string::length()	Returns the length of a string
// string::lowercase()	Converts a string to lowercase
// string::repeat()	Repeats a string a number of times
// string::replace()	Replaces an occurence of a string with another string
// string::reverse()	Reverses a string
// string::slice()	Extracts and returns a section of a string
// string::slug()	Converts a string into human and URL-friendly string
// string::split()	Divides a string into an ordered list of substrings
// string::startsWith()	Checks whether a string starts with another string
// string::trim()	Removes whitespace from the start and end of a string
// string::uppercase()	Converts a string to uppercase
// string::words()	Splits a string into an array of separate words

use crate::array;
use crate::sql::{Binding, Buildable, ToRawStatement};
use surrealdb::sql;

use super::array::Function;

// struct String(sql::Value);

pub fn concat_fn<T: Into<sql::Value>>(values: Vec<T>) -> Function {
    let mut bindings = vec![];

    let values = values
        .into_iter()
        .map(|v| {
            let binding = Binding::new(v.into());
            let string = binding.get_param_dollarised();
            bindings.push(binding);
            string
        })
        .collect::<Vec<_>>();

    let query_string = format!("string::concat({})", values.join(", "));

    Function {
        query_string,
        bindings,
    }
}

#[macro_export]
macro_rules! concat_ {
        ( $val:expr ) => {
            crate::functions::string::concat_fn( $val )
        };
        ($( $val:expr ),*) => {
            crate::functions::string::concat_fn(crate::array![ $( $val ), * ])
        };
    }

pub use concat_;

#[test]
fn test_concat_macro() {
    let result = concat_!("one", "two", 3, 4.15385, "five", true);
    assert_eq!(result.fine_tune_params(), "string::concat($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
    assert_eq!(
        result.to_raw().to_string(),
        "string::concat('one', 'two', 3, 4.15385, 'five', true)"
    );
}

#[test]
fn test_concat_macro_with_array() {
    let result = concat_!(array!["one", "two", 3, 4.15385, "five", true]);
    assert_eq!(result.fine_tune_params(), "string::concat($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
    assert_eq!(
        result.to_raw().to_string(),
        "string::concat('one', 'two', 3, 4.15385, 'five', true)"
    );
}
