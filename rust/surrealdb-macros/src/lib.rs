/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(unused_imports)]

pub use model_id::SurId;
use qbuilder::QueryBuilder;
pub mod db_field;
pub mod operators_macros;
pub mod query_builder;
pub mod prelude {
    use super::query_builder;
}

pub mod links;
pub mod model_id;
pub mod qbuilder;

pub use db_field::DbField;
pub use db_field::DbFilter;
pub mod query_builder_old {

    pub fn query() -> super::qbuilder::QueryBuilder<'static> {
        super::qbuilder::QueryBuilder::new()
    }
}

pub trait SurrealdbNode {
    type Schema;
    type TableNameChecker;
    fn schema() -> Self::Schema;
    fn get_key(&self) -> ::std::option::Option<&SurId>;
    fn get_table_name() -> &'static str;
}

pub trait SurrealdbEdge {
    type In;
    type Out;
    type TableNameChecker;
    type Schema;

    fn schema() -> Self::Schema;
    fn get_table_name() -> &'static str;
    fn get_key(&self) -> ::std::option::Option<&SurId>;
}

// pub enum Clause {
//     All,
//     Where(DbFilter),
//     Id(SurId),
// }

pub fn format_filter(filter: DbFilter, _table_name: &'static str) -> String {
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
