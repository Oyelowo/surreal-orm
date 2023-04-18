/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// array::combine()	Combines all values from two arrays together
// array::concat()	Returns the merged values from two arrays
// array::difference()	Returns the difference between two arrays
// array::distinct()	Returns the unique items in an array
// array::intersect()	Returns the values which intersect two arrays
// array::len()	Returns the length of an array
// array::sort()	Sorts the values in an array in ascending or descending order
// array::sort::asc()	Sorts the values in an array in ascending order
// array::sort::desc()	Sorts the values in an array in descending order
// array::union()
// struct Function(String);

use std::fmt::Display;

use surrealdb::sql;

use crate::traits::{Binding, BindingsList, Buildable, Parametric, ToRaw};
use crate::types::{ArrayLike, Field, Function};
use crate::{arr, array};

fn create_array_helper(
    arr1: impl Into<ArrayLike>,
    arr2: impl Into<ArrayLike>,
    func_name: &str,
) -> Function {
    let arr1: ArrayLike = arr1.into();
    let arr2: ArrayLike = arr2.into();
    let mut bindings = vec![];
    bindings.extend(arr1.get_bindings());
    bindings.extend(arr2.get_bindings());
    Function {
        query_string: format!("array::{func_name}({}, {})", arr1.build(), arr2.build()),
        bindings,
    }
}

macro_rules! create_fn_with_two_array_args {
    ($function_name:expr) => {
        paste::paste! {
            pub fn [<$function_name _fn>](arr1: impl Into<$crate::ArrayLike>, arr2: impl Into<$crate::ArrayLike>) -> Function {
                create_array_helper(arr1, arr2, $function_name)
            }

            #[macro_export]
            macro_rules! [<array_ $function_name>] {
                ( $arr1:expr, $arr2:expr ) => {
                    crate::functions::array::[<$function_name _fn>]($arr1, $arr2)
                };
            }
            pub use [<array_ $function_name>] as [<$function_name>];

            #[test]
            fn [<test $function_name fn_on_array_macro_on_diverse_array>]() {
                let age = Field::new("age");
                let arr1 = $crate::arr![1, "Oyelowo", age];
                let arr2 = $crate::arr![4, "dayo", 6];
                let result = $crate::functions::array::[<$function_name _fn>](arr1, arr2);
                assert_eq!(
                    result.fine_tune_params(),
                    format!("array::{}($_param_00000001, $_param_00000002)", $function_name)
                );
                assert_eq!(
                    result.to_raw().to_string(),
                    format!("array::{}([1, 'Oyelowo', age], [4, 'dayo', 6])", $function_name)
                );
            }

            #[test]
            fn [<test $function_name _fn_on_same_element_types>]() {
                let arr1 = $crate::arr![1, 2, 3];
                let arr2 = $crate::arr![4, 5, 6];
                let result = $crate::functions::array::[<$function_name _fn>](arr1, arr2);
                assert_eq!(
                    result.fine_tune_params(),
                    format!("array::{}($_param_00000001, $_param_00000002)", $function_name)
                );

                assert_eq!(
                    result.to_raw().to_string(),
                    format!("array::{}([1, 2, 3], [4, 5, 6])", $function_name)
                );
            }

            #[test]
            fn [<test $function_name _macro_on_array_macro_on_diverse_array>]() {
                let age = Field::new("age");
                let arr1 = $crate::arr![1, "Oyelowo", age];
                let arr2 = $crate::arr![4, "dayo", 6];
                let result = crate::functions::array::[<$function_name>]!(arr1, arr2);
                assert_eq!(
                    result.fine_tune_params(),
                    format!("array::{}($_param_00000001, $_param_00000002)", $function_name)
                );
                assert_eq!(
                    result.to_raw().to_string(),
                    format!("array::{}([1, 'Oyelowo', age], [4, 'dayo', 6])", $function_name)
                );
            }

            #[test]
            fn [<test $function_name _macro_on_same_element_types>]() {
                let arr1 = $crate::arr![1, 2, 3];
                let arr2 = $crate::arr![4, 5, 6];
                let result = crate::functions::array::[<$function_name>]!(arr1, arr2);
                assert_eq!(
                    result.fine_tune_params(),
                    format!("array::{}($_param_00000001, $_param_00000002)", $function_name)
                );

                assert_eq!(
                    result.to_raw().to_string(),
                    format!("array::{}([1, 2, 3], [4, 5, 6])", $function_name)
                );
            }

            #[test]
            fn [<test $function_name _macro_on_fields>]() {
                let students_ages = Field::new("students_ages");
                let teachers_ages = Field::new("teachers_ages");
                let result = crate::functions::array::[<$function_name>]!(students_ages, teachers_ages);
                assert_eq!(
                    result.fine_tune_params(),
                    format!("array::{}($_param_00000001, $_param_00000002)", $function_name)
                );

                assert_eq!(
                    result.to_raw().to_string(),
                    format!("array::{}(students_ages, teachers_ages)", $function_name)
                );
            }
        }
    };
}

