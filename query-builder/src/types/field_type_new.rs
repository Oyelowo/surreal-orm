use std::{
    fmt::{self, Display},
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alphanumeric0, alphanumeric1, digit1, space0},
        is_digit,
    },
    combinator::{cut, opt, value},
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult, Parser,
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
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
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
    // Point,
    String,
    Uuid,
    Record(Vec<sql::Table>), // record<user | admin> or record<user> or record
    Geometry(Vec<GeometryTypee>), // geometry<point | line | polygon>
    Option(Box<FieldTypee>), // option<string>
    Union(Vec<FieldTypee>),  // string | int | object
    Set(Box<FieldTypee>, Option<u64>), // set<string, 10>, set<string>, set
    Array(Box<FieldTypee>, Option<u64>), // array<string, 10>, array<string>, array
}

fn parse_option_field_type(input: &str) -> IResult<&str, FieldTypee> {
    let (input, _) = tag("option")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("<")(input)?;
    let (input, _) = space0(input)?;
    let (input, ft) = parse_db_field_type(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(">")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, FieldTypee::Option(Box::new(ft))))
}

fn parse_db_field_type(input: &str) -> IResult<&str, FieldTypee> {
    alt((
        tag("any").map(|_| FieldTypee::Any),
        tag("null").map(|_| FieldTypee::Null),
        tag("bool").map(|_| FieldTypee::Bool),
        tag("bytes").map(|_| FieldTypee::Bytes),
        tag("datetime").map(|_| FieldTypee::Datetime),
        tag("decimal").map(|_| FieldTypee::Decimal),
        tag("duration").map(|_| FieldTypee::Duration),
        tag("float").map(|_| FieldTypee::Float),
        tag("int").map(|_| FieldTypee::Int),
        tag("number").map(|_| FieldTypee::Number),
        tag("object").map(|_| FieldTypee::Object),
        tag("string").map(|_| FieldTypee::String),
        tag("uuid").map(|_| FieldTypee::Uuid),
        parse_record_type,
        // parse_record_type2,
        parse_geometry_type,
        parse_option_field_type,
        parse_array_type,
        // tag("record").map(|_| FieldTypee::Record(vec![])),
        // tag("geometry").map(|_| FieldTypee::Geometry(vec![])),
        // tag("option").map(|_| FieldTypee::Option(Box::new(FieldTypee::Any))),
        // tag("set").map(|_| FieldTypee::Set(Box::new(FieldTypee::Any), None)),
        // tag("array").map(|_| FieldTypee::Array(Box::new(FieldTypee::Any), None)),
    ))(input)
}

fn parse_record_inner(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("<")(input)?;
    let (input, _) = space0(input)?;
    let (input, ref_tables) =
        separated_list0(tag("|"), tuple((space0, alphanumeric1, space0)))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(">")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, ref_tables.iter().map(|t| t.1).collect()))
}

fn parse_record_type(input: &str) -> IResult<&str, FieldTypee> {
    let (input, _) = tag("record")(input)?;
    let (input, rt) = opt(parse_record_inner)(input)?;
    // let (input, rt) = cut(opt(parse_record_inner))(input)?;
    Ok((
        input,
        FieldTypee::Record(
            rt.unwrap_or(vec![])
                .iter()
                .map(|t| t.to_string().into())
                .collect(),
        ),
    ))
}

fn parse_record_type2(input: &str) -> IResult<&str, FieldTypee> {
    let (input, (_, _, ref_tables)) =
        tuple((tag("record"), space0, opt(parse_record_inner)))(input)?;
    Ok((
        input,
        FieldTypee::Record(
            ref_tables
                .unwrap_or(vec![])
                .iter()
                .map(|t| t.to_string().into())
                .collect(),
        ),
    ))
}

fn parse_simple_geom(input: &str) -> IResult<&str, GeometryTypee> {
    alt((
        tag("feature").map(|_| GeometryTypee::Feature),
        tag("point").map(|_| GeometryTypee::Point),
        tag("line").map(|_| GeometryTypee::Line),
        tag("polygon").map(|_| GeometryTypee::Polygon),
        tag("multipoint").map(|_| GeometryTypee::Multipoint),
        tag("multiline").map(|_| GeometryTypee::Multiline),
        tag("multipolygon").map(|_| GeometryTypee::Multipolygon),
        tag("collection").map(|_| GeometryTypee::Collection),
    ))(input)
}

fn parse_geometry_inner(input: &str) -> IResult<&str, Vec<GeometryTypee>> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("<")(input)?;
    let (input, _) = space0(input)?;
    let (input, ref_tables) =
        separated_list0(tag("|"), tuple((space0, parse_simple_geom, space0)))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(">")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, ref_tables.iter().map(|t| t.1.clone()).collect()))
}

