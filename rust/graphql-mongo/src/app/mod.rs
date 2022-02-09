pub(crate) mod post;
pub(crate) mod user;

use anyhow::Context;
use mongodb::Database;
use post::{Post, PostMutationRoot, PostQueryRoot};
use user::{User, UserMutationRoot, UserQueryRoot};

use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
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

#[derive(MergedObject, Default)]
pub struct Query(UserQueryRoot, PostQueryRoot);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutationRoot, PostMutationRoot);

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn get_graphql_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}

fn get_error_message<T: Model>() -> String {
    format!("problem syncing {:?}", T::COLLECTION_NAME)
}
