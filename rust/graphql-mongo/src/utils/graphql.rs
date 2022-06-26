use poem::{
    handler,
    http::HeaderMap,
    session::Session,
    web::{websocket::WebSocket, Data, Html},
    IntoResponse,
};

use async_graphql::extensions::ApolloTracing;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, ALL_WEBSOCKET_PROTOCOLS};
use async_graphql_poem::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use common::{
    authentication::TypedSession,
    configurations::{
        application::{ApplicationConfigs, Environment},
        mongodb::MongodbConfigs,
    },
};

use serde::Deserialize;

use super::token::Token;
use crate::app::{get_my_graphql_schema, sync_mongo_models, MyGraphQLSchema};
use common::utils;
extern crate derive_more;

use std::path::Path;

#[handler]
pub async fn graphql_handler(
    session: &Session,
    schema: Data<&MyGraphQLSchema>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let session = TypedSession(session.to_owned());

    // If, using, jwt, Stick jwt token from headers into graphql context.
    // Presently not using it but cookie session managed with redis
    let token = Token::get_token_from_headers(headers);

    let request = req.0.data(session).data(token);
    schema.execute(request).await.into()
}

async fn on_connection_init(
    value: serde_json::Value,
) -> async_graphql::Result<async_graphql::Data> {
    #[derive(Deserialize)]
    struct Payload {
        token: String,
    }

    if let Ok(payload) = serde_json::from_value::<Payload>(value) {
        let mut data = async_graphql::Data::default();
        data.insert(Token(payload.token));
        Ok(data)
    } else {
        Err("Token is required".into())
    }
}

#[handler]
pub async fn graphql_handler_ws(
    schema: Data<&MyGraphQLSchema>,
    headers: &HeaderMap,
    protocol: GraphQLProtocol,
    websocket: WebSocket,
    session: &Session,
) -> impl IntoResponse {
    let mut data = async_graphql::Data::default();
    if let Some(token) = Token::get_token_from_headers(headers) {
        data.insert(token);
    }

    let schema = schema.0.clone();
    let session = TypedSession(session.clone());

    data.insert(session);
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema, protocol)
                .with_data(data)
                .on_connection_init(on_connection_init)
                .serve()
        })
}

#[handler]
pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("graphql")
            .subscription_endpoint("ws")
            .with_setting("credentials", "include"), // e.g allow cookies
    ))
}

pub async fn setup_graphql() -> anyhow::Result<MyGraphQLSchema> {
    let application = ApplicationConfigs::get();
    let database = MongodbConfigs::get();

    use Environment::*;
    let (limit_depth, limit_complexity) = match application.environment {
        Local | Development | Staging => (usize::max_value(), usize::max_value()),
        Production => (8, 200),
    };

    let db = database.get_database()?;

    sync_mongo_models(&db).await?;

    let schema = get_my_graphql_schema()
        .data(db)
        .extension(ApolloTracing)
        .limit_depth(limit_depth) // This and also limit_complexity will prevent the graphql playground document from showing because it's unable to do the complete tree parsing. TODO: Add it conditionally. i.e if not in development or test environemnt.
        .limit_complexity(limit_complexity)
        .finish();

    Ok(schema)
}

pub fn generate_schema(path: impl AsRef<Path>) {
    let data = &get_my_graphql_schema().finish().sdl();
    utils::write_data_to_path(data, path);
}
