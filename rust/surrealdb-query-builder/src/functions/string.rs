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

use crate::{
    arr,
    traits::{Binding, Buildable, ToRaw},
    Parametric, Valuex,
};
use surrealdb::sql;

use crate::types::{Field, Function, NumberLike, StrandLike};

fn create_fn_with_single_string_arg(value: impl Into<StrandLike>, function_name: &str) -> Function {
    let value: StrandLike = value.into();
    let query_string = format!("string::{function_name}({})", value.build());

    Function {
        query_string,
        bindings: value.get_bindings(),
    }
}

pub fn length_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "length")
}

#[macro_export]
macro_rules! string_length {
    ( $string:expr ) => {
        $crate::functions::string::length_fn($string)
    };
}
pub use string_length as length;

pub fn lowercase_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "lowercase")
}

#[macro_export]
macro_rules! string_lowercase {
    ( $string:expr ) => {
        $crate::functions::string::lowercase_fn($string)
    };
}

pub use string_lowercase as lowercase;

pub fn uppercase_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "uppercase")
}

#[macro_export]
macro_rules! string_uppercase {
    ( $string:expr ) => {
        $crate::functions::string::uppercase_fn($string)
    };
}
pub use string_uppercase as uppercase;

pub fn words_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "words")
}

#[macro_export]
macro_rules! string_words {
    ( $string:expr ) => {
        $crate::functions::string::words_fn($string)
    };
}
pub use string_words as words;

pub fn reverse_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "reverse")
}

#[macro_export]
macro_rules! string_reverse {
    ( $string:expr ) => {
        $crate::functions::string::reverse_fn($string)
    };
}

pub use string_reverse as reverse;

pub fn trim_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "trim")
}

#[macro_export]
macro_rules! string_trim {
    ( $string:expr ) => {
        $crate::functions::string::trim_fn($string)
    };
}
pub use string_trim as trim;

pub fn slug_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "slug")
}

#[macro_export]
macro_rules! string_slug {
    ( $string:expr ) => {
        $crate::functions::string::slug_fn($string)
    };
}

pub use string_slug as slug;

pub fn concat_fn<T: Into<Valuex>>(values: Vec<T>) -> Function {
    let mut bindings = vec![];

    let values = values
        .into_iter()
        .map(|v| {
            let v: Valuex = v.into();
            bindings.extend(v.get_bindings());
            v.build()
        })
        .collect::<Vec<_>>();

    let query_string = format!("string::concat({})", values.join(", "));

    Function {
        query_string,
        bindings,
    }
}

#[macro_export]
macro_rules! string_concat {
        ( $val:expr ) => {
            $crate::functions::string::concat_fn( $val )
        };
        ($( $val:expr ),*) => {
            $crate::functions::string::concat_fn($crate::arr![ $( $val ), * ])
        };
    }

pub use string_concat as concat;

pub fn join_fn<T: Into<Valuex>>(values: Vec<T>) -> Function {
    let mut bindings = vec![];

    let values = values
        .into_iter()
        .map(|v| {
            let v: Valuex = v.into();
            bindings.extend(v.get_bindings());
            v.build()
        })
        .collect::<Vec<_>>();

    let query_string = format!("string::join({})", values.join(", "));

    Function {
        query_string,
        bindings,
    }
}

#[macro_export]
macro_rules! string_join {
        ( $val:expr ) => {
            $crate::functions::string::join_fn( $val )
        };
        ($( $val:expr ),*) => {
            $crate::functions::string::join_fn($crate::arr![ $( $val ), * ])
        };
    }

pub use string_join as join;

pub fn ends_with_fn(string: impl Into<StrandLike>, ending: impl Into<StrandLike>) -> Function {
    let string: StrandLike = string.into();
    let ending: StrandLike = ending.into();
    let mut bindings = vec![];
    bindings.extend(string.get_bindings());
    bindings.extend(ending.get_bindings());

    let query_string = format!("string::ends_with({}, {})", string.build(), ending.build());

    Function {
        query_string,
        bindings,
    }
}

#[macro_export]
macro_rules! string_ends_with {
    ( $string:expr, $ending: expr ) => {
        $crate::functions::string::ends_with_fn($string, $ending)
    };
}

pub use string_ends_with as ends_with;

