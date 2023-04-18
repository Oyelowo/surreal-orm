use std::fmt::Display;

use surrealdb::sql;

use crate::Buildable;

use super::{Field, Param};

#[derive(Debug, Clone, Copy)]
pub enum Interval {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Interval::Year => "year",
                Interval::Month => "month",
                Interval::Hour => "hour",
                Interval::Minute => "minute",
                Interval::Second => "second",
                Interval::Week => "week",
                Interval::Day => "day",
            }
        )
    }
}

pub struct IntervalLike(String);

impl From<Field> for IntervalLike {
    fn from(value: Field) -> Self {
        Self(value.build())
    }
}

impl From<Param> for IntervalLike {
    fn from(value: Param) -> Self {
        Self(value.build())
    }
}

// impl From<IntervalLike> for sql::Value {
//     fn from(value: IntervalLike) -> Self {
//         value.0
//     }
// }

impl From<Interval> for IntervalLike {
    fn from(value: Interval) -> Self {
        Self(value.to_string().into())
    }
}
