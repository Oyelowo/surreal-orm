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
// string::len()	Returns the length of a string
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

use crate::{Buildable, Function, NumberLike, Parametric, StrandLike, Valuex};

fn create_fn_with_single_string_arg(value: impl Into<StrandLike>, function_name: &str) -> Function {
    let value: StrandLike = value.into();
    let query_string = format!("string::{function_name}({})", value.build());

    Function {
        query_string,
        bindings: value.get_bindings(),
    }
}

/// Returns length of a string
pub fn len_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "length")
}

/// Returns length of a string.
/// The macro function is also aliases as `string_len!`
///
/// # Arguments
/// * `string` - The string to get the length of. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let len = string::len!("Oyelowo Oyedayo");
/// assert_eq!(len.to_raw().build(), "string::length('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let len = string::len!(string_field);
/// assert_eq!(len.to_raw().build(), "string::length(string_field)");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let len = string::len!(string_param);
/// assert_eq!(len.to_raw().build(), "string::length($string_param)");
/// ```
#[macro_export]
macro_rules! string_len {
    ( $string:expr ) => {
        $crate::functions::string::len_fn($string)
    };
}
pub use string_len as len;

/// The string::lowercase function converts a string to lowercase.
pub fn lowercase_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "lowercase")
}

/// The string::lowercase function converts a string to lowercase.
/// The macro function is also aliases as `string_lowercase!`
/// # Arguments
/// * `string` - The string to convert to lowercase. Can be a string literal, a field or a parameter representing a string.
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let lowercase = string::lowercase!("Oyelowo Oyedayo");
/// assert_eq!(lowercase.to_raw().build(), "string::lowercase('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let lowercase = string::lowercase!(string_field);
/// assert_eq!(lowercase.to_raw().build(), "string::lowercase(string_field)");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let lowercase = string::lowercase!(string_param);
/// assert_eq!(lowercase.to_raw().build(), "string::lowercase($string_param)");
/// ```
#[macro_export]
macro_rules! string_lowercase {
    ( $string:expr ) => {
        $crate::functions::string::lowercase_fn($string)
    };
}

pub use string_lowercase as lowercase;

/// The string::uppercase function converts a string to uppercase.
pub fn uppercase_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "uppercase")
}

/// The string::uppercase function converts a string to uppercase.
/// The macro function is also aliases as `string_uppercase!`
///
/// # Arguments
///
/// * `string` - The string to convert to uppercase. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let uppercase = string::uppercase!("Oyelowo Oyedayo");
/// assert_eq!(uppercase.to_raw().build(), "string::uppercase('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let uppercase = string::uppercase!(string_field);
/// assert_eq!(uppercase.to_raw().build(), "string::uppercase(string_field)");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let uppercase = string::uppercase!(string_param);
/// assert_eq!(uppercase.to_raw().build(), "string::uppercase($string_param)");
/// ```
#[macro_export]
macro_rules! string_uppercase {
    ( $string:expr ) => {
        $crate::functions::string::uppercase_fn($string)
    };
}
pub use string_uppercase as uppercase;

/// The string::words function splits a string into an array of separate words.
pub fn words_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "words")
}

/// The string::words function splits a string into an array of separate words.
/// The macro function is also aliases as `string_words!`
///
/// # Arguments
/// * `string` - The string to split into words. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let words = string::words!("Oyelowo Oyedayo");
/// assert_eq!(words.to_raw().build(), "string::words('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let words = string::words!(string_field);
/// assert_eq!(words.to_raw().build(), "string::words(string_field)");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let words = string::words!(string_param);
/// assert_eq!(words.to_raw().build(), "string::words($string_param)");
/// ```
#[macro_export]
macro_rules! string_words {
    ( $string:expr ) => {
        $crate::functions::string::words_fn($string)
    };
}
pub use string_words as words;

/// The string::reverse function reverses a string.
pub fn reverse_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "reverse")
}

/// The string::reverse function reverses a string.
/// The macro function is also aliases as `string_reverse!`
///
/// # Arguments
/// * `string` - The string to reverse. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let reverse = string::reverse!("Oyelowo Oyedayo");
/// assert_eq!(reverse.to_raw().build(), "string::reverse('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let reverse = string::reverse!(string_field);
/// assert_eq!(reverse.to_raw().build(), "string::reverse(string_field)");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let reverse = string::reverse!(string_param);
/// assert_eq!(reverse.to_raw().build(), "string::reverse($string_param)");
/// ```
#[macro_export]
macro_rules! string_reverse {
    ( $string:expr ) => {
        $crate::functions::string::reverse_fn($string)
    };
}

pub use string_reverse as reverse;

/// The string::trim function removes leading and trailing whitespace from a string.
pub fn trim_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "trim")
}

