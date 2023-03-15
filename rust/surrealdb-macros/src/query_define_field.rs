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
    db_field::{cond, Binding},
    query_create::CreateStatement,
    query_define_token::{Name, Scope},
    query_delete::DeleteStatement,
    query_ifelse::Expression,
    query_insert::{Buildable, InsertStatement},
    query_relate::RelateStatement,
    query_remove::{Event, RemoveScopeStatement, Runnable, Table},
    query_select::{Duration, SelectStatement},
    query_update::UpdateStatement,
    BindingsList, DbField, DbFilter, Parametric, Queryable,
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

pub enum Geometry {
    Feature,
    Point,
    Line,
    Polygon,
    Multipoint,
    Multiline,
    Multipolygon,
    Collection,
}
impl Display for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let geom = match self {
            Geometry::Feature => "feature",
            Geometry::Point => "point",
            Geometry::Line => "line",
            Geometry::Polygon => "polygon",
            Geometry::Multipoint => "multipoint",
            Geometry::Multiline => "multiline",
            Geometry::Multipolygon => "multipolygon",
            Geometry::Collection => "collection",
        };
        //         feature
        // point
        // line
        // polygon
        // multipoint
        // multiline
        // multipolygon
        // collection
        write!(f, "{}", geom)
    }
}
pub enum DataType {
    Any,
    Array,
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
    Geometry(Vec<Geometry>),
}

impl Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_type = match self {
            DataType::Any => "any".to_string(),
            DataType::Array => "array".to_string(),
            DataType::Bool => "bool".to_string(),
            DataType::DateTime => "datetime".to_string(),
            DataType::Decimal => "decimal".to_string(),
            DataType::Duration => "duration".to_string(),
            DataType::Float => "float".to_string(),
            DataType::Int => "int".to_string(),
            DataType::Number => "number".to_string(),
            DataType::Object => "object".to_string(),
            DataType::String => "string".to_string(),
            DataType::Record(table) => format!("record ({table})"),
            DataType::Geometry(geometries) => geometries
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
                .to_string(),
        };
        write!(f, "{}", data_type)
    }
}

pub struct QueryBuilder<'a> {
    db: &'a str,
    namespace: &'a str,
    query: String,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(db: &'a str, namespace: &'a str) -> Self {
        Self {
            db,
            namespace,
            query: String::new(),
        }
    }

    pub fn define_field(&mut self, name: &'a str, table: &'a str) -> &mut Self {
        self.query
            .push_str(&format!("DEFINE FIELD {} ON TABLE {};", name, table));
        self
    }

    pub fn field_type(&mut self, field_type: &'a str) -> &mut Self {
        self.query.push_str(&format!(" TYPE {};", field_type));
        self
    }

    pub fn default_value(&mut self, default_value: &'a str) -> &mut Self {
        self.query.push_str(&format!(" VALUE {};", default_value));
        self
    }

    pub fn assertion(&mut self, assertion: &'a str) -> &mut Self {
        self.query.push_str(&format!(" ASSERT {};", assertion));
        self
    }
}

//
// DEFINE FIELD email ON TABLE user; TYPE string; ASSERT $value != NONE AND is::email($value); VALUE $value OR '';
// ``
