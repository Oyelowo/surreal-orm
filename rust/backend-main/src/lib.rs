use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

pub mod starwar;
pub mod user;

// pub use starwar as _;
use starwar::query::StarWarQueryRoot;
use user::query::UserQueryRoot;

use user::mutation::UserMutationRoot;


#[derive(MergedObject, Default)]
pub struct Query(StarWarQueryRoot, UserQueryRoot);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutationRoot);

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;
// pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn get_graphql_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    return Schema::build(Query::default(), Mutation::default(), EmptySubscription);
}
