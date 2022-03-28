pub(crate) mod error;
pub(crate) mod post;
pub(crate) mod user;

use anyhow::Context;
use mongodb::Database;
use post::{Post, PostMutationRoot, PostQueryRoot, PostSubscriptionRoot};
use user::{User, UserMutationRoot, UserQueryRoot, UserSubscriptionRoot};

use async_graphql::{MergedObject, MergedSubscription, Schema, SchemaBuilder};
use wither::Model;

//pub use crate::app;

// Add new models here
pub async fn sync_mongo_models(db: &Database) -> anyhow::Result<()> {
    // Todo: Make this repetitive work into a macro_rules or function macro if need be. e.g sync_mongo_models!(db; User, Post)
    User::sync(db).await.expect("Problem syncing users");
    // Post::sync(db).await.context("problem syncing post")?;
    Post::sync(db).await.context(get_error_message::<Post>())?;
    Ok(())
}

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

fn get_error_message<T: Model>() -> String {
    format!("problem syncing {:?}", T::COLLECTION_NAME)
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
