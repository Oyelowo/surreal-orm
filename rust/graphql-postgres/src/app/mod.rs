use async_graphql::{EmptySubscription, MergedObject, MergedSubscription, Schema, SchemaBuilder};

use self::{
    post::{PostMutationRoot, PostQueryRoot},
    user::{UserMutationRoot, UserQueryRoot},
};

pub(crate) mod post;
pub(crate) mod user;

// Merged Queries
#[derive(MergedObject, Default)]
pub struct Query(UserQueryRoot, PostQueryRoot);

// Merged Mutations
#[derive(MergedObject, Default)]
pub struct Mutation(UserMutationRoot, PostMutationRoot);

// Merged Subscription
#[derive(MergedSubscription, Default)]
pub struct Subscription(EmptySubscription);

pub type MyGraphQLSchema = Schema<Query, Mutation, Subscription>;

pub fn get_my_graphql_schema() -> SchemaBuilder<Query, Mutation, Subscription> {
    MyGraphQLSchema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
}
