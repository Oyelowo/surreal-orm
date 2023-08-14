/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{
    statements::{
        CreateStatement, DeleteStatement, IfElseStatement, InsertStatement, LetStatement,
        RelateStatement, SelectStatement, UpdateStatement,
    },
    Binding, BindingsList, Buildable, Erroneous, ErrorList, Field, Operation, Param, Parametric,
    SurrealEdge, SurrealModel, SurrealNode, Valuex,
};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

/// Represents the value, or field, param which can all be used
#[derive(Debug, Clone)]
pub(crate) enum ValueType {
    Value,
    Field,
    LetStatement,
    Param,
    Operation,
    IfElseStatement,
    SelectStatement,
    InsertStatement,
    UpdateStatement,
    DeleteStatement,
    RelateStatement,
    CreateStatement,
}

macro_rules! create_value_like_struct {
    ($sql_type_name:expr) => {
        paste::paste! {
            /// Represents the value, or field, param which can all be used
            /// to represent the value itself within a query.
            #[derive(Debug, Clone)]
            pub struct [<$sql_type_name Like>]($crate::Valuex, ValueType);

            impl [<$sql_type_name Like>] {
                #[allow(dead_code)]
                pub(crate) fn bracket_if_operation(&mut self) -> &Self {
                    if matches!(self.1, ValueType::Operation) {
                        self.0.string = format!("({})", self.0.string);
                    }
                    self
                }
            }

            impl From<[<$sql_type_name Like>]> for $crate::Valuex {
                fn from(val: [<$sql_type_name Like>]) -> Self {
                    val.0
                }
            }

            impl $crate::Parametric for [<$sql_type_name Like>] {
                fn get_bindings(&self) -> $crate::BindingsList {
                    self.0.bindings.to_vec()
                }
            }

            impl $crate::Buildable for [<$sql_type_name Like>] {
                fn build(&self) -> ::std::string::String {
                    self.0.build()
                }
            }

            impl $crate::Erroneous for [<$sql_type_name Like>] {
                fn get_errors(&self) -> $crate::ErrorList {
                    self.0.get_errors()
                }
            }

            impl<T: Into<sql::[<$sql_type_name>]>> From<T> for [<$sql_type_name Like>] {
                fn from(value: T) -> Self {
                    let value: sql::[<$sql_type_name>] = value.into();
                    let value: sql::Value = value.into();
                    Self(value.into(), ValueType::Value)
                }
            }

            impl From<Field> for [<$sql_type_name Like>] {
                fn from(val: Field) -> Self {
                    [<$sql_type_name Like>](val.into(), ValueType::Field)
                }
            }

            impl From<&Field> for [<$sql_type_name Like>] {
                fn from(val: &Field) -> Self {
                    [<$sql_type_name Like>](val.clone().into(), ValueType::Field)
                }
            }

            impl From<LetStatement> for [<$sql_type_name Like>] {
                fn from(val: LetStatement) -> Self {
                    [<$sql_type_name Like>](val.into(), ValueType::LetStatement)
                }
            }

            impl From<&LetStatement> for [<$sql_type_name Like>] {
                fn from(val: &LetStatement) -> Self {
                    [<$sql_type_name Like>](val.clone().into(), ValueType::LetStatement)
                }
            }

            impl From<Param> for [<$sql_type_name Like>] {
                fn from(val: Param) -> Self {
                    [<$sql_type_name Like>](val.into(), ValueType::Param)
                }
            }

            impl From<Operation> for [<$sql_type_name Like>] {
                fn from(val: Operation) -> Self {
                    [<$sql_type_name Like>](val.into(), ValueType::Operation)
                }
            }

            impl<T> From<CreateStatement<T>> for [<$sql_type_name Like>]
            where
                T: SurrealNode + Serialize + DeserializeOwned,
            {
                fn from(statement: CreateStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into(), ValueType::CreateStatement)

                }
            }

            impl From<SelectStatement> for [<$sql_type_name Like>]
            {
                fn from(statement: SelectStatement) -> Self {
                    [<$sql_type_name Like>](statement.into(), ValueType::SelectStatement)
                }
            }

            impl<T> From<UpdateStatement<T>> for [<$sql_type_name Like>]
            where
                T: SurrealModel + Serialize + DeserializeOwned,
            {
                fn from(statement: UpdateStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into(), ValueType::UpdateStatement)
                }
            }

            impl<T> From<DeleteStatement<T>> for [<$sql_type_name Like>]
            where
                T: SurrealModel + Serialize + DeserializeOwned,
            {
                fn from(statement: DeleteStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into(), ValueType::DeleteStatement)
                }
            }

            impl<T> From<RelateStatement<T>> for [<$sql_type_name Like>]
            where
                T: SurrealEdge + Serialize + DeserializeOwned,
            {
                fn from(statement: RelateStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into(), ValueType::RelateStatement)
                }
            }

            impl<T> From<InsertStatement<T>> for [<$sql_type_name Like>]
            where
                T: SurrealNode + Serialize + DeserializeOwned,
            {
                fn from(statement: InsertStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into(), ValueType::InsertStatement)
                }
            }

            impl From<IfElseStatement> for [<$sql_type_name Like>] {
                fn from(statement: IfElseStatement) -> Self {
                    [<$sql_type_name Like>](statement.into(), ValueType::IfElseStatement)
                }
            }
        }
    };
}

