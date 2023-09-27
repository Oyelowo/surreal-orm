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
use std::{
    fmt::{self, Display},
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0},
    combinator::{opt, value},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, tuple},
    IResult, Parser,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql;

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
    String,
    Uuid,
    Record(Vec<sql::Table>), // record<user | admin> or record<user> or record
    Geometry(Vec<GeometryTypee>), // geometry<point | line | polygon>
    Option(Box<FieldTypee>), // option<string>
    Union(Vec<FieldTypee>),  // string | int | object
    Set(Box<FieldTypee>, Option<u64>), // set<string, 10>, set<string>, set
    Array(Box<FieldTypee>, Option<u64>), // array<string, 10>, array<string>, array
}

impl FromStr for FieldTypee {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_db_field_type(s)
            .map(|(_, ft)| ft)
            .map_err(|e| format!("{:?}", e))
    }
}

impl Display for FieldTypee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldTypee::Any => write!(f, "any"),
            FieldTypee::Null => write!(f, "null"),
            FieldTypee::Bool => write!(f, "bool"),
            FieldTypee::Bytes => write!(f, "bytes"),
            FieldTypee::Datetime => write!(f, "datetime"),
            FieldTypee::Decimal => write!(f, "decimal"),
            FieldTypee::Duration => write!(f, "duration"),
            FieldTypee::Float => write!(f, "float"),
            FieldTypee::Int => write!(f, "int"),
            FieldTypee::Number => write!(f, "number"),
            FieldTypee::Object => write!(f, "object"),
            FieldTypee::String => write!(f, "string"),
            FieldTypee::Uuid => write!(f, "uuid"),
            FieldTypee::Record(ref_tables) => {
                if ref_tables.is_empty() {
                    write!(f, "record")
                } else {
                    write!(
                        f,
                        "record<{}>",
                        ref_tables
                            .iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join("|")
                    )
                }
            }
            FieldTypee::Geometry(ref_tables) => {
                if ref_tables.is_empty() {
                    write!(f, "geometry")
                } else {
                    write!(
                        f,
                        "geometry<{}>",
                        ref_tables
                            .iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join("|")
                    )
                }
            }
            FieldTypee::Option(ft) => write!(f, "option<{}>", ft),
            FieldTypee::Union(ref_tables) => write!(
                f,
                "{}",
                ref_tables
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" | ")
            ),
            FieldTypee::Set(ft, size) => {
                if let Some(size) = size {
                    write!(f, "set<{}, {}>", ft, size)
                } else {
                    write!(f, "set<{}>", ft)
                }
            }
            FieldTypee::Array(ft, size) => {
                if let Some(size) = size {
                    write!(f, "array<{}, {}>", ft, size)
                } else {
                    write!(f, "array<{}>", ft)
                }
            }
        }
    }
}

