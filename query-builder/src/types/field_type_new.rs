use std::{
    fmt::{self, Display},
    str::FromStr,
};

/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
// Data types
// SurrealQL allows you to describe data with specific data types.
//
// Type	Description
// any	Use this when you explicitly don't want to specify the field's data type. The field will allow any data type supported by SurrealDB.
// array	An array of items. The array type also allows you to define which types can be stored in the array and the max length.
// array
// array<string>
// array<string, 10>
// set	An set of items. The array type also allows you to define which types can be stored in the array and the max length. Items are automatically deduplicated.
// set
// set<string>
// set<string, 10>
// bool	Describes whether something is thruthy or not.
// datetime	An ISO 8601 compliant data type that stores a date with time and time zone.
// decimal	Uses BigDecimal for storing any real number with arbitrary precision.
// duration	Store a value representing a length of time. Can be added or subtracted from datetimes or other durations.
// float	Store a value in a 64 bit float.
// int	Store a value in a 64 bit integer.
// number	Store numbers without specifying the type. SurrealDB will detect the type of number and store it using the minimal number of bytes. For numbers passed in as a string, this field will store the number in a BigDecimal.
// object	Store formatted objects containing values of any supported type with no limit to object depth or nesting.
// option	Makes types optional and guarantees the field to be either empty (NONE), or a number.
// option<number>
// string	Describes a text-like value.
// record	Store a reference to another record. The value must be a Record ID.
// record
// record<string>
// record<string | number>
// geometry	RFC 7946 compliant data type for storing geometry in the GeoJson format.
// Geometric Types include:
// geometry<feature>
// geometry<point>
// geometry<line>
// geometry<polygon>
// geometry<multipoint>
// geometry<multiline>
// geometry<multipolygon>
// geometry<collection>
// -- Define a field with a single type
// DEFINE FIELD location ON TABLE restaurant TYPE geometry<point>;
// -- Define a field with any geometric type
// DEFINE FIELD area ON TABLE restaurant TYPE geometry<feature>;
// -- Define a field with specific geometric types
// DEFINE FIELD area ON TABLE restaurant TYPE geometry<polygon|multipolygon|collection>;
use serde::{Deserialize, Serialize};
use surrealdb::sql;

use crate::Table;

/// Geometry types supported by surrealdb
#[derive(Debug, Clone)]
pub enum GeometryTypee {
    /// Define a field with any geometric type
    Feature,
    /// Define a field with point geometric type
    Point,
    /// Define a field with line geometric type
    Line,
    /// Define a field with polygon geometric type
    Polygon,
    /// Define a field with multipoint geometric type
    Multipoint,
    /// Define a field with multiline geometric type
    Multiline,
    /// Define a field with multpipolygon geometric type
    Multipolygon,
    /// Define a field with collection of geometry types
    Collection,
}

impl FromStr for GeometryTypee {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "feature" => Ok(Self::Feature),
            "point" => Ok(Self::Point),
            "line" => Ok(Self::Line),
            "polygon" => Ok(Self::Polygon),
            "multipoint" => Ok(Self::Multipoint),
            "multiline" => Ok(Self::Multiline),
            "multipolygon" => Ok(Self::Multipolygon),
            "collection" => Ok(Self::Collection),
            _ => Err(format!("Invalid geometry type: {}", s)),
        }
    }
}

impl Display for GeometryTypee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let geom = match self {
            GeometryTypee::Feature => "feature",
            GeometryTypee::Point => "point",
            GeometryTypee::Line => "line",
            GeometryTypee::Polygon => "polygon",
            GeometryTypee::Multipoint => "multipoint",
            GeometryTypee::Multiline => "multiline",
            GeometryTypee::Multipolygon => "multipolygon",
            GeometryTypee::Collection => "collection",
        };
        write!(f, "{}", geom)
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub enum FieldTypee {
    Any,
    Null,
    Bool,
    Bytes,
    Datetime,
    Decimal,
    Duration,
    Float,
    Int,
    Number,
    Object,
    Point,
    String,
    Uuid,
    // Record(Option<sql::Table>),
    Record(Vec<sql::Table>),
    Geometry(Vec<String>),
    Option(Box<FieldTypee>),
    // Either(Vec<FieldTypee>),
    Union(Vec<FieldTypee>),
    Set(Box<FieldTypee>, Option<u64>),
    Array(Box<FieldTypee>, Option<u64>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_type() {
        let field_type = FieldTypee::Any;
        assert_eq!(field_type, FieldTypee::Any);
    }
}
