#![warn(unused_imports)]
#[path = "../user/mod.rs"]
mod user;

use actix_web::{web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use super::configuration::Environemnt;
use crate::configs::Configs;
use async_trait::async_trait;
use user::{User, UserMutationRoot, UserQueryRoot};
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::Client,
    prelude::Model,
    Result,
};

#[derive(MergedObject, Default)]
pub struct Query(UserQueryRoot);
// pub struct Query(StarWarQueryRoot, UserQueryRoot);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutationRoot);

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;
// pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

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

pub struct GraphQlApp;

impl GraphQlApp {
    pub async fn setup() -> anyhow::Result<Schema<Query, Mutation, EmptySubscription>> {
        let Configs { application, .. } = Configs::init();

        use Environemnt::*;
        let (limit_depth, limit_complexity) = match application.environment {
            Local | Development | Staging => (usize::max_value(), usize::max_value()),
            _ => (5, 7),
        };

        let uri = "mongodb://localhost:27017/";
        let db = Client::with_uri_str(uri).await?.database("mydb");

        User::sync(&db).await.expect("problem syncing user");

        let schema = get_graphql_schema()
            .data(db)
            .limit_depth(limit_depth) // This and also limi_complexity will prevent the graphql playground document from showing because it's unable to do the complete tree parsing. TODO: Add it conditionally. i.e if not in development or test environemnt.
            .limit_complexity(limit_complexity)
            .finish();

        Ok(schema)
    }
}
