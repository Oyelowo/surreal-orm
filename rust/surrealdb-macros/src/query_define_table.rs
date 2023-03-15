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

// DEFINE TABLE statement
// The DEFINE TABLE statement allows you to declare your table by name, enabling you to apply strict controls to a table's schema by making it SCHEMAFULL, create a foreign table view, and set permissions specifying what operations can be performed on the field.
//
// Requirements
// You must be authenticated as a root, namespace, or database user before you can use the DEFINE TABLE statement.
// You must select your namespace and database before you can use the DEFINE TABLE statement.
// Statement syntax
// DEFINE TABLE @name
// 	[ DROP ]
// 	[ SCHEMAFULL | SCHEMALESS ]
// 	[ AS SELECT @projections
// 		FROM @tables
// 		[ WHERE @condition ]
// 		[ GROUP [ BY ] @groups ]
// 	]
// 	[ PERMISSIONS [ NONE | FULL
// 		| FOR select @expression
// 		| FOR create @expression
// 		| FOR update @expression
// 		| FOR delete @expression
// 	] ]
// Example usage
// The following expression shows the simplest way to use the DEFINE TABLE statement.
//
// -- Declare the name of a table.
// DEFINE TABLE reading;
// The following example uses the DROP portion of the DEFINE TABLE statement. This would be like telling the database to drop any table that has the given name and replace it with a new one of the same name.
//
// -- Drop the table if it exists and create a new one with the same name.
// DEFINE TABLE reading DROP;
// The following example demonstrates the SCHEMAFULL portion of the DEFINE TABLE statement. When a table is defined as schemafull, the database strictly enforces any schema definitions that are specified using the DEFINE TABLE statement. New fields can not be added to a SCHEMAFULL table unless they are defined via the DEFINE FIELD statement.
//
// -- Create schemafull user table.
// DEFINE TABLE user SCHEMAFULL;

// -- Create schemaless user table.
// DEFINE TABLE user SCHEMALESS;
//
// -- Define a table as a view which aggregates data from the reading table
// DEFINE TABLE temperatures_by_month AS
// 	SELECT
// 		count() AS total,
// 		time::month(recorded_at) AS month,
// 		math::mean(temperature) AS average_temp
// 	FROM reading
// 	GROUP BY city
// ;
//
// -- SEE IT IN ACTION
// -- 1: Add a new temperature reading with some basic attributes
// CREATE reading SET
// 	temperature = 27.4,
// 	recorded_at = time::now(),
// 	city = 'London',
// 	location = (-0.118092, 51.509865)
// ;
//
// -- 2: Query the projection
// SELECT * FROM temperatures_by_month;
// The following shows how to set table level PERMISSIONS using the DEFINE TABLE statement. This allows you to set independent permissions for selecting, creating, updating, and deleting data.
//
// -- Specify access permissions for the 'post' table
// DEFINE TABLE post SCHEMALESS
// 	PERMISSIONS
// 		FOR select
// 			-- Published posts can be selected
// 			WHERE published = true
// 			-- A user can select all their own posts
// 			OR user = $auth.id
// 		FOR create, update
// 			-- A user can create or update their own posts
// 			WHERE user = $auth.id
// 		FOR delete
// 			-- A user can delete their own posts
// 			WHERE user = $auth.id
// 			-- Or an admin can delete any posts
// 			OR $auth.admin = true
// ;
//

//  for([create, update]).where_(user.equal(2));
//
#[derive(Clone, Copy)]
pub enum ForCrudType {
    Create,
    Select,
    Update,
    Delete,
}

impl Display for ForCrudType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let crud_type = match self {
            ForCrudType::Create => "create",
            ForCrudType::Select => "select",
            ForCrudType::Update => "update",
            ForCrudType::Delete => "delete",
        };
        write!(f, "{}", crud_type)
    }
}

struct For {
    crud_types: Vec<ForCrudType>,
    condition: Option<DbFilter>,
    bindings: BindingsList,
}

