/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

// These functions can be used when generating random data values.
//
// Function	Description
// rand()	Generates and returns a random floating point number
// rand::bool()	Generates and returns a random boolean
// rand::enum()	Randomly picks a value from the specified values
// rand::float()	Generates and returns a random floating point number
// rand::guid()	Generates and returns a random guid
// rand::int()	Generates and returns a random integer
// rand::string()	Generates and returns a random string
// rand::time()	Generates and returns a random datetime
// rand::uuid()	Generates and returns a random UUID
// rand::uuid::v4() Generates and returns a random Version 4 UUID
// rand::uuid::v7() Generates and returns a random Version 7 UUID

use crate::{Buildable, Erroneous, Function, NumberLike, Parametric, ValueLike};

/// The rand function generates a random float, between 0 and 1.
///
/// rand() -> number
/// The following example shows this function, and its output, when used in a select statement:
///
/// SELECT * FROM rand();
/// 0.7062321084863658
/// The following example shows this function being used in a SELECT statement with an ORDER BY clause:
///
/// SELECT * FROM [{ age: 33 }, { age: 45 }, { age: 39 }] ORDER BY rand();
/// [
///     {
///         age: 45
///     },
///    {
///        age: 39
///    },
///    {
///        age: 33
///    }
/// ]
pub fn rand_fn() -> Function {
    let query_string = "rand()".to_string();

    Function {
        query_string,
        bindings: vec![],
        errors: vec![],
    }
}

/// The `rand` function generates a random float, between 0 and 1.
///
/// # Example
/// ```rust
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::rand};
///
/// assert_eq!(rand!().to_raw().build(), "rand()");
/// ```
#[macro_export]
macro_rules! rand_rand {
    () => {
        $crate::functions::rand::rand_fn()
    };
}

pub use rand_rand as rand;

/// The rand::bool function generates a random boolean value.
pub fn bool_fn() -> Function {
    let query_string = "rand::bool()".to_string();

    Function {
        query_string,
        bindings: vec![],
        errors: vec![],
    }
}

/// The `rand::bool!` function generates a random boolean value.
/// The function is also aliased as `rand_bool!()`.
///
/// # Example
/// ```rust
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::rand};
/// assert_eq!(rand::bool!().to_raw().build(), "rand::bool()");
/// ```
#[macro_export]
macro_rules! rand_bool {
    () => {
        $crate::functions::rand::bool_fn()
    };
}

pub use rand_bool as bool;

/// The rand::uuid function generates a random UUID.
pub fn uuid_fn() -> Function {
    let query_string = "rand::uuid()".to_string();

    Function {
        query_string,
        bindings: vec![],
        errors: vec![],
    }
}

/// The `rand::uuid!` function generates a random UUID.
/// The function is also aliased as `rand_uuid!()`.
///
/// # Example
/// ```rust
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::rand};
/// assert_eq!(rand::uuid!().to_raw().build(), "rand::uuid()");
/// ```
#[macro_export]
macro_rules! rand_uuid {
    () => {
        $crate::functions::rand::uuid_fn()
    };
}

pub use rand_uuid as uuid;

/// The rand::enum function generates a random value, from a multitude of values.
pub fn enum_fn<T: Into<ValueLike>>(values: Vec<T>) -> Function {
    let mut bindings = vec![];

    let values = values
        .into_iter()
        .map(|v| {
            let v: ValueLike = v.into();
            bindings.extend(v.get_bindings());
            v.build()
        })
        .collect::<Vec<_>>();

    let query_string = format!("rand::enum({})", values.join(", "));

    Function {
        query_string,
        bindings,
        errors: vec![],
    }
}

/// The `rand::enum_!` function generates a random value, from a multitude of values.
/// The function is also aliased as `rand_enum_!()`.
///
/// # Example
/// ```rust
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::rand};
/// assert_eq!(rand::enum_!(1, 2, 3).to_raw().build(), "rand::enum(1, 2, 3)");
/// assert_eq!(rand::enum_!(arr![1, 2, 3]).to_raw().build(), "rand::enum(1, 2, 3)");
/// ```
#[macro_export]
macro_rules! rand_enum {
    ( $val:expr ) => {
        $crate::functions::rand::enum_fn( $val )
    };
    ($( $val:expr ),*) => {
        $crate::functions::rand::enum_fn($crate::array![ $( $val ), * ])
    };
}

