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

use crate::array;
use crate::traits::{Binding, Buildable, ToRaw};
use crate::types::{Field, Function, NumberLike};

pub fn rand_fn() -> Function {
    let query_string = format!("rand()");

    Function {
        query_string,
        bindings: vec![],
    }
}

#[macro_export]
macro_rules! rand_rand {
    () => {
        crate::functions::rand::rand_fn()
    };
}

pub use rand_rand as rand;

struct NumEmpty(sql::Value);

pub(crate) fn create_fn_with_single_num_arg(
    number: impl Into<NumberLike>,
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
    use crate::{
        traits::Binding,
        types::{Function, NumberLike},
    };

    use super::create_fn_with_single_num_arg;
    use surrealdb::sql;

    pub fn bool_fn() -> Function {
        let query_string = format!("rand::bool()");

        Function {
            query_string,
            bindings: vec![],
        }
    }

    #[macro_export]
    macro_rules! rand_bool {
        () => {
            crate::functions::rand::rand::bool_fn()
        };
    }

    pub use rand_bool as bool;

    pub fn uuid_fn() -> Function {
        let query_string = format!("rand::uuid()");

        Function {
            query_string,
            bindings: vec![],
        }
    }

    #[macro_export]
    macro_rules! rand_uuid {
        () => {
            crate::functions::rand::rand::uuid_fn()
        };
    }

    pub use rand_uuid as uuid;

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
    macro_rules! rand_enum {
        ( $val:expr ) => {
            crate::functions::rand::rand::enum_fn( $val )
        };
        ($( $val:expr ),*) => {
            crate::functions::rand::rand::enum_fn(crate::array![ $( $val ), * ])
        };
    }

    pub use rand_enum as enum_;

    pub fn float_fn(
        from: Option<impl Into<NumberLike>>,
        to: Option<impl Into<NumberLike>>,
    ) -> Function {
        let mut bindings = vec![];

        let query_string = match (from, to) {
            (Some(from), Some(to)) => {
                let from_binding = Binding::new(from.into());
                let to_binding = Binding::new(to.into());

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
    macro_rules! rand_float {
        () => {
            crate::functions::rand::rand::float_fn(
                None as Option<NumberLike>,
                None as Option<NumberLike>,
            )
        };
        ( $from:expr, $to:expr ) => {
            crate::functions::rand::rand::float_fn(Some($from), Some($to))
        };
    }

    pub use rand_float as float;

    pub fn int_fn(
        from: Option<impl Into<NumberLike>>,
        to: Option<impl Into<NumberLike>>,
    ) -> Function {
        let mut bindings = vec![];

        let query_string = match (from, to) {
            (Some(from), Some(to)) => {
                let from_binding = Binding::new(from.into());
                let to_binding = Binding::new(to.into());

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
    macro_rules! rand_int {
        () => {
            crate::functions::rand::rand::int_fn(
                None as Option<NumberLike>,
                None as Option<NumberLike>,
            )
        };
        ( $from:expr, $to:expr ) => {
            crate::functions::rand::rand::int_fn(Some($from), Some($to))
        };
    }
    pub use rand_int as int;

    pub fn time_fn(
        from: Option<impl Into<NumberLike>>,
        to: Option<impl Into<NumberLike>>,
    ) -> Function {
        let mut bindings = vec![];

        let query_string = match (from, to) {
            (Some(from), Some(to)) => {
                let from_binding = Binding::new(from.into());
                let to_binding = Binding::new(to.into());

                let query_string = format!(
                    "rand::time({}, {})",
                    from_binding.get_param_dollarised(),
                    to_binding.get_param_dollarised()
                );

                bindings = vec![from_binding, to_binding];
                query_string
            }
            _ => format!("rand::time()"),
        };

        Function {
            query_string,
            bindings,
        }
    }

    #[macro_export]
    macro_rules! rand_time {
        () => {
            crate::functions::rand::rand::time_fn(
                None as Option<NumberLike>,
                None as Option<NumberLike>,
            )
        };
        ( $from:expr, $to:expr ) => {
            crate::functions::rand::rand::time_fn(Some($from), Some($to))
        };
    }
    pub use rand_time as time;

    pub fn string_fn(
        from: Option<impl Into<NumberLike>>,
        to: Option<impl Into<NumberLike>>,
    ) -> Function {
        let mut bindings = vec![];

        let query_string = match (from, to) {
            (Some(length), None) => {
                let length_binding = Binding::new(length.into());

                let query_string =
                    format!("rand::string({})", length_binding.get_param_dollarised(),);

                bindings = vec![length_binding];
                query_string
            }
            (Some(from), Some(to)) => {
                let from_binding = Binding::new(from.into());
                let to_binding = Binding::new(to.into());

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
    macro_rules! rand_string {
        () => {
            crate::functions::rand::rand::string_fn(
                None as Option<NumberLike>,
                None as Option<NumberLike>,
            )
        };
        ( $length:expr) => {
            crate::functions::rand::rand::string_fn(Some($length), None as Option<NumberLike>)
        };
        ( $from:expr, $to:expr ) => {
            crate::functions::rand::rand::string_fn(Some($from), Some($to))
        };
    }
    pub use rand_string as string;

    pub fn guid_fn(length: Option<impl Into<NumberLike>>) -> Function {
        match length {
            None => Function {
                query_string: "rand::guid()".to_string(),
                bindings: vec![],
            },
            Some(length) => {
                let binding = Binding::new(length.into());
                let query_string = format!("rand::guid({})", binding.get_param_dollarised());

                Function {
                    query_string,
                    bindings: vec![binding],
                }
            }
        }
    }

    #[macro_export]
    macro_rules! rand_guid {
        () => {
            crate::functions::rand::rand::guid_fn(None as Option<NumberLike>)
        };
        ( $length:expr ) => {
            crate::functions::rand::rand::guid_fn(Some($length))
        };
    }

    pub use rand_guid as guid;
}

#[test]
fn test_rand_fn() {
    let result = rand_fn();
    assert_eq!(result.fine_tune_params(), "rand()");
    assert_eq!(result.to_raw().to_string(), "rand()");
}

#[test]
fn test_rand_macro() {
    let result = rand_rand!();
    assert_eq!(result.fine_tune_params(), "rand()");
    assert_eq!(result.to_raw().to_string(), "rand()");
}

#[test]
fn test_rand_bool_fn() {
    let result = rand::bool_fn();
    assert_eq!(result.fine_tune_params(), "rand::bool()");
    assert_eq!(result.to_raw().to_string(), "rand::bool()");
}

#[test]
fn test_rand_bool_macro() {
    let result = rand::bool!();
    assert_eq!(result.fine_tune_params(), "rand::bool()");
    assert_eq!(result.to_raw().to_string(), "rand::bool()");
}

#[test]
fn test_rand_uuid_fn() {
    let result = rand::uuid_fn();
    assert_eq!(result.fine_tune_params(), "rand::uuid()");
    assert_eq!(result.to_raw().to_string(), "rand::uuid()");
}

#[test]
fn test_rand_uuid() {
    let result = rand::uuid!();
    assert_eq!(result.fine_tune_params(), "rand::uuid()");
    assert_eq!(result.to_raw().to_string(), "rand::uuid()");
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

macro_rules! create_test_for_fn_with_two_args {
    ($function_ident: expr) => {
        paste::paste! {
                #[test]
                fn [<test_rand_ $function_ident _function_empty>]() {
                    let result = rand::[< $function_ident _fn>](None as Option<NumberLike>, None as Option<NumberLike>);
                    assert_eq!(result.fine_tune_params(), format!("rand::{}()", $function_ident));
                    assert_eq!(result.to_raw().to_string(), format!("rand::{}()", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident _macro_empty>]() {
                    let result = rand::[< $function_ident>]!();
                    assert_eq!(result.fine_tune_params(), format!("rand::{}()", $function_ident));
                    assert_eq!(result.to_raw().to_string(), format!("rand::{}()", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident _macro_with_range>]() {
                    let result = rand::[< $function_ident>]!(34, 65);
                    assert_eq!(result.fine_tune_params(), format!("rand::{}($_param_00000001, $_param_00000002)", $function_ident));
                    assert_eq!(result.to_raw().to_string(), format!("rand::{}(34, 65)", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident macro_with_invalid_input>]() {
                    let result = rand::[< $function_ident>]!(34, "ere");
                    assert_eq!(result.fine_tune_params(), format!("rand::{}($_param_00000001, $_param_00000002)", $function_ident));
                    assert_eq!(result.to_raw().to_string(), format!("rand::{}(34, 0)", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident fn_with_field_inputs>]() {
                    let start = Field::new("start");
                    let end = Field::new("end");

                    let result = rand::[< $function_ident _fn>](Some(start), Some(end));
                    assert_eq!(result.fine_tune_params(), format!("rand::{}($_param_00000001, $_param_00000002)", $function_ident));
                    assert_eq!(result.to_raw().to_string(), format!("rand::{}(start, end)", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident macro_with_field_inputs>]() {
                    let start = Field::new("start");
                    let end = Field::new("end");

                    let result = rand::[< $function_ident>]!(start, end);
                    assert_eq!(result.fine_tune_params(), format!("rand::{}($_param_00000001, $_param_00000002)", $function_ident));
                    assert_eq!(result.to_raw().to_string(), format!("rand::{}(start, end)", $function_ident));
                }
            }
    };
}
create_test_for_fn_with_two_args!("float");
create_test_for_fn_with_two_args!("int");
create_test_for_fn_with_two_args!("string");
create_test_for_fn_with_two_args!("time");

#[test]
fn test_rand_string_macro_with_one_arg_length() {
    let result = rand::string!(34);
    assert_eq!(result.fine_tune_params(), "rand::string($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "rand::string(34)");
}

#[test]
fn test_rand_string_macro_with_one_arg_field() {
    let length_of_name = Field::new("length_of_name");
    let result = rand::string!(length_of_name);
    assert_eq!(result.fine_tune_params(), "rand::string($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "rand::string(length_of_name)");
}

// Test Guid
#[test]
fn test_rand_guid_function_empty() {
    let result = rand::guid_fn(None as Option<NumberLike>);
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

    let result = rand::guid_fn(Some(length));
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
