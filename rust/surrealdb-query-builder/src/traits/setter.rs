use serde::Serialize;
use surrealdb::sql;

use crate::{
    statements::{LetStatement, Subquery},
    Binding, BindingsList, Buildable, Conditional, Erroneous, ErrorList, Field, Param, Parametric,
};

/// A helper struct for generating SQL update statements.
#[derive(Debug, Clone)]
pub struct Setter {
    query_string: String,
    bindings: BindingsList,
    errors: ErrorList,
}

impl std::fmt::Display for Setter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

// struct SetArg {
//     string: String,
//     bindings: BindingsList,
//     errors: ErrorList,
// }
//
// impl Buildable for SetArg {
//     fn build(&self) -> String {
//         self.string.to_string()
//     }
// }
//
// impl Parametric for SetArg {
//     fn get_bindings(&self) -> BindingsList {
//         self.bindings.to_vec()
//     }
// }
//
// impl Erroneous for SetArg {
//     fn get_errors(&self) -> ErrorList {
//         self.errors.to_vec()
//     }
// }
//
// impl<T: Serialize> From<T> for SetArg {
//     fn from(value: T) -> Self {
//         let sql_value = sql::json(&serde_json::to_string(&value).unwrap()).unwrap();
//         let binding = Binding::new(sql_value);
//
//         Self {
//             string: binding.get_param_dollarised(),
//             bindings: vec![binding],
//             errors: vec![],
//         }
//     }
// }
//
// impl From<Field> for SetArg {
//     fn from(value: Field) -> Self {
//         Self {
//             string: value.build(),
//             bindings: value.get_bindings(),
//             errors: value.get_errors(),
//         }
//     }
// }
//
// impl From<Param> for SetArg {
//     fn from(value: Param) -> Self {
//         Self {
//             string: value.build(),
//             bindings: value.get_bindings(),
//             errors: value.get_errors(),
//         }
//     }
// }
//
// impl From<LetStatement> for SetArg {
//     fn from(value: LetStatement) -> Self {
//         Self {
//             string: value.get_param().build(),
//             bindings: value.get_bindings(),
//             errors: value.get_errors(),
//         }
//     }
// }

impl From<Setter> for Vec<Setter> {
    fn from(value: Setter) -> Self {
        vec![value]
    }
}

impl Parametric for Setter {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for Setter {
    fn build(&self) -> String {
        self.query_string.to_string()
    }
}
impl Erroneous for Setter {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}
impl Parametric for Vec<Setter> {
    fn get_bindings(&self) -> BindingsList {
        self.iter().fold(vec![], |mut acc, setter| {
            acc.extend(setter.get_bindings());
            acc
        })
    }
}

impl Buildable for Vec<Setter> {
    fn build(&self) -> String {
        self.iter()
            .map(|setter| setter.build())
            .collect::<Vec<String>>()
            .join(", ")
    }
}
impl Erroneous for Vec<Setter> {
    fn get_errors(&self) -> ErrorList {
        self.iter().fold(vec![], |mut acc, setter| {
            acc.extend(setter.get_errors());
            acc
        })
    }
}

pub enum SetterArg<T>
where
    T: Serialize,
{
    Value(T),
    Field(Field),
    Subquery(Subquery),
    Param(Param),
    LetStatement(LetStatement),
}

// impl<T: Serialize> From<T> for SetterArg<T> {
//     fn from(value: T) -> Self {
//         // Self::Value(value)
//         todo!()
//     }
// }
impl<T: Serialize, V: Into<T> + Serialize> From<V> for SetterArg<T> {
    fn from(value: V) -> Self {
        // Self::Value(value)
        todo!()
    }
}

impl<T: Serialize> From<Field> for SetterArg<T> {
    fn from(value: Field) -> Self {
        Self::Field(value)
    }
}

impl Conditional for Setter {}

/// A trait for assigning values to a field used in `SET`
/// function in create and update statements.
pub trait SetterAssignable<T: Serialize>
where
    Self: std::ops::Deref<Target = Field>,
{
    /// Assigns the given value to the field.
    fn equal_to(&self, value: impl Into<SetterArg<T>>) -> Setter {
        let operator = sql::Operator::Equal;
        let field = self.deref();
        // let set_arg: SetArg = value.into().into();
        let set_arg: SetterArg<T> = value.into().into();

        let (build, bindings, errors) = match set_arg {
            SetterArg::Value(value) => {
                let sql_value = sql::json(&serde_json::to_string(&value).unwrap()).unwrap();
                let binding = Binding::new(sql_value);
                (binding.get_param_dollarised(), vec![binding], vec![])
            }
            SetterArg::Field(field) => (field.build(), field.get_bindings(), field.get_errors()),
            SetterArg::Subquery(subquery) => (
                subquery.build(),
                subquery.get_bindings(),
                subquery.get_errors(),
            ),
            SetterArg::Param(param) => (param.build(), param.get_bindings(), param.get_errors()),
            SetterArg::LetStatement(let_statement) => (
                let_statement.get_param().build(),
                let_statement.get_bindings(),
                let_statement.get_errors(),
            ),
        };

        let column_updater_string = format!("{field} {operator} {}", build);
        Setter {
            query_string: column_updater_string,
            bindings,
            errors,
        }
    }

    /// Derefs to field type.
    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}

/// A trait for incrementing or decrementing values to a field used in `SET`
/// function in create and update statements.
pub trait SetterNumeric<T: Serialize>
where
    Self: std::ops::Deref<Target = Field>,
{
    /// Increments the value of the field by the given value.
    fn increment_by(&self, value: impl Into<T>) -> Setter {
        let operator = sql::Operator::Inc;
        let field = self.deref();
        let set_arg: SetArg = value.into().into();

        let column_updater_string = format!("{field} {operator} {}", set_arg.build());
        Setter {
            query_string: column_updater_string,
            bindings: set_arg.get_bindings(),
            errors: set_arg.get_errors(),
        }
    }

    /// Decrements the value of the field by the given value.
    fn decrement_by(&self, value: impl Into<T>) -> Setter {
        let operator = sql::Operator::Dec;
        let field = self.deref();
        let set_arg: SetArg = value.into().into();

        let column_updater_string = format!("{field} {operator} {}", set_arg.build());
        Setter {
            query_string: column_updater_string,
            bindings: set_arg.get_bindings(),
            errors: set_arg.get_errors(),
        }
    }

    /// Derefs to field type.
    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}

/// Setter for array fields.
pub trait SetterArray<T: Serialize>
where
    Self: std::ops::Deref<Target = Field>,
{
    /// Appends the given value to the array.
    fn append(&self, value: impl Into<T>) -> Setter {
        let operator = sql::Operator::Inc;
        let field = self.deref();
        let set_arg: SetArg = value.into().into();

        let column_updater_string = format!("{field} {operator} {}", set_arg.build());
        Setter {
            query_string: column_updater_string,
            bindings: set_arg.get_bindings(),
            errors: set_arg.get_errors(),
        }
    }

    /// Removes the given value from the array.
    fn remove(&self, value: impl Into<T>) -> Setter {
        let operator = sql::Operator::Dec;
        let field = self.deref();
        let set_arg: SetArg = value.into().into();

        let column_updater_string = format!("{field} {operator} {}", set_arg.build());
        Setter {
            query_string: column_updater_string,
            bindings: set_arg.get_bindings(),
            errors: set_arg.get_errors(),
        }
    }

    /// Derefs to field type.
    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}