// creates NumberLike, StrandLike etc which can also be a field or param
create_value_like_struct!("Number");
create_value_like_struct!("Strand");
create_value_like_struct!("Geometry");
create_value_like_struct!("Thing");

create_value_like_struct!("Duration");
create_value_like_struct!("Datetime");
create_value_like_struct!("Table");
create_value_like_struct!("Object");

/// Represents the surrealdb NULL value
#[derive(Debug, Clone)]
pub struct NULL;

/// Represents the surrealdb NONE value
#[derive(Debug, Clone)]
pub struct NONE;

/// Represents the surrealdb boolean value
#[derive(Debug, Clone)]
pub struct BoolLike(Valuex);

impl From<BoolLike> for Valuex {
    fn from(val: BoolLike) -> Self {
        val.0
    }
}

impl Parametric for BoolLike {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

impl Erroneous for BoolLike {
    fn get_errors(&self) -> ErrorList {
        self.0.get_errors()
    }
}

impl Buildable for BoolLike {
    fn build(&self) -> String {
        self.0.build()
    }
}

impl From<bool> for BoolLike {
    fn from(value: bool) -> Self {
        let value: sql::Value = value.into();
        Self(value.into())
    }
}

impl From<Field> for BoolLike {
    fn from(val: Field) -> Self {
        BoolLike(val.into())
    }
}

impl From<Param> for BoolLike {
    fn from(val: Param) -> Self {
        BoolLike(val.into())
    }
}

impl From<&Field> for BoolLike {
    fn from(val: &Field) -> Self {
        BoolLike(val.clone().into())
    }
}

impl<T> From<CreateStatement<T>> for BoolLike
where
    T: SurrealNode + Serialize + DeserializeOwned,
{
    fn from(statement: CreateStatement<T>) -> Self {
        BoolLike(statement.into())
    }
}

impl<T> From<UpdateStatement<T>> for BoolLike
where
    T: SurrealModel + Serialize + DeserializeOwned,
{
    fn from(statement: UpdateStatement<T>) -> Self {
        BoolLike(statement.into())
    }
}

impl<T> From<DeleteStatement<T>> for BoolLike
where
    T: SurrealModel + Serialize + DeserializeOwned,
{
    fn from(statement: DeleteStatement<T>) -> Self {
        BoolLike(statement.into())
    }
}

impl<T> From<RelateStatement<T>> for BoolLike
where
    T: SurrealEdge + Serialize + DeserializeOwned,
{
    fn from(statement: RelateStatement<T>) -> Self {
        BoolLike(statement.into())
    }
}

impl<T> From<InsertStatement<T>> for BoolLike
where
    T: SurrealNode + Serialize + DeserializeOwned,
{
    fn from(statement: InsertStatement<T>) -> Self {
        BoolLike(statement.into())
    }
}

impl From<IfElseStatement> for BoolLike {
    fn from(statement: IfElseStatement) -> Self {
        BoolLike(statement.into())
    }
}

/// Represents the surrealdb Array value, or field, param which can all be used
/// to represent the value itself within a query.
#[derive(Debug, Clone)]
pub struct ArrayLike(Valuex);
impl From<ArrayLike> for Valuex {
    fn from(val: ArrayLike) -> Self {
        val.0
    }
}
impl Parametric for ArrayLike {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

impl Erroneous for ArrayLike {
    fn get_errors(&self) -> ErrorList {
        self.0.errors.to_vec()
    }
}

impl Buildable for ArrayLike {
    fn build(&self) -> String {
        self.0.build()
    }
}
impl<T: Into<sql::Value>> From<Vec<T>> for ArrayLike {
    fn from(value: Vec<T>) -> Self {
        let value = value
            .into_iter()
            .map(Into::into)
            .collect::<Vec<sql::Value>>();
        Self(value.into())
    }
}

impl<'a, const N: usize, T> From<&[T; N]> for ArrayLike
where
    T: Into<sql::Value> + Clone,
{
    fn from(value: &[T; N]) -> Self {
        let value = value
            .to_vec()
            .into_iter()
            .map(Into::into)
            .collect::<Vec<sql::Value>>();
        Self(value.into())
    }
}

impl From<Field> for ArrayLike {
    fn from(val: Field) -> Self {
        Self(val.into())
    }
}
impl From<Param> for ArrayLike {
    fn from(val: Param) -> Self {
        Self(val.into())
    }
}
impl From<&Param> for ArrayLike {
    fn from(val: &Param) -> Self {
        Self(val.to_owned().into())
    }
}
impl From<LetStatement> for ArrayLike {
    fn from(val: LetStatement) -> Self {
        Self(val.get_param().into())
    }
}
impl From<&LetStatement> for ArrayLike {
    fn from(val: &LetStatement) -> Self {
        Self(val.get_param().into())
    }
}

impl From<&Field> for ArrayLike {
    fn from(val: &Field) -> Self {
        Self(val.clone().into())
    }
}

impl From<SelectStatement> for ArrayLike {
    fn from(statement: SelectStatement) -> Self {
        Self(statement.into())
    }
}

impl<T> From<CreateStatement<T>> for ArrayLike
where
    T: SurrealNode + Serialize + DeserializeOwned,
{
    fn from(statement: CreateStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<UpdateStatement<T>> for ArrayLike
where
    T: SurrealModel + Serialize + DeserializeOwned,
{
    fn from(statement: UpdateStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<DeleteStatement<T>> for ArrayLike
where
    T: SurrealModel + Serialize + DeserializeOwned,
{
    fn from(statement: DeleteStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<RelateStatement<T>> for ArrayLike
where
    T: SurrealEdge + Serialize + DeserializeOwned,
{
    fn from(statement: RelateStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<InsertStatement<T>> for ArrayLike
where
    T: SurrealNode + Serialize + DeserializeOwned,
{
    fn from(statement: InsertStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl From<IfElseStatement> for ArrayLike {
    fn from(statement: IfElseStatement) -> Self {
        Self(statement.into())
    }
}

struct Array(sql::Array);

impl From<Array> for sql::Array {
    fn from(value: Array) -> Self {
        value.0
    }
}

impl From<sql::Array> for Array {
    fn from(value: sql::Array) -> Self {
        Self(value)
    }
}

impl From<Vec<Valuex>> for ArrayLike {
    fn from(value: Vec<Valuex>) -> Self {
        Self(Valuex {
            string: format!("[{}]", value.build()),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}

/// Used to represent a list of arguments to a function
pub struct ArgsList(Valuex);
impl ArgsList {
    pub(crate) fn get_errors(&self) -> Vec<String> {
        self.0.get_errors()
    }
}
impl From<ArgsList> for Valuex {
    fn from(val: ArgsList) -> Self {
        val.0
    }
}
impl Parametric for ArgsList {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

impl Erroneous for ArgsList {
    fn get_errors(&self) -> ErrorList {
        self.0.errors.to_vec()
    }
}

impl Buildable for ArgsList {
    fn build(&self) -> String {
        self.0.build()
    }
}

impl<T: Into<sql::Value>> From<Vec<T>> for ArgsList {
    fn from(value: Vec<T>) -> Self {
        let (params, bindings): (Vec<_>, Vec<_>) = value
            .into_iter()
            .map(|v| {
                let binding = Binding::new(v.into());
                (binding.get_param_dollarised(), binding)
            })
            .unzip();

        Self(Valuex {
            string: params.join(", "),
            bindings,
            errors: vec![],
        })
    }
}

impl<'a, const N: usize, T> From<&[T; N]> for ArgsList
where
    T: Into<sql::Value> + Clone,
{
    fn from(value: &[T; N]) -> Self {
        value.to_vec().into()
    }
}

impl From<Field> for ArgsList {
    fn from(val: Field) -> Self {
        Self(val.into())
    }
}

impl From<Param> for ArgsList {
    fn from(val: Param) -> Self {
        Self(val.into())
    }
}

impl From<&Field> for ArgsList {
    fn from(val: &Field) -> Self {
        Self(val.clone().into())
    }
}

impl From<Vec<Valuex>> for ArgsList {
    fn from(value: Vec<Valuex>) -> Self {
        Self(Valuex {
            string: format!("{}", value.build()),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}