pub use rand_enum as enum_;

/// The rand::float function generates a random float, between 0 and 1.
///
/// rand::float() -> float
/// If two numbers are provided, then the function generates a random float, between two numbers.
///
/// rand::float(number, number) -> float
pub fn float_fn(
    from: Option<impl Into<NumberLike>>,
    to: Option<impl Into<NumberLike>>,
) -> Function {
    let mut bindings = vec![];
    let mut errors = vec![];

    let query_string = match (from, to) {
        (Some(from), Some(to)) => {
            let from: NumberLike = from.into();
            let to: NumberLike = to.into();

            bindings.extend(from.get_bindings());
            bindings.extend(to.get_bindings());
            errors.extend(from.get_errors());
            errors.extend(to.get_errors());
            format!("rand::float({}, {})", from.build(), to.build())
        }
        _ => "rand::float()".to_string(),
    };

    Function {
        query_string,
        bindings,
        errors,
    }
}

/// The `rand::float!` function generates a random float, between 0 and 1 if no arguments are
/// provided, otherwise it generates a random float between the two numbers provided.
///
/// The function is also aliased as `rand_float!()`.
/// If you want to specify a range, you can pass the range as arguments.
///
/// # Arguments
/// * `from` - The minimum value of the range. Can be a number, field or parameter reprsenting
/// the number.
/// * `to` - The maximum value of the range. Can be a number, field or parameter reprsenting
/// the number.
///
/// # Example
/// ```rust
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::rand};
/// assert_eq!(rand::float!().to_raw().build(), "rand::float()");
/// assert_eq!(rand::float!(1, 2).to_raw().build(), "rand::float(1, 2)");
///
/// # let minimum_field = Field::new("minimum_field");
/// # let maximum_field = Field::new("maximum_field");
/// assert_eq!(rand::float!(minimum_field, maximum_field).to_raw().build(), "rand::float(minimum_field, maximum_field)");
///
/// # let minimum_param = Param::new("minimum_param");
/// # let maximum_param = Param::new("maximum_param");
/// assert_eq!(rand::float!(minimum_param, maximum_param).to_raw().build(), "rand::float($minimum_param, $maximum_param)");
#[macro_export]
macro_rules! rand_float {
    () => {
        $crate::functions::rand::float_fn(
            None as Option<$crate::NumberLike>,
            None as Option<$crate::NumberLike>,
        )
    };
    ( $from:expr, $to:expr ) => {
        $crate::functions::rand::float_fn(Some($from), Some($to))
    };
}

pub use rand_float as float;

/// The rand::int function generates a random int.
///
/// rand::int() -> int
/// If two numbers are provided, then the function generates a random int, between two numbers.
///
/// rand::int(number, number) -> int
/// The following examples show this function, and its output, when used in a select statement:
pub fn int_fn(from: Option<impl Into<NumberLike>>, to: Option<impl Into<NumberLike>>) -> Function {
    let mut bindings = vec![];
    let mut errors = vec![];

    let query_string = match (from, to) {
        (Some(from), Some(to)) => {
            let from: NumberLike = from.into();
            let to: NumberLike = to.into();

            bindings.extend(from.get_bindings());
            bindings.extend(to.get_bindings());
            errors.extend(from.get_errors());
            errors.extend(to.get_errors());
            format!("rand::int({}, {})", from.build(), to.build())
        }
        _ => "rand::int()".to_string(),
    };

    Function {
        query_string,
        bindings,
        errors,
    }
}

/// Generates a random integer if no arguments are provided, otherwise it generates a random
/// integer between the two numbers provided.
/// The function is also aliased as `rand_int!()`.
/// If you want to specify a range, you can pass the range as arguments.
///
/// # Arguments
/// * `from` - The minimum value of the range. Can be a number, field or parameter reprsenting
/// the number.
/// * `to` - The maximum value of the range. Can be a number, field or parameter reprsenting
/// the number.
///
/// # Example
/// ```rust
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::rand};
/// assert_eq!(rand::int!().to_raw().build(), "rand::int()");
/// assert_eq!(rand::int!(1, 2).to_raw().build(), "rand::int(1, 2)");
/// # let minimum_field = Field::new("minimum_field");
/// # let maximum_field = Field::new("maximum_field");
/// assert_eq!(rand::int!(minimum_field, maximum_field).to_raw().build(), "rand::int(minimum_field, maximum_field)");
/// # let minimum_param = Param::new("minimum_param");
/// # let maximum_param = Param::new("maximum_param");
/// assert_eq!(rand::int!(minimum_param, maximum_param).to_raw().build(), "rand::int($minimum_param, $maximum_param)");
/// ```
#[macro_export]
macro_rules! rand_int {
    () => {
        $crate::functions::rand::int_fn(
            None as Option<$crate::NumberLike>,
            None as Option<$crate::NumberLike>,
        )
    };
    ( $from:expr, $to:expr ) => {
        $crate::functions::rand::int_fn(Some($from), Some($to))
    };
}
pub use rand_int as int;

