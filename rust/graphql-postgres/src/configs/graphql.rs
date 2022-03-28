use std::time::Duration;

use actix_web::{web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use super::configuration::Environemnt;
use crate::{
    app::{
        post::{PostMutationRoot, PostQueryRoot},
        user::{UserMutationRoot, UserQueryRoot},
    },
    configs::Configs,
};
use sqlx::postgres::PgPoolOptions;

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
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source)
}

pub struct GraphQlApp;

impl GraphQlApp {
    pub async fn setup() -> anyhow::Result<Schema<Query, Mutation, EmptySubscription>> {
        let Configs {
            application_settings: ref application,
            database_settings: database,
            ..
        } = Configs::init();

        use Environemnt::*;
        let (limit_depth, limit_complexity) = match application.environment {
            Local | Development | Staging => (usize::max_value(), usize::max_value()),
            _ => (5, 7),
        };

        let connection_pool = PgPoolOptions::new()
            .connect_timeout(Duration::from_secs(15))
            .connect_lazy_with(database.with_db());

        sqlx::migrate!("./migrations").run(&connection_pool).await?;

        let schema = get_graphql_schema()
            .data(connection_pool)
            .limit_depth(limit_depth) // This and also limi_complexity will prevent the graphql playground document from showing because it's unable to do the complete tree parsing. TODO: Add it conditionally. i.e if not in development or test environemnt.
            .limit_complexity(limit_complexity)
            .finish();

        Ok(schema)
    }
}
