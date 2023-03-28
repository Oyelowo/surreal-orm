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

use crate::{
    sql::{Binding, Buildable, Empty, ToRawStatement},
    Field,
};

use super::{array::Function, math::Number};

use crate::array;

fn rand() -> Function {
    let query_string = format!("rand()");

    Function {
        query_string,
        bindings: vec![],
    }
}

struct NumEmpty(sql::Value);

pub(crate) fn create_fn_with_single_num_arg(
    number: impl Into<Number>,
    function_name: &str,
) -> Function {
    let binding = Binding::new(number.into());
    let query_string = format!("rand::{function_name}({})", binding.get_param_dollarised());

    Function {
        query_string,
        bindings: vec![binding],
    }
}

pub mod rand {
    use super::create_fn_with_single_num_arg;
    use crate::{
        functions::{
            array::Function,
            geo::NumberOrEmpty,
            math::{Array, Number},
        },
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

    pub fn int_fn(from: impl Into<NumberOrEmpty>, to: impl Into<NumberOrEmpty>) -> Function {
        let mut bindings = vec![];
        let from: NumberOrEmpty = from.into();
        let to: NumberOrEmpty = to.into();

        let query_string = match (from, to) {
            (NumberOrEmpty::Number(from), NumberOrEmpty::Number(to)) => {
                let from_binding = Binding::new(from);
                let to_binding = Binding::new(to);

                let query_string = format!(
                    "rand::int({}, {})",
                    from_binding.get_param_dollarised(),
                    to_binding.get_param_dollarised()
                );

                bindings = vec![from_binding, to_binding];
                query_string
            }
            _ => format!("rand::int()"),
        };

        Function {
            query_string,
            bindings,
        }
    }

    #[macro_export]
    macro_rules! int {
        () => {
            crate::functions::rand::rand::int_fn(crate::sql::Empty, crate::sql::Empty)
        };
        ( $from:expr, $to:expr ) => {
            crate::functions::rand::rand::int_fn($from, $to)
        };
    }
    pub use int;

    pub fn string_fn(from: impl Into<NumberOrEmpty>, to: impl Into<NumberOrEmpty>) -> Function {
        let mut bindings = vec![];
        let from: NumberOrEmpty = from.into();
        let to: NumberOrEmpty = to.into();

        let query_string = match (from, to) {
            (NumberOrEmpty::Number(length), NumberOrEmpty::Empty) => {
                let length_binding = Binding::new(length);

                let query_string =
                    format!("rand::string({})", length_binding.get_param_dollarised(),);

                bindings = vec![length_binding];
                query_string
            }
            (NumberOrEmpty::Number(from), NumberOrEmpty::Number(to)) => {
                let from_binding = Binding::new(from);
                let to_binding = Binding::new(to);

                let query_string = format!(
                    "rand::string({}, {})",
                    from_binding.get_param_dollarised(),
                    to_binding.get_param_dollarised()
                );

                bindings = vec![from_binding, to_binding];
                query_string
            }
            _ => format!("rand::string()"),
        };

        Function {
            query_string,
            bindings,
        }
    }

    #[macro_export]
    macro_rules! string {
        () => {
            crate::functions::rand::rand::string_fn(crate::sql::Empty, crate::sql::Empty)
        };
        ( $length:expr) => {
            crate::functions::rand::rand::string_fn($from, crate::sql::Empty)
        };
        ( $from:expr, $to:expr ) => {
            crate::functions::rand::rand::string_fn($from, $to)
        };
    }
    pub use string;

    pub fn guid_fn(length: impl Into<NumberOrEmpty>) -> Function {
        let length: NumberOrEmpty = length.into();
        match length {
            NumberOrEmpty::Empty => Function {
                query_string: "rand::guid()".to_string(),
                bindings: vec![],
            },
            NumberOrEmpty::Number(length) => {
                let binding = Binding::new(length);
                let query_string = format!("rand::guid({})", binding.get_param_dollarised());

                Function {
                    query_string,
                    bindings: vec![binding],
                }
            }
        }
    }

    #[macro_export]
    macro_rules! guid {
        () => {
            crate::functions::rand::rand::guid_fn(crate::sql::Empty)
        };
        ( $length:expr ) => {
            crate::functions::rand::rand::guid_fn($length)
        };
    }

    pub use guid;
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

// float
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

#[test]
fn test_rand_float_fn_with_field_inputs() {
    let start = Field::new("start");
    let end = Field::new("end");

    let result = rand::float_fn(start, end);
    assert_eq!(
        result.fine_tune_params(),
        "rand::float($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "rand::float(start, end)");
}

#[test]
fn test_rand_float_macro_with_field_inputs() {
    let start = Field::new("start");
    let end = Field::new("end");

    let result = rand::float!(start, end);
    assert_eq!(
        result.fine_tune_params(),
        "rand::float($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "rand::float(start, end)");
}

// Test Int
#[test]
fn test_rand_int_function_empty() {
    let result = rand::int_fn(Empty, Empty);
    assert_eq!(result.fine_tune_params(), "rand::int()");
    assert_eq!(result.to_raw().to_string(), "rand::int()");
}

#[test]
fn test_rand_int_macro_empty() {
    let result = rand::int!();
    assert_eq!(result.fine_tune_params(), "rand::int()");
    assert_eq!(result.to_raw().to_string(), "rand::int()");
}

#[test]
fn test_rand_int_macro_with_range() {
    let result = rand::int!(34, 65);
    assert_eq!(
        result.fine_tune_params(),
        "rand::int($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "rand::int(34, 65)");
}

#[test]
fn test_rand_int_macro_with_invalid_input() {
    let result = rand::int!(34, "ere");
    assert_eq!(
        result.fine_tune_params(),
        "rand::int($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "rand::int(34, 0)");
}

#[test]
fn test_rand_int_fn_with_field_inputs() {
    let start = Field::new("start");
    let end = Field::new("end");

    let result = rand::int_fn(start, end);
    assert_eq!(
        result.fine_tune_params(),
        "rand::int($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "rand::int(start, end)");
}

#[test]
fn test_rand_int_macro_with_field_inputs() {
    let start = Field::new("start");
    let end = Field::new("end");

    let result = rand::int!(start, end);
    assert_eq!(
        result.fine_tune_params(),
        "rand::int($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "rand::int(start, end)");
}

// Test Guid
#[test]
fn test_rand_guid_function_empty() {
    let result = rand::guid_fn(Empty);
    assert_eq!(result.fine_tune_params(), "rand::guid()");
    assert_eq!(result.to_raw().to_string(), "rand::guid()");
}

#[test]
fn test_rand_guid_macro_empty() {
    let result = rand::guid!();
    assert_eq!(result.fine_tune_params(), "rand::guid()");
    assert_eq!(result.to_raw().to_string(), "rand::guid()");
}

#[test]
fn test_rand_guid_macro_with_range() {
    let result = rand::guid!(34);
    assert_eq!(result.fine_tune_params(), "rand::guid($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "rand::guid(34)");
}

#[test]
fn test_rand_guid_macro_with_invalid_input() {
    let result = rand::guid!("ere");
    assert_eq!(result.fine_tune_params(), "rand::guid($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "rand::guid(0)");
}

#[test]
fn test_rand_guid_fn_with_field_input() {
    let length = Field::new("length");

    let result = rand::guid_fn(length);
    assert_eq!(result.fine_tune_params(), "rand::guid($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "rand::guid(length)");
}

#[test]
fn test_rand_guid_macro_with_field_input() {
    let length = Field::new("length");

    let result = rand::guid!(length);
    assert_eq!(result.fine_tune_params(), "rand::guid($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "rand::guid(length)");
}
