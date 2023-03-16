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
    query_remove::{RemoveScopeStatement, Runnable},
    query_select::{Duration, SelectStatement},
    query_update::UpdateStatement,
    BindingsList, DbField, DbFilter, Parametric, Queryable,
};

// DEFINE INDEX statement
// Just like in other databases, SurrealDB uses indexes to help optimize query performance.
// An index can consist of one or more fields in a table and can enforce a uniqueness constraint.
// If you don't intend for your index to have a uniqueness constraint, then the fields you select
// for your index should have a high degree of cardinality, meaning that there is a high amount
// of diversity between the data in the indexed table records.
//
// Requirements
// You must be authenticated as a root, namespace, or database user before you can use the DEFINE INDEX statement.
// You must select your namespace and database before you can use the DEFINE INDEX statement.
// Statement syntax
// DEFINE INDEX @name ON [ TABLE ] @table [ FIELDS | COLUMNS ] @fields [ UNIQUE ]
// Example usage
// Below is an example showing how to create a unique index for the email address field on a user table.
//
// -- Make sure that email addresses in the user table are always unique
// DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;

use std::collections::HashMap;

// Struct to represent a SurrealDB index definition
pub struct DefineIndexStatement {
    index_name: String,
    table_name: Option<String>,
    fields: Vec<String>,
    columns: Vec<String>,
    unique: Option<bool>,
    bindings: BindingsList,
}

pub struct Table(sql::Table);

impl Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl From<Table> for sql::Value {
    fn from(value: Table) -> Self {
        Self::Table(value.0)
    }
}

impl<T: Into<sql::Table>> From<T> for Table {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Deref for Table {
    type Target = sql::Table;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum Columns {
    Field(DbField),
    Fields(Vec<DbField>),
}
pub type Fields = Columns;

impl From<DbField> for Columns {
    fn from(value: DbField) -> Self {
        Self::Field(value)
    }
}

// impl<T: Into<DbField>> From<T> for Columns {
//     fn from(value: T) -> Self {
//         Self::Column(value.into())
//     }
// }

impl<const N: usize> From<&[DbField; N]> for Columns {
    fn from(value: &[DbField; N]) -> Self {
        Self::Fields(value.into_iter().map(ToOwned::to_owned).collect::<Vec<_>>())
    }
    // impl<T: Into<[const DbField]>> From<T> for Columns {
}

// impl<T: Into<Vec<DbField>>> From<T> for Columns {
//     fn from(value: T) -> Self {
//         Self::Fields(value.into())
//     }
// }

// TODO: Not doing any parametization or binding in this DefineIndexStatement
// as that is usually not recceiving input from external sources.
// I am still contemplating using parametization everywhere and exposing a feature flag
// to turn it on and off.
impl Parametric for Columns {
    fn get_bindings(&self) -> BindingsList {
        match self {
            // Columns::Column(c) => vec![Binding::new(sql::Value::Idiom(c.to_owned()))],
            Columns::Field(c) => vec![],
            Columns::Fields(cs) => vec![]
            // cs
            //     .into_iter()
            //     .map(|c| Binding::new(sql::Value::Idiom(c.to_owned())))
            //     .collect(),
        }
    }
}

pub type Index = Name;

pub fn define_index(index_name: impl Into<Index>) -> DefineIndexStatement {
    DefineIndexStatement::new(index_name)
}

impl DefineIndexStatement {
    pub fn new(index_name: impl Into<Index>) -> Self {
        // let binding_index_name = Binding::new(index_name.into()).with_description("Index name");
        let index_name: Index = index_name.into();
        let index_name: sql::Idiom = index_name.into();
        let index_name: String = index_name.to_string();

        Self {
            index_name,
            table_name: None,
            fields: vec![],
            columns: vec![],
            unique: None,
            bindings: vec![],
        }
    }

    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        let table: Table = table.into();

