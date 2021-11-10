use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

pub mod starwar;

// pub use starwar as _;
use starwar::query::StarWarQueryRoot;

#[derive(MergedObject, Default)]
pub struct Query(StarWarQueryRoot);

pub fn get_graphql_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    return Schema::build(Query::default(), EmptyMutation, EmptySubscription);
}
