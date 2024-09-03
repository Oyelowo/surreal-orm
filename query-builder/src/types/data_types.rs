/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::{
    statements::{
        CreateStatement, DeleteStatement, IfElseStatement, InsertStatement, LetStatement,
        RelateStatement, SelectStatement, UpdateStatement,
    },
    Binding, BindingsList, Buildable, Edge, Erroneous, ErrorList, Field, Model, Node, Operation,
    Param, Parametric, ValueLike,
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
            pub struct [<$sql_type_name Like>]($crate::ValueLike, ValueType);

            impl [<$sql_type_name Like>] {
                #[allow(dead_code)]
                pub(crate) fn bracket_if_operation(&mut self) -> &Self {
                    if matches!(self.1, ValueType::Operation) {
                        self.0.string = format!("({})", self.0.string);
                    }
                    self
                }
            }

            impl From<[<$sql_type_name Like>]> for $crate::ValueLike {
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
                T: Node + Serialize + DeserializeOwned,
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
                T: Model + Serialize + DeserializeOwned,
            {
                fn from(statement: UpdateStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into(), ValueType::UpdateStatement)
                }
            }

            impl<T> From<DeleteStatement<T>> for [<$sql_type_name Like>]
            where
                T: Model + Serialize + DeserializeOwned,
            {
                fn from(statement: DeleteStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into(), ValueType::DeleteStatement)
                }
            }

            impl<T> From<RelateStatement<T>> for [<$sql_type_name Like>]
            where
                T: Edge + Serialize + DeserializeOwned,
            {
                fn from(statement: RelateStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into(), ValueType::RelateStatement)
                }
            }

            impl<T> From<InsertStatement<T>> for [<$sql_type_name Like>]
            where
                T: Node + Serialize + DeserializeOwned,
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

/// Database index name
pub type IndexName = Field;

/// Represents the surrealdb NULL value
#[derive(Debug, Clone)]
pub struct NULL;

/// Represents the surrealdb NONE value
#[derive(Debug, Clone)]
pub struct NONE;

/// Represents the surrealdb boolean value
#[derive(Debug, Clone)]
pub struct BoolLike(ValueLike);

