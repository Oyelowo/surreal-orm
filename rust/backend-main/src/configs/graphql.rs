use actix_web::{web, HttpResponse};
use anyhow::Context;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use super::configuration::Environemnt;
use crate::configs::Configs;
use crate::post::{Post, PostMutationRoot, PostQueryRoot};
use crate::user::{User, UserMutationRoot, UserQueryRoot};
use wither::{mongodb::Client, prelude::Model};

#[derive(MergedObject, Default)]
pub struct Query(UserQueryRoot, PostQueryRoot);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutationRoot, PostMutationRoot);

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn get_graphql_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}

pub async fn index(schema: web::Data<GraphQLSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_playground() -> HttpResponse {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source)
}

fn get_error_message<T: Model>() -> String {
    format!("problem syncing {:?}", T::COLLECTION_NAME)
}

pub struct GraphQlApp;

impl GraphQlApp {
    pub async fn setup() -> anyhow::Result<Schema<Query, Mutation, EmptySubscription>> {
        let Configs {
            ref application,
            database,
        } = Configs::init();

        use Environemnt::*;
        let (limit_depth, limit_complexity) = match application.environment {
            Local | Development | Staging => (usize::max_value(), usize::max_value()),
            _ => (5, 7),
        };

        let db = Client::with_uri_str(database.get_url().as_ref())
            .await?
            .database(database.name.as_str());

        User::sync(&db).await.expect("Problem syncing users");
        // Post::sync(&db).await.context("problem syncing post")?;
        Post::sync(&db).await.context(get_error_message::<Post>())?;

        let schema = get_graphql_schema()
            .data(db)
            .limit_depth(limit_depth) // This and also limi_complexity will prevent the graphql playground document from showing because it's unable to do the complete tree parsing. TODO: Add it conditionally. i.e if not in development or test environemnt.
            .limit_complexity(limit_complexity)
            .finish();

        Ok(schema)
    }
}