impl FieldTypee {
    /// Returns true if the field type is a primitive type
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            FieldTypee::Any
                | FieldTypee::Null
                | FieldTypee::Bool
                | FieldTypee::Bytes
                | FieldTypee::Datetime
                | FieldTypee::Decimal
                | FieldTypee::Duration
                | FieldTypee::Float
                | FieldTypee::Int
                | FieldTypee::Number
                | FieldTypee::Object
                | FieldTypee::String
                | FieldTypee::Uuid
        )
    }

    /// Returns true if the field type is a record type
    pub fn is_record(&self) -> bool {
        matches!(self, FieldTypee::Record(_))
    }

    /// Returns true if the field type is a geometry type
    pub fn is_geometry(&self) -> bool {
        matches!(self, FieldTypee::Geometry(_))
    }

    /// Returns true if the field type is an option type
    pub fn is_option(&self) -> bool {
        matches!(self, FieldTypee::Option(_))
    }

    /// Returns true if the field type is a union type
    pub fn is_union(&self) -> bool {
        matches!(self, FieldTypee::Union(_))
    }

    /// Returns true if the field type is a set type
    pub fn is_set(&self) -> bool {
        matches!(self, FieldTypee::Set(_, _))
    }

    /// Returns true if the field type is an array type
    pub fn is_array(&self) -> bool {
        matches!(self, FieldTypee::Array(_, _))
    }

    /// Returns true if the field type is a collection type
    pub fn is_collection(&self) -> bool {
        matches!(self, FieldTypee::Array(_, _) | FieldTypee::Set(_, _))
    }

    /// Returns true if the field type is a list type
    pub fn is_list(&self) -> bool {
        matches!(self, FieldTypee::Array(_, _) | FieldTypee::Set(_, _))
    }

    /// Returns true if the field type is a numeric type
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            FieldTypee::Decimal | FieldTypee::Float | FieldTypee::Int | FieldTypee::Number
        )
    }

    /// Returns true if the field type is a string type
    pub fn is_string(&self) -> bool {
        matches!(self, FieldTypee::String)
    }

    /// Returns true if the field type is a boolean type
    pub fn is_bool(&self) -> bool {
        matches!(self, FieldTypee::Bool)
    }

    /// Returns true if the field type is a bytes type
    pub fn is_bytes(&self) -> bool {
        matches!(self, FieldTypee::Bytes)
    }

    /// Returns true if the field type is a datetime type
    pub fn is_datetime(&self) -> bool {
        matches!(self, FieldTypee::Datetime)
    }

    /// Returns true if the field type is a duration type
    pub fn is_duration(&self) -> bool {
        matches!(self, FieldTypee::Duration)
    }

    /// Returns true if the field type is a uuid type
    pub fn is_uuid(&self) -> bool {
        matches!(self, FieldTypee::Uuid)
    }

    /// Returns true if the field type is an object type
    pub fn is_object(&self) -> bool {
        matches!(self, FieldTypee::Object)
    }

    /// Returns true if the field type is a null type
    pub fn is_null(&self) -> bool {
        matches!(self, FieldTypee::Null)
    }

    /// Returns true if the field type is an any type
    pub fn is_any(&self) -> bool {
        matches!(self, FieldTypee::Any)
    }

    /// Returns true if the field type is a number type
    pub fn is_number(&self) -> bool {
        matches!(self, FieldTypee::Number)
    }

    /// Returns true if the field type is a float type
    pub fn is_float(&self) -> bool {
        matches!(self, FieldTypee::Float)
    }

    /// Returns true if the field type is an int type
    pub fn is_int(&self) -> bool {
        matches!(self, FieldTypee::Int)
    }

    /// Returns true if the field type is a decimal type
    pub fn is_decimal(&self) -> bool {
        matches!(self, FieldTypee::Decimal)
    }

    /// Returns true if the field type is a record type with no reference tables
    pub fn is_empty_record(&self) -> bool {
        matches!(self, FieldTypee::Record(ref_tables) if ref_tables.is_empty())
    }

    /// Returns true if the field type is a geometry type with no reference tables
    pub fn is_empty_geometry(&self) -> bool {
        matches!(self, FieldTypee::Geometry(ref_tables) if ref_tables.is_empty())
    }
}

/// Parses a field type
/// ```
/// use surrealdb::{FieldTypee, parse_db_field_type};
/// assert_eq!(parse_db_field_type("any"), Ok(("", FieldTypee::Any)));
/// assert_eq!(parse_db_field_type("null"), Ok(("", FieldTypee::Null)));
/// assert_eq!(parse_db_field_type("bool"), Ok(("", FieldTypee::Bool)));
/// assert_eq!(parse_db_field_type("option<string>"), Ok(("", FieldTypee::Option(Box::new(FieldTypee::String)))));
/// ```
pub fn parse_db_field_type(input: &str) -> IResult<&str, FieldTypee> {
    alt((parse_union_type, parse_option_field_type))(input)
}

fn parse_pipe(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, ()))
}

fn parse_single_field_type(input: &str) -> IResult<&str, FieldTypee> {
    alt((
        parse_primitive_type,
        parse_record_type,
        // parse_record_type2,
        parse_geometry_type,
        parse_option_field_type,
        parse_array_type,
        parse_set_type,
        // parse_union_type,
    ))(input)
}

