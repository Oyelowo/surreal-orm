/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
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
//

use surrealdb::sql;

use crate::sql::{Buildable, Empty, ToRawStatement};

use super::array::Function;

use crate::array;

fn rand() -> Function {
    let query_string = format!("rand()");

    Function {
        query_string,
        bindings: vec![],
    }
}

struct NumEmpty(sql::Value);

#[macro_export]
macro_rules! lowo {
    () => {
        bool()
    };
}
pub mod rand {
    use crate::{
        functions::{array::Function, geo::NumberOrEmpty, math::Array},
        sql::Binding,
    };
    use surrealdb::sql;

    pub fn bool_fn() -> Function {
        let query_string = format!("rand::bool()");

        Function {
            query_string,
            bindings: vec![],
        }
    }

    macro_rules! bool {
        () => {
            crate::functions::rand::rand::bool_fn()
        };
    }

    pub use bool;

    pub fn enum_fn<T: Into<sql::Value>>(values: Vec<T>) -> Function {
        // let values: sql::Value = values.into().into();
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

        // let binding = Binding::new(values.into());
        let query_string = format!("rand::enum({})", values.join(", "));

        Function {
            query_string,
            bindings,
        }
    }

    #[macro_export]
    macro_rules! enum_ {
        ( $val:expr ) => {
            crate::functions::rand::rand::enum_fn( $val )
        };
        ($( $val:expr ),*) => {
            crate::functions::rand::rand::enum_fn(crate::array![ $( $val ), * ])
        };
    }

    pub use enum_;

    pub fn float_fn(from: impl Into<NumberOrEmpty>, to: impl Into<NumberOrEmpty>) -> Function {
        let mut bindings = vec![];
        let from: NumberOrEmpty = from.into();
        let to: NumberOrEmpty = to.into();

        let query_string = match (from, to) {
            (NumberOrEmpty::Number(from), NumberOrEmpty::Number(to)) => {
                let from_binding = Binding::new(from);
                let to_binding = Binding::new(to);

                let query_string = format!(
                    "rand::float({}, {})",
                    from_binding.get_param_dollarised(),
                    to_binding.get_param_dollarised()
                );

                bindings = vec![from_binding, to_binding];
                query_string
            }
            _ => format!("rand::float()"),
        };

        Function {
            query_string,
            bindings,
        }
    }

    #[macro_export]
    macro_rules! float {
        () => {
            crate::functions::rand::rand::float_fn(crate::sql::Empty, crate::sql::Empty)
        };
        ( $from:expr, $to:expr ) => {
            crate::functions::rand::rand::float_fn($from, $to)
        };
    }

    pub use float;
}

#[test]
fn test_rand() {
    let result = rand();
    assert_eq!(result.fine_tune_params(), "rand()");
    assert_eq!(result.to_raw().to_string(), "rand()");
}

#[test]
fn test_rand_bool() {
    let result = rand::bool!();
    assert_eq!(result.fine_tune_params(), "rand::bool()");
    assert_eq!(result.to_raw().to_string(), "rand::bool()");
}

#[test]
fn test_rand_enum_macro() {
    let result = rand::enum_!("one", "two", 3, 4.15385, "five", true);
    assert_eq!(result.fine_tune_params(), "rand::enum($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
    assert_eq!(
        result.to_raw().to_string(),
        "rand::enum('one', 'two', 3, 4.15385, 'five', true)"
    );
}

#[test]
fn test_rand_enum_macro_with_array() {
    let result = rand::enum_!(array!["one", "two", 3, 4.15385, "five", true]);
    assert_eq!(result.fine_tune_params(), "rand::enum($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
    assert_eq!(
        result.to_raw().to_string(),
        "rand::enum('one', 'two', 3, 4.15385, 'five', true)"
    );
}

#[test]
fn test_rand_float_function_empty() {
    let result = rand::float_fn(Empty, Empty);
    assert_eq!(result.fine_tune_params(), "rand::float()");
    assert_eq!(result.to_raw().to_string(), "rand::float()");
}

#[test]
fn test_rand_float_macro_empty() {
    let result = rand::float!();
    assert_eq!(result.fine_tune_params(), "rand::float()");
    assert_eq!(result.to_raw().to_string(), "rand::float()");
}

#[test]
fn test_rand_float_macro_with_range() {
    let result = rand::float!(34, 65);
    assert_eq!(
        result.fine_tune_params(),
        "rand::float($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "rand::float(34, 65)");
}

#[test]
fn test_rand_float_macro_with_invalid_input() {
    let result = rand::float!(34, "ere");
    assert_eq!(
        result.fine_tune_params(),
        "rand::float($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "rand::float(34, 0)");
}
