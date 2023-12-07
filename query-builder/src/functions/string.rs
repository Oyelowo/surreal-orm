/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// String functions
// These functions can be used when working with and manipulating text and string values.
//
// Function	Description
// string::concat()	Concatenates strings together
// string::contains()	Check wether a string contains another string
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
// string::distance::hamming
// string::distance::levenshtein,
// string::similarity::fuzzy
// string::similarity::jaro
// string::similarity::smithwaterman
// string::is::alphanum()	Checks whether a value has only alphanumeric characters
// string::is::alpha()	Checks whether a value has only alpha characters
// string::is::ascii()	Checks whether a value has only ascii characters
// string::is::datetime() Checks whether a value matches a datetime format
// string::is::domain()	Checks whether a value is a domain
// string::is::email()	Checks whether a value is an email
// string::is::hexadecimal()	Checks whether a value is hexadecimal
// string::is::latitude()	Checks whether a value is a latitude value
// string::is::longitude()	Checks whether a value is a longitude value
// string::is::numeric()	Checks whether a value has only numeric characters
// string::is::semver()	Checks whether a value matches a semver version
// string::is::url() Checks whether a value is a valid URL
// string::is::uuid()	Checks whether a value is a UUID

use crate::{
    ArgsList, Buildable, Erroneous, Function, NumberLike, Parametric, StrandLike, ValueLike,
};

