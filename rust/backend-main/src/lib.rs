use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

pub mod starwar;
pub mod user;

// pub use starwar as _;
use starwar::query::StarWarQueryRoot;
use user::query::UserQueryRoot;


#[derive(MergedObject, Default)]
pub struct Query(StarWarQueryRoot, UserQueryRoot);

pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn get_graphql_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    return Schema::build(Query::default(), EmptyMutation, EmptySubscription);
}
