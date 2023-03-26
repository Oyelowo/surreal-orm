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
struct Function(String);

use std::fmt::Display;

use surrealdb::sql;
use surrealdb::sql::Value;

use crate::internal::replace_params;
use crate::sql::{ArrayCustom, Binding, Buildable, ToRawStatement};
use crate::{BindingsList, Field, Parametric};

pub fn val(val: impl Into<Value>) -> sql::Value {
    val.into()
}

#[macro_use]
macro_rules! array {
    ($( $val:expr ),*) => {{
        vec![
            $( val($val) ),*
        ]
    }};
}

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
        // Self(xx)
    }
}

impl<U: Into<sql::Array>> From<U> for ArrayOrField {
    fn from(value: U) -> Self {
        let value: sql::Array = value.into();
        Self::Array(value)
    }
}

#[derive(Debug, Clone)]
pub struct Operatee {
    pub query_string: String,
    pub bindings: BindingsList,
}

// impl ToRawStatement for Operatee {}

impl Parametric for Operatee {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Display for Operatee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Buildable for Operatee {
    fn build(&self) -> String {
        self.query_string.clone()
    }
}

pub fn combine(arr1: impl Into<ArrayCustom>, arr2: impl Into<ArrayCustom>) -> Operatee {
    let arr1: sql::Value = arr1.into().into();
    let arr1 = Binding::new(arr1).with_description("array 1 to be combined");

    let arr2: sql::Value = arr2.into().into();
    let arr2 = Binding::new(arr2).with_description("array 2 to be combined");
    let xx = Operatee {
        query_string: format!(
            "array::combine({}, {})",
            arr1.get_param_dollarised(),
            arr2.get_param_dollarised()
        ),
        bindings: vec![arr1, arr2],
    };
    xx
}

pub fn concat(arr1: impl Into<ArrayCustom>, arr2: impl Into<ArrayCustom>) -> String {
    let arr1: sql::Value = arr1.into().into();
    let arr2: sql::Value = arr2.into().into();
    format!("array::concat({}, {})", arr1, arr2)
}

pub fn union(arr1: impl Into<ArrayCustom>, arr2: impl Into<ArrayCustom>) -> String {
    let arr1: sql::Value = arr1.into().into();
    let arr2: sql::Value = arr2.into().into();
    format!("array::union({}, {})", arr1, arr2)
}

pub fn difference(arr1: impl Into<ArrayCustom>, arr2: impl Into<ArrayCustom>) -> String {
    let arr1: sql::Value = arr1.into().into();
    let arr2: sql::Value = arr2.into().into();
    format!("array::difference({}, {})", arr1, arr2)
}

pub fn distinct(arr1: impl Into<ArrayCustom>) -> String {
    let arr1: sql::Value = arr1.into().into();
    format!("array::distinct({})", arr1)
}

pub fn intersect(arr1: impl Into<ArrayCustom>, arr2: impl Into<ArrayCustom>) -> String {
    let arr1: sql::Value = arr1.into().into();
    let arr2: sql::Value = arr2.into().into();
    format!("array::intersect({}, {})", arr1, arr2)
}

// pub fn len(arr1: Vec<impl Into<sql::Value>>) -> String {
pub fn len(arr1: impl Into<ArrayCustom>) -> String {
    let arr1: sql::Value = arr1.into().into();
    format!("array::len({})", arr1)
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

pub fn sort(arr1: impl Into<ArrayCustom>, ordering: Ordering) -> String {
    let arr1: sql::Value = arr1.into().into();
    match ordering {
        Ordering::Empty => format!("array::sort({arr1})"),
        _ => format!("array::sort({arr1}, {ordering})"),
    }
}

pub mod sort {
    use crate::sql::ArrayCustom;
    use surrealdb::sql;

    pub fn asc(arr1: impl Into<ArrayCustom>) -> String {
        let arr1: sql::Value = arr1.into().into();
        format!("array::sort::asc({arr1})")
    }

    pub fn desc(arr1: impl Into<ArrayCustom>) -> String {
        let arr1: sql::Value = arr1.into().into();
        format!("array::sort::desc({arr1})")
    }
}

#[test]
fn test_array_macro_on_diverse_array() {
    let age = Field::new("age");
    let arr1 = array![1, "Oyelowo", age];
    let arr2 = array![4, "dayo", 6];
    let result = combine(arr1, arr2);
    assert_eq!(
        replace_params(&result.to_string()),
        "array::combine($_param_00000001, $_param_00000002)".to_string()
    );
    assert_eq!(
        result.to_raw().to_string(),
        "array::combine([1, 'Oyelowo', age], [4, 'dayo', 6])".to_string()
    );
}

#[test]
fn test_combine() {
    let arr1 = vec![1, 2, 3];
    let arr2 = vec![4, 5, 6];
    let result = combine(arr1, arr2);
    assert_eq!(
        replace_params(&result.to_string()),
        "array::combine($_param_00000001, $_param_00000002)".to_string()
    );

    assert_eq!(
        result.to_raw().to_string(),
        "array::combine([1, 2, 3], [4, 5, 6])".to_string()
    );
}

#[test]
fn test_concat() {
    let arr1 = vec![1, 2, 3];
    let arr2 = vec![4, 5, 6];
    let result = concat(arr1, arr2);
    assert_eq!(result, "array::concat([1, 2, 3], [4, 5, 6])");
}

#[test]
fn test_union() {
    let arr1 = vec![1, 2, 3];
    let arr2 = vec![4, 5, 6];
    let result = union(arr1, arr2);
    assert_eq!(result, "array::union([1, 2, 3], [4, 5, 6])");
}

#[test]
fn test_difference() {
    let arr1 = vec![1, 2, 3];
    let arr2 = vec![2, 3, 4];
    let result = difference(arr1, arr2);
    assert_eq!(result, "array::difference([1, 2, 3], [2, 3, 4])");
}

#[test]
fn test_distinct() {
    let arr = vec![1, 2, 3, 3, 2, 1];
    let result = distinct(arr);
    assert_eq!(result, "array::distinct([1, 2, 3, 3, 2, 1])");
}

#[test]
fn test_intersect() {
    let arr1 = vec![1, 2, 3];
    let arr2 = vec![2, 3, 4];
    let result = intersect(arr1, arr2);
    assert_eq!(result, "array::intersect([1, 2, 3], [2, 3, 4])");
}

#[test]
fn test_len_on_diverse_array_custom_array_function() {
    let email = Field::new("email");
    let arr = array![1, 2, 3, 4, 5, "4334", "Oyelowo", email];
    let result = len(arr);
    assert_eq!(
        result,
        "array::len([1, 2, 3, 4, 5, '4334', 'Oyelowo', email])"
    );
}

#[test]
fn test_sort() {
    let arr = vec![3, 2, 1];
    let result = sort(arr.clone(), Ordering::Asc);
    assert_eq!(result, "array::sort([3, 2, 1], 'asc')");

    let result = sort(arr.clone(), Ordering::Desc);
    assert_eq!(result, "array::sort([3, 2, 1], 'desc')");

    let result = sort(arr.clone(), Ordering::Empty);
    assert_eq!(result, "array::sort([3, 2, 1])");

    let result = sort(arr.clone(), Ordering::False);
    assert_eq!(result, "array::sort([3, 2, 1], false)");
}

#[test]
fn test_sort_asc() {
    let arr = vec![3, 2, 1];
    let result = sort::asc(arr);
    assert_eq!(result, "array::sort::asc([3, 2, 1])");
}

#[test]
fn test_sort_desc() {
    let arr = vec![3, 2, 1];
    let result = sort::desc(arr);
    assert_eq!(result, "array::sort::desc([3, 2, 1])");
}
