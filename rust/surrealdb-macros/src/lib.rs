/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![allow(unused_imports)]

use std::fmt::Display;
use std::ops::Deref;

pub(crate) mod binding;
pub(crate) mod clause;
pub(crate) mod errors;
pub mod field;
mod field_updater;
pub mod filter;
pub mod links;
pub mod model_id;
mod operators_macros;
mod param;
mod query_create;
mod query_define_database;
mod query_define_event;
mod query_define_field;
mod query_define_index;
mod query_define_login;
mod query_define_namespace;
mod query_define_scope;
mod query_define_table;
mod query_define_token;
mod query_delete;
pub(crate) mod query_for;
mod query_ifelse;
mod query_info;
mod query_insert;
mod query_let;
mod query_relate;
mod query_remove;
mod query_select;
mod query_sleep;
mod query_transaction;
mod query_update;
mod query_use;
mod sql_components;
pub(crate) mod sql_traits;

use binding::{BindingsList, Parametric};
pub use field::Field;
pub use field::Operatable;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use sql::Clause;
pub use sql::Table;

pub mod sql {
    pub use super::clause::*;
    pub use super::field::*;
    pub use super::field_updater::*;
    pub use super::param::*;
    pub use super::sql_components::*;
    pub use super::sql_traits::*;
}

pub mod utils {
    pub use super::filter::cond;
    pub use super::query_for::for_;
}
pub mod statements {
    pub use super::query_create::{create, CreateStatement};
    pub use super::query_define_database::{define_database, DefineDatabaseStatement};
    pub use super::query_define_event::{define_event, DefineEventStatement};
    pub use super::query_define_field::{define_field, DefineFieldStatement};
    pub use super::query_define_index::{define_index, DefineIndexStatement};
    pub use super::query_define_login::{define_login, DefineLoginStatement};
    pub use super::query_define_namespace::{define_namespace, DefineNamespaceStatement};
    pub use super::query_define_scope::{define_scope, DefineScopeStatement};
    pub use super::query_define_table::{define_table, DefineTableStatement};
    pub use super::query_define_token::{define_token, DefineTokenStatement};
    pub use super::query_delete::{delete, DeleteStatement};
    pub use super::query_for::for_;
    pub use super::query_ifelse::{if_, IfStatement};
    pub use super::query_info::{info_for, InfoStatement};
    pub use super::query_insert::{insert, InsertStatement};
    pub use super::query_let::{let_, LetStatement};
    pub use super::query_relate::{relate, RelateStatement};
    pub use super::query_remove::{
        remove_database, remove_event, remove_field, remove_index, remove_login, remove_namespace,
        remove_scope, remove_table, remove_token,
    };
    pub use super::query_select::{order, select, Order, SelectStatement, TargettablesForSelect};
    pub use super::query_sleep::{sleep, SleepStatement};
    pub use super::query_transaction::{begin_transaction, BeginTransactionStatement};
    pub use super::query_update::{update, TargettablesForUpdate, UpdateStatement};
    pub use super::query_use::{use_, UseStatement};
}
pub mod prelude {
    use super::query_select;
}

// pub use field::Param;
// pub use field::ParamsExtractor;
pub use surrealdb::opt::RecordId;

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
