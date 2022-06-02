use actix_session::SessionExt;

use actix_web::{get, post, web, HttpRequest, HttpResponse};

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Data, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use common::authentication::TypedSession;

use serde::Deserialize;

use super::configuration::{self, Environment};

use super::token::Token;
use crate::app::{get_my_graphql_schema, sync_mongo_models, MyGraphQLSchema};
use common::utils;
extern crate derive_more;

use std::path::Path;

#[post("/graphql")]
pub async fn index(
    schema: web::Data<MyGraphQLSchema>,
    req: HttpRequest,
    // db: actix_web::web::Data<Database>,
    gql_request: GraphQLRequest,
    // _session: Session,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();

    // Get session data and stick it into graphql context
    let session = TypedSession::new(req.get_session());

    // If, using, jwt, Stick jwt token from headers into graphql context.
    // Presently not using it but cookie session managed with redis
    let token = Token::get_token_from_headers(req.headers());

    request = request.data(session).data(token);

    schema.execute(request).await.into()
}

async fn on_connection_init(value: serde_json::Value) -> async_graphql::Result<Data> {
    #[derive(Deserialize)]
    struct Payload {
        token: String,
    }

    if let Ok(payload) = serde_json::from_value::<Payload>(value) {
        let mut data = Data::default();
        data.insert(Token(payload.token));
        Ok(data)
    } else {
        Err("Token is required".into())
    }
}

pub async fn index_ws(
    schema: web::Data<MyGraphQLSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    let mut data = Data::default();
    let session = TypedSession::new(req.get_session());
    let token = Token::get_token_from_headers(req.headers());

    data.insert(token);
    data.insert(session);

    GraphQLSubscription::new(Schema::clone(&*schema))
        .with_data(data)
        .on_connection_init(on_connection_init)
        .start(&req, payload)
}

#[get("/graphql")]
pub async fn gql_playground() -> HttpResponse {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/graphql")
            .subscription_endpoint("/graphql")
            .with_setting("credentials", "include"), // e.g allow cookies
    );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source)
}

pub async fn setup_graphql() -> anyhow::Result<MyGraphQLSchema> {
    let application = configuration::get_app_config();
    let database = configuration::get_db_config();

    use Environment::*;
    let (limit_depth, limit_complexity) = match application.environment {
        Local | Development | Staging => (usize::max_value(), usize::max_value()),
        Production => (8, 200),
    };

    let db = database.get_database()?;

    sync_mongo_models(&db).await?;

    let schema = get_my_graphql_schema()
        .data(db)
        .limit_depth(limit_depth) // This and also limit_complexity will prevent the graphql playground document from showing because it's unable to do the complete tree parsing. TODO: Add it conditionally. i.e if not in development or test environemnt.
        .limit_complexity(limit_complexity)
        .finish();

    Ok(schema)
}

pub fn generate_schema(path: impl AsRef<Path>) {
    let data = &get_my_graphql_schema().finish().sdl();
    utils::write_data_to_path(data, path);
}
