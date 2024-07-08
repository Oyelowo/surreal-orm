/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
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
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    combinator::{all_consuming, cut, opt, value},
    error::context,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, tuple},
    IResult, Parser,
};
use serde::{Deserialize, Serialize};

/// Geometry types supported by surrealdb
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub enum GeometryType {
    /// Define a field with any geometric type
    Feature,
    /// Define a field with point geometric type
    Point,
    /// Define a field with line geometric type
    LineString,
    /// Define a field with polygon geometric type
    Polygon,
    /// Define a field with multipoint geometric type
    MultiPoint,
    /// Define a field with multiline geometric type
    MultiLine,
    /// Define a field with multpipolygon geometric type
    MultiPolygon,
    /// Define a field with collection of geometry types
    Collection,
}

impl FromStr for GeometryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "feature" => Ok(Self::Feature),
            "point" => Ok(Self::Point),
            "LineString" => Ok(Self::LineString),
            "polygon" => Ok(Self::Polygon),
            "multipoint" => Ok(Self::MultiPoint),
            "multiline" => Ok(Self::MultiLine),
            "multipolygon" => Ok(Self::MultiPolygon),
            "collection" => Ok(Self::Collection),
            _ => Err(format!("Invalid geometry type: {}", s)),
        }
    }
}

impl Display for GeometryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let geom = match self {
            GeometryType::Feature => "feature",
            GeometryType::Point => "point",
            GeometryType::LineString => "LineString",
            GeometryType::Polygon => "polygon",
            GeometryType::MultiPoint => "multipoint",
            GeometryType::MultiLine => "multiline",
            GeometryType::MultiPolygon => "multipolygon",
            GeometryType::Collection => "collection",
        };
        write!(f, "{}", geom)
    }
}

#[allow(missing_docs)]
#[derive(Clone, Default, Debug, PartialEq)]
pub enum FieldType {
    /// Use this when you explicitly don't want to specify the field's data type. The field will
    /// allow any data type supported by SurrealDB.
    #[default]
    Any,
    Null,
    /// true of false
    Bool,
    Bytes,
    /// An ISO 8601 compliant data type that stores a date with time and time zone.
    Datetime,
    /// Uses BigDecimal for storing any real number with arbitrary precision.
    Decimal,
    /// Store a value representing a length of time. Can be added or subtracted from datetimes or
    /// other durations.
    Duration,
    /// Store a value in a 64 bit float.
    Float,
    /// Store a value in a 64 bit integer.
    Int,
    /// Store numbers without specifying the type. SurrealDB will detect the type of number and
    /// store it using the minimal number of bytes. For numbers passed in as a string, this field
    /// will store the number in a BigDecimal.
    Number,
    /// Store formatted objects containing values of any supported type with no limit to object
    /// depth or nesting.
    Object,
    String,
    Uuid,
    /// Store a reference to another record. The value must be a Record ID.
    Record(Vec<crate::Table>), // record<user | admin> or record<user> or record
    /// RFC 7946 compliant data type for storing geometry in the GeoJson format.
    Geometry(Vec<GeometryType>), // geometry<point | line | polygon>
    Option(Box<FieldType>),           // option<string>
    Union(Vec<FieldType>),            // string | int | object
    Set(Box<FieldType>, Option<u64>), // set<string, 10>, set<string>, set
    /// a list
    Array(Box<FieldType>, Option<u64>), // array<string, 10>, array<string>, array
}

impl FromStr for FieldType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //         let error = format!(
        //     "Invalid type. Expected one of - `{:?}`",
        //     FieldType::variants()
        // );

        parse_field_type(s)
            .map(|(_, ft)| ft)
            .map_err(|e| format!("{:?}", e))
    }
}