pub fn starts_with_fn(string: impl Into<StrandLike>, starting: impl Into<StrandLike>) -> Function {
    let string: StrandLike = string.into();
    let starting: StrandLike = starting.into();
    let mut bindings = vec![];
    bindings.extend(string.get_bindings());
    bindings.extend(starting.get_bindings());

    let query_string = format!(
        "string::starts_with({}, {})",
        string.build(),
        starting.build()
    );

    Function {
        query_string,
        bindings,
    }
}

#[macro_export]
macro_rules! string_starts_with {
    ( $string:expr, $ending: expr ) => {
        $crate::functions::string::starts_with_fn($string, $ending)
    };
}
pub use string_starts_with as starts_with;

pub fn split_fn(string: impl Into<StrandLike>, by: impl Into<StrandLike>) -> Function {
    let string: StrandLike = string.into();
    let by: StrandLike = by.into();
    let mut bindings = string.get_bindings();
    bindings.extend(by.get_bindings());

    let query_string = format!("string::split({}, {})", string.build(), by.build());

    Function {
        query_string,
        bindings,
    }
}

#[macro_export]
macro_rules! string_split {
    ( $string:expr, $by: expr ) => {
        $crate::functions::string::split_fn($string, $by)
    };
}

pub use string_split as split;

pub fn repeat_fn(string: impl Into<StrandLike>, ending: impl Into<NumberLike>) -> Function {
    let string: StrandLike = string.into();
    let ending: NumberLike = ending.into();
    let mut bindings = string.get_bindings();
    bindings.extend(ending.get_bindings());

    let query_string = format!("string::repeat({}, {})", string.build(), ending.build(),);

    Function {
        query_string,
        bindings,
    }
}

#[macro_export]
macro_rules! string_repeat {
    ( $string:expr, $ending: expr ) => {
        $crate::functions::string::repeat_fn($string, $ending)
    };
}

pub use string_repeat as repeat;

pub fn slice_fn(
    string: impl Into<StrandLike>,
    from: impl Into<NumberLike>,
    to: impl Into<NumberLike>,
) -> Function {
    let string: StrandLike = string.into();
    let from: NumberLike = from.into();
    let to: NumberLike = to.into();
    let mut bindings = string.get_bindings();
    bindings.extend(from.get_bindings());
    bindings.extend(to.get_bindings());

    let query_string = format!(
        "string::slice({}, {}, {})",
        string.build(),
        from.build(),
        to.build()
    );

    Function {
        query_string,
        bindings,
    }
}

#[macro_export]
macro_rules! string_slice {
    ( $string:expr, $from: expr, $to: expr ) => {
        $crate::functions::string::slice_fn($string, $from, $to)
    };
}

pub use string_slice as slice;

pub fn replace_fn(
    string: impl Into<StrandLike>,
    to_match: impl Into<StrandLike>,
    replacement: impl Into<StrandLike>,
) -> Function {
    let string: StrandLike = string.into();
    let to_match: StrandLike = to_match.into();
    let replacement: StrandLike = replacement.into();

    let mut bindings = string.get_bindings();
    bindings.extend(to_match.get_bindings());
    bindings.extend(replacement.get_bindings());

    let query_string = format!(
        "string::replace({}, {}, {})",
        string.build(),
        to_match.build(),
        replacement.build()
    );

    Function {
        query_string,
        bindings,
    }
}

#[macro_export]
macro_rules! string_replace {
    ( $string:expr, $match: expr, $replacement: expr ) => {
        $crate::functions::string::replace_fn($string, $match, $replacement)
    };
}

pub use string_replace as replace;

#[test]
fn test_concat_macro() {
    let title = Field::new("title");
    let result = self::concat!(title, "one", 3, 4.15385, "  ", true);
    assert_eq!(result.fine_tune_params(), "string::concat(title, $_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005)");
    assert_eq!(
        result.to_raw().build(),
        "string::concat(title, 'one', 3, 4.15385, '  ', true)"
    );
}