/// The rand::time function generates a random datetime.
///
/// rand::time() -> datetime
/// The rand::time function generates a random datetime, between two unix timestamps.
///
/// rand::time(number, number) -> datetime
pub fn time_fn(from: Option<impl Into<NumberLike>>, to: Option<impl Into<NumberLike>>) -> Function {
    let mut bindings = vec![];
    let mut errors = vec![];

    let query_string = match (from, to) {
        (Some(from), Some(to)) => {
            let from: NumberLike = from.into();
            let to: NumberLike = to.into();
            let query_string = format!("rand::time({}, {})", from.build(), to.build());

            bindings.extend(from.get_bindings());
            bindings.extend(to.get_bindings());
            errors.extend(from.get_errors());
            errors.extend(to.get_errors());
            query_string
        }
        _ => "rand::time()".to_string(),
    };

    Function {
        query_string,
        bindings,
        errors,
    }
}

/// Generates a random datetime if no arguments are provided, otherwise it generates a random
/// datetime between the two unix timestamps provided.
/// The function is also aliased as `rand_time!()`.
/// If you want to specify a range, you can pass the range as arguments.
///
/// # Arguments
/// * `from` - The minimum value of the range. Can be a number, field or parameter reprsenting
/// the number.
/// * `to` - The maximum value of the range. Can be a number, field or parameter reprsenting
/// the number.
///
/// # Example
/// ```rust
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::rand};
/// assert_eq!(rand::time!().to_raw().build(), "rand::time()");
/// assert_eq!(rand::time!(1, 2).to_raw().build(), "rand::time(1, 2)");
/// # let minimum_field = Field::new("minimum_field");
/// # let maximum_field = Field::new("maximum_field");
/// assert_eq!(rand::time!(minimum_field, maximum_field).to_raw().build(), "rand::time(minimum_field, maximum_field)");
/// # let minimum_param = Param::new("minimum_param");
/// # let maximum_param = Param::new("maximum_param");
/// assert_eq!(rand::time!(minimum_param, maximum_param).to_raw().build(), "rand::time($minimum_param, $maximum_param)");
/// ```
#[macro_export]
macro_rules! rand_time {
    () => {
        $crate::functions::rand::time_fn(
            None as Option<$crate::NumberLike>,
            None as Option<$crate::NumberLike>,
        )
    };
    ( $from:expr, $to:expr ) => {
        $crate::functions::rand::time_fn(Some($from), Some($to))
    };
}
pub use rand_time as time;

/// The rand::string function generates a random string, with 32 characters.
///
/// rand::string() -> string
/// The rand::string function generates a random string, with a specific length.
///
/// rand::string(number) -> string
/// If two numbers are provided, then the function generates a random string, with a length between two numbers.
pub fn string_fn(
    from: Option<impl Into<NumberLike>>,
    to: Option<impl Into<NumberLike>>,
) -> Function {
    let mut bindings = vec![];
    let mut errors = vec![];

    let query_string = match (from, to) {
        (Some(length), None) => {
            let length: NumberLike = length.into();

            bindings.extend(length.get_bindings());
            format!("rand::string({})", length.build())
        }
        (Some(from), Some(to)) => {
            let from: NumberLike = from.into();
            let to: NumberLike = to.into();

            bindings.extend(from.get_bindings());
            bindings.extend(to.get_bindings());
            errors.extend(from.get_errors());
            errors.extend(to.get_errors());

            format!("rand::string({}, {})", from.build(), to.build())
        }
        _ => "rand::string()".to_string(),
    };

    Function {
        query_string,
        bindings,
        errors,
    }
}

