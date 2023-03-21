/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    fmt::{self, Display},
    ops::Deref,
};

use insta::{assert_debug_snapshot, assert_display_snapshot};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::sql::{self, statements::DefineStatement};

use crate::{
    binding::{BindingsList, Parametric},
    filter::Filter,
    param::Param,
    query_for::PermisisonForables,
    sql::{Buildable, Queryable, Table},
    Erroneous, Field,
};

// DEFINE FIELD statement
// The DEFINE FIELD statement allows you to instantiate a named field on a table, enabling you to set the field's data type, set a default value, apply assertions to protect data consistency, and set permissions specifying what operations can be performed on the field.
//
// Requirements
// You must be authenticated as a root, namespace, or database user before you can use the DEFINE FIELD statement.
// You must select your namespace and database before you can use the DEFINE FIELD statement.
// Statement syntax
// DEFINE FIELD @name ON [ TABLE ] @table
// 	[ TYPE @type ]
// 	[ VALUE @expression ]
// 	[ ASSERT @expression ]
// 	[ PERMISSIONS [ NONE | FULL
// 		| FOR select @expression
// 		| FOR create @expression
// 		| FOR update @expression
// 		| FOR delete @expression
// 	] ]
// Example usage
// The following expression shows the simplest way to use the DEFINE FIELD statement.
//
// -- Declare the name of a field.
// DEFINE FIELD email ON TABLE user;
// Defining data types
// Simple data types
// -- Set a field to have the string data type
// DEFINE FIELD email ON TABLE user TYPE string;
//
// -- Set a field to have the datetime data type
// DEFINE FIELD created ON TABLE user TYPE datetime;
//
// -- Set a field to have the object data type
// DEFINE FIELD metadata ON TABLE user TYPE object;
//
// -- Set a field to have the bool data type
// DEFINE FIELD locked ON TABLE user TYPE bool;
// Array data type
// -- Set a field to have the array data type
// DEFINE FIELD roles ON TABLE user TYPE array;
// -- Set the contents of the array to only support a string data type
// DEFINE FIELD roles.* ON TABLE user TYPE string;
//
// -- Set a field to have the array data type
// DEFINE FIELD posts ON TABLE user TYPE array;
// -- Set the contents of the array to only support a record data type
// DEFINE FIELD posts.* ON TABLE user TYPE record;
// Setting a default value
// -- A user is not locked by default.
// DEFINE FIELD locked ON TABLE user TYPE bool
//   -- Set a default value if empty
//   VALUE $value OR false;
// Asserting rules on fields
// You can take your field definitions even further by using asserts. Assert is a powerful feature that can be used to ensure that your data remains consistent.
//
// Email is required
// -- Give the user table an email field. Store it in a string
// DEFINE FIELD email ON TABLE user TYPE string
//   -- Make this field required
//   ASSERT $value != NONE
//   -- Check if the value is a properly formatted email address
//   AND is::email($value);
// Array with allowed values
// By using an Access Control List as an example we can show how we can restrict what values can be stored in an array.
//
// DEFINE FIELD resource on acl TYPE record
//   ASSERT $value != NONE;
// DEFINE FIELD user ON TABLE acl TYPE record (user)
//   ASSERT $value != NONE;
//
// -- A user can have multiple permissions on a acl
// DEFINE FIELD permission ON TABLE acl TYPE array
//   -- The array must not be empty because at least one permission is required
//   ASSERT array::len($value) > 0;
//
// -- Assigned permissions are identified by strings
// DEFINE FIELD type.* ON TABLE resource TYPE string
//   -- Allow only these values in the array
//   ASSERT $value INSIDE ["create", "read", "write", "delete"];
// Use regex to validate a string
// -- Specify a field on the user table
// DEFINE FIELD countrycode ON user TYPE string
// 	-- Ensure country code is ISO-3166
// 	ASSERT $value != NONE AND $value = /[A-Z]{3}/
// 	-- Set a default value if empty
// 	VALUE $value OR 'GBR'
// ;
// Field data types
// The DEFINE FIELD statement allows specify the following data types on the field.
//
// Type	Description
// any	Use this when you explicitly don't want to specify the field's data type. The field will allow any data type supported by SurrealDB.
// array
// bool
// datetime	An ISO 8601 compliant data type that stores a date with time and time zone.
// decimal	Uses BigDecimal for storing any real number with arbitrary precision.
// duration	Store a value representing a length of time. Can be added or subtracted from datetimes or other durations.
// float	Store a value in a 64 bit float.
// int	Store a value in a 64 bit integer.
// number	Store numbers without specifying the type. SurrealDB will detect the type of number and store it using the minimal number of bytes. For numbers passed in as a string, this field will store the number in a BigDecimal.
// object	Store formatted objects containing values of any supported type with no limit to object depth or nesting.
// string
// record	Store a reference to another record. The value must be a Record ID.
// geometry	RFC 7946 compliant data type for storing geometry in the GeoJson format.
// Geometric Types include:
// feature
// point
// line
// polygon
// multipoint
// multiline
// multipolygon
// collection
// -- Define a field with a single type
// DEFINE FIELD location ON TABLE restaurant TYPE geometry (point);
// -- Define a field with any geometric type
// DEFINE FIELD area ON TABLE restaurant TYPE geometry (feature);
// -- Define a field with specific geometric types
// DEFINE FIELD area ON TABLE restaurant TYPE geometry (polygon, multipolygon, collection);

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
    Array(Box<FieldType>),
    Bool,
    DateTime,
    Decimal,
    Duration,
    Float,
    Int,
    Number,
    Object,
    String,
    Record(Table),
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
            FieldType::Array(field_type) => format!("array ({field_type})"),
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
            FieldType::Geometry(geometries) => geometries
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
                .to_string(),
        };
        write!(f, "{}", data_type)
    }
}

