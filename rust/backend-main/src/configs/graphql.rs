#![warn(unused_imports)]
#[path = "../starwar/mod.rs"]
mod starwar;
#[path = "../user/mod.rs"]
mod user;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use anyhow::{Context, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use async_graphql_actix_web::{Request, Response};
use common::{self, alt_good_morning, good_morning, maths, sum};
use std::env::{self, VarError};

use starwar::{StarWarQueryRoot, StarWars};
use user::{UserData, UserMutationRoot, UserQueryRoot};

use super::configuration::{Environemnt, EnvironmentVariableError};

#[derive(MergedObject, Default)]
pub struct Query(StarWarQueryRoot, UserQueryRoot);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutationRoot);

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;
// pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn get_graphql_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    return Schema::build(Query::default(), Mutation::default(), EmptySubscription);
}

pub async fn index(schema: web::Data<GraphQLSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}


pub async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}

pub struct GraphQlApp;

impl GraphQlApp {
    pub fn setup() -> Result<Schema<Query, Mutation, EmptySubscription>, EnvironmentVariableError> {
        let env = Environemnt::try_from(
            env::var("RUST_ENV").unwrap_or_else(|e| format!("problem:{:?}", e)),
        )?;

        use Environemnt::*;
        let limit = match env {
            LOCAL | DEVEVELOPMENT | STAGING => usize::max_value(),
            _ => 5,
        };

        let schema = get_graphql_schema()
            .data(StarWars::new())
            .data(UserData::new())
            .limit_depth(limit)
            // .limit_depth(5) // This and also limi_complexity will prevent the graphql playground document from showing because it's unable to do the complete tree parsing. TODO: Add it conditionally. i.e if not in development or test environemnt.
            .finish();

        Ok(schema)
    }
}
