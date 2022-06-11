use std::process;

use anyhow::Context;
use common::{
    configurations::{application::ApplicationConfigs, redis::RedisConfigs},
    middleware,
};

use graphql_mongo::utils::graphql::{
    graphql_handler, graphql_handler_ws, graphql_playground, setup_graphql,
};
use log::info;
use poem::{get, listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};

#[tokio::main]
async fn main() {
    env_logger::init();
    let application = ApplicationConfigs::get();
    let redis_config = RedisConfigs::get();
    let app_url = &application.get_url();

    let schema = setup_graphql()
        .await
        .with_context(|| "Problem setting up graphql")
        .unwrap_or_else(|e| {
            log::error!("{e:?}");
            process::exit(1)
        });

    let session = middleware::get_session(redis_config, application.environment)
        .await
        .unwrap_or_else(|e| {
            log::error!("{e:?}");
            process::exit(1)
        });

    let app = Route::new()
        .at("/graphql/", get(graphql_playground).post(graphql_handler))
        .at("/graphql/ws", get(graphql_handler_ws))
        .data(schema)
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
