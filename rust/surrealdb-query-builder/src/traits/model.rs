/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use serde::Serialize;
use surrealdb::opt::RecordId;

use crate::{
    types::{Clause, Table},
    Table,
};

use super::Raw;

// SurrealdbModel is a market trait signifying superset of SurrealdbNode and SurrealdbEdge. IOW, both are
pub trait SurrealdbModel {
    // fn table_name() -> surrealdb::sql::Table;
    fn table_name() -> Table;
    fn get_serializable_field_names() -> Vec<&'static str>;
    fn define_table() -> Raw;
    fn define_fields() -> Vec<Raw>;
}

pub trait SurrealdbNode: SurrealdbModel + Serialize {
    type Schema;
    type TableNameChecker;
    fn schema() -> Self::Schema;
    // fn get_key<T: Into<RecordId>>(&self) -> ::std::option::Option<&T>;
    fn get_key<T: From<RecordId>>(self) -> Option<T>;
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
    fn get_key<T: From<RecordId>>(self) -> Option<T>;
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