/// The string::trim function removes leading and trailing whitespace from a string.
/// The macro function is also aliases as `string_trim!`
///
/// # Arguments
/// * `string` - The string to trim. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let trim = string::trim!("Oyelowo Oyedayo");
/// assert_eq!(trim.to_raw().build(), "string::trim('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let trim = string::trim!(string_field);
/// assert_eq!(trim.to_raw().build(), "string::trim(string_field)");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let trim = string::trim!(string_param);
/// assert_eq!(trim.to_raw().build(), "string::trim($string_param)");
/// ```
#[macro_export]
macro_rules! string_trim {
    ( $string:expr ) => {
        $crate::functions::string::trim_fn($string)
    };
}
pub use string_trim as trim;

/// The string::slug function converts a string into a human and URL-friendly string.
pub fn slug_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "slug")
}

/// The string::slug function converts a string into a human and URL-friendly string.
/// The macro function is also aliases as `string_slug!`
///
/// # Arguments
///
/// * `string` - The string to convert into a slug. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let slug = string::slug!("Oyelowo Oyedayo");
/// assert_eq!(slug.to_raw().build(), "string::slug('Oyelowo Oyedayo')");
/// let string_field = Field::new("string_field");
/// let slug = string::slug!(string_field);
/// assert_eq!(slug.to_raw().build(), "string::slug(string_field)");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let slug = string::slug!(string_param);
/// assert_eq!(slug.to_raw().build(), "string::slug($string_param)");
/// ```
#[macro_export]
macro_rules! string_slug {
    ( $string:expr ) => {
        $crate::functions::string::slug_fn($string)
    };
}

pub use string_slug as slug;

/// The string::concat function concatenates strings together.
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

/// The string::concat function concatenates strings together.
/// The macro function is also aliases as `string_concat!`
///
/// # Arguments
///
/// * `values` - The strings to concatenate. Can be string literals, fields or parameters representing strings.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let concat = string::concat!("Oyelowo", "Oyedayo");
/// assert_eq!(concat.to_raw().build(), "string::concat('Oyelowo', 'Oyedayo')");
///
/// let concat = string::concat!(vec!["Oyelowo", "Oyedayo"]);
/// assert_eq!(concat.to_raw().build(), "string::concat('Oyelowo', 'Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let concat = string::concat!(string_field, "Oyedayo");
/// assert_eq!(concat.to_raw().build(), "string::concat(string_field, 'Oyedayo')");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let concat = string::concat!(string_param, "Oyedayo");
/// assert_eq!(concat.to_raw().build(), "string::concat($string_param, 'Oyedayo')");
/// ```
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

/// The string::join function joins strings together with a delimiter.
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

/// The string::join function joins strings together with a delimiter.
/// The macro function is also aliases as `string_join!`
///
/// # Arguments
///
/// * `values` - The strings to join. Can be string literals, fields or parameters representing strings.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let join = string::join!("Oyelowo", "Oyedayo");
/// assert_eq!(join.to_raw().build(), "string::join('Oyelowo', 'Oyedayo')");
///
/// let join = string::join!(vec!["Oyelowo", "Oyedayo"]);
/// assert_eq!(join.to_raw().build(), "string::join('Oyelowo', 'Oyedayo')");
/// let string_field = Field::new("string_field");
///
/// let join = string::join!(string_field);
/// assert_eq!(join.to_raw().build(), "string::join(string_field)");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let join = string::join!(string_param);
/// assert_eq!(join.to_raw().build(), "string::join($string_param)");
/// ```
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

/// The string::endsWith function checks whether a string ends with another string.
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

/// The string::endsWith function checks whether a string ends with another string.
/// The macro function is also aliases as `string_ends_with!`
///
/// # Arguments
///
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `ending` - The string to check for. Can be a string literal, field or parameter representing a string.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let ends_with = string::ends_with!("Oyelowo", "Oyedayo");
/// assert_eq!(ends_with.to_raw().build(), "string::ends_with('Oyelowo', 'Oyedayo')");
/// let string_field = Field::new("string_field");
/// let ends_with = string::ends_with!(string_field, "Oyedayo");
/// assert_eq!(ends_with.to_raw().build(), "string::ends_with(string_field, 'Oyedayo')");
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let ends_with = string::ends_with!(string_param, "Oyedayo");
/// assert_eq!(ends_with.to_raw().build(), "string::ends_with($string_param, 'Oyedayo')");
/// ```
#[macro_export]
macro_rules! string_ends_with {
    ( $string:expr, $ending: expr ) => {
        $crate::functions::string::ends_with_fn($string, $ending)
    };
}

pub use string_ends_with as ends_with;

/// The string::startsWith function checks whether a string starts with another string.
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

/// The string::startsWith function checks whether a string starts with another string.
/// The macro function is also aliases as `string_starts_with!`
///
/// # Arguments
///
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `starting` - The string to check for. Can be a string literal, field or parameter representing a string.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let starts_with = string::starts_with!("Oyelowo", "Oyedayo");
/// assert_eq!(starts_with.to_raw().build(), "string::starts_with('Oyelowo', 'Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let starts_with = string::starts_with!(string_field, "Oyedayo");
/// assert_eq!(starts_with.to_raw().build(), "string::starts_with(string_field, 'Oyedayo')");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let starts_with = string::starts_with!(string_param, "Oyedayo");
/// assert_eq!(starts_with.to_raw().build(), "string::starts_with($string_param, 'Oyedayo')");
/// ```
#[macro_export]
macro_rules! string_starts_with {
    ( $string:expr, $ending: expr ) => {
        $crate::functions::string::starts_with_fn($string, $ending)
    };
}
pub use string_starts_with as starts_with;

