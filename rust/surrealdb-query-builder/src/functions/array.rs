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
use surrealdb::sql::Value;

use crate::internal::replace_params;
use crate::sql::{ArrayCustom, Binding, Buildable, ToRawStatement};
use crate::{array, BindingsList, Field, Parametric};

use super::math::Array;

// pub fn val(val: impl Into<Value>) -> sql::Value {
//     val.into()
// }
//
// #[macro_export]
// macro_rules! array {
//     ($( $val:expr ),*) => {{
//         vec![
//             $( val($val) ),*
//         ]
//     }};
// }

pub enum ArrayOrField {
    Field(Field),
    Array(sql::Array),
}

impl From<Field> for ArrayOrField {
    fn from(value: Field) -> Self {
        Self::Field(value)
    }
}

struct Mana(sql::Value);

impl Mana {
    fn to_array(self) -> sql::Value {
        self.0
    }
}

impl From<ArrayOrField> for Mana {
    fn from(value: ArrayOrField) -> Self {
        match value {
            ArrayOrField::Field(f) => Self(f.into()),
            ArrayOrField::Array(a) => Self(a.into()),
        }
    }
}

impl<U: Into<sql::Array>> From<U> for ArrayOrField {
    fn from(value: U) -> Self {
        let value: sql::Array = value.into();
        Self::Array(value)
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub query_string: String,
    pub bindings: BindingsList,
}

impl Parametric for Function {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Buildable for Function {
    fn build(&self) -> String {
        self.query_string.clone()
    }
}

fn create_array_helper(
    arr1: impl Into<Array>,
    arr2: impl Into<Array>,
    func_name: &str,
) -> Function {
    let arr1: sql::Value = arr1.into().into();
    let arr1 = Binding::new(arr1).with_description("array 1 to be combined");

    let arr2: sql::Value = arr2.into().into();
    let arr2 = Binding::new(arr2).with_description("array 2 to be combined");
    Function {
        query_string: format!(
            "array::{func_name}({}, {})",
            arr1.get_param_dollarised(),
            arr2.get_param_dollarised()
        ),
        bindings: vec![arr1, arr2],
    }
}

macro_rules! create_fn_with_two_array_args {
    ($function_name:expr) => {
        paste::paste! {

            pub fn [<$function_name _fn>](arr1: impl Into<Array>, arr2: impl Into<Array>) -> Function {
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
                let arr1 = array![1, "Oyelowo", age];
                let arr2 = array![4, "dayo", 6];
                let result = crate::functions::array::[<$function_name _fn>](arr1, arr2);
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
                let arr1 = array![1, 2, 3];
                let arr2 = array![4, 5, 6];
                let result = crate::functions::array::[<$function_name _fn>](arr1, arr2);
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
                let arr1 = array![1, "Oyelowo", age];
                let arr2 = array![4, "dayo", 6];
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
                let arr1 = array![1, 2, 3];
                let arr2 = array![4, 5, 6];
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

pub fn distinct(arr: impl Into<ArrayCustom>) -> Function {
    let arr: sql::Value = arr.into().into();
    let arr = Binding::new(arr).with_description("Array to be made distinct");

    Function {
        query_string: format!("array::distinct({})", arr.get_param_dollarised()),
        bindings: vec![arr],
    }
}

// pub fn len(arr1: Vec<impl Into<sql::Value>>) -> String {
pub fn len(arr1: impl Into<ArrayCustom>) -> Function {
    let arr: sql::Value = arr1.into().into();
    let arr =
        Binding::new(arr).with_description("Length of array to be checked. Also checks falsies");

    Function {
        query_string: format!("array::len({})", arr.get_param_dollarised()),
        bindings: vec![arr],
    }
}

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

pub fn sort(arr: impl Into<ArrayCustom>, ordering: Ordering) -> Function {
    let arr: sql::Value = arr.into().into();
    let arr = Binding::new(arr);
    let query_string = match ordering {
        Ordering::Empty => format!("array::sort({})", arr.get_param_dollarised()),
        _ => format!("array::sort({}, {ordering})", arr.get_param_dollarised()),
    };
    Function {
        query_string,
        bindings: vec![arr],
    }
}

pub mod sort {
    use crate::sql::{ArrayCustom, Binding};
    use surrealdb::sql;

    use super::Function;

    pub fn asc(arr: impl Into<ArrayCustom>) -> Function {
        let arr: sql::Value = arr.into().into();
        let arr = Binding::new(arr).with_description("Array to be made distinct");

        Function {
            query_string: format!("array::sort::asc({})", arr.get_param_dollarised()),
            bindings: vec![arr],
        }
    }

    pub fn desc(arr: impl Into<ArrayCustom>) -> Function {
        let arr: sql::Value = arr.into().into();
        let arr = Binding::new(arr).with_description("Array to be made distinct");

        Function {
            query_string: format!("array::sort::desc({})", arr.get_param_dollarised()),
            bindings: vec![arr],
        }
    }
}

// #[test]
// fn test_concat() {
//     let arr1 = array![1, 2, 3];
//     let arr2 = array![4, 5, 6];
//     let result = concat(arr1, arr2);
//     assert_eq!(
//         result.fine_tune_params(),
//         "array::concat($_param_00000001, $_param_00000002)"
//     );
//
//     assert_eq!(
//         result.to_raw().to_string(),
//         "array::concat([1, 2, 3], [4, 5, 6])"
//     );
// }
//
// #[test]
// fn test_union() {
//     let arr1 = array![1, 2, 3];
//     let arr2 = array![4, 5, 6];
//     let result = union(arr1, arr2);
//
//     assert_eq!(
//         result.fine_tune_params(),
//         "array::union($_param_00000001, $_param_00000002)"
//     );
//     assert_eq!(
//         result.to_raw().to_string(),
//         "array::union([1, 2, 3], [4, 5, 6])"
//     );
// }
//
// #[test]
// fn test_difference() {
//     let arr1 = array![1, 2, 3];
//     let arr2 = array![2, 3, 4];
//     let result = difference(arr1, arr2);
//     assert_eq!(
//         result.fine_tune_params(),
//         "array::difference($_param_00000001, $_param_00000002)"
//     );
//     assert_eq!(
//         result.to_raw().to_string(),
//         "array::difference([1, 2, 3], [2, 3, 4])"
//     );
// }
//
// #[test]
// fn test_intersect() {
//     let arr1 = array![1, 2, 3];
//     let arr2 = array![2, 3, 4];
//     let result = intersect(arr1, arr2);
//     assert_eq!(
//         result.fine_tune_params(),
//         "array::intersect($_param_00000001, $_param_00000002)"
//     );
//     assert_eq!(
//         result.to_raw().to_string(),
//         "array::intersect([1, 2, 3], [2, 3, 4])"
//     );
// }

#[test]
fn test_distinct() {
    let arr = array![1, 2, 3, 3, 2, 1];
    let result = distinct(arr);
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
    let arr = array![1, 2, 3, 4, 5, "4334", "Oyelowo", email];
    let result = len(arr);
    assert_eq!(result.fine_tune_params(), "array::len($_param_00000001)");
    assert_eq!(
        result.to_raw().to_string(),
        "array::len([1, 2, 3, 4, 5, '4334', 'Oyelowo', email])"
    );
}

#[test]
fn test_sort() {
    let arr = array![3, 2, 1];
    let result = sort(arr.clone(), Ordering::Asc);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, 'asc')"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1], 'asc')");

    let result = sort(arr.clone(), Ordering::Desc);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, 'desc')"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "array::sort([3, 2, 1], 'desc')"
    );

    let result = sort(arr.clone(), Ordering::Empty);
    assert_eq!(result.fine_tune_params(), "array::sort($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1])");

    let result = sort(arr.clone(), Ordering::False);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort($_param_00000001, false)"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1], false)");
}

#[test]
fn test_sort_asc() {
    let arr = array![3, 2, 1];
    let result = sort::asc(arr);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort::asc($_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort::asc([3, 2, 1])");
}

#[test]
fn test_sort_desc() {
    let arr = array![3, 2, 1];
    let result = sort::desc(arr);
    assert_eq!(
        result.fine_tune_params(),
        "array::sort::desc($_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "array::sort::desc([3, 2, 1])");
}