/// Generates a random string if no arguments are provided. If one argument is provided, then it
/// generates a random string with the length provided. If two arguments are provided, then it
/// generates a random string with a length between the two numbers provided.
/// The function is also aliased as `rand_string!()`.
///
/// # Arguments
/// * `from` - The length of the string if one argument is provided. If two arguments are
/// provided, then it is the minimum length of the string. Can be a number, field or parameter
/// reprsenting the number.
/// * `to` - The maximum length of the string. Can be a number, field or parameter reprsenting
/// the number.
///
/// # Example
/// ```rust
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::rand};
///
/// assert_eq!(rand::string!().to_raw().build(), "rand::string()");
/// assert_eq!(rand::string!(1).to_raw().build(), "rand::string(1)");
/// assert_eq!(rand::string!(1, 2).to_raw().build(), "rand::string(1, 2)");
/// # let minimum_field = Field::new("minimum_field");
/// # let maximum_field = Field::new("maximum_field");
/// assert_eq!(rand::string!(minimum_field, maximum_field).to_raw().build(), "rand::string(minimum_field, maximum_field)");
/// # let minimum_param = Param::new("minimum_param");
/// # let maximum_param = Param::new("maximum_param");
/// assert_eq!(rand::string!(minimum_param, maximum_param).to_raw().build(), "rand::string($minimum_param, $maximum_param)");
/// ```
#[macro_export]
macro_rules! rand_string {
    () => {
        $crate::functions::rand::string_fn(
            None as Option<$crate::NumberLike>,
            None as Option<$crate::NumberLike>,
        )
    };
    ( $length:expr) => {
        $crate::functions::rand::string_fn(Some($length), None as Option<$crate::NumberLike>)
    };
    ( $from:expr, $to:expr ) => {
        $crate::functions::rand::string_fn(Some($from), Some($to))
    };
}
pub use rand_string as string;

/// The rand::guid function generates a 20-character random guid.
///
/// rand::guid() -> string
/// If a number is provided, then the function generates a random guid, with a specific length.
pub fn guid_fn(length: Option<impl Into<NumberLike>>) -> Function {
    match length {
        None => Function {
            query_string: "rand::guid()".into(),
            bindings: vec![],
            errors: vec![],
        },
        Some(length) => {
            let length: NumberLike = length.into();
            let query_string = format!("rand::guid({})", length.build());

            Function {
                query_string,
                bindings: length.get_bindings(),
                errors: length.get_errors(),
            }
        }
    }
}

/// Generates a random guid if no arguments are provided. If one argument is provided, then it
/// generates a random guid with the length provided.
/// The function is also aliased as `rand_guid!()`.
///
/// # Arguments
///
/// * `length` - The length of the guid. Can be a number, field or parameter reprsenting the
/// number.
///
/// # Example
/// ```rust
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::rand};
///
/// assert_eq!(rand::guid!().to_raw().build(), "rand::guid()");
/// assert_eq!(rand::guid!(1).to_raw().build(), "rand::guid(1)");
/// # let length_field = Field::new("length_field");
/// assert_eq!(rand::guid!(length_field).to_raw().build(), "rand::guid(length_field)");
/// # let length_param = Param::new("length_param");
/// assert_eq!(rand::guid!(length_param).to_raw().build(), "rand::guid($length_param)");
/// ```
#[macro_export]
macro_rules! rand_guid {
    () => {
        $crate::functions::rand::guid_fn(None as Option<$crate::NumberLike>)
    };
    ( $length:expr ) => {
        $crate::functions::rand::guid_fn(Some($length))
    };
}

pub use rand_guid as guid;

/// This module contains functions for generating versioned random uuids.
pub mod uuid {
    use super::*;

    /// The rand::uuid::v4 function generates a random Version 4 UUID.
    ///
    /// rand::uuid::v4() -> uuid
    pub fn v4_fn() -> Function {
        Function {
            query_string: "rand::uuid::v4()".into(),
            bindings: vec![],
            errors: vec![],
        }
    }

    /// Generates a random Version 4 UUID.
    /// The function is also aliased as `rand_uuid_v4!()`.
    ///
    /// # Example
    /// ```rust
    /// use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::rand};
    ///
    /// assert_eq!(rand::uuid::v4!().to_raw().build(), "rand::uuid::v4()");
    /// ```
    #[macro_export]
    macro_rules! rand_uuid_v4 {
        () => {
            $crate::functions::rand::uuid::v4_fn()
        };
    }