impl From<BoolLike> for ValueLike {
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
    T: Node + Serialize + DeserializeOwned,
{
    fn from(statement: CreateStatement<T>) -> Self {
        BoolLike(statement.into())
    }
}

impl<T> From<UpdateStatement<T>> for BoolLike
where
    T: Model + Serialize + DeserializeOwned,
{
    fn from(statement: UpdateStatement<T>) -> Self {
        BoolLike(statement.into())
    }
}

impl<T> From<DeleteStatement<T>> for BoolLike
where
    T: Model + Serialize + DeserializeOwned,
{
    fn from(statement: DeleteStatement<T>) -> Self {
        BoolLike(statement.into())
    }
}

impl<T> From<RelateStatement<T>> for BoolLike
where
    T: Edge + Serialize + DeserializeOwned,
{
    fn from(statement: RelateStatement<T>) -> Self {
        BoolLike(statement.into())
    }
}

impl<T> From<InsertStatement<T>> for BoolLike
where
    T: Node + Serialize + DeserializeOwned,
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
pub struct ArrayLike(ValueLike);
impl From<ArrayLike> for ValueLike {
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

impl<const N: usize, T> From<[T; N]> for ArrayLike
where
    T: Into<sql::Value> + Clone,
{
    fn from(value: [T; N]) -> Self {
        let value = value
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<Vec<sql::Value>>();
        Self(value.into())
    }
}

impl<const N: usize, T> From<&[T; N]> for ArrayLike
where
    T: Into<sql::Value> + Clone,
{
    fn from(value: &[T; N]) -> Self {
        let value = value
            .iter()
            .cloned()
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
    T: Node + Serialize + DeserializeOwned,
{
    fn from(statement: CreateStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<UpdateStatement<T>> for ArrayLike
where
    T: Model + Serialize + DeserializeOwned,
{
    fn from(statement: UpdateStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<DeleteStatement<T>> for ArrayLike
where
    T: Model + Serialize + DeserializeOwned,
{
    fn from(statement: DeleteStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<RelateStatement<T>> for ArrayLike
where
    T: Edge + Serialize + DeserializeOwned,
{
    fn from(statement: RelateStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<InsertStatement<T>> for ArrayLike
where
    T: Node + Serialize + DeserializeOwned,
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

impl From<Vec<ValueLike>> for ArrayLike {
    fn from(value: Vec<ValueLike>) -> Self {
        Self(ValueLike {
            string: format!("[{}]", value.build()),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}

/// Represents the surrealdb Set value, or field, param which can all be used
pub type SetLike = ArrayLike;

/// Used to represent a list of arguments to a function
pub struct ArgsList(ValueLike);
impl ArgsList {
    pub(crate) fn get_errors(&self) -> Vec<String> {
        self.0.get_errors()
    }
}
impl From<ArgsList> for ValueLike {
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

        Self(ValueLike {
            string: params.join(", "),
            bindings,
            errors: vec![],
        })
    }
}

impl<const N: usize, T> From<&[T; N]> for ArgsList
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

impl From<Vec<ValueLike>> for ArgsList {
    fn from(value: Vec<ValueLike>) -> Self {
        Self(ValueLike {
            string: value.build().to_string(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}

/// Represents the string literal representation of a value
/// without the surrounding quotes. For example, the string
/// literal representation of the string "Hello" is Hello
/// without the quotes. Also allows for the use of parameters
/// and fields.
#[derive(Debug, Clone)]
pub struct LiteralLike(ValueLike);

impl From<LiteralLike> for ValueLike {
    fn from(val: LiteralLike) -> Self {
        val.0
    }
}

impl Parametric for LiteralLike {
    fn get_bindings(&self) -> BindingsList {
        self.0
            .bindings
            .iter()
            .cloned()
            .map(|b| b.as_raw())
            .collect()
    }
}

impl Erroneous for LiteralLike {
    fn get_errors(&self) -> ErrorList {
        self.0.get_errors()
    }
}

impl Buildable for LiteralLike {
    fn build(&self) -> String {
        self.0.build()
    }
}

impl From<String> for LiteralLike {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<&str> for LiteralLike {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<Field> for LiteralLike {
    fn from(val: Field) -> Self {
        LiteralLike(val.into())
    }
}

impl From<Param> for LiteralLike {
    fn from(val: Param) -> Self {
        LiteralLike(val.into())
    }
}

impl From<&Field> for LiteralLike {
    fn from(val: &Field) -> Self {
        LiteralLike(val.clone().into())
    }
}

impl<T> From<CreateStatement<T>> for LiteralLike
where
    T: Node + Serialize + DeserializeOwned,
{
    fn from(statement: CreateStatement<T>) -> Self {
        LiteralLike(statement.into())
    }
}

impl<T> From<UpdateStatement<T>> for LiteralLike
where
    T: Model + Serialize + DeserializeOwned,
{
    fn from(statement: UpdateStatement<T>) -> Self {
        LiteralLike(statement.into())
    }
}

impl<T> From<DeleteStatement<T>> for LiteralLike
where
    T: Model + Serialize + DeserializeOwned,
{
    fn from(statement: DeleteStatement<T>) -> Self {
        LiteralLike(statement.into())
    }
}

impl<T> From<RelateStatement<T>> for LiteralLike
where
    T: Edge + Serialize + DeserializeOwned,
{
    fn from(statement: RelateStatement<T>) -> Self {
        LiteralLike(statement.into())
    }
}

impl<T> From<InsertStatement<T>> for LiteralLike
where
    T: Node + Serialize + DeserializeOwned,
{
    fn from(statement: InsertStatement<T>) -> Self {
        LiteralLike(statement.into())
    }
}

impl From<IfElseStatement> for LiteralLike {
    fn from(statement: IfElseStatement) -> Self {
        LiteralLike(statement.into())
    }
}
