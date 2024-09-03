/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::Display;

use crate::{BindingsList, Buildable, Erroneous, Parametric, ValueLike};

/// Represents a cast function.
pub struct Cast(ValueLike);

impl std::ops::Deref for Cast {
    type Target = ValueLike;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parametric for Cast {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

impl Display for Cast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Buildable for Cast {
    fn build(&self) -> String {
        self.0.string.to_string()
    }
}

impl Erroneous for Cast {}

macro_rules! create_cast_function {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name>](value: impl Into<ValueLike>) -> Cast {
                let value: ValueLike = value.into();
                let string = format!("<{}> {}", $function_name, value.build());

                Cast(ValueLike {
                    string,
                    bindings: value.get_bindings(),
                    errors: value.get_errors(),
                })
            }

            #[cfg(test)]
            mod [<test_ $function_name>] {
                use super::*;
                use crate::*;

                #[test]
                fn [<test_ $function_name _cast_function >] () {
                    let value = 39;
                    let casted_value = [<$function_name>](value);
                    assert_eq!(casted_value.fine_tune_params(), format!("<{}> $_param_00000001", $function_name));
                    assert_eq!(casted_value.to_raw().build(), format!("<{}> {}", $function_name, value));
                }

                #[test]
                fn [<test_ $function_name _cast_function_with_field >] () {
                    let value = $crate::Field::new("test");
                    let casted_value = [<$function_name>](value.clone());
                    assert_eq!(casted_value.fine_tune_params(), format!("<{}> test", $function_name));
                    assert_eq!(casted_value.to_raw().build(), format!("<{}> test", $function_name));
                }

                #[test]
                fn [<test_ $function_name _cast_function_with_param >] () {
                    let value = $crate::Param::new("test");
                    let casted_value = [<$function_name>](value.clone());
                    assert_eq!(casted_value.fine_tune_params(), format!("<{}> $test", $function_name));
                    assert_eq!(casted_value.to_raw().build(), format!("<{}> $test", $function_name));
                }
            }
        }
    };
}

create_cast_function!(
    /// Casts a value to a boolean.
    => "bool"
);

create_cast_function!(
    /// Casts a value to a datetime.
    => "datetime"
);

create_cast_function!(
    /// Casts a value to a decimal.
    => "decimal"
);

create_cast_function!(
    /// Casts a value to a duration.
    => "duration"
);

create_cast_function!(
    /// Casts a value to a float.
    => "float"
);

create_cast_function!(
    /// Casts a value to an int.
    => "int"
);

create_cast_function!(
    /// Casts a value to a number.
    => "number"
);

create_cast_function!(
    /// Casts a value to a string.
    => "string"
);

// create_cast_function!(
//     /// Casts a value to a future.
//     => "future"
// );
