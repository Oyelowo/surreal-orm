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

use crate::sql::ArrayCustom;
use crate::Field;

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
#[test]
fn erer() {
    let arr1 = &[434, 54];
    // let arr1 = vec![434, 54];
    let arr2 = vec!["ksd", "dayo"];
    // let arr2 = vec![434, 54];
    let arr1 = Field::new("lowo");
    // let arr2 = Field::new("dayo");
    // let xx = combine(arr1, arr2);
    // assert_eq!(xx, "nawa".to_string());

    // let xx = len(vec![val(34), val("34"), val(arr1)]);
    // let xx = len(vec![54, 65]);
    let xx = len(array![34, "34", arr1]);
    assert_eq!(xx, "nawa".to_string());
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

pub fn combine(arr1: impl Into<ArrayCustom>, arr2: impl Into<ArrayCustom>) -> String {
    // let arr1: ArrayOrField = arr1.into();
    // let arr1: Mana = arr1.into();
    // let arr1 = arr1.to_array();
    let arr1: sql::Value = arr1.into().into();
    let arr2: sql::Value = arr2.into().into();
    format!("array::combine({}, {})", arr1, arr2)
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
    format!("array::sort({arr1}), {ordering}")
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
        format!("array::sort::asc({arr1})")
    }
}
