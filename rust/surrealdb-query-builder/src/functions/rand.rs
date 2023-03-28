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

use crate::sql::{Buildable, ToRawStatement};

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

    #[macro_export]
    macro_rules! bool {
        () => {
            crate::functions::rand::rand::bool_fn()
        };
    }

    pub(crate) use bool;

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
        () => {
            crate::functions::rand::rand::enum_fn()
        };
    }

    pub(crate) use enum_;

    pub fn float(from: impl Into<NumberOrEmpty>, to: impl Into<NumberOrEmpty>) -> Function {
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
}

#[test]
fn test_rand() {
    let result = rand();
    assert_eq!(result.fine_tune_params(), "rand()");
    assert_eq!(result.to_raw().to_string(), "rand()");
}

#[test]
fn test_rand_bool() {
    // crate::functions::rand::rand::

    let result = rand::bool!();
    assert_eq!(result.fine_tune_params(), "rand::bool()");
    assert_eq!(result.to_raw().to_string(), "rand::bool()");
}

#[test]
fn test_rand_enum() {
    let result = rand::enum_fn(array!["one", "two", 3, 4.15385, "five", true]);
    assert_eq!(result.fine_tune_params(), "rand::enum($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
    assert_eq!(
        result.to_raw().to_string(),
        "rand::enum('one', 'two', 3, 4.15385, 'five', true)"
    );
}