create_fn_with_two_array_args!("combine");
create_fn_with_two_array_args!("concat");
create_fn_with_two_array_args!("union");
create_fn_with_two_array_args!("difference");
create_fn_with_two_array_args!("intersect");

pub fn distinct_fn(arr: impl Into<ArrayLike>) -> Function {
    let arr: ArrayLike = arr.into();

    Function {
        query_string: format!("array::distinct({})", arr.build()),
        bindings: arr.get_bindings(),
    }
}

#[macro_export]
macro_rules! array_distinct_fn {
    ( $arr:expr ) => {
        crate::functions::array::distinct_fn($arr)
    };
}
pub use array_distinct_fn as distinct;

pub fn len_fn(arr: impl Into<ArrayLike>) -> Function {
    let arr: ArrayLike = arr.into();

    Function {
        query_string: format!("array::len({})", arr.build()),
        bindings: arr.get_bindings(),
    }
}

#[macro_export]
macro_rules! array_len_fn {
    ( $arr:expr ) => {
        crate::functions::array::len_fn($arr)
    };
}
pub use array_len_fn as len;

pub enum Ordering {
    Asc,
    Desc,
    False,
    Empty,
}

impl Display for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ordering::Asc => "'asc'",
                Ordering::Desc => "'desc'",
                Ordering::False => "false",
                Ordering::Empty => "",
            }
        )
    }
}

pub fn sort_fn(arr: impl Into<ArrayLike>, ordering: Ordering) -> Function {
    let arr: ArrayLike = arr.into();
    let query_string = match ordering {
        Ordering::Empty => format!("array::sort({})", arr.build()),
        _ => format!("array::sort({}, {ordering})", arr.build()),
    };
    Function {
        query_string,
        bindings: arr.get_bindings(),
    }
}

#[macro_export]
macro_rules! array_sort {
    ( $arr:expr, "asc" ) => {
        crate::functions::array::sort_fn($arr, crate::functions::array::Ordering::Asc)
    };
    ( $arr:expr, "desc" ) => {
        crate::functions::array::sort_fn($arr, crate::functions::array::Ordering::Desc)
    };
    ( $arr:expr, false ) => {
        crate::functions::array::sort_fn($arr, crate::functions::array::Ordering::False)
    };
    ( $arr:expr, $ordering:expr ) => {
        crate::functions::array::sort_fn($arr, $ordering)
    };
    ( $arr:expr ) => {
        crate::functions::array::sort_fn($arr, crate::functions::array::Ordering::Empty)
    };
}
pub use array_sort as sort;

pub mod sort {
    use surrealdb::sql;

    use crate::{traits::Binding, types::ArrayLike, Buildable, Parametric};

    use super::Function;

    pub fn asc_fn(arr: impl Into<ArrayLike>) -> Function {
        let arr: ArrayLike = arr.into();

        Function {
            query_string: format!("array::sort::asc({})", arr.build()),
            bindings: arr.get_bindings(),
        }
    }

    #[macro_export]
    macro_rules! array_sort_asc_fn {
        ( $arr:expr ) => {
            crate::functions::array::sort::asc_fn($arr)
        };
    }
    pub use array_sort_asc_fn as asc;

    pub fn desc_fn(arr: impl Into<ArrayLike>) -> Function {
        let arr: ArrayLike = arr.into();

        Function {
            query_string: format!("array::sort::desc({})", arr.build()),
            bindings: arr.get_bindings(),
        }
    }

    #[macro_export]
    macro_rules! array_sort_desc_fn {
        ( $arr:expr ) => {
            crate::functions::array::sort::desc_fn($arr)
        };
    }
    pub use array_sort_desc_fn as desc;
}

