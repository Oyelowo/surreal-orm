use std::process;

use anyhow::Context;
use async_graphql::extensions::Logger;
use common::configurations::{application::ApplicationConfigs, redis::RedisConfigs};
use env_logger::Logger;
use graphql_mongo::{
    middleware,
    utils::{
        configuration, cors,
        graphql::{graphql_handler, graphql_handler_ws, graphql_playground, setup_graphql},
        session,
    },
};
use log::info;
use poem::{
    get,
    listener::TcpListener,
    middleware::Tracing,
    session::{CookieConfig, RedisStorage, ServerSession},
    EndpointExt, Route, Server,
};
use redis::aio::ConnectionManager;

#[tokio::main]
async fn main() {
    env_logger::init();
    let application = ApplicationConfigs::get();
    let redis = RedisConfigs::get();
    let app_url = &application.get_url();

    let schema = setup_graphql()
        .await
        .with_context(|| "Problem setting up graphql")
        .unwrap_or_else(|e| {
            log::error!("{e:?}");
            process::exit(1)
        });

    let app = Route::new()
        .at("/graphql/", get(graphql_playground).post(graphql_handler))
        .at("/graphql/ws", get(graphql_handler_ws))
        .with(middleware::get_session(
            redis_config,
            application.environment,
        ))
        .with(middleware::get_cors())
        .with(Logger)
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
