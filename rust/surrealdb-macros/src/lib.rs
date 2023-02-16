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

pub mod links;
// pub mod main_backup;
pub mod model_id;
pub mod qbuilder;

pub trait SurrealdbNode {
    type Schema;
    type TableNameChecker;
    fn schema() -> Self::Schema;
    fn get_key(&self) -> ::std::option::Option<&SurId>;
}

pub trait SurrealdbEdge {
    type In;
    type Out;
    type TableNameChecker;
    type Schema;

    fn schema() -> Self::Schema;
    fn get_key(&self) -> ::std::option::Option<&SurId>;
}

pub enum Clause<'a> {
    All,
    Where(QueryBuilder<'a>),
    Id(SurId),
}

pub fn format_clause(clause: Clause, table_name: &'static str) -> String {
    match clause {
        Clause::All => "".into(),
        Clause::Where(where_clause) => {
            let where_clause = where_clause.build();
            if !where_clause.to_lowercase().starts_with("where") {
                panic!("Invalid where clause, must start with `WHERE`")
            }
            format!("[{where_clause}]")
        }
        Clause::Id(id) => {
            if !id
                .to_string()
                .starts_with(format!("{table_name}:").as_str())
            {
                panic!("invalid id {id}. Id does not belong to table {table_name}")
            }
            format!("[WHERE id = {id}]")
        }
    }
}
pub use db_field::DbField;
pub mod query_builder_old {

    pub fn query() -> super::qbuilder::QueryBuilder<'static> {
        super::qbuilder::QueryBuilder::new()
    }
}
