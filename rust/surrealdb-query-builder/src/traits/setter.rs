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

/// A helper struct for generating SQL update statements.
#[allow(missing_docs)]
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

impl<T: Serialize, V: Into<T> + Serialize> From<V> for SetterArg<T> {
    fn from(value: V) -> Self {
        Self::Value(value.into())
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
        get_setter(value, self.deref(), sql::Operator::Equal)
    }

    /// Derefs to field type.
    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}

fn get_setter<T: Serialize>(
    value: impl Into<SetterArg<T>>,
    field: &Field,
    operator: sql::Operator,
) -> Setter {
    let set_arg: SetterArg<T> = value.into();

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

/// A trait for incrementing or decrementing values to a field used in `SET`
/// function in create and update statements.
pub trait SetterNumeric<T: Serialize>
where
    Self: std::ops::Deref<Target = Field>,
{
    /// Increments the value of the field by the given value.
    fn increment_by(&self, value: impl Into<SetterArg<T>>) -> Setter {
        get_setter(value, self.deref(), sql::Operator::Inc)
    }

    /// Decrements the value of the field by the given value.
    fn decrement_by(&self, value: impl Into<SetterArg<T>>) -> Setter {
        get_setter(value, self.deref(), sql::Operator::Dec)
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
    fn append(&self, value: impl Into<SetterArg<T>>) -> Setter {
        get_setter(value, self.deref(), sql::Operator::Inc)
    }

    /// Removes the given value from the array.
    fn remove(&self, value: impl Into<SetterArg<T>>) -> Setter {
        get_setter(value, self.deref(), sql::Operator::Dec)
    }

    /// Derefs to field type.
    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}
