use bigdecimal::BigDecimal;
use surrealdb::sql;

use super::Field;

#[derive(serde::Serialize, Debug, Clone)]
pub enum Ordinal {
    Datetime(sql::Datetime),
    Number(sql::Number),
    Field(sql::Value),
}
impl From<sql::Datetime> for Ordinal {
    fn from(value: sql::Datetime) -> Self {
        Self::Datetime(value.into())
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

impl_number_or_field_from!(
    i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, BigDecimal
);

impl From<Field> for Ordinal {
    fn from(val: Field) -> Self {
        Ordinal::Field(val.into())
    }
}

impl From<&Field> for Ordinal {
    fn from(val: &Field) -> Self {
        Ordinal::Field(val.into())
    }
}
impl From<Ordinal> for sql::Value {
    fn from(val: Ordinal) -> Self {
        match val {
            Ordinal::Datetime(n) => n.into(),
            Ordinal::Number(n) => n.into(),
            Ordinal::Field(f) => f.into(),
        }
    }
}

impl From<sql::Number> for Ordinal {
    fn from(val: sql::Number) -> Self {
        Ordinal::Number(val)
    }
}
impl<T: Into<sql::Number>> From<T> for Ordinal {
    fn from(value: T) -> Self {
        let value: sql::Number = value.into();
        Self::Number(value.into())
    }
}

impl<T: Into<sql::Datetime>> From<T> for Ordinal {
    fn from(value: T) -> Self {
        let value: sql::Datetime = value.into();
        Self::Datetime(value.into())
    }
}

impl<T: Into<Field>> From<T> for Ordinal {
    fn from(value: T) -> Self {
        let value: Field = value.into();
        Self::Field(value.into())
    }
}
