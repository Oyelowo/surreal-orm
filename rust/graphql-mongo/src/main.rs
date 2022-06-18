use std::process;

use anyhow::Context;
use common::{
    configurations::{
        application::ApplicationConfigs, mongodb::MongodbConfigs, redis::RedisConfigs,
    },
    middleware,
};

use graphql_mongo::{
    handlers::oauth::{oauth_login_authentication, oauth_login_initiator},
    utils::graphql::{graphql_handler, graphql_handler_ws, graphql_playground, setup_graphql},
};
use log::info;
use poem::{
    get,
    listener::TcpListener,
    middleware::{AddData, Tracing},
    EndpointExt, Route, Server,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let application = ApplicationConfigs::get();
    let redis_config = RedisConfigs::get();
    let redis = redis_config.clone().get_client().unwrap_or_else(|e| {
        log::error!("Problem getting database. Error: {e:?}");
        process::exit(-1)
    });
    let database = MongodbConfigs::get();
    let database = database.get_database().unwrap_or_else(|e| {
        log::error!("Problem getting database. Error: {e:?}");
        process::exit(-1)
    });

    let app_url = &application.get_url();

    let con = redis_config.clone().get_client().unwrap();
    let schema = setup_graphql()
        .await
        .with_context(|| "Problem setting up graphql")
        .unwrap_or_else(|e| {
            log::error!("{e:?}");
            process::exit(1)
        });

    let session = middleware::get_session(redis_config.clone(), application.environment)
        .await
        .unwrap_or_else(|e| {
            log::error!("{e:?}");
            process::exit(1)
        });

    let app = Route::new()
        .at(
            "/api/oauth/signin/:oauth_provider",
            get(oauth_login_initiator),
        )
        .at("/api/oauth/callback", get(oauth_login_authentication))
        .at(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .at("/api/graphql/ws", get(graphql_handler_ws))
        .data(schema)
        .data(database)
        .data(redis)
        .data(redis_config)
        .with(AddData::new(con))
        .with(session)
        .with(middleware::get_cors())
        // .with(Logger)
        .with(Tracing);

    info!("Playground: {app_url}");

    Server::new(TcpListener::bind(app_url))
        .run(app)
        .await
        .unwrap_or_else(|e| {
            log::error!("Problem running server. Error: {e}");
            process::exit(-1)
        });
}
