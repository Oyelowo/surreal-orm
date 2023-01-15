pub trait Edge {
    type EdgeChecker;
    type InNode;
    type OutNode;
    // const EDGE_RELATION: &'static str;
    // fn to(&self) -> ::proc_macro2::TokenStream;
    // fn from(&self) -> ::proc_macro2::TokenStream;
    // fn km(&self) -> String;
}

// Re-export surrealdbmodel proc macro alongside the trait.
// With this, users dont have to import both the derive macro and trait
// themselves. They can just simple `use surrealdb_macros::SurrealdbModel`
pub use surrealdb_derive::SurrealdbModel;
pub trait SurrealdbModel {
    type Schema<const T: usize>;
    fn get_schema() -> Self::Schema<0>;
    // fn get_key(&self) -> Key;
    fn get_key(&self) -> ::std::option::Option<String>;
}
pub struct Key(String);

pub mod query {
    pub use surreal_simple_querybuilder::prelude::*;
}