impl Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::Any => write!(f, "any"),
            FieldType::Null => write!(f, "null"),
            FieldType::Bool => write!(f, "bool"),
            FieldType::Bytes => write!(f, "bytes"),
            FieldType::Datetime => write!(f, "datetime"),
            FieldType::Decimal => write!(f, "decimal"),
            FieldType::Duration => write!(f, "duration"),
            FieldType::Float => write!(f, "float"),
            FieldType::Int => write!(f, "int"),
            FieldType::Number => write!(f, "number"),
            FieldType::Object => write!(f, "object"),
            FieldType::String => write!(f, "string"),
            FieldType::Uuid => write!(f, "uuid"),
            FieldType::Record(ref_tables) => {
                if ref_tables.is_empty() {
                    write!(f, "record<any>")
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
            FieldType::Geometry(ref_tables) => {
                if ref_tables.is_empty() {
                    write!(f, "geometry<feature>")
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
            FieldType::Option(ft) => write!(f, "option<{}>", ft),
            FieldType::Union(ref_tables) => write!(
                f,
                "{}",
                ref_tables
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" | ")
            ),
            FieldType::Set(ft, size) => {
                if let Some(size) = size {
                    write!(f, "set<{}, {}>", ft, size)
                } else {
                    write!(f, "set<{}>", ft)
                }
            }
            FieldType::Array(ft, size) => {
                if let Some(size) = size {
                    write!(f, "array<{}, {}>", ft, size)
                } else {
                    write!(f, "array<{}>", ft)
                }
            }
        }
    }
}

impl FieldType {
    /// Returns a list of all the variants of the enum
    pub fn variants() -> Vec<&'static str> {
        vec![
            "any",
            "null",
            "array<string>",
            "set<string>",
            "bool",
            "datetime",
            "decimal",
            "duration",
            "float",
            "int",
            "number",
            "object",
            "string",
            "record<user>",
            "geometry<point>",
            "option<string>",
            "string | number",
        ]
    }

    /// Returns true if the field_type is a record
    pub fn is_record_any(&self) -> bool {
        matches!(self, Self::Record(_))
    }

    /// Returns true if the field_type is a record
    pub fn is_record_of_the_table(&self, table: &String) -> bool {
        if table.is_empty() {
            return false;
        }
        matches!(self, Self::Record(t) if &t.first().map(ToString::to_string).unwrap_or_default() == table)
    }

    /// Returns true if the field type is a primitive type
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            // FieldType::Any
            |FieldType::Null| FieldType::Bool
                | FieldType::Bytes
                | FieldType::Datetime
                | FieldType::Decimal
                | FieldType::Duration
                | FieldType::Float
                | FieldType::Int
                | FieldType::Number
                | FieldType::Object
                | FieldType::String
                | FieldType::Uuid
        )
    }

    /// Returns true if the field type is a record type
    pub fn is_record(&self) -> bool {
        matches!(self, FieldType::Record(_))
    }

    /// Returns true if the field type is a geometry type
    pub fn is_geometry(&self) -> bool {
        matches!(self, FieldType::Geometry(_))
    }

    /// Returns true if the field type is an option type
    pub fn is_option(&self) -> bool {
        matches!(self, FieldType::Option(_))
    }

    /// Returns true if the field type is a union type
    pub fn is_union(&self) -> bool {
        matches!(self, FieldType::Union(_))
    }

    /// Returns true if the field type is a set type
    pub fn is_set(&self) -> bool {
        matches!(self, FieldType::Set(_, _))
    }

    /// Returns true if the field type is an array type
    pub fn is_array(&self) -> bool {
        matches!(self, FieldType::Array(_, _))
    }

    /// Returns true if the field type is a collection type
    pub fn is_collection(&self) -> bool {
        matches!(self, FieldType::Array(_, _) | FieldType::Set(_, _))
    }

    /// Returns true if the field type is a list type
    // TODO: Remove this?
    // pub fn is_list(&self) -> bool {
    //     matches!(self, FieldType::Array(_, _) | FieldType::Set(_, _))
    // }

    /// Returns true if the field type is a numeric type
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            FieldType::Decimal | FieldType::Float | FieldType::Int | FieldType::Number
        )
    }

    /// Returns true if the field type is a string type
    pub fn is_string(&self) -> bool {
        matches!(self, FieldType::String)
    }

    /// Returns true if the field type is a boolean type
    pub fn is_bool(&self) -> bool {
        matches!(self, FieldType::Bool)
    }

    /// Returns true if the field type is a bytes type
    pub fn is_bytes(&self) -> bool {
        matches!(self, FieldType::Bytes)
    }

    /// Returns true if the field type is a datetime type
    pub fn is_datetime(&self) -> bool {
        matches!(self, FieldType::Datetime)
    }

    /// Returns true if the field type is a duration type
    pub fn is_duration(&self) -> bool {
        matches!(self, FieldType::Duration)
    }

    /// Returns true if the field type is a uuid type
    pub fn is_uuid(&self) -> bool {
        matches!(self, FieldType::Uuid)
    }

    /// Returns true if the field type is an object type
    pub fn is_object(&self) -> bool {
        matches!(self, FieldType::Object)
    }

    /// Returns true if the field type is a null type
    pub fn is_null(&self) -> bool {
        matches!(self, FieldType::Null)
    }

    /// Returns true if the field type is an any type
    pub fn is_any(&self) -> bool {
        matches!(self, FieldType::Any)
    }

    /// Returns true if the field type is a number type
    pub fn is_number(&self) -> bool {
        matches!(self, FieldType::Number)
    }

    /// Returns true if the field type is a float type
    pub fn is_float(&self) -> bool {
        matches!(self, FieldType::Float)
    }

    /// Returns true if the field type is an int type
    pub fn is_int(&self) -> bool {
        matches!(self, FieldType::Int)
    }

    /// Returns true if the field type is a decimal type
    pub fn is_decimal(&self) -> bool {
        matches!(self, FieldType::Decimal)
    }

    /// Returns true if the field type is a record type with no reference tables
    pub fn is_empty_record(&self) -> bool {
        matches!(self, FieldType::Record(ref_tables) if ref_tables.is_empty())
    }

    /// Returns true if the field type is a geometry type with no reference tables
    pub fn is_empty_geometry(&self) -> bool {
        matches!(self, FieldType::Geometry(ref_tables) if ref_tables.is_empty())
    }
}

