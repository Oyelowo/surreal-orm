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

use crate::array;
use surrealdb::sql;

use crate::{
    sql::{Binding, Buildable, ToRawStatement},
    Field,
};

use super::{
    array::{ArrayOrField, Function},
    geo::NumberOrEmpty,
};

pub struct Number(sql::Value);

impl From<Number> for sql::Value {
    fn from(value: Number) -> Self {
        value.0
    }
}

impl<T: Into<sql::Number>> From<T> for Number {
    fn from(value: T) -> Self {
        let value: sql::Number = value.into();
        Self(value.into())
    }
}

impl From<Field> for Number {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}

pub struct Array(sql::Value);

impl From<Array> for sql::Value {
    fn from(value: Array) -> Self {
        value.0
    }
}

impl<T: Into<sql::Array>> From<T> for Array {
    fn from(value: T) -> Self {
        let value: sql::Array = value.into();
        Self(value.into())
    }
}

impl From<Field> for Array {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}
fn create_fn_with_single_num_arg(number: impl Into<Number>, function_name: &str) -> Function {
    let binding = Binding::new(number.into());
    let query_string = format!("math::{function_name}({})", binding.get_param_dollarised());

    Function {
        query_string,
        bindings: vec![binding],
    }
}

fn create_fn_with_single_array_arg(value: impl Into<Array>, function_name: &str) -> Function {
    let binding = Binding::new(value.into());
    let query_string = format!("math::{function_name}({})", binding.get_param_dollarised());

    Function {
        query_string,
        bindings: vec![binding],
    }
}

// Although, surrealdb technically accept stringified number also,
// I dont see why that should be allowed at the app layer in rust
// Obviously, if a field has stringified number that would work
// during query execution
fn abs(number: impl Into<Number>) -> Function {
    create_fn_with_single_num_arg(number, "abs")
}

fn ceil(number: impl Into<Number>) -> Function {
    create_fn_with_single_num_arg(number, "ceil")
}

fn floor(number: impl Into<Number>) -> Function {
    create_fn_with_single_num_arg(number, "floor")
}

fn round(number: impl Into<Number>) -> Function {
    create_fn_with_single_num_arg(number, "round")
}

fn max(number: impl Into<Array>) -> Function {
    create_fn_with_single_array_arg(number, "max")
}

fn mean(number: impl Into<Array>) -> Function {
    create_fn_with_single_array_arg(number, "mean")
}

fn median(number: impl Into<Array>) -> Function {
    create_fn_with_single_array_arg(number, "median")
}

fn min(number: impl Into<Array>) -> Function {
    create_fn_with_single_array_arg(number, "min")
}

fn product(number: impl Into<Array>) -> Function {
    create_fn_with_single_array_arg(number, "product")
}

fn fixed(number: impl Into<Number>, decimal_number: impl Into<Number>) -> Function {
    let num_binding = Binding::new(number.into());
    let decimal_place_binding = Binding::new(decimal_number.into());

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

use paste::paste;

macro_rules! create_test_for_fn_with_single_arg {
    ($function_ident: ident, $function_name_str: expr) => {
        paste! {
            #[test]
            fn [<test_ $function_ident _fn_with_field_data >] () {
                let temparate = Field::new("temperature");
                let result = $function_ident(temparate);

                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name_str));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(temperature)", $function_name_str));
            }

            #[test]
            fn [<test_ $function_ident _fn_with_fraction>]() {
                let result = $function_ident(45.23);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name_str));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(45.23)", $function_name_str));
            }

            #[test]
            fn [<test_ $function_ident _fn_with_negative_number>]() {
                let result = $function_ident(-454);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name_str));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(-454)", $function_name_str));
            }
        }
    };
}

create_test_for_fn_with_single_arg!(abs, "abs");
create_test_for_fn_with_single_arg!(ceil, "ceil");
create_test_for_fn_with_single_arg!(floor, "floor");
create_test_for_fn_with_single_arg!(round, "round");

macro_rules! create_test_for_fn_with_single_array_arg {
    ($function_ident: ident, $function_name_str: expr) => {
        paste! {
            #[test]
            fn [<test_ $function_ident _fn_with_field_data >] () {
                let size_list = Field::new("size_list");
                let result = $function_ident(size_list);

                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name_str));
                assert_eq!(result.to_raw().to_string(), format!("math::{}(size_list)", $function_name_str));
            }

            #[test]
            fn [<test_ $function_ident _fn_with_number_array>]() {
                let arr1 = array![1, 2, 3, 3.5];
                let result = $function_ident(arr1);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name_str));
                assert_eq!(result.to_raw().to_string(), format!("math::{}([1, 2, 3, 3.5])", $function_name_str));
            }

            #[test]
            fn [<test_ $function_ident _fn_with_mixed_array>]() {
                let age = Field::new("age");
                let arr = array![1, 2, 3, 4, 5, "4334", "Oyelowo", age];
                let result = $function_ident(arr);
                assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name_str));
                assert_eq!(result.to_raw().to_string(), format!("math::{}([1, 2, 3, 4, 5, '4334', 'Oyelowo', age])", $function_name_str));
            }
        }
    };
}

create_test_for_fn_with_single_array_arg!(max, "max");
create_test_for_fn_with_single_array_arg!(mean, "mean");
create_test_for_fn_with_single_array_arg!(median, "median");
create_test_for_fn_with_single_array_arg!(min, "min");
create_test_for_fn_with_single_array_arg!(product, "product");

#[test]
fn test_fixed_fn_with_field_data() {
    let land_size = Field::new("land_size");
    let decimal_place = Field::new("decimal_place");
    let result = fixed(land_size, decimal_place);

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
    let result = fixed(13.45423, 4);
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
    let result = fixed(land_mass, 4);
    assert_eq!(
        result.fine_tune_params(),
        "math::fixed($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "math::fixed(`\\`country.land_mass\\``, 4)"
    );
}
