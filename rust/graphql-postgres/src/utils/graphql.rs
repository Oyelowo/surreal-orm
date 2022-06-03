use std::path::Path;
use std::time::Duration;

use actix_web::{web, HttpRequest, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use common::utils;

use super::configuration::Environment;
use crate::app::{get_my_graphql_schema, MyGraphQLSchema};

use crate::utils::configuration;
use sqlx::postgres::PgPoolOptions;

pub async fn index(
    schema: web::Data<MyGraphQLSchema>,
    // req: HttpRequest,
    // db: actix_web::web::Data<Database>,
    gql_request: GraphQLRequest,
    // _session: Session,
) -> GraphQLResponse {
    let request = gql_request.into_inner();
    // Get session data and stick it into graphql context
    // let session = TypedSession::new(req.get_session());

    // If, using, jwt, Stick jwt token from headers into graphql context.
    // Presently not using it but cookie session managed with redis
    // let token = Token::get_token_from_headers(req.headers());

    // request = request.data(session).data(token);
    schema.execute(request).await.into()
}

pub async fn index_playground() -> HttpResponse {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source)
}

pub async fn setup_graphql_schema() -> anyhow::Result<MyGraphQLSchema> {
    // env_logger::init();
    use Environment::*;
    let application = configuration::get_app_config();
    let database = configuration::get_postgres_config();

    let (limit_depth, limit_complexity) = match application.environment {
        Local | Development | Staging => (usize::max_value(), usize::max_value()),
        _ => (5, 300),
    };

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(15))
        .connect_lazy_with(database.with_db());

    sqlx::migrate!("./migrations").run(&connection_pool).await?;

    let schema = get_my_graphql_schema()
        .data(connection_pool)
        .limit_depth(limit_depth) // This and also limi_complexity will prevent the graphql playground document from showing because it's unable to do the complete tree parsing. TODO: Add it conditionally. i.e if not in development or test environemnt.
        .limit_complexity(limit_complexity)
        .finish();

    Ok(schema)
}

pub fn generate_schema(path: impl AsRef<Path>) {
    let data = &get_my_graphql_schema().finish().sdl();
    utils::write_data_to_path(data, path);
}
