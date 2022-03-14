use actix_session::{CookieSession, Session, UserSession};
use actix_web::http::header::HeaderMap;
use actix_web::{
    get, post,
    web::{self},
    FromRequest, HttpRequest, HttpResponse,
};

use anyhow::Context;
use async_graphql::Schema;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Data,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use bson::oid::ObjectId;
use common::authentication::session_state::TypedSession;
use mongodb::Database;
use serde::Deserialize;

use super::configuration::Environemnt;
use crate::app::user::User;
use crate::app::{get_my_graphql_schema, sync_mongo_models, MyGraphQLSchema};
use crate::configs::Configs;
use common::utils;

use std::default;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Token(pub String);

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Token")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
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

    // let session = TypedSession(req.get_session());
    // access session data
    // if let Some(count) = req.get_session().get::<i32>("user_id").unwrap() {
    //     println!("get_session value: {}", count);
    //     req.get_session().insert("user_id", "23213");
    // } else {
    //     req.get_session().set("user_id", "rere")?;
    // }

    // let k = req.get_session().get::<i32>("user_id");
    // let session = SessionShared::new(req.get_session());

    // let session = req.get_session();
    // let cookier: Arc<Mutex<Option<String>>> = Default::default();

    // cookier.lock().
    // let p = Arc::new(Mutex::new(req.get_session()));
    // if let Some(id) = session.get::<ObjectId>("user_id").unwrap_or(None) {
    //     // let user = User::find_by_id(&db, &id).await.unwrap();
    //     if let Some(user) = User::find_by_id(&db, &id).await {
    //         request = request.data(user);
    //     }
    // }
    // if let Some(token) = get_token_from_headers(req.headers()) {
    //     request = request.data(token);
    // }
    // request = request.data(session);

    // let session = Shared::new(session);
    // //  cookier.lock().await = Some(session);
    // let sess = req.get_session();
    // let k = Arc::new(Mutex::new(sess));
    // let p = k.clone().lock().as_deref().unwrap();
    let session = TypedSession::new(req.get_session());
    request = request.data(session);
    // request = request.data(req.get_session());

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
    if let Some(token) = get_token_from_headers(req.headers()) {
        data.insert(token)
    }

    GraphQLSubscription::new(Schema::clone(&*schema))
        .with_data(data)
        .on_connection_init(on_connection_init)
        .start(&req, payload)
}

#[get("/graphql")]
pub async fn gql_playground() -> HttpResponse {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source)
}

pub struct GraphQlApp;

impl GraphQlApp {
    pub async fn setup() -> anyhow::Result<MyGraphQLSchema> {
        let Configs {
            ref application,
            database,
            ..
        } = Configs::init();

        use Environemnt::*;
        let (limit_depth, limit_complexity) = match application.environment {
            Local | Development | Staging => (usize::max_value(), usize::max_value()),
            Production => (5, 7),
        };

        let db = database.get_database()?;

        // let db = Client::with_uri_str(database.get_url())
        //     .await?
        //     .database(database.name.as_str());

        sync_mongo_models(&db).await?;

        let schema = get_my_graphql_schema()
            .data(db)
            .limit_depth(limit_depth) // This and also limi_complexity will prevent the graphql playground document from showing because it's unable to do the complete tree parsing. TODO: Add it conditionally. i.e if not in development or test environemnt.
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
