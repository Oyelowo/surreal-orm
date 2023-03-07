/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(unused_imports)]

use std::ops::Deref;

use db_field::Empty;
pub mod db_field;
pub mod operators_macros;
pub mod query_insert;
pub mod query_relate;
pub mod query_select;
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
    fn with(filterable: impl Into<DbFilter>) -> Self::Schema;
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

// pub fn format_filter(filter: DbFilter, _table_name: &'static str) -> String {
pub fn format_filter(filter: impl Into<DbFilter>) -> String {
    let filter: DbFilter = filter.into();
    // println!("FFFFILEEERRR {}", filter);
    if filter.to_string().is_empty() {
        "".into()
    } else {
        format!("[WHERE {filter}]")
    }
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

pub enum Clause {
    Empty,
    Where(DbFilter),
    Query(SelectStatement),
    Id(SurrealId),
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
// fn fdfdf<T>(xx: impl Into<Clause<T>>) {}
// pub fn format_clause<T: Serialize + DeserializeOwned>(
pub fn format_clause(clause: Clause, table_name: &'static str) -> String {
    match clause {
        Clause::Empty => "".into(),
        Clause::Where(filter) => {
            let filter = filter.to_string();
            format!("[WHERE {filter}]")
        }
        Clause::Id(id) => {
            if !id
                .to_string()
                .starts_with(format!("{table_name}:").as_str())
            {
                panic!("invalid id {id}. Id does not belong to table {table_name}")
            }
            // format!("[WHERE id = {id}]")
            format!("{id}]")
        }
        Clause::Query(select_statement) => format!("({select_statement})"),
    }
}

// impl<T> Parametric for T
// where
//     T: SurrealdbEdge + DeserializeOwned + Serialize,
// {
//     fn get_bindings(&self) -> BindingsList {
//         let value = self;
//         // let fields_names = get_field_names(value);
//         let field_names = T::get_serializable_field_names();
//
//         field_names
//             .into_iter()
//             .map(|field_name| {
//                 let field_value = get_field_value(value, &field_name)
//                     .expect("Unable to get value name. This should never happen!");
//                 Binding::new(field_value).with_name(field_name.into())
//             })
//             .collect::<Vec<_>>()
//     }
// }

// struct Mana<T: SurrealdbEdge + Serialize>(T);
//
// impl<T> Into<sql::Value> for Mana<T>
// where
//     T: SurrealdbEdge + Serialize,
// {
//     fn into(self) -> sql::Value {
//         // self.0;
//         sql::Value::from(self.0)
//     }
// }
