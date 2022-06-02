use actix_session::SessionExt;
use actix_web::http::header::HeaderMap;
use actix_web::{get, post, web, HttpRequest, HttpResponse};

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Data, ErrorExtensions, Result, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use common::authentication::TypedSession;
use common::error_handling::ApiHttpStatus;
use log::warn;
use serde::Deserialize;

use super::configuration::Environment;
use crate::app::{get_my_graphql_schema, sync_mongo_models, MyGraphQLSchema};
use crate::configs::Configs;
use common::utils;
extern crate derive_more;
use derive_more::From;
use std::path::Path;

pub static MONGO_ID_KEY: &str = "_id";

#[derive(From, PartialEq)]
pub struct Token(pub String);

impl Token {
    pub fn from_ctx<'a>(ctx: &'a async_graphql::Context<'_>) -> Result<&'a Self> {
        return ctx.data::<Self>().map_err(|e| {
            warn!("{e:?}");
            ApiHttpStatus::InternalServerError("Something went wrong while getting session".into())
                .extend()
        });
    }

    fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
        // This should probably include some validations
        // of the token and its expiry date and maybe refreshing the token or something
        headers
            .get("Token")
            .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
    }
}

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

pub async fn on_connection_init(value: serde_json::Value) -> async_graphql::Result<Data> {
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

pub struct GraphQlApp;

impl GraphQlApp {
    pub async fn setup() -> anyhow::Result<MyGraphQLSchema> {
        let application = Configs::get_app_config();
        let database = Configs::get_db_config();

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
}
// TEsTING for session