    pub use rand_uuid_v4 as v4;

    /// The rand::uuid::v7 function generates a random Version 7 UUID.
    ///
    /// rand::uuid::v7() -> uuid
    pub fn v7_fn() -> Function {
        Function {
            query_string: "rand::uuid::v7()".into(),
            bindings: vec![],
            errors: vec![],
        }
    }

    /// Generates a random Version 7 UUID.
    /// The function is also aliased as `rand_uuid_v7!()`.
    ///
    /// # Example
    /// ```rust
    /// use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::rand};
    ///
    /// assert_eq!(rand::uuid::v7!().to_raw().build(), "rand::uuid::v7()");
    /// ```
    #[macro_export]
    macro_rules! rand_uuid_v7 {
        () => {
            $crate::functions::rand::uuid::v7_fn()
        };
    }

    pub use rand_uuid_v7 as v7;
}

#[cfg(test)]
mod tests {
    use crate::{arr, Buildable, Field, NumberLike, ToRaw};

    use crate::functions::rand;

    #[test]
    fn test_rand_fn() {
        let result = rand::rand_fn();
        assert_eq!(result.fine_tune_params(), "rand()");
        assert_eq!(result.to_raw().build(), "rand()");
    }

    #[test]
    fn test_rand_macro() {
        let result = rand!();
        assert_eq!(result.fine_tune_params(), "rand()");
        assert_eq!(result.to_raw().build(), "rand()");
    }

    #[test]
    fn test_rand_bool_fn() {
        let result = rand::bool_fn();
        assert_eq!(result.fine_tune_params(), "rand::bool()");
        assert_eq!(result.to_raw().build(), "rand::bool()");
    }

    #[test]
    fn test_rand_bool_macro() {
        let result = rand::bool!();
        assert_eq!(result.fine_tune_params(), "rand::bool()");
        assert_eq!(result.to_raw().build(), "rand::bool()");
    }

    #[test]
    fn test_rand_uuid_fn() {
        let result = rand::uuid_fn();
        assert_eq!(result.fine_tune_params(), "rand::uuid()");
        assert_eq!(result.to_raw().build(), "rand::uuid()");
    }

    #[test]
    fn test_rand_uuid() {
        let result = rand::uuid!();
        assert_eq!(result.fine_tune_params(), "rand::uuid()");
        assert_eq!(result.to_raw().build(), "rand::uuid()");
    }

    #[test]
    fn test_rand_uuid_v4_fn() {
        let result = rand::uuid::v4_fn();
        assert_eq!(result.fine_tune_params(), "rand::uuid::v4()");
        assert_eq!(result.to_raw().build(), "rand::uuid::v4()");
    }

    #[test]
    fn test_rand_uuid_v4() {
        let result = rand::uuid::v4!();
        assert_eq!(result.fine_tune_params(), "rand::uuid::v4()");
        assert_eq!(result.to_raw().build(), "rand::uuid::v4()");
    }

    #[test]
    fn test_rand_uuid_v7_fn() {
        let result = rand::uuid::v7_fn();
        assert_eq!(result.fine_tune_params(), "rand::uuid::v7()");
        assert_eq!(result.to_raw().build(), "rand::uuid::v7()");
    }

    #[test]
    fn test_rand_uuid_v7() {
        let result = rand::uuid::v7!();
        assert_eq!(result.fine_tune_params(), "rand::uuid::v7()");
        assert_eq!(result.to_raw().build(), "rand::uuid::v7()");
    }

