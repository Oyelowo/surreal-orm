/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use super::Field;
use crate::{Binding, Buildable, Parametric, ValueLike};
// use bigdecimal::BigDecimal;
use surrealdb::sql;

/// A value that cen be ordered
#[derive(Debug, Clone)]
pub enum Ordinal {
    /// A datetime value
    Datetime(sql::Datetime),
    /// A number value
    Number(sql::Number),
    /// A field value
    Field(Field),
    /// A geometry value
    Geometry(sql::Geometry),
}

impl From<Ordinal> for ValueLike {
    fn from(value: Ordinal) -> Self {
        let (string, bindings) = match value {
            Ordinal::Datetime(d) => {
                let binding = Binding::new(d);
                let param = binding.get_param_dollarised();
                (param, vec![binding])
            }
            Ordinal::Number(n) => {
                let binding = Binding::new(n);
                let param = binding.get_param_dollarised();
                (param, vec![binding])
            }
            Ordinal::Field(f) => (f.build(), f.get_bindings()),
            Ordinal::Geometry(g) => {
                let binding = Binding::new(g);
                let param = binding.get_param_dollarised();
                (param, vec![binding])
            }
        };
        ValueLike {
            string,
            bindings,
            errors: vec![],
        }
    }
}

impl From<sql::Datetime> for Ordinal {
    fn from(value: sql::Datetime) -> Self {
        Self::Datetime(value)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Ordinal {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self::Datetime(value.into())
    }
}

macro_rules! impl_number_or_field_from {
    ($($t:ty),*) => {
        $(impl From<$t> for Ordinal {
            fn from(value: $t) -> Self {
                Self::Number(sql::Number::from(value))
            }
        })*
    };
}

// Using this over generics because sql::Number, sql::Geeomtry also
// impl<T: Into<sql::Number>> From<T> for Ordinal and that creates
// a conflict and ambiguity with the From<sql::Number> for Ordinal impl.
impl_number_or_field_from!(
    i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32,
    f64 // i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, BigDecimal
);

impl<T: Into<Field>> From<T> for Ordinal {
    fn from(val: T) -> Self {
        Ordinal::Field(val.into())
    }
}

impl From<sql::Number> for Ordinal {
    fn from(val: sql::Number) -> Self {
        Ordinal::Number(val)
    }
}