        // let binding = Binding::new(table).with_description("table name which fields are indexed");
        // self.table_name_param = Some(format!("{}", binding.get_param_dollarised()));
        // self.bindings.push(binding);

        self.table_name = Some(table.to_string());
        self
    }

    pub fn columns(mut self, columns: impl Into<Columns>) -> Self {
        // let binding =
        //     Binding::new(table.into()).with_description("table name which fields are indexed");
        //
        // self.table_name_param = Some(format!("{}", binding.get_param_dollarised()));
        // self.bindings.push(binding);
        let columns: Columns = columns.into();
        let columns = match columns {
            Columns::Field(f) => vec![f],
            Columns::Fields(fs) => fs,
        };
        self.columns.extend(
            columns
                .into_iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>(),
        );
        self
    }

    pub fn fields(mut self, fields: impl Into<Fields>) -> Self {
        let fields: Fields = fields.into();
        let fields = match fields {
            Columns::Field(f) => vec![f],
            Columns::Fields(fs) => fs,
        };
        self.fields.extend(
            fields
                .into_iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>(),
        );
        self
    }
    pub fn unique(mut self) -> Self {
        self.unique = Some(true);
        self
    }
}

impl Buildable for DefineIndexStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE INDEX {}", self.index_name);

        if let Some(table) = &self.table_name {
            query = format!("{query} ON TABLE {table}");
        }

        if !self.fields.is_empty() {
            let fields_str = self.fields.join(", ");
            query = format!("{query} FIELDS {fields_str}");
        }

        if !self.columns.is_empty() {
            let columns_str = self.columns.join(", ");
            query = format!("{query} COLUMNS {columns_str}");
        }
        // Define index
        if self.unique.unwrap_or(false) {
            query = format!("{query} UNIQUE");
        }
        query += ";";
        query
    }
}

impl Display for DefineIndexStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for DefineIndexStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Queryable for DefineIndexStatement {}

impl Runnable for DefineIndexStatement {}

#[cfg(test)]
mod tests {

    use std::time::Duration;

    use super::*;

    #[test]
    fn test_define_index_statement_single_field() {
        let email = DbField::new("email");

        let query = define_index("userEmailIndex")
            .on_table("user")
            .fields(email)
            .unique();

        assert_eq!(
            query.to_string(),
            "DEFINE INDEX userEmailIndex ON TABLE user FIELDS email UNIQUE;"
        );
        insta::assert_debug_snapshot!(query.get_bindings());
    }

    #[test]
    fn test_define_index_statement_single_column() {
        let email = DbField::new("email");

        let query = define_index("userEmailIndex")
            .on_table("user")
            .columns(email)
            .unique();

        assert_eq!(
            query.to_string(),
            "DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;"
        );
        insta::assert_debug_snapshot!(query.get_bindings());
    }

    #[test]
    fn test_define_index_statement_multiple_fields() {
        let age = DbField::new("age");
        let name = DbField::new("name");
        let email = DbField::new("email");
        let dob = DbField::new("dob");

        let query = define_index("alien_index")
            .on_table("alien")
            .fields(&[age, name, email, dob])
            .unique();

        assert_eq!(
            query.to_string(),
            "DEFINE INDEX alien_index ON TABLE alien FIELDS age, name, email, dob UNIQUE;"
        );
        insta::assert_debug_snapshot!(query.get_bindings());
    }

    #[test]
    fn test_define_index_statement_multiple_columns() {
        let age = DbField::new("age");
        let name = DbField::new("name");
        let email = DbField::new("email");
        let dob = DbField::new("dob");

        let query = define_index("alien_index")
            .on_table("alien")
            .columns(&[age, name, email, dob])
            .unique();

        assert_eq!(
            query.to_string(),
            "DEFINE INDEX alien_index ON TABLE alien COLUMNS age, name, email, dob UNIQUE;"
        );
        insta::assert_debug_snapshot!(query.get_bindings());
    }
}