/// The string::split function splits a string by a given delimiter.
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

/// The string::split function splits a string by a given delimiter.
/// The macro function is also aliases as `string_split!`
///
/// # Arguments
///
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `by` - The string to check for. Can be a string literal, field or parameter representing a string.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let split = string::split!("Oyelowo Oyedayo", " ");
/// assert_eq!(split.to_raw().build(), "string::split('Oyelowo Oyedayo', ' ')");
///
/// let string_field = Field::new("string_field");
/// let split = string::split!(string_field, " ");
/// assert_eq!(split.to_raw().build(), "string::split(string_field, ' ')");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let split = string::split!(string_param, " ");
/// assert_eq!(split.to_raw().build(), "string::split($string_param, ' ')");
/// ```
#[macro_export]
macro_rules! string_split {
    ( $string:expr, $by: expr ) => {
        $crate::functions::string::split_fn($string, $by)
    };
}

pub use string_split as split;

/// The string::repeat function repeats a string a number of times.
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

/// The string::repeat function repeats a string a number of times.
/// The macro function is also aliases as `string_repeat!`
///
/// # Arguments
///
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `count` - The number of times to repeat the string. Can be a number, field or parameter representing a number.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let repeat = string::repeat!("Oyelowo", 3);
/// assert_eq!(repeat.to_raw().build(), "string::repeat('Oyelowo', 3)");
/// let string_field = Field::new("string_field");
/// let repeat = string::repeat!(string_field, 3);
///
/// assert_eq!(repeat.to_raw().build(), "string::repeat(string_field, 3)");
/// let string_param = let_("string_param").equal("Oyelowo").get_param();
/// let repeat = string::repeat!(string_param, 3);
/// assert_eq!(repeat.to_raw().build(), "string::repeat($string_param, 3)");
/// ```
#[macro_export]
macro_rules! string_repeat {
    ( $string:expr, $count: expr ) => {
        $crate::functions::string::repeat_fn($string, $count)
    };
}

pub use string_repeat as repeat;

/// The string::slice function extracts and returns a section of a string.
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

/// The string::slice function extracts and returns a section of a string.
/// The macro function is also aliases as `string_slice!`
///
/// # Arguments
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `from` - The index to start the slice from. Can be a number, field or parameter representing a number.
/// * `to` - The index to end the slice at. Can be a number, field or parameter representing a number.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// let slice = string::slice!("Oyelowo Oyedayo", 0, 7);
/// assert_eq!(slice.to_raw().build(), "string::slice('Oyelowo Oyedayo', 0, 7)");
///
/// let string_field = Field::new("string_field");
/// let slice = string::slice!(string_field, 0, 7);
/// assert_eq!(slice.to_raw().build(), "string::slice(string_field, 0, 7)");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let slice = string::slice!(string_param, 0, 7);
/// assert_eq!(slice.to_raw().build(), "string::slice($string_param, 0, 7)");
/// ```
#[macro_export]
macro_rules! string_slice {
    ( $string:expr, $from: expr, $to: expr ) => {
        $crate::functions::string::slice_fn($string, $from, $to)
    };
}

pub use string_slice as slice;

/// The string::replace function replaces an occurence of a string with another string.
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

/// The string::replace function replaces an occurence of a string with another string.
/// The macro function is also aliases as `string_replace!`
///
/// # Arguments
///
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `to_match` - The string to match. Can be a string literal, field or parameter representing a string.
/// * `replacement` - The string to replace the matched string with. Can be a string literal, field or parameter representing a string.
///
/// # Example
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::string, statements::let_};
/// string::replace!("Oyelowo Oyedayo", "Oyedayo", "Oyelowo");
///
/// let string_field = Field::new("string_field");
/// let replace = string::replace!(string_field, "Oyedayo", "Oyelowo");
/// assert_eq!(replace.to_raw().build(), "string::replace(string_field, 'Oyedayo', 'Oyelowo')");
///
/// let string_param = let_("string_param").equal("Oyelowo Oyedayo").get_param();
/// let replace = string::replace!(string_param, "Oyedayo", "Oyelowo");
/// assert_eq!(replace.to_raw().build(), "string::replace($string_param, 'Oyedayo', 'Oyelowo')");
/// ```
#[macro_export]
macro_rules! string_replace {
    ( $string:expr, $match: expr, $replacement: expr ) => {
        $crate::functions::string::replace_fn($string, $match, $replacement)
    };
}

pub use string_replace as replace;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
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
        let result = len!(name);
        assert_eq!(result.fine_tune_params(), "string::length(name)");
        assert_eq!(result.to_raw().build(), "string::length(name)");
    }

    #[test]
    fn test_length_with_macro_with_plain_string() {
        let result = len!("toronto");
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
}