fn parse_union_type(input: &str) -> IResult<&str, FieldTypee> {
    // let (input, ft) = separated_list1(parse_pipe, parse_db_field_type)(input)?;
    let (input, mut ft) = separated_list1(parse_pipe, alt((parse_single_field_type,)))(input)?;
    let ft = match ft.len() {
        1 => ft.remove(0),
        _ => FieldTypee::Union(ft),
    };
    Ok((input, ft))
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

// fn surrounded_by_spaces<'a, O, F>(parser: F) -> impl Fn(&'a str) -> IResult<&'a str, O>
// where
//     F: Fn(&'a str) -> IResult<&'a str, O>,
// {
//     move |input: &'a str| {
//         let (input, _) = multispace0(input)?;
//         let (input, value) = parser(input)?;
//         let (input, _) = multispace0(input)?;
//         Ok((input, value))
//     }
// }

fn parse_primitive_type(input: &str) -> IResult<&str, FieldTypee> {
    alt((
        value(FieldTypee::Any, tag("any")),
        value(FieldTypee::Null, tag("null")),
        value(FieldTypee::Bool, tag("bool")),
        value(FieldTypee::Bytes, tag("bytes")),
        value(FieldTypee::Datetime, tag("datetime")),
        value(FieldTypee::Decimal, tag("decimal")),
        value(FieldTypee::Duration, tag("duration")),
        value(FieldTypee::Float, tag("float")),
        value(FieldTypee::Int, tag("int")),
        value(FieldTypee::Number, tag("number")),
        value(FieldTypee::Object, tag("object")),
        value(FieldTypee::String, tag("string")),
        value(FieldTypee::Uuid, tag("uuid")),
        // tag("null").map(|_| FieldTypee::Null),
        // tag("bool").map(|_| FieldTypee::Bool),
        // tag("bytes").map(|_| FieldTypee::Bytes),
        // tag("datetime").map(|_| FieldTypee::Datetime),
        // tag("decimal").map(|_| FieldTypee::Decimal),
        // tag("duration").map(|_| FieldTypee::Duration),
        // tag("float").map(|_| FieldTypee::Float),
        // tag("int").map(|_| FieldTypee::Int),
        // tag("number").map(|_| FieldTypee::Number),
        // tag("object").map(|_| FieldTypee::Object),
        // tag("string").map(|_| FieldTypee::String),
        // tag("uuid").map(|_| FieldTypee::Uuid),
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

// fn parse_record_type2(input: &str) -> IResult<&str, FieldTypee> {
//     let (input, (_, _, ref_tables)) =
//         tuple((tag("record"), space0, opt(parse_record_inner)))(input)?;
//     Ok((
//         input,
//         FieldTypee::Record(
//             ref_tables
//                 .unwrap_or(vec![])
//                 .iter()
//                 .map(|t| t.to_string().into())
//                 .collect(),
//         ),
//     ))
// }

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

struct ListItem {
    item_type: FieldTypee,
    size: Option<u64>,
}
fn parse_list_inner(input: &str) -> IResult<&str, ListItem> {
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
    Ok((
        input,
        ListItem {
            item_type: field_type,
            size,
        },
    ))
}

fn parse_array_type(input: &str) -> IResult<&str, FieldTypee> {
    let (input, _) = tag("array")(input)?;
    let (input, item_type) = opt(parse_list_inner)(input)?;

    if let Some(ListItem { item_type, size }) = item_type {
        Ok((input, FieldTypee::Array(Box::new(item_type), size)))
    } else {
        Ok((input, FieldTypee::Array(Box::new(FieldTypee::Any), None)))
    }
}

fn parse_set_type(input: &str) -> IResult<&str, FieldTypee> {
    let (input, _) = tag("set")(input)?;
    let (input, item_type) = opt(parse_list_inner)(input)?;

    if let Some(ListItem { item_type, size }) = item_type {
        Ok((input, FieldTypee::Set(Box::new(item_type), size)))
    } else {
        Ok((input, FieldTypee::Set(Box::new(FieldTypee::Any), None)))
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

    // Union/Either
    test_parse_db_field_type!(
        union_string_int,
        "string|int",
        FieldTypee::Union(vec![FieldTypee::String, FieldTypee::Int])
    );

    // Array
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
        array_any,
        "array",
        FieldTypee::Array(Box::new(FieldTypee::Any), None)
    );
    test_parse_db_field_type!(
        array_string,
        "array<string>",
        FieldTypee::Array(Box::new(FieldTypee::String), None)
    );
    test_parse_db_field_type!(
        array_object_10_nospace,
        "array<object,69>",
        FieldTypee::Array(Box::new(FieldTypee::Object), Some(69))
    );
    test_parse_db_field_type!(
        array_geometry_10,
        "array<geometry<point | polygon | multipolygon>, 10>",
        FieldTypee::Array(
            Box::new(FieldTypee::Geometry(vec![
                GeometryTypee::Point,
                GeometryTypee::Polygon,
                GeometryTypee::Multipolygon
            ])),
            Some(10)
        )
    );
    test_parse_db_field_type!(
        array_null_10_spaced,
        "array    < null , 10> ",
        FieldTypee::Array(Box::new(FieldTypee::Null), Some(10))
    );
    test_parse_db_field_type!(
        array_nested_array,
        "array<array<float, 42> , 10> ",
        FieldTypee::Array(
            Box::new(FieldTypee::Array(Box::new(FieldTypee::Float), Some(42))),
            Some(10)
        )
    );

    // SET TESTS
    test_parse_db_field_type!(
        set_any,
        "set",
        FieldTypee::Set(Box::new(FieldTypee::Any), None)
    );
    test_parse_db_field_type!(
        set_string,
        "set<string>",
        FieldTypee::Set(Box::new(FieldTypee::String), None)
    );
    test_parse_db_field_type!(
        set_object_10_nospace,
        "set<object,69>",
        FieldTypee::Set(Box::new(FieldTypee::Object), Some(69))
    );
    test_parse_db_field_type!(
        set_geometry_10,
        "set<geometry<point | polygon | multipolygon>, 10>",
        FieldTypee::Set(
            Box::new(FieldTypee::Geometry(vec![
                GeometryTypee::Point,
                GeometryTypee::Polygon,
                GeometryTypee::Multipolygon
            ])),
            Some(10)
        )
    );
    test_parse_db_field_type!(
        set_null_10_spaced,
        "set    < null , 10> ",
        FieldTypee::Set(Box::new(FieldTypee::Null), Some(10))
    );
    test_parse_db_field_type!(
        set_nested_array,
        "set<set<float, 42> , 10> ",
        FieldTypee::Set(
            Box::new(FieldTypee::Set(Box::new(FieldTypee::Float), Some(42))),
            Some(10)
        )
    );

    /////// Test Union
    // test_parse_db_field_type!(union_any_single, "any", FieldTypee::Any);

    test_parse_db_field_type!(
        union_any_null,
        "any | null",
        FieldTypee::Union(vec![FieldTypee::Any, FieldTypee::Null])
    );

    test_parse_db_field_type!(
        union_any_null_bool,
        "any | null | bool",
        FieldTypee::Union(vec![FieldTypee::Any, FieldTypee::Null, FieldTypee::Bool])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes,
        "any | null | bool | bytes",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime,
        "any | null | bool | bytes | datetime",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal,
        "any | null | bool | bytes | datetime | decimal",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration,
        "any | null | bool | bytes | datetime | decimal | duration",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float,
        "any | null | bool | bytes | datetime | decimal | duration | float",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration,
            FieldTypee::Float
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int,
        "any | null | bool | bytes | datetime | decimal | duration | float | int",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration,
            FieldTypee::Float,
            FieldTypee::Int
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration,
            FieldTypee::Float,
            FieldTypee::Int,
            FieldTypee::Number
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration,
            FieldTypee::Float,
            FieldTypee::Int,
            FieldTypee::Number,
            FieldTypee::Object
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration,
            FieldTypee::Float,
            FieldTypee::Int,
            FieldTypee::Number,
            FieldTypee::Object,
            FieldTypee::String
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration,
            FieldTypee::Float,
            FieldTypee::Int,
            FieldTypee::Number,
            FieldTypee::Object,
            FieldTypee::String,
            FieldTypee::Uuid
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration,
            FieldTypee::Float,
            FieldTypee::Int,
            FieldTypee::Number,
            FieldTypee::Object,
            FieldTypee::String,
            FieldTypee::Uuid,
            FieldTypee::Record(vec![])
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration,
            FieldTypee::Float,
            FieldTypee::Int,
            FieldTypee::Number,
            FieldTypee::Object,
            FieldTypee::String,
            FieldTypee::Uuid,
            FieldTypee::Record(vec![]),
            FieldTypee::Geometry(vec![])
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry_array,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry | array",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration,
            FieldTypee::Float,
            FieldTypee::Int,
            FieldTypee::Number,
            FieldTypee::Object,
            FieldTypee::String,
            FieldTypee::Uuid,
            FieldTypee::Record(vec![]),
            FieldTypee::Geometry(vec![]),
            FieldTypee::Array(Box::new(FieldTypee::Any), None)
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry_array_set,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry | array | set",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration,
            FieldTypee::Float,
            FieldTypee::Int,
            FieldTypee::Number,
            FieldTypee::Object,
            FieldTypee::String,
            FieldTypee::Uuid,
            FieldTypee::Record(vec![]),
            FieldTypee::Geometry(vec![]),
            FieldTypee::Array(Box::new(FieldTypee::Any), None),
            FieldTypee::Set(Box::new(FieldTypee::Any), None)
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry_array_set_option,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry | array | set | option<int>",
        FieldTypee::Union(vec![
            FieldTypee::Any,
            FieldTypee::Null,
            FieldTypee::Bool,
            FieldTypee::Bytes,
            FieldTypee::Datetime,
            FieldTypee::Decimal,
            FieldTypee::Duration,
            FieldTypee::Float,
            FieldTypee::Int,
            FieldTypee::Number,
            FieldTypee::Object,
            FieldTypee::String,
            FieldTypee::Uuid,
            FieldTypee::Record(vec![]),
            FieldTypee::Geometry(vec![]),
            FieldTypee::Array(Box::new(FieldTypee::Any), None),
            FieldTypee::Set(Box::new(FieldTypee::Any), None),
            FieldTypee::Option(Box::new(FieldTypee::Int))
        ])
    );

    test_parse_db_field_type!(
        union_array_of_string_and_int_or_null,
        "array<string|int|null>",
        FieldTypee::Array(
            Box::new(FieldTypee::Union(vec![
                FieldTypee::String,
                FieldTypee::Int,
                FieldTypee::Null
            ])),
            None
        )
    );

    test_parse_db_field_type!(
        union_array_of_string_and_int_or_null_10,
        "int | option<float> | array<option<string>|int|null, 10> | set<option<number>|float|null, 10> | option<array> | option<set<option<int>>>",
        FieldTypee::Union(vec![
            FieldTypee::Int,
            FieldTypee::Option(Box::new(FieldTypee::Float)),
            FieldTypee::Array(
                Box::new(FieldTypee::Union(vec![
                    FieldTypee::Option(Box::new(FieldTypee::String)),
                    FieldTypee::Int,
                    FieldTypee::Null
                ])),
                Some(10)
            ),
            FieldTypee::Set(
                Box::new(FieldTypee::Union(vec![
                    FieldTypee::Option(Box::new(FieldTypee::Number)),
                    FieldTypee::Float,
                    FieldTypee::Null
                ])),
                Some(10)
            ),
            FieldTypee::Option(Box::new(FieldTypee::Array(
                Box::new(FieldTypee::Any),
                None
            ))),
            FieldTypee::Option(Box::new(FieldTypee::Set(
                Box::new(FieldTypee::Option(Box::new(FieldTypee::Int))),
                None
            )))
        ])
    );
}
