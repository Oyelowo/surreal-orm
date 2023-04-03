use std::{
    fmt::{self, Display},
    ops::Deref,
    str::FromStr,
    string::ParseError,
};

use crate::Table;

#[derive(Debug, Clone)]
pub enum GeometryType {
    Feature,
    Point,
    Line,
    Polygon,
    Multipoint,
    Multiline,
    Multipolygon,
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

#[derive(Debug, Clone)]
pub enum FieldType {
    Any,
    Array,
    // ArrayList(Box<FieldType>),
    Bool,
    DateTime,
    Decimal,
    Duration,
    Float,
    Int,
    Number,
    Object,
    String,
    // Even though the documentation states that you can have a record type without specifying
    // the reference table, from my test as at 3rd of April, this is an invalid query. Same goes
    // for geometry, you have to specify the geomtry type. So, I am commenting these out for now.
    // Prior to that, I had a different name for Record with referent table name. Same for
    // geometry.
    // If these happen to be supported in the future, I can have Record(Table) ->
    // RecordWithTable(Table) and Geomtry(Vec<GeomtryType>) -> GeometryWithTypes(Vec<GeomtryType>)
    // Record,
    Record(Table),
    // Geometry,
    Geometry(Vec<GeometryType>),
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
            // FieldType::ArrayList(field_type) => format!("array ({field_type})"),
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
            // FieldType::Record => "record".to_string(),
            // FieldType::Geometry => "geometry".to_string(),
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
        // Examples:
        // datetime
        // record
        // record (user)
        // geometry (polygon, multipolygon, collection)
        // geometry
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
            // (Some("record"), None) => FieldType::Record,
            (Some("record"), Some(record_type)) => FieldType::Record(Table::from(record_type)),
            (Some("array"), None) => FieldType::Array,
            // (Some("array"), Some(content)) => {
            //     let content_type = Self::from_str(content)?;
            //     FieldType::ArrayList(Box::new(content_type))
            // }
            // (Some("geometry"), None) => FieldType::Geometry,
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
