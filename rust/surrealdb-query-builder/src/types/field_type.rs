use std::{
    fmt::{self, Display},
    str::FromStr,
};

use crate::Table;

/// Geometry types supported by surrealdb
#[derive(Debug, Clone)]
pub enum GeometryType {
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

impl FromStr for GeometryType {
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
            _ => {
                return Err(format!("Invalid geometry type: {}", s));
            }
        }
    }
}

impl Display for GeometryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let geom = match self {
            GeometryType::Feature => "feature",
            GeometryType::Point => "point",
            GeometryType::Line => "line",
            GeometryType::Polygon => "polygon",
            GeometryType::Multipoint => "multipoint",
            GeometryType::Multiline => "multiline",
            GeometryType::Multipolygon => "multipolygon",
            GeometryType::Collection => "collection",
        };
        write!(f, "{}", geom)
    }
}

/// Database field types supported by SurrealDB
#[derive(Debug, Clone)]
pub enum FieldType {
    /// Use this when you explicitly don't want to specify the field's data type. The field will
    /// allow any data type supported by SurrealDB.
    Any,
    /// a list
    Array,
    /// true of false
    Bool,
    /// An ISO 8601 compliant data type that stores a date with time and time zone.
    DateTime,
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
    ///
    String,
    /// Store a reference to another record. The record can belong to any table
    RecordAny,
    /// Store a reference to another record. The value must be a Record ID.
    Record(Table),
    /// RFC 7946 compliant data type for storing geometry in the GeoJson format.
    Geometry(Vec<GeometryType>),
}

impl FieldType {
    /// Returns a list of all the variants of the enum
    pub fn variants() -> Vec<&'static str> {
        vec![
            "any", "array", "bool", "datetime", "decimal", "duration", "float", "int", "number",
            "object", "string", "record", "geometry",
        ]
    }
}

impl From<FieldType> for String {
    fn from(val: FieldType) -> Self {
        val.to_string()
    }
}

impl Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_type = match self {
            FieldType::Any => "any".to_string(),
            FieldType::Array => "array".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::DateTime => "datetime".to_string(),
            FieldType::Decimal => "decimal".to_string(),
            FieldType::Duration => "duration".to_string(),
            FieldType::Float => "float".to_string(),
            FieldType::Int => "int".to_string(),
            FieldType::Number => "number".to_string(),
            FieldType::Object => "object".to_string(),
            FieldType::String => "string".to_string(),
            FieldType::Record(table) => format!("record ({table})"),
            FieldType::RecordAny => "record()".to_string(),
            FieldType::Geometry(geometries) => format!(
                "geometry ({})",
                geometries
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
                    .to_string()
            ),
        };
        write!(f, "{}", data_type)
    }
}

impl FromStr for FieldType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let type_stringified = s.replace(" ", "");
        let mut type_with_content = type_stringified.trim_end_matches(")").split("(");

        let db_type = match (type_with_content.next(), type_with_content.next()) {
            (Some("any"), None) => FieldType::Any,
            (Some("datetime"), None) => FieldType::DateTime,
            (Some("decimal"), None) => FieldType::Decimal,
            (Some("duration"), None) => FieldType::Duration,
            (Some("float"), None) => FieldType::Float,
            (Some("int"), None) => FieldType::Int,
            (Some("number"), None) => FieldType::Number,
            (Some("object"), None) => FieldType::Object,
            (Some("string"), None) => FieldType::String,
            (Some("array"), None) => FieldType::Array,
            (Some("record()"), None) => FieldType::RecordAny,
            (Some("record"), Some(record_type)) => FieldType::Record(Table::from(record_type)),
            (Some("geometry"), Some(geom_types)) => {
                let geoms: Result<Vec<_>, _> = geom_types
                    .split(",")
                    .map(|g| g.parse::<GeometryType>())
                    .collect();
                FieldType::Geometry(geoms?)
            }
            _ => return Err(format!("Invalid/Unsupported database type: {s}")),
        };
        Ok(db_type)
    }
}
