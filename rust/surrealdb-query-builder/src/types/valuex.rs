/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{
    Alias, All, Binding, BindingsList, Buildable, Field, Filter, Function, Operation, Param,
    Parametric, E, NONE, NULL,
};
use surrealdb::sql;

/// A value that can be used in a SQL statement. Serves as the bind and arbiter between
/// `sql::Value` and the query building world.
#[derive(Debug, Clone)]
pub struct Valuex {
    pub(crate) string: String,
    pub(crate) bindings: BindingsList,
}

impl Parametric for Valuex {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Parametric for Vec<Valuex> {
    fn get_bindings(&self) -> BindingsList {
        self.into_iter()
            .flat_map(|m| m.get_bindings())
            .collect::<Vec<_>>()
    }
}

impl Buildable for Valuex {
    fn build(&self) -> String {
        self.string.to_string()
    }
}

impl Buildable for Vec<Valuex> {
    fn build(&self) -> String {
        self.into_iter()
            .map(|m| m.build())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl From<&Field> for Valuex {
    fn from(value: &Field) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}

impl From<Field> for Valuex {
    fn from(value: Field) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}

impl From<Param> for Valuex {
    fn from(value: Param) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}

impl From<Alias> for Valuex {
    fn from(value: Alias) -> Self {
        Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}

impl From<All> for Valuex {
    fn from(_value: All) -> Self {
        Valuex {
            string: "*".to_string(),
            bindings: vec![],
        }
    }
}

impl From<NULL> for Valuex {
    fn from(_value: NULL) -> Self {
        Valuex {
            string: "NULL".to_string(),
            bindings: vec![],
        }
    }
}

impl From<E> for Valuex {
    fn from(_value: E) -> Self {
        Valuex {
            string: "".to_string(),
            bindings: vec![],
        }
    }
}

impl From<NONE> for Valuex {
    fn from(_value: NONE) -> Self {
        Valuex {
            string: "NONE".to_string(),
            bindings: vec![],
        }
    }
}

impl From<Filter> for Valuex {
    fn from(value: Filter) -> Self {
        Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}

impl From<Operation> for Valuex {
    fn from(value: Operation) -> Self {
        Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}

impl From<Function> for Valuex {
    fn from(value: Function) -> Self {
        Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}

impl<T: Into<sql::Value>> From<T> for Valuex {
    fn from(value: T) -> Self {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);

        Valuex {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
        }
    }
}

/// A macro to create a heterogenous list of anything
/// that can be converted into `Valuex` including all
/// supported surrealdb types like numbers,
/// dates, field, param, geometry etc, and statements.
/// It creates Vec<Valuex>` from a list of values.
///
/// # Arguments
/// * `$( $val:expr ),*` - A list of values that can be converted into `Valuex`
/// # Example
/// ```rust
/// use surrealdb::sql;
/// use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, functions::{math, count}};
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
            $( $crate::Valuex::from($val) ),*
        ]
    }};
}

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