fn parse_geometry_type(input: &str) -> IResult<&str, FieldTypee> {
    let (input, _) = tag("geometry")(input)?;
    let (input, rt) = opt(parse_geometry_inner)(input)?;
    // let (input, rt) = cut(opt(parse_record_inner))(input)?;
    Ok((input, FieldTypee::Geometry(rt.unwrap_or(vec![]))))
}

fn parse_array_inner(input: &str) -> IResult<&str, FieldTypee> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("<")(input)?;
    let (input, _) = space0(input)?;
    let (input, field_type) = parse_db_field_type(input)?;
    let (input, _) = space0(input)?;
    let (input, size) = opt(preceded(
        tuple((tag(","), space0)),
        nom::character::complete::u64,
    ))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(">")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, FieldTypee::Array(Box::new(field_type), size)))
}

fn parse_array_type(input: &str) -> IResult<&str, FieldTypee> {
    let (input, _) = tag("array")(input)?;
    let (input, item_type) = opt(parse_array_inner)(input)?;

    if let Some(FieldTypee::Array(item, size)) = item_type {
        Ok((input, FieldTypee::Array(item, size)))
    } else {
        Ok((input, FieldTypee::Array(Box::new(FieldTypee::Any), None)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! test_parse_db_field_type {
        ($name:ident, $input:expr, $output:expr) => {
            paste::paste! {
                #[test]
                fn [< test_field_type_$name >]() {
                    let result = parse_db_field_type($input);
                    let (input, ouput) = result.unwrap();
                    assert_eq!(input, "");
                    assert_eq!(ouput, $output);
                }
            }
        };
    }

    test_parse_db_field_type!(any, "any", FieldTypee::Any);
    test_parse_db_field_type!(null, "null", FieldTypee::Null);
    test_parse_db_field_type!(bool, "bool", FieldTypee::Bool);
    test_parse_db_field_type!(bytes, "bytes", FieldTypee::Bytes);
    test_parse_db_field_type!(datetime, "datetime", FieldTypee::Datetime);
    test_parse_db_field_type!(decimal, "decimal", FieldTypee::Decimal);
    test_parse_db_field_type!(duration, "duration", FieldTypee::Duration);
    test_parse_db_field_type!(flaot, "float", FieldTypee::Float);
    test_parse_db_field_type!(int, "int", FieldTypee::Int);
    test_parse_db_field_type!(number, "number", FieldTypee::Number);
    test_parse_db_field_type!(object, "object", FieldTypee::Object);
    test_parse_db_field_type!(string, "string", FieldTypee::String);
    test_parse_db_field_type!(uuild, "uuid", FieldTypee::Uuid);
    test_parse_db_field_type!(record_any, "record", FieldTypee::Record(vec![]));
    test_parse_db_field_type!(
        record_single_alien,
        "record<alien> ",
        FieldTypee::Record(vec!["alien".into()])
    );
    test_parse_db_field_type!(
        record_spaced,
        "record      < lowo | dayo  |     oye>",
        FieldTypee::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
    );
    test_parse_db_field_type!(
        record_no_space,
        "record<lowo|dayo|oye>",
        FieldTypee::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
    );

    test_parse_db_field_type!(
        geometry_empty_optional,
        "geometry",
        FieldTypee::Geometry(vec![])
    );

    test_parse_db_field_type!(
        geometry_single,
        "geometry<point>",
        FieldTypee::Geometry(vec![GeometryTypee::Point])
    );

    test_parse_db_field_type!(
        geometry_spaced,
        "geometry    < point | line  |     polygon>",
        FieldTypee::Geometry(vec![
            GeometryTypee::Point,
            GeometryTypee::Line,
            GeometryTypee::Polygon
        ])
    );

    test_parse_db_field_type!(
        geometry_no_space,
        "geometry<collection| point|multipolygon|line|polygon>",
        FieldTypee::Geometry(vec![
            GeometryTypee::Collection,
            GeometryTypee::Point,
            GeometryTypee::Multipolygon,
            GeometryTypee::Line,
            GeometryTypee::Polygon
        ])
    );
    test_parse_db_field_type!(
        option_string,
        "option<string>",
        FieldTypee::Option(Box::new(FieldTypee::String))
    );
    test_parse_db_field_type!(
        option_float,
        "option<float>",
        FieldTypee::Option(Box::new(FieldTypee::Float))
    );
    test_parse_db_field_type!(
        option_bool,
        "option<bool>",
        FieldTypee::Option(Box::new(FieldTypee::Bool))
    );
    test_parse_db_field_type!(
        option_decimal,
        "option<decimal>",
        FieldTypee::Option(Box::new(FieldTypee::Decimal))
    );
    test_parse_db_field_type!(
        option_duration,
        "option<duration>",
        FieldTypee::Option(Box::new(FieldTypee::Duration))
    );
    test_parse_db_field_type!(
        option_datetime,
        "option<datetime>",
        FieldTypee::Option(Box::new(FieldTypee::Datetime))
    );
    test_parse_db_field_type!(
        option_uuid,
        "option<uuid>",
        FieldTypee::Option(Box::new(FieldTypee::Uuid))
    );
    test_parse_db_field_type!(
        option_number,
        "option<number>",
        FieldTypee::Option(Box::new(FieldTypee::Number))
    );
    test_parse_db_field_type!(
        option_object,
        "option<object>",
        FieldTypee::Option(Box::new(FieldTypee::Object))
    );
    test_parse_db_field_type!(
        option_bytes,
        "option<bytes>",
        FieldTypee::Option(Box::new(FieldTypee::Bytes))
    );
    test_parse_db_field_type!(
        option_any,
        "option<any>",
        FieldTypee::Option(Box::new(FieldTypee::Any))
    );
    test_parse_db_field_type!(
        option_null,
        "option<null>",
        FieldTypee::Option(Box::new(FieldTypee::Null))
    );
    test_parse_db_field_type!(
        option_geometry,
        "option<geometry>",
        FieldTypee::Option(Box::new(FieldTypee::Geometry(vec![])))
    );
    test_parse_db_field_type!(
        option_geometry_single,
        "option<geometry<point>>",
        FieldTypee::Option(Box::new(FieldTypee::Geometry(vec![GeometryTypee::Point])))
    );
    test_parse_db_field_type!(
        option_geometry_spaced,
        "option<geometry    < point | line  |     polygon>>",
        FieldTypee::Option(Box::new(FieldTypee::Geometry(vec![
            GeometryTypee::Point,
            GeometryTypee::Line,
            GeometryTypee::Polygon
        ])))
    );
    test_parse_db_field_type!(
        option_geometry_no_space,
        "option<geometry<collection| point|multipolygon|line|polygon>>",
        FieldTypee::Option(Box::new(FieldTypee::Geometry(vec![
            GeometryTypee::Collection,
            GeometryTypee::Point,
            GeometryTypee::Multipolygon,
            GeometryTypee::Line,
            GeometryTypee::Polygon
        ])))
    );
    test_parse_db_field_type!(
        option_int,
        "option<int>",
        FieldTypee::Option(Box::new(FieldTypee::Int))
    );
    test_parse_db_field_type!(
        option_simple_record,
        "option<record>",
        FieldTypee::Option(Box::new(FieldTypee::Record(vec![])))
    );
    test_parse_db_field_type!(
        option_record,
        "option<record<alien>>",
        FieldTypee::Option(Box::new(FieldTypee::Record(vec!["alien".into()])))
    );
    test_parse_db_field_type!(
        option_record_spaced,
        "option<record    < lowo | dayo  |     oye>>",
        FieldTypee::Option(Box::new(FieldTypee::Record(vec![
            "lowo".into(),
            "dayo".into(),
            "oye".into()
        ])))
    );
    test_parse_db_field_type!(
        option_record_no_space,
        "option<record<lowo|dayo|oye>>",
        FieldTypee::Option(Box::new(FieldTypee::Record(vec![
            "lowo".into(),
            "dayo".into(),
            "oye".into()
        ])))
    );
    test_parse_db_field_type!(
        option_array,
        "option<array>",
        FieldTypee::Option(Box::new(FieldTypee::Array(Box::new(FieldTypee::Any), None)))
    );
    test_parse_db_field_type!(
        option_array_string,
        "option<array<string>>",
        FieldTypee::Option(Box::new(FieldTypee::Array(
            Box::new(FieldTypee::String),
            None
        )))
    );
    test_parse_db_field_type!(
        option_array_string_10_nospace,
        "option<array<string,10>>",
        FieldTypee::Option(Box::new(FieldTypee::Array(
            Box::new(FieldTypee::String),
            Some(10)
        )))
    );

    test_parse_db_field_type!(
        option_array_string_10,
        "option<array<string, 10>>",
        FieldTypee::Option(Box::new(FieldTypee::Array(
            Box::new(FieldTypee::String),
            Some(10)
        )))
    );
    test_parse_db_field_type!(
        option_array_string_10_spaced,
        "option<array    < string , 10> >",
        FieldTypee::Option(Box::new(FieldTypee::Array(
            Box::new(FieldTypee::String),
            Some(10)
        )))
    );
    test_parse_db_field_type!(
        option_array_string_10_spaced2,
        "option   <  array    < string ,   10> >",
        FieldTypee::Option(Box::new(FieldTypee::Array(
            Box::new(FieldTypee::String),
            Some(10)
        )))
    );
    // parse for array
    test_parse_db_field_type!(
        array,
        "array",
        FieldTypee::Array(Box::new(FieldTypee::Any), None)
    );
    test_parse_db_field_type!(
        array_string,
        "array<string>",
        FieldTypee::Array(Box::new(FieldTypee::String), None)
    );
    test_parse_db_field_type!(
        array_string_10_nospace,
        "array<string,69>",
        FieldTypee::Array(Box::new(FieldTypee::String), Some(69))
    );
    test_parse_db_field_type!(
        array_string_10,
        "array<string, 10>",
        FieldTypee::Array(Box::new(FieldTypee::String), Some(10))
    );
    test_parse_db_field_type!(
        array_string_10_spaced,
        "array    < string , 10> ",
        FieldTypee::Array(Box::new(FieldTypee::String), Some(10))
    );

    ///////

    #[test]
    fn test_parse_record_type_empty_optional() {
        let result = parse_record_type("record");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(ouput, FieldTypee::Record(vec![]));
    }

    #[test]
    fn test_parse_record_type_single() {
        let result = parse_record_type("record<alien>");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(ouput, FieldTypee::Record(vec!["alien".into()]));
    }

    #[test]
    fn test_parse_record_type_spaced() {
        let result = parse_record_type("record    < lowo | dayo  |     oye>");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(
            ouput,
            FieldTypee::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
        );
    }

    #[test]
    fn test_parse_record_type_no_space() {
        let result = parse_record_type("record<lowo|dayo|oye>");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(
            ouput,
            FieldTypee::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
        );
    }

    #[test]
    fn test_parse_record_type_no_space2() {
        let result = parse_record_type2("record<lowo|dayo|oye>");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(
            ouput,
            FieldTypee::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
        );
    }

    /////// parse record upper level
    #[test]
    fn test_parse_record_type_empty_optional_field_type() {
        let result = parse_db_field_type("record");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(ouput, FieldTypee::Record(vec![]));
    }

    #[test]
    fn test_parse_record_type_single_field_type() {
        let result = parse_db_field_type("record<alien>");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(ouput, FieldTypee::Record(vec!["alien".into()]));
    }

    #[test]
    fn test_parse_record_type_spaced_field_type() {
        let result = parse_db_field_type("record    < lowo | dayo  |     oye>");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(
            ouput,
            FieldTypee::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
        );
    }

    #[test]
    fn test_parse_record_type_no_space_field_type() {
        let result = parse_db_field_type("record<lowo|dayo|oye>");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(
            ouput,
            FieldTypee::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
        );
    }

    #[test]
    fn test_parse_record_type_no_space2_field_type() {
        let result = parse_db_field_type("record<lowo|dayo|oye>");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(
            ouput,
            FieldTypee::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
        );
    }

    /// Parse geometry type
    #[test]
    fn test_parse_geometry_type_empty_optional() {
        let result = parse_db_field_type("geometry");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(ouput, FieldTypee::Geometry(vec![]));
    }

    #[test]
    fn test_parse_geometry_type_single() {
        let result = parse_db_field_type("geometry<point>");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(ouput, FieldTypee::Geometry(vec![GeometryTypee::Point]));
    }

    #[test]
    fn test_parse_geometry_type_spaced() {
        let result = parse_db_field_type("geometry    < point | line  |     polygon>");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(
            ouput,
            FieldTypee::Geometry(vec![
                GeometryTypee::Point,
                GeometryTypee::Line,
                GeometryTypee::Polygon
            ])
        );
    }

    #[test]
    fn test_parse_geometry_type_no_space() {
        let result = parse_db_field_type("geometry<collection| point|multipolygon|line|polygon>");
        let (input, ouput) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(
            ouput,
            FieldTypee::Geometry(vec![
                GeometryTypee::Collection,
                GeometryTypee::Point,
                GeometryTypee::Multipolygon,
                GeometryTypee::Line,
                GeometryTypee::Polygon
            ])
        );
    }

    // #[test]
    // fn test_parse_record_type_no_space_prefixed() {
    //     let result = parse_record_type("record<|lowo|dayo|oye>");
    //     let (input, ouput) = result.unwrap();
    //     assert_eq!(input, "<<|lowo|dayo|oye>");
    //     assert_eq!(
    //         ouput,
    //         FieldTypee::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
    //     );
    // }
}
