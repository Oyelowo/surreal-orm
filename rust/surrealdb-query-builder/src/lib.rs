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
pub mod clause;
pub(crate) mod errors;
pub mod field;
mod field_updater;
pub mod filter;
pub(crate) mod helpers;
pub(crate) mod internal;
pub mod links;
pub mod model_id;
mod operators_macros;
mod param;
pub(crate) mod raw_statements;
mod sql_components;
pub(crate) mod sql_traits;
mod statements;

pub mod functions;

pub mod query {
    pub use super::raw_statements::{RawStatement, ToRawStatement};
    pub use super::statements::statements::*;
}

pub use binding::{BindingsList, Parametric};
pub use field::Field;
pub use field::Operatable;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
pub use sql::Clause;
use sql::RawStatement;
pub use sql::Table;
use statements::define_field::DefineFieldStatement;
use statements::define_table::DefineTableStatement;
pub use surrealdb::sql::json;
pub use surrealdb::sql::Value;

pub mod sql {
    pub use super::binding::*;
    pub use super::clause::*;
    pub use super::field::*;
    pub use super::field_updater::*;
    pub use super::param::*;
    pub use super::raw_statements::*;
    pub use super::sql_components::*;
    pub use super::sql_traits::*;
}

pub mod utils {
    pub use super::filter::cond;
}
pub mod prelude {
    use super::statements;
}

// pub use field::Param;
// pub use field::ParamsExtractor;
pub use surrealdb::opt::RecordId;

// SurrealdbModel is a market trait signifying superset of SurrealdbNode and SurrealdbEdge. IOW, both are
pub trait SurrealdbModel {
    // fn table_name() -> surrealdb::sql::Table;
    fn table_name() -> Table;
    fn get_serializable_field_names() -> Vec<&'static str>;
    fn define_table() -> RawStatement;
    fn define_fields() -> Vec<RawStatement>;
}

pub trait SurrealdbNode: SurrealdbModel + Serialize {
    type Schema;
    type TableNameChecker;
    fn schema() -> Self::Schema;
    // fn get_key<T: Into<RecordId>>(&self) -> ::std::option::Option<&T>;
    fn get_key<T: From<RecordId>>(self) -> ::std::option::Option<T>;
    // fn get_table_name() -> surrealdb::sql::Table;
    fn get_table_name() -> Table;
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
    // fn get_table_name() -> surrealdb::sql::Table;
    fn get_table_name() -> Table;
}

pub trait Schemaful {
    fn get_connection(&self) -> String;
}

pub type ErrorList = Vec<String>;
pub trait Erroneous {
    fn get_errors(&self) -> ErrorList {
        vec![]
    }
}
