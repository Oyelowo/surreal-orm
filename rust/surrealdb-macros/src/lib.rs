/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![allow(unused_imports)]

use std::fmt::Display;
use std::ops::Deref;

use field::Conditional;
pub mod field;
pub mod operators_macros;
pub mod param;
pub mod query_create;
pub mod query_define_database;
pub mod query_define_event;
pub mod query_define_field;
pub mod query_define_index;
pub mod query_define_login;
pub mod query_define_namespace;
pub mod query_define_scope;
pub mod query_define_table;
pub mod query_define_token;
pub mod query_delete;
pub mod query_ifelse;
pub mod query_info;
pub mod query_insert;
pub mod query_let;
pub mod query_relate;
pub mod query_remove;
pub mod query_select;
pub mod query_sleep;
pub mod query_transaction;
pub mod query_update;
pub mod query_use;
pub mod value_type_wrappers;
// pub mod querydb;
pub mod prelude {
    use super::query_select;
}

pub mod clause;
pub mod links;
pub mod model_id;

pub use clause::*;
pub use field::BindingsList;
pub use field::Field;
pub use field::Filter;
pub use field::Operatable;
pub use field::Parametric;
use query_insert::Buildable;
use query_select::SelectStatement;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
// pub use field::Param;
// pub use field::ParamsExtractor;
pub use surrealdb::opt::RecordId;
use surrealdb::sql;
use value_type_wrappers::SurrealId;

pub trait Queryable: Parametric + Buildable + Display {}

// SurrealdbModel is a market trait signifying superset of SurrealdbNode and SurrealdbEdge. IOW, both are
pub trait SurrealdbModel {
    fn table_name() -> sql::Table;
    fn get_serializable_field_names() -> Vec<&'static str>;
}

pub trait SurrealdbNode: SurrealdbModel + Serialize {
    type Schema;
    type TableNameChecker;
    fn schema() -> Self::Schema;
    // fn get_key<T: Into<RecordId>>(&self) -> ::std::option::Option<&T>;
    fn get_key<T: From<RecordId>>(self) -> ::std::option::Option<T>;
    fn get_table_name() -> sql::Table;
    fn with(clause: impl Into<Clause>) -> Self::Schema;
}

pub trait SurrealdbEdge: SurrealdbModel + Serialize {
    type In;
    type Out;
    type TableNameChecker;
    type Schema;

    fn schema() -> Self::Schema;
    // fn get_key(&self) -> ::std::option::Option<&SurId>;
    fn get_key<T: From<RecordId>>(self) -> ::std::option::Option<T>;
    fn get_table_name() -> sql::Table;
}

pub trait Schemaful {
    fn get_connection(&self) -> String;
}

pub trait Erroneous {
    fn get_errors(&self) -> Vec<String>;
}
