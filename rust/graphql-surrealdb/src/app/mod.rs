pub(crate) mod post;
pub(crate) mod user;
use post::{Post, PostMutationRoot, PostQueryRoot, PostSubscriptionRoot};
use user::{User, UserMutationRoot, UserQueryRoot, UserSubscriptionRoot};

use async_graphql::{MergedObject, MergedSubscription, Schema, SchemaBuilder};

// Merged Queries
#[derive(MergedObject, Default)]
pub struct Query(UserQueryRoot, PostQueryRoot);

// Merged Mutations
#[derive(MergedObject, Default)]
pub struct Mutation(UserMutationRoot, PostMutationRoot);

// Merged Subscription
#[derive(MergedSubscription, Default)]
pub struct Subscription(UserSubscriptionRoot, PostSubscriptionRoot);

pub type MyGraphQLSchema = Schema<Query, Mutation, Subscription>;

pub fn get_my_graphql_schema() -> SchemaBuilder<Query, Mutation, Subscription> {
    MyGraphQLSchema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Authentication failed.")]
    InvalidPassword(#[source] anyhow::Error),

    #[error("Forbidden failed.")]
    Forbidden(#[source] anyhow::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
