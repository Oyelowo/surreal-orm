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
    query_define_table::PermisisonForables,
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
pub enum FieldType {
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
pub fn define_field(fieldable: impl Into<DbField>) -> DefineFieldStatement {
    let field: DbField = fieldable.into();
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

pub struct ValueAssert(DbField);

impl Display for ValueAssert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let xx: sql::Idiom = self.0.clone().into();
        // let mm = sql::Param::from(xx);
        write!(f, "R{}", self.0)
    }
}

impl Deref for ValueAssert {
    type Target = DbField;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn value() -> ValueAssert {
    ValueAssert(DbField::new("value"))
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

    pub fn assert(mut self, assertion: impl Into<DbFilter>) -> Self {
        let assertion: DbFilter = assertion.into();
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
mod tests {

    use super::*;
    use std::time::Duration;

    use crate::{
        db_field::NONE,
        query_define_table::{for_, ForCrudType},
        query_select::{order, select, All},
        value_type_wrappers::SurrealId,
    };

    use super::*;

    #[test]
    fn test_define_field_statement_full() {
        use ForCrudType::*;
        let name = DbField::new("name");
        let user_table = Table::from("user");
        let age = DbField::new("age");
        let email = DbField::new("email");
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
            "DEFINE FIELD email ON TABLE user TYPE string VALUE OR 'example@codebreather.com' ASSERT (`$value` IS NOT $_param_00000000) AND (`$value` ~ $_param_00000000)\nPERMISSIONS\nFOR select\n\tWHERE age >= $_param_00000000\nFOR create, update\n\tWHERE name IS $_param_00000000\nFOR create, delete\n\tWHERE name IS $_param_00000000\nFOR update\n\tWHERE age <= $_param_00000000;"
        );
        insta::assert_display_snapshot!(statement);
        insta::assert_debug_snapshot!(statement.get_bindings());
    }

    #[test]
    fn test_define_field_statement_simple() {
        use FieldType::*;

        let email = DbField::new("email");
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
