#![allow(unused_imports)]

pub use model_id::SurId;
use qbuilder::QueryBuilder;

pub mod links;
// pub mod main_backup;
pub mod model_id;
pub mod node_builder;
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

#[derive(serde::Serialize, Debug, Default)]
pub struct DbField(String);

impl DbField {
    pub fn push_str(&mut self, string: &str) {
        self.0.push_str(string)
    }

    pub fn __as__(&self, alias: impl std::fmt::Display) -> String {
        format!("{self} AS {alias}")
    }
}

impl From<String> for DbField {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}
impl From<&str> for DbField {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
impl From<DbField> for String {
    fn from(value: DbField) -> Self {
        value.0
    }
}

impl std::fmt::Display for DbField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

/* impl std::fmt::Debug for DbField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
} */

impl query_builder::ToNodeBuilder for DbField {}

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

pub mod query_builder {

    pub fn query() -> super::qbuilder::QueryBuilder<'static> {
        super::qbuilder::QueryBuilder::new()
    }

    pub use super::node_builder::{NodeBuilder, ToNodeBuilder};
}
