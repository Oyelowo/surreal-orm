/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Math functions
// These functions can be used when analysing numeric data and numeric collections.
//
// Function	Description
// math::abs()	Returns the absolute value of a number
// math::ceil()	Rounds a number up to the next largest integer
// math::fixed()	Returns a number with the specified number of decimal places
// math::floor()	Rounds a number down to the next largest integer
// math::max()	Returns the maximum number in a set of numbers
// math::mean()	Returns the mean of a set of numbers
// math::median()	Returns the median of a set of numbers
// math::min()	Returns the minimum number in a set of numbers
// math::product()	Returns the product of a set of numbers
// math::round()	Rounds a number up or down to the nearest integer
// math::sqrt()	Returns the square root of a number
// math::sum()	Returns the total sum of a set of numbers

use crate::{
    array,
    types::{ArrayLike, Field, Function, NumberLike, Param},
};
use surrealdb::sql;

use crate::traits::{Binding, Buildable, ToRaw};

fn create_fn_with_single_num_arg(number: impl Into<NumberLike>, function_name: &str) -> Function {
    let binding = Binding::new(number.into());
    let query_string = format!("math::{function_name}({})", binding.get_param_dollarised());

    Function {
        query_string,
        bindings: vec![binding],
    }
}

fn create_fn_with_single_array_arg(value: impl Into<ArrayLike>, function_name: &str) -> Function {
    let binding = Binding::new(value.into());
    let query_string = format!("math::{function_name}({})", binding.get_param_dollarised());

    Function {
        query_string,
        bindings: vec![binding],
    }
}

fn fixed_fn(number: impl Into<NumberLike>, decimal_place: impl Into<NumberLike>) -> Function {
    let num_binding = Binding::new(number.into());
    let decimal_place_binding = Binding::new(decimal_place.into());

    let query_string = format!(
        "math::fixed({}, {})",
        num_binding.get_param_dollarised(),
        decimal_place_binding.get_param_dollarised()
    );

    Function {
        query_string,
        bindings: vec![num_binding, decimal_place_binding],
    }
}

#[macro_export]
macro_rules! math_fixed {
    ( $number:expr, $decimal_place:expr ) => {
        crate::functions::math::fixed_fn($number, $decimal_place)
    };
}

pub use math_fixed as fixed;

macro_rules! create_test_for_fn_with_single_arg {
    ($function_name: expr) => {
        paste::paste! {
            // Although, surrealdb technically accepts stringified number also,
            // I dont see why that should be allowed at the app layer in rust
            // Obviously, if a field has stringified number that would work
            // during query execution
            fn [<$function_name _fn>](number: impl Into<NumberLike>) -> Function {
                create_fn_with_single_num_arg(number, $function_name)
            }

            #[macro_export]
            macro_rules!  [<math_ $function_name>] {
                ( $value:expr ) => {
                    crate::functions::math::[<$function_name _fn>]($value)
                };
            }

            pub use [<math_ $function_name>] as [<$function_name>];

            #[test]
            fn [<test_ $function_name _fn_with_field_data >] () {
                let temparate = Field::new("temperature");
                let result = [<$function_name _fn>](temparate);

                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(temperature)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _fn_with_fraction>]() {
                let result = [<$function_name _fn>](45.23);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(45.23)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _fn_with_negative_number>]() {
                let result = [<$function_name _fn>](-454);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(-454)", $function_name));
            }

            // Macro version
            #[test]
            fn [<test_ $function_name _macro_with_field_data >] () {
                let temparate = Field::new("temperature");
                let result = [<$function_name>]!(temparate);

                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(temperature)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_with_param >] () {
                let temparate = Param::new("temperature");
                let result = [<$function_name>]!(temparate);

                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}($temperature)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_with_fraction>]() {
                let result = [<$function_name>]!(45.23);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(45.23)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_with_negative_number>]() {
                let result = [<$function_name>]!(-454);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(-454)", $function_name));
            }

        }
    };
}

create_test_for_fn_with_single_arg!("abs");
create_test_for_fn_with_single_arg!("ceil");
create_test_for_fn_with_single_arg!("floor");
create_test_for_fn_with_single_arg!("round");
create_test_for_fn_with_single_arg!("sqrt");

