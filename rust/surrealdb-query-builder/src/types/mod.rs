/*cond,
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

pub(crate) mod alias;
pub(crate) mod array;
pub(crate) mod clause;
pub(crate) mod crud_type;
pub(crate) mod expression;
pub(crate) mod field;
pub(crate) mod field_type;
pub(crate) mod field_updater;
pub(crate) mod filter;
pub(crate) mod function;
pub(crate) mod geometry;
pub(crate) mod idiom;
pub(crate) mod interval;
pub(crate) mod links;
pub(crate) mod numbers;
pub(crate) mod ordinal;
pub(crate) mod param;
pub(crate) mod params_standard;
pub(crate) mod return_;
pub(crate) mod strand;
pub(crate) mod surreal_id;
pub(crate) mod token_target;
pub(crate) mod value;
pub(crate) mod valuex;

pub use alias::*;
pub use array::*;
pub use clause::*;
pub use crud_type::*;
pub use field::*;
pub use field_type::*;
pub use field_updater::*;
pub use filter::*;
pub use function::*;
pub use geometry::*;
pub use idiom::*;
pub use interval::*;
pub use links::*;
pub use numbers::*;
pub use numbers::*;
pub use ordinal::*;
pub use ordinal::*;
pub use param::*;
pub use param::*;
pub use params_standard::*;
pub use return_::*;
pub use strand::*;
pub use surreal_id::*;
pub use token_target::*;
pub use value::*;
pub use valuex::*;

use surrealdb::sql;
macro_rules! create_value_like_struct {
    ($sql_type_name:expr) => {
        paste::paste! {
            #[derive(serde::Serialize, Debug, Clone)]
            pub enum [<$sql_type_name Like>] {
                [<$sql_type_name>](sql::[<$sql_type_name>]),
                Field(sql::Idiom),
                Param(sql::Param),
            }

            impl [<$sql_type_name Like>] {
                pub fn to_value(self) -> sql::Value {
                    match self {
                        [<$sql_type_name Like>]::[<$sql_type_name>](g) => g.into(),
                        [<$sql_type_name Like>]::Field(f) => f.into(),
                        [<$sql_type_name Like>]::Param(p) => p.into(),
                    }
                }
            }
            // macro_rules! impl_geometry_like_from {
            //     ($($t:ty),*) => {
            //         $(impl From<$t> for GeometryLike {
            //             fn from(value: $t) -> Self {
            //             }
            //         })*
            //     };
            // }
            //
            // impl_geometry_like_from!(
            //     geo::Polygon,
            //     geo::Point,
            //     geo::LineString,
            //     geo::MultiPoint,
            //     geo::MultiPolygon,
            //     geo::MultiLineString
            // );


            impl<T: Into<sql::[<$sql_type_name>]>> From<T> for [<$sql_type_name Like>] {
                fn from(value: T) -> Self {
                    let value: sql::[<$sql_type_name>] = value.into();
                    Self::[<$sql_type_name>](value.into())
                }
            }

            // impl From<crate::types::Empty> for Option<[<$sql_type_name Like>]> {
            //     fn from(value: crate::types::Empty) -> Self {
            //         None
            //     }
            // }

            // impl From<Option<Self>> for [<$sql_type_name Like>] {
            //     fn from(value: Option<Self>) -> Self {
            //         todo!()
            //     }
            // }
            impl From<Field> for [<$sql_type_name Like>] {
                fn from(val: Field) -> Self {
                    [<$sql_type_name Like>]::Field(val.into())
                }
            }

            impl From<Param> for [<$sql_type_name Like>] {
                fn from(val: Param) -> Self {
                    [<$sql_type_name Like>]::Param(val.clone().into())
                }
            }

            impl From<&Field> for [<$sql_type_name Like>] {
                fn from(val: &Field) -> Self {
                    [<$sql_type_name Like>]::Field(val.clone().into())
                }
            }

            // impl From<sql::Value> for [<$sql_type_name Like>] {
            //     fn from(value: sql::Value) -> [<$sql_type_name Like>] {
            //         Self::[<$sql_type_name>](value)
            //     }
            // }

            impl From<[<$sql_type_name Like>]> for sql::Value {
                fn from(val: [<$sql_type_name Like>]) -> sql::Value {
                    match val {
                        [<$sql_type_name Like>]::[<$sql_type_name>](g) => g.into(),
                        [<$sql_type_name Like>]::Field(f) => f.into(),
                        [<$sql_type_name Like>]::Param(p) => p.into(),
                    }
                }
            }
        }
    };
}

// creates NumberLike, StrandLike etc which can also be a field or param
create_value_like_struct!("Number");
create_value_like_struct!("Strand");
create_value_like_struct!("Geometry");
create_value_like_struct!("Array");
// create_value_like_struct!("Idiom");
create_value_like_struct!("Duration");
create_value_like_struct!("Datetime");
create_value_like_struct!("Table");
// create_value_like_struct!("Value");
create_value_like_struct!("Object");

// impl<T> From<T> for ArrayLike
// where
//     T: Into<sql::Value>,
// {
//     fn from(value: T) -> Self {
//         Self::Array(sql::Value::from(value.into()))
//     }
// }

// impl<T, const N: usize> From<&[T; N]> for ArrayLike
// where
//     T: Into<sql::Value> + Clone,
// {
//     fn from(value: &[T; N]) -> Self {
//         Self::Array(
//             value
//                 .into_iter()
//                 .map(|v| v.clone().into())
//                 .collect::<Vec<sql::Value>>()
//                 .into(),
//         )
//     }
// }
