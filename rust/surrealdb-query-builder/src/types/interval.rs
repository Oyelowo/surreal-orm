use std::fmt::Display;

use surrealdb::sql;

use crate::{Buildable, StrandLike, Valuex};

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

impl From<Interval> for StrandLike {
    fn from(value: Interval) -> Self {
        value.to_string().into()
    }
}
