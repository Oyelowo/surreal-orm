/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(unused_imports)]

pub use model_id::SurId;
pub mod db_field;
pub mod operators_macros;
pub mod query_insert;
pub mod query_select;
pub mod value_type_wrappers;
// pub mod querydb;
pub mod prelude {
    use super::query_select;
}

pub mod links;
pub mod model_id;

pub use db_field::DbField;
pub use db_field::DbFilter;
// pub use db_field::Param;
// pub use db_field::ParamsExtractor;
pub use surrealdb::opt::RecordId;
use surrealdb::sql;

pub trait SurrealdbNode {
    type Schema;
    type TableNameChecker;
    fn schema() -> Self::Schema;
    // fn get_key<T: Into<RecordId>>(&self) -> ::std::option::Option<&T>;
    fn get_key<T: From<RecordId>>(self) -> ::std::option::Option<T>;
    fn get_table_name() -> sql::Table;
}

pub trait SurrealdbEdge {
    type In;
    type Out;
    type TableNameChecker;
    type Schema;

    fn schema() -> Self::Schema;
    fn get_table_name() -> sql::Table;
    // fn get_key(&self) -> ::std::option::Option<&SurId>;
    fn get_key<T: From<RecordId>>(self) -> ::std::option::Option<T>;
}

// pub enum Clause {
//     All,
//     Where(DbFilter),
//     Id(SurId),
// }

// pub fn format_filter(filter: DbFilter, _table_name: &'static str) -> String {
pub fn format_filter(filter: impl Into<DbFilter>) -> String {
    let filter: DbFilter = filter.into();
    if filter.to_string().is_empty() {
        "".into()
    } else {
        format!("[WHERE {filter}]")
    }
}

// pub fn format_clause(clause: Clause, table_name: &'static str) -> String {
//     match clause {
//         Clause::All => "".into(),
//         Clause::Where(filter) => {
//             let filter = filter.to_string();
//             format!("[WHERE {filter}]")
//         }
//         Clause::Id(id) => {
//             if !id
//                 .to_string()
//                 .starts_with(format!("{table_name}:").as_str())
//             {
//                 panic!("invalid id {id}. Id does not belong to table {table_name}")
//             }
//             format!("[WHERE id = {id}]")
//         }
//     }
// }