// pub enum FieldType {
//     Any,
//     Array(Box<FieldType>),
//     Bool,
//     DateTime,
//     Decimal,
//     Duration,
//     Float,
//     Int,
//     Number,
//     Object,
//     String,
//     Record(String),
//     Geometry(Vec<Geometry>),
// }
//
// pub enum Geometry {
//     Feature,
//     Point,
//     Line,
//     Polygon,
//     Multipoint,
//     Multiline,
//     Multipolygon,
//     Collection,
// }

pub enum FieldType {
    Any,
    Array(Box<FieldType>),
    Bool,
    DateTime,
    Decimal,
    Duration,
    Float,
    Int,
    Number,
    Object,
    String,
    Record(String),
    Geometry(Vec<Geometry>),
}

pub fn from_string(field_type: &str) -> Option<FieldType> {
    let mut parts = field_type.split(" (");
    let type_name = parts.next()?;
    let geo_types = parts
        .filter(|p| p.ends_with(")"))
        .next()
        .map(|p| &p[1..p.len() - 1])
        .map(|p| {
            p.split(", ")
                .filter_map(|t| match t {
                    "Feature" => Some(Geometry::Feature),
                    "Point" => Some(Geometry::Point),
                    "Line" => Some(Geometry::Line),
                    "Polygon" => Some(Geometry::Polygon),
                    "Multipoint" => Some(Geometry::Multipoint),
                    "Multiline" => Some(Geometry::Multiline),
                    "Multipolygon" => Some(Geometry::Multipolygon),
                    "Collection" => Some(Geometry::Collection),
                    _ => None,
                })
                .collect::<Vec<Geometry>>()
        })
        .unwrap_or_default();

    Some(match type_name {
        "any" => FieldType::Any,
        "array" => {
            let content_type = from_string(parts.next()?.trim_start())?;
            FieldType::Array(Box::new(content_type))
        }
        "bool" => FieldType::Bool,
        "datetime" => FieldType::DateTime,
        "decimal" => FieldType::Decimal,
        "duration" => FieldType::Duration,
        "float" => FieldType::Float,
        "int" => FieldType::Int,
        "number" => FieldType::Number,
        "object" => FieldType::Object,
        "string" => FieldType::String,
        "record" => {
            let record_type = parts.next()?.trim_start().to_string();
            FieldType::Record(record_type)
        }
        "geometry" => FieldType::Geometry(geo_types),
        _ => return None,
    })
}
impl FieldType {
    pub fn from_str(s: &str) -> Option<FieldType> {
        match s.replace(" ", ""){
            "any" => Some(FieldType::Any),
            "bool" => Some(FieldType::Bool),
            "datetime" => Some(FieldType::DateTime),
            "decimal" => Some(FieldType::Decimal),
            "duration" => Some(FieldType::Duration),
            "float" => Some(FieldType::Float),
            "int" => Some(FieldType::Int),
            "number" => Some(FieldType::Number),
            "object" => Some(FieldType::Object),
            "string" => Some(FieldType::String),
            _ if s.starts_with("array<") && s.ends_with(">") => {
                let inner_type_str = &s[6..s.len() - 1];
                let inner_type = FieldType::from_str(inner_type_str)?;
                Some(FieldType::Array(Box::new(inner_type)))
            }
            _ if s.starts_with("record<") && s.ends_with(">") => {
                let record_type_str = &s[7..s.len() - 1];
                Some(FieldType::Record(record_type_str.to_owned()))
            }
            _ if s.starts_with("geometry(") && s.ends_with(")") => {
                let geometry_types_str = &s[9..s.len() - 1];
                let geometry_types: Vec<Geometry> = geometry_types_str
                    .split(",")
                    .map(|s| match s.trim() {
                        "Feature" => Geometry::Feature,
                        "Point" => Geometry::Point,
                        "Line" => Geometry::Line,
                        "Polygon" => Geometry::Polygon,
                        "Multipoint" => Geometry::Multipoint,
                        "Multiline" => Geometry::Multiline,
                        "Multipolygon" => Geometry::Multipolygon,
                        "Collection" => Geometry::Collection,
                        _ => return None,
                    })
                    .collect();
                Some(FieldType::Geometry
// impl FromStr for Geometry {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "Feature" => Ok(Self::Feature),
//             "Point" => Ok(Self::Point),
//             "Line" => Ok(Self::Line),
//             "Polygon" => Ok(Self::Polygon),
//             "Multipoint" => Ok(Self::Multipoint),
//             "Multiline" => Ok(Self::Multiline),
//             "Multipolygon" => Ok(Self::Multipolygon),
//             "Collection" => Ok(Self::Collection),
//             _ => {
//                 let mut types = s.split(',');
//                 let mut geom_types = Vec::new();
//                 for t in types {
//                     let geom_type = t.trim().parse::<Geometry>();
//                     if let Ok(g) = geom_type {
//                         geom_types.push(g);
//                     } else {
//                         return Err(format!("Invalid geometry type: {}", t));
//                     }
//                 }
//                 if geom_types.is_empty() {
//                     Err("Empty geometry type list".to_string())
//                 } else {
//                     Ok(Self::Collection(geom_types))
//                 }
//             }
//         }
//     }
// }
#[derive(Clone)]
pub struct DefineFieldStatement {
    field_name: String,
    table_name: Option<String>,
    type_: Option<String>,
    value: Option<String>,
    assert: Option<String>,
    permissions_none: Option<bool>,
    permissions_full: Option<bool>,
    permissions_for: Vec<String>,
    bindings: BindingsList,
}

pub fn define_field(fieldable: impl Into<Field>) -> DefineFieldStatement {
    let field: Field = fieldable.into();
    DefineFieldStatement {
        field_name: field.to_string(),
        table_name: None,
        type_: None,
        value: None,
        assert: None,
        permissions_none: None,
        permissions_full: None,
        permissions_for: vec![],
        bindings: vec![],
    }
}

pub struct ValueAssert(Param);

impl Deref for ValueAssert {
    type Target = Param;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn value() -> ValueAssert {
    ValueAssert(Param::new("value"))
}

impl DefineFieldStatement {
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        let table: Table = table.into();
        self.table_name = Some(table.to_string());
        self
    }

    pub fn type_(mut self, field_type: impl Into<FieldType>) -> Self {
        let field_type: FieldType = field_type.into();
        self.type_ = Some(field_type.to_string());
        self
    }

    pub fn value(mut self, default_value: impl Into<sql::Value>) -> Self {
        let value: sql::Value = default_value.into();
        self.value = Some(value.to_string());
        self
    }

    pub fn assert(mut self, assertion: impl Into<Filter>) -> Self {
        let assertion: Filter = assertion.into();
        self.assert = Some(assertion.to_string());
        self
    }

    pub fn permissions_none(mut self) -> Self {
        self.permissions_none = Some(true);
        self
    }

    pub fn permissions_full(mut self) -> Self {
        self.permissions_full = Some(true);
        self
    }

    pub fn permissions_for(mut self, fors: impl Into<PermisisonForables>) -> Self {
        let fors: PermisisonForables = fors.into();
        match fors {
            PermisisonForables::For(one) => {
                self.permissions_for.push(one.to_string());
                self.bindings.extend(one.get_bindings());
            }
            PermisisonForables::Fors(many) => many.iter().for_each(|f| {
                self.permissions_for.push(f.to_string());
                self.bindings.extend(f.get_bindings());
            }),
        }
        self
    }
}

impl Queryable for DefineFieldStatement {}
impl Erroneous for DefineFieldStatement {}

impl Parametric for DefineFieldStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for DefineFieldStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE FIELD {}", &self.field_name);

        if let Some(table) = &self.table_name {
            query = format!("{query} ON TABLE {table}");
        }

        if let Some(field_type) = &self.type_ {
            query = format!("{query} TYPE {field_type}");
        }

        if let Some(value) = &self.value {
            query = format!("{query} VALUE $value OR {value}");
        }

        if let Some(assertion) = &self.assert {
            query = format!("{query} ASSERT {assertion}");
        }

        if let Some(true) = self.permissions_none {
            query = format!("{query} PERMISSIONS NONE");
        } else if let Some(true) = self.permissions_full {
            query = format!("{query} PERMISSIONS FULL");
        } else if !&self.permissions_for.is_empty() {
            query = format!("{query}\nPERMISSIONS\n{}", self.permissions_for.join("\n"));
        }
        query.push_str(";");

        query
    }
}

impl Display for DefineFieldStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

//
// DEFINE FIELD email ON TABLE user; TYPE string; ASSERT $value != NONE AND is::email($value); VALUE $value OR '';
// ``

#[cfg(test)]
#[cfg(feature = "mock")]
mod tests {

    use super::*;
    use std::time::Duration;

    use crate::{
        query_for::ForCrudType,
        sql::NONE,
        utils::{cond, for_},
        Operatable,
    };

    use super::*;

    #[test]
    fn test_define_field_statement_full() {
        use ForCrudType::*;
        let name = Field::new("name");
        let user_table = Table::from("user");
        let age = Field::new("age");
        let email = Field::new("email");
        use FieldType::*;

        let statement = define_field(email)
            .on_table(user_table)
            .type_(String)
            .value("example@codebreather.com")
            .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
            .permissions_for(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
            .permissions_for(for_(&[Create, Update]).where_(name.is("Oyedayo"))) //Multiple
            .permissions_for(&[
                for_(&[Create, Delete]).where_(name.is("Oyedayo")),
                for_(Update).where_(age.less_than_or_equal(130)),
            ]);

        assert_eq!(
            statement.to_string(),
            "DEFINE FIELD email ON TABLE user TYPE string VALUE $value OR 'example@codebreather.com' ASSERT ($value IS NOT $_param_00000000) AND ($value ~ $_param_00000000)\nPERMISSIONS\nFOR select\n\tWHERE age >= $_param_00000000\nFOR create, update\n\tWHERE name IS $_param_00000000\nFOR create, delete\n\tWHERE name IS $_param_00000000\nFOR update\n\tWHERE age <= $_param_00000000;"
        );
        insta::assert_display_snapshot!(statement);
        insta::assert_debug_snapshot!(statement.get_bindings());
    }

    #[test]
    fn test_define_field_statement_simple() {
        use FieldType::*;

        let email = Field::new("email");
        let user_table = Table::from("user");
        let statement = define_field(email).on_table(user_table).type_(String);

        assert_eq!(
            statement.to_string(),
            "DEFINE FIELD email ON TABLE user TYPE string;"
        );
        insta::assert_display_snapshot!(statement);
        insta::assert_debug_snapshot!(statement.get_bindings());
    }
}