impl Parametric for ForEnding {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

// pub struct For {
//     for_data: Vec<ForData>,
// }

#[derive(Clone)]
pub enum ForArgs {
    ForOption(ForCrudType),
    ForOptions(Vec<ForCrudType>),
}
impl From<ForCrudType> for ForArgs {
    fn from(value: ForCrudType) -> Self {
        Self::ForOption(value)
    }
}

impl From<Vec<ForCrudType>> for ForArgs {
    fn from(value: Vec<ForCrudType>) -> Self {
        Self::ForOptions(value)
    }
}
impl From<ForArgs> for Vec<ForCrudType> {
    fn from(value: ForArgs) -> Self {
        match value {
            ForArgs::ForOption(one) => vec![one],
            ForArgs::ForOptions(many) => many,
        }
    }
}

impl<'a, const N: usize> From<&[ForCrudType; N]> for ForArgs {
    fn from(value: &[ForCrudType; N]) -> Self {
        Self::ForOptions(value.to_vec())
    }
}

fn eerer() {
    For::new(ForCrudType::Create);
    For::new(&[ForCrudType::Create]);
    For::new(vec![ForCrudType::Create]);
}

impl For {
    // fn new<'a, const N: usize>(for_crud_types: impl Into<&'a [ForCrudType; N]>) -> Self {
    fn new(for_crud_types: impl Into<ForArgs>) -> ForStart {
        ForStart(For {
            crud_types: for_crud_types.into().into(),
            condition: None,
            bindings: vec![],
        })
    }
}

pub struct ForStart(For);

impl ForStart {
    pub fn where_(mut self, condition: impl Into<DbFilter>) -> ForEnding {
        let condition: DbFilter = condition.into();
        self.0.condition = Some(condition.clone());
        self.0.bindings.extend(condition.get_bindings());
        ForEnding(self.0)
    }
}

pub fn for_(for_crud_types: impl Into<ForArgs>) -> ForStart {
    ForStart(For {
        crud_types: for_crud_types.into().into(),
        condition: None,
        bindings: vec![],
    })
}
pub struct ForEnding(For);