macro_rules! create_test_for_fn_with_single_array_arg {
    ($function_name: expr) => {
        paste::paste! {
            fn [<$function_name _fn>](number: impl Into<ArrayLike>) -> Function {
                create_fn_with_single_array_arg(number, $function_name)
            }

            #[macro_export]
            macro_rules!  [<math_ $function_name>] {
                ( $value:expr ) => {
                    crate::functions::math::[<$function_name _fn>]($value)
                };
            }
            pub use [<math_ $function_name>] as [<$function_name>];

            #[test]
            fn [<test_ $function_name _fn_with_field_data >] () {
                let size_list = Field::new("size_list");
                let result = [<$function_name _fn>](size_list);

                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(size_list)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _fn_with_number_array>]() {
                let arr1 = array![1, 2, 3, 3.5];
                let result = [<$function_name _fn>](arr1);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}([1, 2, 3, 3.5])", $function_name));
            }

            #[test]
            fn [<test_ $function_name _fn_with_mixed_array>]() {
                let age = Field::new("age");
                let arr = array![1, 2, 3, 4, 5, "4334", "Oyelowo", age];
                let result = [<$function_name _fn>](arr);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}([1, 2, 3, 4, 5, '4334', 'Oyelowo', age])", $function_name));
            }

            // Macro version
            #[test]
            fn [<test_ $function_name _macro_with_field_data >] () {
                let size_list = Field::new("size_list");
                let result = [<$function_name>]!(size_list);

                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(size_list)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_with_param >] () {
                let size_list = Param::new("size_list");
                let result = [<$function_name>]!(size_list);

                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}($size_list)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_with_number_array>]() {
                let arr1 = array![1, 2, 3, 3.5];
                let result = [<$function_name>]!(arr1);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}([1, 2, 3, 3.5])", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_with_mixed_array>]() {
                let age = Field::new("age");
                let arr = array![1, 2, 3, 4, 5, "4334", "Oyelowo", age];
                let result = [<$function_name>]!(arr);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("math::{}([1, 2, 3, 4, 5, '4334', 'Oyelowo', age])", $function_name));
            }
        }
    };
}

create_test_for_fn_with_single_array_arg!("max");
create_test_for_fn_with_single_array_arg!("mean");
create_test_for_fn_with_single_array_arg!("median");
create_test_for_fn_with_single_array_arg!("min");
create_test_for_fn_with_single_array_arg!("product");
create_test_for_fn_with_single_array_arg!("sum");

#[test]
fn test_fixed_fn_with_field_data() {
    let land_size = Field::new("land_size");
    let decimal_place = Field::new("decimal_place");
    let result = fixed_fn(land_size, decimal_place);

    assert_eq!(
        result.fine_tune_params(),
        "math::fixed($_param_00000001, $_param_00000002)"
    );

    assert_eq!(
        result.to_raw().to_string(),
        "math::fixed(land_size, decimal_place)"
    );
}

#[test]
fn test_fixed_fn_with_raw_numbers() {
    let result = fixed_fn(13.45423, 4);
    let email = Field::new("email");
    let arr = array![1, 2, 3, 4, 5, "4334", "Oyelowo", email];
    assert_eq!(
        result.fine_tune_params(),
        "math::fixed($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "math::fixed(13.45423, 4)");
}

#[test]
fn test_fixed_fn_with_raw_number_with_field() {
    let land_mass = Field::new("country.land_mass");
    let result = fixed_fn(land_mass, 4);
    assert_eq!(
        result.fine_tune_params(),
        "math::fixed($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "math::fixed(`country.land_mass`, 4)"
    );
}

// Macro versions
#[test]
fn test_fixed_macro_with_field_data() {
    let land_size = Field::new("land_size");
    let decimal_place = Field::new("decimal_place");
    let result = fixed!(land_size, decimal_place);

    assert_eq!(
        result.fine_tune_params(),
        "math::fixed($_param_00000001, $_param_00000002)"
    );

    assert_eq!(
        result.to_raw().to_string(),
        "math::fixed(land_size, decimal_place)"
    );
}

#[test]
fn test_fixed_macro_with_params() {
    let land_size = Param::new("land_size");
    let decimal_place = Param::new("decimal_place");
    let result = fixed!(land_size, decimal_place);

    assert_eq!(
        result.fine_tune_params(),
        "math::fixed($_param_00000001, $_param_00000002)"
    );

    assert_eq!(
        result.to_raw().to_string(),
        "math::fixed($land_size, $decimal_place)"
    );
}

#[test]
fn test_fixed_macro_with_raw_numbers() {
    let result = fixed!(13.45423, 4);
    let email = Field::new("email");
    let arr = array![1, 2, 3, 4, 5, "4334", "Oyelowo", email];
    assert_eq!(
        result.fine_tune_params(),
        "math::fixed($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "math::fixed(13.45423, 4)");
}

#[test]
fn test_fixed_macro_with_raw_number_with_field() {
    let land_mass = Field::new("country.land_mass");
    let result = fixed!(land_mass, 4);
    assert_eq!(
        result.fine_tune_params(),
        "math::fixed($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "math::fixed(`country.land_mass`, 4)"
    );
}
