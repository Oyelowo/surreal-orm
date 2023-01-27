#![allow(unused_imports)]

pub mod links;
pub mod model_id;
pub mod node_builder;

pub trait SurrealdbNode {
    type Schema;
    fn get_schema() -> Self::Schema;
    fn get_key(&self) -> ::std::option::Option<String>;
}

pub trait SurrealdbEdge {
    type In;
    type Out;
    type TableNameChecker;
    fn get_key(&self) -> ::std::option::Option<String>;
}

#[derive(serde::Serialize, Debug, Default)]
pub struct DbField(String);

impl DbField {
    pub fn push_str(&mut self, string: &str) {
        self.0.push_str(string)
    }

    pub fn __as__(&self, alias: impl std::fmt::Display) -> String {
        // let xx = self.___________store;
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

// impl ToNodeBuilder2 for DbField {}

pub enum Clause {
    All,
    Where(String),
    // Change to SurId
    Id(String),
}

pub fn format_clause(clause: Clause, table_name: &'static str) -> String {
    match clause {
        Clause::All => "".into(),
        Clause::Where(where_clause) => {
            if !where_clause.to_lowercase().starts_with("where") {
                panic!("Invalid where clause, must start with `WHERE`")
            }
            format!("[{where_clause}]")
        }
        Clause::Id(id) => {
            if !id
                .to_lowercase()
                .starts_with(format!("{table_name}:").as_str())
            {
                // let xx = format!("invalid id {id}. Id does not belong to table {table_name}")
                //     .as_str();
                panic!("invalid id {id}. Id does not belong to table {table_name}")
            }
            format!("[WHERE id = {id}]")
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EdgeDirection {
    OutArrowRight,
    InArrowLeft,
}

impl std::fmt::Display for EdgeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrow_direction = match self {
            EdgeDirection::OutArrowRight => "->",
            EdgeDirection::InArrowLeft => "<-",
        };
        f.write_str(arrow_direction)
    }
}
impl From<EdgeDirection> for String {
    fn from(direction: EdgeDirection) -> Self {
        match direction {
            EdgeDirection::OutArrowRight => "->".into(),
            EdgeDirection::InArrowLeft => "<-".into(),
        }
    }
}
///////////////////
// pub trait Edge {
//     type EdgeChecker;
//     type InNode;
//     type OutNode;
//     // const EDGE_RELATION: &'static str;
//     // fn to(&self) -> ::proc_macro2::TokenStream;
//     // fn from(&self) -> ::proc_macro2::TokenStream;
//     // fn km(&self) -> String;
// }

// Re-export surrealdbmodel proc macro alongside the trait.
// With this, users dont have to import both the derive macro and trait
// themselves. They can just simple `use surrealdb_macros::SurrealdbModel`
// pub use surrealdb_derive::SurrealdbModel;
// pub trait SurrealdbModel {
//     type Schema<const T: usize>;
//     fn get_schema() -> Self::Schema<0>;
//     // fn get_key(&self) -> Key;
//     fn get_key(&self) -> ::std::option::Option<String>;
// }
// pub struct Key(String);

pub mod query_builder {
    use surreal_simple_querybuilder::prelude as query_builder;

    pub fn query() -> query_builder::QueryBuilder<'static> {
        query_builder::QueryBuilder::new()
    }

    // pub use query_builder::*;
    pub use query_builder::{model, NodeBuilder, SchemaField, SchemaFieldType, ToNodeBuilder};
}
