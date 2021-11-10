use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub mod model;
pub mod type_gql;

mod query_root;
mod query_human;
mod query_droid;

pub mod query {
    pub use super::{query_human::Human, query_droid::Droid, query_root::{QueryRoot}};
}

pub type StarWarsSchema = Schema<query::QueryRoot, EmptyMutation, EmptySubscription>;