/// Parses a field type
/// ```
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::{FieldType, parse_field_type};
/// assert_eq!(parse_field_type("any"), Ok(("", FieldType::Any)));
/// assert_eq!(parse_field_type("null"), Ok(("", FieldType::Null)));
/// assert_eq!(parse_field_type("bool"), Ok(("", FieldType::Bool)));
/// assert_eq!(parse_field_type("option<string>"), Ok(("", FieldType::Option(Box::new(FieldType::String)))));
/// ```
pub fn parse_field_type(input: &str) -> IResult<&str, FieldType> {
    // all_consuming(parse_top_level_field_type)(input)
    all_consuming(cut(context(
        "Unexpected characters found after parsing",
        parse_top_level_field_type,
    )))(input)
}

fn parse_top_level_field_type(input: &str) -> IResult<&str, FieldType> {
    alt((parse_union_type, parse_option_field_type))(input)
}

fn parse_pipe(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, ()))
}

fn parse_single_field_type(input: &str) -> IResult<&str, FieldType> {
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

fn parse_union_type(input: &str) -> IResult<&str, FieldType> {
    // let (input, ft) = separated_list1(parse_pipe, parse_db_field_type)(input)?;
    let (input, mut ft) = separated_list1(parse_pipe, alt((parse_single_field_type,)))(input)?;
    let ft = match ft.len() {
        1 => ft.remove(0),
        _ => FieldType::Union(ft),
    };
    Ok((input, ft))
}

fn parse_option_field_type(input: &str) -> IResult<&str, FieldType> {
    let (input, _) = tag("option")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("<")(input)?;
    let (input, _) = space0(input)?;
    let (input, ft) = parse_top_level_field_type(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(">")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, FieldType::Option(Box::new(ft))))
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

fn parse_primitive_type(input: &str) -> IResult<&str, FieldType> {
    alt((
        value(FieldType::Any, tag("any")),
        value(FieldType::Null, tag("null")),
        value(FieldType::Bool, tag("bool")),
        value(FieldType::Bytes, tag("bytes")),
        value(FieldType::Datetime, tag("datetime")),
        value(FieldType::Decimal, tag("decimal")),
        value(FieldType::Duration, tag("duration")),
        value(FieldType::Float, tag("float")),
        value(FieldType::Int, tag("int")),
        value(FieldType::Number, tag("number")),
        value(FieldType::Object, tag("object")),
        value(FieldType::String, tag("string")),
        value(FieldType::Uuid, tag("uuid")),
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

fn is_valid_id_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    take_while1(is_valid_id_char)(input)
}

fn parse_record_inner(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("<")(input)?;
    let (input, _) = space0(input)?;
    let (input, ref_tables) =
        separated_list0(tag("|"), tuple((space0, parse_identifier, space0)))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(">")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, ref_tables.iter().map(|t| t.1).collect()))
}

fn parse_record_type(input: &str) -> IResult<&str, FieldType> {
    let (input, _) = tag("record")(input)?;
    let (input, rt) = opt(parse_record_inner)(input)?;
    // let (input, rt) = cut(opt(parse_record_inner))(input)?;
    Ok((
        input,
        FieldType::Record(
            rt.unwrap_or(vec![])
                .into_iter()
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

fn parse_simple_geom(input: &str) -> IResult<&str, GeometryType> {
    alt((
        tag("feature").map(|_| GeometryType::Feature),
        tag("point").map(|_| GeometryType::Point),
        tag("LineString").map(|_| GeometryType::LineString),
        tag("polygon").map(|_| GeometryType::Polygon),
        tag("multipoint").map(|_| GeometryType::MultiPoint),
        tag("multiline").map(|_| GeometryType::MultiLine),
        tag("multipolygon").map(|_| GeometryType::MultiPolygon),
        tag("collection").map(|_| GeometryType::Collection),
    ))(input)
}

fn parse_geometry_inner(input: &str) -> IResult<&str, Vec<GeometryType>> {
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

fn parse_geometry_type(input: &str) -> IResult<&str, FieldType> {
    let (input, _) = tag("geometry")(input)?;
    let (input, rt) = opt(parse_geometry_inner)(input)?;
    // let (input, rt) = cut(opt(parse_record_inner))(input)?;
    Ok((input, FieldType::Geometry(rt.unwrap_or(vec![]))))
}

struct ListItem {
    item_type: FieldType,
    size: Option<u64>,
}
fn parse_list_inner(input: &str) -> IResult<&str, ListItem> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("<")(input)?;
    let (input, _) = space0(input)?;
    let (input, field_type) = parse_top_level_field_type(input)?;
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

fn parse_array_type(input: &str) -> IResult<&str, FieldType> {
    let (input, _) = tag("array")(input)?;
    let (input, item_type) = opt(parse_list_inner)(input)?;

    if let Some(ListItem { item_type, size }) = item_type {
        Ok((input, FieldType::Array(Box::new(item_type), size)))
    } else {
        Ok((input, FieldType::Array(Box::new(FieldType::Any), None)))
    }
}

fn parse_set_type(input: &str) -> IResult<&str, FieldType> {
    let (input, _) = tag("set")(input)?;
    let (input, item_type) = opt(parse_list_inner)(input)?;

    if let Some(ListItem { item_type, size }) = item_type {
        Ok((input, FieldType::Set(Box::new(item_type), size)))
    } else {
        Ok((input, FieldType::Set(Box::new(FieldType::Any), None)))
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
                    let result = parse_field_type($input);
                    let (input, ouput) = result.unwrap();
                    assert_eq!(input, "");
                    assert_eq!(ouput, $output);
                }
            }
        };
    }

    test_parse_db_field_type!(any, "any", FieldType::Any);
    test_parse_db_field_type!(null, "null", FieldType::Null);
    test_parse_db_field_type!(bool, "bool", FieldType::Bool);
    test_parse_db_field_type!(bytes, "bytes", FieldType::Bytes);
    test_parse_db_field_type!(datetime, "datetime", FieldType::Datetime);
    test_parse_db_field_type!(decimal, "decimal", FieldType::Decimal);
    test_parse_db_field_type!(duration, "duration", FieldType::Duration);
    test_parse_db_field_type!(flaot, "float", FieldType::Float);
    test_parse_db_field_type!(int, "int", FieldType::Int);
    test_parse_db_field_type!(number, "number", FieldType::Number);
    test_parse_db_field_type!(object, "object", FieldType::Object);
    test_parse_db_field_type!(string, "string", FieldType::String);
    test_parse_db_field_type!(uuild, "uuid", FieldType::Uuid);
    test_parse_db_field_type!(record_any, "record", FieldType::Record(vec![]));
    test_parse_db_field_type!(
        record_single_alien,
        "record<alien> ",
        FieldType::Record(vec!["alien".into()])
    );
    test_parse_db_field_type!(
        record_spaced,
        "record      < lowo | dayo  |     oye>",
        FieldType::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
    );
    test_parse_db_field_type!(
        record_no_space,
        "record<lowo|dayo|oye>",
        FieldType::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
    );

    test_parse_db_field_type!(
        geometry_empty_optional,
        "geometry",
        FieldType::Geometry(vec![])
    );

    test_parse_db_field_type!(
        geometry_single,
        "geometry<point>",
        FieldType::Geometry(vec![GeometryType::Point])
    );

    test_parse_db_field_type!(
        geometry_spaced,
        "geometry    < point | line  |     polygon>",
        FieldType::Geometry(vec![
            GeometryType::Point,
            GeometryType::LineString,
            GeometryType::Polygon
        ])
    );

    test_parse_db_field_type!(
        geometry_no_space,
        "geometry<collection| point|multipolygon|line|polygon>",
        FieldType::Geometry(vec![
            GeometryType::Collection,
            GeometryType::Point,
            GeometryType::MultiPolygon,
            GeometryType::LineString,
            GeometryType::Polygon
        ])
    );
    test_parse_db_field_type!(
        option_string,
        "option<string>",
        FieldType::Option(Box::new(FieldType::String))
    );
    test_parse_db_field_type!(
        option_float,
        "option<float>",
        FieldType::Option(Box::new(FieldType::Float))
    );
    test_parse_db_field_type!(
        option_bool,
        "option<bool>",
        FieldType::Option(Box::new(FieldType::Bool))
    );
    test_parse_db_field_type!(
        option_decimal,
        "option<decimal>",
        FieldType::Option(Box::new(FieldType::Decimal))
    );
    test_parse_db_field_type!(
        option_duration,
        "option<duration>",
        FieldType::Option(Box::new(FieldType::Duration))
    );
    test_parse_db_field_type!(
        option_datetime,
        "option<datetime>",
        FieldType::Option(Box::new(FieldType::Datetime))
    );
    test_parse_db_field_type!(
        option_uuid,
        "option<uuid>",
        FieldType::Option(Box::new(FieldType::Uuid))
    );
    test_parse_db_field_type!(
        option_number,
        "option<number>",
        FieldType::Option(Box::new(FieldType::Number))
    );
    test_parse_db_field_type!(
        option_object,
        "option<object>",
        FieldType::Option(Box::new(FieldType::Object))
    );
    test_parse_db_field_type!(
        option_bytes,
        "option<bytes>",
        FieldType::Option(Box::new(FieldType::Bytes))
    );
    test_parse_db_field_type!(
        option_any,
        "option<any>",
        FieldType::Option(Box::new(FieldType::Any))
    );
    test_parse_db_field_type!(
        option_null,
        "option<null>",
        FieldType::Option(Box::new(FieldType::Null))
    );
    test_parse_db_field_type!(
        option_geometry,
        "option<geometry>",
        FieldType::Option(Box::new(FieldType::Geometry(vec![])))
    );
    test_parse_db_field_type!(
        option_geometry_single,
        "option<geometry<point>>",
        FieldType::Option(Box::new(FieldType::Geometry(vec![GeometryType::Point])))
    );
    test_parse_db_field_type!(
        option_geometry_spaced,
        "option<geometry    < point | line  |     polygon>>",
        FieldType::Option(Box::new(FieldType::Geometry(vec![
            GeometryType::Point,
            GeometryType::LineString,
            GeometryType::Polygon
        ])))
    );
    test_parse_db_field_type!(
        option_geometry_no_space,
        "option<geometry<collection| point|multipolygon|line|polygon>>",
        FieldType::Option(Box::new(FieldType::Geometry(vec![
            GeometryType::Collection,
            GeometryType::Point,
            GeometryType::MultiPolygon,
            GeometryType::LineString,
            GeometryType::Polygon
        ])))
    );
    test_parse_db_field_type!(
        option_int,
        "option<int>",
        FieldType::Option(Box::new(FieldType::Int))
    );
    test_parse_db_field_type!(
        option_simple_record,
        "option<record>",
        FieldType::Option(Box::new(FieldType::Record(vec![])))
    );
    test_parse_db_field_type!(
        option_record,
        "option<record<alien>>",
        FieldType::Option(Box::new(FieldType::Record(vec!["alien".into()])))
    );
    test_parse_db_field_type!(
        option_record_spaced,
        "option<record    < lowo | dayo  |     oye>>",
        FieldType::Option(Box::new(FieldType::Record(vec![
            "lowo".into(),
            "dayo".into(),
            "oye".into()
        ])))
    );
    test_parse_db_field_type!(
        option_record_no_space,
        "option<record<lowo|dayo|oye>>",
        FieldType::Option(Box::new(FieldType::Record(vec![
            "lowo".into(),
            "dayo".into(),
            "oye".into()
        ])))
    );

    // Union/Either
    test_parse_db_field_type!(
        union_string_int,
        "string|int",
        FieldType::Union(vec![FieldType::String, FieldType::Int])
    );

    // Array
    test_parse_db_field_type!(
        option_array,
        "option<array>",
        FieldType::Option(Box::new(FieldType::Array(Box::new(FieldType::Any), None)))
    );
    test_parse_db_field_type!(
        option_array_string,
        "option<array<string>>",
        FieldType::Option(Box::new(FieldType::Array(
            Box::new(FieldType::String),
            None
        )))
    );
    test_parse_db_field_type!(
        option_array_string_10_nospace,
        "option<array<string,10>>",
        FieldType::Option(Box::new(FieldType::Array(
            Box::new(FieldType::String),
            Some(10)
        )))
    );

    test_parse_db_field_type!(
        option_array_string_10,
        "option<array<string, 10>>",
        FieldType::Option(Box::new(FieldType::Array(
            Box::new(FieldType::String),
            Some(10)
        )))
    );
    test_parse_db_field_type!(
        option_array_string_10_spaced,
        "option<array    < string , 10> >",
        FieldType::Option(Box::new(FieldType::Array(
            Box::new(FieldType::String),
            Some(10)
        )))
    );
    test_parse_db_field_type!(
        option_array_string_10_spaced2,
        "option   <  array    < string ,   10> >",
        FieldType::Option(Box::new(FieldType::Array(
            Box::new(FieldType::String),
            Some(10)
        )))
    );
    // parse for array
    test_parse_db_field_type!(
        array_any,
        "array",
        FieldType::Array(Box::new(FieldType::Any), None)
    );
    test_parse_db_field_type!(
        array_string,
        "array<string>",
        FieldType::Array(Box::new(FieldType::String), None)
    );
    test_parse_db_field_type!(
        array_object_10_nospace,
        "array<object,69>",
        FieldType::Array(Box::new(FieldType::Object), Some(69))
    );
    test_parse_db_field_type!(
        array_geometry_10,
        "array<geometry<point | polygon | multipolygon>, 10>",
        FieldType::Array(
            Box::new(FieldType::Geometry(vec![
                GeometryType::Point,
                GeometryType::Polygon,
                GeometryType::MultiPolygon
            ])),
            Some(10)
        )
    );
    test_parse_db_field_type!(
        array_null_10_spaced,
        "array    < null , 10> ",
        FieldType::Array(Box::new(FieldType::Null), Some(10))
    );
    test_parse_db_field_type!(
        array_nested_array,
        "array<array<float, 42> , 10> ",
        FieldType::Array(
            Box::new(FieldType::Array(Box::new(FieldType::Float), Some(42))),
            Some(10)
        )
    );

    // SET TESTS
    test_parse_db_field_type!(
        set_any,
        "set",
        FieldType::Set(Box::new(FieldType::Any), None)
    );
    test_parse_db_field_type!(
        set_string,
        "set<string>",
        FieldType::Set(Box::new(FieldType::String), None)
    );
    test_parse_db_field_type!(
        set_object_10_nospace,
        "set<object,69>",
        FieldType::Set(Box::new(FieldType::Object), Some(69))
    );
    test_parse_db_field_type!(
        set_geometry_10,
        "set<geometry<point | polygon | multipolygon>, 10>",
        FieldType::Set(
            Box::new(FieldType::Geometry(vec![
                GeometryType::Point,
                GeometryType::Polygon,
                GeometryType::MultiPolygon
            ])),
            Some(10)
        )
    );
    test_parse_db_field_type!(
        set_null_10_spaced,
        "set    < null , 10> ",
        FieldType::Set(Box::new(FieldType::Null), Some(10))
    );
    test_parse_db_field_type!(
        set_nested_array,
        "set<set<float, 42> , 10> ",
        FieldType::Set(
            Box::new(FieldType::Set(Box::new(FieldType::Float), Some(42))),
            Some(10)
        )
    );

    /////// Test Union
    // test_parse_db_field_type!(union_any_single, "any", FieldTypee::Any);

    test_parse_db_field_type!(
        union_any_null,
        "any | null",
        FieldType::Union(vec![FieldType::Any, FieldType::Null])
    );

    test_parse_db_field_type!(
        union_any_null_bool,
        "any | null | bool",
        FieldType::Union(vec![FieldType::Any, FieldType::Null, FieldType::Bool])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes,
        "any | null | bool | bytes",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime,
        "any | null | bool | bytes | datetime",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal,
        "any | null | bool | bytes | datetime | decimal",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration,
        "any | null | bool | bytes | datetime | decimal | duration",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float,
        "any | null | bool | bytes | datetime | decimal | duration | float",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int,
        "any | null | bool | bytes | datetime | decimal | duration | float | int",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid,
            FieldType::Record(vec![])
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid,
            FieldType::Record(vec![]),
            FieldType::Geometry(vec![])
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry_array,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry | array",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid,
            FieldType::Record(vec![]),
            FieldType::Geometry(vec![]),
            FieldType::Array(Box::new(FieldType::Any), None)
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry_array_set,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry | array | set",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid,
            FieldType::Record(vec![]),
            FieldType::Geometry(vec![]),
            FieldType::Array(Box::new(FieldType::Any), None),
            FieldType::Set(Box::new(FieldType::Any), None)
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry_array_set_option,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry | array | set | option<int>",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid,
            FieldType::Record(vec![]),
            FieldType::Geometry(vec![]),
            FieldType::Array(Box::new(FieldType::Any), None),
            FieldType::Set(Box::new(FieldType::Any), None),
            FieldType::Option(Box::new(FieldType::Int))
        ])
    );

    test_parse_db_field_type!(
        union_array_of_string_and_int_or_null,
        "array<string|int|null>",
        FieldType::Array(
            Box::new(FieldType::Union(vec![
                FieldType::String,
                FieldType::Int,
                FieldType::Null
            ])),
            None
        )
    );

    test_parse_db_field_type!(
        union_array_of_string_and_int_or_null_10,
        "int | option<float> | array<option<string>|int|null, 10> | set<option<number>|float|null, 10> | option<array> | option<set<option<int>>>",
        FieldType::Union(vec![
            FieldType::Int,
            FieldType::Option(Box::new(FieldType::Float)),
            FieldType::Array(
                Box::new(FieldType::Union(vec![
                    FieldType::Option(Box::new(FieldType::String)),
                    FieldType::Int,
                    FieldType::Null
                ])),
                Some(10)
            ),
            FieldType::Set(
                Box::new(FieldType::Union(vec![
                    FieldType::Option(Box::new(FieldType::Number)),
                    FieldType::Float,
                    FieldType::Null
                ])),
                Some(10)
            ),
            FieldType::Option(Box::new(FieldType::Array(
                Box::new(FieldType::Any),
                None
            ))),
            FieldType::Option(Box::new(FieldType::Set(
                Box::new(FieldType::Option(Box::new(FieldType::Int))),
                None
            )))
        ])
    );
}
