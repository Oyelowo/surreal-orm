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

use surrealdb::sql;
use surrealdb::sql::Value;

use crate::sql::ArrayCustom;
use crate::Field;
#[test]
fn erer() {
    let arr1 = &[434, 54];
    let arr1 = vec![434, 54];
    let arr2 = vec!["ksd"];
    let field = Field::new("lowo");
    let arr2 = Field::new("dayo");
    let xx = combine(field, arr2);
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

pub fn combine(arr1: impl Into<ArrayOrField>, arr2: impl Into<ArrayCustom>) -> String {
    let arr1: ArrayOrField = arr1.into();
    let arr1: Mana = arr1.into();
    let arr1 = arr1.to_array();
    let arr2: sql::Value = arr2.into().into();
    format!("array::combine({}, {})", arr1, arr2)
}

pub fn concat(arr1: impl Into<sql::Array>, arr2: impl Into<sql::Array>) -> String {
    let arr1: sql::Array = arr1.into();
    let arr2: sql::Array = arr2.into();
    format!("array::concat({}, {})", arr1, arr2)
}

pub fn union(arr1: impl Into<sql::Array>, arr2: impl Into<sql::Array>) -> String {
    let arr1: sql::Array = arr1.into();
    let arr2: sql::Array = arr2.into();
    format!("array::union({}, {})", arr1, arr2)
}

pub fn difference(arr1: impl Into<sql::Array>, arr2: impl Into<sql::Array>) -> String {
    let arr1: sql::Array = arr1.into();
    let arr2: sql::Array = arr2.into();
    format!("array::difference({}, {})", arr1, arr2)
}

pub fn distinct(arr1: impl Into<sql::Array>) -> String {
    let arr1: sql::Array = arr1.into();
    format!("array::distinct({})", arr1)
}

// pub fn concat<T: Clone>(arr1: &[T], arr2: &[T]) -> Vec<T> {
//     let mut result = arr1.to_vec();
//     result.extend_from_slice(arr2);
//     result
// }
//
// pub fn difference<T: Clone + PartialEq>(arr1: &[T], arr2: &[T]) -> Vec<T> {
//     arr1.iter()
//         .filter(|x| !arr2.contains(x))
//         .chain(arr2.iter().filter(|y| !arr1.contains(y)))
//         .cloned()
//         .collect()
// }
//
// pub fn distinct<T: Clone + PartialEq>(arr: &[T]) -> Vec<T> {
//     arr.iter()
//         .cloned()
//         .collect::<std::collections::HashSet<T>>()
//         .into_iter()
//         .collect()
// }
//
// pub fn intersect<T: Clone + PartialEq>(arr1: &[T], arr2: &[T]) -> Vec<T> {
//     arr1.iter().filter(|x| arr2.contains(x)).cloned().collect()
// }
//
// pub fn len<T>(arr: &[T]) -> usize {
//     arr.len()
// }
//
// pub fn sort<T: Ord + Clone>(arr: &[T], ascending: bool) -> Vec<T> {
//     let mut result = arr.to_vec();
//     result.sort();
//     if !ascending {
//         result.reverse();
//     }
//     result
// }
