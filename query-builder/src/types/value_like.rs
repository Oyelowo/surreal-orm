/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{
    statements::{
        CreateStatement, DeleteStatement, IfElseStatement, InsertStatement, LetStatement,
        RelateStatement, SelectStatement, Subquery, UpdateStatement,
    },
    Alias, All, Binding, BindingsList, Buildable, Edge, Erroneous, ErrorList, Field, Filter,
    Function, Model, Node, Operation, Param, Parametric, E, NONE, NULL,
};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

/// A value that can be used in a SQL statement. Serves as the bind and arbiter between
/// `sql::Value` and the query building world.
#[derive(Debug, Clone)]
pub struct ValueLike {
    pub(crate) string: String,
    pub(crate) bindings: BindingsList,
    pub(crate) errors: ErrorList,
}

impl ValueLike {
    /// Create a new `ValueLike` from other values.
    pub fn new(value: impl Buildable + Parametric + Erroneous) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

impl Parametric for ValueLike {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Parametric for Vec<ValueLike> {
    fn get_bindings(&self) -> BindingsList {
        self.iter()
            .flat_map(|m| m.get_bindings())
            .collect::<Vec<_>>()
    }
}

impl Erroneous for ValueLike {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl Erroneous for Vec<ValueLike> {
    fn get_errors(&self) -> ErrorList {
        self.iter().flat_map(|m| m.get_errors()).collect::<Vec<_>>()
    }
}

impl Buildable for ValueLike {
    fn build(&self) -> String {
        self.string.to_string()
    }
}

impl Buildable for Vec<ValueLike> {
    fn build(&self) -> String {
        self.iter()
            .map(|m| m.build())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl From<&Field> for ValueLike {
    fn from(value: &Field) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

impl From<Field> for ValueLike {
    fn from(value: Field) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

impl From<Param> for ValueLike {
    fn from(value: Param) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

impl From<&Param> for ValueLike {
    fn from(value: &Param) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

impl From<LetStatement> for ValueLike {
    fn from(value: LetStatement) -> Self {
        Self {
            string: value.get_param().build(),
            bindings: vec![],
            errors: vec![],
        }
    }
}

impl From<&LetStatement> for ValueLike {
    fn from(value: &LetStatement) -> Self {
        Self {
            string: value.get_param().build(),
            bindings: vec![],
            errors: vec![],
        }
    }
}

impl From<Alias> for ValueLike {
    fn from(value: Alias) -> Self {
        ValueLike {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

impl From<All> for ValueLike {
    fn from(_value: All) -> Self {
        ValueLike {
            string: "*".to_string(),
            bindings: vec![],
            errors: vec![],
        }
    }
}

impl From<NULL> for ValueLike {
    fn from(_value: NULL) -> Self {
        ValueLike {
            string: "NULL".to_string(),
            bindings: vec![],
            errors: vec![],
        }
    }
}

impl From<E> for ValueLike {
    fn from(_value: E) -> Self {
        ValueLike {
            string: "".to_string(),
            bindings: vec![],
            errors: vec![],
        }
    }
}

impl From<NONE> for ValueLike {
    fn from(_value: NONE) -> Self {
        ValueLike {
            string: "NONE".to_string(),
            bindings: vec![],
            errors: vec![],
        }
    }
}

impl From<Filter> for ValueLike {
    fn from(value: Filter) -> Self {
        ValueLike {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

impl From<Operation> for ValueLike {
    fn from(value: Operation) -> Self {
        ValueLike {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

impl From<Function> for ValueLike {
    fn from(value: Function) -> Self {
        ValueLike {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

// impl<T: Into<sql::Value>> From<T> for ValueLike {
//     fn from(value: T) -> Self {
//         let value: sql::Value = value.into();
//         let binding = Binding::new(value);
//
//         ValueLike {
//             string: binding.get_param_dollarised(),
//             bindings: vec![binding],
//             errors: vec![],
//         }
//     }
// }

// You can deref into the value
// T<Derefed into U>, T can be converted to sql::Value, and I want anything that can be derefed to
// U converted to ValueLike
// impl<T, U> From<T> for ValueLike
// where
//     T: std::ops::Deref<Target = U>,
//     U: Into<sql::Value>,
// {
//     fn from(value: T) -> Self {
//         let value: U = value.into();
//         let binding = Binding::new(value.into());
//
//         ValueLike {
//             string: binding.get_param_dollarised(),
//             bindings: vec![binding],
//             errors: vec![],
//         }
//     }
// }
//
// impl<T> From<T> for ValueLike
// where
//     T: Deref,
//     T::Target: Into<sql::Value> + Clone,
// {
//     fn from(value: T) -> Self {
//         let derefed_value: T::Target = value.clone();
//         let value: sql::Value = derefed_value.into();
//         let binding = Binding::new(value);
//
//         ValueLike {
//             string: binding.get_param_dollarised(),
//             bindings: vec![binding],
//             errors: vec![],
//         }
//     }
// }
//
//
//
// Implement From for types that can be directly converted into sql::Value
impl<T> From<T> for ValueLike
where
    T: Into<sql::Value>,
{
    fn from(value: T) -> Self {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);

        ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        }
    }
}

impl<T, U> From<U> for ValueLike
where
    T: Deref<Target = U>,
    T: Into<sql::Value>,
    U: Into<T>,
    T: Into<U>
    // U: Into<sql::Value>,
{
    fn from(value: U) -> Self {
        let value: T = value.into();
        let value: sql::Value = value.into();
        let binding = Binding::new(value);

        ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        }
    }
}

// Implement From for types that can be dereferenced into types that can be converted into sql::Value
impl<T, U> From<T> for ValueLike
where
    T: Deref<Target = U>,
    U: Into<sql::Value>,
{
    fn from(value: T) -> Self {
        let value: sql::Value = (*value).into();
        let binding = Binding::new(value);

        ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        }
    }
}

fn statement_to_value_like<T>(statement: T) -> ValueLike
where
    T: Into<Subquery>,
{
    let subquery: Subquery = statement.into();

    ValueLike {
        string: subquery.build(),
        bindings: subquery.get_bindings(),
        errors: subquery.get_errors(),
    }
}

impl From<SelectStatement> for ValueLike {
    fn from(statement: SelectStatement) -> Self {
        statement_to_value_like(statement)
    }
}

impl<T> From<CreateStatement<T>> for ValueLike
where
    T: Node + Serialize + DeserializeOwned,
{
    fn from(statement: CreateStatement<T>) -> Self {
        statement_to_value_like(statement)
    }
}

impl<T> From<UpdateStatement<T>> for ValueLike
where
    T: Model + Serialize + DeserializeOwned,
{
    fn from(statement: UpdateStatement<T>) -> Self {
        statement_to_value_like(statement)
    }
}

impl<T> From<DeleteStatement<T>> for ValueLike
where
    T: Model + Serialize + DeserializeOwned,
{
    fn from(statement: DeleteStatement<T>) -> Self {
        statement_to_value_like(statement)
    }
}

impl<T> From<RelateStatement<T>> for ValueLike
where
    T: Edge + Serialize + DeserializeOwned,
{
    fn from(statement: RelateStatement<T>) -> Self {
        statement_to_value_like(statement)
    }
}

impl<T> From<InsertStatement<T>> for ValueLike
where
    T: Node + Serialize + DeserializeOwned,
{
    fn from(statement: InsertStatement<T>) -> Self {
        statement_to_value_like(statement)
    }
}

impl From<IfElseStatement> for ValueLike {
    fn from(statement: IfElseStatement) -> Self {
        statement_to_value_like(statement)
    }
}

/// A macro to create a heterogenous list of anything
/// that can be converted into `ValueLike` including all
/// supported surrealdb types like numbers,
/// dates, field, param, geometry etc, and statements.
/// It creates Vec<ValueLike>` from a list of values.
///
/// # Arguments
/// * `$( $val:expr ),*` - A list of values that can be converted into `ValueLike`
/// # Example
/// ```rust
/// use surrealdb::sql;
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::{math, count}};
/// let country = Field::new("country");
/// let age = Field::new("age");
/// let total = AliasName::new("total");
///
/// let values = arr![
///         1,
///         2,
///         3,
///         count!().__as__(total),
///         math::sum!(age),
///         country,
///         54,
///         sql::Duration(std::time::Duration::from_secs(43))
///     ];
///     
///    assert_eq!(
///         values.into_iter()
///             .map(|v| v.to_raw().build())
///             .collect::<Vec<_>>()
///             .join(", "),
///         "1, 2, 3, count() AS total, math::sum(age), country, 54, 43s"
///    );
///    ```
#[macro_export]
macro_rules! arr {
    ($( $val:expr ),*) => {{
        vec![
            $( $crate::ValueLike::from($val) ),*
        ]
    }};
}

// impl<T: Into<Field>> From<T> for ValueLike {
//     fn from(value: T) -> Self {
//         let value: Field = value.into();
//         let binding = Binding::new(value);
//
//         ValueLike {
//             string: binding.get_param_dollarised(),
//             bindings: vec![binding],
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use surrealdb::sql;

    use crate::{functions::math, *};

    #[test]
    fn test_heterogeonous_array_values() {
        let country = Field::new("country");
        let age = Field::new("age");
        let gender = Field::new("gender");
        let total = AliasName::new("total");
        let mut values = arr![
            1,
            2,
            3,
            count!().__as__(total),
            math::sum!(age),
            gender,
            country,
            54,
            sql::Duration(std::time::Duration::from_secs(43))
        ];
        values.push(34.into());

        assert_eq!(
            values
                .into_iter()
                .map(|m| m.to_raw().build())
                .collect::<Vec<_>>()
                .join(", "),
            "1, 2, 3, count() AS total, math::sum(age), gender, country, 54, 43s, 34"
        );
    }
}