    #[test]
    fn test_rand_enum_macro() {
        let result = rand::enum_!("one", "two", 3, 4.15385, "five", true);
        assert_eq!(result.fine_tune_params(), "rand::enum($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
        assert_eq!(
            result.to_raw().build(),
            "rand::enum('one', 'two', 3, 4.15385f, 'five', true)"
        );
    }

    #[test]
    fn test_rand_enum_macro_with_array() {
        let result = rand::enum_!(arr!["one", "two", 3, 4.15385, "five", true]);
        assert_eq!(result.fine_tune_params(), "rand::enum($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
        assert_eq!(
            result.to_raw().build(),
            "rand::enum('one', 'two', 3, 4.15385f, 'five', true)"
        );
    }

    #[test]
    fn test_rand_string_macro_with_one_arg_length() {
        let result = rand::string!(34);
        assert_eq!(result.fine_tune_params(), "rand::string($_param_00000001)");
        assert_eq!(result.to_raw().build(), "rand::string(34)");
    }

    #[test]
    fn test_rand_string_macro_with_one_arg_field() {
        let length_of_name = Field::new("length_of_name");
        let result = rand::string!(length_of_name);
        assert_eq!(result.fine_tune_params(), "rand::string(length_of_name)");
        assert_eq!(result.to_raw().build(), "rand::string(length_of_name)");
    }

    // Test Guid
    #[test]
    fn test_rand_guid_function_empty() {
        let result = rand::guid_fn(None as Option<NumberLike>);
        assert_eq!(result.fine_tune_params(), "rand::guid()");
        assert_eq!(result.to_raw().build(), "rand::guid()");
    }

    #[test]
    fn test_rand_guid_macro_empty() {
        let result = rand::guid!();
        assert_eq!(result.fine_tune_params(), "rand::guid()");
        assert_eq!(result.to_raw().build(), "rand::guid()");
    }

    #[test]
    fn test_rand_guid_macro_with_range() {
        let result = rand::guid!(34);
        assert_eq!(result.fine_tune_params(), "rand::guid($_param_00000001)");
        assert_eq!(result.to_raw().build(), "rand::guid(34)");
    }

    #[test]
    fn test_rand_guid_fn_with_field_input() {
        let length = Field::new("length");

        let result = rand::guid_fn(Some(length));
        assert_eq!(result.fine_tune_params(), "rand::guid(length)");
        assert_eq!(result.to_raw().build(), "rand::guid(length)");
    }

    #[test]
    fn test_rand_guid_macro_with_field_input() {
        let length = Field::new("length");

        let result = rand::guid!(length);
        assert_eq!(result.fine_tune_params(), "rand::guid(length)");
        assert_eq!(result.to_raw().build(), "rand::guid(length)");
    }
}

macro_rules! create_test_for_fn_with_two_args {
    ($function_ident: expr) => {
        paste::paste! {
            #[cfg(test)]
            mod [<test_ $function_ident _fn>] {
                use crate::*;
                use crate::functions::rand;

                #[test]
                fn [<test_rand_ $function_ident _function_empty>]() {
                    let result = rand::[< $function_ident _fn>](None as Option<$crate::NumberLike>, None as Option<$crate::NumberLike>);
                    assert_eq!(result.fine_tune_params(), format!("rand::{}()", $function_ident));
                    assert_eq!(result.to_raw().build(), format!("rand::{}()", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident _macro_empty>]() {
                    let result = rand::[< $function_ident>]!();
                    assert_eq!(result.fine_tune_params(), format!("rand::{}()", $function_ident));
                    assert_eq!(result.to_raw().build(), format!("rand::{}()", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident _macro_with_range>]() {
                    let result = rand::[< $function_ident>]!(34, 65);
                    assert_eq!(result.fine_tune_params(), format!("rand::{}($_param_00000001, $_param_00000002)", $function_ident));
                    assert_eq!(result.to_raw().build(), format!("rand::{}(34, 65)", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident fn_with_field_inputs>]() {
                    let start = Field::new("start");
                    let end = Field::new("end");

                    let result = rand::[< $function_ident _fn>](Some(start), Some(end));
                    assert_eq!(result.fine_tune_params(), format!("rand::{}(start, end)", $function_ident));
                    assert_eq!(result.to_raw().build(), format!("rand::{}(start, end)", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident macro_with_field_inputs>]() {
                    let start = Field::new("start");
                    let end = Field::new("end");

                    let result = rand::[< $function_ident>]!(start, end);
                    assert_eq!(result.fine_tune_params(), format!("rand::{}(start, end)", $function_ident));
                    assert_eq!(result.to_raw().build(), format!("rand::{}(start, end)", $function_ident));
                }
            }
        }
    };
}
create_test_for_fn_with_two_args!("float");
create_test_for_fn_with_two_args!("int");
create_test_for_fn_with_two_args!("string");
create_test_for_fn_with_two_args!("time");