impl Buildable for ForEnding {
    fn build(&self) -> String {
        let mut query = format!("FOR");
        if !&self.0.crud_types.is_empty() {
            query = format!(
                "{query} {}",
                &self
                    .0
                    .crud_types
                    .iter()
                    .map(|ct| {
                        let ct = ct.to_string();
                        ct
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        if let Some(cond) = &self.0.condition {
            query = format!("{query}\n\tWHERE {cond}");
        }
        query
    }
}

impl Display for ForEnding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

enum SchemaType {
    Schemafull,
    Schemaless,
}
pub struct DefineTable<'a> {
    table_name: &'a str,
    drop: Option<bool>,
    schema_type: Option<SchemaType>,
    as_select: Option<String>,
    permissions_none: Option<bool>,
    permissions_full: Option<bool>,
    permissions_for: Option<bool>,
    permissions: Option<Permissions<'a>>,
}
//
// impl<'a> DefineTable<'a> {
//     pub fn new(name: &'a str) -> Self {
//         DefineTable {
//             table_name: name,
//             drop: false,
//             schema_type: false,
//             projections: None,
//             tables: None,
//             condition: None,
//             groups: None,
//             permissions: None,
//         }
//     }
//
//     pub fn drop(mut self) -> Self {
//         self.drop = true;
//         self
//     }
//
//     pub fn schemafull(mut self) -> Self {
//         self.schema_type = true;
//         self
//     }
//
//     pub fn projections(mut self, projections: Vec<&'a str>) -> Self {
//         self.projections = Some(projections);
//         self
//     }
//
//     pub fn tables(mut self, tables: Vec<&'a str>) -> Self {
//         self.tables = Some(tables);
//         self
//     }
//
//     pub fn condition(mut self, condition: &'a str) -> Self {
//         self.condition = Some(condition);
//         self
//     }
//
//     pub fn groups(mut self, groups: Vec<&'a str>) -> Self {
//         self.groups = Some(groups);
//         self
//     }
//
//     pub fn permissions(mut self, permissions: Permissions<'a>) -> Self {
//         self.permissions = Some(permissions);
//         self
//     }
// }
//
// impl Buildable for DefineStatement {
//     fn build(&self) -> String {
//         let mut statement = String::new();
//         statement.push_str("DEFINE TABLE ");
//         statement.push_str(self.name);
//
//         if self.drop {
//             statement.push_str(" DROP");
//         }
//
//         if self.schemafull {
//             statement.push_str(" SCHEMAFULL");
//         } else {
//             statement.push_str(" SCHEMALESS");
//         }
//
//         if let Some(projections) = self.projections {
//             statement.push_str(" AS SELECT ");
//             statement.push_str(&projections.join(", "));
//         }
//
//         if let Some(tables) = self.tables {
//             statement.push_str(" FROM ");
//             statement.push_str(&tables.join(", "));
//         }
//
//         if let Some(condition) = self.condition {
//             statement.push_str(" WHERE ");
//             statement.push_str(condition);
//         }
//
//         if let Some(groups) = self.groups {
//             statement.push_str(" GROUP BY ");
//             statement.push_str(&groups.join(", "));
//         }
//
//         if let Some(permissions) = self.permissions {
//             statement.push_str(" PERMISSIONS");
//
//             if permissions.none {
//                 statement.push_str(" NONE");
//             }
//
//             if permissions.full {
//                 statement.push_str(" FULL");
//             }
//
//             if let Some(select) = permissions.select {
//                 statement.push_str(" FOR select ");
//                 statement.push_str(select);
//             }
//
//             if let Some(create) = permissions.create {
//                 statement.push_str(" FOR create ");
//                 statement.push_str(create);
//             }
//
//             if let Some(update) = permissions.update {
//                 statement.push_str(" FOR update ");
//                 statement.push_str(update);
//             }
//
//             if let Some(delete) = permissions.delete {
//                 statement.push_str(" FOR delete ");
//                 statement.push_str(delete);
//             }
//         }
//
//         statement
//     }
// }
//
//
// Statement syntax
// DEFINE TABLE @name
// 	[ DROP ]
// 	[ SCHEMAFULL | SCHEMALESS ]
// 	[ AS SELECT @projections
// 		FROM @tables
// 		[ WHERE @condition ]
// 		[ GROUP [ BY ] @groups ]
// 	]
// 	[ PERMISSIONS [ NONE | FULL
// 		| FOR select @expression
// 		| FOR create @expression
// 		| FOR update @expression
// 		| FOR delete @expression
// 	] ]
//
//
//

#[cfg(test)]
mod tests {

    use std::time::Duration;

    use crate::{
        query_remove::Table,
        query_select::{select, All},
        value_type_wrappers::SurrealId,
    };

    use super::*;

    #[test]
    fn test_define_for_statement_state_machine() {
        let name = DbField::new("name");

        let for_res = for_(ForCrudType::Create).where_(name.like("Oyelowo"));
        assert_eq!(
            for_res.to_string(),
            "FOR create\n\tWHERE name ~ $_param_00000000".to_string()
        );
        insta::assert_display_snapshot!(for_res);
        insta::assert_debug_snapshot!(for_res.get_bindings());
    }

    #[test]
    fn test_define_for_statement_state_machine_multiple() {
        use ForCrudType::*;
        let name = DbField::new("name");

        let for_res = for_(&[Create, Delete, Select, Update]).where_(name.is("Oyedayo"));
        assert_eq!(
            for_res.to_string(),
            "FOR create, delete, select, update\n\tWHERE name IS $_param_00000000".to_string()
        );
        insta::assert_display_snapshot!(for_res);
        insta::assert_debug_snapshot!(for_res.get_bindings());
    }
}
