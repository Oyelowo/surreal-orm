#[derive(serde::Serialize, Debug, Clone)]
pub enum Ordinal {
    Datetime(sql::Datetime),
    Number(sql::Number),
    Field(sql::Value),
}
impl From<sql::Datetime> for Ordinal {
    fn from(value: sql::Datetime) -> Operator {
        Self::Datetime(value.into())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Ordinal {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Operator {
        Self::Datetime(value.into())
    }
}

macro_rules! impl_number_or_field_from {
    ($($t:ty),*) => {
        $(impl From<$t> for Ordinal {
            fn from(value: $t) -> Operator {
                Self::Number(sql::Number::from(value))
            }
        })*
    };
}

impl_number_or_field_from!(
    i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, BigDecimal
);

impl Into<Ordinal> for Field {
    fn into(self) -> Ordinal {
        Ordinal::Field(self.into())
    }
}

impl Into<Ordinal> for &Field {
    fn into(self) -> Ordinal {
        Ordinal::Field(self.into())
    }
}
impl Into<sql::Value> for Ordinal {
    fn into(self) -> sql::Value {
        match self {
            Ordinal::Datetime(n) => n.into(),
            Ordinal::Number(n) => n.into(),
            Ordinal::Field(f) => f.into(),
        }
    }
}

impl Into<Ordinal> for sql::Number {
    fn into(self) -> Ordinal {
        Ordinal::Number(self)
    }
}