#[test]
fn test_concat_macro_with_array() {
    let result = self::concat!(arr!["one", "two", 3, 4.15385, "five", true]);
    assert_eq!(result.fine_tune_params(), "string::concat($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
    assert_eq!(
        result.to_raw().build(),
        "string::concat('one', 'two', 3, 4.15385, 'five', true)"
    );
}

#[test]
fn test_join_macro() {
    let title = Field::new("title");
    let result = join!(title, "one", 3, 4.15385, "  ", true);
    assert_eq!(result.fine_tune_params(), "string::join(title, $_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005)");
    assert_eq!(
        result.to_raw().build(),
        "string::join(title, 'one', 3, 4.15385, '  ', true)"
    );
}

#[test]
fn test_join_macro_with_array() {
    let result = join!(arr!["one", "two", 3, 4.15385, "five", true]);
    assert_eq!(result.fine_tune_params(), "string::join($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
    assert_eq!(
        result.to_raw().build(),
        "string::join('one', 'two', 3, 4.15385, 'five', true)"
    );
}

#[test]
fn test_ends_with_macro_with_field_and_string() {
    let name = Field::new("name");
    let result = ends_with!(name, "lowo");
    assert_eq!(
        result.fine_tune_params(),
        "string::ends_with(name, $_param_00000001)"
    );
    assert_eq!(result.to_raw().build(), "string::ends_with(name, 'lowo')");
}

#[test]
fn test_ends_with_macro_with_plain_strings() {
    let result = ends_with!("Oyelowo", "lowo");
    assert_eq!(
        result.fine_tune_params(),
        "string::ends_with($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().build(),
        "string::ends_with('Oyelowo', 'lowo')"
    );
}

#[test]
fn test_starts_with_macro_with_field_and_string() {
    let name = Field::new("name");
    let result = starts_with!(name, "lowo");
    assert_eq!(
        result.fine_tune_params(),
        "string::starts_with(name, $_param_00000001)"
    );
    assert_eq!(result.to_raw().build(), "string::starts_with(name, 'lowo')");
}

#[test]
fn test_starts_with_macro_with_plain_strings() {
    let result = starts_with!("Oyelowo", "Oye");
    assert_eq!(
        result.fine_tune_params(),
        "string::starts_with($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().build(),
        "string::starts_with('Oyelowo', 'Oye')"
    );
}
#[test]
fn test_split_macro_with_field_and_string() {
    let phrase = Field::new("phrase");
    let result = split!(phrase, ", ");
    assert_eq!(
        result.fine_tune_params(),
        "string::split(phrase, $_param_00000001)"
    );
    assert_eq!(result.to_raw().build(), "string::split(phrase, ', ')");
}

#[test]
fn test_split_macro_with_plain_strings() {
    let result = split!("With great power, comes great responsibility", ", ");
    assert_eq!(
        result.fine_tune_params(),
        "string::split($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().build(),
        "string::split('With great power, comes great responsibility', ', ')"
    );
}

#[test]
fn test_length_with_macro_with_field() {
    let name = Field::new("name");
    let result = length!(name);
    assert_eq!(result.fine_tune_params(), "string::length(name)");
    assert_eq!(result.to_raw().build(), "string::length(name)");
}

#[test]
fn test_length_with_macro_with_plain_string() {
    let result = length!("toronto");
    assert_eq!(
        result.fine_tune_params(),
        "string::length($_param_00000001)"
    );
    assert_eq!(result.to_raw().build(), "string::length('toronto')");
}

#[test]
fn test_reverse_with_macro_with_field() {
    let name = Field::new("name");
    let result = reverse!(name);
    assert_eq!(result.fine_tune_params(), "string::reverse(name)");
    assert_eq!(result.to_raw().build(), "string::reverse(name)");
}

#[test]
fn test_reverse_with_macro_with_plain_string() {
    let result = reverse!("oyelowo");
    assert_eq!(
        result.fine_tune_params(),
        "string::reverse($_param_00000001)"
    );
    assert_eq!(result.to_raw().build(), "string::reverse('oyelowo')");
}

#[test]
fn test_trim_with_macro_with_field() {
    let name = Field::new("name");
    let result = trim!(name);
    assert_eq!(result.fine_tune_params(), "string::trim(name)");
    assert_eq!(result.to_raw().build(), "string::trim(name)");
}

#[test]
fn test_trim_with_macro_with_plain_string() {
    let result = trim!("oyelowo");
    assert_eq!(result.fine_tune_params(), "string::trim($_param_00000001)");
    assert_eq!(result.to_raw().build(), "string::trim('oyelowo')");
}

#[test]
fn test_slug_with_macro_with_field() {
    let name = Field::new("name");
    let result = slug!(name);
    assert_eq!(result.fine_tune_params(), "string::slug(name)");
    assert_eq!(result.to_raw().build(), "string::slug(name)");
}

#[test]
fn test_slug_with_macro_with_plain_string() {
    let result = slug!("Codebreather is from #Jupiter");
    assert_eq!(result.fine_tune_params(), "string::slug($_param_00000001)");
    assert_eq!(
        result.to_raw().build(),
        "string::slug('Codebreather is from #Jupiter')"
    );
}

#[test]
fn test_lowercase_with_macro_with_field() {
    let name = Field::new("name");
    let result = lowercase!(name);
    assert_eq!(result.fine_tune_params(), "string::lowercase(name)");
    assert_eq!(result.to_raw().build(), "string::lowercase(name)");
}

#[test]
fn test_lowercase_with_macro_with_plain_string() {
    let result = lowercase!("OYELOWO");
    assert_eq!(
        result.fine_tune_params(),
        "string::lowercase($_param_00000001)"
    );
    assert_eq!(result.to_raw().build(), "string::lowercase('OYELOWO')");
}

#[test]
fn test_uppercase_with_macro_with_field() {
    let name = Field::new("name");
    let result = uppercase!(name);
    assert_eq!(result.fine_tune_params(), "string::uppercase(name)");
    assert_eq!(result.to_raw().build(), "string::uppercase(name)");
}

#[test]
fn test_uppercase_with_macro_with_plain_string() {
    let result = uppercase!("oyelowo");
    assert_eq!(
        result.fine_tune_params(),
        "string::uppercase($_param_00000001)"
    );
    assert_eq!(result.to_raw().build(), "string::uppercase('oyelowo')");
}

#[test]
fn test_words_with_macro_with_field() {
    let sentence = Field::new("sentence");
    let result = words!(sentence);
    assert_eq!(result.fine_tune_params(), "string::words(sentence)");
    assert_eq!(result.to_raw().build(), "string::words(sentence)");
}

#[test]
fn test_words_with_macro_with_plain_string() {
    let result = words!(
        "This is the first day of the rest of my life. I will make every single moment count!"
    );
    assert_eq!(result.fine_tune_params(), "string::words($_param_00000001)");
    assert_eq!(result.to_raw().build(), "string::words('This is the first day of the rest of my life. I will make every single moment count!')");
}

#[test]
fn test_repeat_with_macro_with_fields() {
    let name = Field::new("name");
    let count = Field::new("count");
    let result = repeat!(name, count);
    assert_eq!(result.fine_tune_params(), "string::repeat(name, count)");
    assert_eq!(result.to_raw().build(), "string::repeat(name, count)");
}

#[test]
fn test_repeat_with_macro_with_plain_string() {
    let result = repeat!("Oyelowo", 5);
    assert_eq!(
        result.fine_tune_params(),
        "string::repeat($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().build(), "string::repeat('Oyelowo', 5)");
}

#[test]
fn test_replace_with_macro_with_fields() {
    let name = Field::new("name");
    let last_name = Field::new("last_name");
    let first_name = Field::new("first_name");

    let result = replace!(name, last_name, first_name);
    assert_eq!(
        result.fine_tune_params(),
        "string::replace(name, last_name, first_name)"
    );
    assert_eq!(
        result.to_raw().build(),
        "string::replace(name, last_name, first_name)"
    );
}

#[test]
fn test_slice_with_macro_with_plain_string() {
    let result = slice!("Oyelowo", 3, 5);
    assert_eq!(
        result.fine_tune_params(),
        "string::slice($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(result.to_raw().build(), "string::slice('Oyelowo', 3, 5)");
}

#[test]
fn test_slice_with_macro_with_fields() {
    let name = Field::new("name");
    let last_name = Field::new("last_name");
    let first_name = Field::new("first_name");

    let result = slice!(name, last_name, first_name);
    assert_eq!(
        result.fine_tune_params(),
        "string::slice(name, last_name, first_name)"
    );
    assert_eq!(
        result.to_raw().build(),
        "string::slice(name, last_name, first_name)"
    );
}

#[test]
fn test_replace_with_macro_with_plain_strings() {
    let result = replace!("Oyelowo Oyedayo", "Oyelowo", "Oyedayo");
    assert_eq!(
        result.fine_tune_params(),
        "string::replace($_param_00000001, $_param_00000002, $_param_00000003)"
    );
    assert_eq!(
        result.to_raw().build(),
        "string::replace('Oyelowo Oyedayo', 'Oyelowo', 'Oyedayo')"
    );
}
