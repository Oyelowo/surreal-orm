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
use surrealdb::sql;

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
    index_name_param: String,
    table_name_param: Option<String>,
    fields_params: Vec<String>,
    columns_params: Vec<String>,
    unique: Option<bool>,
    bindings: BindingsList,
}

pub struct Table(sql::Table);

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
    Column(sql::Idiom),
    Columns(Vec<sql::Idiom>),
}

impl Parametric for Columns {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Columns::Column(c) => vec![Binding::new(sql::Value::Idiom(c.to_owned()))],
            Columns::Columns(cs) => cs
                .into_iter()
                .map(|c| Binding::new(sql::Value::Idiom(c.to_owned())))
                .collect(),
        }
    }
}

pub type Index = Name;

pub fn define_index(index_name: impl Into<Index>) -> DefineIndexStatement {
    DefineIndexStatement::new(index_name)
}

impl DefineIndexStatement {
    pub fn new(index_name: impl Into<Index>) -> Self {
        let binding_index_name = Binding::new(index_name.into()).with_description("Index name");

        Self {
            index_name_param: binding_index_name.get_param_dollarised(),
            table_name_param: None,
            fields_params: vec![],
            columns_params: vec![],
            unique: None,
            bindings: vec![binding_index_name],
        }
    }

    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        let binding =
            Binding::new(table.into()).with_description("table name which fields are indexed");

        self.table_name_param = Some(format!("{}", binding.get_param_dollarised()));
        self.bindings.push(binding);
        self
    }

    pub fn columns(mut self, columns: impl Into<Columns>) -> Self {
        // let binding =
        //     Binding::new(table.into()).with_description("table name which fields are indexed");
        //
        // self.table_name_param = Some(format!("{}", binding.get_param_dollarised()));
        // self.bindings.push(binding);
        self
    }
}

// Build the query
//     fn build(&self) -> String {
//         let mut query = String::new();
//
//         // Define namespace and database
//         if let Some(namespace) = &self.namespace {
//             query += &format!("USE NAMESPACE {};", namespace);
//         }
//         if let Some(database) = &self.database {
//             query += &format!("USE DATABASE {};", database);
//         }
//
//         // Define index
//         if let Some(index) = &self.index {
//             let fields_str = index.fields_params.join(", ");
//             let unique_str = if index.unique { "UNIQUE" } else { "" };
//             query += &format!(
//                 "DEFINE INDEX {} ON TABLE {} FIELDS {} {};",
//                 index.index_name_param, index.table_name_param, fields_str, unique_str
//             );
//         }
//
//         query
//     }
// }
