/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![allow(unused_imports)]

use std::ops::Deref;

use db_field::Empty;
pub mod db_field;
pub mod operators_macros;
pub mod query_create;
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

pub mod links;
pub mod model_id;

pub use db_field::BindingsList;
pub use db_field::DbField;
pub use db_field::DbFilter;
pub use db_field::Parametric;
use query_select::SelectStatement;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
// pub use db_field::Param;
// pub use db_field::ParamsExtractor;
pub use surrealdb::opt::RecordId;
use surrealdb::sql;
use value_type_wrappers::SurrealId;

pub trait Queryable {}

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

pub fn where_(
    condition: impl Parametric + Into<DbFilter> + std::fmt::Display + Erroneous,
) -> DbFilter {
    // let filter = DbFilter::new(format!("{condition}")).___update_bindings(&condition);

    if condition.get_errors().is_empty() {
        // TODO: Maybe pass to DB filter and check and return Result<DbFilter> in relate_query
    }
    condition.into()
}

#[derive(Debug, Clone)]
pub enum Clause {
    Empty,
    Where(DbFilter),
    Query(SelectStatement),
    Id(SurrealId),
}

impl From<&Self> for Clause {
    fn from(value: &Self) -> Self {
        value.clone()
    }
}

impl Parametric for Clause {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Clause::Empty => vec![],
            Clause::Where(filter) => filter.get_bindings(),
            Clause::Query(select_statement) => select_statement.get_bindings(),
            Clause::Id(id) => id.get_bindings(),
        }
    }
}

impl Clause {
    pub fn get_errors(&self, table_name: &'static str) -> Vec<String> {
        let mut errors = vec![];
        if let Clause::Id(id) = self {
            if !id
                .to_string()
                .starts_with(format!("{table_name}:").as_str())
            {
                errors.push(format!(
                    "invalid id {id}. Id does not belong to table {table_name}"
                ))
            }
        }
        errors
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let clause = match self {
            Clause::Empty => "".into(),
            Clause::Where(filter) => {
                format!("[WHERE {filter}]")
            }
            Clause::Id(surreal_id) => {
                // The Table name component of the Id comes from the macro. e.g For student:5, the Schema which this is wrapped into provide. So all we need here is the id component, student
                format!(":{}", surreal_id.id)
            }
            Clause::Query(select_statement) => format!("({select_statement})"),
        };

        write!(f, "{}", clause)
    }
}

impl From<SurrealId> for Clause {
    fn from(value: SurrealId) -> Self {
        Self::Id(value)
    }
}

impl From<&SurrealId> for Clause {
    fn from(value: &SurrealId) -> Self {
        Self::Id(value.to_owned())
    }
}

impl From<DbField> for Clause {
    fn from(value: DbField) -> Self {
        Self::Where(value.into())
    }
}

impl From<&DbField> for Clause {
    fn from(value: &DbField) -> Self {
        Self::Where(value.to_owned().into())
    }
}

impl From<DbFilter> for Clause {
    fn from(value: DbFilter) -> Self {
        Self::Where(value)
    }
}

impl From<&DbFilter> for Clause {
    fn from(value: &DbFilter) -> Self {
        Self::Where(value.to_owned())
    }
}

impl From<Empty> for Clause {
    fn from(value: Empty) -> Self {
        Self::Empty
    }
}

impl From<SelectStatement> for Clause {
    fn from(value: SelectStatement) -> Self {
        Self::Query(value.into())
    }
}

impl From<&SelectStatement> for Clause {
    fn from(value: &SelectStatement) -> Self {
        Self::Query(value.to_owned().into())
    }
}
