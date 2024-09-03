/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::Display;

use crate::StrandLike;

/// Represents a time interval.
#[derive(Debug, Clone, Copy)]
pub enum Interval {
    /// Year
    Year,
    /// Month
    Month,
    /// Week
    Week,
    /// Day
    Day,
    /// Hour
    Hour,
    /// Minute
    Minute,
    /// Second
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