fn create_fn_with_single_string_arg(value: impl Into<StrandLike>, function_name: &str) -> Function {
    let value: StrandLike = value.into();
    let query_string = format!("string::{function_name}({})", value.build());

    Function {
        query_string,
        bindings: value.get_bindings(),
        errors: value.get_errors(),
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let len = string::len!("Oyelowo Oyedayo");
/// assert_eq!(len.to_raw().build(), "string::length('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let len = string::len!(string_field);
/// assert_eq!(len.to_raw().build(), "string::length(string_field)");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let lowercase = string::lowercase!("Oyelowo Oyedayo");
/// assert_eq!(lowercase.to_raw().build(), "string::lowercase('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let lowercase = string::lowercase!(string_field);
/// assert_eq!(lowercase.to_raw().build(), "string::lowercase(string_field)");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let uppercase = string::uppercase!("Oyelowo Oyedayo");
/// assert_eq!(uppercase.to_raw().build(), "string::uppercase('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let uppercase = string::uppercase!(string_field);
/// assert_eq!(uppercase.to_raw().build(), "string::uppercase(string_field)");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let words = string::words!("Oyelowo Oyedayo");
/// assert_eq!(words.to_raw().build(), "string::words('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let words = string::words!(string_field);
/// assert_eq!(words.to_raw().build(), "string::words(string_field)");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let reverse = string::reverse!("Oyelowo Oyedayo");
/// assert_eq!(reverse.to_raw().build(), "string::reverse('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let reverse = string::reverse!(string_field);
/// assert_eq!(reverse.to_raw().build(), "string::reverse(string_field)");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let trim = string::trim!("Oyelowo Oyedayo");
/// assert_eq!(trim.to_raw().build(), "string::trim('Oyelowo Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let trim = string::trim!(string_field);
/// assert_eq!(trim.to_raw().build(), "string::trim(string_field)");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let slug = string::slug!("Oyelowo Oyedayo");
/// assert_eq!(slug.to_raw().build(), "string::slug('Oyelowo Oyedayo')");
/// let string_field = Field::new("string_field");
/// let slug = string::slug!(string_field);
/// assert_eq!(slug.to_raw().build(), "string::slug(string_field)");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
pub fn concat_fn<T: Into<ValueLike>>(values: Vec<T>) -> Function {
    let mut bindings = vec![];
    let mut errors = vec![];

    let values = values
        .into_iter()
        .map(|v| {
            let v: ValueLike = v.into();
            bindings.extend(v.get_bindings());
            errors.extend(v.get_errors());
            v.build()
        })
        .collect::<Vec<_>>();

    let query_string = format!("string::concat({})", values.join(", "));

    Function {
        query_string,
        bindings,
        errors,
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
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
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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

// contains
/// The string::contains function checks whether a string contains another string.
pub fn contains_fn(string: impl Into<StrandLike>, contains: impl Into<StrandLike>) -> Function {
    let string: StrandLike = string.into();
    let contains: StrandLike = contains.into();
    let mut bindings = vec![];
    let mut errors = vec![];
    bindings.extend(string.get_bindings());
    bindings.extend(contains.get_bindings());
    errors.extend(string.get_errors());
    errors.extend(contains.get_errors());

    let query_string = format!("string::contains({}, {})", string.build(), contains.build());

    Function {
        query_string,
        bindings,
        errors,
    }
}

/// The string::contains function checks whether a string contains another string.
/// The macro function is also aliases as `string_contains!`
/// # Arguments
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `contains` - The string to check for. Can be a string literal, field or parameter representing a string.
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
///
/// let contains = string::contains!("Oyelowo Oyedayo", "Oyedayo");
/// assert_eq!(contains.to_raw().build(), "string::contains('Oyelowo Oyedayo', 'Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let contains = string::contains!(string_field, "Oyedayo");
/// assert_eq!(contains.to_raw().build(), "string::contains(string_field, 'Oyedayo')");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
/// let contains = string::contains!(string_param, "Oyedayo");
/// assert_eq!(contains.to_raw().build(), "string::contains($string_param, 'Oyedayo')");
/// ```
#[macro_export]
macro_rules! string_contains {
    ( $string:expr, $contains: expr ) => {
        $crate::functions::string::contains_fn($string, $contains)
    };
}
pub use string_contains as contains;

/// The string::join function joins strings together with a delimiter.
pub fn join_fn(values: impl Into<ArgsList>) -> Function {
    let values: ArgsList = values.into();
    let query_string = format!("string::join({})", values.build());

    Function {
        query_string,
        bindings: values.get_bindings(),
        errors: values.get_errors(),
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
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
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
    let mut errors = vec![];
    bindings.extend(string.get_bindings());
    bindings.extend(ending.get_bindings());
    errors.extend(string.get_errors());
    errors.extend(ending.get_errors());

    let query_string = format!("string::ends_with({}, {})", string.build(), ending.build());

    Function {
        query_string,
        bindings,
        errors,
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let ends_with = string::ends_with!("Oyelowo", "Oyedayo");
/// assert_eq!(ends_with.to_raw().build(), "string::ends_with('Oyelowo', 'Oyedayo')");
/// let string_field = Field::new("string_field");
/// let ends_with = string::ends_with!(string_field, "Oyedayo");
/// assert_eq!(ends_with.to_raw().build(), "string::ends_with(string_field, 'Oyedayo')");
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
    let mut errors = vec![];
    bindings.extend(string.get_bindings());
    bindings.extend(starting.get_bindings());
    errors.extend(string.get_errors());
    errors.extend(starting.get_errors());

    let query_string = format!(
        "string::starts_with({}, {})",
        string.build(),
        starting.build()
    );

    Function {
        query_string,
        bindings,
        errors,
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let starts_with = string::starts_with!("Oyelowo", "Oyedayo");
/// assert_eq!(starts_with.to_raw().build(), "string::starts_with('Oyelowo', 'Oyedayo')");
///
/// let string_field = Field::new("string_field");
/// let starts_with = string::starts_with!(string_field, "Oyedayo");
/// assert_eq!(starts_with.to_raw().build(), "string::starts_with(string_field, 'Oyedayo')");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
    let mut errors = string.get_errors();
    bindings.extend(by.get_bindings());
    errors.extend(by.get_errors());

    let query_string = format!("string::split({}, {})", string.build(), by.build());

    Function {
        query_string,
        bindings,
        errors,
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let split = string::split!("Oyelowo Oyedayo", " ");
/// assert_eq!(split.to_raw().build(), "string::split('Oyelowo Oyedayo', ' ')");
///
/// let string_field = Field::new("string_field");
/// let split = string::split!(string_field, " ");
/// assert_eq!(split.to_raw().build(), "string::split(string_field, ' ')");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
    let mut errors = string.get_errors();
    bindings.extend(ending.get_bindings());
    errors.extend(ending.get_errors());

    let query_string = format!("string::repeat({}, {})", string.build(), ending.build(),);

    Function {
        query_string,
        bindings,
        errors,
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let repeat = string::repeat!("Oyelowo", 3);
/// assert_eq!(repeat.to_raw().build(), "string::repeat('Oyelowo', 3)");
/// let string_field = Field::new("string_field");
/// let repeat = string::repeat!(string_field, 3);
///
/// assert_eq!(repeat.to_raw().build(), "string::repeat(string_field, 3)");
/// let string_param = let_("string_param").equal_to("Oyelowo").get_param();
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
    let mut errors = string.get_errors();

    bindings.extend(from.get_bindings());
    bindings.extend(to.get_bindings());
    errors.extend(from.get_errors());
    errors.extend(to.get_errors());

    let query_string = format!(
        "string::slice({}, {}, {})",
        string.build(),
        from.build(),
        to.build()
    );

    Function {
        query_string,
        bindings,
        errors,
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// let slice = string::slice!("Oyelowo Oyedayo", 0, 7);
/// assert_eq!(slice.to_raw().build(), "string::slice('Oyelowo Oyedayo', 0, 7)");
///
/// let string_field = Field::new("string_field");
/// let slice = string::slice!(string_field, 0, 7);
/// assert_eq!(slice.to_raw().build(), "string::slice(string_field, 0, 7)");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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
    let mut errors = string.get_errors();

    bindings.extend(to_match.get_bindings());
    bindings.extend(replacement.get_bindings());
    errors.extend(to_match.get_errors());
    errors.extend(replacement.get_errors());

    let query_string = format!(
        "string::replace({}, {}, {})",
        string.build(),
        to_match.build(),
        replacement.build()
    );

    Function {
        query_string,
        bindings,
        errors,
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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::string, statements::let_};
/// string::replace!("Oyelowo Oyedayo", "Oyedayo", "Oyelowo");
///
/// let string_field = Field::new("string_field");
/// let replace = string::replace!(string_field, "Oyedayo", "Oyelowo");
/// assert_eq!(replace.to_raw().build(), "string::replace(string_field, 'Oyedayo', 'Oyelowo')");
///
/// let string_param = let_("string_param").equal_to("Oyelowo Oyedayo").get_param();
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

fn create_string_validation_function(value: impl Into<ValueLike>, function_name: &str) -> Function {
    let value: ValueLike = value.into();

    Function {
        query_string: format!("string::is::{function_name}({})", value.build()),
        bindings: value.get_bindings(),
        errors: value.get_errors(),
    }
}

macro_rules! create_validation_with_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](value: impl Into<$crate::ValueLike>) -> $crate::Function {
                super::create_string_validation_function(value, $function_name)
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules!  [<string_validation_is_ $function_name>]{
                ( $geometry:expr ) => {
                    $crate::functions::string::is::[<$function_name _fn>]($geometry)
                };
            }
            pub use [<string_validation_is_ $function_name>] as [<$function_name>];

            #[cfg(test)]
            mod [<test_ $function_name>] {
                use crate::*;
                use crate::functions::string::is;

                #[test]
                fn [<test_ $function_name _with_field>] ()  {
                    let username = Field::new("username");
                    let result = is::[<$function_name _fn>](username);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}(username)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(username)", $function_name));
                    }

                #[test]
                fn [<test_ $function_name _string_username>] ()  {
                    let result = is::[<$function_name _fn>]("oyelowo1234");

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}('oyelowo1234')", $function_name));
                }

                #[test]
                fn [<test_ $function_name _with_number>] ()  {
                    let result = is::[<$function_name _fn>](123456423);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(123456423)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _with_fraction>] ()  {
                    let result = is::[<$function_name _fn>](12.3456423);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(12.3456423f)", $function_name));
                }

                // Macro versions
                #[test]
                fn [<test_ $function_name _macro_with_field>] ()  {
                    let username = Field::new("username");
                    let result = is::[<$function_name>]!(username);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}(username)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(username)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_string_username>] ()  {
                    let result = is::[<$function_name>]!("oyelowo1234");

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}('oyelowo1234')", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_number>] ()  {
                    let result = is::[<$function_name>]!(123456423);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(123456423)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_fraction>] ()  {
                    let result = is::[<$function_name>]!(12.3456423);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(12.3456423f)", $function_name));
                }
            }

        }
    };
}

fn create_two_strings_args_helper(
    str1: impl Into<StrandLike>,
    str2: impl Into<StrandLike>,
    func_name: &str,
) -> Function {
    let str1: StrandLike = str1.into();
    let str2: StrandLike = str2.into();
    let mut bindings = vec![];
    let mut errors = vec![];
    bindings.extend(str1.get_bindings());
    bindings.extend(str2.get_bindings());
    errors.extend(str1.get_errors());
    errors.extend(str2.get_errors());
    Function {
        query_string: format!("string::{func_name}({}, {})", str1.build(), str2.build()),
        bindings,
        errors,
    }
}

macro_rules! create_fn_with_two_strings_args {
    ($(#[$attr:meta])* => $function_name:expr, $function_path:expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](str1: impl Into<$crate::StrandLike>, str2: impl Into<$crate::StrandLike>) -> $crate::Function {
                create_two_strings_args_helper(str1, str2, $function_path)
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<string_ $function_name>] {
                ( $str1:expr, $str2:expr ) => {
                    $crate::functions::string::[<$function_name _fn>]($str1, $str2)
                };
            }
            pub use [<string_ $function_name>];
            // pub(self) use [<string_ $function_name>] as [<$function_name>];

            #[cfg(test)]
            mod [<test_ $function_name>] {
                 use [<string_ $function_name>] as [<$function_name>];
                use $crate::{functions::string, *};

                #[test]
                fn [<test $function_name fn_on_strand_macro_on_diverse_strands>]() {
                    let name = Field::new("name");
                    let result = functions::string::[<$function_name _fn>](name, "Oyelowo");
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("string::{}(name, $_param_00000001)", $function_path)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("string::{}(name, 'Oyelowo')", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _fn_on_same_element_types>]() {
                    let result = string::[<$function_name _fn>]("Oyelowo", "Oyedayo");
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("string::{}($_param_00000001, $_param_00000002)", $function_path)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("string::{}('Oyelowo', 'Oyedayo')", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_strand_macro_on_diverse_strands>]() {
                    let name = Field::new("name");
                    let result = self::[<$function_name>]!(name, "Oyelowo");
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("string::{}(name, $_param_00000001)", $function_path)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("string::{}(name, 'Oyelowo')", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_same_element_types>]() {
                    let result = self::[<$function_name>]!("Oyelowo", "Oyedayo");
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("string::{}($_param_00000001, $_param_00000002)", $function_path)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("string::{}('Oyelowo', 'Oyedayo')", $function_path)
                    );
                }
            }
        }
    };
}

create_fn_with_two_strings_args!(
    /// The string::is::format function checks whether a value matches a format.
    /// Also aliased as `string_is_format!`
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check. Could be a field or a parameter that represents the
    /// value.
    /// * `format` - The format to check against. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::is::format!(name, "Oyelowo");
    /// assert_eq!(result.to_raw().build(), "string::is::format(name, 'Oyelowo')");
    ///
    /// let result = string::is::format!("Oyelowo", "Oyedayo");
    /// assert_eq!(result.to_raw().build(), "string::is::format('Oyelowo', 'Oyedayo')");
    /// ```
    =>
    "is_format",
    "is::format"
);

/// This module contains functions that validate values
pub mod is {
    pub use super::string_is_format as format;

    // The is::alphanum function checks whether a value has only alphanumeric characters.
    create_validation_with_tests!(
        /// The string::is::alphanum function checks whether a value has only alphanumeric characters.
        /// Also aliased as `string::is_alphanum!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::alphanum!("oyelowo1234");
        /// assert_eq!(result.to_raw().to_string(), "string::is::alphanum('oyelowo1234')");
        ///
        /// let alphanum_field = Field::new("alphanum_field");
        /// let result = string::is::alphanum!(alphanum_field);
        /// assert_eq!(result.to_raw().to_string(), "string::is::alphanum(alphanum_field)");
        ///
        /// let alphanum_param = let_("alphanum_param").equal_to("oyelowo1234").get_param();
        /// let result = string::is::alphanum!(alphanum_param);
        /// assert_eq!(result.fine_tune_params(), "string::is::alphanum($alphanum_param)");
        /// ```
        =>
        "alphanum"
    );

    create_validation_with_tests!(
        /// The string::is::alpha function checks whether a value has only alpha characters.
        /// Also aliased as `string::is_alpha!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::alpha!("oyelowo");
        /// assert_eq!(result.to_raw().to_string(), "string::is::alpha('oyelowo')");
        ///
        /// let alpha_field = Field::new("alpha_field");
        /// let result = string::is::alpha!(alpha_field);
        /// assert_eq!(result.to_raw().to_string(), "string::is::alpha(alpha_field)");
        ///
        /// let alpha_param = let_("alpha_param").equal_to("oyelowo").get_param();
        /// let result = string::is::alpha!(alpha_param);
        /// assert_eq!(result.fine_tune_params(), "string::is::alpha($alpha_param)");
        /// ```
        =>
        "alpha"
    );

    create_validation_with_tests!(
        /// The string::is::ascii function checks whether a value has only ascii characters.
        /// Also aliased as `string_is_ascii!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::ascii!("oyelowo");
        /// assert_eq!(result.to_raw().to_string(), "string::is::ascii('oyelowo')");
        ///
        /// let ascii_field = Field::new("ascii_field");
        /// let result = string::is::ascii!(ascii_field);
        /// assert_eq!(result.to_raw().to_string(), "string::is::ascii(ascii_field)");
        ///
        /// let ascii_param = let_("ascii_param").equal_to("oyelowo").get_param();
        /// let result = string::is::ascii!(ascii_param);
        /// assert_eq!(result.fine_tune_params(), "string::is::ascii($ascii_param)");
        /// ```
        =>
        "ascii"
    );

    create_validation_with_tests!(
        /// The string::is::domain function checks whether a value is a domain.
        /// Also aliased as `string_is_domain!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::domain!("oyelowo.com");
        /// assert_eq!(result.to_raw().to_string(), "string::is::domain('oyelowo.com')");
        ///
        /// let domain_field = Field::new("domain_field");
        /// let result = string::is::domain!(domain_field);
        /// assert_eq!(result.to_raw().to_string(), "string::is::domain(domain_field)");
        ///
        /// let domain_param = let_("domain_param").equal_to("oyelowo.com").get_param();
        /// let result = string::is::domain!(domain_param);
        /// assert_eq!(result.fine_tune_params(), "string::is::domain($domain_param)");
        /// ```
        =>
        "domain"
    );

    create_validation_with_tests!(
        /// The string::is::email function checks whether a value is an email.
        /// Also aliased as `string_is_email!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::email!("oyelowo@codebreather.com");
        /// assert_eq!(result.to_raw().to_string(), "string::is::email('oyelowo@codebreather.com')");
        ///
        /// let email_field = Field::new("email_field");
        /// let result = string::is::email!(email_field);
        /// assert_eq!(result.to_raw().to_string(), "string::is::email(email_field)");
        ///
        /// let email_param = let_("email_param").equal_to("oyelowo@codebreather").get_param();
        ///
        /// let result = string::is::email!(email_param);
        /// assert_eq!(result.fine_tune_params(), "string::is::email($email_param)");
        /// ```
        =>
        "email"
    );

    create_validation_with_tests!(
            /// The string::is::hexadecimal function checks whether a value is hexadecimal.
            /// Also aliased as `string_is_hexadecimal!`
            ///
            /// # Arguments
            ///
            /// * `value` - The value to check. Could be a field or a parameter that represents the
            /// value.
            ///
            /// # Example
            /// ```rust
            /// # use surreal_query_builder as surreal_orm;
            /// use surreal_orm::{*, functions::string, statements::let_};
            ///
            /// let result = string::is::hexadecimal!("oyelowo");
            /// assert_eq!(result.to_raw().to_string(), "string::is::hexadecimal('oyelowo')");
            ///
            /// let hexadecimal_field = Field::new("hexadecimal_field");
            /// let result = string::is::hexadecimal!(hexadecimal_field);
            /// assert_eq!(result.to_raw().to_string(), "string::is::hexadecimal(hexadecimal_field)");
            ///
            /// let hexadecimal_param = let_("hexadecimal_param").equal_to("oyelowo").get_param();
            /// let result = string::is::hexadecimal!(hexadecimal_param);
            /// assert_eq!(result.fine_tune_params(), "string::is::hexadecimal($hexadecimal_param)");
            /// ```
            =>
            "hexadecimal"
    );

    create_validation_with_tests!(
        /// The string::is::latitude function checks whether a value is a latitude value.
        /// Also aliased as `string_is_latitude!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::latitude!("-0.118092");
        /// assert_eq!(result.to_raw().to_string(), "string::is::latitude('-0.118092')");
        ///
        /// let latitude_field = Field::new("latitude_field");
        /// let result = string::is::latitude!(latitude_field);
        /// assert_eq!(result.to_raw().to_string(), "string::is::latitude(latitude_field)");
        ///
        /// let latitude_param = let_("latitude_param").equal_to("oyelowo").get_param();
        /// let result = string::is::latitude!(latitude_param);
        /// assert_eq!(result.fine_tune_params(), "string::is::latitude($latitude_param)");
        /// ```
        =>
        "latitude"
    );

    create_validation_with_tests!(
        /// The string::is::longitude function checks whether a value is a longitude value.
        /// Also aliased as `string_is_longitude!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::longitude!("51.509865");
        /// assert_eq!(result.to_raw().to_string(), "string::is::longitude('51.509865')");
        ///
        /// let longitude_field = Field::new("longitude_field");
        /// let result = string::is::longitude!(longitude_field);
        /// assert_eq!(result.to_raw().to_string(), "string::is::longitude(longitude_field)");
        ///
        /// let longitude_param = let_("longitude_param").equal_to("oyelowo").get_param();
        /// let result = string::is::longitude!(longitude_param);
        /// assert_eq!(result.fine_tune_params(), "string::is::longitude($longitude_param)");
        /// ```
        =>
        "longitude"
    );

    create_validation_with_tests!(
        /// The string::is::numeric function checks whether a value has only numeric characters.
        /// Also aliased as `string_is_numeric!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::numeric!("oyelowo");
        /// assert_eq!(result.to_raw().to_string(), "string::is::numeric('oyelowo')");
        ///
        /// let numeric_field = Field::new("numeric_field");
        /// let result = string::is::numeric!(numeric_field);
        /// assert_eq!(result.to_raw().to_string(), "string::is::numeric(numeric_field)");
        ///
        /// let numeric_param = let_("numeric_param").equal_to("oyelowo").get_param();
        /// let result = string::is::numeric!(numeric_param);
        /// assert_eq!(result.fine_tune_params(), "string::is::numeric($numeric_param)");
        /// ```
        =>
        "numeric"
    );

    create_validation_with_tests!(
        /// The string::is::semver function checks whether a value matches a semver version.
        /// Also aliased as `string_is_semver!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::semver!("oyelowo");
        /// assert_eq!(result.to_raw().to_string(), "string::is::semver('oyelowo')");
        ///
        /// let semver_field = Field::new("semver_field");
        /// let result = string::is::semver!(semver_field);
        /// assert_eq!(result.to_raw().to_string(), "string::is::semver(semver_field)");
        ///
        /// let semver_param = let_("semver_param").equal_to("oyelowo").get_param();
        /// let result = string::is::semver!(semver_param);
        /// assert_eq!(result.fine_tune_params(), "string::is::semver($semver_param)");
        /// ```
        =>
        "semver"
    );

    create_validation_with_tests!(
        /// The string::is::uuid function checks whether a value is a UUID.
        /// Also aliased as `string_is_uuid!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::uuid!("oyelowo");
        /// assert_eq!(result.to_raw().to_string(), "string::is::uuid('oyelowo')");
        ///
        /// let uuid_field = Field::new("uuid_field");
        /// let result = string::is::uuid!(uuid_field);
        /// assert_eq!(result.to_raw().to_string(), "string::is::uuid(uuid_field)");
        ///
        /// let uuid_param = let_("uuid_param").equal_to("oyelowo").get_param();
        /// let result = string::is::uuid!(uuid_param);
        /// assert_eq!(result.fine_tune_params(), "string::is::uuid($uuid_param)");
        /// ```
        =>
        "uuid"
    );

    create_validation_with_tests!(
        /// The string::is::datetime function checks whether a value matches a datetime format.
        /// Also aliased as `string_is_datetime!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// # use surreal_query_builder as surreal_orm;
        /// use surreal_orm::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::datetime!("oyelowo");
        /// assert_eq!(result.to_raw().to_string(), "string::is::datetime('oyelowo')");
        ///
        /// let datetime_field = Field::new("datetime_field");
        /// let result = string::is::datetime!(datetime_field);
        /// assert_eq!(result.to_raw().to_string(), "string::is::datetime(datetime_field)");
        ///
        /// let datetime_param = let_("datetime_param").equal_to("oyelowo").get_param();
        /// let result = string::is::datetime!(datetime_param);
        /// assert_eq!(result.fine_tune_params(), "string::is::datetime($datetime_param)");
        /// ```
        =>
        "datetime"
    );
}

#[cfg(test)]
mod tests {
    use crate::functions::string;
    use crate::*;
    #[test]
    fn test_concat_macro() {
        let title = Field::new("title");
        let result = string::concat!(title, "one", 3, 4.15385, "  ", true);
        assert_eq!(result.fine_tune_params(), "string::concat(title, $_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005)");
        assert_eq!(
            result.to_raw().build(),
            "string::concat(title, 'one', 3, 4.15385f, '  ', true)"
        );
    }

    #[test]
    fn test_concat_macro_with_array() {
        let result = string::concat!(arr!["one", "two", 3, 4.15385, "five", true]);
        assert_eq!(result.fine_tune_params(), "string::concat($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
        assert_eq!(
            result.to_raw().build(),
            "string::concat('one', 'two', 3, 4.15385f, 'five', true)"
        );
    }

    #[test]
    fn test_join_macro() {
        let title = Field::new("title");
        let result = string::join!(title, "one", 3, 4.15385, "  ", true);
        assert_eq!(result.fine_tune_params(), "string::join(title, $_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005)");
        assert_eq!(
            result.to_raw().build(),
            "string::join(title, 'one', 3, 4.15385f, '  ', true)"
        );
    }

    #[test]
    fn test_join_macro_with_array() {
        let result = string::join!(arr!["one", "two", 3, 4.15385, "five", true]);
        assert_eq!(result.fine_tune_params(), "string::join($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
        assert_eq!(
            result.to_raw().build(),
            "string::join('one', 'two', 3, 4.15385f, 'five', true)"
        );
    }

    #[test]
    fn test_ends_with_macro_with_field_and_string() {
        let name = Field::new("name");
        let result = string::ends_with!(name, "lowo");
        assert_eq!(
            result.fine_tune_params(),
            "string::ends_with(name, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::ends_with(name, 'lowo')");
    }

    #[test]
    fn test_ends_with_macro_with_plain_strings() {
        let result = string::ends_with!("Oyelowo", "lowo");
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
        let result = string::starts_with!(name, "lowo");
        assert_eq!(
            result.fine_tune_params(),
            "string::starts_with(name, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::starts_with(name, 'lowo')");
    }

    #[test]
    fn test_starts_with_macro_with_plain_strings() {
        let result = string::starts_with!("Oyelowo", "Oye");
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
        let result = string::split!(phrase, ", ");
        assert_eq!(
            result.fine_tune_params(),
            "string::split(phrase, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::split(phrase, ', ')");
    }

    #[test]
    fn test_split_macro_with_plain_strings() {
        let result = string::split!("With great power, comes great responsibility", ", ");
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
        let result = string::len!(name);
        assert_eq!(result.fine_tune_params(), "string::length(name)");
        assert_eq!(result.to_raw().build(), "string::length(name)");
    }

    #[test]
    fn test_length_with_macro_with_plain_string() {
        let result = string::len!("toronto");
        assert_eq!(
            result.fine_tune_params(),
            "string::length($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::length('toronto')");
    }

    #[test]
    fn test_reverse_with_macro_with_field() {
        let name = Field::new("name");
        let result = string::reverse!(name);
        assert_eq!(result.fine_tune_params(), "string::reverse(name)");
        assert_eq!(result.to_raw().build(), "string::reverse(name)");
    }

    #[test]
    fn test_reverse_with_macro_with_plain_string() {
        let result = string::reverse!("oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "string::reverse($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::reverse('oyelowo')");
    }

    #[test]
    fn test_trim_with_macro_with_field() {
        let name = Field::new("name");
        let result = string::trim!(name);
        assert_eq!(result.fine_tune_params(), "string::trim(name)");
        assert_eq!(result.to_raw().build(), "string::trim(name)");
    }

    #[test]
    fn test_trim_with_macro_with_plain_string() {
        let result = string::trim!("oyelowo");
        assert_eq!(result.fine_tune_params(), "string::trim($_param_00000001)");
        assert_eq!(result.to_raw().build(), "string::trim('oyelowo')");
    }

    #[test]
    fn test_slug_with_macro_with_field() {
        let name = Field::new("name");
        let result = string::slug!(name);
        assert_eq!(result.fine_tune_params(), "string::slug(name)");
        assert_eq!(result.to_raw().build(), "string::slug(name)");
    }

    #[test]
    fn test_slug_with_macro_with_plain_string() {
        let result = string::slug!("Codebreather is from #Jupiter");
        assert_eq!(result.fine_tune_params(), "string::slug($_param_00000001)");
        assert_eq!(
            result.to_raw().build(),
            "string::slug('Codebreather is from #Jupiter')"
        );
    }

    #[test]
    fn test_lowercase_with_macro_with_field() {
        let name = Field::new("name");
        let result = string::lowercase!(name);
        assert_eq!(result.fine_tune_params(), "string::lowercase(name)");
        assert_eq!(result.to_raw().build(), "string::lowercase(name)");
    }

    #[test]
    fn test_lowercase_with_macro_with_plain_string() {
        let result = string::lowercase!("OYELOWO");
        assert_eq!(
            result.fine_tune_params(),
            "string::lowercase($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::lowercase('OYELOWO')");
    }

    #[test]
    fn test_uppercase_with_macro_with_field() {
        let name = Field::new("name");
        let result = string::uppercase!(name);
        assert_eq!(result.fine_tune_params(), "string::uppercase(name)");
        assert_eq!(result.to_raw().build(), "string::uppercase(name)");
    }

    #[test]
    fn test_uppercase_with_macro_with_plain_string() {
        let result = string::uppercase!("oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "string::uppercase($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::uppercase('oyelowo')");
    }

    #[test]
    fn test_words_with_macro_with_field() {
        let sentence = Field::new("sentence");
        let result = string::words!(sentence);
        assert_eq!(result.fine_tune_params(), "string::words(sentence)");
        assert_eq!(result.to_raw().build(), "string::words(sentence)");
    }

    #[test]
    fn test_words_with_macro_with_plain_string() {
        let result = string::words!(
            "This is the first day of the rest of my life. I will make every single moment count!"
        );
        assert_eq!(result.fine_tune_params(), "string::words($_param_00000001)");
        assert_eq!(result.to_raw().build(), "string::words('This is the first day of the rest of my life. I will make every single moment count!')");
    }

    #[test]
    fn test_repeat_with_macro_with_fields() {
        let name = Field::new("name");
        let count = Field::new("count");
        let result = string::repeat!(name, count);
        assert_eq!(result.fine_tune_params(), "string::repeat(name, count)");
        assert_eq!(result.to_raw().build(), "string::repeat(name, count)");
    }

    #[test]
    fn test_repeat_with_macro_with_plain_string() {
        let result = string::repeat!("Oyelowo", 5);
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

        let result = string::replace!(name, last_name, first_name);
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
        let result = string::slice!("Oyelowo", 3, 5);
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

        let result = string::slice!(name, last_name, first_name);
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
        let result = string::replace!("Oyelowo Oyedayo", "Oyelowo", "Oyedayo");
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

create_fn_with_two_strings_args!(
    /// The string::distance::hamming function calculates the hamming distance between two strings.
    /// Also aliased as `string_distance_hamming!`
    ///
    /// # Arguments
    ///
    /// * `str1` - The first string to compare. Could be a field or a parameter that represents the
    /// value.
    /// * `str2` - The second string to compare. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::distance::hamming!(name, "Oyelowo");
    /// assert_eq!(result.to_raw().build(), "string::distance::hamming(name, 'Oyelowo')");
    ///
    /// let result = string::distance::hamming!("Oyelowo", "Oyedayo");
    /// assert_eq!(result.to_raw().build(), "string::distance::hamming('Oyelowo', 'Oyedayo')");
    /// ```
    =>
    "distance_hamming",
    "distance::hamming"
);

create_fn_with_two_strings_args!(
    /// The string::distance::levenshtein function calculates the levenshtein distance between two strings.
    /// Also aliased as `string_distance_levenshtein!`
    ///
    /// # Arguments
    ///
    /// * `str1` - The first string to compare. Could be a field or a parameter that represents the
    /// value.
    /// * `str2` - The second string to compare. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::distance::levenshtein!(name, "Oyelowo");
    /// assert_eq!(result.to_raw().build(), "string::distance::levenshtein(name, 'Oyelowo')");
    ///
    /// let result = string::distance::levenshtein!("Oyelowo", "Oyedayo");
    /// assert_eq!(result.to_raw().build(), "string::distance::levenshtein('Oyelowo', 'Oyedayo')");
    /// ```
    =>
    "distance_levenshtein",
    "distance::levenshtein"
);

/// The string::distance::levenshtein function calculates the damerau levenshtein distance between two strings.
pub mod distance {
    pub use super::string_distance_hamming as hamming;
    pub use super::string_distance_levenshtein as levenshtein;
}

create_fn_with_two_strings_args!(
    /// The string::similarity::fuzzy function calculates the fuzzy similarity between two strings.
    /// Also aliased as `string_similarity_fuzzy!`
    ///
    /// # Arguments
    ///
    /// * `str1` - The first string to compare. Could be a field or a parameter that represents the
    /// value.
    /// * `str2` - The second string to compare. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::similarity::fuzzy!(name, "Oyelowo");
    /// assert_eq!(result.to_raw().build(), "string::similarity::fuzzy(name, 'Oyelowo')");
    ///
    /// let result = string::similarity::fuzzy!("Oyelowo", "Oyedayo");
    /// assert_eq!(result.to_raw().build(), "string::similarity::fuzzy('Oyelowo', 'Oyedayo')");
    /// ```
    =>
    "similarity_fuzzy",
    "similarity::fuzzy"
);

create_fn_with_two_strings_args!(
    /// The string::similarity::jaro function calculates the jaro similarity between two strings.
    /// Also aliased as `string_similarity_jaro!`
    ///
    /// # Arguments
    ///
    /// * `str1` - The first string to compare. Could be a field or a parameter that represents the
    /// value.
    /// * `str2` - The second string to compare. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::similarity::jaro!(name, "Oyelowo");
    /// assert_eq!(result.to_raw().build(), "string::similarity::jaro(name, 'Oyelowo')");
    ///
    /// let result = string::similarity::jaro!("Oyelowo", "Oyedayo");
    /// assert_eq!(result.to_raw().build(), "string::similarity::jaro('Oyelowo', 'Oyedayo')");
    /// ```
    =>
    "similarity_jaro",
    "similarity::jaro"
);

create_fn_with_two_strings_args!(
    /// The string::similarity::smithwaterman function calculates the smithwaterman similarity between two strings.
    /// Also aliased as `string_similarity_smithwaterman!`
    ///
    /// # Arguments
    ///
    /// * `str1` - The first string to compare. Could be a field or a parameter that represents the
    /// value.
    /// * `str2` - The second string to compare. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::similarity::smithwaterman!(name, "Oyelowo");
    /// assert_eq!(result.to_raw().build(), "string::similarity::smithwaterman(name, 'Oyelowo')");
    ///
    /// let result = string::similarity::smithwaterman!("Oyelowo", "Oyedayo");
    /// assert_eq!(result.to_raw().build(), "string::similarity::smithwaterman('Oyelowo', 'Oyedayo')");
    /// ```
    =>
    "similarity_smithwaterman",
    "similarity::smithwaterman"
);

/// The string::similarity module contains functions that calculate the similarity between two strings.
pub mod similarity {
    pub use super::string_similarity_fuzzy as fuzzy;
    pub use super::string_similarity_jaro as jaro;
    pub use super::string_similarity_smithwaterman as smithwaterman;
}