#[test]
fn test_distinct() {
    let arr = arr![1, 2, 3, 3, 2, 1];
    let result = distinct_fn(arr);
    assert_eq!(
        result.fine_tune_params(),
        "array::distinct($_param_00000001)".to_string()
    );
    assert_eq!(
        result.to_raw().to_string(),
        "array::distinct([1, 2, 3, 3, 2, 1])"
    );
}

#[test]
fn test_distinct_macro() {
    let arr = arr![1, 2, 3, 3, 2, 1];
    let result = distinct!(arr);
    assert_eq!(
        result.fine_tune_params(),
        "array::distinct($_param_00000001)".to_string()
    );
    assert_eq!(
        result.to_raw().to_string(),
        "array::distinct([1, 2, 3, 3, 2, 1])"
    );
}

#[test]
fn test_len_on_diverse_array_custom_array_function() {
    let email = Field::new("email");
    let arr = arr![1, 2, 3, 4, 5, "4334", "Oyelowo", email];
    let result = len_fn(arr);
    assert_eq!(result.fine_tune_params(), "array::len($_param_00000001)");
    assert_eq!(
        result.to_raw().to_string(),
        "array::len([1, 2, 3, 4, 5, '4334', 'Oyelowo', email])"
    );
}

#[test]
fn test_len_macro_on_diverse_array_custom_array_function() {
    let email = Field::new("email");
    let arr = arr![1, 2, 3, 4, 5, "4334", "Oyelowo", email];
    let result = len!(arr);
    assert_eq!(result.fine_tune_params(), "array::len($_param_00000001)");
    assert_eq!(
        result.to_raw().to_string(),
        "array::len([1, 2, 3, 4, 5, '4334', 'Oyelowo', email])"
    );
}

#[test]
fn test_sort() {
    let arr = arr![3, 2, 1];
    let result = sort_fn(arr.clone(), Ordering::Asc);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, 'asc')"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1], 'asc')");

    let result = sort_fn(arr.clone(), Ordering::Desc);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, 'desc')"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "array::sort([3, 2, 1], 'desc')"
    );

    let result = sort_fn(arr.clone(), Ordering::Empty);
    assert_eq!(result.fine_tune_params(), "array::sort($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1])");

    let result = sort_fn(arr.clone(), Ordering::False);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, false)"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1], false)");
}

#[test]
fn test_sort_macro_ordering_type() {
    let arr = arr![3, 2, 1];
    let result = sort!(arr.clone(), Ordering::Asc);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, 'asc')"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1], 'asc')");

    let result = sort!(arr.clone(), Ordering::Desc);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, 'desc')"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "array::sort([3, 2, 1], 'desc')"
    );

    let result = sort!(arr.clone(), Ordering::Empty);
    assert_eq!(result.fine_tune_params(), "array::sort($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1])");

    let result = sort!(arr.clone(), Ordering::False);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, false)"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1], false)");
}

#[test]
fn test_sort_macro() {
    let arr = arr![3, 2, 1];
    let result = sort!(arr.clone(), "asc");
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, 'asc')"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1], 'asc')");

    let result = sort!(arr.clone(), "desc");
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, 'desc')"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "array::sort([3, 2, 1], 'desc')"
    );

    // Without ordering
    let result = sort!(arr.clone());
    assert_eq!(result.fine_tune_params(), "array::sort($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1])");

    let result = sort!(arr.clone(), false);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, false)"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1], false)");
}

#[test]
fn test_sort_asc() {
    let arr = arr![3, 2, 1];
    let result = sort::asc_fn(arr);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort::asc($_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort::asc([3, 2, 1])");
}

#[test]
fn test_sort_asc_macro() {
    let arr = arr![3, 2, 1];
    let result = sort::asc!(arr);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort::asc($_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort::asc([3, 2, 1])");
}

#[test]
fn test_sort_desc() {
    let arr = arr![3, 2, 1];
    let result = sort::desc_fn(arr);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort::desc($_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort::desc([3, 2, 1])");
}

#[test]
fn test_sort_desc_macro() {
    let arr = arr![3, 2, 1];
    let result = sort::desc!(arr);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort::desc($_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort::desc([3, 2, 1])");
}
