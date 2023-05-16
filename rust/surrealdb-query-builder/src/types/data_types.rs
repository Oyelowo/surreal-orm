use crate::{
    statements::{
        CreateStatement, DeleteStatement, IfElseStatement, InsertStatement, LetStatement,
        RelateStatement, SelectStatement, Subquery, UpdateStatement,
    },
    Binding, BindingsList, Buildable, Erroneous, Field, Param, Parametric, SurrealdbEdge,
    SurrealdbModel, SurrealdbNode, Valuex,
};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

macro_rules! create_value_like_struct {
    ($sql_type_name:expr) => {
        paste::paste! {
            /// Represents the value, or field, param which can all be used
            /// to represent the value itself within a query.
            #[derive(Debug, Clone)]
            pub struct [<$sql_type_name Like>]($crate::Valuex);

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
                fn build(&self) -> String {
                    self.0.build()
                }
            }

            impl<T: Into<sql::[<$sql_type_name>]>> From<T> for [<$sql_type_name Like>] {
                fn from(value: T) -> Self {
                    let value: sql::[<$sql_type_name>] = value.into();
                    let value: sql::Value = value.into();
                    Self(value.into())
                }
            }

            impl From<Field> for [<$sql_type_name Like>] {
                fn from(val: Field) -> Self {
                    [<$sql_type_name Like>](val.into())
                }
            }

            impl From<Param> for [<$sql_type_name Like>] {
                fn from(val: Param) -> Self {
                    [<$sql_type_name Like>](val.into())
                }
            }

            impl From<&Field> for [<$sql_type_name Like>] {
                fn from(val: &Field) -> Self {
                    [<$sql_type_name Like>](val.clone().into())
                }
            }


            impl<T> From<CreateStatement<T>> for [<$sql_type_name Like>]
            where
                T: SurrealdbNode + Serialize + DeserializeOwned,
            {
                fn from(statement: CreateStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into())

                }
            }

            impl<T> From<UpdateStatement<T>> for [<$sql_type_name Like>]
            where
                T: SurrealdbModel + Serialize + DeserializeOwned,
            {
                fn from(statement: UpdateStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into())
                }
            }

            impl<T> From<DeleteStatement<T>> for [<$sql_type_name Like>]
            where
                T: SurrealdbModel + Serialize + DeserializeOwned,
            {
                fn from(statement: DeleteStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into())
                }
            }

            impl<T> From<RelateStatement<T>> for [<$sql_type_name Like>]
            where
                T: SurrealdbEdge + Serialize + DeserializeOwned,
            {
                fn from(statement: RelateStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into())
                }
            }

            impl<T> From<InsertStatement<T>> for [<$sql_type_name Like>]
            where
                T: SurrealdbNode + Serialize + DeserializeOwned,
            {
                fn from(statement: InsertStatement<T>) -> Self {
                    [<$sql_type_name Like>](statement.into())
                }
            }

            impl From<IfElseStatement> for [<$sql_type_name Like>] {
                fn from(statement: IfElseStatement) -> Self {
                    [<$sql_type_name Like>](statement.into())
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
    T: SurrealdbNode + Serialize + DeserializeOwned,
{
    fn from(statement: CreateStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<UpdateStatement<T>> for ArrayLike
where
    T: SurrealdbModel + Serialize + DeserializeOwned,
{
    fn from(statement: UpdateStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<DeleteStatement<T>> for ArrayLike
where
    T: SurrealdbModel + Serialize + DeserializeOwned,
{
    fn from(statement: DeleteStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<RelateStatement<T>> for ArrayLike
where
    T: SurrealdbEdge + Serialize + DeserializeOwned,
{
    fn from(statement: RelateStatement<T>) -> Self {
        Self(statement.into())
    }
}

impl<T> From<InsertStatement<T>> for ArrayLike
where
    T: SurrealdbNode + Serialize + DeserializeOwned,
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
