use std::fmt::{self, Display};

use surrealdb::sql;

use crate::{
    filter::Filter,
    sql::{ArrayCustom, Binding, Buildable, Empty},
    Field, Parametric,
};

use super::array::Function;

pub enum CountArg {
    Empty,
    Field(Field),
    Filter(Filter),
    Array(ArrayCustom),
}

impl Parametric for CountArg {
    fn get_bindings(&self) -> crate::BindingsList {
        match self {
            CountArg::Empty => vec![],
            CountArg::Field(field) => field.get_bindings(),
            CountArg::Filter(filter) => filter.get_bindings(),
            CountArg::Array(array) => array.get_bindings(),
        }
    }
}

impl From<Empty> for CountArg {
    fn from(value: Empty) -> Self {
        CountArg::Empty
    }
}

// impl From<Field> for CountArg {
//     fn from(value: Field) -> Self {
//         CountArg::Field(value)
//     }
// }

impl From<Filter> for CountArg {
    fn from(value: Filter) -> Self {
        CountArg::Filter(value)
    }
}

impl<T: Into<ArrayCustom>> From<T> for CountArg {
    fn from(value: T) -> Self {
        Self::Array(value.into())
    }
}

impl Buildable for CountArg {
    fn build(&self) -> String {
        match self {
            CountArg::Empty => format!(""),
            CountArg::Field(field) => field.to_string(),
            CountArg::Filter(filter) => filter.to_string(),
            CountArg::Array(array) => array.to_string(),
        }
    }
}

impl Display for CountArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}
// impl From<CountArg> for sql::Value {
//     fn from(value: CountArg) -> Self {
//         match value {
//             CountArg::Empty => sql::Value::from(""),
//             CountArg::Field(field) => field.into(),
//             CountArg::Filter(filter) => filter.to_string().into(),
//             CountArg::Array(array) => array.into(),
//         }
//     }
// }

pub fn count(countable: impl Into<CountArg>) -> Function {
    let countable: CountArg = countable.into();

    Function {
        bindings: countable.get_bindings(),
        query_string: format!("count({})", &countable),
    }
}

use crate::sql::ToRawStatement;

use super::*;

#[test]
fn test_count_withoout_arguments() {
    let result = count(Empty);
    assert_eq!(result.fine_tune_params(), "count()");
    assert_eq!(result.to_raw().to_string(), "count()");
}

#[test]
fn test_count_with_db_field() {
    let email = Field::new("email");
    let result = count(email);
    assert_eq!(result.fine_tune_params(), "count(email)");
    assert_eq!(result.to_raw().to_string(), "count(email)");
}
