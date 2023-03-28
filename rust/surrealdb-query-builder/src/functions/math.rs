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

use surrealdb::sql;

use crate::{
    sql::{Binding, Buildable, ToRawStatement},
    Field,
};

use super::array::Function;

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

fn create_fn_with_single_num_arg(number: impl Into<Number>, function_name: &str) -> Function {
    let binding = Binding::new(number.into());
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
